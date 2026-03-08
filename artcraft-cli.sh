#!/bin/bash
# ArtCraft CLI Wrapper
# Enables external automation of ArtCraft AI generation

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ARTCRAFT_DIR="$SCRIPT_DIR"
CLI_BINARY="$ARTCRAFT_DIR/target/release/artcraft"  # Will be built later

# Helper functions (with color support)
log_info() { printf '\033[0;32m[INFO]\033[0m %s\n' "$1"; }
log_error() { printf '\033[0;31m[ERROR]\033[0m %s\n' "$1" >&2; }
log_warn() { printf '\033[1;33m[WARN]\033[0m %s\n' "$1"; }

# Show help
show_help() {
    cat << EOF
ArtCraft CLI - External Automation Interface

Usage: $0 <command> [options]

Commands:
  generate:text-to-image    Generate image from text prompt
  generate:image-to-video   Generate video from image
  generate:edit-image       Edit an existing image
  queue:list               List all tasks in queue
  queue:dismiss <id>       Dismiss a task from queue
  queue:wait <id>          Wait for task completion
  queue:purge              Remove all completed tasks
  download <token>         Download a media file
  invoke <command>         Invoke any Tauri command directly
  help                     Show this help message

Options:
  --json                   Output in JSON format (for scripting)
  --quiet                  Suppress non-essential output
  --timeout <seconds>      Timeout for wait operations (default: 300)
  --provider <name>        Provider to use (openai, grok, midjourney, artcraft)
  --model <name>           Model to use
  --prompt <text>          Text prompt for generation

Examples:
  $0 generate:text-to-image "cyberpunk city" --provider openai
  $0 queue:list --json
  $0 invoke get_task_queue_command
  $0 invoke enqueue_text_to_image_command --prompt "cat robot" --provider openai
  $0 download abc123token

EOF
}

# Core function: Invoke a Tauri command via CLI plugin
invoke_command() {
    local command="$1"
    shift
    
    if [[ -z "$command" ]]; then
        log_error "Command name is required"
        exit 1
    fi
    
    # Build the CLI invocation
    # tauri-plugin-cli uses: binary cli invoke <command> [args...]
    if [[ -x "$CLI_BINARY" ]]; then
        "$CLI_BINARY" cli invoke "$command" "$@"
    else
        log_warn "Binary not found at $CLI_BINARY"
        log_info "Simulating command: $command $*"
        # Return simulated JSON response for testing
        echo '{"status": "simulated", "command": "'"$command"'", "args": '"$(echo "$@" | jq -R -s -c 'split(" ")')"'}'
    fi
}

# Command implementations
cmd_generate_text_to_image() {
    local prompt=""
    local provider="artcraft"
    local model=""
    local json=false
    local quiet=false
    
    while [[ $# -gt 0 ]]; do
        case $1 in
            --provider) provider="$2"; shift 2 ;;
            --model) model="$2"; shift 2 ;;
            --json) json=true; shift ;;
            --quiet) quiet=true; shift ;;
            --prompt) prompt="$2"; shift 2 ;;
            *) 
                # If it looks like a prompt (not starting with --), treat as prompt
                if [[ ! "$1" =~ ^-- ]]; then
                    if [[ -z "$prompt" ]]; then
                        prompt="$1"
                    fi
                fi
                shift ;;
        esac
    done
    
    if [[ -z "$prompt" ]]; then
        log_error "Prompt is required"
        exit 1
    fi
    
    if [[ "$quiet" != "true" ]]; then
        log_info "Generating image with prompt: $prompt"
        log_info "Provider: $provider"
        [[ -n "$model" ]] && log_info "Model: $model"
    fi
    
    # Map provider names to ArtCraft's GenerationProvider enum
    local gen_provider=""
    case "$provider" in
        openai|artcraft) gen_provider="artcraft" ;;
        grok) gen_provider="grok" ;;
        midjourney) gen_provider="midjourney" ;;
        sora) gen_provider="sora" ;;
        *) gen_provider="$provider" ;;
    esac
    
    # Invoke Tauri command
    # enqueue_text_to_image_command expects a JSON request object
    # For CLI plugin, we pass args that get serialized
    invoke_command "enqueue_text_to_image_command" \
        --prompt "$prompt" \
        --provider "$gen_provider" \
        ${model:+--model "$model"} \
        ${json:+--json}
}

cmd_generate_image_to_video() {
    local image_token=""
    local model=""
    local provider="artcraft"
    local json=false
    local quiet=false
    
    while [[ $# -gt 0 ]]; do
        case $1 in
            --provider) provider="$2"; shift 2 ;;
            --model) model="$2"; shift 2 ;;
            --json) json=true; shift ;;
            --quiet) quiet=true; shift ;;
            --image-token) image_token="$2"; shift 2 ;;
            *) 
                if [[ ! "$1" =~ ^-- ]] && [[ -z "$image_token" ]]; then
                    image_token="$1"
                fi
                shift ;;
        esac
    done
    
    if [[ -z "$image_token" ]]; then
        log_error "Image token is required"
        exit 1
    fi
    
    if [[ "$quiet" != "true" ]]; then
        log_info "Generating video from image token: $image_token"
        log_info "Provider: $provider"
        [[ -n "$model" ]] && log_info "Model: $model"
    fi
    
    invoke_command "enqueue_image_to_video_command" \
        --image_media_token "$image_token" \
        --provider "$provider" \
        ${model:+--model "$model"} \
        ${json:+--json}
}

