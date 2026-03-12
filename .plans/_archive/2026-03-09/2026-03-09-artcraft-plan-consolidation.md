# ArtCraft Plan Consolidation (Single Canonical Reference)

**Date:** 2026-03-09  
**Status:** Complete  
**Agent:** Chip 🐱‍💻

---

## Goal

Inventory all ArtCraft-related plan documents in `workspace/plans/` and in ArtCraft-related project repos/folders (including the CLI docs area), then converge on **one canonical plan** we can reference going forward.

---

## Overview

We already have a strong candidate canonical document: `plans/artcraft/2026-03-08-artcraft-canonical.md`.

This task is about:
- Finding all other ArtCraft plans/notes that look plan-like.
- Deciding what to archive vs merge into the canonical.
- Ensuring there is exactly one “start here” document.

We will avoid destructive deletes; default to **archiving** (moving) older plans into an `_archive/` folder, and updating the canonical plan with a short “supersedes” list.

---

## Tasks

### Task 1: Inventory ArtCraft plan documents across workspace + projects

**SubAgent:** `primary`

**Prompt:**
Inventory all ArtCraft-related planning documents in:

1) `/home/derrick/.openclaw/workspace/plans/` (especially `plans/artcraft/` but also any other subfolders containing ArtCraft plans)
2) ArtCraft-related project repos under `/home/derrick/.openclaw/workspace/projects/` including:
   - `gambit-artcraft/`
   - `openclaw-artcraft/`
   - any other repo/folder with `artcraft` in the name

Find:
- Markdown files that are clearly plans (in `plans/` or other plan-like folders)
- Any “canonical plan” references
- Any doc files that are acting like a plan (e.g., ROADMAP-style docs, or `docs/` files with plan structure)

Output:
- A categorized list of files (path + short description)
- Recommendation: which single file should be the canonical “start here”, and which files should be archived vs merged into the canonical.

Do not modify files.

**Status:** ✅ Complete

**Results:**
- Canonical “start here” recommendation: `plans/artcraft/2026-03-08-artcraft-canonical.md`
- Other plan-like workspace files found (high level):
  - `plans/artcraft/*` (many one-off execution plans + policy/matrix plans)
  - `plans/openclaw-artcraft/*` + `plans/openclaw-artcraft-client/*`
  - `plans/gambit-artcraft/*`
  - `plans/handoff/*`
- Repo docs that function as canonical specs (keep in repos; don’t treat as start-here plans):
  - `projects/gambit-artcraft/docs/COMMAND_MATRIX.md`
  - `projects/gambit-artcraft/docs/COMMAND_VERIFICATION_MATRIX.md`
  - `projects/gambit-artcraft/scripts/tools/verify_artcraft_cli_commands.py`
  - `projects/openclaw-artcraft/packages/client/README.md`
  - `projects/openclaw-artcraft/skills/openclaw-artcraft/SKILL.md`
- Actionable notes:
  - Canonical plan has a small inconsistency: it claims `artcraft-external-ipc-control-2026-03-07.md` was hard-deleted, but it exists at `plans/gambit-artcraft/artcraft-external-ipc-control-2026-03-07.md` — we should reconcile this during consolidation.

---

### Task 2: Consolidate into one canonical plan (archive + merge)

**SubAgent:** `primary`

**Prompt:**
Using the inventory + recommendation, prepare an execution proposal:
- Create `plans/artcraft/_archive/` if needed.
- Move (archive) superseded ArtCraft plans.
- Update the chosen canonical plan with:
  - a short index of what was archived
  - any key missing context merged in (minimal; link out where possible)

Do not execute moves/edits until Derrick confirms.

**Status:** ✅ Complete

**Results:**
- Created archive dir: `plans/artcraft/_archive/2026-03-09/`
- Moved all non-canonical ArtCraft plans from `plans/artcraft/` into the archive folder.
- Updated the canonical plan with a clear “Start Here” index and corrected the prior hard-delete note.

---

## Final Results

**Status:** ✅ Complete

**What We Built:** One canonical ArtCraft plan at `plans/artcraft/2026-03-08-artcraft-canonical.md`, with all other ArtCraft plans archived under `plans/artcraft/_archive/2026-03-09/`.

*Completed on 2026-03-09*
