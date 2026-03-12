# ArtCraft: Sync Upstream + Fix sqlite_tasks Build/Install Bug

**Project:** GambitGamesLLC/artcraft (fork)  
**Date:** 2026-03-08  
**Status:** In Progress  
**Agent:** Chip 🐱‍💻  
**Overseer:** Derrick

---

## Goal

Sync our fork (`gambit-artcraft`) with upstream changes, then patch the local build/install blocker (sqlite_tasks compile errors) in a feature branch so ArtCraft builds successfully and we can test the external CLI work.

---

## Overview

We currently have uncommitted work in `projects/gambit-artcraft/` (tauri CLI plugin integration + wrapper scripts + examples). Separately, the repo is blocked from building by `sqlite_tasks` compilation errors (reported as ~24 errors in the prior plan).

This plan:
1) preserves the current WIP changes onto a branch,
2) updates the fork’s `main` with upstream,
3) reproduces + fixes the `sqlite_tasks` build errors on a new branch based on the synced `main`,
4) layers the CLI WIP commits onto the fixed build branch so we can run real end-to-end tests.

We will **not** open issues/PRs yet (per your request). We will push branches to our fork so we can collaborate and later PR upstream.

---

## Tasks

### Task 1: Snapshot current WIP changes into a branch

**SubAgent:** `coder`  
**Prompt:**
- In `projects/gambit-artcraft`, create a branch (e.g., `chip/wip-artcraft-cli`) from current `origin/main`.
- Commit all current local modifications + new files (CLI plugin changes, scripts, examples, test app).
- Ensure working tree is clean afterward.

**Files/Folders (expected):**
- Modified: `Cargo.lock`, `crates/desktop/artcraft/Cargo.toml`, `crates/desktop/artcraft/src/lib.rs`, `crates/desktop/artcraft/tauri.conf.json`, `frontend/package-lock.json`
- New: `artcraft-cli.sh`, `artcraft-cli/`, `examples/`, `test-cli-plugin/`

**Status:** ✅ Complete

**Results:**
- Created branch: `chip/wip-artcraft-cli`
- Committed all current WIP (CLI plugin + wrapper script + examples + test app)
- Commit: `9b73f1904` - `WIP: add CLI plugin, wrapper script, examples, and client integration`
- Working tree left clean afterwards

---

### Task 2: Sync fork `main` with upstream

**SubAgent:** `coder`  
**Prompt:**
- Inspect git remotes in `projects/gambit-artcraft`.
- Ensure an `upstream` remote exists pointing at the upstream ArtCraft repo.
- Fetch upstream.
- Bring `main` up-to-date with `upstream/main` (prefer fast-forward; otherwise merge commit; avoid rebase if it would rewrite shared history).
- Push updated `main` to `origin`.
- Report any conflicts/resolutions.

**Status:** ✅ Complete

**Results:**
- Added missing `upstream` remote: `git@github.com:storytold/artcraft.git`
- Fetched upstream and confirmed `main` already matched `upstream/main` (no-op fast-forward; already up to date)
- `origin/main` already up to date; no merge commit created
- `main` commit: `0651afd6f`
- No conflicts

---

### Task 3: Reproduce sqlite_tasks build failure on synced main

**SubAgent:** `coder`  
**Prompt:**
- Create branch `chip/fix-sqlite_tasks-build` from synced `main`.
- Run the normal build path (`./script/artcraft/unix_build.sh`) and capture the exact error output.
- Identify the root cause (feature flags, sqlx offline data, missing migrations, crate version mismatch, etc.).

**Status:** ✅ Complete

**Results:**
- Failure log captured at `build-logs/sqlite_tasks-failure.txt`
- Root cause: SQLx macro validation was hitting a live SQLite DB without migrated schema (`no such table: tasks`), causing cascading compile errors

---

### Task 4: Patch sqlite_tasks build errors

**SubAgent:** `coder`  
**Prompt:**
- Implement minimal local code/config changes to fix the build.
- Keep the patch tight and well-documented in commit message(s).
- Re-run `./script/artcraft/unix_build.sh` until clean.
- Push branch `chip/fix-sqlite_tasks-build` to origin.

**Status:** ⚠️ Partial (fix pushed; full build still needs final verification)

**Results:**
- Branch created + pushed: `chip/fix-sqlite_tasks-build`
- Commit: `501e64410`
- Changes:
  - `crates/schema/database/sqlite_tasks/build.rs`: set `SQLX_OFFLINE=true` so SQLx uses offline metadata instead of requiring a migrated live SQLite DB at compile time
  - `script/artcraft/unix_build.sh`: fixed config path from missing `tauri.artcraft_3d.no_dev.conf.json` to `tauri.conf.json`
  - Added docs: `docs/dev/sqlite_tasks-build-fix.md`
  - Added build log: `build-logs/sqlite_tasks-failure.txt`
- Verification:
  - `cargo check -p sqlite_tasks` confirmed passing
  - `unix_build.sh` progressed further but was not fully confirmed end-to-end within the time window

---

### Task 4.5: Verify full ArtCraft build on `chip/fix-sqlite_tasks-build`

**SubAgent:** `coder`  
**Prompt:**
- Checkout `chip/fix-sqlite_tasks-build`.
- Run `./script/artcraft/unix_build.sh` to completion.
- Capture output to `build-logs/unix_build-fix-sqlite_tasks-build.txt`.
- If it fails, keep iterating with minimal additional fixes (each as a separate commit) until it succeeds.

**Status:** ⏳ Pending

---

### Task 5: Create an integration branch that includes both build fix + CLI work

**SubAgent:** `coder`  
**Prompt:**
- Create branch `chip/artcraft-cli-buildfix` from `chip/fix-sqlite_tasks-build`.
- Merge/cherry-pick commits from `chip/wip-artcraft-cli` onto it.
- Resolve conflicts if any.
- Build again.
- Push `chip/artcraft-cli-buildfix`.

**Status:** ⏳ Pending

---

### Task 6: Smoke-test real CLI invocation (post-build)

**SubAgent:** `coder`  
**Prompt:**
- With a working build on `chip/artcraft-cli-buildfix`, verify the CLI plugin is actually invocable (even just `--help` / minimal command) from the built binary.
- Document the exact commands used + results.

**Status:** ⏳ Pending

---

## Verification Checklist

- [x] `projects/gambit-artcraft` clean working tree on all branches after commits
- [x] `main` updated to match upstream changes and pushed
- [ ] `./script/artcraft/unix_build.sh` succeeds on `chip/fix-sqlite_tasks-build` (pending final end-to-end confirmation)
- [ ] `./script/artcraft/unix_build.sh` succeeds on `chip/artcraft-cli-buildfix`
- [ ] Basic CLI invocation path is proven against the real built binary (not simulation)

---

## Final Results (to fill in)

**Status:** ⏳

**Branches:**
- `chip/wip-artcraft-cli`
- `chip/fix-sqlite_tasks-build`
- `chip/artcraft-cli-buildfix`

**Commits:**
- _TBD_

---

*Created: 2026-03-08*