cmd_generate_edit_image() {
    local image_token=""
    local prompt=""
    local model=""
    local provider="artcraft"
    local json=false
    local quiet=false
    
    while [[ $# -gt 0 ]]; do
        case $1 in
            --provider) provider="$2"; shift 2 ;;
            --model) model="$2"; shift 2 ;;
            --json) json=true; shift ;;
            --quiet) quiet=true; shift ;;
            --prompt) prompt="$2"; shift 2 ;;
            --image-token) image_token="$2"; shift 2 ;;
            *) 
                if [[ ! "$1" =~ ^-- ]]; then
                    if [[ -z "$prompt" ]]; then
                        prompt="$1"
                    elif [[ -z "$image_token" ]]; then
                        image_token="$1"
                    fi
                fi
                shift ;;
        esac
    done
    
    if [[ -z "$prompt" ]] || [[ -z "$image_token" ]]; then
        log_error "Both prompt and image token are required"
        exit 1
    fi
    
    if [[ "$quiet" != "true" ]]; then
        log_info "Editing image with prompt: $prompt"
        log_info "Image token: $image_token"
        log_info "Provider: $provider"
        [[ -n "$model" ]] && log_info "Model: $model"
    fi
    
    invoke_command "enqueue_edit_image_command" \
        --prompt "$prompt" \
        --image_media_token "$image_token" \
        --provider "$provider" \
        ${model:+--model "$model"} \
        ${json:+--json}
}

cmd_queue_list() {
    local json=false
    local quiet=false
    
    while [[ $# -gt 0 ]]; do
        case $1 in
            --json) json=true; shift ;;
            --quiet) quiet=true; shift ;;
            *) shift ;;
        esac
    done
    
    invoke_command "get_task_queue_command" ${json:+--json}
}

cmd_queue_dismiss() {
    local task_id="$1"
    shift
    
    if [[ -z "$task_id" ]]; then
        log_error "Task ID is required"
        exit 1
    fi
    
    invoke_command "mark_task_as_dismissed_command" --task "$task_id"
}

cmd_queue_wait() {
    local task_id="$1"
    local timeout=300
    local interval=2
    local quiet=false
    
    shift
    
    while [[ $# -gt 0 ]]; do
        case $1 in
            --timeout) timeout="$2"; shift 2 ;;
            --interval) interval="$2"; shift 2 ;;
            --quiet) quiet=true; shift ;;
            *) shift ;;
        esac
    done
    
    if [[ -z "$task_id" ]]; then
        log_error "Task ID is required"
        exit 1
    fi
    
    if [[ "$quiet" != "true" ]]; then
        log_info "Waiting for task $task_id (timeout: ${timeout}s, interval: ${interval}s)"
    fi
    
    # Poll until task completes or timeout
    local elapsed=0
    while [[ $elapsed -lt $timeout ]]; do
        result=$(invoke_command "get_task_queue_command" --json 2>/dev/null || echo "{}")
        
        # Check if task is complete (look for task status in response)
        # This is a simplified check - in production, parse JSON properly
        if echo "$result" | grep -q "\"task_status\":\"complete_success\""; then
            if [[ "$quiet" != "true" ]]; then
                log_info "Task $task_id completed successfully"
            fi
            echo "$result"
            return 0
        fi
        
        sleep "$interval"
        elapsed=$((elapsed + interval))
    done
    
    log_error "Timeout waiting for task $task_id"
    exit 1
}

cmd_queue_purge() {
    local quiet=false
    
    while [[ $# -gt 0 ]]; do
        case $1 in
            --quiet) quiet=true; shift ;;
            *) shift ;;
        esac
    done
    
    if [[ "$quiet" != "true" ]]; then
        log_info "Purging all completed tasks"
    fi
    
    invoke_command "tasks_nuke_all_command"
}

cmd_download() {
    local media_token="$1"
    shift
    
    if [[ -z "$media_token" ]]; then
        log_error "Media token is required"
        exit 1
    fi
    
    log_info "Downloading media file: $media_token"
    
    invoke_command "download_media_file_command" --media_token "$media_token"
}

cmd_invoke() {
    local command="$1"
    shift
    
    if [[ -z "$command" ]]; then
        log_error "Command name is required"
        exit 1
    fi
    
    log_info "Invoking command: $command"
    
    # Pass through all remaining arguments
    invoke_command "$command" "$@"
}

# Main entry point
main() {
    if [[ $# -eq 0 ]]; then
        show_help
        exit 0
    fi
    
    local command="$1"
    shift
    
    case $command in
        generate:text-to-image) cmd_generate_text_to_image "$@" ;;
        generate:image-to-video) cmd_generate_image_to_video "$@" ;;
        generate:edit-image) cmd_generate_edit_image "$@" ;;
        queue:list) cmd_queue_list "$@" ;;
        queue:dismiss) cmd_queue_dismiss "$@" ;;
        queue:wait) cmd_queue_wait "$@" ;;
        queue:purge) cmd_queue_purge "$@" ;;
        download) cmd_download "$@" ;;
        invoke) cmd_invoke "$@" ;;
        help|--help|-h) show_help ;;
        *)
            log_error "Unknown command: $command"
            show_help
            exit 1
            ;;
    esac
}

main "$@"
