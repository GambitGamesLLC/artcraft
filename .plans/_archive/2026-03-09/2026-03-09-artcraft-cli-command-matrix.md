# ArtCraft — CLI Command Matrix + Workflow

**Date:** 2026-03-09  
**Status:** Draft  
**Agent:** Chip 🐱‍💻

---

## Goal

1) Create a canonical **CLI Command Matrix** doc that enumerates all ArtCraft `#[tauri::command]` commands and classifies them as **SAFE (read-only)** vs **UNSAFE** (token spend / network / mutation / sensitive access) per our policy.
2) Make the canonical ArtCraft plan aware of the matrix-driven workflow for adding + testing commands.
3) Write a clear handoff note to memory so we can start a fresh session and execute the command-add/test workflow.

---

## Overview

We have a working CLI invoke dispatcher with:
- SAFE allowlist (read-only)
- UNSAFE allowlist + `--unsafe` + gate (env/config)

However, only a small subset of commands are currently wired into the dispatcher. The matrix will become the authoritative checklist for systematically exposing commands, bootstrapping required state, and adding tests without expanding risk accidentally.

---

## Tasks

### Task 1: Create `COMMAND_MATRIX.md` in the ArtCraft repo

**SubAgent:** `primary`
**Prompt:** In `/home/derrick/.openclaw/workspace/projects/gambit-artcraft` on branch `chip/cli-config-cross-platform`:
- Generate a markdown doc `COMMAND_MATRIX.md` (or `docs/cli-command-matrix.md` if you prefer) that:
  - Enumerates all `#[tauri::command]` functions (at least those under `crates/desktop/artcraft/src/core/commands/`).
  - For each command, include:
    - command name (string the CLI would use)
    - module/file path
    - SAFE/UNSAFE classification (per policy)
    - reason (read-only vs mutation vs network/token spend, etc.)
    - payload schema requirements (if any) and how CLI payload should map
    - required state bootstrapping (e.g., TaskDatabase, ProviderPriorityStore, credentials managers)
    - test strategy (unit/integration/smoke), including which tests can be run without spending tokens.
    - current status: wired in dispatcher? smoke-tested? (yes/no)
  - Add a short “Workflow” section: the step-by-step process for adding one command at a time + tests.
- Keep it concise but complete enough to drive implementation.
- Commit and push to `origin/chip/cli-config-cross-platform`.

**Status:** ⏳ Pending

---

### Task 2: Update canonical ArtCraft plan to reference the matrix workflow

**SubAgent:** `primary`
**Prompt:** Update `plans/artcraft/2026-03-08-artcraft-canonical.md` to include:
- Link to the matrix doc path in repo
- The matrix-driven workflow (add command → classify → wire → bootstrap state → test)
- Explicit reminder: safe is read-only; generation is unsafe; unsafe requires `--unsafe` + gate.

Commit these plan changes in the orchestration repo (`~/.openclaw`) if applicable.

**Status:** ⏳ Pending

---

### Task 3: Write memory handoff note

**SubAgent:** `primary`
**Prompt:** Update today’s memory file with a handoff note:
- What’s shipped so far (branches/commits)
- Where the matrix doc lives + branch name
- Next steps to start in the new session (pick next commands to wire, start with safe read-only commands, etc.)

**Status:** ⏳ Pending

---

## Final Results

**Status:** ⏳ Draft

---

*Created on 2026-03-09*