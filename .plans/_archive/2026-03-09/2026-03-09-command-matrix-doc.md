# ArtCraft — CLI Command Matrix Doc (docs/)

**Date:** 2026-03-09  
**Status:** Draft  
**Agent:** Chip 🐱‍💻

---

## Goal

Add a CLI command matrix document to our `gambit-artcraft` fork, placed under `docs/` (not repo root), capturing SAFE/UNSAFE `artcraft invoke` tiers and the higher-level CLI wrapper commands.

---

## Overview

We previously attempted to generate a `COMMAND_MATRIX.md`, but it didn’t materialize in the repo (likely a subagent/no-op). We’ll explicitly create the doc inside `docs/` to avoid polluting the repo root, and tie it to the current CLI contract:

- `artcraft invoke <tauri_command> [--payload <json>] [--unsafe] --json`
- strict SAFE tier (default) vs UNSAFE tier (requires `--unsafe` + gate)
- include `artcraft invoke --list-allowed --json`

---

## Tasks

### Task 1: Author doc in `gambit-artcraft/docs/`

**SubAgent:** `coder`
**Prompt:** In `~/.openclaw/workspace/projects/gambit-artcraft` on the current work branch (likely `chip/artcraft-cli-buildfix`), add a doc under `docs/` named `COMMAND_MATRIX.md` (or a better name if conventions exist) that:
- Explains the `artcraft invoke` contract and exit codes.
- Documents SAFE vs UNSAFE tiers (strict safe: `platform_info_command`, `flip_image`; all others UNSAFE).
- Shows `--list-allowed` example + sample output shape.
- Optionally includes a mapping table of “friendly” CLI wrapper commands (if any) → tauri commands.
Commit and push.

**Status:** ⏳ Pending

---

## Final Results

**Status:** ⏳ Pending
