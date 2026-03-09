#!/usr/bin/env python3
"""Verify ArtCraft CLI invoke tiering / safety contract.

Default behavior is deliberately NON-DESTRUCTIVE:
- SAFE commands are executed and must succeed.
- UNSAFE commands are *not* executed; we only verify enforcement:
  - without --unsafe => unsafe_required (exit 3)
  - with --unsafe but gate disabled => unsafe_gate_disabled (exit 2)
  - payload @file is not read before enforcement

Optional mode (explicit opt-in): run an approved, readonly UNSAFE subset with the
unsafe gate enabled. This is useful as a smoke-test for the --unsafe path.

This script is intended to validate the contract documented in:
- docs/COMMAND_MATRIX.md

Usage:
  ./scripts/tools/verify_artcraft_cli_commands.py
  ./scripts/tools/verify_artcraft_cli_commands.py --run-unsafe-subset readonly --unsafe-gate-on
  ./scripts/tools/verify_artcraft_cli_commands.py --run-unsafe-subset readonly-network-cost --unsafe-gate-on
  ./scripts/tools/verify_artcraft_cli_commands.py --run-unsafe-subset readonly-account --unsafe-gate-on --allow-credentialed

Notes:
- By default we isolate config reads by setting XDG_CONFIG_HOME to a temp dir.
  This makes the unsafe gate deterministically disabled, regardless of a
  developer's real ~/.config/artcraft/cli.json.
- This tool intentionally avoids printing command payloads or responses.
"""

from __future__ import annotations

import argparse
import copy
import json
import os
import subprocess
import sys
import tempfile
from dataclasses import dataclass
from pathlib import Path
from typing import Any


DEFAULT_BINARY = Path("./target/release/artcraft")

# 1x1 transparent PNG
# Generated once and embedded to avoid filesystem dependencies.
ONE_BY_ONE_PNG_B64 = (
    "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAIAAACQd1PeAAAACXBIWXMAAAABAAAAAQBPJcTW"
    "AAAADElEQVR4nGNkYGAAAAAIAAI76MGHAAAAAElFTkSuQmCC"
)

# Initial, hardcoded UNSAFE subset(s) we allow this verifier to execute.
# v1: readonly subset.
# v2: readonly-network-cost subset (hits network, should remain readonly; no credentials).
# v3: readonly-account subset (credentialed account reads; explicit opt-in required).
UNSAFE_SUBSETS: dict[str, list[str]] = {
    "readonly": [
        "get_app_info_command",
        "get_provider_order_command",
        "get_task_queue_command",
        "get_app_preferences_command",
    ],
    "readonly-network-cost": [
        "estimate_image_cost_command",
        "estimate_video_cost_command",
    ],
    "readonly-account": [
        "storyteller_get_credits_command",
        "storyteller_get_subscription_command",
    ],
}

# If we ever include parsed JSON in an error message, redact these key families.
_SENSITIVE_KEY_SUBSTRINGS = (
    "token",
    "secret",
    "bearer",
    "authorization",
    "password",
    "api_key",
    "apikey",
)


@dataclass
class RunResult:
    argv: list[str]
    code: int
    stdout: str
    stderr: str


class VerificationError(RuntimeError):
    pass


def _is_sensitive_key(key: str) -> bool:
    k = key.lower()
    return any(s in k for s in _SENSITIVE_KEY_SUBSTRINGS)


def _redact_sensitive(obj: Any) -> Any:
    """Deep-redact known-sensitive fields in JSON-like structures."""

    if isinstance(obj, dict):
        out: dict[str, Any] = {}
        for k, v in obj.items():
            if isinstance(k, str) and _is_sensitive_key(k):
                out[k] = "***REDACTED***"
            else:
                out[k] = _redact_sensitive(v)
        return out

    if isinstance(obj, list):
        return [_redact_sensitive(x) for x in obj]

    return obj


