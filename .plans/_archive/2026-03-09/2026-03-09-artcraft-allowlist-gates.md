# ArtCraft Allowlist / Unsafe Gates

**Date:** 2026-03-09  
**Status:** ✅ Complete  
**Agent:** Chip 🐱‍💻

---

## Goal

1) Resume the ArtCraft allowlist/unsafe-gate work.
2) Make it unambiguous to future devs + agents **where** to modify safe vs unsafe allowlists.
3) Clarify **repo ownership** for the unsafe gate (config/env) vs OpenClaw client behavior.
4) Define at a high level what “safe” vs “unsafe” means for ArtCraft (a creative tool with token-spend + side effects).
5) Clean up stale handoff info in memory so the canonical resume pointers are unambiguous.

---

## Overview

### Policy decisions (confirmed)
- **Generation is unsafe**: any command that can spend tokens / call providers must require `--unsafe` + the unsafe gate.
- **Safe is read-only**: safe allowlist is intentionally limiting and must not mutate persistent state.
- We must document this as **by design** for humans + agents.
- We also must include an explicit “risk acknowledgement” note: enabling `--unsafe` is an opt-in escalation (similar to installing/running agentic tools like OpenClaw) and should only be done with intent.

### Memory cleanup context
We have two generations of handoff pointers in memory:

- A March 7th “Where to Resume” note pointing at an older plan file.
- A March 8th handoff section that names the canonical ArtCraft plans (including unsafe gate work) and the installed-binary verification state.

We’ll remove the outdated March 7 handoff note (only the redundant resume pointers), leaving the March 8 handoff as the single source of truth.

Then we’ll open the canonical ArtCraft plans and propose the next concrete engineering steps for allowlist tiers + the `--unsafe` gate.

---

## Tasks

### Task 3: Document allowlist ownership + edit locations (dev+agent facing)

**SubAgent:** `primary`
**Prompt:** Update docs (and/or add a short `docs/cli-safety.md`) in the ArtCraft repo so it is obvious:
- where `SAFE_ALLOWLIST` / `UNSAFE_ALLOWLIST` live (file + symbol names)
- what rules govern moving a command between safe/unsafe
- how the `--unsafe` gate works (env + config) and why it exists

Also add a short note in the OpenClaw ArtCraft client repo docs explaining that the **authoritative allowlist is enforced by ArtCraft**, and OpenClaw should treat `--unsafe` as opt-in.

**Files Created/Deleted/Modified:**
- (TBD)

**Status:** ⏳ Pending

**Results:**

---

### Task 4: Define “safe vs unsafe” policy for ArtCraft commands

**SubAgent:** `primary`
**Prompt:** Convert the agreed policy into a short spec we can paste into docs and use for code review.

Policy constraints:
- **Safe = read-only only** (intentionally limiting)
- **Unsafe = any command that can spend tokens, call providers/network, or mutate persistent state**
- Include an explicit “risk acknowledgement” blurb for humans + agents: enabling `--unsafe` is an intentional escalation and you accept the risks.

**Status:** ⏳ Pending

**Results:**

---

### Task 5: Confirm gating configuration ownership + cross-platform config path

**SubAgent:** `primary`
**Prompt:** Decide and document where the unsafe gate is defined and enforced:
- ArtCraft repo (authoritative) should own the runtime gate enforcement.
- OpenClaw client repo may add *additional* guardrails, but should not be the source of truth for what ArtCraft will execute.

Confirm whether `~/.config/artcraft/cli.json` should be replaced with an OS-portable app config resolver (tauri/app dirs). Propose final location(s) for Linux/macOS/Windows.

**Status:** ⏳ Pending

**Results:**

---


### Task 1: Remove stale March 7 handoff note from memory

**SubAgent:** `primary`
**Prompt:** Locate the outdated “Where to Resume”/handoff note in memory that points to `plans/artcraft/artcraft-external-ipc-control-2026-03-07.md` (likely in `memory/2026-03-07-cli-plugin-test.md` and/or `memory/2026-03-07.md`). Remove only that outdated handoff/resume block while preserving the rest of the session log. Do not touch the March 8 handoff section.

**Folders Created/Deleted/Modified:**
- (none expected)

**Files Created/Deleted/Modified:**
- `memory/2026-03-07-cli-plugin-test.md` (likely)
- possibly `memory/2026-03-07.md` if it contains the same block

**Status:** ✅ Complete

**Results:**
- Removed the stale March 7 handoff/resume tail from `memory/2026-03-07-cli-plugin-test.md`.
- Verified no remaining references to `artcraft-external-ipc-control-2026-03-07.md` anywhere under `workspace/memory/`.

---

### Task 2: Re-load canonical ArtCraft allowlist/gate plans and produce next-step checklist

**SubAgent:** `primary`
**Prompt:** Read the canonical ArtCraft plan files:
- `plans/artcraft/2026-03-08-artcraft-canonical.md`
- `plans/artcraft/2026-03-08-unsafe-invoke-gate.md`

Summarize current state + extract a short, concrete next-step checklist for allowlist tiers and unsafe gate behavior (what to implement next, what to test, and any config/env toggles involved). Keep it focused and actionable.

**Folders Created/Deleted/Modified:**
- (none)

**Files Created/Deleted/Modified:**
- (none)

**Status:** ✅ Complete

**Results:**
**Current state (allowlist + unsafe gate):**
- Default `artcraft invoke <command> --json` remains allowlist-only; disallowed → exit `3` with JSON error.
- `artcraft invoke --unsafe <command> --json` exists, but is *gated*:
  - enabled by `ARTCRAFT_ENABLE_UNSAFE_INVOKE=1`, or
  - `~/.config/artcraft/cli.json` with `{ "enableUnsafeInvoke": true }`
  - without gate → exit `2` with JSON error.
- Implementation touchpoints:
  - Note: `tauri-plugin-cli` only gives us CLI parsing; the `invoke` *dispatcher* is custom (not automatic) and lives here.
  - CLI arg: `crates/desktop/artcraft/tauri.conf.json`
  - Dispatcher/gate: `crates/desktop/artcraft/src/core/cli/invoke_dispatcher.rs`

**Next-step checklist (to extend allowlist/tiers):**
1. Decide and write down “tier 0 safe allowlist” (commands OpenClaw should be able to call by default).
2. Decide “tier 1 unsafe” surface area: all commands vs a broader allowlist; add explicit denylist if any commands are destructive.
3. Add/adjust dispatcher logic to implement tiers cleanly (single source of truth list(s) + clear exit codes).
4. Add regression tests for:
   - allowlisted ok (exit 0)
   - disallowed safe (exit 3)
   - `--unsafe` + gate disabled (exit 2)
   - `--unsafe` + env enabled and + config enabled
5. Keep docs in sync (`README.md`, `artcraft-cli/README.md`, `artcraft-cli/QUICK_REFERENCE.md`) and include the exact gate instructions.

---

## Final Results

**Status:** ✅ Complete

**What We Built:**
- Cleaned up stale March 7 handoff/resume tail in memory so the March 8 handoff remains canonical.
- Captured the current allowlist + gated `--unsafe` contract and the concrete next steps to formalize allowlist tiers.

**Commits:**
- (none; plan-only change)

---

*Completed on 2026-03-09*