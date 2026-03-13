# gambit-artcraft

**Date:** 2026-03-12  
**Status:** Complete  
**Agent:** Chip 🐱‍💻

---

## Goal

Figure out what API/OAuth/account state is required for each ArtCraft CLI command so we can prepare the right credentials, funding/billing prerequisites, and E2E test environment for full unsafe-command validation.

---

## Overview

The current ArtCraft verifier and command matrix already separate safe commands from unsafe ones, and they intentionally keep credentialed/network-cost work behind explicit latches. That was the right move for non-destructive verification, but it leaves a practical gap: before Derrick can provision real credentials and token/billing-backed test accounts, we need an exact inventory of which commands require what.

This work belongs in `gambit-artcraft` because the command inventory, safety tiers, verifier logic, and auth-related implementation all live there. The next step should be to audit the command matrix and implementation code, classify each unsafe command by the external dependency it needs, and turn that into a credential/OAuth readiness matrix. That matrix should tell us, command by command, whether the test needs: no credentials, provider login/session only, app-managed browser OAuth/account session, API token/bearer, manual secret file or environment injection, funded credits, subscription/billing state, or destructive test fixtures.

A critical part of the audit is determining how ArtCraft actually stores and retrieves credentials at runtime for each provider path: whether credentials are entered manually in-app, persisted in app preferences/storage, established via browser/OAuth login flows, or expected from environment variables / secret files such as `.env`. Only after that inventory exists should Derrick hand over credentials or load billing into test accounts. Secrets and real billing should not be embedded in plans or committed to git; the plan should instead document the credential slots, storage mechanism, and the safe manual provisioning sequence.

---

## Tasks

### Task 1: Audit command inventory and map external requirements

**Bead ID:** `gambit-artcraft-tpo`  
**SubAgent:** `primary`  
**Prompt:** In `/home/derrick/.openclaw/workspace/projects/gambit-artcraft`, inspect the authoritative command inventory and implementation paths (`docs/COMMAND_MATRIX.md`, `docs/COMMAND_VERIFICATION_MATRIX.md`, `scripts/tools/verify_artcraft_cli_commands.py`, and the CLI/invoke dispatcher code). Build a command-by-command classification of what each command requires for meaningful E2E validation: no credentials, local-only state, provider login/session, app-managed OAuth/browser auth, API token/bearer, network access, credits/tokens, paid subscription/billing state, queued job side effects, destructive fixture state, etc. Note uncertainties explicitly instead of guessing.

**Folders Created/Deleted/Modified:**
- `/home/derrick/.openclaw/workspace/projects/gambit-artcraft/.plans/`
- `/home/derrick/.openclaw/workspace/projects/gambit-artcraft/docs/`

**Files Created/Deleted/Modified:**
- `/home/derrick/.openclaw/workspace/projects/gambit-artcraft/docs/CLI_E2E_CREDENTIAL_MATRIX.md`
- `/home/derrick/.openclaw/workspace/projects/gambit-artcraft/.plans/artcraft/2026-03-12-credential-and-oauth-matrix-for-e2e-cli-testing.md`

**Status:** ✅ Complete

**Results:**
- Audited the invoke dispatcher allowlists, verifier subsets, and provider/auth/storage implementations.
- Added `docs/CLI_E2E_CREDENTIAL_MATRIX.md` with a command-by-command dependency matrix covering safe commands, local-state commands, credentialed account reads, provider-login flows, generation enqueue paths, destructive commands, and billing flows.
- Documented concrete runtime storage findings under `~/Artcraft/{credentials,settings,state,downloads,temp}` and identified the relevant persisted files used by Storyteller, Sora, Midjourney, Grok, and WorldLabs.
- Called out key uncertainties explicitly instead of guessing, especially around the partially plugin-owned Storyteller cookie persistence, the WorldLabs bearer bridge, and the currently unclear/stale-looking FAL CLI support.

---

### Task 2: Define credential slots and safe secret-handling workflow

**Bead ID:** `gambit-artcraft-sr2`  
**SubAgent:** `primary`  
**Prompt:** Based on the requirement audit, define the environment/credential slots we will eventually need for testing, without putting secrets into git. Document where each credential should live at runtime for each provider path (for example environment variables, secret `.env`-style files, app config/preferences storage, browser session, OAuth callback flow, or manual in-app login state), what account/billing setup Derrick needs to provide, and which commands/tests depend on each slot. Include a safe handoff checklist for later credential loading and test execution.