def _json_from_stdout(stdout: str) -> Any:
    """Parse a single JSON value from CLI stdout.

    Contract: when invoked with --json, the CLI must emit *JSON-only* on stdout
    (aside from leading/trailing whitespace). Any debug/log output belongs on
    stderr.

    NOTE: This verifier intentionally avoids echoing stdout on parse failure.
    """

    s = stdout.strip()
    if not s:
        raise VerificationError("expected JSON-only stdout, got empty output")

    try:
        # json.loads() already requires the entire string to be valid JSON.
        return json.loads(s)
    except json.JSONDecodeError as e:
        raise VerificationError(f"failed to parse JSON-only stdout: {e}")


def _extract_error_code(obj: Any) -> str | None:
    if not isinstance(obj, dict):
        return None
    details = obj.get("error_details")
    if isinstance(details, dict):
        code = details.get("code")
        if isinstance(code, str):
            return code
    return None


def run_cmd(argv: list[str], env: dict[str, str], timeout_s: float = 30.0) -> RunResult:
    p = subprocess.run(
        argv,
        env=env,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        timeout=timeout_s,
    )
    return RunResult(argv=argv, code=p.returncode, stdout=p.stdout, stderr=p.stderr)


def require(cond: bool, msg: str) -> None:
    if not cond:
        raise VerificationError(msg)


def verify_list_allowed(
    binary: Path,
    env: dict[str, str],
    *,
    expected_gate_enabled: bool | None = None,
) -> tuple[list[str], list[str], bool]:
    rr = run_cmd([str(binary), "invoke", "--list-allowed", "--json"], env=env)
    require(rr.code == 0, f"--list-allowed should exit 0, got {rr.code}")

    obj = _json_from_stdout(rr.stdout)
    require(isinstance(obj, dict), "--list-allowed should output a JSON object")

    safe = obj.get("safe")
    unsafe = obj.get("unsafe")
    gate = obj.get("unsafeGateEnabled")

    require(isinstance(safe, list) and all(isinstance(x, str) for x in safe), "safe must be an array of strings")
    require(isinstance(unsafe, list) and all(isinstance(x, str) for x in unsafe), "unsafe must be an array of strings")
    require(isinstance(gate, bool), "unsafeGateEnabled must be a boolean")

    if expected_gate_enabled is not None:
        require(gate is expected_gate_enabled, f"unsafeGateEnabled expected {expected_gate_enabled}, got {gate}")

    # This repository's expected SAFE tier (stable contract).
    require(
        set(["platform_info_command", "flip_image"]).issubset(set(safe)),
        "expected SAFE allowlist to include platform_info_command and flip_image",
    )

    # Sanity: SAFE commands must not show up in unsafe listing.
    require("platform_info_command" not in unsafe, "platform_info_command should not appear in unsafe list")
    require("flip_image" not in unsafe, "flip_image should not appear in unsafe list")

    return safe, unsafe, gate


def verify_safe_platform_info(binary: Path, env: dict[str, str]) -> None:
    rr = run_cmd([str(binary), "invoke", "platform_info_command", "--json"], env=env)
    require(rr.code == 0, f"SAFE platform_info_command should exit 0, got {rr.code}")
    _json_from_stdout(rr.stdout)


def verify_safe_flip_image(binary: Path, env: dict[str, str]) -> None:
    payload = json.dumps({"image": ONE_BY_ONE_PNG_B64})
    rr = run_cmd([str(binary), "invoke", "flip_image", "--payload", payload, "--json"], env=env)
    require(rr.code == 0, f"SAFE flip_image should exit 0, got {rr.code}")
    obj = _json_from_stdout(rr.stdout)
    # flip_image uses cli_success_json wrapper.
    require(isinstance(obj, dict), "flip_image should return a JSON object")
    require(obj.get("status") in ("Success", "success", "SUCCESS"), "flip_image expected status=Success")


def verify_unknown_command(binary: Path, env: dict[str, str]) -> None:
    rr = run_cmd([str(binary), "invoke", "totally_fake_command_xyz", "--json"], env=env)
    require(rr.code == 3, f"unknown/disallowed command should exit 3, got {rr.code}")
    obj = _json_from_stdout(rr.stdout)
    require(_extract_error_code(obj) == "disallowed_command", "expected disallowed_command error code")


