# ArtCraft Tauri Command Matrix (Desktop)

This document inventories `#[tauri::command]` functions under `crates/desktop/**/src` and maps them to a proposed CLI surface.

## Policy / Safety model

ArtCraft’s CLI is intended to expose a *subset* of Tauri commands via a generic entrypoint:

- **SAFE (default)**: read-only introspection only.
  - No file writes
  - No DB writes / state mutation
  - No credential mutation
  - No network/provider calls that could spend tokens or hit third-party services
- **UNSAFE (explicit opt-in)**: anything that mutates state, touches sensitive data, writes to disk, or calls external providers (including *generation*).
  - Requires **both**:
    1) `--unsafe` on the CLI, **and**
    2) the unsafe gate enabled (`ARTCRAFT_ENABLE_UNSAFE_INVOKE=1` or `~/.config/artcraft/cli.json` with `{ "enableUnsafeInvoke": true }`).

Source of truth for CLI allowlisting/wiring:
- `crates/desktop/artcraft/src/core/cli/invoke_dispatcher.rs`

## Currently wired in the CLI invoke dispatcher

As of this matrix, the only commands wired + allowlisted in `invoke_dispatcher.rs` are:

**SAFE allowlist (wired):**
- `platform_info_command`
- `get_app_info_command`
- `get_task_queue_command`

**UNSAFE allowlist (wired):**
- `get_provider_order_command`

Everything else below is *Tauri-exposed* but not (yet) callable from `artcraft invoke ...`.

---

## Command table

Legend:
- **Payload type**: the request struct or raw args used by the Tauri command signature.
- **Required state**: `AppHandle` and `State<...>` dependencies.
- **Dispatcher status**:
  - `WIRED+ALLOWLISTED` = callable via `artcraft invoke <command>` today
  - `NOT WIRED` = not reachable from CLI until added to `invoke_dispatcher.rs`
  - `DIFFERENT CRATE` = defined in `tauri-realtime`; not part of the `artcraft` CLI wiring today

> Proposed CLI strings assume the existing shape: `artcraft invoke <tauri_command_fn_name> [--payload <json|@file>] [--json] [--unsafe]`.

