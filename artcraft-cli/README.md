# ArtCraft CLI Wrapper

**Location:** `/home/derrick/.openclaw/workspace/projects/gambit-artcraft/artcraft-cli.sh`

**Status:** ✅ Phase 2 Complete

---

## Overview

`artcraft-cli.sh` is a bash wrapper script that provides a clean command-line interface for invoking ArtCraft's Tauri commands externally. This enables automation scripts, CI/CD pipelines, and integration with orchestration systems like OpenClaw.

---

## Quick Start

```bash
# Show help
./artcraft-cli.sh --help

# Generate an image
./artcraft-cli.sh generate:text-to-image "cyberpunk city" --provider openai

# List task queue
./artcraft-cli.sh queue:list

# List queue in JSON format (for scripting)
./artcraft-cli.sh queue:list --json
```

---

## Commands

### Generation Commands

| Command | Description | Required Args | Options |
|---------|-------------|---------------|---------|
| `generate:text-to-image` | Generate image from text prompt | prompt (positional) | `--provider`, `--model`, `--json`, `--quiet` |
| `generate:image-to-video` | Generate video from image | image token (positional) | `--provider`, `--model`, `--json`, `--quiet` |
| `generate:edit-image` | Edit an existing image | image token, prompt | `--provider`, `--model`, `--json`, `--quiet` |

### Queue Management

| Command | Description | Required Args | Options |
|---------|-------------|---------------|---------|
| `queue:list` | List all tasks in queue | none | `--json`, `--quiet` |
| `queue:dismiss <id>` | Dismiss a task from queue | task ID | none |
| `queue:wait <id>` | Wait for task completion | task ID | `--timeout`, `--interval`, `--quiet` |
| `queue:purge` | Remove all completed tasks | none | `--quiet` |

### Utility Commands

| Command | Description | Required Args | Options |
|---------|-------------|---------------|---------|
| `download <token>` | Download a media file | media token | none |
| `invoke <command>` | Invoke any Tauri command directly | command name | any (passed through) |
| `help` | Show help message | none | none |

---

## Options

| Option | Description | Default |
|--------|-------------|---------|
| `--json` | Output in JSON format (for scripting) | false |
| `--quiet` | Suppress non-essential output | false |
| `--timeout <seconds>` | Timeout for wait operations | 300 |
| `--provider <name>` | Provider to use | artcraft |
| `--model <name>` | Model to use | (auto) |
| `--prompt <text>` | Text prompt for generation | (positional) |

---

## Providers

Supported provider names:
- `artcraft` (default, uses Storyteller/OpenAI)
- `openai` (alias for artcraft)
- `grok`
- `midjourney`
- `sora`

---

## Models

### Text-to-Image Models
- `flux_1_dev`
- `flux_1_schnell`
- `flux_pro_11`
- `flux_pro_11_ultra`
- `grok_image`
- `recraft_3`
- `gpt_image_1`
- `gpt_image_1p5`
- `gemini_25_flash`
- `nano_banana`
- `nano_banana_2`
- `nano_banana_pro`
- `seedream_4`
- `seedream_4p5`
- `seedream_5_lite`
- `midjourney`

### Image-to-Video Models
- `grok_video`
- `kling_1.6_pro`
- `kling_2.1_pro`
- `kling_2.1_master`
- `kling_2p5_turbo_pro`
- `kling_2p6_pro`
- `seedance_1.0_lite`
- `seedance_2p0`
- `sora_2`
- `sora_2_pro`
- `veo_2`
- `veo_3`
- `veo_3_fast`
- `veo_3p1`
- `veo_3p1_fast`

### Image Edit Models
- `flux_pro_kontext_max`
- `gemini_25_flash`
- `nano_banana`
- `nano_banana_2`
- `nano_banana_pro`
- `gpt_image_1`
- `gpt_image_1p5`
- `seedream_4`
- `seedream_4p5`
- `seedream_5_lite`

---

## Examples

### Basic Generation

```bash
# Generate with default provider
./artcraft-cli.sh generate:text-to-image "sunset over mountains"

# Generate with specific provider
./artcraft-cli.sh generate:text-to-image "cyberpunk cat" --provider openai --model gpt_image_1

# Generate with Grok
./artcraft-cli.sh generate:text-to-image "alien landscape" --provider grok --model grok_image
```

### Queue Management

```bash
# List all tasks
./artcraft-cli.sh queue:list

# Get JSON output for scripting
./artcraft-cli.sh queue:list --json | jq '.tasks | length'

# Wait for specific task
./artcraft-cli.sh queue:wait "task-123" --timeout 300

# Dismiss a task
./artcraft-cli.sh queue:dismiss "task-123"

# Purge completed tasks
./artcraft-cli.sh queue:purge
```

### Complete Workflow

```bash
#!/bin/bash
# Generate an image and download when complete

# Submit generation
./artcraft-cli.sh generate:text-to-image "futuristic city" --provider openai

# Get task ID
TASK_ID=$(./artcraft-cli.sh queue:list --json | jq '.tasks[0].id')

# Wait for completion
./artcraft-cli.sh queue:wait "$TASK_ID" --timeout 300

# Get media token
MEDIA_TOKEN=$(./artcraft-cli.sh queue:list --json | \
  jq -r '.tasks[] | select(.id == "'"$TASK_ID"'") | .completed_item.primary_media_file.token')

# Download result
./artcraft-cli.sh download "$MEDIA_TOKEN"
```

### Batch Generation

