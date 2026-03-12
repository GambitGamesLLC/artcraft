# ArtCraft Docs: Reality Sync (gambit-artcraft + openclaw-artcraft)

**Date:** 2026-03-09  
**Status:** Complete  
**Agent:** Chip 🐱‍💻

---

## Goal

Update documentation across the ArtCraft repos so it reflects current reality:
- `projects/gambit-artcraft` (CLI contract, verifier, latches)
- `projects/openclaw-artcraft` (client + skill docs that integrate with the CLI contract)

---

## Overview

Recent changes that must be reflected consistently:
- Canonical CLI contract: `artcraft invoke <command> [--payload <json|@file>] [--unsafe] --json`
- SAFE vs UNSAFE tiering + unsafe gate: `ARTCRAFT_ENABLE_UNSAFE_INVOKE=1` or `~/.config/artcraft/cli.json`.
- `invoke --list-allowed --json` output shape.
- Verifier supports optional UNSAFE subsets, now with explicit manual-only latches:
  - `readonly-network-cost` requires `--allow-network`/`--allow-network-cost`
  - `readonly-account` requires `--allow-credentialed`
  - these should never run by default/CI.

---

## Tasks

### Task 1: Update docs in `projects/gambit-artcraft`

**SubAgent:** `coder`

**Prompt:**
In `/home/derrick/.openclaw/workspace/projects/gambit-artcraft` on branch `chip/artcraft-cli-buildfix`, audit and update any docs that are out of date relative to the current CLI contract and verifier behavior.

Focus files:
- `README.md` (CLI automation section)
- `artcraft-cli/README.md`
- `artcraft-cli/QUICK_REFERENCE.md`
- `docs/COMMAND_MATRIX.md`
- `docs/COMMAND_VERIFICATION_MATRIX.md`

Ensure examples mention the current latches for manual subsets (`--allow-network`/`--allow-network-cost`, `--allow-credentialed`) and do not imply those subsets run by default.

After editing:
- run a quick grep to ensure old flag names aren’t referenced.
- commit + push to `origin/chip/artcraft-cli-buildfix`.

Return commit hash + files changed.

**Status:** ✅ Complete

**Results:**
- Commits pushed to `chip/artcraft-cli-buildfix`:
  - `3fd658e1a` — update `README.md` “CLI automation” examples to match real SAFE tier + `--list-allowed`.
  - `7d30aeba2` — sync CLI wrapper docs + verifier latch docs (see below).
- `7d30aeba2` updated:
  - `artcraft-cli/README.md` (wrapper vs binary usage; fixed `invoke` arg ordering; corrected UNSAFE wrapper example)
  - `artcraft-cli/QUICK_REFERENCE.md` (added correct wrapper invoke example + ordering note)
  - `docs/COMMAND_VERIFICATION_MATRIX.md` (mentions both `--allow-network` and `--allow-network-cost`; example uses `--allow-network-cost`)

---

### Task 2: Update docs in `projects/openclaw-artcraft` (client + skill)

**Status:** ✅ Complete

**Results:**
- Commit `0205e6b` pushed to `main`.
- Updated: root `README.md`, `packages/client/README.md`, `skills/openclaw-artcraft/SKILL.md`.
- Tests: `pytest -q packages/client/tests` (13 passed).

<!-- duplicate task block removed during plan cleanup -->

---

## Final Results

**Status:** ✅ Complete

**What We Built:** Documentation across both repos now matches the shipped CLI contract, safety model, and verifier latch behavior.

**Commits:**
- `projects/gambit-artcraft` (`chip/artcraft-cli-buildfix`): `3fd658e1a`, `7d30aeba2`
- `projects/openclaw-artcraft` (`main`): `0205e6b`

*Completed on 2026-03-09*