| Command fn | Proposed CLI command string | Path | Tier | Reason | Payload type | Required state | Notes / Test strategy | Dispatcher status |
|---|---|---|---|---|---|---|---|---|
| `platform_info_command` | `artcraft invoke platform_info_command --json` | `crates/desktop/artcraft/src/core/commands/platform_info_command.rs` | SAFE | Local platform introspection only | `()` | none | Smoke: run via CLI, assert JSON parses | WIRED+ALLOWLISTED (SAFE); smoke: TODO |
| `get_app_info_command` | `artcraft invoke get_app_info_command --json` | `crates/desktop/artcraft/src/core/commands/get_app_info_command.rs` | SAFE | Read-only app/env info | `()` | `State<AppDataRoot>`, `State<AppEnvConfigs>`, `State<ArtcraftPlatformInfo>`, `State<AppPreferencesManager>` | Smoke: CLI prints expected fields; avoid secrets | WIRED+ALLOWLISTED (SAFE); smoke: TODO |
| `get_task_queue_command` | `artcraft invoke get_task_queue_command --json` | `crates/desktop/artcraft/src/core/commands/task_queue/get_task_queue_command.rs` | SAFE | Read-only task DB query | `()` | `AppHandle`, `State<AppEnvConfigs>`, `State<TaskDatabase>` | Smoke: CLI bootstraps DB and returns list | WIRED+ALLOWLISTED (SAFE); smoke: TODO |
| `get_provider_order_command` | `artcraft invoke get_provider_order_command --unsafe --json` | `crates/desktop/artcraft/src/core/commands/providers/get_provider_order_command.rs` | UNSAFE | Touches provider priority config (sensitive surface); allowlisted as UNSAFE today | `()` | `State<ProviderPriorityStore>` | Smoke: enable unsafe gate + call; ensure redaction if needed | WIRED+ALLOWLISTED (UNSAFE); smoke: TODO |
| `set_provider_order_command` | `artcraft invoke set_provider_order_command --unsafe --payload @req.json` | `crates/desktop/artcraft/src/core/commands/providers/set_provider_order_command.rs` | UNSAFE | Mutates provider priority config on disk | `SetProviderOrderRequest` | `State<ProviderPriorityStore>`, `State<AppDataRoot>` | Add CLI payload mapping + integration test w/temp config dir | NOT WIRED |
| `get_app_preferences_command` | `artcraft invoke get_app_preferences_command --json` | `crates/desktop/artcraft/src/core/commands/app_preferences/get_app_preferences_command.rs` | SAFE | Read-only preferences fetch | `()` | `State<AppPreferencesManager>` | Unit test: deserialize + golden output | NOT WIRED |
| `update_app_preferences_command` | `artcraft invoke update_app_preferences_command --unsafe --payload @req.json` | `crates/desktop/artcraft/src/core/commands/app_preferences/update_app_preference_command.rs` | UNSAFE | Mutates preferences on disk | `UpdateAppPreferencesRequest` | `State<AppPreferencesManager>`, `State<AppDataRoot>` | Needs CLI payload bridging; test writes + rollback | NOT WIRED |
| `estimate_image_cost_command` | `artcraft invoke estimate_image_cost_command --json --payload @req.json` | `crates/desktop/artcraft/src/core/commands/cost_estimate/estimate_image_cost_command.rs` | SAFE | Pure-ish cost estimation (no generation) | `EstimateImageCostRequest` | `State<AppEnvConfigs>` | Unit test: fixed inputs; ensure no network | NOT WIRED |
| `estimate_video_cost_command` | `artcraft invoke estimate_video_cost_command --json --payload @req.json` | `crates/desktop/artcraft/src/core/commands/cost_estimate/estimate_video_cost_command.rs` | SAFE | Pure-ish cost estimation (no generation) | `EstimateVideoCostRequest` | `State<AppEnvConfigs>` | Unit test: fixed inputs; ensure no network | NOT WIRED |
| `flip_image` | `artcraft invoke flip_image --json --payload '{"image":"...base64..."}'` | `crates/desktop/artcraft/src/core/commands/flip_image.rs` | SAFE | Deterministic local transform | `image: &str` | none | Unit test: known input hash/output | NOT WIRED |
| `load_without_cors_command` | `artcraft invoke load_without_cors_command --unsafe --payload '{"url":"https://..."}'` | `crates/desktop/artcraft/src/core/commands/load_without_cors_command.rs` | UNSAFE | Network fetch / proxying | `url: String` | none | Integration test behind mock server; SSRF guard review | NOT WIRED |
| `download_url_command` | `artcraft invoke download_url_command --unsafe --payload @req.json` | `crates/desktop/artcraft/src/core/commands/download/download_url_command.rs` | UNSAFE | Network + filesystem write | `DownloadUrlRequest` | `AppHandle`, `State<AppPreferencesManager>`, `State<AppDataRoot>`, `State<AppEnvConfigs>` | Integration test w/temp dir + local http server | NOT WIRED |
| `download_media_file_command` | `artcraft invoke download_media_file_command --unsafe --payload @req.json` | `crates/desktop/artcraft/src/core/commands/download/download_media_file_command.rs` | UNSAFE | Network + filesystem write | `DownloadMediaFileRequest` | `AppHandle`, `State<AppPreferencesManager>`, `State<AppDataRoot>`, `State<AppEnvConfigs>` | Integration test: mock API + temp dir | NOT WIRED |
| `download_directory_reveal_command` | `artcraft invoke download_directory_reveal_command --unsafe` | `crates/desktop/artcraft/src/core/commands/download/download_directory_reveal_command.rs` | UNSAFE | Opens/reveals directory in OS shell (side effects) | `()` | `AppHandle`, `State<AppPreferencesManager>`, `State<AppDataRoot>` | Smoke only; no CI (headless) | NOT WIRED |
| `media_file_delete_command` | `artcraft invoke media_file_delete_command --unsafe --payload @req.json` | `crates/desktop/artcraft/src/core/commands/media_files/media_file_delete_command.rs` | UNSAFE | Deletes remote media / mutates server state | `MediaFileDeleteRequest` | `AppHandle`, `State<AppEnvConfigs>`, `State<StorytellerCredentialManager>` | Integration test behind staging; require explicit confirmation | NOT WIRED |
| `reset_artcraft_command` | `artcraft invoke reset_artcraft_command --unsafe` | `crates/desktop/artcraft/src/core/commands/app_state/reset_artcraft_command.rs` | UNSAFE | Resets local app state | `()` | `AppHandle` | Manual smoke; verify it wipes expected dirs only | NOT WIRED |
| `mark_task_as_dismissed_command` | `artcraft invoke mark_task_as_dismissed_command --unsafe --payload @req.json` | `crates/desktop/artcraft/src/core/commands/task_queue/mark_task_as_dismissed_command.rs` | UNSAFE | Mutates task DB row | `MarkTaskAsDismissedRequest` | `AppHandle`, `State<AppEnvConfigs>`, `State<TaskDatabase>` | Integration test w/temp sqlite | NOT WIRED |
| `tasks_nuke_all_command` | `artcraft invoke tasks_nuke_all_command --unsafe` | `crates/desktop/artcraft/src/core/commands/task_queue/tasks_nuke_all_command.rs` | UNSAFE | Deletes all tasks (destructive) | `()` | `State<TaskDatabase>` | Add extra CLI confirmation prompt; integration test with fixture DB | NOT WIRED |
| `enqueue_text_to_image_command` | `artcraft invoke enqueue_text_to_image_command --unsafe --payload @req.json` | `crates/desktop/artcraft/src/core/commands/enqueue/text_to_image/enqueue_text_to_image_command.rs` | UNSAFE | Generation: spends tokens/cost; enqueues tasks + network | `EnqueueTextToImageRequest` | `AppHandle` + multiple `State<...>` (env/config, usage, provider priority, DB, creds/queues) | Integration test behind mocked providers; verify task DB insert | NOT WIRED |
| `enqueue_edit_image_command` | `artcraft invoke enqueue_edit_image_command --unsafe --payload @req.json` | `crates/desktop/artcraft/src/core/commands/enqueue/image_edit/enqueue_edit_image_command.rs` | UNSAFE | Generation/edit: spends tokens; enqueues + network + DB writes | `EnqueueEditImageCommand` | `AppHandle` + multiple `State<...>` (env/config, usage, provider priority, DB, creds/queues) | Integration test behind mocked providers; verify event emissions optional | NOT WIRED |
| `enqueue_image_inpaint_command` | `artcraft invoke enqueue_image_inpaint_command --unsafe --payload @req.json` | `crates/desktop/artcraft/src/core/commands/enqueue/image_inpaint/enqueue_image_inpaint_command.rs` | UNSAFE | Generation/inpaint: spends tokens; enqueues + network + DB writes | `EnqueueInpaintImageCommand` | `AppHandle` + multiple `State<...>` (env/config, usage, provider priority, DB, creds/queue) | Integration test behind mocked providers; mask XOR validation tests | NOT WIRED |
| `enqueue_image_bg_removal_command` | `artcraft invoke enqueue_image_bg_removal_command --unsafe --payload @req.json` | `crates/desktop/artcraft/src/core/commands/enqueue/image_bg_removal/enqueue_image_bg_removal_command.rs` | UNSAFE | Mutates state + may call providers; DB writes | `EnqueueImageBgRemovalCommand` | `AppHandle` + `State<AppDataRoot>`, `State<AppEnvConfigs>`, `State<ArtcraftUsageTracker>`, `State<ProviderPriorityStore>`, `State<TaskDatabase>`, `State<StorytellerCredentialManager>` | Integration test w/mock; verify task created | NOT WIRED |
| `enqueue_image_to_video_command` | `artcraft invoke enqueue_image_to_video_command --unsafe --payload @req.json` | `crates/desktop/artcraft/src/core/commands/enqueue/image_to_video/enqueue_image_to_video_command.rs` | UNSAFE | Generation/video: spends tokens/cost; network + DB writes | `EnqueueImageToVideoRequest` | `AppHandle` + many `State<...>` (env/config, usage, provider priority, DB, creds/queues) | Integration test w/mock providers; verify queue + DB insert | NOT WIRED |
| `enqueue_image_to_gaussian_command` | `artcraft invoke enqueue_image_to_gaussian_command --unsafe --payload @req.json` | `crates/desktop/artcraft/src/core/commands/enqueue/image_to_gaussian/enqueue_image_to_gaussian_command.rs` | UNSAFE | Generation/3D pipeline: likely network + DB writes | `EnqueueImageToGaussianRequest` | `AppHandle`, `State<AppDataRoot>`, `State<AppEnvConfigs>`, `State<ArtcraftUsageTracker>`, `State<TaskDatabase>`, `State<StorytellerCredentialManager>`, `State<WorldlabsCredentialManager>` | Integration test w/mock; verify artifact paths | NOT WIRED |
| `enqueue_image_to_3d_object_command` | `artcraft invoke enqueue_image_to_3d_object_command --unsafe --payload @req.json` | `crates/desktop/artcraft/src/core/commands/enqueue/image_to_object/enqueue_image_to_3d_object_command.rs` | UNSAFE | Generation/object: spends tokens/cost; network + DB writes | `EnqueueImageTo3dObjectRequest` | `AppHandle`, `State<AppEnvConfigs>`, `State<AppDataRoot>`, `State<ArtcraftUsageTracker>`, `State<ProviderPriorityStore>`, `State<TaskDatabase>`, `State<StorytellerCredentialManager>`, `State<SoraTaskQueue>` | Integration test w/mock; verify task created | NOT WIRED |
| `sora_get_credential_info_command` | `artcraft invoke sora_get_credential_info_command --unsafe --json` | `crates/desktop/artcraft/src/services/sora/commands/sora_get_credential_info_command.rs` | UNSAFE | Exposes credential-derived info (sensitive) | `()` | `State<SoraCredentialManager>` | Ensure output redaction; unit test only | NOT WIRED |
| `open_sora_login_command` | `artcraft invoke open_sora_login_command --unsafe` | `crates/desktop/artcraft/src/services/sora/commands/open_sora_login_command.rs` | UNSAFE | Opens browser / login flow (side effects) | `()` | `AppHandle`, `State<AppDataRoot>`, `State<SoraCredentialManager>` | Manual smoke only | NOT WIRED |
| `check_sora_session_command` | `artcraft invoke check_sora_session_command --unsafe --json` | `crates/desktop/artcraft/src/services/sora/commands/check_sora_session_command.rs` | UNSAFE | Likely network/session renewal | `()` | `AppHandle`, `State<SoraCredentialManager>` | Add mocked HTTP + session expiry tests | NOT WIRED |
| `sora_logout_command` | `artcraft invoke sora_logout_command --unsafe` | `crates/desktop/artcraft/src/services/sora/commands/sora_logout_command.rs` | UNSAFE | Mutates credentials/session state | `()` | `State<SoraCredentialManager>` | Unit test: clears cookie store | NOT WIRED |
| `midjourney_get_credential_info_command` | `artcraft invoke midjourney_get_credential_info_command --unsafe --json` | `crates/desktop/artcraft/src/services/midjourney/commands/midjourney_get_credential_info_command.rs` | UNSAFE | Returns user email / credential-derived info | `()` | `State<MidjourneyCredentialManager>` | Verify no tokens/cookies are printed | NOT WIRED |
| `midjourney_open_login_command` | `artcraft invoke midjourney_open_login_command --unsafe` | `crates/desktop/artcraft/src/services/midjourney/commands/midjourney_open_login_command.rs` | UNSAFE | Opens browser / login flow | `()` | `AppHandle`, `State<AppDataRoot>`, `State<MidjourneyCredentialManager>` | Manual smoke only | NOT WIRED |
| `midjourney_clear_credentials_command` | `artcraft invoke midjourney_clear_credentials_command --unsafe` | `crates/desktop/artcraft/src/services/midjourney/commands/midjourney_clear_credentials_command.rs` | UNSAFE | Deletes credential files | `()` | `State<AppDataRoot>`, `State<MidjourneyCredentialManager>` | Unit test: deletes expected paths only | NOT WIRED |
| `grok_get_credential_info_command` | `artcraft invoke grok_get_credential_info_command --unsafe --json` | `crates/desktop/artcraft/src/services/grok/commands/grok_get_credential_info_command.rs` | UNSAFE | Credential-derived info (sensitive surface) | `()` | `State<GrokCredentialManager>` | Verify no secrets printed | NOT WIRED |
| `grok_open_login_command` | `artcraft invoke grok_open_login_command --unsafe` | `crates/desktop/artcraft/src/services/grok/commands/grok_open_login_command.rs` | UNSAFE | Opens browser / login flow | `()` | `AppHandle`, `State<AppDataRoot>`, `State<GrokCredentialManager>` | Manual smoke only | NOT WIRED |
| `grok_clear_credentials_command` | `artcraft invoke grok_clear_credentials_command --unsafe` | `crates/desktop/artcraft/src/services/grok/commands/grok_clear_credentials_command.rs` | UNSAFE | Deletes credential files | `()` | `State<AppDataRoot>`, `State<GrokCredentialManager>` | Unit test: deletes expected paths only | NOT WIRED |
| `worldlabs_get_credential_info_command` | `artcraft invoke worldlabs_get_credential_info_command --unsafe --json` | `crates/desktop/artcraft/src/services/worldlabs/commands/worldlabs_get_credential_info_command.rs` | UNSAFE | Credential-derived info (sensitive) | `()` | `State<WorldlabsCredentialManager>` | Verify no bearer/token output | NOT WIRED |
| `worldlabs_open_login_command` | `artcraft invoke worldlabs_open_login_command --unsafe` | `crates/desktop/artcraft/src/services/worldlabs/commands/worldlabs_open_login_command.rs` | UNSAFE | Opens browser / login flow | `()` | `AppHandle`, `State<AppDataRoot>`, `State<WorldlabsBearerBridge>`, `State<WorldlabsCredentialManager>` | Manual smoke only | NOT WIRED |
| `worldlabs_clear_credentials_command` | `artcraft invoke worldlabs_clear_credentials_command --unsafe` | `crates/desktop/artcraft/src/services/worldlabs/commands/worldlabs_clear_credentials_command.rs` | UNSAFE | Deletes credential files | `()` | `State<AppDataRoot>`, `State<WorldlabsCredentialManager>` | Unit test: deletes expected paths only | NOT WIRED |
| `worldlabs_receive_bearer_command` | `artcraft invoke worldlabs_receive_bearer_command --unsafe --payload @req.json` | `crates/desktop/artcraft/src/services/worldlabs/commands/worldlabs_receive_bearer_command.rs` | UNSAFE | Writes bearer token/credentials | `WorldlabsReceiveBearerRequest` | `State<AppDataRoot>`, `State<WorldlabsBearerBridge>` | Security review: avoid logging raw token | NOT WIRED |
| `storyteller_get_credits_command` | `artcraft invoke storyteller_get_credits_command --unsafe --json` | `crates/desktop/artcraft/src/services/storyteller/commands/storyteller_get_credits_command.rs` | UNSAFE | Network call using creds; sensitive account data | `()` | `State<AppEnvConfigs>`, `State<StorytellerCredentialManager>` | Integration test w/mock; redact PII | NOT WIRED |
| `storyteller_get_subscription_command` | `artcraft invoke storyteller_get_subscription_command --unsafe --json` | `crates/desktop/artcraft/src/services/storyteller/commands/storyteller_get_subscription_command.rs` | UNSAFE | Network call using creds; sensitive billing data | `()` | `State<AppEnvConfigs>`, `State<StorytellerCredentialManager>` | Integration test w/mock; redact PII | NOT WIRED |
| `storyteller_purge_credentials_command` | `artcraft invoke storyteller_purge_credentials_command --unsafe` | `crates/desktop/artcraft/src/services/storyteller/commands/storyteller_purge_credentials_command.rs` | UNSAFE | Deletes credentials | `()` | `AppHandle`, `State<StorytellerCredentialManager>` | Unit test: clears expected stores | NOT WIRED |
| `storyteller_open_credits_purchase_command` | `artcraft invoke storyteller_open_credits_purchase_command --unsafe --payload @req.json` | `crates/desktop/artcraft/src/services/storyteller/commands/stripe_checkout/storyteller_open_credits_purchase_command.rs` | UNSAFE | Opens Stripe checkout (side effects) | `StorytellerOpenCreditsPurchaseCommand` | `AppHandle`, `State<AppDataRoot>`, `State<AppEnvConfigs>`, `State<StorytellerCredentialManager>` | Manual smoke only; ensure URL origin allowlist | NOT WIRED |
| `storyteller_open_subscription_purchase_command` | `artcraft invoke storyteller_open_subscription_purchase_command --unsafe --payload @req.json` | `crates/desktop/artcraft/src/services/storyteller/commands/stripe_checkout/storyteller_open_subscription_purchase_command.rs` | UNSAFE | Opens Stripe checkout (side effects) | `StorytellerOpenSubscriptionPurchaseCommand` | `AppHandle`, `State<AppDataRoot>`, `State<AppEnvConfigs>`, `State<StorytellerCredentialManager>` | Manual smoke only | NOT WIRED |
| `storyteller_open_customer_portal_manage_plan_command` | `artcraft invoke storyteller_open_customer_portal_manage_plan_command --unsafe` | `crates/desktop/artcraft/src/services/storyteller/commands/stripe_customer_portal/storyteller_open_customer_portal_manage_plan_command.rs` | UNSAFE | Opens Stripe customer portal | `()` | `AppHandle`, `State<AppDataRoot>`, `State<AppEnvConfigs>`, `State<StorytellerCredentialManager>` | Manual smoke only | NOT WIRED |
| `storyteller_open_customer_portal_update_payment_method_command` | `artcraft invoke storyteller_open_customer_portal_update_payment_method_command --unsafe` | `crates/desktop/artcraft/src/services/storyteller/commands/stripe_customer_portal/storyteller_open_customer_portal_update_payment_method_command.rs` | UNSAFE | Opens Stripe customer portal | `()` | `AppHandle`, `State<AppDataRoot>`, `State<AppEnvConfigs>`, `State<StorytellerCredentialManager>` | Manual smoke only | NOT WIRED |
| `storyteller_open_customer_portal_switch_plan_command` | `artcraft invoke storyteller_open_customer_portal_switch_plan_command --unsafe --payload @req.json` | `crates/desktop/artcraft/src/services/storyteller/commands/stripe_customer_portal/storyteller_open_customer_portal_switch_plan_command.rs` | UNSAFE | Opens Stripe customer portal (plan change) | `StorytellerOpenCustomerPortalSwitchPlanCommand` | `AppHandle`, `State<AppDataRoot>`, `State<AppEnvConfigs>`, `State<StorytellerCredentialManager>` | Manual smoke only | NOT WIRED |
| `storyteller_open_customer_portal_cancel_plan_command` | `artcraft invoke storyteller_open_customer_portal_cancel_plan_command --unsafe` | `crates/desktop/artcraft/src/services/storyteller/commands/stripe_customer_portal/storyteller_open_customer_portal_cancel_plan_command.rs` | UNSAFE | Opens Stripe customer portal (cancel) | `()` | `AppHandle`, `State<AppDataRoot>`, `State<AppEnvConfigs>`, `State<StorytellerCredentialManager>` | Manual smoke only | NOT WIRED |

