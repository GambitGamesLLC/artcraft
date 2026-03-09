# ArtCraft CLI - Quick Reference

## Generic invoke

```bash
# Print platform + webview runtime info
./target/release/artcraft invoke platform_info_command --json

# Print app/version/build info
./target/release/artcraft invoke get_app_info_command --json

# List the current task queue
./target/release/artcraft invoke get_task_queue_command --json
```

## Wrapper shortcuts

```bash
./artcraft-cli.sh platform:info --json
./artcraft-cli.sh app:info --json
./artcraft-cli.sh queue:list --json
```
