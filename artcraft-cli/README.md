# ArtCraft CLI Wrapper

**Location:** `./artcraft-cli.sh`

This repo includes a small bash wrapper around ArtCraft’s **generic** CLI entrypoint:

```bash
artcraft invoke <command> [--payload <json|@file>] [--json]
```

## Quick start

```bash
# help
./artcraft-cli.sh --help

# platform info
./artcraft-cli.sh platform:info --json

# app info
./artcraft-cli.sh app:info --json

# task queue
./artcraft-cli.sh queue:list --json

# generic invoke
./artcraft-cli.sh invoke platform_info_command --json
./artcraft-cli.sh invoke get_task_queue_command --json
```

## Notes

- The wrapper expects the binary at: `./target/release/artcraft`
- The current Rust-side CLI dispatcher **allowlists** a small subset of commands for automation:
  - `platform_info_command`
  - `get_app_info_command`
  - `get_task_queue_command`

(Expand the allowlist as needed once the CLI contract stabilizes.)