### `tauri-realtime` commands (different crate)

These are `#[tauri::command]` endpoints in `crates/desktop/tauri-realtime`. They are not wired into `artcraft invoke` today.

| Command fn | Proposed CLI command string | Path | Tier | Reason | Payload type | Required state | Notes / Test strategy | Dispatcher status |
|---|---|---|---|---|---|---|---|---|
| `download_models` | `artcraft invoke download_models --unsafe` | `crates/desktop/tauri-realtime/src/endpoints/download_models.rs` | UNSAFE | Downloads ML weights (network + disk) | `()` | `State<AppDataRoot>` | Integration test behind local mirror; large downloads | DIFFERENT CRATE |
| `flip_image` | `artcraft invoke flip_image --json --payload '{"image":"..."}'` | `crates/desktop/tauri-realtime/src/endpoints/flip_image.rs` | SAFE | Deterministic local transform | `image: &str` | none | Unit test: known input/output | DIFFERENT CRATE |
| `infer_image` | `artcraft invoke infer_image --unsafe --payload @req.json` | `crates/desktop/tauri-realtime/src/endpoints/realtime_image_endpoint.rs` | UNSAFE | Local/ML inference (heavy compute; potential model downloads) | `image: &str` + options | `AppHandle`, `State<AppConfig>`, `State<ModelCache>`, `State<PromptCache>`, `State<AppDataRoot>` | Smoke only on capable hardware; gate behind flag | DIFFERENT CRATE |
| `text_to_image` | `artcraft invoke text_to_image --unsafe --payload @req.json` | `crates/desktop/tauri-realtime/src/endpoints/text_to_image_endpoint.rs` | UNSAFE | ML generation | `prompt: String` + caches | `AppHandle`, `State<AppConfig>`, `State<ModelCache>`, `State<PromptCache>`, `State<AppDataRoot>` | Smoke only on capable hardware | DIFFERENT CRATE |
| `inpaint_image` | `artcraft invoke inpaint_image --unsafe --payload @req.json` | `crates/desktop/tauri-realtime/src/endpoints/inpaint_image_endpoint.rs` | UNSAFE | ML generation | `image: &str`, `mask: &str`, `prompt: String` | none | Unit test basic validation; smoke for runtime | DIFFERENT CRATE |
| `remove_background` | `artcraft invoke remove_background --unsafe --payload @req.json` | `crates/desktop/tauri-realtime/src/endpoints/remove_background_endpoint.rs` | UNSAFE | ML inference + writes/cache | `image: &str` | `State<AppDataRoot>` | Smoke only | DIFFERENT CRATE |
| `save_image` | `artcraft invoke save_image --unsafe --payload @req.json` | `crates/desktop/tauri-realtime/src/endpoints/save_image_endpoint.rs` | UNSAFE | Writes image to disk | `image: &str` | `State<AppDataRoot>` | Integration test with temp dir | DIFFERENT CRATE |

