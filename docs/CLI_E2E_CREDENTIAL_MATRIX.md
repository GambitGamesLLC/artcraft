# ArtCraft CLI E2E credential + dependency matrix

_Date: 2026-03-12_

This document audits the current `artcraft invoke` command inventory and maps what each command needs for **meaningful** E2E validation, not just contract-level invocation.

Authoritative inventory sources reviewed:

- `docs/COMMAND_MATRIX.md`
- `docs/COMMAND_VERIFICATION_MATRIX.md`
- `scripts/tools/verify_artcraft_cli_commands.py`
- `crates/desktop/artcraft/src/core/cli/invoke_dispatcher.rs`
- provider/auth/storage code under `crates/desktop/artcraft/src/core/state/**` and `crates/desktop/artcraft/src/services/**`

## Runtime storage findings

ArtCraft's default app data root is the user's home directory under `~/Artcraft` (via `AppDataRoot::create_default()`), with these important subdirectories:

- `~/Artcraft/credentials/`
- `~/Artcraft/settings/`
- `~/Artcraft/state/`
- `~/Artcraft/downloads/`
- `~/Artcraft/temp/`

Relevant persisted state paths found in code:

### Credential / session storage

- `credentials/grok_state.json`
- `credentials/grok_cookies.txt`
- `credentials/midjourney_state.json`
- `credentials/worldlabs_state.json`
- `credentials/worldlabs_cookies.txt`
- `credentials/worldlabs_bearer.txt`
- `credentials/worldlabs_refresh.txt`
- `credentials/sora_cookies.txt`
- `credentials/sora_bearer_token.txt`
- `credentials/sora_sentinel_token_store.json`
- `credentials/artcraft_avt.txt`
- `credentials/artcraft_session.txt`
- `credentials/fal_api_key.txt` (path exists in code; current CLI setter/getter was **not** found in the scanned dispatcher/command implementations)

### Local settings / state

- `settings/app_preferences.json`
- `settings/provider_preferences.json`
- `state/tasks_v*.sqlite`
- `state/window_size.json`
- `state/window_position.json`

## How credentials are actually acquired and reused

### Storyteller / ArtCraft account

ArtCraft account state is cookie/session based, not env-var based in the scanned code.

- Runtime source: cookies from the main Tauri HTTP plugin cookie jar.
- Sync path: `persist_storyteller_cookies_task.rs` reads `visitor` and `session` cookies for `https://api.storyteller.ai/` and pushes them into `StorytellerCredentialManager`.
- Purge path: `storyteller_purge_credentials_command` clears the main webview cookies and deletes persisted copies on disk.
- Billing and account-read commands call Storyteller endpoints with the current session credentials.

### Sora

Sora auth is an app-managed login-webview flow, followed by local credential persistence.

- Acquisition: `open_sora_login_command` -> Sora login webview -> `sora_login_thread` extracts cookies, upgrades/renews session, then writes via `SoraCredentialManager`.
- Storage: Sora cookies + bearer/sentinel state under `~/Artcraft/credentials/`.
- Reuse: Sora generation paths call `maybe_upgrade_or_renew_session(...)` and may overwrite stored credentials with renewed values.

### Midjourney

Midjourney auth is a browser/webview login flow with cookie capture and user-info lookup.

- Acquisition: `midjourney_open_login_command` -> login webview -> `midjourney_login_window_thread` captures cookies, calls Midjourney `get_user_info`, stores cookies + cached user info.
- Storage: `midjourney_state.json` plus related cookie/user-info state handled by `MidjourneyCredentialManager`.
- Reuse: image generation requires both auth cookies **and** cached user info containing a usable user id.

### Grok

Grok auth starts as a browser/webview cookie capture and may be upgraded into fuller API credentials.

- Acquisition: `grok_open_login_command` -> login webview -> `grok_login_window_thread` persists cookies.
- Upgrade: generation code can call `request_client_secrets(...)` using cookies and then persist a fuller credential bundle.
- Storage: `grok_state.json` / `grok_cookies.txt`.
- Reuse: Grok image/video flows need either upgraded full credentials or at least cookies that can be upgraded at runtime.

### WorldLabs

