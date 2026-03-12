# ArtCraft — Verify command list end-to-end

**Date:** 2026-03-09  
**Status:** In Progress  
**Agent:** Chip 🐱‍💻

---

## Goal

Go through the canonical ArtCraft CLI command list in the repo and verify each command behaves correctly (success where safe/possible; correct gating + structured errors otherwise).

---

## Overview

Now that `/usr/bin/artcraft` is updated to the strict SAFE/UNSAFE tier model and the Python client is updated, we want a repeatable verification pass across the command list documented in the repo.

Key constraints:
- Many commands are **UNSAFE** and/or cost-incurring (generation), credentialed, destructive, or side-effectful.
- For those, “verify” should typically mean:
  - correct **tier enforcement** (blocked without `--unsafe`, correct JSON error code)
  - correct **gate enforcement** (blocked without gate)
  - optional deeper verification only when credentials/funds are available and Derrick explicitly opts in.

---

## Tasks

### Task 1: Identify the canonical command list in repo root

**SubAgent:** `coder`
**Prompt:** In `~/.openclaw/workspace/projects/gambit-artcraft`, find the canonical command list Derrick refers to ("command list at the root of the repo"). This might be a README section, `artcraft-cli/QUICK_REFERENCE.md`, `artcraft-cli/README.md`, or a script. Report the exact file path and extract the command list.

**Status:** ✅ Complete

**Results:**
- Canonical tier policy doc: `docs/COMMAND_MATRIX.md` (SAFE: `platform_info_command`, `flip_image`; UNSAFE: everything else; discovery via `invoke --list-allowed`).
- Wrapper command entrypoints: `artcraft-cli.sh` and root `README.md` examples.
- Concrete roster is intended to be discovered via `artcraft invoke --list-allowed --json` (snapshot captured in build log).

---

### Task 2: Build a verification matrix + script

**SubAgent:** `coder`
**Prompt:** Using the discovered canonical command list, create a verification matrix and an automated verifier script (bash or python) that:
- Iterates commands.
- For SAFE commands: expects success JSON.
- For UNSAFE commands: expects `unsafe_required` when run without `--unsafe`.
- For UNSAFE with `--unsafe` but gate off: expects `unsafe_gate_disabled`.
- For UNSAFE with gate on: run only non-destructive, non-cost-incurring commands by default; for others, verify they return a structured error without crashing (and do not actually enqueue generation unless explicitly enabled).
- Records results to a log file + summary table.

Place script under `scripts/` or `tools/` in the repo (choose a sensible convention). Commit + push on the working branch.

**Status:** ✅ Complete

**Results:**
- Added verification matrix doc: `docs/COMMAND_VERIFICATION_MATRIX.md`
- Added automated verifier script: `scripts/tools/verify_artcraft_cli_commands.py`
- Commit: `a86a361bf` — Add automated CLI invoke contract verifier
- Note: running against `/usr/bin/artcraft` currently fails due to `flip_image` stdout contamination; will pass after `flip_image` is fixed.

---

### Task 3: Execute verification against installed `/usr/bin/artcraft`

**SubAgent:** `primary`
**Prompt:** Run the verifier script against the installed `/usr/bin/artcraft` and capture outputs. Produce a summary: pass/fail counts, first failures, and suggested fixes.

**Status:** ✅ Complete

**Results:**
- Full run against installed `/usr/bin/artcraft` using discovery `invoke --list-allowed` (47 total: SAFE 2, UNSAFE 45).
- Pass: 46 commands returned clean JSON-only stdout.
- Fail: 1 command returned JSON contaminated with extra stdout lines:
  - `flip_image` prints debug lines to stdout before the JSON payload.
- Artifacts:
  - Verifier: `/home/derrick/.openclaw/workspace-primary/artcraft_verify.py`
  - Report: `/home/derrick/.openclaw/workspace-primary/artcraft_verify_report.json`

---

## Final Results

**Status:** ⚠️ Partial

**What We Verified:**
- `sessions.list`-style full roster discovery via `artcraft invoke --list-allowed --json` and tier enforcement across 47 commands.

**Findings:**
- 46/47 commands emit clean JSON-only stdout in `--json` mode.
- 1/47 (`flip_image`) contaminates stdout with extra lines before the JSON response.

**Next Fix:**
- Patch `flip_image` command implementation to send debug output to stderr (or suppress in `--json` mode), so stdout is JSON-only.