**Folders Created/Deleted/Modified:**
- `/home/derrick/.openclaw/workspace/projects/gambit-artcraft/docs/`
- `/home/derrick/.openclaw/workspace/projects/gambit-artcraft/.plans/`

**Files Created/Deleted/Modified:**
- `/home/derrick/.openclaw/workspace/projects/gambit-artcraft/docs/CLI_E2E_CREDENTIAL_HANDOFF.md`
- `/home/derrick/.openclaw/workspace/projects/gambit-artcraft/.plans/artcraft/2026-03-12-credential-and-oauth-matrix-for-e2e-cli-testing.md`

**Status:** ✅ Complete

**Results:**
- Added `docs/CLI_E2E_CREDENTIAL_HANDOFF.md` to define the credential slots Derrick will need later, where each one should live at runtime, and which commands depend on it.
- Recommended using interactive provider login and app-managed persistence for Storyteller, Sora, Midjourney, Grok, and WorldLabs instead of inventing env-var-based setup that the scanned code does not use.
- Documented the staged provisioning order: local-only first, then ArtCraft readonly-account checks, then provider login/bootstrap, then network-cost reads, then cost-bearing generation, then destructive/admin-style commands last.
- Included a concrete safety checklist for credential loading, generation runs, destructive fixture handling, and billing-flow precautions.

---

### Task 3: Update verifier/testing plan with explicit next execution steps

**Bead ID:** `gambit-artcraft-e8m`  
**SubAgent:** `primary`  
**Prompt:** Update the active planning trail in `gambit-artcraft` so the next execution step is explicit: once Derrick provides the required test credentials/accounts, run staged E2E validation by dependency class (readonly-account first, then network-cost/provider-specific generation flows, then destructive/admin-style commands only with deliberate fixtures). Tie the new credential matrix back to the canonical plan and note any upstream-maintainer response dependency if a digital twin/sandbox becomes available.

**Folders Created/Deleted/Modified:**
- `/home/derrick/.openclaw/workspace/projects/gambit-artcraft/.plans/`
- `/home/derrick/.openclaw/workspace/projects/gambit-artcraft/docs/`

**Files Created/Deleted/Modified:**
- `/home/derrick/.openclaw/workspace/projects/gambit-artcraft/docs/CLI_E2E_CREDENTIAL_MATRIX.md`
- `/home/derrick/.openclaw/workspace/projects/gambit-artcraft/docs/CLI_E2E_CREDENTIAL_HANDOFF.md`
- `/home/derrick/.openclaw/workspace/projects/gambit-artcraft/.plans/artcraft/2026-03-12-credential-and-oauth-matrix-for-e2e-cli-testing.md`

**Status:** ✅ Complete

**Results:**
- Updated the active planning trail to make the next execution step explicit: once Derrick provides the required credentials/accounts, run staged E2E in this order: readonly-account first, then provider-login/bootstrap validation, then readonly/network-cost calls, then generation flows, then destructive/admin/billing mutations last.
- Linked the plan output to the two durable docs in `docs/` so the next person can start from the audited matrix rather than re-reading auth code.
- Recorded the upstream dependency clearly: if maintainers provide a billing sandbox, digital twin, or clarify FAL support, that should shrink the current risk envelope and expand safe E2E coverage.

---

## Final Results

**Status:** ✅ Complete

**What We Built:**
- `docs/CLI_E2E_CREDENTIAL_MATRIX.md` — command-by-command auth, network, billing, local-state, and destructive-fixture matrix for the current `artcraft invoke` surface.
- `docs/CLI_E2E_CREDENTIAL_HANDOFF.md` — credential slot definitions, runtime storage expectations, staged provisioning order, and a safe execution checklist for later credentialed E2E work.
- Updated this active plan so the next execution step is explicit: wait for Derrick to provide the needed disposable accounts/sessions, then run staged E2E by dependency class rather than mixing readonly, costly, and destructive flows.

**Commits:**
- Not created in this subtask.

**Lessons Learned:**
- The current CLI verifier and allowlists are in good shape for non-destructive contract testing, but meaningful credentialed E2E coverage depends more on provider-specific local session capture than on environment variables.
- Storyteller, Sora, Midjourney, Grok, and WorldLabs each have distinct storage/reuse semantics, so a single generic "load secrets and run all tests" workflow would be brittle.
- FAL support needs upstream clarification before it should be included in the next credentialed CLI execution pass.

---

*Completed on 2026-03-12*