WorldLabs currently uses a browser/webview login flow plus JS token extraction, not a classical OAuth callback.

- Acquisition: `worldlabs_open_login_command` opens login webview.
- Token bridge: the login thread injects JS (`WORLDLABS_JAVASCRIPT_EXPORT_BEARER_TOKENS`) and reads bearer + refresh tokens from `WorldlabsBearerBridge`.
- Persistence: login thread stores cookies, bearer token, and refresh token via `WorldlabsCredentialManager`.
- Storage: `worldlabs_state.json`, `worldlabs_cookies.txt`, `worldlabs_bearer.txt`, `worldlabs_refresh.txt`.
- `worldlabs_receive_bearer_command` is part of that bridge; by itself it is not a full user-facing login substitute.

### FAL

The codebase contains `fal_api_key.txt` support and several enqueue error paths that mention `NeedsFalApiKey`, but in the scanned current CLI dispatcher and enqueue routing:

- no `get_fal_api_key_command` / `set_fal_api_key_command` implementation was found in `crates/desktop/artcraft/src`
- the current invoke dispatcher allowlists do **not** include FAL credential management commands
- the current scanned CLI enqueue routing did **not** show active `GenerationProvider::Fal` branches for the exposed invoke commands

Treat FAL-backed CLI E2E requirements as **unconfirmed / probably stale until maintainers clarify current intended support**.

## Command-by-command matrix

Legend used below:

- **None** = no credential/account requirement
- **Local state** = requires only filesystem/app state
- **Credentialed account** = requires a logged-in account/session
- **Provider login** = requires provider webview/browser login state
- **Network** = meaningful E2E should hit network
- **Cost/billing** = may consume credits, subscription entitlements, or paid provider quota
- **Destructive** = mutates/deletes state and needs deliberate fixtures

