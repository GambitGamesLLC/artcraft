#!/usr/bin/env python3
"""Verify ArtCraft CLI invoke tiering / safety contract.

Default behavior is deliberately NON-DESTRUCTIVE:
- SAFE commands are executed and must succeed.
- UNSAFE commands are *not* executed; we only verify enforcement:
  - without --unsafe => unsafe_required (exit 3)
  - with --unsafe but gate disabled => unsafe_gate_disabled (exit 2)
  - payload @file is not read before enforcement

This script is intended to validate the contract documented in:
- docs/COMMAND_MATRIX.md

Usage:
  ./scripts/tools/verify_artcraft_cli_commands.py

Notes:
- By default we isolate config reads by setting XDG_CONFIG_HOME to a temp dir.
  This makes the unsafe gate deterministically disabled, regardless of a
  developer's real ~/.config/artcraft/cli.json.
"""

from __future__ import annotations

import argparse
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


@dataclass
class RunResult:
    argv: list[str]
    code: int
    stdout: str
    stderr: str


class VerificationError(RuntimeError):
    pass


def _json_from_stdout(stdout: str) -> Any:
    """Parse a single JSON object from CLI stdout.

    The CLI is expected to print exactly one JSON value. We still try to be
    tolerant of leading/trailing whitespace.
    """

    s = stdout.strip()
    if not s:
        raise VerificationError("expected JSON on stdout, got empty output")

    # If something logs above JSON, try to recover by finding first '{' or '['.
    first_obj = min([i for i in (s.find("{"), s.find("[")) if i != -1], default=-1)
    if first_obj > 0:
        s = s[first_obj:]

    try:
        return json.loads(s)
    except json.JSONDecodeError as e:
        raise VerificationError(f"failed to parse JSON from stdout: {e}\n--- stdout ---\n{stdout}")


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


def verify_list_allowed(binary: Path, env: dict[str, str]) -> tuple[list[str], list[str], bool]:
    rr = run_cmd([str(binary), "invoke", "--list-allowed", "--json"], env=env)
    require(rr.code == 0, f"--list-allowed should exit 0, got {rr.code}\n{rr.stderr}")

    obj = _json_from_stdout(rr.stdout)
    require(isinstance(obj, dict), "--list-allowed should output a JSON object")

    safe = obj.get("safe")
    unsafe = obj.get("unsafe")
    gate = obj.get("unsafeGateEnabled")

    require(isinstance(safe, list) and all(isinstance(x, str) for x in safe), "safe must be an array of strings")
    require(isinstance(unsafe, list) and all(isinstance(x, str) for x in unsafe), "unsafe must be an array of strings")
    require(isinstance(gate, bool), "unsafeGateEnabled must be a boolean")

    # This repository's expected SAFE tier (stable contract).
    require(set(["platform_info_command", "flip_image"]).issubset(set(safe)), f"expected SAFE allowlist to include platform_info_command and flip_image; got: {safe}")

    # Sanity: SAFE commands must not show up in unsafe listing.
    require("platform_info_command" not in unsafe, "platform_info_command should not appear in unsafe list")
    require("flip_image" not in unsafe, "flip_image should not appear in unsafe list")

    return safe, unsafe, gate


def verify_safe_platform_info(binary: Path, env: dict[str, str]) -> None:
    rr = run_cmd([str(binary), "invoke", "platform_info_command", "--json"], env=env)
    require(rr.code == 0, f"SAFE platform_info_command should exit 0, got {rr.code}\n{rr.stderr}\n{rr.stdout}")
    _json_from_stdout(rr.stdout)


def verify_safe_flip_image(binary: Path, env: dict[str, str]) -> None:
    payload = json.dumps({"image": ONE_BY_ONE_PNG_B64})
    rr = run_cmd([str(binary), "invoke", "flip_image", "--payload", payload, "--json"], env=env)
    require(rr.code == 0, f"SAFE flip_image should exit 0, got {rr.code}\n{rr.stderr}\n{rr.stdout}")
    obj = _json_from_stdout(rr.stdout)
    # flip_image uses cli_success_json wrapper.
    require(isinstance(obj, dict), "flip_image should return a JSON object")
    require(obj.get("status") in ("Success", "success", "SUCCESS"), f"flip_image expected status=Success; got {obj.get('status')}")


def verify_unknown_command(binary: Path, env: dict[str, str]) -> None:
    rr = run_cmd([str(binary), "invoke", "totally_fake_command_xyz", "--json"], env=env)
    require(rr.code == 3, f"unknown/disallowed command should exit 3, got {rr.code}\n{rr.stderr}\n{rr.stdout}")
    obj = _json_from_stdout(rr.stdout)
    require(_extract_error_code(obj) == "disallowed_command", f"expected disallowed_command error code; got {obj}")


