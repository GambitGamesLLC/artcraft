#!/bin/bash
# ArtCraft CLI - Example Usage Scripts
# Demonstrates practical automation workflows

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CLI="$SCRIPT_DIR/../artcraft-cli.sh"

echo "========================================"
echo "ArtCraft CLI - Example Workflows"
echo "========================================"
echo

# Example 1: Generate a text-to-image
echo "Example 1: Generate image from text prompt"
echo "-------------------------------------------"
echo "Command: $CLI generate:text-to-image \"futuristic cat robot\" --provider openai"
echo
# Uncomment to run:
# $CLI generate:text-to-image "futuristic cat robot" --provider openai
echo

# Example 2: List queue in JSON format
echo "Example 2: List task queue (JSON output)"
echo "-------------------------------------------"
echo "Command: $CLI queue:list --json"
echo
# Uncomment to run:
# $CLI queue:list --json
echo

# Example 3: Generate and wait for completion
echo "Example 3: Generate image and wait for completion"
echo "--------------------------------------------------"
cat << 'EOF'
#!/bin/bash
# Generate an image and wait for it to complete

# Step 1: Submit generation request
$CLI generate:text-to-image "cyberpunk city at night" --provider openai

# Step 2: Get the task ID from the queue
TASK_ID=$($CLI queue:list --json | jq '.tasks[0].id')
echo "Task ID: $TASK_ID"

# Step 3: Wait for completion (timeout: 300 seconds)
$CLI queue:wait "$TASK_ID" --timeout 300

# Step 4: Get the media token from completed task
MEDIA_TOKEN=$($CLI queue:list --json | jq -r '.tasks[] | select(.id == "'"$TASK_ID"'") | .completed_item.primary_media_file.token')
echo "Media token: $MEDIA_TOKEN"

# Step 5: Download the result
$CLI download "$MEDIA_TOKEN"

echo "Download complete!"
EOF
echo

# Example 4: Image-to-video generation
echo "Example 4: Generate video from image"
echo "-------------------------------------"
cat << 'EOF'
#!/bin/bash
# Generate video from an existing image

# Assume we have an image token from a previous generation
IMAGE_TOKEN="your_image_token_here"

# Submit video generation
$CLI generate:image-to-video "$IMAGE_TOKEN" --model sora_2 --provider sora

# Wait for video to complete
TASK_ID=$($CLI queue:list --json | jq '.tasks[] | select(.task_type == "image_to_video") | .id' | head -1)
$CLI queue:wait "$TASK_ID" --timeout 600  # Videos take longer

echo "Video generation complete!"
EOF
echo

# Example 5: Image editing
echo "Example 5: Edit an existing image"
echo "----------------------------------"
cat << 'EOF'
#!/bin/bash
# Edit an image with a text prompt

IMAGE_TOKEN="your_image_token_here"
PROMPT="add sunglasses and a hat"

$CLI generate:edit-image "$IMAGE_TOKEN" --prompt "$PROMPT" --provider openai

# Wait and download as before
TASK_ID=$($CLI queue:list --json | jq '.tasks[0].id')
$CLI queue:wait "$TASK_ID" --timeout 300
$CLI download "$($CLI queue:list --json | jq -r '.tasks[0].completed_item.primary_media_file.token')"
EOF
echo

# Example 6: Queue management
echo "Example 6: Queue management operations"
echo "---------------------------------------"
cat << 'EOF'
#!/bin/bash
# List all tasks
$CLI queue:list

# List tasks in JSON format (for scripting)
$CLI queue:list --json | jq '.tasks | length'  # Count tasks

# Dismiss a specific task
$CLI queue:dismiss "task_id_here"

# Purge all completed tasks
$CLI queue:purge

# Check queue status periodically
while true; do
    PENDING=$($CLI queue:list --json | jq '[.tasks[] | select(.task_status != "complete_success")] | length')
    echo "Pending tasks: $PENDING"
    [[ "$PENDING" -eq 0 ]] && break
    sleep 5
done
EOF
echo

# Example 7: Direct command invocation
echo "Example 7: Direct Tauri command invocation"
echo "-------------------------------------------"
cat << 'EOF'
#!/bin/bash
# Invoke any Tauri command directly

# Get app info (if such a command exists)
$CLI invoke get_app_info_command

# Invoke with custom parameters
$CLI invoke enqueue_text_to_image_command \
    --prompt "sunset over mountains" \
    --provider artcraft \
    --model flux_pro_11

# List with JSON parsing
$CLI invoke get_task_queue_command --json | jq '.tasks[] | {id, status: .task_status, type: .task_type}'
EOF
echo

# Example 8: Batch generation workflow
echo "Example 8: Batch generation workflow"
echo "-------------------------------------"
cat << 'EOF'
#!/bin/bash
# Generate multiple images in sequence

PROMPTS=(
    "futuristic city"
    "alien landscape"
    "cyberpunk street"
    "space station"
    "robot portrait"
)

for prompt in "${PROMPTS[@]}"; do
    echo "Generating: $prompt"
    $CLI generate:text-to-image "$prompt" --provider openai --quiet
    
    # Optional: Wait between submissions to avoid rate limits
    sleep 2
done

echo "All generations submitted!"

# Wait for all to complete
echo "Waiting for all tasks to complete..."
while true; do
    PENDING=$($CLI queue:list --json | jq '[.tasks[] | select(.task_status != "complete_success")] | length')
    echo "Still pending: $PENDING"
    [[ "$PENDING" -eq 0 ]] && break
    sleep 10
done

echo "All tasks complete! Downloading results..."

# Download all completed tasks
$CLI queue:list --json | jq -r '.tasks[] | select(.task_status == "complete_success") | .completed_item.primary_media_file.token' | \
while read token; do
    echo "Downloading $token"
    $CLI download "$token"
done

echo "Batch complete!"
EOF
echo

# Example 9: OpenClaw integration pattern
echo "Example 9: OpenClaw integration (JSON output)"
echo "----------------------------------------------"
cat << 'EOF'
#!/bin/bash
# Pattern for OpenClaw subagent integration

# All commands support --json for programmatic use
# OpenClaw can parse and act on the results

# Submit generation
RESULT=$($CLI generate:text-to-image "test image" --provider openai --json 2>/dev/null)

# Parse result
STATUS=$(echo "$RESULT" | jq -r '.status // "unknown"')
if [[ "$STATUS" == "success" ]]; then
    echo "Generation submitted successfully"
    
    # Get task queue
    QUEUE=$($CLI queue:list --json)
    TASK_COUNT=$(echo "$QUEUE" | jq '.tasks | length')
    
    # Report to OpenClaw
    echo "Task count: $TASK_COUNT"
fi
EOF
echo

echo "========================================"
echo "To run any example, uncomment the commands"
echo "or copy the code blocks into separate scripts."
echo "========================================"
