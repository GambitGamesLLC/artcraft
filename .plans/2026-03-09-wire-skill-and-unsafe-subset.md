# OpenClaw ↔ ArtCraft — Wire Skill + Unsafe Subset Verification

**Date:** 2026-03-09  
**Status:** In Progress  
**Agent:** Chip 🐱‍💻

---

## Goal

1) Wire ArtCraft capability into OpenClaw workflows by enabling/updating the `openclaw-artcraft` skill/docs and ensuring the recommended invocation surface is the tier-based client/CLI wrapper.  
2) Run a controlled verification subset of **UNSAFE** commands with the unsafe gate enabled (non-destructive only).

---

## Overview

We now have:
- Installed `/usr/bin/artcraft` enforcing strict SAFE/UNSAFE tiers and passing the verifier.
- `openclaw-artcraft` Python client on `main` with `tier=` + `list_allowed()` and JSON error-code mapping.
- Optional OpenClaw skill doc at `projects/openclaw-artcraft/skills/openclaw-artcraft/SKILL.md`.

“Wire up OpenClaw” here means:
- Update the skill doc to match current reality (tier-based escalation, list_allowed, correct command examples).
- Copy/symlink the skill into `~/.openclaw/workspace/skills/` so it is available to agents (note: may require restart/restore depending on how skills are loaded).

Then we’ll run a non-destructive UNSAFE subset with gate enabled.

---

## Tasks

### Task 1: Update + enable `openclaw-artcraft` skill

**SubAgent:** `coder`
**Prompt:**
1) Update `~/.openclaw/workspace/projects/openclaw-artcraft/skills/openclaw-artcraft/SKILL.md` to:
   - reference the tier model and `--list-allowed`.
   - prefer `tier="unsafe"` (Python client) / `openclaw-artcraft invoke --tier unsafe` over direct `--unsafe` usage.
   - update examples to real commands: `platform_info_command`, `flip_image`, `get_app_info_command` (unsafe).
2) Install/enable the skill for OpenClaw by copying or symlinking the folder into:
   - `~/.openclaw/workspace/skills/openclaw-artcraft`
3) Report what is required for it to take effect (restart gateway/session vs immediate).
4) Commit + push changes in `openclaw-artcraft` repo.

**Status:** ⏳ Pending

---

### Task 2: Unsafe subset verification (gate enabled; non-destructive)

**SubAgent:** `primary`
**Prompt:** With `ARTCRAFT_ENABLE_UNSAFE_INVOKE=1`, verify a small UNSAFE subset on installed `/usr/bin/artcraft` (no destructive/cost-incurring actions):
- `get_app_info_command`
- `get_provider_order_command`
- `get_task_queue_command` (read-only but may contain tokens; just ensure JSON-only stdout and no crash)
Also verify the same subset via Python client with `tier="unsafe"`.
Capture outputs and confirm all stdout is JSON-only.

**Status:** ⏳ Pending

---

## Final Results

**Status:** ⏳ Pending
