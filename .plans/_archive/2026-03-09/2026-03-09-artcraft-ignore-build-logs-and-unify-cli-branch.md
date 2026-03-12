# ArtCraft — Ignore build-logs + Unify CLI Branch

**Date:** 2026-03-09  
**Status:** In Progress  
**Agent:** Chip 🐱‍💻

---

## Goal

1) Ensure `build-logs/*` stay untracked and are ignored in `gambit-artcraft`.
2) Unify all CLI safety + config changes under `chip/cli-config-cross-platform`.
3) Clarify upstream PR scope: CLI feature changes vs Linux build fixes.
4) Run a post-unify smoke test.

---

## Overview

We currently have multiple related branches in `projects/gambit-artcraft`:
- `chip/artcraft-cli-buildfix` — implements allowlist + gated `--unsafe` behavior.
- `chip/cli-docs-safety` — adds policy docs + allowlist constant names.
- `chip/cli-config-cross-platform` — cross-platform config path docs/code + upstream notes.

We will:
- Add `build-logs/` to `.gitignore` and verify no log artifacts are tracked.
- Verify `chip/cli-config-cross-platform` contains (or is merged with) the docs-safety work and upstream notes.
- Keep Linux build fixes (Nx/RPM/deb bundling) as separate upstream Issues/PRs unless strictly required for this CLI feature.

---

## Tasks

### Task 1: Ignore build logs and keep repo clean

**SubAgent:** `primary`
**Prompt:** In `/home/derrick/.openclaw/workspace/projects/gambit-artcraft`:
- Confirm `build-logs/*` are untracked.
- Add `build-logs/` to `.gitignore` (root) so logs stay untracked.
- Ensure we do not accidentally add/commit any build-logs.
- Commit this change onto `chip/cli-config-cross-platform` and push.

**Status:** ⏳ Pending

---

### Task 2: Unify branches under `chip/cli-config-cross-platform`

**SubAgent:** `primary`
**Prompt:** In `gambit-artcraft`:
- Check whether `chip/cli-config-cross-platform` already contains `chip/cli-docs-safety`.
- If not, merge/rebase so the unified branch contains:
  - policy docs + allowlist constants
  - upstream notes
  - cross-platform config path wording (no Linux-only docs)
- Push the unified branch.

**Status:** ⏳ Pending

---

### Task 3: Smoke test after unify

**SubAgent:** `primary`
**Prompt:** After unifying:
- Run a quick smoke test (using the built binary or installed `artcraft`, depending on what’s available) validating exit codes:
  - safe allowlisted command succeeds (e.g. `platform_info_command`)
  - disallowed safe command returns exit 3
  - `--unsafe` without gate returns exit 2
  - `--unsafe` with env var gate returns exit 0
- Record the exact commands + outputs in a short `SMOKETEST.md` (or append to `UPSTREAM_NOTES.md` if more appropriate).
- Commit smoke test notes to the unified branch.

**Status:** ⏳ Pending

---

## Final Results

**Status:** ⏳ In Progress

---

*Created on 2026-03-09*