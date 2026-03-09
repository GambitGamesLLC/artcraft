# ArtCraft CLI - Quick Reference

## Generic invoke (safe allowlist)

```bash
# Print platform + webview runtime info
./target/release/artcraft invoke platform_info_command --json

# Print app/version/build info
./target/release/artcraft invoke get_app_info_command --json

# List the current task queue
./target/release/artcraft invoke get_task_queue_command --json
```

## Unsafe invoke (gated)

```bash
# Fails with exit 2 unless one gate is enabled:
#   - ARTCRAFT_ENABLE_UNSAFE_INVOKE=1
#   - ~/.config/artcraft/cli.json with {"enableUnsafeInvoke": true}

ARTCRAFT_ENABLE_UNSAFE_INVOKE=1 ./target/release/artcraft invoke --unsafe get_provider_order_command --json
```

## Wrapper shortcuts

```bash
./artcraft-cli.sh platform:info --json
./artcraft-cli.sh app:info --json
./artcraft-cli.sh queue:list --json
```