def verify_unsafe_enforcement(binary: Path, env: dict[str, str], command: str) -> None:
    # 1) Missing --unsafe
    rr1 = run_cmd([str(binary), "invoke", command, "--json"], env=env)
    require(rr1.code == 3, f"UNSAFE {command} without --unsafe should exit 3; got {rr1.code}")
    obj1 = _json_from_stdout(rr1.stdout)
    require(_extract_error_code(obj1) == "unsafe_required", f"expected unsafe_required for {command} without --unsafe")

    # 2) --unsafe present but gate disabled
    rr2 = run_cmd([str(binary), "invoke", "--unsafe", command, "--json"], env=env)
    require(rr2.code == 2, f"UNSAFE {command} with --unsafe but gate disabled should exit 2; got {rr2.code}")
    obj2 = _json_from_stdout(rr2.stdout)
    require(_extract_error_code(obj2) == "unsafe_gate_disabled", f"expected unsafe_gate_disabled for {command} when gate disabled")


def verify_payload_hardening(binary: Path, env: dict[str, str], command: str) -> None:
    # The contract: tier enforcement happens before any @file read.
    missing_path = "/nonexistent/should-not-be-read-by-cli.json"

    rr1 = run_cmd([str(binary), "invoke", command, "--payload", f"@{missing_path}", "--json"], env=env)
    require(rr1.code == 3, f"UNSAFE {command} missing --unsafe should exit 3 even with @payload; got {rr1.code}")
    obj1 = _json_from_stdout(rr1.stdout)
    require(_extract_error_code(obj1) == "unsafe_required", f"expected unsafe_required (not payload read error) for {command} with @payload")

    rr2 = run_cmd([str(binary), "invoke", "--unsafe", command, "--payload", f"@{missing_path}", "--json"], env=env)
    require(rr2.code == 2, f"UNSAFE {command} gate disabled should exit 2 even with @payload; got {rr2.code}")
    obj2 = _json_from_stdout(rr2.stdout)
    require(_extract_error_code(obj2) == "unsafe_gate_disabled", f"expected unsafe_gate_disabled (not payload read error) for {command} with @payload")


def _payload_for_unsafe_subset_command(subset_name: str, cmd: str) -> str | None:
    """Optional per-command payloads for UNSAFE subsets.

    NOTE: Do not print payload contents.
    """

    if subset_name != "readonly-network-cost":
        # readonly + readonly-account commands currently take no payload.
        return None

    if cmd == "estimate_image_cost_command":
        # NOTE: estimate_*_cost requests are NOT wrapped in {"request": ...}.
        # Keep this payload minimal and stable for smoke tests.
        payload = {
            "model": "nano_banana_pro",
            "provider": "artcraft",
            "generation_mode": {"type": "text_to_image"},
            "aspect_ratio": None,
            "resolution": None,
            "image_batch_count": None,
        }
        return json.dumps(payload)

    if cmd == "estimate_video_cost_command":
        # Use a known-supported model to keep the verifier stable.
        payload = {
            "model": "seedance_2p0",
            "provider": "artcraft",
            "generation_mode": {"type": "text_to_video"},
            "aspect_ratio": None,
            "resolution": None,
            # Some models enforce a minimum duration; 1s keeps the request small.
            "duration_seconds": 1,
            "video_batch_count": None,
        }
        return json.dumps(payload)

    return None


def run_unsafe_subset(
    binary: Path,
    env: dict[str, str],
    *,
    subset_name: str,
    unsafe_allowlist: list[str],
    allow_credentialed: bool,
) -> None:
    commands = UNSAFE_SUBSETS[subset_name]

    # Validate we are only running known-UNSAFE commands.
    for cmd in commands:
        require(cmd in unsafe_allowlist, f"requested UNSAFE subset command not in CLI unsafe allowlist: {cmd}")

    for cmd in commands:
        if subset_name == "readonly-account" and not allow_credentialed:
            print(f"{cmd}: SKIPPED (credentialed; re-run with --allow-credentialed)")
            continue

        payload = _payload_for_unsafe_subset_command(subset_name, cmd)

        argv = [str(binary), "invoke", "--unsafe", cmd]
        if payload is not None:
            argv += ["--payload", payload]
        argv += ["--json"]

        rr = run_cmd(argv, env=env)
        if rr.code != 0:
            print(f"{cmd}: FAIL", file=sys.stderr)
            raise VerificationError(f"UNSAFE subset command {cmd} should exit 0, got {rr.code}")

        # Require JSON-only stdout for all subset commands (no debug/log spew).
        obj = _json_from_stdout(rr.stdout)
        # Defensive: ensure any sensitive keys would be redacted if we ever print.
        _ = _redact_sensitive(copy.deepcopy(obj))

        print(f"{cmd}: OK")


