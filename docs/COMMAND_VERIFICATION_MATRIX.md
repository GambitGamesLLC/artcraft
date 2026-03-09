# ArtCraft CLI Verification Matrix (automated)

This document is the **verification matrix** for the tiered `artcraft invoke` contract described in:

- `docs/COMMAND_MATRIX.md`

It maps each canonical command to the expected behavior under the default (safe) verification environment.

## Canonical command list

The canonical lists live in the desktop CLI dispatcher:

- `crates/desktop/artcraft/src/core/cli/invoke_dispatcher.rs`
  - `SAFE_ALLOWLIST`
  - `UNSAFE_ALLOWLIST`

The automated verifier derives the runtime lists via:

```bash
./target/release/artcraft invoke --list-allowed --json
```

## Verification environment assumptions

The verifier intentionally runs with the UNSAFE gate **disabled** (non-destructive by default):

- `ARTCRAFT_ENABLE_UNSAFE_INVOKE=0`
- `XDG_CONFIG_HOME` pointed at an empty temp dir (prevents accidentally picking up `~/.config/artcraft/cli.json`)

This ensures that **no UNSAFE command is ever executed** during default verification.

## Expected outcomes (exit codes + JSON error codes)

See `docs/COMMAND_MATRIX.md` for the authoritative exit-code meanings.

Legend used in the matrix below:

- **SAFE succeeds**: exit `0` with valid JSON output
- **UNSAFE requires `--unsafe`**: exit `3` with `error_details.code = "unsafe_required"`
- **UNSAFE gate disabled** (when `--unsafe` is provided but the gate is off): exit `2` with `error_details.code = "unsafe_gate_disabled"`
- **Payload hardening** for UNSAFE: `--payload @/nonexistent/file.json` must **still** fail with `unsafe_required` / `unsafe_gate_disabled` (i.e. do not read the file before enforcing tier/gate)

## Matrix (default verifier mode)

