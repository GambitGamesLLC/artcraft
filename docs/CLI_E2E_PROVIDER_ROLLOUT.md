# ArtCraft CLI staged provider rollout checklist

_Date: 2026-03-12_

This document turns the command inventory and credential audit into a **provider-first manual rollout plan** Derrick can follow when provisioning test accounts.

Companion docs:

- `docs/CLI_E2E_CREDENTIAL_MATRIX.md`
- `docs/CLI_E2E_CREDENTIAL_HANDOFF.md`
- `.plans/2026-03-08-artcraft-canonical.md`

## Operator intent

Roll out ArtCraft CLI E2E in this order:

1. prove local-only and readonly local commands first
2. prove each provider's login/bootstrap capture path before spending anything
3. run readonly account/status checks next
4. fund and run the cheapest cost-bearing provider commands first
5. leave destructive reset, cleanup, and billing-mutation commands for the end

That keeps the early passes cheap, reversible, and practical for a human provisioning disposable accounts.

## Risk tiers

### Tier A — no-token-spend login/bootstrap

Use these first. They validate that ArtCraft can open the provider login flow and persist local session state.

- `open_sora_login_command`
- `midjourney_open_login_command`
- `grok_open_login_command`
- `worldlabs_open_login_command`

These should not intentionally consume model tokens or paid generation credits. They may still require:

- a real provider account
- network access
- browser/webview login completion

### Tier B — readonly account/status

Use these immediately after login/bootstrap succeeds.

**ArtCraft / Storyteller**
- `storyteller_get_credits_command`
- `storyteller_get_subscription_command`

**Sora**
- `check_sora_session_command`
- `sora_get_credential_info_command`

**Midjourney**
- `midjourney_get_credential_info_command`

**Grok**
- `grok_get_credential_info_command`

**WorldLabs**
- `worldlabs_get_credential_info_command`

**Low-risk local/global readonly helpers**
- `platform_info_command`
- `flip_image`
- `get_app_info_command`
- `get_app_preferences_command`
- `get_provider_order_command`
- `get_task_queue_command`

### Tier C — token/credit/billing-consuming or otherwise cost-bearing

Only run these after the relevant provider login and readonly checks pass.

**Low-cost network reads / estimate calls**
- `estimate_image_cost_command`
- `estimate_video_cost_command`
- `download_url_command`
- `load_without_cors_command`
- `download_media_file_command` (only if using a safe existing media token)

**ArtCraft / Storyteller generation**
- `enqueue_image_bg_removal_command`
- `enqueue_image_inpaint_command`
- `enqueue_text_to_image_command` (ArtCraft-backed model)
- `enqueue_edit_image_command` (ArtCraft-backed model)
- `enqueue_image_to_3d_object_command`
- `enqueue_image_to_video_command` (ArtCraft-backed model)

**Sora generation**
- `enqueue_text_to_image_command` (if routed to Sora-backed model)
- `enqueue_edit_image_command` (if routed to Sora-backed model)
- `enqueue_image_to_video_command` (Sora-backed model)

**Grok generation**
- `enqueue_text_to_image_command` (Grok-backed model)
- `enqueue_image_to_video_command` (Grok-backed model)

**Midjourney generation**
- `enqueue_text_to_image_command` (Midjourney-backed model)

**WorldLabs generation**
- `enqueue_image_to_gaussian_command`

**Billing / checkout entrypoints**
- `storyteller_open_credits_purchase_command`
- `storyteller_open_subscription_purchase_command`
- `storyteller_open_customer_portal_manage_plan_command`
- `storyteller_open_customer_portal_switch_plan_command`
- `storyteller_open_customer_portal_update_payment_method_command`

### Tier D — destructive reset / cleanup / billing mutation

Run these last, only against disposable local state and disposable remote fixtures.

**Local destructive resets**
- `tasks_nuke_all_command`
- `mark_task_as_dismissed_command` (local mutation, lower-risk but still stateful)
- `update_app_preferences_command`
- `set_provider_order_command`
- `sora_logout_command`
- `midjourney_clear_credentials_command`
- `grok_clear_credentials_command`
- `worldlabs_clear_credentials_command`
- `storyteller_purge_credentials_command`

**Remote destructive / billing-mutating**
- `media_file_delete_command`
- `storyteller_open_customer_portal_cancel_plan_command`

## Recommended operator rollout order

## Stage 0 — local-only smoke pass

Goal: verify the installed binary, local paths, and local state handling before touching any external account.

Run:

- `platform_info_command`
- `flip_image`
- `get_app_info_command`
- `get_app_preferences_command`
- `get_provider_order_command`
- `get_task_queue_command`
- `download_directory_reveal_command`

Then, only if using disposable local state:

- `update_app_preferences_command`
- `set_provider_order_command`
- `mark_task_as_dismissed_command`
- `tasks_nuke_all_command`

## Stage 1 — ArtCraft / Storyteller first

Goal: validate the app's own account before third-party providers.

Why first:

- it unlocks the safest credentialed readonly checks
- it tells Derrick whether the base ArtCraft account has credits/subscription state
- several later generation and billing flows depend on this session anyway

Run in order:

1. log into ArtCraft in-app
2. `storyteller_get_credits_command`
3. `storyteller_get_subscription_command`

Do **not** start with billing or generation yet.

## Stage 2 — no-spend provider login/bootstrap sweep

Goal: prove session capture and persistence for providers that rely on app-managed login.

