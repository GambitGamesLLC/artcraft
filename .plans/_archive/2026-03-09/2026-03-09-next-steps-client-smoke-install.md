# ArtCraft — Next Steps: Update call sites, install CLI, smoke test

**Date:** 2026-03-09  
**Status:** In Progress  
**Agent:** Chip 🐱‍💻

---

## Goal

1) Update any call sites to the new `openclaw-artcraft` client API (no `unsafe=`; use `tier=`).  
2) Update the local ArtCraft installation so the new strict SAFE/UNSAFE invoke tiers + `--list-allowed` are available.  
3) Run end-to-end smoke tests using the updated Python client against the installed `artcraft` binary.

---

## Overview

We landed:
- ArtCraft CLI changes in `gambit-artcraft` branch `chip/artcraft-cli-buildfix` (invoke tiers, list-allowed, JSON error codes, payload hardening).
- `openclaw-artcraft` Python client change on `main` (`3856ce2`): clean switch to `tier="safe"|"unsafe"`, plus `list_allowed()` and deterministic JSON error mapping.

Next we need to:
- ensure no downstream code still calls `unsafe=`
- rebuild/install ArtCraft locally from the branch so `/usr/bin/artcraft` matches what the client expects
- smoke test

---

## Tasks

### Task 1: Sweep & update call sites (remove `unsafe=`)

**SubAgent:** `coder`
**Prompt:** Search `~/.openclaw/workspace/projects/` for Python call sites using `ArtCraftClient.invoke(..., unsafe=...)` or similar. Update them to use `tier="unsafe"` instead. Also update any CLI wrappers/docs in `openclaw-artcraft` repo that mention `--unsafe` if removed. Run whatever unit tests are available for the affected repos. Commit + push changes (default branch unless repo policy says otherwise). Report a list of repos/files changed + commit hashes.

**Status:** ✅ Complete

**Results:**
- No downstream call sites using `unsafe=` were found.
- Docs updated in `openclaw-artcraft` to reflect tier escalation:
  - Commit `d57f8d0` — `docs: describe unsafe tier instead of --unsafe flag`

---

### Task 2: Rebuild + install ArtCraft locally (so testing hits new CLI)

**SubAgent:** `primary`
**Prompt:** In `~/.openclaw/workspace/projects/gambit-artcraft` on branch `chip/artcraft-cli-buildfix`, rebuild and install ArtCraft on this machine so `/usr/bin/artcraft` includes:
- strict SAFE tier (platform_info_command, flip_image)
- unsafe gate behavior
- `invoke --list-allowed --json`
- JSON error codes in error payloads

Then verify installed commit via:
- `artcraft invoke get_app_info_command --json` (likely UNSAFE; use gate + --unsafe if needed)
- and/or `artcraft invoke platform_info_command --json`

Note: this will require sudo (installing deb / package) and will replace the current installation.

**Status:** ✅ Complete

**Results:**
- Built `.deb` from `gambit-artcraft` `chip/artcraft-cli-buildfix` and installed it (user confirmed sudo install completed).
- Verified installed `/usr/bin/artcraft` now supports:
  - `invoke --list-allowed --json`
  - strict SAFE/UNSAFE enforcement
  - unsafe gate behavior
- Installed binary reports git commit short id: `3997197`.

---

### Task 3: End-to-end smoke test: Python client ↔ installed artcraft

**SubAgent:** `primary`
**Prompt:** Using the updated `openclaw-artcraft` Python client (main @ 3856ce2), run a smoke test against installed `/usr/bin/artcraft`:
- `client.list_allowed()`
- SAFE invoke (`platform_info_command`)
- UNSAFE invoke without gate should raise `UnsafeGateDisabled`
- UNSAFE invoke with gate enabled should succeed (pick a low-risk UNSAFE command like `get_app_info_command`)
Document exact commands/env used.

**Status:** ✅ Complete

**Results:**
- Verified installed CLI:
  - `/usr/bin/artcraft invoke --list-allowed --json` works.
  - SAFE `platform_info_command` succeeds.
  - `get_app_info_command` is blocked without `--unsafe` (exit 3, `unsafe_required`).
  - With `ARTCRAFT_ENABLE_UNSAFE_INVOKE=1` and `--unsafe`, it succeeds and reports commit `3997197`.
- Verified Python client (using package venv):
  - `client.list_allowed()` returns safe list `[platform_info_command, flip_image]` and gate false.
  - `client.invoke('platform_info_command')` succeeds.
  - `client.invoke('get_app_info_command', tier='unsafe')` raises `UnsafeGateDisabled` when gate off.
  - With `ARTCRAFT_ENABLE_UNSAFE_INVOKE=1`, unsafe invoke succeeds and returns `git_commit_short_id=3997197`.

---

## Final Results

**Status:** ✅ Complete

**What We Built:**
- Updated `openclaw-artcraft` Python client to tier-based API + `list_allowed()` (commit `3856ce2`) plus doc follow-up (commit `d57f8d0`).
- Built and installed ArtCraft .deb from `gambit-artcraft` so `/usr/bin/artcraft` now enforces strict SAFE/UNSAFE tiers and supports `invoke --list-allowed` (installed commit short id `3997197`).
- End-to-end smoke test confirmed Python client ↔ installed CLI behavior, including unsafe gate handling.
