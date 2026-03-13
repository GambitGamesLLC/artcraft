# ArtCraft CLI E2E credential handoff plan

_Date: 2026-03-12_

This document defines the **credential slots** Derrick will need to provision later for staged ArtCraft CLI E2E testing, without storing secrets in git.

Companion docs:

- `docs/CLI_E2E_CREDENTIAL_MATRIX.md`
- `docs/CLI_E2E_PROVIDER_ROLLOUT.md`

## Ground rules

- Do **not** commit secrets, cookies, bearer tokens, refresh tokens, payment details, or exported browser storage.
- Prefer runtime provisioning into the local app data directory (`~/Artcraft`) or interactive in-app login where the code already expects it.
- If a temporary `.env`-style file is ever used for local setup notes, keep it outside git or gitignored and do not teach tests to print it.
- Run destructive/billing tests only against disposable fixtures.

## Credential slots Derrick will need

| Slot | Provider / scope | What Derrick needs to provide | Where it should live at runtime | Commands/tests that depend on it |
|---|---|---|---|---|
| `ARTCRAFT_SESSION` | ArtCraft / Storyteller | A real ArtCraft login session for the test account | Main app webview/Tauri HTTP cookie jar; mirrored into Storyteller credential manager / cookie files under `~/Artcraft/credentials/` | `storyteller_get_credits_command`, `storyteller_get_subscription_command`, ArtCraft-backed enqueue commands, billing commands, `media_file_delete_command`, `storyteller_purge_credentials_command` |
| `ARTCRAFT_BILLING_FIXTURE` | ArtCraft / Stripe | Test account state: either no plan, disposable paid plan, or disposable credits depending on scenario | Remote account state on Storyteller/Stripe; not a local secret file | subscription purchase / portal commands; credit-consuming ArtCraft generation tests |
| `SORA_SESSION` | Sora / OpenAI | A Sora-capable OpenAI account that can log in interactively | Sora login webview -> `~/Artcraft/credentials/sora_*` | `open_sora_login_command`, `check_sora_session_command`, `sora_get_credential_info_command`, `sora_logout_command`, Sora-backed enqueue commands |
| `MIDJOURNEY_SESSION` | Midjourney | A Midjourney-capable account that can log in via webview and still submit jobs | Midjourney login webview -> persisted local state in `~/Artcraft/credentials/` | `midjourney_open_login_command`, `midjourney_get_credential_info_command`, `midjourney_clear_credentials_command`, Midjourney text-to-image enqueue |
| `GROK_SESSION` | Grok / xAI | A Grok-capable account able to authenticate in the login webview | Grok login webview -> `grok_state.json` / `grok_cookies.txt`; runtime may upgrade to fuller secrets | `grok_open_login_command`, `grok_get_credential_info_command`, `grok_clear_credentials_command`, Grok image/video enqueue |
| `WORLDLABS_SESSION` | WorldLabs | A WorldLabs-capable account able to complete login in the webview | WorldLabs login webview plus JS handoff -> `worldlabs_state.json`, bearer, refresh, cookies under `~/Artcraft/credentials/` | `worldlabs_open_login_command`, `worldlabs_get_credential_info_command`, `worldlabs_clear_credentials_command`, `enqueue_image_to_gaussian_command` |
| `WORLDLABS_BRIDGE_FIXTURE` | WorldLabs bridge testing only | Optional test harness payload that imitates the login JS bearer/refresh handoff | passed only at runtime to `worldlabs_receive_bearer_command`; do not store in repo | `worldlabs_receive_bearer_command` focused tests |
| `MEDIA_FIXTURES` | ArtCraft media tokens / remote fixtures | Safe media files owned by the test ArtCraft account, intended for download/delete/transform workflows | Remote ArtCraft account plus local downloads/temp dirs during tests | `download_media_file_command`, `media_file_delete_command`, image/video/object/gaussian enqueue commands |
| `DOWNLOAD_FIXTURES` | Public network | Stable public URLs and local writable download dir | runtime payload + local downloads dir | `download_url_command`, `load_without_cors_command` |
| `TASK_DB_FIXTURES` | Local app state | Seeded or naturally created local task rows suitable for dismiss/nuke tests | `~/Artcraft/state/tasks_v*.sqlite` | `get_task_queue_command`, `mark_task_as_dismissed_command`, `tasks_nuke_all_command` |
| `LOCAL_PREFS_FIXTURE` | Local app state | Writable local preferences/provider-order state | `~/Artcraft/settings/app_preferences.json`, `provider_preferences.json` | `get_app_preferences_command`, `update_app_preferences_command`, `get_provider_order_command`, `set_provider_order_command`, `download_directory_reveal_command` |
| `FAL_API_KEY` | FAL (uncertain current CLI support) | Only if maintainers confirm current CLI-backed FAL testing is still intended | likely `~/Artcraft/credentials/fal_api_key.txt` if backend support is restored/confirmed | currently **uncertain**; keep out of scope until clarified |

## Recommended staged provisioning order

Use `docs/CLI_E2E_PROVIDER_ROLLOUT.md` as the operator-facing checklist. The condensed order here stays aligned with that provider-first rollout.

### Stage 0: local-only smoke setup

Provision nothing secret yet.

Use this stage to validate:

- `platform_info_command`
- `flip_image`
- `get_app_info_command`
- `get_app_preferences_command`
- `get_provider_order_command`
- `get_task_queue_command`
- `update_app_preferences_command`
- `set_provider_order_command`
- `mark_task_as_dismissed_command`
- `tasks_nuke_all_command`
- `download_directory_reveal_command`

