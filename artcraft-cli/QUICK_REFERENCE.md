# ArtCraft CLI - Quick Reference

## Discover allowed commands

```bash
./target/release/artcraft invoke --list-allowed --json
```

## Generic invoke (SAFE)

```bash
# Print platform + webview runtime info
./target/release/artcraft invoke platform_info_command --json

# Flip an image (payload is base64 of image bytes)
# (Example uses a placeholder; provide a real base64 string)
./target/release/artcraft invoke flip_image --payload '{"image":"<base64>"}' --json
```

## Generic invoke (UNSAFE; gated)

```bash
# Fails with exit 2 unless one gate is enabled:
#   - ARTCRAFT_ENABLE_UNSAFE_INVOKE=1
#   - ~/.config/artcraft/cli.json with {"enableUnsafeInvoke": true}

ARTCRAFT_ENABLE_UNSAFE_INVOKE=1 ./target/release/artcraft invoke --unsafe get_app_info_command --json
ARTCRAFT_ENABLE_UNSAFE_INVOKE=1 ./target/release/artcraft invoke --unsafe get_task_queue_command --json
ARTCRAFT_ENABLE_UNSAFE_INVOKE=1 ./target/release/artcraft invoke --unsafe get_provider_order_command --json
```

## Wrapper shortcuts

```bash
./artcraft-cli.sh platform:info --json

# UNSAFE commands require --unsafe + gate
ARTCRAFT_ENABLE_UNSAFE_INVOKE=1 ./artcraft-cli.sh app:info --unsafe --json
ARTCRAFT_ENABLE_UNSAFE_INVOKE=1 ./artcraft-cli.sh queue:list --unsafe --json

# Generic via wrapper (note: command must come immediately after "invoke")
ARTCRAFT_ENABLE_UNSAFE_INVOKE=1 ./artcraft-cli.sh invoke get_task_queue_command --unsafe --json
```
