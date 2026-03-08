# ArtCraft CLI - Quick Reference

## One-Liners

```bash
# Generate image
./artcraft-cli.sh generate:text-to-image "prompt" --provider openai

# List queue
./artcraft-cli.sh queue:list --json | jq '.tasks[] | {id, status: .task_status}'

# Wait for task
./artcraft-cli.sh queue:wait TASK_ID --timeout 300

# Download result
./artcraft-cli.sh download MEDIA_TOKEN

# Purge completed
./artcraft-cli.sh queue:purge
```

## Complete Workflow

```bash
# 1. Generate
./artcraft-cli.sh generate:text-to-image "cyberpunk city" --provider openai

# 2. Get task ID
TASK=$(./artcraft-cli.sh queue:list --json | jq -r '.tasks[0].id')

# 3. Wait
./artcraft-cli.sh queue:wait "$TASK"

# 4. Download
TOKEN=$(./artcraft-cli.sh queue:list --json | jq -r ".tasks[] | select(.id==\"$TASK\") | .completed_item.primary_media_file.token")
./artcraft-cli.sh download "$TOKEN"
```

## Providers

- `artcraft` (default)
- `grok`
- `midjourney`
- `sora`

## Common Models

**Text-to-Image:** `flux_pro_11`, `gpt_image_1`, `nano_banana_pro`, `grok_image`  
**Video:** `sora_2`, `kling_2p6_pro`, `veo_3`  
**Edit:** `flux_pro_kontext_max`, `nano_banana_pro`

## Exit Codes

- `0` - Success
- `1` - Error (check stderr)

## JSON Output

All commands support `--json` for scripting:

```bash
./artcraft-cli.sh queue:list --json | jq '.tasks | length'
```