Recommended order:

1. `open_sora_login_command`
   - follow with `check_sora_session_command`
   - then `sora_get_credential_info_command`
2. `midjourney_open_login_command`
   - follow with `midjourney_get_credential_info_command`
3. `grok_open_login_command`
   - follow with `grok_get_credential_info_command`
4. `worldlabs_open_login_command`
   - follow with `worldlabs_get_credential_info_command`

Why this order:

- Sora has the cleanest explicit session-check command, so it is the cheapest provider bootstrap to validate end-to-end.
- Midjourney and Grok are still login-first but stay in readonly territory after capture.
- WorldLabs is login-first too, but its bearer-token bridge is a bit more bespoke, so it belongs after the simpler login captures are proven.

## Stage 3 — readonly and low-cost network checks

Goal: validate networked reads before paid generation.

Run:

- `estimate_image_cost_command`
- `estimate_video_cost_command`
- `download_url_command`
- `load_without_cors_command`
- optionally `download_media_file_command` with a known-safe existing media token

These are the cheapest post-login commands to validate because they do not intentionally create provider jobs.

## Stage 4 — cheapest funded generation coverage

Goal: get one successful paid/provider-backed job per provider family, starting with the lowest expected spend and simplest cleanup.

Recommended order:

1. **ArtCraft cheapest available image generation/edit path**
   - start with the lowest-cost ArtCraft-backed `enqueue_text_to_image_command`
   - if needed, then `enqueue_edit_image_command`
2. **ArtCraft image utility generation**
   - `enqueue_image_bg_removal_command`
   - `enqueue_image_inpaint_command`
3. **Sora funded coverage**
   - cheapest Sora-backed enqueue path first
   - only move to `enqueue_image_to_video_command` after Sora session reuse is proven
4. **Grok funded coverage**
   - cheapest Grok-backed image path first
   - Grok video second
5. **Midjourney funded coverage**
   - one `enqueue_text_to_image_command` using a Midjourney-backed model
6. **WorldLabs funded coverage**
   - `enqueue_image_to_gaussian_command`
7. **Most expensive ArtCraft generation last**
   - `enqueue_image_to_3d_object_command`
   - ArtCraft-backed `enqueue_image_to_video_command`

Practical rule: only fund the next provider once the previous provider's login and readonly checks are green.

## Stage 5 — destructive cleanup and billing mutation

Goal: exercise cleanup and remote-mutation commands after useful fixtures already exist.

Run last:

**Credential cleanup**
- `sora_logout_command`
- `midjourney_clear_credentials_command`
- `grok_clear_credentials_command`
- `worldlabs_clear_credentials_command`
- `storyteller_purge_credentials_command`

**Local task/media cleanup**
- `tasks_nuke_all_command`
- `media_file_delete_command` only against disposable remote media fixtures

**Billing mutation**
- `storyteller_open_credits_purchase_command`
- `storyteller_open_subscription_purchase_command`
- `storyteller_open_customer_portal_manage_plan_command`
- `storyteller_open_customer_portal_switch_plan_command`
- `storyteller_open_customer_portal_update_payment_method_command`
- `storyteller_open_customer_portal_cancel_plan_command`

Billing commands should only be executed if Derrick deliberately wants billing-flow coverage for a disposable account.

## Cheapest commands to validate first

If Derrick wants the shortest path to confidence, do this mini-sequence first:

1. `platform_info_command`
2. `get_app_info_command`
3. `storyteller_get_credits_command`
4. `storyteller_get_subscription_command`
5. `open_sora_login_command`
6. `check_sora_session_command`
7. `sora_get_credential_info_command`
8. `midjourney_open_login_command`
9. `midjourney_get_credential_info_command`
10. `grok_open_login_command`
11. `grok_get_credential_info_command`
12. `worldlabs_open_login_command`
13. `worldlabs_get_credential_info_command`
14. `estimate_image_cost_command`
15. `estimate_video_cost_command`

That sequence maximizes signal while minimizing token burn, remote mutations, and cleanup burden.

## Provider summary

| Provider | Cheapest useful validation first | Cost-bearing later | Destructive last |
|---|---|---|---|
| ArtCraft / Storyteller | `storyteller_get_credits_command`, `storyteller_get_subscription_command` | ArtCraft-backed enqueue commands; checkout/subscription flows | `storyteller_purge_credentials_command`, `media_file_delete_command`, cancel-plan flow |
| Sora | `open_sora_login_command`, `check_sora_session_command`, `sora_get_credential_info_command` | Sora-backed enqueue commands, especially video | `sora_logout_command` |
| Midjourney | `midjourney_open_login_command`, `midjourney_get_credential_info_command` | Midjourney-backed text-to-image enqueue | `midjourney_clear_credentials_command` |
| Grok | `grok_open_login_command`, `grok_get_credential_info_command` | Grok-backed image/video enqueue | `grok_clear_credentials_command` |
| WorldLabs | `worldlabs_open_login_command`, `worldlabs_get_credential_info_command` | `enqueue_image_to_gaussian_command` | `worldlabs_clear_credentials_command` |

## Notes and exclusions

- FAL remains out of scope for this staged rollout until maintainers clarify whether the current CLI path still supports it.
- `worldlabs_receive_bearer_command` is treated as login-plumbing, not a first-pass operator workflow.
- Use only disposable remote media, billing fixtures, and provider accounts for Tier C and Tier D work.