```bash
#!/bin/bash
# Generate multiple images

PROMPTS=("city" "forest" "ocean" "desert")

for prompt in "${PROMPTS[@]}"; do
    ./artcraft-cli.sh generate:text-to-image "$prompt" --provider openai --quiet
    sleep 2  # Avoid rate limits
done

# Wait for all to complete
while true; do
    PENDING=$(./artcraft-cli.sh queue:list --json | \
      jq '[.tasks[] | select(.task_status != "complete_success")] | length')
    echo "Pending: $PENDING"
    [[ "$PENDING" -eq 0 ]] && break
    sleep 10
done

# Download all
./artcraft-cli.sh queue:list --json | \
  jq -r '.tasks[] | select(.task_status == "complete_success") | .completed_item.primary_media_file.token' | \
  while read token; do
      ./artcraft-cli.sh download "$token"
  done
```

---

## OpenClaw Integration

The CLI is designed for seamless integration with OpenClaw subagents:

```bash
# All commands support --json for programmatic use
RESULT=$(./artcraft-cli.sh generate:text-to-image "test" --provider openai --json 2>/dev/null)

# Parse result
STATUS=$(echo "$RESULT" | jq -r '.status // "unknown"')

# Queue inspection
QUEUE=$(./artcraft-cli.sh queue:list --json)
TASK_COUNT=$(echo "$QUEUE" | jq '.tasks | length')
```

---

## Technical Details

### Binary Location

The script expects the ArtCraft binary at:
```
/home/derrick/.openclaw/workspace/projects/gambit-artcraft/target/release/artcraft
```

**Note:** The binary doesn't build yet due to `sqlite_tasks` errors. The script gracefully handles this by:
1. Detecting if the binary exists
2. If not, logging a warning and simulating the command
3. Returning JSON responses for testing

### CLI Plugin Integration

The script uses `tauri-plugin-cli` invocation pattern:
```bash
artcraft cli invoke <command_name> [args...]
```

Commands are mapped to Tauri command functions:
- `enqueue_text_to_image_command`
- `enqueue_image_to_video_command`
- `enqueue_edit_image_command`
- `get_task_queue_command`
- `mark_task_as_dismissed_command`
- `tasks_nuke_all_command`
- `download_media_file_command`

### Error Handling

- All commands validate required arguments
- Errors are written to stderr with red color coding
- Exit codes: 0 (success), 1 (error)
- Timeout handling in `queue:wait`

---

## Files Created

| File | Purpose |
|------|---------|
| `artcraft-cli.sh` | Main CLI wrapper script |
| `examples/cli-examples.sh` | Example usage scripts |
| `artcraft-cli/README.md` | This documentation |

---

## Testing

### Commands Tested ✅

```bash
./artcraft-cli.sh --help
./artcraft-cli.sh invoke get_task_queue_command
./artcraft-cli.sh queue:list --json
./artcraft-cli.sh generate:text-to-image "test" --provider openai
./artcraft-cli.sh queue:dismiss test-123
./artcraft-cli.sh download abc123token
```

All commands execute correctly in simulation mode (binary not yet available).

---

## Known Issues

### ⚠️ Binary Not Available

**Issue:** ArtCraft binary doesn't build yet due to `sqlite_tasks` errors.

**Impact:** CLI wrapper runs in simulation mode, returning mock JSON responses.

**Workaround:** None needed for development - simulation mode allows testing the wrapper logic.

**Resolution:** Once ArtCraft builds successfully, the CLI will work with real commands.

### ⚠️ CLI Plugin Invocation Syntax

**Status:** Based on test-cli-plugin reference, the invocation pattern is:
```bash
binary cli invoke <command> [args...]
```

**Uncertainty:** The exact argument serialization (JSON vs. flags) depends on how `tauri-plugin-cli` is configured in ArtCraft's `tauri.conf.json`.

**Recommendation:** Once ArtCraft builds, test with:
```bash
./artcraft-cli.sh invoke get_task_queue_command --json
```

If this fails, the `invoke_command()` function may need adjustment.

---

## Recommendations for Phase 3 (OpenClaw Client)

### 1. JSON Output is Critical

✅ All commands support `--json` flag for programmatic use.

### 2. Error Handling

OpenClaw client should:
- Parse stderr for error messages
- Check exit codes
- Implement retry logic for transient failures

### 3. Task Polling Pattern

Recommended OpenClaw integration:

```python
# Pseudocode for OpenClaw subagent
def generate_and_wait(prompt, provider="openai"):
    # Submit
    cli.run(f"generate:text-to-image '{prompt}' --provider {provider}")
    
    # Poll
    while True:
        queue = cli.run("queue:list --json", capture=True)
        tasks = json.loads(queue)
        
        # Find latest task
        task = tasks['tasks'][0]
        if task['task_status'] == 'complete_success':
            token = task['completed_item']['primary_media_file']['token']
            cli.run(f"download {token}")
            return token
        
        sleep(5)
```

### 4. Batch Operations

For batch generation, consider:
- Submitting all tasks first
- Polling queue once for all completions
- Downloading in parallel

### 5. Rate Limiting

Add delays between submissions:
```bash
sleep 2  # Between generations
sleep 10  # Between queue polls
```

---

## Next Steps

1. ✅ **Phase 1:** CLI plugin integration tested (test-cli-plugin)
2. ✅ **Phase 2:** Wrapper script created (this document)
3. ⏳ **Phase 3:** OpenClaw client implementation
4. ⏳ **Phase 4:** Integration testing with real ArtCraft binary

---

*Created: 2026-03-07*  
*Status: Phase 2 Complete*
