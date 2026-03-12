# ArtCraft — Safe/Unsafe CLI Commands (Allowlist Tiers)

**Date:** 2026-03-09  
**Status:** In Progress  
**Agent:** Chip 🐱‍💻

---

## Goal

Define and implement a clear safe-by-default CLI allowlist for `artcraft invoke`, with an explicit, gated `--unsafe` tier for sensitive commands.

---

## Overview

We already shipped an `artcraft invoke <tauri_command> ... --json` contract with safe allowlist defaults and a gated `--unsafe` mode (enabled via `ARTCRAFT_ENABLE_UNSAFE_INVOKE=1` or `~/.config/artcraft/cli.json`). The next step is to formalize *tiers* of commands:

- **Safe (default):** commands that are read-only, non-destructive, and do not expose sensitive data.
- **Unsafe (requires `--unsafe` + gate):** anything that can change state, execute code-like behavior, exfiltrate secrets, or access environment/system details beyond what’s needed.

We’ll produce a documented policy, implement it in the CLI dispatcher, and update the OpenClaw ArtCraft client to rely on these semantics.

---

## Tasks

### Task 1: Inventory & classification of ArtCraft Tauri commands

**SubAgent:** `research`  
**Prompt:** Scan the ArtCraft repo command list used by the CLI plugin (tauri command names). Propose a tiering: SAFE vs UNSAFE, with rationale for each command. Include edge cases (provider config, filesystem access, any token/key exposure).

**Folders Created/Deleted/Modified:**
- (none)

**Files Created/Deleted/Modified:**
- `plans/artcraft/2026-03-09-safe-unsafe-cli-allowlist.md`

**Status:** ✅ Complete

**Results:**
- Identified **47** invokable desktop Tauri commands in `crates/desktop/artcraft/src/lib.rs` `generate_handler![...]`.
- Proposed tiering for `artcraft invoke` (updated with Derrick’s direction):
  - **SAFE (strict / minimal default allowlist):**
    - `platform_info_command`
    - `flip_image`
  - **UNSAFE (`--unsafe` + gate):** everything else (no backwards-compat requirements; no separate “blocked” bucket).
- Key security edge cases to address while implementing:
  - `--payload @/path` **file read happens before allowlist check** today → move payload parsing after allowlist decision, and restrict `@file` usage to commands that are actually permitted (and likely only under `--unsafe`).
  - Ensure `--json` output is consistently **machine-readable JSON** for debugging (errors too, not just successes).
  - `get_task_queue_command` may expose `MediaFileToken`/URLs → treat as unsafe unless we explicitly redact.

---

### Task 2: Implement tiered allowlist in `gambit-artcraft` CLI dispatcher

**SubAgent:** `coder`  
**Prompt:** In `~/.openclaw/workspace/projects/gambit-artcraft` on branch `chip/artcraft-cli-buildfix`, implement tiered allowlist for `artcraft invoke`:
- Maintain explicit **STRICT SAFE** allowlist (default):
  - `platform_info_command`
  - `flip_image`
- Maintain explicit UNSAFE allowlist (everything else) requiring `--unsafe` + existing gate
- Keep exit code semantics (0/2/3/4)
- Add `artcraft invoke --list-allowed` output (JSON object) with tiers
- Ensure `--json` errors are also machine-readable JSON (ideally with a stable error `code`)
- Add/extend tests if present

**Folders Created/Deleted/Modified:**
- `projects/gambit-artcraft/`

**Files Created/Deleted/Modified:**
- (TBD by subagent)

**Status:** ✅ Complete

**Results:**
- Implemented **strict SAFE** invoke tier:
  - SAFE (no `--unsafe`): `platform_info_command`, `flip_image`
  - UNSAFE: everything else requires `--unsafe` **and** the existing gate (`ARTCRAFT_ENABLE_UNSAFE_INVOKE=1` or `~/.config/artcraft/cli.json` with `{ "enableUnsafeInvoke": true }`).
- Added `artcraft invoke --list-allowed --json`:
  - Output JSON object: `{ safe: [...], unsafe: [...], unsafeGateEnabled: bool }`.
  - Updated `crates/desktop/artcraft/tauri.conf.json`: added `list-allowed`, made positional `command` optional.
- Security hardening: defer `--payload` parsing (including `@file`) until after tier + gate checks.
- Error JSON improvements (when `--json` is used): include `error_details.code` such as `invalid_args`, `unsafe_gate_disabled`, `disallowed_command`, `unsafe_required`, `runtime_invoke_error`.
- Docs updated:
  - `artcraft-cli/README.md`
  - `artcraft-cli/QUICK_REFERENCE.md`
- Commit:
  - `e9601f330` — artcraft invoke: tiered allowlist + --list-allowed

---

### Task 3: Update OpenClaw ArtCraft client (`openclaw-artcraft`) to consume tiers

**SubAgent:** `primary`  
**Prompt:** In `~/.openclaw/workspace/projects/openclaw-artcraft/packages/client`, update client docs + errors to reflect SAFE/UNSAFE tiers. Optionally add a convenience method to query `--list-allowed` and validate before calling. Ensure tests remain hermetic (fake artcraft executable).

**Folders Created/Deleted/Modified:**
- `projects/openclaw-artcraft/`

**Files Created/Deleted/Modified:**
- (TBD by subagent)

**Status:** ⏳ Pending

**Results:**
- ✅ Client preflight completed (no code changes yet):
  - Implementation: `packages/client/artcraft_client/client.py`
    - `invoke_raw()` builds: `artcraft invoke <command> [--payload <json>] [--unsafe] --json`
    - `invoke()` maps exit codes: `2`→`InvalidArgs` or `UnsafeGateDisabled` (via stderr substring), `3`→`DisallowedCommand`, `4`→`InvokeError`.
  - Exceptions: `packages/client/artcraft_client/exceptions.py`
  - Contract/docs/tests: `packages/client/README.md`, `packages/client/tests/test_client.py`
- Recommendations to support tiers + `--list-allowed` cleanly:
  - Prefer `tier="safe"|"unsafe"` (keep `unsafe: bool` as deprecated alias).
  - Avoid brittle stderr parsing for exit code `2`; CLI should return machine-readable error codes (JSON error schema) or distinct exit codes.
  - Add a dedicated `list_allowed()` method; note `invoke()` currently requires stdout JSON to be an object/dict (arrays would fail).

---

### Task 4: Verification on installed `/usr/bin/artcraft` + docs

**SubAgent:** `primary`  
**Prompt:** Smoke test the installed `artcraft` binary behavior for:
- safe command works by default
- unsafe command blocked without `--unsafe`
- unsafe command blocked with `--unsafe` but gate off
- unsafe command works with `--unsafe` + gate on
Update docs/README snippets with exact commands.

**Status:** ⏳ Pending

---

## Final Results

**Status:** ⏳ Pending

**What We Built:**
- Tiered safe/unsafe command policy + implementation + client integration.

**Commits:**
- (TBD)

**Lessons Learned:**
- (TBD)
