# gambit-artcraft

**Date:** 2026-03-12  
**Status:** Complete  
**Agent:** Chip 🐱‍💻

---

## Goal

Create a staged ArtCraft CLI E2E rollout plan grouped by provider, prioritizing lowest-risk / no-token-spend login-based validation first, then escalating to token/credit/billing-backed commands only after each provider’s basic auth flow is proven.

---

## Overview

We now know ArtCraft’s credentialed CLI paths are mostly app-managed login/session flows rather than generic `.env` injection. That means the cleanest rollout is provider-first: prove that each provider’s login/bootstrap commands work through the ArtCraft app, confirm ArtCraft persists and reuses the resulting session state correctly, and only then move into commands that consume credits, tokens, subscription value, or destructive fixtures.

The plan should make the operator path obvious. For each provider, we want to distinguish between: (1) login/account commands that should not spend tokens, (2) readonly account/network checks, (3) cost-bearing generation or billing commands, and (4) destructive reset/cleanup commands. That gives Derrick a clear onboarding path for test accounts: start with free/no-spend auth checks, then selectively fund only the providers whose deeper E2E coverage we actually want.

This plan belongs in `gambit-artcraft` because the provider command inventory, verifier docs, and auth/storage findings all live there.

---

## Tasks

### Task 1: Group commands by provider and spend/risk tier

**Bead ID:** `gambit-artcraft-94d`  
**SubAgent:** `primary`  
**Prompt:** In `/home/derrick/.openclaw/workspace/projects/gambit-artcraft`, use the existing command matrix and credential audit docs to group CLI commands by provider and by spend/risk tier. At minimum, identify: (a) no-token-spend login/bootstrap commands, (b) readonly account/status commands, (c) token/credit/billing-consuming commands, and (d) destructive reset/cleanup commands. Produce a provider-first staged checklist that Derrick can use for manual provisioning and test rollout.

**Folders Created/Deleted/Modified:**
- `/home/derrick/.openclaw/workspace/projects/gambit-artcraft/docs/`
- `/home/derrick/.openclaw/workspace/projects/gambit-artcraft/.plans/`

**Files Created/Deleted/Modified:**
- `/home/derrick/.openclaw/workspace/projects/gambit-artcraft/docs/CLI_E2E_PROVIDER_ROLLOUT.md`
- `/home/derrick/.openclaw/workspace/projects/gambit-artcraft/.plans/artcraft/2026-03-12-staged-provider-e2e-rollout-plan.md`

**Status:** ✅ Complete

**Results:**
- Added `docs/CLI_E2E_PROVIDER_ROLLOUT.md`, grouping the current CLI surface by provider and by risk tier: no-spend login/bootstrap, readonly account/status, cost-bearing generation/billing, and destructive reset/cleanup.
- Produced a provider-first staged checklist Derrick can follow for manual provisioning without mixing cheap bootstrap checks and funded/destructive flows.

---

### Task 2: Define the recommended rollout order for Derrick

**Bead ID:** `gambit-artcraft-coe`  
**SubAgent:** `primary`  
**Prompt:** Based on the grouped provider checklist, define the recommended operator rollout order. Prioritize providers/commands that only require account creation plus app-based OAuth/login and do not spend tokens first. Then stage readonly account checks, then cheapest cost-bearing commands, and finally destructive/billing-mutating commands. Make the rollout practical for a human provisioning test accounts.

**Folders Created/Deleted/Modified:**
- `/home/derrick/.openclaw/workspace/projects/gambit-artcraft/docs/`
- `/home/derrick/.openclaw/workspace/projects/gambit-artcraft/.plans/`

**Files Created/Deleted/Modified:**
- `/home/derrick/.openclaw/workspace/projects/gambit-artcraft/docs/CLI_E2E_PROVIDER_ROLLOUT.md`
- `/home/derrick/.openclaw/workspace/projects/gambit-artcraft/.plans/artcraft/2026-03-12-staged-provider-e2e-rollout-plan.md`

**Status:** ✅ Complete

**Results:**
- Defined the recommended operator rollout order: ArtCraft readonly first, then provider login/bootstrap (Sora, Midjourney, Grok, WorldLabs), then readonly/network-cost checks, then cheapest funded generation, then destructive/billing mutations last.
- Explicitly identified the shortest low-cost validation sequence so Derrick can get confidence before funding more providers.

---

### Task 3: Tie the staged provider rollout back into the canonical ArtCraft plan

**Bead ID:** `gambit-artcraft-cu4`  
**SubAgent:** `primary`  
**Prompt:** Update the canonical ArtCraft planning trail so the next execution flow is explicit: provider-first login/bootstrap validation, then readonly checks, then selective funded provider coverage. Make sure the new staged provider rollout is referenced from the canonical plan and from the credential handoff docs.

**Folders Created/Deleted/Modified:**
- `/home/derrick/.openclaw/workspace/projects/gambit-artcraft/.plans/`
- `/home/derrick/.openclaw/workspace/projects/gambit-artcraft/docs/`

**Files Created/Deleted/Modified:**
- `/home/derrick/.openclaw/workspace/projects/gambit-artcraft/.plans/2026-03-08-artcraft-canonical.md`
- `/home/derrick/.openclaw/workspace/projects/gambit-artcraft/docs/CLI_E2E_CREDENTIAL_HANDOFF.md`
- `/home/derrick/.openclaw/workspace/projects/gambit-artcraft/docs/CLI_E2E_PROVIDER_ROLLOUT.md`
- `/home/derrick/.openclaw/workspace/projects/gambit-artcraft/.plans/artcraft/2026-03-12-staged-provider-e2e-rollout-plan.md`

**Status:** ✅ Complete

**Results:**
- Linked the new staged provider rollout into the canonical ArtCraft plan so the next execution flow is explicit.
- Updated the credential handoff doc to point operators at the provider-first rollout checklist as the practical execution guide.

---

## Final Results

**Status:** ✅ Complete

**What We Built:**
- `docs/CLI_E2E_PROVIDER_ROLLOUT.md` — provider-first staged checklist that groups commands by no-spend login/bootstrap, readonly status, funded generation/billing, and destructive cleanup.
- Updated `docs/CLI_E2E_CREDENTIAL_HANDOFF.md` to reference the new rollout doc as the operator-facing execution guide.
- Updated `.plans/2026-03-08-artcraft-canonical.md` so the next ArtCraft execution pass explicitly follows provider-first login/bootstrap validation, then readonly checks, then selective funded coverage.

**Commits:**
- Not created in this subtask.

**Lessons Learned:**
- The cheapest useful confidence comes from validating session capture and readonly account checks before funding providers.
- Sora is the cleanest third-party bootstrap to validate first because it has an explicit session-check path, while WorldLabs belongs later because its bearer bridge is more bespoke.
- Billing and destructive coverage should stay separate from the initial provider rollout unless Derrick deliberately wants disposable billing fixtures.

---

*Completed on 2026-03-12*
