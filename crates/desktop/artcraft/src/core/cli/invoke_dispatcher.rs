use crate::core::commands::app_preferences::get_app_preferences_command::get_app_preferences_command;
use crate::core::commands::app_preferences::update_app_preference_command::{
  update_app_preferences_command, UpdateAppPreferencesRequest,
};
use crate::core::commands::cost_estimate::estimate_image_cost_command::estimate_image_cost_command;
use crate::core::commands::cost_estimate::estimate_video_cost_command::estimate_video_cost_command;
use artcraft_api_defs::generate::cost_estimate::estimate_image_cost::EstimateImageCostRequest;
use artcraft_api_defs::generate::cost_estimate::estimate_video_cost::EstimateVideoCostRequest;
use crate::core::commands::download::download_directory_reveal_command::download_directory_reveal_command;
use crate::core::commands::download::download_media_file_command::{
  download_media_file_command, DownloadMediaFileRequest,
};
use crate::core::commands::download::download_url_command::{
  download_url_command, DownloadUrlRequest,
};
use crate::core::commands::enqueue::image_bg_removal::enqueue_image_bg_removal_command::{
  enqueue_image_bg_removal_command, EnqueueImageBgRemovalCommand,
};
use crate::core::commands::enqueue::image_edit::enqueue_edit_image_command::{
  enqueue_edit_image_command, EnqueueEditImageCommand,
};
use crate::core::commands::enqueue::image_inpaint::enqueue_image_inpaint_command::{
  enqueue_image_inpaint_command, EnqueueInpaintImageCommand,
};
use crate::core::commands::enqueue::image_to_gaussian::enqueue_image_to_gaussian_command::{
  enqueue_image_to_gaussian_command, EnqueueImageToGaussianRequest,
};
use crate::core::commands::enqueue::image_to_object::enqueue_image_to_3d_object_command::{
  enqueue_image_to_3d_object_command, EnqueueImageTo3dObjectRequest,
};
use crate::core::commands::enqueue::image_to_video::enqueue_image_to_video_command::{
  enqueue_image_to_video_command, EnqueueImageToVideoRequest,
};
use crate::core::commands::enqueue::text_to_image::enqueue_text_to_image_command::{
  enqueue_text_to_image_command, EnqueueTextToImageRequest,
};
use crate::core::commands::flip_image::flip_image;
use crate::core::commands::get_app_info_command::get_app_info_command;
use crate::core::commands::load_without_cors_command::load_without_cors_command;
use crate::core::commands::media_files::media_file_delete_command::{
  media_file_delete_command, MediaFileDeleteRequest,
};
use crate::core::commands::platform_info_command::platform_info_command;
use crate::core::commands::providers::get_provider_order_command::get_provider_order_command;
use crate::core::commands::providers::set_provider_order_command::{
  set_provider_order_command, SetProviderOrderRequest,
};
use crate::core::commands::response::failure_response_wrapper::{
  CommandErrorResponseWrapper, CommandErrorStatus,
};
use crate::core::commands::response::success_response_wrapper::{
  CommandSuccessResponseWrapper, CommandSuccessStatus,
};
use crate::core::commands::task_queue::get_task_queue_command::get_task_queue_command;
use crate::core::commands::task_queue::mark_task_as_dismissed_command::{
  mark_task_as_dismissed_command, MarkTaskAsDismissedRequest,
};
use crate::core::commands::task_queue::tasks_nuke_all_command::tasks_nuke_all_command;
use crate::core::state::app_env_configs::app_env_configs::AppEnvConfigs;
use crate::core::state::app_preferences::app_preferences_manager::AppPreferencesManager;
use crate::core::state::artcraft_platform_info::ArtcraftPlatformInfo;
use crate::core::state::data_dir::app_data_root::AppDataRoot;
use crate::core::state::provider_priority::ProviderPriorityStore;
use crate::core::state::task_database::TaskDatabase;
use crate::services::grok::commands::grok_clear_credentials_command::grok_clear_credentials_command;
use crate::services::grok::commands::grok_get_credential_info_command::grok_get_credential_info_command;
use crate::services::grok::commands::grok_open_login_command::grok_open_login_command;
use crate::services::grok::state::grok_credential_manager::GrokCredentialManager;
use crate::services::grok::state::grok_image_prompt_queue::GrokImagePromptQueue;
use crate::services::midjourney::commands::midjourney_clear_credentials_command::midjourney_clear_credentials_command;
use crate::services::midjourney::commands::midjourney_get_credential_info_command::midjourney_get_credential_info_command;
use crate::services::midjourney::commands::midjourney_open_login_command::midjourney_open_login_command;
use crate::services::midjourney::state::midjourney_credential_manager::MidjourneyCredentialManager;
use crate::services::sora::commands::check_sora_session_command::check_sora_session_command;
use crate::services::sora::commands::open_sora_login_command::open_sora_login_command;
use crate::services::sora::commands::sora_get_credential_info_command::sora_get_credential_info_command;
use crate::services::sora::commands::sora_logout_command::sora_logout_command;
use crate::services::sora::state::sora_credential_manager::SoraCredentialManager;
use crate::services::sora::state::sora_task_queue::SoraTaskQueue;
use crate::services::storyteller::commands::storyteller_get_credits_command::storyteller_get_credits_command;
use crate::services::storyteller::commands::storyteller_get_subscription_command::storyteller_get_subscription_command;
use crate::services::storyteller::commands::storyteller_purge_credentials_command::storyteller_purge_credentials_command;
use crate::services::storyteller::commands::stripe_checkout::storyteller_open_credits_purchase_command::{
  storyteller_open_credits_purchase_command, StorytellerOpenCreditsPurchaseCommand,
};
use crate::services::storyteller::commands::stripe_checkout::storyteller_open_subscription_purchase_command::{
  storyteller_open_subscription_purchase_command, StorytellerOpenSubscriptionPurchaseCommand,
};
use crate::services::storyteller::commands::stripe_customer_portal::storyteller_open_customer_portal_cancel_plan_command::storyteller_open_customer_portal_cancel_plan_command;
use crate::services::storyteller::commands::stripe_customer_portal::storyteller_open_customer_portal_manage_plan_command::storyteller_open_customer_portal_manage_plan_command;
use crate::services::storyteller::commands::stripe_customer_portal::storyteller_open_customer_portal_switch_plan_command::{
  storyteller_open_customer_portal_switch_plan_command, StorytellerOpenCustomerPortalSwitchPlanCommand,
};
use crate::services::storyteller::commands::stripe_customer_portal::storyteller_open_customer_portal_update_payment_method_command::storyteller_open_customer_portal_update_payment_method_command;
use crate::services::storyteller::state::storyteller_credential_manager::StorytellerCredentialManager;
use crate::services::worldlabs::commands::worldlabs_clear_credentials_command::worldlabs_clear_credentials_command;
use crate::services::worldlabs::commands::worldlabs_get_credential_info_command::worldlabs_get_credential_info_command;
use crate::services::worldlabs::commands::worldlabs_open_login_command::worldlabs_open_login_command;
use crate::services::worldlabs::commands::worldlabs_receive_bearer_command::{
  worldlabs_receive_bearer_command, WorldlabsReceiveBearerRequest,
};
use crate::services::worldlabs::state::worldlabs_bearer_bridge::WorldlabsBearerBridge;
use crate::services::worldlabs::state::worldlabs_credential_manager::WorldlabsCredentialManager;
use base64::Engine;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::ipc::{IpcResponse, InvokeResponseBody};
use tauri::Manager;
use tauri_plugin_cli::Matches;