| Command | Meaningful E2E dependency class | Requirements for meaningful E2E validation | Runtime storage / state touched | Notes |
|---|---|---|---|---|
| `platform_info_command` | None; local readonly | none | none | SAFE tier |
| `flip_image` | None; local transform | valid base64 image payload | none | SAFE tier |
| `check_sora_session_command` | Provider login; network readonly | valid Sora session already stored | Sora credential files in `credentials/` | verifies Sora session health; may renew session |
| `download_directory_reveal_command` | Local state | existing downloads dir / prefs | `settings/app_preferences.json`, downloads dir | desktop/UI side effect only |
| `download_media_file_command` | Network + local write | valid media token resolvable by Storyteller/CDN | downloads dir, app prefs | code does not pass Storyteller creds here; token validity still required |
| `download_url_command` | Network + local write | reachable URL | downloads dir, app prefs | public URL fetch; no provider creds |
| `enqueue_edit_image_command` | Credentialed generation + queue side effects | provider/model-specific: ArtCraft session for ArtCraft models; Sora session for Sora models; valid source images; network | task DB, provider creds, downloads/temp as needed | can consume credits/quota and creates queued tasks |
| `enqueue_image_bg_removal_command` | Credentialed generation + queue side effects | current scanned path is ArtCraft-only; requires ArtCraft/Storyteller session and network | task DB, Storyteller session | likely credit-consuming |
| `enqueue_image_inpaint_command` | Credentialed generation + queue side effects | current scanned path is ArtCraft-only; requires ArtCraft/Storyteller session, source image + mask, network | task DB, Storyteller session | likely credit-consuming |
| `enqueue_image_to_3d_object_command` | Credentialed generation + queue side effects | current scanned path is ArtCraft object generation; requires ArtCraft/Storyteller session, source image, network | task DB, Storyteller session | error types still mention FAL API key; active invoke routing looked ArtCraft-only in scanned code |
| `enqueue_image_to_gaussian_command` | Provider login + queue side effects | current scanned meaningful path is WorldLabs Marble; requires WorldLabs cookies + bearer + refresh, plus valid source image token/network | task DB, WorldLabs credential files, downloads dir/temp | provider-side world creation side effects |
| `enqueue_image_to_video_command` | Credentialed generation + queue side effects | provider/model-specific: ArtCraft session for ArtCraft video models, Grok login/full creds for Grok Video, Sora session for Sora2 | task DB, provider creds, temp/downloads | cost-bearing and provider-specific |
| `enqueue_text_to_image_command` | Credentialed generation + queue side effects | provider/model-specific: ArtCraft session, Midjourney cookies+user id, Grok creds/session, or Sora session depending on model/provider | task DB, provider creds | can consume credits/quota |
| `estimate_image_cost_command` | Network readonly | network access to Storyteller cost-estimate API; no creds in code | none | verifier already classifies this as `readonly-network-cost` |
| `estimate_video_cost_command` | Network readonly | network access to Storyteller cost-estimate API; no creds in code | none | verifier already classifies this as `readonly-network-cost` |
| `get_app_info_command` | Local readonly | none | app root, prefs-derived download dir | good first unsafe smoke test |
| `get_app_preferences_command` | Local readonly | none | `settings/app_preferences.json` | |
| `get_provider_order_command` | Local readonly | none | `settings/provider_preferences.json` | |
| `get_task_queue_command` | Local readonly | existing task DB (or empty DB) | `state/tasks_v*.sqlite` | reads local queue state only |
| `grok_clear_credentials_command` | Destructive local credential reset | existing Grok credential fixture | Grok credential files | should run only against disposable local state |
| `grok_get_credential_info_command` | Provider login readonly | existing Grok cookies/full creds | Grok credential files | returns email if present |
| `grok_open_login_command` | Provider login bootstrap | interactive browser/webview login; reachable Grok/X/IdP endpoints | Grok credential files after successful login | browser/session side effects |
| `load_without_cors_command` | Network readonly | reachable target URL | none | unrestricted fetch helper; no auth in code |
| `mark_task_as_dismissed_command` | Local mutating | existing task fixture in local SQLite DB | `state/tasks_v*.sqlite` | local-only mutation |
| `media_file_delete_command` | Credentialed destructive remote mutation | logged-in ArtCraft/Storyteller session and a deletable media token owned by test account | Storyteller session; remote account data | destructive; requires deliberate remote fixture |
| `midjourney_clear_credentials_command` | Destructive local credential reset | existing Midjourney credential fixture | Midjourney credential files | disposable local fixture only |
| `midjourney_get_credential_info_command` | Provider login readonly | stored Midjourney cookies/user info | Midjourney credential files | meaningful E2E should confirm email/user-info capture |
| `midjourney_open_login_command` | Provider login bootstrap | interactive Midjourney webview login and any upstream IdP | Midjourney credential files | captures cookies and user info |
| `open_sora_login_command` | Provider login bootstrap | interactive Sora/OpenAI login | Sora credential files | upgrades/renews and persists credentials |
| `set_provider_order_command` | Local mutating | valid provider list payload | `settings/provider_preferences.json` | local-only mutation |
| `sora_get_credential_info_command` | Provider login readonly | stored Sora session state | Sora credential files | can surface email from JWT if present |
| `sora_logout_command` | Destructive local credential reset | existing Sora credential fixture | Sora credential files | disposable local fixture only |
| `storyteller_get_credits_command` | Credentialed account readonly | logged-in ArtCraft/Storyteller session; network | Storyteller session/cookies | verifier already classifies this as `readonly-account` |
| `storyteller_get_subscription_command` | Credentialed account readonly | logged-in ArtCraft/Storyteller session; network | Storyteller session/cookies | verifier already classifies this as `readonly-account` |
| `storyteller_open_credits_purchase_command` | Credentialed billing flow | logged-in ArtCraft session; network; Stripe/browser availability | Storyteller session; billing webview | opens checkout; can lead to real purchase |
| `storyteller_open_customer_portal_cancel_plan_command` | Credentialed billing flow; destructive remote | logged-in subscribed ArtCraft account; network | Storyteller session; billing webview | can cancel plan; only with disposable billing fixture |
| `storyteller_open_customer_portal_manage_plan_command` | Credentialed billing flow | logged-in subscribed ArtCraft account; network | Storyteller session; billing webview | remote billing/account mutation path |
| `storyteller_open_customer_portal_switch_plan_command` | Credentialed billing flow; remote mutation | logged-in subscribed ArtCraft account; network; deliberate test plan fixture | Storyteller session; billing webview | can alter plan/billing state |
| `storyteller_open_customer_portal_update_payment_method_command` | Credentialed billing flow; remote mutation | logged-in billing-enabled ArtCraft account; network | Storyteller session; billing webview | should use sandbox/disposable payment method only |
| `storyteller_open_subscription_purchase_command` | Credentialed billing flow | logged-in ArtCraft session; network; Stripe/browser availability | Storyteller session; billing webview | can create paid subscription |
| `storyteller_purge_credentials_command` | Destructive local credential reset | existing ArtCraft session fixture | Storyteller session cookies + persisted copies | clears main webview cookies too |
| `tasks_nuke_all_command` | Destructive local mutation | existing local task fixtures | `state/tasks_v*.sqlite` | local-only but wipes queue state |
| `update_app_preferences_command` | Local mutating | valid payload | `settings/app_preferences.json` | local-only mutation |
| `worldlabs_clear_credentials_command` | Destructive local credential reset | existing WorldLabs credential fixture | WorldLabs credential files | disposable local fixture only |
| `worldlabs_get_credential_info_command` | Provider login readonly | stored WorldLabs bearer/cookies | WorldLabs credential files | email derived from JWT bearer claims when parseable |
| `worldlabs_open_login_command` | Provider login bootstrap | interactive WorldLabs login/webview with JS token capture | WorldLabs credential files | captures cookies + bearer + refresh |
| `worldlabs_receive_bearer_command` | Login-bridge helper; mutating | valid bearer/refresh payload delivered from login flow or explicit test harness | WorldLabs in-memory bridge / eventual credential persistence via login thread | not a complete standalone user flow |

