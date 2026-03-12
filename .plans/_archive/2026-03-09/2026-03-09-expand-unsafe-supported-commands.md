# ArtCraft — Expand supported UNSAFE commands (verification + wrappers)

**Date:** 2026-03-09  
**Status:** In Progress  
**Agent:** Chip 🐱‍💻

---

## Goal

Expand the set of **UNSAFE** ArtCraft commands we actively “support” (document + verify + optionally provide wrapper helpers) now that the strict tier/gate model is stable.

---

## Overview

We currently have:
- Strict SAFE tier: `platform_info_command`, `flip_image`
- UNSAFE tier: everything else (requires `--unsafe` + gate)
- Verified contract end-to-end; verifier passes against installed `/usr/bin/artcraft`.

“Support” can mean different things. This plan focuses on:
1) defining an **approved UNSAFE subset** that is safe enough to run in automation when the gate is enabled
2) expanding the verifier to run that subset under gate-on
3) updating docs/skill examples accordingly

We will NOT run cost-incurring or destructive commands by default.

---

## Proposed UNSAFE tiers for automation

### UNSAFE-READONLY (OK to run under gate-on by default)
- `get_app_info_command`
- `get_provider_order_command`
- `get_task_queue_command` *(may contain tokens/user content; treat logs carefully)*
- `get_app_preferences_command`

### UNSAFE-MUTATING (requires explicit opt-in)
- `set_provider_order_command`
- `update_app_preferences_command`
- `mark_task_as_dismissed_command`
- `tasks_nuke_all_command`

### UNSAFE-NETWORK/EXFIL/DESTRUCTIVE/COST (explicit opt-in + extra warnings)
- `load_without_cors_command`
- `download_*`
- `media_file_delete_command`
- all `enqueue_*` generation commands
- credential + login + billing commands

---

## Tasks

### Task 1: Confirm scope + pick the next UNSAFE subset

**SubAgent:** `primary`
**Prompt:** Propose a concrete “UNSAFE supported list v1” based on the installed allowlist and Derrick’s risk tolerance. Group into readonly vs mutating vs cost/destructive. Provide rationale and any gotchas (token exposure, file paths, network).

**Status:** ⏳ Pending

---

### Task 2: Extend verifier for gate-on subset runs

**SubAgent:** `coder`
**Prompt:** In `gambit-artcraft` (`chip/artcraft-cli-buildfix`), extend `scripts/tools/verify_artcraft_cli_commands.py` to optionally run an approved UNSAFE subset with the gate enabled (e.g. `--unsafe-subset readonly`).
- Default remains non-destructive.
- When enabled, set `ARTCRAFT_ENABLE_UNSAFE_INVOKE=1` in the subprocess env.
- Ensure stdout remains JSON-only and command returns success JSON.
- Ensure logs and report redact or avoid printing task tokens if present.
Commit + push.

**Status:** ✅ Complete

**Results:**
- Added optional gate-on UNSAFE subset execution behind explicit flags:
  - `--run-unsafe-subset readonly` (alias `--unsafe-subset readonly`)
  - `--unsafe-gate-on` (required)
- Readonly subset v1 hardcoded:
  - `get_app_info_command`, `get_provider_order_command`, `get_task_queue_command`, `get_app_preferences_command`
- Commit: `debfbfcb8` — tools: allow optional readonly unsafe invoke subset behind gate
- Verified on installed `/usr/bin/artcraft`:
  - default mode passes
  - gate-on subset mode passes (all 4 commands OK)

---

### Task 3: Add OpenClaw-facing wrappers/examples for supported unsafe commands

**SubAgent:** `coder`
**Prompt:** In `openclaw-artcraft`:
- Update docs/examples to show safe usage of the supported UNSAFE subset via `tier='unsafe'`.
- Optionally add small typed helper functions (or example functions) for the readonly UNSAFE subset.
Commit + push.

**Status:** ✅ Complete

**Results:**
- Documented supported UNSAFE readonly subset across:
  - `skills/openclaw-artcraft/SKILL.md`
  - repo `README.md`
  - `packages/client/README.md`
- Added example: `packages/client/examples/unsafe_readonly_subset.py` (prints only summary fields).
- Commit: `b10afb4c3c2` — docs: document unsafe readonly subset + example (pushed to `main`).

---

### Task 4: Run expanded verification

**SubAgent:** `primary`
**Prompt:** Run the updated verifier against installed `/usr/bin/artcraft` in:
- default mode (gate off)
- gate-on readonly subset mode
Report pass/fail and any outputs that look sensitive.

**Status:** ✅ Complete

**Results:**
- Gate-off full roster run: PASS (47/47 clean JSON-only; unsafe commands return structured errors as expected).
  - Report: `/home/derrick/.openclaw/workspace-primary/verify_gate_off.json`
- Gate-on readonly subset run: PASS for:
  - `get_app_info_command`, `get_app_preferences_command`, `get_provider_order_command`, `get_task_queue_command`
  - Report (redacted): `/home/derrick/.openclaw/workspace-primary/verify_gate_on_readonly.json`

---

## Final Results

**Status:** ✅ Complete

**What We Built:**
- Defined supported UNSAFE readonly subset v1 and documented it in `openclaw-artcraft`.
- Extended `gambit-artcraft` verifier to optionally execute the UNSAFE readonly subset under gate-on, while keeping default mode non-destructive.

**Commits:**
- `openclaw-artcraft`:
  - `b10afb4c3c2` — docs: document unsafe readonly subset + example
- `gambit-artcraft`:
  - `debfbfcb8` — tools: allow optional readonly unsafe invoke subset behind gate

**Verified:**
- `/usr/bin/artcraft` passes:
  - `./scripts/tools/verify_artcraft_cli_commands.py --binary /usr/bin/artcraft`
  - `ARTCRAFT_ENABLE_UNSAFE_INVOKE=1 ./scripts/tools/verify_artcraft_cli_commands.py --binary /usr/bin/artcraft --run-unsafe-subset readonly --unsafe-gate-on`