const SAFE_ALLOWLIST: &[&str] = &["platform_info_command", "flip_image"];

// Everything else from the desktop `tauri::generate_handler![...]` list in crates/desktop/artcraft/src/lib.rs.
const UNSAFE_ALLOWLIST: &[&str] = &[
  "check_sora_session_command",
  "download_directory_reveal_command",
  "download_media_file_command",
  "download_url_command",
  "enqueue_edit_image_command",
  "enqueue_image_bg_removal_command",
  "enqueue_image_inpaint_command",
  "enqueue_image_to_3d_object_command",
  "enqueue_image_to_gaussian_command",
  "enqueue_image_to_video_command",
  "enqueue_text_to_image_command",
  "estimate_image_cost_command",
  "estimate_video_cost_command",
  "get_app_info_command",
  "get_app_preferences_command",
  "get_provider_order_command",
  "get_task_queue_command",
  "grok_clear_credentials_command",
  "grok_get_credential_info_command",
  "grok_open_login_command",
  "load_without_cors_command",
  "mark_task_as_dismissed_command",
  "media_file_delete_command",
  "midjourney_clear_credentials_command",
  "midjourney_get_credential_info_command",
  "midjourney_open_login_command",
  "open_sora_login_command",
  "platform_info_command", // also in SAFE_ALLOWLIST; tier resolution checks SAFE first.
  "set_provider_order_command",
  "sora_get_credential_info_command",
  "sora_logout_command",
  "storyteller_get_credits_command",
  "storyteller_get_subscription_command",
  "storyteller_open_credits_purchase_command",
  "storyteller_open_customer_portal_cancel_plan_command",
  "storyteller_open_customer_portal_manage_plan_command",
  "storyteller_open_customer_portal_switch_plan_command",
  "storyteller_open_customer_portal_update_payment_method_command",
  "storyteller_open_subscription_purchase_command",
  "storyteller_purge_credentials_command",
  "tasks_nuke_all_command",
  "update_app_preferences_command",
  "worldlabs_clear_credentials_command",
  "worldlabs_get_credential_info_command",
  "worldlabs_open_login_command",
  "worldlabs_receive_bearer_command",
];

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum InvokeTier {
  Safe,
  Unsafe,
}

fn is_in_list(list: &[&str], command: &str) -> bool {
  list.iter().any(|c| *c == command)
}

fn tier_for_command(command: &str) -> Option<InvokeTier> {
  if is_in_list(SAFE_ALLOWLIST, command) {
    return Some(InvokeTier::Safe);
  }
  if is_in_list(UNSAFE_ALLOWLIST, command) {
    return Some(InvokeTier::Unsafe);
  }
  None
}

fn arg_string(matches: &Matches, name: &str) -> Option<String> {
  matches
    .args
    .get(name)
    .and_then(|arg| arg.value.as_str().map(|s| s.to_string()))
}

fn arg_bool(matches: &Matches, name: &str) -> bool {
  matches
    .args
    .get(name)
    .and_then(|arg| arg.value.as_bool())
    .unwrap_or(false)
}

#[derive(Serialize)]
struct CliErrorDetails {
  code: &'static str,
}

fn cli_error_json(code: &'static str, message: impl Into<String>) -> CommandErrorResponseWrapper<(), CliErrorDetails> {
  CommandErrorResponseWrapper {
    status: CommandErrorStatus::BadRequest,
    error_message: Some(message.into()),
    error_type: None,
    error_details: Some(CliErrorDetails { code }),
  }
}

fn cli_error_json_with_status(
  status: CommandErrorStatus,
  code: &'static str,
  message: impl Into<String>,
) -> CommandErrorResponseWrapper<(), CliErrorDetails> {
  CommandErrorResponseWrapper {
    status,
    error_message: Some(message.into()),
    error_type: None,
    error_details: Some(CliErrorDetails { code }),
  }
}

fn cli_success_json<T: Serialize>(payload: T) -> CommandSuccessResponseWrapper<T> {
  CommandSuccessResponseWrapper {
    status: CommandSuccessStatus::Success,
    success_message: None,
    payload: Some(payload),
  }
}

fn read_payload_source(payload: &str) -> Result<String, String> {
  let payload = payload.trim().to_string();
  if payload.is_empty() {
    return Err("--payload was provided but empty".to_string());
  }

  if let Some(path) = payload.strip_prefix('@') {
    std::fs::read_to_string(path)
      .map_err(|err| format!("failed to read payload file '{path}': {err:?}"))
  } else {
    Ok(payload)
  }
}

fn parse_payload_value(payload: Option<String>) -> Result<Value, String> {
  let Some(payload) = payload else {
    return Err("missing required --payload".to_string());
  };

  let raw = read_payload_source(&payload)?;
  serde_json::from_str(&raw).map_err(|err| format!("invalid JSON payload: {err}"))
}