## Provider-specific E2E implications

### ArtCraft / Storyteller-backed generation

Commands that enqueue ArtCraft-hosted work need:

- a logged-in ArtCraft session
- network access to the Storyteller host
- enough credits/subscription entitlement for the chosen model
- tolerance for queued async task side effects in `state/tasks_v*.sqlite`

This applies to the ArtCraft paths behind:

- `enqueue_text_to_image_command`
- `enqueue_edit_image_command`
- `enqueue_image_bg_removal_command`
- `enqueue_image_inpaint_command`
- `enqueue_image_to_3d_object_command`
- `enqueue_image_to_video_command`

### Sora-backed generation

Requires:

- `open_sora_login_command` first (or equivalent valid stored Sora credentials)
- OpenAI/Sora account state that permits the selected operation
- network access
- tolerance for session renewal mutating stored credentials

### Midjourney-backed generation

Requires:

- `midjourney_open_login_command`
- successful capture of cookies **and** user info with user id
- network access
- account state that still allows job submission

### Grok-backed generation

Requires:

- `grok_open_login_command`
- either full stored Grok credentials or cookies that can be upgraded by requesting client secrets
- network access
- account state permitting image/video generation

### WorldLabs-backed generation

Requires:

- `worldlabs_open_login_command`
- valid cookies + bearer + refresh token persisted locally
- valid source image media token for Marble flows
- network access
- tolerance for remote world-creation side effects

## Known uncertainties / places not to guess

1. **FAL support is unclear in the current CLI path.**
   - `fal_api_key.txt` and `NeedsFalApiKey` references still exist.
   - Frontend TS bindings reference `get_fal_api_key_command` / `set_fal_api_key_command`.
   - Those backend command implementations were **not found** in the scanned Rust source or current invoke allowlist.
   - Current scanned invoke enqueue routing did not expose an active `GenerationProvider::Fal` branch.

2. **Storyteller persistence is partly plugin-owned.**
   - Code comments say `tauri-plugin-http` stores credentials on disk and the explicit `persist_all_to_disk()` call is deferred/commented.
   - So the durable source of truth is partly the Tauri cookie jar, not just the credential manager files.

3. **`worldlabs_receive_bearer_command` is a bridge, not a full operator workflow.**
   - Meaningful standalone E2E for this command may need a harness that imitates the login-webview JS handoff.

4. **Billing-flow commands are real-money capable.**
   - Without an upstream sandbox/digital twin, they should only be exercised with disposable fixtures and deliberate human approval.
