# OpenClaw ↔ ArtCraft Integration Repo + Client Plan

**Date:** 2026-03-09  
**Status:** In Progress  
**Agent:** Chip 🐱‍💻

---

## Goal

Turn `projects/openclaw-artcraft-client/` into an **umbrella integration repo** under `GambitGamesLLC/openclaw-artcraft`, and plan the implementation that talks to the installed `artcraft invoke ... --json` backend.

---

## Overview

We already have an installed ArtCraft CLI that exposes a safe-by-default `artcraft invoke <command> --json` interface, with an additional `--unsafe` tier gated by an env var or config file.

Decision: for now we are **Python-only** (no JS package) and we’ll keep the client **agent-agnostic**: OpenClaw is just one consumer.

This plan covers:
1) Repo naming + ownership decisions (now decided: `GambitGamesLLC/openclaw-artcraft`).
2) Repo bootstrap + structure (umbrella repo; likely `/packages/client` as the first package).
3) Client architecture for invoking ArtCraft commands, parsing JSON, and mapping errors/exit codes.
4) Allowlist strategy (safe tier vs unsafe tier) aligned with the ArtCraft CLI contract.

---

## Tasks

### Task 1: Read canonical ArtCraft plan + extract interface contract

**SubAgent:** `primary`
**Prompt:** Read `plans/artcraft/2026-03-08-artcraft-canonical.md` and summarize any decisions that affect the OpenClaw client integration (CLI contract, allowlists, expected command set, error/exit code handling, config paths).

**Files Created/Deleted/Modified:**
- (none)

**Status:** ✅ Complete

**Results:**
- Canonical plan finding: `tauri-plugin-cli` does **not** automatically provide an `artcraft cli invoke ...` command; it provides CLI match parsing (`cli_matches`). A custom dispatcher subcommand was required.
- For the client integration, the intended steady-state contract (per later shipped work) is:
  - `artcraft invoke <tauri_command_name> [--payload ...] --json`
  - Safe-by-default allowlist + optional `--unsafe` tier gated by either `ARTCRAFT_ENABLE_UNSAFE_INVOKE=1` or `~/.config/artcraft/cli.json`.
- Implication: treat ArtCraft as an external JSON CLI with strict exit-code semantics; parse stdout JSON only and surface stderr for diagnostics.

---

### Task 2: Decide GitHub repo name + scope

**SubAgent:** `primary`
**Prompt:** Propose repo naming options and rationale (avoid ambiguity with ArtCraft itself, room for future server/skill components). Recommend a final name and include a short README pitch.

**Files Created/Deleted/Modified:**
- (none)

**Status:** ✅ Complete

**Results:**
- Decision: use umbrella repo **`GambitGamesLLC/openclaw-artcraft`**
  - URL: https://github.com/GambitGamesLLC/openclaw-artcraft
- Initial structure assumption: migrate existing client code into `packages/client/`.
- Package name decision: **drop `-client`** → `@gambitgamesllc/openclaw-artcraft` (or unscoped `openclaw-artcraft` if we decide to go unscoped later).

---

### Task 3: Repo bootstrap + migrate existing client

**SubAgent:** `coder`
**Prompt:** Draft a repo bootstrap checklist for `openclaw-artcraft` umbrella repo: monorepo vs single package, folder layout (`packages/client`), package manager, build/test, lint, CI, release strategy, and how OpenClaw should consume it.

**Folders Created/Deleted/Modified:**
- `projects/openclaw-artcraft/`
- `projects/openclaw-artcraft/packages/client/`

**Files Created/Deleted/Modified:**
- `projects/openclaw-artcraft/.gitignore`
- `projects/openclaw-artcraft/README.md`
- `projects/openclaw-artcraft/.github/workflows/ci.yml`
- `projects/openclaw-artcraft/packages/client/**` (migrated from `projects/openclaw-artcraft-client/`)

**Status:** ✅ Complete

**Results:**
- Repo cloned via SSH: `git@github.com:GambitGamesLLC/openclaw-artcraft.git`
- Migrated existing Python client into `packages/client/` (excluded `venv/`, `.pytest_cache/`, `*.egg-info/`).
- Naming clarification: npm-style `@gambitgamesllc/openclaw-artcraft` is not valid for Python packaging.
  - Set Python distribution name to `openclaw-artcraft`.
  - Console script renamed to `openclaw-artcraft`.
  - Kept import module `artcraft_client` to avoid breaking code (optional future rename).
