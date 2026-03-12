# openclaw-artcraft — Client updates for invoke tiers + list_allowed

**Date:** 2026-03-09  
**Status:** In Progress  
**Agent:** Chip 🐱‍💻

---

## Goal

Update the `openclaw-artcraft` Python client to match the new ArtCraft CLI semantics:

- Strict SAFE vs UNSAFE tiers for `artcraft invoke`
- `artcraft invoke --list-allowed --json`
- Prefer machine-readable JSON error codes over stderr substring parsing

---

## Overview

ArtCraft CLI changes landed in `gambit-artcraft`:
- SAFE tier: `platform_info_command`, `flip_image`
- Everything else is UNSAFE and requires `--unsafe` + gate
- `invoke --list-allowed --json` returns `{ safe: [...], unsafe: [...], unsafeGateEnabled: bool }`
- Errors include JSON with a stable `error_details.code` (when `--json` is used)

The `openclaw-artcraft` client currently:
- exposes `unsafe: bool=False`
- treats exit code `2` as either invalid args or unsafe-gate-disabled via stderr substring matching
- requires stdout JSON to be an object/dict

Decision (Derrick): **no legacy support**. We will do a clean switch:
- remove `unsafe` boolean API entirely
- standardize on tiers (`tier="safe"|"unsafe"`)
- use CLI JSON `error_details.code` for deterministic exception mapping

---

## Tasks

### Task 1: Implement tiered API + list_allowed

**SubAgent:** `coder`
**Prompt:** In `~/.openclaw/workspace/projects/openclaw-artcraft/packages/client`:

1) Update `ArtCraftClient.invoke()` API (clean switch):
   - Replace `unsafe: bool` with `tier: Literal["safe","unsafe"]` (default `"safe"`).
   - Remove `unsafe` parameter entirely (no backwards compat).
   - Map `tier="unsafe"` → pass `--unsafe`.

2) Add `ArtCraftClient.list_allowed()`:
   - Calls: `artcraft invoke --list-allowed --json`.
   - Returns a typed dict/object with keys: `safe`, `unsafe`, `unsafeGateEnabled`.

3) Improve error mapping to use JSON error codes:
   - When `returncode != 0`, parse the CLI JSON error payload for `error_details.code`.
   - Use that to raise stable exceptions (at minimum: `UnsafeGateDisabled`, `DisallowedCommand`, `InvalidArgs`, `InvokeError`; optionally add `UnsafeRequired`).
   - Remove stderr substring heuristics (only fall back to stderr if JSON is missing/malformed).

4) Update docs + tests:
   - `packages/client/README.md` updated to describe tiers + `list_allowed()`.
   - Update hermetic tests (`tests/test_client.py`) and fake `artcraft` executable to support `--list-allowed`, tiers, and JSON error codes.
   - Update any call sites to remove usage of `unsafe=`.

Deliver back: branch name, commit hash(es), and how to use the new APIs.

**Status:** ✅ Complete

**Results:**
- Implemented clean switch (no legacy `unsafe: bool`):
  - Removed `unsafe` from `invoke()` and `invoke_raw()`.
  - Added `tier: Literal["safe","unsafe"]` (default `"safe"`), with `tier="unsafe"` mapping to CLI `--unsafe`.
- Added allowlist introspection:
  - `ArtCraftClient.list_allowed()` calls `artcraft invoke --list-allowed --json`.
  - Returns `AllowedCommands(safe, unsafe, unsafe_gate_enabled)` dataclass.
- Deterministic error mapping:
  - On non-zero exit codes, client parses JSON error payload (stdout first, then stderr) and maps via `error_details.code`.
  - Stderr substring heuristics kept only as a last-resort fallback.
- Files updated:
  - `packages/client/artcraft_client/client.py`
  - `packages/client/artcraft_client/__init__.py`
  - `packages/client/tests/test_client.py`
  - `packages/client/README.md`
  - `packages/client/setup.py` (version bump to `0.3.0`)
- Tests updated (hermetic fake `artcraft` extended for `--list-allowed` + JSON error codes); reported passing in subagent.
- Commit pushed to `main`:
  - `3856ce2` — `client: switch to tier API + add list_allowed`

---

### Task 2: Verify client against real installed CLI (optional)

**SubAgent:** `primary`
**Prompt:** If the installed `artcraft` binary supports the new behavior, run a small smoke test using the updated Python client to:
- call `list_allowed()`
- call a SAFE command
- call an UNSAFE command without gate and confirm exception

**Status:** ⏳ Pending (not run yet; local python env lacks pytest in this session)

---

## Final Results

**Status:** ⚠️ Partial (client changes shipped; optional real-binary smoke test pending)

**What We Built:**
- `openclaw-artcraft` Python client updated to tier-based API (`tier="safe"|"unsafe"`), plus `list_allowed()`.
- Deterministic exception mapping via CLI JSON `error_details.code`.

**Commits:**
- `3856ce2` — client: switch to tier API + add list_allowed