### Stage 1: readonly account credentials

Provision only ArtCraft login/session first.

Use it for:

- `storyteller_get_credits_command`
- `storyteller_get_subscription_command`

This matches the verifier's existing `readonly-account` subset and is the safest first credentialed milestone.

### Stage 2: provider-login capture

Individually provision provider accounts and validate the login/bootstrap commands before any generation:

- `open_sora_login_command` -> `check_sora_session_command` -> `sora_get_credential_info_command`
- `midjourney_open_login_command` -> `midjourney_get_credential_info_command`
- `grok_open_login_command` -> `grok_get_credential_info_command`
- `worldlabs_open_login_command` -> `worldlabs_get_credential_info_command`

Do **not** start with costly generation. First prove that each provider's local credential capture and storage path works.

### Stage 3: readonly/network-cost but non-destructive network calls

Provision no new secret types beyond Stage 1; just allow network.

Use for:

- `estimate_image_cost_command`
- `estimate_video_cost_command`
- `download_url_command`
- `load_without_cors_command`
- optionally `download_media_file_command` if a safe media token is available

### Stage 4: generation providers, cheapest-first

Run generation E2E by provider dependency and expected blast radius:

1. ArtCraft readonly-ish / cheapest generation paths first
2. Sora / Grok / Midjourney / WorldLabs provider-specific generation after login capture is verified
3. only one provider/model family at a time
4. save resulting task/media tokens for later destructive tests

Suggested order:

1. ArtCraft-backed enqueue using lowest-cost model available
2. Sora-backed enqueue
3. Grok-backed enqueue
4. Midjourney text-to-image
5. WorldLabs gaussian/world creation
6. more expensive ArtCraft video/object models last

### Stage 5: destructive/admin-style commands

Run only after disposable fixtures exist:

- `media_file_delete_command`
- `grok_clear_credentials_command`
- `midjourney_clear_credentials_command`
- `sora_logout_command`
- `storyteller_purge_credentials_command`
- `worldlabs_clear_credentials_command`
- `tasks_nuke_all_command`
- billing portal commands that can change or cancel subscription state

## What should live where

### Prefer interactive login over manual secret injection

For these providers, the code clearly expects **interactive login state** plus local persistence managed by the app:

- ArtCraft / Storyteller
- Sora
- Midjourney
- Grok
- WorldLabs

That means the safest path is:

1. launch ArtCraft in a disposable local profile
2. use the provider's login command/window
3. let the app write the resulting local credential files under `~/Artcraft/credentials/`
4. run the corresponding readonly verification command before any generation

### Local settings should be fixture-driven, not hand-edited in git

Use local runtime state under `~/Artcraft/settings/` and `~/Artcraft/state/` for:

- app preferences
- provider order
- task DB fixtures
- window state

Do not add seeded real-user state to the repo.

### Avoid env vars unless the code truly expects them

In the scanned ArtCraft desktop CLI path, the credentialed provider flows are mostly **cookie/session storage based**, not environment-variable based.

So for now:

- do **not** invent env-var-based setup for Storyteller/Sora/Midjourney/Grok/WorldLabs
- reserve env vars or secret files only for future cases where upstream code explicitly supports them
- keep FAL out of scope until maintainers confirm whether current CLI support is still active

## Safe handoff checklist for later execution

Before running credentialed E2E:

- [ ] use a clean local `~/Artcraft` state or back up the current one
- [ ] confirm `ARTCRAFT_ENABLE_UNSAFE_INVOKE=1` only for the test shell/session that needs it
- [ ] confirm no secrets/tokens are echoed by helper scripts
- [ ] confirm local logs are not being copied into git commits
- [ ] confirm destructive tests are pointed at disposable local fixtures and disposable remote accounts/assets

For ArtCraft readonly account checks:

- [ ] log into ArtCraft in-app
- [ ] run `storyteller_get_credits_command`
- [ ] run `storyteller_get_subscription_command`
- [ ] record whether the test account has free credits, paid credits, or subscription entitlements

For provider bootstrap checks:

- [ ] Sora login captured and `sora_get_credential_info_command` returns expected state
- [ ] Midjourney login captured and `midjourney_get_credential_info_command` shows email/user state
- [ ] Grok login captured and `grok_get_credential_info_command` shows stored state
- [ ] WorldLabs login captured and `worldlabs_get_credential_info_command` shows bearer-derived identity if available

Before generation:

- [ ] verify which exact model/provider combination is under test
- [ ] verify expected account billing/quota for that provider
- [ ] verify a safe output fixture path and note resulting task/media tokens
- [ ] run only one cost-bearing scenario at a time

Before destructive commands:

- [ ] create or identify disposable media/task/account fixtures
- [ ] verify that deleting/canceling them will not affect a real working account
- [ ] run destructive commands last
- [ ] capture resulting local state for audit, then clean it up

## Explicit open questions for maintainers / upstream

1. Is FAL still intended to be testable through the current CLI invoke path?
2. If yes, where are the active Rust implementations for FAL key set/get in the current desktop app?
3. Is there an upstream sandbox or digital twin for Stripe/billing and provider-cost flows?
4. Is `worldlabs_receive_bearer_command` intended to remain a testable public bridge, or only an internal login-plumbing command?

Until those are answered, keep FAL and billing-mutation coverage behind explicit human review.
