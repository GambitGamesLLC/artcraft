// CLI invoke safety model
//
// ArtCraft exposes a single generic CLI entrypoint: `artcraft invoke <command> ...`.
// By design, this is split into two tiers:
//
// - SAFE (default): read-only / introspection commands only.
//   No file writes, no state mutation, and no network/provider calls that could spend tokens.
//
// - UNSAFE (explicit opt-in): anything that could mutate state, touch sensitive data,
//   or call external providers (including *generation*, which is token/cost spending).
//
// Unsafe invocation requires BOTH:
//   1) `--unsafe` on the CLI, AND
//   2) the unsafe gate enabled (env var or config file; see `unsafe_gate_enabled()`).
//
// If you need to change what is allowed in each tier, edit this file:
//   crates/desktop/artcraft/src/core/cli/invoke_dispatcher.rs
// and update the allowlists below:
//   - SAFE_INVOKE_ALLOWLIST
//   - UNSAFE_INVOKE_ALLOWLIST

use crate::core::commands::get_app_info_command::get_app_info_command;
use crate::core::commands::platform_info_command::platform_info_command;
use crate::core::commands::providers::get_provider_order_command::get_provider_order_command;
use crate::core::commands::response::failure_response_wrapper::CommandErrorResponseWrapper;
use crate::core::commands::task_queue::get_task_queue_command::get_task_queue_command;
use crate::core::state::app_env_configs::app_env_configs::AppEnvConfigs;
use crate::core::state::data_dir::app_data_root::AppDataRoot;
use crate::core::state::provider_priority::ProviderPriorityStore;
use crate::core::state::task_database::TaskDatabase;
use serde::Deserialize;
use serde_json::Value;
use tauri::Manager;
use tauri_plugin_cli::Matches;

const CMD_PLATFORM_INFO: &str = "platform_info_command";
const CMD_GET_APP_INFO: &str = "get_app_info_command";
const CMD_GET_TASK_QUEUE: &str = "get_task_queue_command";

const CMD_GET_PROVIDER_ORDER: &str = "get_provider_order_command";

/// Safe commands are intentionally read-only.
const SAFE_INVOKE_ALLOWLIST: [&str; 3] = [CMD_PLATFORM_INFO, CMD_GET_APP_INFO, CMD_GET_TASK_QUEUE];

/// Unsafe commands require BOTH `--unsafe` and an enabled gate.
const UNSAFE_INVOKE_ALLOWLIST: [&str; 1] = [CMD_GET_PROVIDER_ORDER];

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

fn parse_payload(payload: Option<String>) -> Result<Option<Value>, String> {
  let Some(payload) = payload else {
    return Ok(None);
  };

  let payload = payload.trim().to_string();
  if payload.is_empty() {
    return Err("--payload was provided but empty".to_string());
  }

  let raw = if let Some(path) = payload.strip_prefix('@') {
    std::fs::read_to_string(path)
      .map_err(|err| format!("failed to read payload file '{path}': {err:?}"))?
  } else {
    payload
  };

  let parsed: Value = serde_json::from_str(&raw)
    .map_err(|err| format!("invalid JSON payload: {err}"))?;

  Ok(Some(parsed))
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

/// Handles: `artcraft invoke <command> [--payload <json|@file>] [--json]`
///
/// Returns the exit code that should be used.
pub fn dispatch_invoke(app: &tauri::App, invoke_matches: &Matches) -> i32 {
  let _json_only = arg_bool(invoke_matches, "json");
  let unsafe_requested = arg_bool(invoke_matches, "unsafe");

  let command = match arg_string(invoke_matches, "command") {
    Some(val) => val,
    None => {
      let err = CommandErrorResponseWrapper::<(), ()>::from(
        "missing required positional arg: <command>",
      );
      println!("{}", serde_json::to_string(&err).unwrap());
      return 2;
    }
  };

  // Validate payload upfront (even if the allowlisted commands don't use it yet).
  if let Err(msg) = parse_payload(arg_string(invoke_matches, "payload")) {
    let err = CommandErrorResponseWrapper::<(), ()>::from(msg);
    println!("{}", serde_json::to_string(&err).unwrap());
    return 2;
  }

  if unsafe_requested {
    match unsafe_gate_enabled() {
      Ok(true) => {}
      Ok(false) => {
        let err = CommandErrorResponseWrapper::<(), ()>::from(
          "--unsafe requested but gate is disabled; set ARTCRAFT_ENABLE_UNSAFE_INVOKE=1 or ~/.config/artcraft/cli.json with {\"enableUnsafeInvoke\":true}",
        );
        println!("{}", serde_json::to_string(&err).unwrap());
        return 2;
      }
      Err(msg) => {
        let err = CommandErrorResponseWrapper::<(), ()>::from(msg);
        println!("{}", serde_json::to_string(&err).unwrap());
        return 2;
      }
    }
  }

  match command.as_str() {
    CMD_PLATFORM_INFO => {
      let result = platform_info_command();
      println!("{}", serde_json::to_string(&result).unwrap());
      0
    }

    CMD_GET_APP_INFO => {
      let result = get_app_info_command(
        app.state::<AppDataRoot>(),
        app.state::<AppEnvConfigs>(),
        app.state(),
        app.state(),
      );
      println!("{}", serde_json::to_string(&result).unwrap());
      0
    }

    CMD_GET_TASK_QUEUE => {
      // The GUI path registers TaskDatabase during startup; the CLI path must bootstrap it.
      if app.try_state::<TaskDatabase>().is_none() {
        let root_state = app.state::<AppDataRoot>();
        let root: &AppDataRoot = &*root_state;

        let connect_result = tauri::async_runtime::block_on(async {
          TaskDatabase::connect(root).await
        });

        match connect_result {
          Ok(task_database) => {
            app.manage(task_database);
          }
          Err(err) => {
            let err = CommandErrorResponseWrapper::<(), ()>::from(format!(
              "failed to connect task database: {err:?}"
            ));
            println!("{}", serde_json::to_string(&err).unwrap());
            return 4;
          }
        }
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
        Ok(success) => {
          println!("{}", serde_json::to_string(&success).unwrap());
          0
        }
        Err(err) => {
          println!("{}", serde_json::to_string(&err).unwrap());
          4
        }
      }
    }

    CMD_GET_PROVIDER_ORDER if unsafe_requested => {
      ensure_provider_priority_store(app);

      let result = tauri::async_runtime::block_on(async {
        get_provider_order_command(app.state::<ProviderPriorityStore>()).await
      });

      match result {
        Ok(success) => {
          println!("{}", serde_json::to_string(&success).unwrap());
          0
        }
        Err(err) => {
          println!("{}", serde_json::to_string(&err).unwrap());
          4
        }
      }
    }

    _ => {
      let err = CommandErrorResponseWrapper::<(), ()>::from(format!(
        "unknown or disallowed command: {command}. Safe allowlist: {:?}. Unsafe allowlist (requires --unsafe + gate): {:?}",
        SAFE_INVOKE_ALLOWLIST, UNSAFE_INVOKE_ALLOWLIST
      ));
      println!("{}", serde_json::to_string(&err).unwrap());
      3
    }
  }
}