---

## Workflow checklist: adding a command end-to-end (Tauri → CLI)

1) **Inventory + signature**
   - Confirm the function is annotated with `#[tauri::command]` and identify:
     - payload args (request struct / primitives)
     - required `State<...>` / `AppHandle`

2) **Decide tier**
   - SAFE only if read-only + local + non-sensitive.
   - Otherwise UNSAFE.

3) **Wire into CLI dispatcher** (`crates/desktop/artcraft/src/core/cli/invoke_dispatcher.rs`)
   - Add a `const CMD_...` string matching the command name
   - Add to `SAFE_INVOKE_ALLOWLIST` or `UNSAFE_INVOKE_ALLOWLIST`
   - Add a `match` arm to call the function
   - If the command needs `async`, wrap with `tauri::async_runtime::block_on`

4) **Payload bridging (if needed)**
   - `dispatch_invoke` currently validates `--payload` but does not pass it.
   - Add parsing + `serde_json` -> request struct conversion in the dispatcher arm.
   - Prefer `--payload @file.json` for complex requests.

5) **State bootstrapping**
   - If the GUI registers state during startup, ensure CLI path bootstraps it (like `TaskDatabase`).

6) **Safety gates + confirmations**
   - For destructive ops (delete/nuke/reset), add an extra confirmation layer in the CLI (even under `--unsafe`).

7) **Tests / smoke**
   - Unit tests for request parsing + validation.
   - Integration tests using temp dirs / mock servers.
   - Add at least one manual smoke command line to the matrix row.

8) **Update docs**
   - Update this `COMMAND_MATRIX.md` and any CLI docs/help text.