def main() -> int:
    ap = argparse.ArgumentParser(description="Verify ArtCraft CLI invoke contract")
    ap.add_argument("--binary", type=Path, default=DEFAULT_BINARY, help="Path to artcraft binary (default: ./target/release/artcraft)")
    ap.add_argument("--skip-safe", action="store_true", help="Skip executing SAFE commands (still checks unsafe enforcement)")
    ap.add_argument("--skip-payload-hardening", action="store_true", help="Skip @file payload hardening checks")
    ap.add_argument(
        "--run-unsafe-subset",
        "--unsafe-subset",
        dest="unsafe_subset",
        choices=sorted(UNSAFE_SUBSETS.keys()),
        help="Explicit opt-in: execute an approved UNSAFE subset (requires --unsafe-gate-on)",
    )
    ap.add_argument(
        "--unsafe-gate-on",
        action="store_true",
        help="Explicit opt-in safety latch: enable the UNSAFE invoke gate for --run-unsafe-subset",
    )
    ap.add_argument(
        "--allow-credentialed",
        action="store_true",
        help="Allow running credentialed UNSAFE subsets (e.g. readonly-account). Without this flag they are SKIPPED.",
    )
    args = ap.parse_args()

    if args.unsafe_subset and not args.unsafe_gate_on:
        print("ERROR: --run-unsafe-subset requires --unsafe-gate-on (explicit opt-in)", file=sys.stderr)
        return 2

    binary: Path = args.binary
    if not binary.exists() or not os.access(binary, os.X_OK):
        print(f"ERROR: artcraft binary not found/executable at: {binary}", file=sys.stderr)
        print("Build it first (example): cargo build --release -p artcraft", file=sys.stderr)
        return 2

    failures: list[str] = []

    with tempfile.TemporaryDirectory(prefix="artcraft-cli-verify-") as td:
        env_base = dict(os.environ)
        # Isolate config reads for determinism.
        env_base["XDG_CONFIG_HOME"] = td

        env_gate_off = dict(env_base)
        env_gate_off["ARTCRAFT_ENABLE_UNSAFE_INVOKE"] = "0"

        env_gate_on = dict(env_base)
        env_gate_on["ARTCRAFT_ENABLE_UNSAFE_INVOKE"] = "1"

        try:
            safe, unsafe, gate = verify_list_allowed(binary=binary, env=env_gate_off, expected_gate_enabled=False)

            if not args.skip_safe:
                verify_safe_platform_info(binary=binary, env=env_gate_off)
                verify_safe_flip_image(binary=binary, env=env_gate_off)

            verify_unknown_command(binary=binary, env=env_gate_off)

            # Enforce for every UNSAFE command.
            for cmd in unsafe:
                verify_unsafe_enforcement(binary=binary, env=env_gate_off, command=cmd)
                if not args.skip_payload_hardening:
                    verify_payload_hardening(binary=binary, env=env_gate_off, command=cmd)

            # Optional: run a readonly subset with gate enabled.
            if args.unsafe_subset:
                # Ensure gate is truly on in this env.
                _safe2, unsafe2, _gate2 = verify_list_allowed(
                    binary=binary,
                    env=env_gate_on,
                    expected_gate_enabled=True,
                )

                run_unsafe_subset(
                    binary=binary,
                    env=env_gate_on,
                    subset_name=args.unsafe_subset,
                    unsafe_allowlist=unsafe2,
                    allow_credentialed=args.allow_credentialed,
                )

        except (VerificationError, subprocess.TimeoutExpired) as e:
            failures.append(str(e))

    if failures:
        print("FAILED:", file=sys.stderr)
        for f in failures:
            print(f"- {f}", file=sys.stderr)
        return 1

    print("OK: ArtCraft CLI invoke contract verified")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
