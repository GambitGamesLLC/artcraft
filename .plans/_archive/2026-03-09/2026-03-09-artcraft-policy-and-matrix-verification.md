# ArtCraft Policy Decision + CLI Command Matrix Verification

**Date:** 2026-03-09  
**Status:** In Progress  
**Agent:** Chip 🐱‍💻

---

## Goal

Locate the relevant ArtCraft CLI policy + verification/matrix files (in repo + session memory), then proceed with the policy decision and continue verification work.

---

## Overview

Derrick wants to move forward on two threads:
1) The policy decision around which command subsets (safe/unsafe, readonly tiers, credential-latched tiers) should be supported.
2) Continuing work on the ArtCraft CLI command matrix + verification tooling.

First we’ll inventory: (a) the exact files in `projects/gambit-artcraft` that represent the current policy and verification system, and (b) the relevant handoff notes in `memory/*.md`.

---

## Tasks

### Task 0: Load the canonical ArtCraft plan and align

**SubAgent:** `primary`

**Prompt:**
Open and summarize `plans/artcraft/2026-03-08-artcraft-canonical.md`. Extract: current desired CLI contract, tiering policy, matrix/verification expectations, and any defined next steps that relate to policy decisions and verifier work.

**Files Modified:**
- None

**Status:** ✅ Complete

**Results:**
- Canonical CLI contract: `artcraft invoke <tauri_command> [--payload <json|@file>] [--unsafe] --json` + `--list-allowed`.
- Tiering policy: SAFE is small + read-only; UNSAFE is everything else and requires `--unsafe` AND unsafe gate (`ARTCRAFT_ENABLE_UNSAFE_INVOKE=1` or `~/.config/artcraft/cli.json`).
- Verification: `docs/COMMAND_VERIFICATION_MATRIX.md` + `scripts/tools/verify_artcraft_cli_commands.py` (default non-destructive; opt-in UNSAFE subsets: `readonly`, `readonly-network-cost`, credentialed `readonly-account`).
- Remaining work called out: upstream PR(s) + decide whether to split core CLI contract vs verifier tooling.

---

### Task 1: Pull relevant memory handoff snippets

**SubAgent:** `primary`

**Prompt:**
Search `~/.openclaw/workspace/memory/` for ArtCraft policy decision + command matrix verification references (e.g. COMMAND_MATRIX, COMMAND_VERIFICATION_MATRIX, verify_artcraft_cli_commands.py, safe/unsafe tiers, readonly subsets, allow-credentialed). Return a short list of the best memory files + the key bullet(s) we should use as handoff context.

**Files Modified:**
- None

**Status:** ✅ Complete

**Results:**
- `memory/2026-03-09.md`: SAFE/UNSAFE tiering; docs paths; verifier + subset names; notes that `/usr/bin/artcraft` passes verifier; policy decision handoff (readonly vs readonly-network-cost; credential-latched readonly-account). (Source: memory/2026-03-09.md)
- `memory/2026-03-08.md`: points to the canonical plan file; shows the gate contract (`ARTCRAFT_ENABLE_UNSAFE_INVOKE=1` or config file); “decide which additional commands to allowlist vs require unsafe gate.” (Source: memory/2026-03-08.md)
- `memory/2026-03-07-cli-plugin-test.md`: earlier wrapper-script phase + command mapping; notes about tauri-plugin-cli invocation syntax ambiguity (flag args vs JSON request object). (Source: memory/2026-03-07-cli-plugin-test.md)

---

### Task 2: Locate the relevant files in projects/gambit-artcraft

**SubAgent:** `primary`

**Prompt:**
In `~/.openclaw/workspace/projects/gambit-artcraft`, find and list the key files involved in:
- The CLI invoke policy tiers (SAFE/UNSAFE gating + allowlists)
- The command matrix docs
- The verification matrix docs
- The verifier script/tooling

Provide paths + a 1-line description for each file, plus any other files you discover that obviously control allowlists/tiers.

**Files Modified:**
- None

**Status:** ✅ Complete

**Results:**
- `docs/COMMAND_MATRIX.md`: human-facing list/classification of CLI-invokable commands.
- `docs/COMMAND_VERIFICATION_MATRIX.md`: expected contract + what the verifier asserts for SAFE/UNSAFE (including optional UNSAFE subsets).
- `scripts/tools/verify_artcraft_cli_commands.py`: deterministic verifier (default non-destructive; optional subset execution behind explicit flags).
- `crates/desktop/artcraft/src/core/cli/invoke_dispatcher.rs`: core implementation of `artcraft invoke ...` including SAFE/UNSAFE allowlists, `--list-allowed`, payload parsing + the unsafe gate (`ARTCRAFT_ENABLE_UNSAFE_INVOKE` or `~/.config/artcraft/cli.json`).
- `artcraft-cli/README.md` + `artcraft-cli/QUICK_REFERENCE.md`: user docs for CLI contract and safety model.
- `artcraft-cli.sh`: wrapper script that calls the binary and exposes convenience commands.
- `build-logs/2026-03-09_*invoke*smoketest*.log`: local smoke test transcripts (useful for regression expectations).