def verify_unsafe_enforcement(binary: Path, env: dict[str, str], command: str) -> None:
    # 1) Missing --unsafe
    rr1 = run_cmd([str(binary), "invoke", command, "--json"], env=env)
    require(rr1.code == 3, f"UNSAFE {command} without --unsafe should exit 3; got {rr1.code}\n{rr1.stderr}\n{rr1.stdout}")
    obj1 = _json_from_stdout(rr1.stdout)
    require(_extract_error_code(obj1) == "unsafe_required", f"expected unsafe_required for {command} without --unsafe; got {obj1}")

    # 2) --unsafe present but gate disabled
    rr2 = run_cmd([str(binary), "invoke", "--unsafe", command, "--json"], env=env)
    require(rr2.code == 2, f"UNSAFE {command} with --unsafe but gate disabled should exit 2; got {rr2.code}\n{rr2.stderr}\n{rr2.stdout}")
    obj2 = _json_from_stdout(rr2.stdout)
    require(_extract_error_code(obj2) == "unsafe_gate_disabled", f"expected unsafe_gate_disabled for {command} when gate disabled; got {obj2}")


def verify_payload_hardening(binary: Path, env: dict[str, str], command: str) -> None:
    # The contract: tier enforcement happens before any @file read.
    missing_path = "/nonexistent/should-not-be-read-by-cli.json"

    rr1 = run_cmd([str(binary), "invoke", command, "--payload", f"@{missing_path}", "--json"], env=env)
    require(rr1.code == 3, f"UNSAFE {command} missing --unsafe should exit 3 even with @payload; got {rr1.code}\n{rr1.stderr}\n{rr1.stdout}")
    obj1 = _json_from_stdout(rr1.stdout)
    require(_extract_error_code(obj1) == "unsafe_required", f"expected unsafe_required (not payload read error) for {command} with @payload; got {obj1}")

    rr2 = run_cmd([str(binary), "invoke", "--unsafe", command, "--payload", f"@{missing_path}", "--json"], env=env)
    require(rr2.code == 2, f"UNSAFE {command} gate disabled should exit 2 even with @payload; got {rr2.code}\n{rr2.stderr}\n{rr2.stdout}")
    obj2 = _json_from_stdout(rr2.stdout)
    require(_extract_error_code(obj2) == "unsafe_gate_disabled", f"expected unsafe_gate_disabled (not payload read error) for {command} with @payload; got {obj2}")


def main() -> int:
    ap = argparse.ArgumentParser(description="Verify ArtCraft CLI invoke contract")
    ap.add_argument("--binary", type=Path, default=DEFAULT_BINARY, help="Path to artcraft binary (default: ./target/release/artcraft)")
    ap.add_argument("--skip-safe", action="store_true", help="Skip executing SAFE commands (still checks unsafe enforcement)")
    ap.add_argument("--skip-payload-hardening", action="store_true", help="Skip @file payload hardening checks")
    args = ap.parse_args()

    binary: Path = args.binary
    if not binary.exists() or not os.access(binary, os.X_OK):
        print(f"ERROR: artcraft binary not found/executable at: {binary}", file=sys.stderr)
        print("Build it first (example): cargo build --release -p artcraft", file=sys.stderr)
        return 2

    failures: list[str] = []

    with tempfile.TemporaryDirectory(prefix="artcraft-cli-verify-") as td:
        env = dict(os.environ)
        # Deterministically disable gate by isolating config_dir and ensuring env gate isn't set.
        env["XDG_CONFIG_HOME"] = td
        env["ARTCRAFT_ENABLE_UNSAFE_INVOKE"] = "0"

        try:
            safe, unsafe, gate = verify_list_allowed(binary=binary, env=env)
            require(gate is False, "unsafeGateEnabled should be false in verification env")

            if not args.skip_safe:
                verify_safe_platform_info(binary=binary, env=env)
                verify_safe_flip_image(binary=binary, env=env)

            verify_unknown_command(binary=binary, env=env)

            # Enforce for every UNSAFE command.
            for cmd in unsafe:
                verify_unsafe_enforcement(binary=binary, env=env, command=cmd)
                if not args.skip_payload_hardening:
                    verify_payload_hardening(binary=binary, env=env, command=cmd)

        except (VerificationError, subprocess.TimeoutExpired) as e:
            failures.append(str(e))

    if failures:
        print("FAILED:\n", file=sys.stderr)
        for f in failures:
            print(f"- {f}", file=sys.stderr)
        return 1

    print("OK: ArtCraft CLI invoke contract verified")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