- Added CI to run `pytest` for the Python client; tests were updated to not require a real ArtCraft install (they create a fake `artcraft-cli.sh`).
- Local verify: `pip install -e ./packages/client[dev]` + `pytest`.
- Committed + pushed to `main`.

---

---

### Task 4: Implement Python client API + command mapping (real `artcraft invoke`)

**SubAgent:** `coder`
**Prompt:** Update the Python client in `projects/openclaw-artcraft/packages/client` to call the installed ArtCraft binary contract:
- default executable: `artcraft` (resolve via PATH) with override env var (e.g. `ARTCRAFT_BIN`) and/or constructor arg
- invoke form: `artcraft invoke <command> [--payload <json>] [--unsafe] --json`
- capture stdout/stderr, parse stdout JSON, and map exit codes into typed exceptions
- keep a simulation/fake-binary friendly test harness (tests should not require ArtCraft installed)
- update docs/examples to reflect `artcraft invoke` (not `artcraft-cli.sh`)

**Files Created/Deleted/Modified:**
- `projects/openclaw-artcraft/packages/client/artcraft_client/**`
- `projects/openclaw-artcraft/packages/client/tests/**`
- `projects/openclaw-artcraft/packages/client/README.md`

**Status:** ✅ Complete

**Results:**
- Implemented `ArtCraftClient.invoke(command, payload=None, unsafe=False, timeout=None) -> dict` calling:
  - `artcraft invoke <tauri_command> [--payload ...] [--unsafe] --json`
- Binary resolution order: constructor arg `artcraft_bin` → env `ARTCRAFT_BIN` → `artcraft` on PATH.
- stdout JSON parsing only; errors include stdout/stderr snippets for debugging.
- Exit-code → typed exception mapping:
  - timeout → `Timeout`
  - code `2` → `InvalidArgs` or `UnsafeGateDisabled` (stderr heuristic)
  - code `3` → `DisallowedCommand`
  - code `4` (and other unexpected) → `InvokeError`
- Tests are hermetic: they generate a fake `artcraft` executable and validate success + each error mapping + timeout.
- Docs/examples updated to reference `artcraft invoke ... --json`.
- Version bumped to `0.2.0`; pytest run clean.

---

---

### Task 5: Ship minimal optional OpenClaw Skill + on-demand `--help` strategy

**SubAgent:** `coder`
**Prompt:** Add a minimal OpenClaw Skill to the `openclaw-artcraft` repo that teaches agents how to use the ArtCraft CLI safely *without bloating every turn*.

Requirements:
- Put the skill in-repo under something like `skills/openclaw-artcraft/SKILL.md`.
- Keep SKILL.md body very short:
  - Contract: `artcraft invoke <command> [--payload ...] [--unsafe] --json`
  - Mention exit codes + unsafe gate concept at a high level.
  - Tell agents/humans to run `artcraft invoke --help` and/or `openclaw-artcraft --help` for details (on-demand).
  - Include 1-2 minimal examples.
- Add a short section to the repo README explaining how to enable the skill (copy/symlink into `~/.openclaw/workspace/skills/`).
- Do NOT add extra docs beyond what’s necessary.
- Commit + push.

**Files Created/Deleted/Modified:**
- `projects/openclaw-artcraft/skills/openclaw-artcraft/SKILL.md`
- `projects/openclaw-artcraft/README.md`

**Status:** ✅ Complete

**Results:**
- Added minimal optional skill at `skills/openclaw-artcraft/SKILL.md` (contract + exit codes + unsafe gate + on-demand help + 2 examples).
- Updated root README with “Optional OpenClaw Skill” section and copy/symlink instructions; noted skill loads next turn (typically no gateway restart).
- Committed + pushed.

---

---

## Final Results

**Status:** ✅ Complete

---

## Execution Notes / Assumptions

- Repo already exists on GitHub (per Derrick): https://github.com/GambitGamesLLC/openclaw-artcraft
- Next execution step will involve Git operations and pushing commits to GitHub (external action).

**What We Built:**
- `GambitGamesLLC/openclaw-artcraft` now contains the migrated Python client under `packages/client/`, with CI + docs.

**Commits:**
- `7c6adcc` — Bootstrap repo and migrate python client package
- `54f08ff` — Implement Python invoke client for ArtCraft CLI
- `ea3ddbe` — Add optional OpenClaw skill + on-demand help docs

*Draft created on 2026-03-09*
