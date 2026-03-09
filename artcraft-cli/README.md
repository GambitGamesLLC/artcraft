# ArtCraft CLI Wrapper

**Location:** `./artcraft-cli.sh`

This repo includes a small bash wrapper around ArtCraft’s **generic** CLI entrypoint:

```bash
artcraft invoke <command> [--payload <json|@file>] [--json] [--unsafe]
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

### Safety policy (by design)

- **Safe (default) = read-only only.**
- **Generation (token-spend/provider calls) = unsafe.**
- Unsafe requires **both** `--unsafe` **and** an enabled gate.
- Enabling unsafe is an intentional escalation — you accept the risks (cost, side effects, data exposure).

### Allowlists / gating

- Default mode keeps a strict safe allowlist for automation:
  - `platform_info_command`
  - `get_app_info_command`
  - `get_task_queue_command`
- `--unsafe` enables the unsafe dispatch tier (currently includes `get_provider_order_command`) but requires one gate:
  - env var: `ARTCRAFT_ENABLE_UNSAFE_INVOKE=1`
  - or config file: `<app_config_dir>/artcraft/cli.json` with `{"enableUnsafeInvoke": true}`
- If `--unsafe` is used without a gate, CLI exits with code `2` and prints a JSON error.

Where to edit the allowlists:

- `../docs/cli-safety.md`
- `../crates/desktop/artcraft/src/core/cli/invoke_dispatcher.rs` (`SAFE_INVOKE_ALLOWLIST`, `UNSAFE_INVOKE_ALLOWLIST`)