---

### Task 3: Policy decision (subsets + out-of-scope constraints)

**SubAgent:** `primary`

**Prompt:**
Based on the discovered files + memory handoff, decide the subset policy and verifier execution constraints.

**Files Modified:**
- None (decision only)

**Status:** ✅ Complete

**Results:**
- `readonly-account`: MUST remain an additional manual test only (requires auth). The verifier should keep it *skipped by default* unless an explicit credential latch flag is passed.
- Cost-incurring CLI tests (token consumption, network calls, etc.): **in scope**, but **manual-only** (explicit opt-in latches; never run by default/CI).
- Digital twin approach for cost/auth tests: out of scope for this workstream (Derrick will raise with maintainers).
- `readonly-network-cost`: treat as an additional manual test subset (may hit network and incur cost); require an extra latch flag and do not run in default verification.

---

## Final Results

**Status:** ⏳ Pending

---

## Follow-up Execution Tasks (queued)

### Task 5: Repo hygiene — ignore build logs / local noise

**SubAgent:** `coder`

**Prompt:**
In `/home/derrick/.openclaw/workspace/projects/gambit-artcraft` on branch `chip/artcraft-cli-buildfix`, update `.gitignore` to ignore local build logs and agent scratch files without hiding important artifacts.

Specifically, ignore:
- `build-logs/` (or at least `build-logs/*.log` and `build-logs/*.txt`)
- `.gastown-ignore`

Then:
- verify `git status` is clean (no untracked build-logs).
- commit + push to `origin/chip/artcraft-cli-buildfix`.

Return commit hash.

**Status:** ⏳ Pending

---

### Task 6: Upstreaming — prep PR(s) from chip/artcraft-cli-buildfix

**SubAgent:** `github`

**Prompt:**
Prepare upstream PR(s) from `GambitGamesLLC/artcraft` branch `chip/artcraft-cli-buildfix`.

Goals:
- Open at least one PR with the core CLI contract + safety model (invoke dispatcher, allowlists, gate, payload hardening).
- Optionally split a second PR for verifier tooling + docs (COMMAND_MATRIX / COMMAND_VERIFICATION_MATRIX / verify_artcraft_cli_commands.py).

Provide:
- PR titles + descriptions
- Links
- Any requested reviewer notes

Do NOT open PRs until Derrick explicitly confirms.

**Status:** ⏳ Pending

---

### Task 4: Update verifier + docs to reflect manual-only subsets

**SubAgent:** `coder`

**Prompt:**
In `/home/derrick/.openclaw/workspace/projects/gambit-artcraft` on the active CLI branch, implement the policy:

1) `readonly-account` remains credential-latched (already requires `--allow-credentialed`). Ensure the verifier clearly SKIPS it by default with a clear message.
2) Make `readonly-network-cost` an additional explicit latch (e.g., `--allow-network` or `--allow-cost`) so it is SKIPPED unless opted-in, because it may hit network and incur token/cost.
3) Update docs to match new verifier UX:
   - `docs/COMMAND_VERIFICATION_MATRIX.md`
   - `scripts/tools/verify_artcraft_cli_commands.py` (usage text and behavior)
   - (optional) `docs/COMMAND_MATRIX.md` note that network/cost subsets are manual-only

After changes:
- Run the verifier in default mode (should still be non-destructive).
- Run `--run-unsafe-subset readonly --unsafe-gate-on` as a smoke test.
- Do NOT run credentialed or network-cost subsets unless explicitly requested.
- Commit and push to the correct branch on origin.

Return: list of files changed + commit hash(es).

**Files Modified:**
- `projects/gambit-artcraft/scripts/tools/verify_artcraft_cli_commands.py`
- `projects/gambit-artcraft/docs/COMMAND_VERIFICATION_MATRIX.md`

**Status:** ✅ Complete

**Results:**
- Commit `f4b27eb95` pushed on `chip/artcraft-cli-buildfix`: adds `--allow-network/--allow-network-cost` latch and explicit SKIP messaging; updates verification matrix docs accordingly.

*Completed on 2026-03-09*
