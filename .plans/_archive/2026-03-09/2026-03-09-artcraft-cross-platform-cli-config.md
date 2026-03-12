# ArtCraft — Cross-Platform CLI Config Lookup (`cli.json`)

**Date:** 2026-03-09  
**Status:** Draft  
**Agent:** Chip 🐱‍💻

---

## Goal

Make the `--unsafe` gate config lookup for `cli.json` cross-platform (Linux/macOS/Windows) by resolving the OS-specific *app config directory* rather than hardcoding `~/.config/artcraft/cli.json`.

Also prepare clean breadcrumbs for a future upstream Issue + PR: clear change structure, rationale, and links to the new CLI safety docs.

---

## Overview

Right now, the unsafe gate supports:
- `ARTCRAFT_ENABLE_UNSAFE_INVOKE=1` (env), and
- Linux-centric config path: `~/.config/artcraft/cli.json`

We want a portable solution:
- Use a platform-appropriate config directory resolver (preferably whatever ArtCraft/Tauri already uses for app data/config paths; otherwise use a small Rust dependency like `directories`).
- Once the cross-platform solution is implemented, **remove** the Linux-only hardcoded path behavior (no legacy fallback / no archiving). The single canonical behavior should be the cross-platform app config dir resolver.

Upstream readiness:
- Keep the changes small and well-isolated (single module for config lookup).
- Update docs to point maintainers at `docs/cli-safety.md` and the allowlist constants.

---

## Tasks

### Task 1: Audit existing path/config patterns in ArtCraft

**SubAgent:** `primary`
**Prompt:** In `projects/gambit-artcraft`, search for existing config/app-data path utilities (Tauri path APIs, `app_config_dir`, `app_data_dir`, `directories` crate usage, etc.). Determine whether ArtCraft already has a standard location/pattern we should follow. Report findings with exact file paths and recommended approach.

**Files Modified:**
- (none)

**Status:** ⏳ Pending

---

### Task 2: Implement cross-platform config lookup for `cli.json`

**SubAgent:** `coder`
**Prompt:** On a new branch (name TBD, likely `chip/cli-config-cross-platform`), implement cross-platform lookup for the unsafe gate config.

Requirements:
- Canonical path: `<app_config_dir>/artcraft/cli.json` (naming consistent with existing ArtCraft conventions).
- Prefer using existing ArtCraft/Tauri path resolver if present.
- If none exists, introduce minimal dependency (e.g. `directories`) and keep it scoped.
- Keep env var gate behavior unchanged.
- **Remove** the Linux-only hardcoded lookup (`~/.config/artcraft/cli.json`). No fallback/legacy behavior once cross-platform is in place.

Update the docs added on branch `chip/cli-docs-safety` to reflect cross-platform behavior.

**Files Modified:**
- `crates/desktop/artcraft/src/core/cli/invoke_dispatcher.rs` (or the module where config lookup lives)
- possibly new helper module for config paths
- docs: `docs/cli-safety.md` (+ any linked docs)

**Status:** ⏳ Pending

---

### Task 3: Add tests / verification notes

**SubAgent:** `primary`
**Prompt:** Add a small test plan section (or automated tests if practical) verifying:
- config file is read from resolved app config dir
- env var still works
- safe/unsafe exit codes unchanged

If automated tests are hard (platform-specific dirs), provide a deterministic way to override config path for tests (env var like `ARTCRAFT_CLI_CONFIG_PATH`), but only if acceptable.

**Files Modified:**
- (TBD)

**Status:** ⏳ Pending

---

### Task 4: Prepare upstream Issue/PR outline (no submission yet)

**SubAgent:** `primary`
**Prompt:** Draft an `UPSTREAM_NOTES.md` (or section in docs) that explains:
- What the CLI feature is
- Safe vs unsafe policy (read-only safe; generation unsafe)
- Where allowlists live (`SAFE_INVOKE_ALLOWLIST`, `UNSAFE_INVOKE_ALLOWLIST`)
- How unsafe gating works (env + config)
- Cross-platform config path rationale
- Links to `docs/cli-safety.md`

Do not create the GitHub Issue/PR yet; just prepare the text we’ll paste later.

**Files Modified:**
- `UPSTREAM_NOTES.md` (new) or similar

**Status:** ⏳ Pending

---

## Final Results

**Status:** ⏳ Draft

---

*Created on 2026-03-09*