| Tier | Command | No `--unsafe` (expected) | With `--unsafe` + gate disabled (expected) | Notes |
|---|---|---|---|---|
| SAFE | `platform_info_command` | ✅ exit 0 | ✅ exit 0 (tier resolution is SAFE; gate not consulted) | No payload |
| SAFE | `flip_image` | ✅ exit 0 (with valid payload) | ✅ exit 0 (tier resolution is SAFE; gate not consulted) | Requires payload: `{ "image": "<base64>" }` |
| UNSAFE | `check_sora_session_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | payload hardening enforced |
| UNSAFE | `download_directory_reveal_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | payload hardening enforced |
| UNSAFE | `download_media_file_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | payload hardening enforced |
| UNSAFE | `download_url_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | payload hardening enforced |
| UNSAFE | `enqueue_edit_image_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | would enqueue work; never executed by default verifier |
| UNSAFE | `enqueue_image_bg_removal_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | would enqueue work; never executed by default verifier |
| UNSAFE | `enqueue_image_inpaint_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | would enqueue work; never executed by default verifier |
| UNSAFE | `enqueue_image_to_3d_object_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | would enqueue work; never executed by default verifier |
| UNSAFE | `enqueue_image_to_gaussian_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | would enqueue work; never executed by default verifier |
| UNSAFE | `enqueue_image_to_video_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | would enqueue work; never executed by default verifier |
| UNSAFE | `enqueue_text_to_image_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | would enqueue work; never executed by default verifier |
| UNSAFE | `estimate_image_cost_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | payload hardening enforced |
| UNSAFE | `estimate_video_cost_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | payload hardening enforced |
| UNSAFE | `get_app_info_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | payload hardening enforced |
| UNSAFE | `get_app_preferences_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | payload hardening enforced |
| UNSAFE | `get_provider_order_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | payload hardening enforced |
| UNSAFE | `get_task_queue_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | payload hardening enforced |
| UNSAFE | `grok_clear_credentials_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | payload hardening enforced |
| UNSAFE | `grok_get_credential_info_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | payload hardening enforced |
| UNSAFE | `grok_open_login_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | payload hardening enforced |
| UNSAFE | `load_without_cors_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | payload hardening enforced |
| UNSAFE | `mark_task_as_dismissed_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | payload hardening enforced |
| UNSAFE | `media_file_delete_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | destructive; never executed by default verifier |
| UNSAFE | `midjourney_clear_credentials_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | payload hardening enforced |
| UNSAFE | `midjourney_get_credential_info_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | payload hardening enforced |
| UNSAFE | `midjourney_open_login_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | payload hardening enforced |
| UNSAFE | `open_sora_login_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | payload hardening enforced |
| UNSAFE | `set_provider_order_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | payload hardening enforced |
| UNSAFE | `sora_get_credential_info_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | payload hardening enforced |
| UNSAFE | `sora_logout_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | payload hardening enforced |
| UNSAFE | `storyteller_get_credits_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | payload hardening enforced |
| UNSAFE | `storyteller_get_subscription_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | payload hardening enforced |
| UNSAFE | `storyteller_open_credits_purchase_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | might open browser / billing; never executed by default verifier |
| UNSAFE | `storyteller_open_customer_portal_cancel_plan_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | might open browser / billing; never executed by default verifier |
| UNSAFE | `storyteller_open_customer_portal_manage_plan_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | might open browser / billing; never executed by default verifier |
| UNSAFE | `storyteller_open_customer_portal_switch_plan_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | might open browser / billing; never executed by default verifier |
| UNSAFE | `storyteller_open_customer_portal_update_payment_method_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | might open browser / billing; never executed by default verifier |
| UNSAFE | `storyteller_open_subscription_purchase_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | might open browser / billing; never executed by default verifier |
| UNSAFE | `storyteller_purge_credentials_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | destructive for credentials; never executed by default verifier |
| UNSAFE | `tasks_nuke_all_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | destructive; never executed by default verifier |
| UNSAFE | `update_app_preferences_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | mutating; never executed by default verifier |
| UNSAFE | `worldlabs_clear_credentials_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | destructive for credentials; never executed by default verifier |
| UNSAFE | `worldlabs_get_credential_info_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | payload hardening enforced |
| UNSAFE | `worldlabs_open_login_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | might open browser; never executed by default verifier |
| UNSAFE | `worldlabs_receive_bearer_command` | ❌ exit 3 / `unsafe_required` | ❌ exit 2 / `unsafe_gate_disabled` | payload hardening enforced |

## Automated verifier

The verifier lives at:

- `scripts/tools/verify_artcraft_cli_commands.py`

Run it from the repo root:

```bash
./scripts/tools/verify_artcraft_cli_commands.py
```

Common options:

- `--binary ./target/release/artcraft`
- `--skip-payload-hardening`

The verifier will **not** enable the UNSAFE gate and will **not** run any UNSAFE command beyond the expected pre-flight enforcement failures.

### Optional UNSAFE smoke tests (explicit opt-in)

For an extra smoke test of the `--unsafe` path, the verifier can execute a small, approved UNSAFE subset **only** when you explicitly opt in:

```bash
./scripts/tools/verify_artcraft_cli_commands.py --run-unsafe-subset readonly --unsafe-gate-on
```

Additional latches:

- `readonly-network-cost` is **SKIPPED** unless you also pass `--allow-network` (it may hit the network / incur cost):

  ```bash
  ./scripts/tools/verify_artcraft_cli_commands.py --run-unsafe-subset readonly-network-cost --unsafe-gate-on --allow-network
  ```

- `readonly-account` is **SKIPPED** unless you also pass `--allow-credentialed` (it reads credentialed account state):

  ```bash
  ./scripts/tools/verify_artcraft_cli_commands.py --run-unsafe-subset readonly-account --unsafe-gate-on --allow-credentialed
  ```
