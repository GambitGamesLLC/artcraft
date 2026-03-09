#!/bin/bash
# ArtCraft CLI Wrapper (minimal)
#
# This wrapper targets the generic external entrypoint:
#   artcraft invoke <command> [--payload <json|@file>] [--json]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ARTCRAFT_DIR="$SCRIPT_DIR"
CLI_BINARY="$ARTCRAFT_DIR/target/release/artcraft"

log_info() { printf '[INFO] %s\n' "$1"; }
log_error() { printf '[ERROR] %s\n' "$1" >&2; }

show_help() {
  cat <<'EOF'
ArtCraft CLI Wrapper

Usage:
  ./artcraft-cli.sh invoke <command> [--payload <json|@file>] [--json]

Convenience commands:
  ./artcraft-cli.sh platform:info [--json]
  ./artcraft-cli.sh app:info [--json]
  ./artcraft-cli.sh queue:list [--json]

Examples:
  ./artcraft-cli.sh platform:info --json
  ./artcraft-cli.sh app:info --json
  ./artcraft-cli.sh queue:list --json
  ./artcraft-cli.sh invoke platform_info_command --json
  ./artcraft-cli.sh invoke get_task_queue_command --json

Notes:
  - The underlying binary must exist at: ./target/release/artcraft
  - The current build allowlists a small subset of commands for CLI automation.
EOF
}

invoke() {
  local command="${1:-}"
  shift || true

  if [[ -z "$command" ]]; then
    log_error "Command name is required"
    exit 2
  fi

  if [[ -x "$CLI_BINARY" ]]; then
    "$CLI_BINARY" invoke "$command" "$@"
  else
    # Simulation mode (useful when the binary is not built yet)
    log_error "Binary not found/executable at $CLI_BINARY"
    printf '{"status":"simulated","command":%q}\n' "$command"
    exit 1
  fi
}

main() {
  if [[ $# -eq 0 ]]; then
    show_help
    exit 0
  fi

  local cmd="$1"
  shift || true

  case "$cmd" in
    invoke)
      invoke "$@"
      ;;

    platform:info)
      invoke platform_info_command "$@"
      ;;

    app:info)
      invoke get_app_info_command "$@"
      ;;

    queue:list)
      invoke get_task_queue_command "$@"
      ;;

    help|--help|-h)
      show_help
      ;;

    *)
      log_error "Unknown command: $cmd"
      show_help
      exit 2
      ;;
  esac
}

main "$@"