fn parse_payload_as<T: DeserializeOwned>(payload: Option<String>) -> Result<T, String> {
  let v = parse_payload_value(payload)?;
  serde_json::from_value(v).map_err(|err| format!("invalid JSON payload shape: {err}"))
}

fn parse_string_arg_payload(payload: Option<String>, field_name: &str) -> Result<String, String> {
  let v = parse_payload_value(payload)?;

  match v {
    Value::String(s) => Ok(s),
    Value::Object(map) => map
      .get(field_name)
      .and_then(|v| v.as_str().map(|s| s.to_string()))
      .ok_or_else(|| {
        format!(
          "payload must be a JSON string or object with string field '{field_name}'"
        )
      }),
    _ => Err(format!(
      "payload must be a JSON string or object with string field '{field_name}'"
    )),
  }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CliConfig {
  enable_unsafe_invoke: Option<bool>,
}

fn unsafe_gate_enabled() -> Result<bool, String> {
  if std::env::var("ARTCRAFT_ENABLE_UNSAFE_INVOKE")
    .map(|v| v == "1")
    .unwrap_or(false)
  {
    return Ok(true);
  }

  let Some(config_dir) = dirs::config_dir() else {
    return Ok(false);
  };

  let cli_config_path = config_dir.join("artcraft").join("cli.json");
  if !cli_config_path.exists() {
    return Ok(false);
  }

  let raw = std::fs::read_to_string(&cli_config_path)
    .map_err(|err| format!("failed to read {}: {err}", cli_config_path.display()))?;

  let parsed: CliConfig = serde_json::from_str(&raw)
    .map_err(|err| format!("failed to parse {}: {err}", cli_config_path.display()))?;

  Ok(parsed.enable_unsafe_invoke.unwrap_or(false))
}

fn ensure_provider_priority_store(app: &tauri::App) {
  if app.try_state::<ProviderPriorityStore>().is_some() {
    return;
  }

  let root_state = app.state::<AppDataRoot>();
  let root: &AppDataRoot = &*root_state;

  let provider_priority_store = match ProviderPriorityStore::from_filesystem_configs(root) {
    Ok(Some(store)) => store,
    Ok(None) => ProviderPriorityStore::default(),
    Err(_) => ProviderPriorityStore::default(),
  };

  app.manage(provider_priority_store);
}

fn ensure_task_database(app: &tauri::App) -> Result<(), String> {
  if app.try_state::<TaskDatabase>().is_some() {
    return Ok(());
  }

  let root_state = app.state::<AppDataRoot>();
  let root: &AppDataRoot = &*root_state;

  let connect_result = tauri::async_runtime::block_on(async { TaskDatabase::connect(root).await });

  match connect_result {
    Ok(task_database) => {
      app.manage(task_database);
      Ok(())
    }
    Err(err) => Err(format!("failed to connect task database: {err:?}")),
  }
}

fn print_json<T: Serialize>(val: &T) {
  println!("{}", serde_json::to_string(val).unwrap());
}

fn print_cli_error_and_exit(code: &'static str, message: impl Into<String>, exit_code: i32) -> i32 {
  let err = cli_error_json(code, message);
  print_json(&err);
  exit_code
}

fn print_cli_error_and_exit_with_status(
  status: CommandErrorStatus,
  code: &'static str,
  message: impl Into<String>,
  exit_code: i32,
) -> i32 {
  let err = cli_error_json_with_status(status, code, message);
  print_json(&err);
  exit_code
}

/// Handles: `artcraft invoke <command> [--payload <json|@file>] [--json] [--unsafe] [--list-allowed]`
///
/// Returns the exit code that should be used.
pub fn dispatch_invoke(app: &tauri::App, invoke_matches: &Matches) -> i32 {
  let _json_only = arg_bool(invoke_matches, "json");
  let unsafe_requested = arg_bool(invoke_matches, "unsafe");
  let list_allowed = arg_bool(invoke_matches, "list-allowed");

  if list_allowed {
    let unsafe_gate_enabled = unsafe_gate_enabled().unwrap_or(false);
    let unsafe_list: Vec<&str> = UNSAFE_ALLOWLIST
      .iter()
      .copied()
      .filter(|c| !is_in_list(SAFE_ALLOWLIST, c))
      .collect();

    let payload = serde_json::json!({
      "safe": SAFE_ALLOWLIST,
      "unsafe": unsafe_list,
      "unsafeGateEnabled": unsafe_gate_enabled,
    });
    print_json(&payload);
    return 0;
  }

  let command = match arg_string(invoke_matches, "command") {
    Some(val) => val,
    None => {
      return print_cli_error_and_exit(
        "invalid_args",
        "missing required positional arg: <command> (or pass --list-allowed)",
        2,
      );
    }
  };

  let tier = match tier_for_command(&command) {
    Some(tier) => tier,
    None => {
      return print_cli_error_and_exit(
        "disallowed_command",
        format!("unknown or disallowed command: {command}"),
        3,
      );
    }
  };

  if tier == InvokeTier::Unsafe {
    if !unsafe_requested {
      return print_cli_error_and_exit(
        "unsafe_required",
        format!("command requires --unsafe: {command}"),
        3,
      );
    }

    match unsafe_gate_enabled() {
      Ok(true) => {}
      Ok(false) => {
        return print_cli_error_and_exit(
          "unsafe_gate_disabled",
          "--unsafe requested but gate is disabled; set ARTCRAFT_ENABLE_UNSAFE_INVOKE=1 or ~/.config/artcraft/cli.json with {\"enableUnsafeInvoke\":true}",
          2,
        );
      }
      Err(msg) => {
        return print_cli_error_and_exit("unsafe_gate_error", msg, 2);
      }
    }
  }

  let payload_arg = arg_string(invoke_matches, "payload");

  match command.as_str() {
    // SAFE
    "platform_info_command" => {
      let result = platform_info_command();
      print_json(&result);
      0
    }

    "flip_image" => {
      let image = match parse_string_arg_payload(payload_arg, "image") {
        Ok(v) => v,
        Err(msg) => return print_cli_error_and_exit("invalid_args", msg, 2),
      };

      match flip_image(&image) {
        Ok(out) => {
          let resp = cli_success_json(out);
          print_json(&resp);
          0
        }
        Err(msg) => {
          let err = cli_error_json_with_status(CommandErrorStatus::ServerError, "runtime_invoke_error", msg);
          print_json(&err);
          4
        }
      }
    }

    // UNSAFE
    "check_sora_session_command" => {
      let app_handle = app.handle().clone();
      let result = tauri::async_runtime::block_on(async {
        check_sora_session_command(app_handle, app.state::<SoraCredentialManager>()).await
      });
      match result {
        Ok(ok) => {
          print_json(&ok);
          0
        }
        Err(err) => {
          print_json(&err);
          4
        }
      }
    }

    "download_directory_reveal_command" => {
      let app_handle = app.handle().clone();
      let result = tauri::async_runtime::block_on(async {
        download_directory_reveal_command(
          app_handle,
          app.state::<AppPreferencesManager>(),
          app.state::<AppDataRoot>(),
        )
        .await
      });
      match result {
        Ok(ok) => {
          print_json(&ok);
          0
        }
        Err(err) => {
          print_json(&err);
          4
        }
      }
    }

    "download_media_file_command" => {
      let request: DownloadMediaFileRequest = match parse_payload_as(payload_arg) {
        Ok(r) => r,
        Err(msg) => return print_cli_error_and_exit("invalid_args", msg, 2),
      };

      let app_handle = app.handle().clone();
      let result = tauri::async_runtime::block_on(async {
        download_media_file_command(
          request,
          app_handle,
          app.state::<AppPreferencesManager>(),
          app.state::<AppDataRoot>(),
          app.state::<AppEnvConfigs>(),
        )
        .await
      });

      match result {
        Ok(ok) => {
          print_json(&ok);
          0
        }
        Err(err) => {
          print_json(&err);
          4
        }
      }
    }

    "download_url_command" => {
      let request: DownloadUrlRequest = match parse_payload_as(payload_arg) {
        Ok(r) => r,
        Err(msg) => return print_cli_error_and_exit("invalid_args", msg, 2),
      };

      let app_handle = app.handle().clone();
      let result = tauri::async_runtime::block_on(async {
        download_url_command(
          request,
          app_handle,
          app.state::<AppPreferencesManager>(),
          app.state::<AppDataRoot>(),
          app.state::<AppEnvConfigs>(),
        )
        .await
      });

      match result {
        Ok(ok) => {
          print_json(&ok);
          0
        }
        Err(err) => {
          print_json(&err);
          4
        }
      }
    }

    "enqueue_edit_image_command" => {
      if let Err(msg) = ensure_task_database(app) {
        return print_cli_error_and_exit_with_status(
          CommandErrorStatus::ServerError,
          "runtime_invoke_error",
          msg,
          4,
        );
      }
      ensure_provider_priority_store(app);

      let request: EnqueueEditImageCommand = match parse_payload_as(payload_arg) {
        Ok(r) => r,
        Err(msg) => return print_cli_error_and_exit("invalid_args", msg, 2),
      };
      let app_handle = app.handle().clone();

      let result = tauri::async_runtime::block_on(async {
        enqueue_edit_image_command(
          app_handle,
          request,
          app.state::<AppDataRoot>(),
          app.state::<AppEnvConfigs>(),
          app.state(),
          app.state::<ProviderPriorityStore>(),
          app.state::<TaskDatabase>(),
          app.state::<StorytellerCredentialManager>(),
          app.state::<SoraCredentialManager>(),
          app.state::<SoraTaskQueue>(),
        )
        .await
      });

      match result {
        Ok(ok) => {
          print_json(&ok);
          0
        }
        Err(err) => {
          print_json(&err);
          4
        }
      }
    }

    "enqueue_image_bg_removal_command" => {
      if let Err(msg) = ensure_task_database(app) {
        return print_cli_error_and_exit_with_status(
          CommandErrorStatus::ServerError,
          "runtime_invoke_error",
          msg,
          4,
        );
      }
      ensure_provider_priority_store(app);

      let request: EnqueueImageBgRemovalCommand = match parse_payload_as(payload_arg) {
        Ok(r) => r,
        Err(msg) => return print_cli_error_and_exit("invalid_args", msg, 2),
      };

      let app_handle = app.handle().clone();

      let result = tauri::async_runtime::block_on(async {
        enqueue_image_bg_removal_command(
          app_handle,
          request,
          app.state::<AppDataRoot>(),
          app.state::<AppEnvConfigs>(),
          app.state(),
          app.state::<ProviderPriorityStore>(),
          app.state::<TaskDatabase>(),
          app.state::<StorytellerCredentialManager>(),
        )
        .await
      });

      match result {
        Ok(ok) => {
          print_json(&ok);
          0
        }
        Err(err) => {
          print_json(&err);
          4
        }
      }
    }

    "enqueue_image_inpaint_command" => {
      if let Err(msg) = ensure_task_database(app) {
        return print_cli_error_and_exit_with_status(
          CommandErrorStatus::ServerError,
          "runtime_invoke_error",
          msg,
          4,
        );
      }
      ensure_provider_priority_store(app);

      let request: EnqueueInpaintImageCommand = match parse_payload_as(payload_arg) {
        Ok(r) => r,
        Err(msg) => return print_cli_error_and_exit("invalid_args", msg, 2),
      };

      let app_handle = app.handle().clone();

      let result = tauri::async_runtime::block_on(async {
        enqueue_image_inpaint_command(
          app_handle,
          request,
          app.state::<AppDataRoot>(),
          app.state::<AppEnvConfigs>(),
          app.state(),
          app.state::<ProviderPriorityStore>(),
          app.state::<TaskDatabase>(),
          app.state::<StorytellerCredentialManager>(),
          app.state::<SoraCredentialManager>(),
          app.state::<SoraTaskQueue>(),
        )
        .await
      });

      match result {
        Ok(ok) => {
          print_json(&ok);
          0
        }
        Err(err) => {
          print_json(&err);
          4
        }
      }
    }

    "enqueue_image_to_3d_object_command" => {
      if let Err(msg) = ensure_task_database(app) {
        return print_cli_error_and_exit_with_status(
          CommandErrorStatus::ServerError,
          "runtime_invoke_error",
          msg,
          4,
        );
      }
      ensure_provider_priority_store(app);

      let request: EnqueueImageTo3dObjectRequest = match parse_payload_as(payload_arg) {
        Ok(r) => r,
        Err(msg) => return print_cli_error_and_exit("invalid_args", msg, 2),
      };

      let app_handle = app.handle().clone();

      let result = tauri::async_runtime::block_on(async {
        enqueue_image_to_3d_object_command(
          app_handle,
          request,
          app.state::<AppEnvConfigs>(),
          app.state::<AppDataRoot>(),
          app.state(),
          app.state::<ProviderPriorityStore>(),
          app.state::<TaskDatabase>(),
          app.state::<StorytellerCredentialManager>(),
          app.state::<SoraTaskQueue>(),
        )
        .await
      });

      match result {
        Ok(ok) => {
          print_json(&ok);
          0
        }
        Err(err) => {
          print_json(&err);
          4
        }
      }
    }

    "enqueue_image_to_gaussian_command" => {
      if let Err(msg) = ensure_task_database(app) {
        return print_cli_error_and_exit_with_status(
          CommandErrorStatus::ServerError,
          "runtime_invoke_error",
          msg,
          4,
        );
      }

      let request: EnqueueImageToGaussianRequest = match parse_payload_as(payload_arg) {
        Ok(r) => r,
        Err(msg) => return print_cli_error_and_exit("invalid_args", msg, 2),
      };

      let app_handle = app.handle().clone();

      let result = tauri::async_runtime::block_on(async {
        enqueue_image_to_gaussian_command(
          request,
          app_handle,
          app.state::<AppDataRoot>(),
          app.state::<AppEnvConfigs>(),
          app.state(),
          app.state::<TaskDatabase>(),
          app.state::<StorytellerCredentialManager>(),
          app.state::<WorldlabsCredentialManager>(),
        )
        .await
      });

      match result {
        Ok(ok) => {
          print_json(&ok);
          0
        }
        Err(err) => {
          print_json(&err);
          4
        }
      }
    }

    "enqueue_image_to_video_command" => {
      if let Err(msg) = ensure_task_database(app) {
        return print_cli_error_and_exit_with_status(
          CommandErrorStatus::ServerError,
          "runtime_invoke_error",
          msg,
          4,
        );
      }
      ensure_provider_priority_store(app);

      let request: EnqueueImageToVideoRequest = match parse_payload_as(payload_arg) {
        Ok(r) => r,
        Err(msg) => return print_cli_error_and_exit("invalid_args", msg, 2),
      };

      let app_handle = app.handle().clone();

      let result = tauri::async_runtime::block_on(async {
        enqueue_image_to_video_command(
          request,
          app_handle,
          app.state::<AppEnvConfigs>(),
          app.state::<AppDataRoot>(),
          app.state(),
          app.state::<ProviderPriorityStore>(),
          app.state::<TaskDatabase>(),
          app.state::<GrokCredentialManager>(),
          app.state::<StorytellerCredentialManager>(),
          app.state::<SoraTaskQueue>(),
          app.state::<SoraCredentialManager>(),
        )
        .await
      });

      match result {
        Ok(ok) => {
          print_json(&ok);
          0
        }
        Err(err) => {
          print_json(&err);
          4
        }
      }
    }

    "enqueue_text_to_image_command" => {
      if let Err(msg) = ensure_task_database(app) {
        return print_cli_error_and_exit_with_status(
          CommandErrorStatus::ServerError,
          "runtime_invoke_error",
          msg,
          4,
        );
      }
      ensure_provider_priority_store(app);

      let request: EnqueueTextToImageRequest = match parse_payload_as(payload_arg) {
        Ok(r) => r,
        Err(msg) => return print_cli_error_and_exit("invalid_args", msg, 2),
      };

      let app_handle = app.handle().clone();

      let result = tauri::async_runtime::block_on(async {
        enqueue_text_to_image_command(
          request,
          app_handle,
          app.state::<AppDataRoot>(),
          app.state::<AppEnvConfigs>(),
          app.state(),
          app.state::<ProviderPriorityStore>(),
          app.state::<TaskDatabase>(),
          app.state::<MidjourneyCredentialManager>(),
          app.state::<GrokCredentialManager>(),
          app.state::<GrokImagePromptQueue>(),
          app.state::<StorytellerCredentialManager>(),
          app.state::<SoraCredentialManager>(),
          app.state::<SoraTaskQueue>(),
        )
        .await
      });

      match result {
        Ok(ok) => {
          print_json(&ok);
          0
        }
        Err(err) => {
          print_json(&err);
          4
        }
      }
    }

    "estimate_image_cost_command" => {
      let request: EstimateImageCostRequest = match parse_payload_as(payload_arg) {
        Ok(r) => r,
        Err(msg) => return print_cli_error_and_exit("invalid_args", msg, 2),
      };

      let result = tauri::async_runtime::block_on(async {
        estimate_image_cost_command(request, app.state::<AppEnvConfigs>()).await
      });

      match result {
        Ok(ok) => {
          print_json(&ok);
          0
        }
        Err(err) => {
          print_json(&err);
          4
        }
      }
    }

    "estimate_video_cost_command" => {
      let request: EstimateVideoCostRequest = match parse_payload_as(payload_arg) {
        Ok(r) => r,
        Err(msg) => return print_cli_error_and_exit("invalid_args", msg, 2),
      };

      let result = tauri::async_runtime::block_on(async {
        estimate_video_cost_command(request, app.state::<AppEnvConfigs>()).await
      });

      match result {
        Ok(ok) => {
          print_json(&ok);
          0
        }
        Err(err) => {
          print_json(&err);
          4
        }
      }
    }

    "get_app_info_command" => {
      let result = get_app_info_command(
        app.state::<AppDataRoot>(),
        app.state::<AppEnvConfigs>(),
        app.state::<ArtcraftPlatformInfo>(),
        app.state::<AppPreferencesManager>(),
      );
      print_json(&result);
      0
    }

    "get_app_preferences_command" => {
      let result = tauri::async_runtime::block_on(async {
        get_app_preferences_command(app.state::<AppPreferencesManager>()).await
      });
      match result {
        Ok(ok) => {
          print_json(&ok);
          0
        }
        Err(msg) => {
          let err = cli_error_json_with_status(CommandErrorStatus::ServerError, "runtime_invoke_error", msg);
          print_json(&err);
          4
        }
      }
    }

    "get_provider_order_command" => {
      ensure_provider_priority_store(app);

      let result = tauri::async_runtime::block_on(async {
        get_provider_order_command(app.state::<ProviderPriorityStore>()).await
      });

      match result {
        Ok(ok) => {
          print_json(&ok);
          0
        }
        Err(err) => {
          print_json(&err);
          4
        }
      }
    }

    "get_task_queue_command" => {
      if let Err(msg) = ensure_task_database(app) {
        return print_cli_error_and_exit_with_status(
          CommandErrorStatus::ServerError,
          "runtime_invoke_error",
          msg,
          4,
        );
      }

      let app_handle = app.handle().clone();
      let result = tauri::async_runtime::block_on(async {
        get_task_queue_command(
          app_handle,
          app.state::<AppEnvConfigs>(),
          app.state::<TaskDatabase>(),
        )
        .await
      });

      match result {
        Ok(ok) => {
          print_json(&ok);
          0
        }
        Err(err) => {
          print_json(&err);
          4
        }
      }
    }

    "grok_clear_credentials_command" => {
      let result = tauri::async_runtime::block_on(async {
        grok_clear_credentials_command(app.state::<AppDataRoot>(), app.state::<GrokCredentialManager>()).await
      });

      match result {
        Ok(ok) => {
          print_json(&ok);
          0
        }
        Err(err) => {
          print_json(&err);
          4
        }
      }
    }

    "grok_get_credential_info_command" => {
      let result = tauri::async_runtime::block_on(async {
        grok_get_credential_info_command(app.state::<GrokCredentialManager>()).await
      });

      match result {
        Ok(ok) => {
          print_json(&ok);
          0
        }
        Err(err) => {
          print_json(&err);
          4
        }
      }
    }

    "grok_open_login_command" => {
      let app_handle = app.handle().clone();
      let result = tauri::async_runtime::block_on(async {
        grok_open_login_command(
          app_handle,
          app.state::<AppDataRoot>(),
          app.state::<GrokCredentialManager>(),
        )
        .await
      });

      match result {
        Ok(url) => {
          let resp = cli_success_json(url);
          print_json(&resp);
          0
        }
        Err(msg) => {
          let err = cli_error_json_with_status(CommandErrorStatus::ServerError, "runtime_invoke_error", msg);
          print_json(&err);
          4
        }
      }
    }

    "load_without_cors_command" => {
      let url = match parse_string_arg_payload(payload_arg, "url") {
        Ok(v) => v,
        Err(msg) => return print_cli_error_and_exit("invalid_args", msg, 2),
      };

      let result = tauri::async_runtime::block_on(async { load_without_cors_command(url).await });

      match result {
        Ok(resp) => {
          match resp.body() {
            Ok(InvokeResponseBody::Json(s)) => {
              // The command already produced JSON; wrap for consistency.
              let payload = serde_json::json!({"response": {"type": "json", "value": s}});
              print_json(&payload);
              0
            }
            Ok(InvokeResponseBody::Raw(bytes)) => {
              let b64 = base64::engine::general_purpose::STANDARD.encode(bytes);
              let payload = serde_json::json!({"response": {"type": "raw_base64", "value": b64}});
              print_json(&payload);
              0
            }
            Err(err) => {
              let err = cli_error_json_with_status(
                CommandErrorStatus::ServerError,
                "runtime_invoke_error",
                format!("failed to serialize response: {err}"),
              );
              print_json(&err);
              4
            }
          }
        }
        Err(msg) => {
          let err = cli_error_json_with_status(CommandErrorStatus::ServerError, "runtime_invoke_error", msg);
          print_json(&err);
          4
        }
      }
    }

    "mark_task_as_dismissed_command" => {
      if let Err(msg) = ensure_task_database(app) {
        return print_cli_error_and_exit_with_status(
          CommandErrorStatus::ServerError,
          "runtime_invoke_error",
          msg,
          4,
        );
      }

      let request: MarkTaskAsDismissedRequest = match parse_payload_as(payload_arg) {
        Ok(r) => r,
        Err(msg) => return print_cli_error_and_exit("invalid_args", msg, 2),
      };

      let app_handle = app.handle().clone();
      let result = tauri::async_runtime::block_on(async {
        mark_task_as_dismissed_command(
          request,
          app_handle,
          app.state::<AppEnvConfigs>(),
          app.state::<TaskDatabase>(),
        )
        .await
      });

      match result {
        Ok(ok) => {
          print_json(&ok);
          0
        }
        Err(err) => {
          print_json(&err);
          4
        }
      }
    }

    "media_file_delete_command" => {
      let request: MediaFileDeleteRequest = match parse_payload_as(payload_arg) {
        Ok(r) => r,
        Err(msg) => return print_cli_error_and_exit("invalid_args", msg, 2),
      };

      let app_handle = app.handle().clone();
      let result = tauri::async_runtime::block_on(async {
        media_file_delete_command(
          app_handle,
          request,
          app.state::<AppEnvConfigs>(),
          app.state::<StorytellerCredentialManager>(),
        )
        .await
      });

      match result {
        Ok(ok) => {
          print_json(&ok);
          0
        }
        Err(err) => {
          print_json(&err);
          4
        }
      }
    }

    "midjourney_clear_credentials_command" => {
      let result = tauri::async_runtime::block_on(async {
        midjourney_clear_credentials_command(
          app.state::<AppDataRoot>(),
          app.state::<MidjourneyCredentialManager>(),
        )
        .await
      });

      match result {
        Ok(ok) => {
          print_json(&ok);
          0
        }
        Err(err) => {
          print_json(&err);
          4
        }
      }
    }

    "midjourney_get_credential_info_command" => {
      let result = tauri::async_runtime::block_on(async {
        midjourney_get_credential_info_command(app.state::<MidjourneyCredentialManager>()).await
      });

      match result {
        Ok(ok) => {
          print_json(&ok);
          0
        }
        Err(err) => {
          print_json(&err);
          4
        }
      }
    }

    "midjourney_open_login_command" => {
      let app_handle = app.handle().clone();
      let result = tauri::async_runtime::block_on(async {
        midjourney_open_login_command(
          app_handle,
          app.state::<AppDataRoot>(),
          app.state::<MidjourneyCredentialManager>(),
        )
        .await
      });

      match result {
        Ok(url) => {
          let resp = cli_success_json(url);
          print_json(&resp);
          0
        }
        Err(msg) => {
          let err = cli_error_json_with_status(CommandErrorStatus::ServerError, "runtime_invoke_error", msg);
          print_json(&err);
          4
        }
      }
    }

    "open_sora_login_command" => {
      let app_handle = app.handle().clone();
      let result = tauri::async_runtime::block_on(async {
        open_sora_login_command(
          app_handle,
          app.state::<AppDataRoot>(),
          app.state::<SoraCredentialManager>(),
        )
        .await
      });

      match result {
        Ok(url) => {
          let resp = cli_success_json(url);
          print_json(&resp);
          0
        }
        Err(msg) => {
          let err = cli_error_json_with_status(CommandErrorStatus::ServerError, "runtime_invoke_error", msg);
          print_json(&err);
          4
        }
      }
    }

    "set_provider_order_command" => {
      ensure_provider_priority_store(app);

      let request: SetProviderOrderRequest = match parse_payload_as(payload_arg) {
        Ok(r) => r,
        Err(msg) => return print_cli_error_and_exit("invalid_args", msg, 2),
      };

      let result = tauri::async_runtime::block_on(async {
        set_provider_order_command(
          request,
          app.state::<ProviderPriorityStore>(),
          app.state::<AppDataRoot>(),
        )
        .await
      });

      match result {
        Ok(ok) => {
          print_json(&ok);
          0
        }
        Err(err) => {
          print_json(&err);
          4
        }
      }
    }

    "sora_get_credential_info_command" => {
      let result = tauri::async_runtime::block_on(async {
        sora_get_credential_info_command(app.state::<SoraCredentialManager>()).await
      });

      match result {
        Ok(ok) => {
          print_json(&ok);
          0
        }
        Err(err) => {
          print_json(&err);
          4
        }
      }
    }

    "sora_logout_command" => {
      let result = tauri::async_runtime::block_on(async {
        sora_logout_command(app.state::<SoraCredentialManager>()).await
      });

      match result {
        Ok(ok) => {
          print_json(&ok);
          0
        }
        Err(err) => {
          print_json(&err);
          4
        }
      }
    }

    "storyteller_get_credits_command" => {
      let result = tauri::async_runtime::block_on(async {
        storyteller_get_credits_command(
          app.state::<AppEnvConfigs>(),
          app.state::<StorytellerCredentialManager>(),
        )
        .await
      });

      match result {
        Ok(ok) => {
          print_json(&ok);
          0
        }
        Err(err) => {
          print_json(&err);
          4
        }
      }
    }

    "storyteller_get_subscription_command" => {
      let result = tauri::async_runtime::block_on(async {
        storyteller_get_subscription_command(
          app.state::<AppEnvConfigs>(),
          app.state::<StorytellerCredentialManager>(),
        )
        .await
      });

      match result {
        Ok(ok) => {
          print_json(&ok);
          0
        }
        Err(err) => {
          print_json(&err);
          4
        }
      }
    }

    "storyteller_open_credits_purchase_command" => {
      let request: StorytellerOpenCreditsPurchaseCommand = match parse_payload_as(payload_arg) {
        Ok(r) => r,
        Err(msg) => return print_cli_error_and_exit("invalid_args", msg, 2),
      };
      let app_handle = app.handle().clone();

      let result = tauri::async_runtime::block_on(async {
        storyteller_open_credits_purchase_command(
          app_handle,
          request,
          app.state::<AppDataRoot>(),
          app.state::<AppEnvConfigs>(),
          app.state::<StorytellerCredentialManager>(),
        )
        .await
      });

      match result {
        Ok(url) => {
          let resp = cli_success_json(url);
          print_json(&resp);
          0
        }
        Err(msg) => {
          let err = cli_error_json_with_status(CommandErrorStatus::ServerError, "runtime_invoke_error", msg);
          print_json(&err);
          4
        }
      }
    }

    "storyteller_open_customer_portal_cancel_plan_command" => {
      let app_handle = app.handle().clone();
      let result = tauri::async_runtime::block_on(async {
        storyteller_open_customer_portal_cancel_plan_command(
          app_handle,
          app.state::<AppDataRoot>(),
          app.state::<AppEnvConfigs>(),
          app.state::<StorytellerCredentialManager>(),
        )
        .await
      });

      match result {
        Ok(url) => {
          let resp = cli_success_json(url);
          print_json(&resp);
          0
        }
        Err(msg) => {
          let err = cli_error_json_with_status(CommandErrorStatus::ServerError, "runtime_invoke_error", msg);
          print_json(&err);
          4
        }
      }
    }

    "storyteller_open_customer_portal_manage_plan_command" => {
      let app_handle = app.handle().clone();
      let result = tauri::async_runtime::block_on(async {
        storyteller_open_customer_portal_manage_plan_command(
          app_handle,
          app.state::<AppDataRoot>(),
          app.state::<AppEnvConfigs>(),
          app.state::<StorytellerCredentialManager>(),
        )
        .await
      });

      match result {
        Ok(url) => {
          let resp = cli_success_json(url);
          print_json(&resp);
          0
        }
        Err(msg) => {
          let err = cli_error_json_with_status(CommandErrorStatus::ServerError, "runtime_invoke_error", msg);
          print_json(&err);
          4
        }
      }
    }

    "storyteller_open_customer_portal_switch_plan_command" => {
      let request: StorytellerOpenCustomerPortalSwitchPlanCommand = match parse_payload_as(payload_arg) {
        Ok(r) => r,
        Err(msg) => return print_cli_error_and_exit("invalid_args", msg, 2),
      };
      let app_handle = app.handle().clone();

      let result = tauri::async_runtime::block_on(async {
        storyteller_open_customer_portal_switch_plan_command(
          app_handle,
          request,
          app.state::<AppDataRoot>(),
          app.state::<AppEnvConfigs>(),
          app.state::<StorytellerCredentialManager>(),
        )
        .await
      });

      match result {
        Ok(url) => {
          let resp = cli_success_json(url);
          print_json(&resp);
          0
        }
        Err(msg) => {
          let err = cli_error_json_with_status(CommandErrorStatus::ServerError, "runtime_invoke_error", msg);
          print_json(&err);
          4
        }
      }
    }

    "storyteller_open_customer_portal_update_payment_method_command" => {
      let app_handle = app.handle().clone();
      let result = tauri::async_runtime::block_on(async {
        storyteller_open_customer_portal_update_payment_method_command(
          app_handle,
          app.state::<AppDataRoot>(),
          app.state::<AppEnvConfigs>(),
          app.state::<StorytellerCredentialManager>(),
        )
        .await
      });

      match result {
        Ok(url) => {
          let resp = cli_success_json(url);
          print_json(&resp);
          0
        }
        Err(msg) => {
          let err = cli_error_json_with_status(CommandErrorStatus::ServerError, "runtime_invoke_error", msg);
          print_json(&err);
          4
        }
      }
    }

    "storyteller_open_subscription_purchase_command" => {
      let request: StorytellerOpenSubscriptionPurchaseCommand = match parse_payload_as(payload_arg) {
        Ok(r) => r,
        Err(msg) => return print_cli_error_and_exit("invalid_args", msg, 2),
      };
      let app_handle = app.handle().clone();

      let result = tauri::async_runtime::block_on(async {
        storyteller_open_subscription_purchase_command(
          app_handle,
          request,
          app.state::<AppDataRoot>(),
          app.state::<AppEnvConfigs>(),
          app.state::<StorytellerCredentialManager>(),
        )
        .await
      });

      match result {
        Ok(url) => {
          let resp = cli_success_json(url);
          print_json(&resp);
          0
        }
        Err(msg) => {
          let err = cli_error_json_with_status(CommandErrorStatus::ServerError, "runtime_invoke_error", msg);
          print_json(&err);
          4
        }
      }
    }

    "storyteller_purge_credentials_command" => {
      let app_handle = app.handle().clone();
      let result = tauri::async_runtime::block_on(async {
        storyteller_purge_credentials_command(app_handle, app.state::<StorytellerCredentialManager>()).await
      });

      match result {
        Ok(ok) => {
          print_json(&ok);
          0
        }
        Err(err) => {
          print_json(&err);
          4
        }
      }
    }

    "tasks_nuke_all_command" => {
      if let Err(msg) = ensure_task_database(app) {
        return print_cli_error_and_exit_with_status(
          CommandErrorStatus::ServerError,
          "runtime_invoke_error",
          msg,
          4,
        );
      }

      let result = tauri::async_runtime::block_on(async {
        tasks_nuke_all_command(app.state::<TaskDatabase>()).await
      });

      match result {
        Ok(ok) => {
          print_json(&ok);
          0
        }
        Err(err) => {
          print_json(&err);
          4
        }
      }
    }

    "update_app_preferences_command" => {
      let request: UpdateAppPreferencesRequest = match parse_payload_as(payload_arg) {
        Ok(r) => r,
        Err(msg) => return print_cli_error_and_exit("invalid_args", msg, 2),
      };

      let result = tauri::async_runtime::block_on(async {
        update_app_preferences_command(
          request,
          app.state::<AppPreferencesManager>(),
          app.state::<AppDataRoot>(),
        )
        .await
      });

      match result {
        Ok(ok) => {
          print_json(&ok);
          0
        }
        Err(msg) => {
          let err = cli_error_json_with_status(CommandErrorStatus::ServerError, "runtime_invoke_error", msg);
          print_json(&err);
          4
        }
      }
    }

    "worldlabs_clear_credentials_command" => {
      let result = tauri::async_runtime::block_on(async {
        worldlabs_clear_credentials_command(
          app.state::<AppDataRoot>(),
          app.state::<WorldlabsCredentialManager>(),
        )
        .await
      });

      match result {
        Ok(ok) => {
          print_json(&ok);
          0
        }
        Err(err) => {
          print_json(&err);
          4
        }
      }
    }

    "worldlabs_get_credential_info_command" => {
      let result = tauri::async_runtime::block_on(async {
        worldlabs_get_credential_info_command(app.state::<WorldlabsCredentialManager>()).await
      });

      match result {
        Ok(ok) => {
          print_json(&ok);
          0
        }
        Err(err) => {
          print_json(&err);
          4
        }
      }
    }

    "worldlabs_open_login_command" => {
      let app_handle = app.handle().clone();
      let result = tauri::async_runtime::block_on(async {
        worldlabs_open_login_command(
          app_handle,
          app.state::<AppDataRoot>(),
          app.state::<WorldlabsBearerBridge>(),
          app.state::<WorldlabsCredentialManager>(),
        )
        .await
      });

      match result {
        Ok(url) => {
          let resp = cli_success_json(url);
          print_json(&resp);
          0
        }
        Err(msg) => {
          let err = cli_error_json_with_status(CommandErrorStatus::ServerError, "runtime_invoke_error", msg);
          print_json(&err);
          4
        }
      }
    }

    "worldlabs_receive_bearer_command" => {
      let request: WorldlabsReceiveBearerRequest = match parse_payload_as(payload_arg) {
        Ok(r) => r,
        Err(msg) => return print_cli_error_and_exit("invalid_args", msg, 2),
      };

      let result = tauri::async_runtime::block_on(async {
        worldlabs_receive_bearer_command(
          app.state::<AppDataRoot>(),
          request,
          app.state::<WorldlabsBearerBridge>(),
        )
        .await
      });

      match result {
        Ok(ok) => {
          print_json(&ok);
          0
        }
        Err(err) => {
          print_json(&err);
          4
        }
      }
    }

    // Should be unreachable due to tier_for_command() check, but keep exit code semantics.
    _ => print_cli_error_and_exit(
      "disallowed_command",
      format!("unknown or disallowed command: {command}"),
      3,
    ),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn safe_allowlist_is_strict_minimal() {
    assert_eq!(SAFE_ALLOWLIST, &["platform_info_command", "flip_image"]);
  }

  #[test]
  fn safe_commands_are_not_unknown() {
    for c in SAFE_ALLOWLIST {
      assert!(tier_for_command(c).is_some());
    }
  }
}
