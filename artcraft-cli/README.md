# ArtCraft CLI Wrapper

**Location:** `./artcraft-cli.sh`

This repo includes a small bash wrapper around ArtCraft’s **generic** CLI entrypoint:

```bash
artcraft invoke <command> [--payload <json|@file>] [--json] [--unsafe] [--list-allowed]
```

## Quick start

```bash
# help
./artcraft-cli.sh --help

# platform info
./artcraft-cli.sh platform:info --json

# list allowed commands
./target/release/artcraft invoke --list-allowed --json

# app info (UNSAFE; requires gate + --unsafe)
ARTCRAFT_ENABLE_UNSAFE_INVOKE=1 ./artcraft-cli.sh app:info --unsafe --json

# task queue (UNSAFE; requires gate + --unsafe)
ARTCRAFT_ENABLE_UNSAFE_INVOKE=1 ./artcraft-cli.sh queue:list --unsafe --json

# generic invoke (SAFE)
./artcraft-cli.sh invoke platform_info_command --json

# generic invoke (UNSAFE)
ARTCRAFT_ENABLE_UNSAFE_INVOKE=1 ./artcraft-cli.sh invoke --unsafe get_task_queue_command --json
```

## Notes

- The wrapper expects the binary at: `./target/release/artcraft`
- `artcraft invoke` is tiered:
  - **SAFE** (no `--unsafe` needed):
    - `platform_info_command`
    - `flip_image`
  - **UNSAFE** (everything else): requires `--unsafe` *and* one gate:
    - env var: `ARTCRAFT_ENABLE_UNSAFE_INVOKE=1`
    - or config file: `~/.config/artcraft/cli.json` with `{"enableUnsafeInvoke": true}`
- Discover the current tiered allowlist:
  - `./target/release/artcraft invoke --list-allowed --json`
- Exit codes:
  - `0` success
  - `2` invalid args / unsafe gate disabled
  - `3` disallowed/unknown (or missing `--unsafe` for an unsafe command)
  - `4` runtime invoke error
- When `--json` is passed, CLI-generated errors include `error_details.code` (e.g. `unsafe_gate_disabled`, `disallowed_command`, `invalid_args`).
