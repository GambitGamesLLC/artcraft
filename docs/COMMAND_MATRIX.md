# ArtCraft CLI Command Matrix (tiered `invoke`)

This document describes the contract and safety model for the **generic** CLI entrypoint:

```bash
artcraft invoke <tauri_command> [--payload <json>] [--unsafe] --json
```

Where `<tauri_command>` is an internal Tauri command name (e.g. `platform_info_command`).

> Note: the implementation also supports `--payload @/path/to/file.json` (see **Payload hardening** below).

---

## Exit codes

`artcraft invoke` uses a small set of stable exit codes:

| Exit code | Meaning |
|---:|---|
| `0` | Success (command executed, JSON response printed) |
| `2` | Invalid arguments **or** unsafe invoke gate disabled / gate error |
| `3` | Disallowed/unknown command **or** missing `--unsafe` for an unsafe command |
| `4` | Runtime invoke error (the command was allowed and attempted, but failed) |

When `--json` is passed, CLI-generated errors are emitted as JSON and include a machine-readable code (for example: `invalid_args`, `unsafe_gate_disabled`, `disallowed_command`).

---

## Invoke tiers

`artcraft invoke` is **tiered**. The tier is computed from a fixed allowlist, and enforced *before* any payload parsing / `@file` reads.

### SAFE tier (strict)

SAFE commands require **no** `--unsafe` flag and do not require any gate.

Allowed SAFE commands:

- `platform_info_command`
- `flip_image`

### UNSAFE tier (everything else)

All other internal commands are treated as **UNSAFE**.

To invoke an UNSAFE command, both of the following are required:

1. `--unsafe` on the CLI
2. An additional **gate** is enabled via one of:
   - Environment variable: `ARTCRAFT_ENABLE_UNSAFE_INVOKE=1`
   - Config file: `~/.config/artcraft/cli.json` containing:

     ```json
     {"enableUnsafeInvoke": true}
     ```

If `--unsafe` is missing for an unsafe command, the CLI exits with **3**.

If `--unsafe` is present but the gate is disabled (or fails to load), the CLI exits with **2**.

---

## Listing allowed commands

To programmatically discover the current tiered allowlist:

```bash
artcraft invoke --list-allowed --json
```

### Output shape

The output is a single JSON object:

```json
{
  "safe": ["platform_info_command", "flip_image"],
  "unsafe": ["..."],
  "unsafeGateEnabled": false
}
```

Notes:

- `safe`: array of SAFE command strings
- `unsafe`: array of UNSAFE command strings
- `unsafeGateEnabled`: boolean reflecting whether the UNSAFE gate is currently enabled (env/config)

---

## Payload hardening (`--payload` and `@file`)

`--payload` accepts either:

- an inline JSON string
- a file reference prefixed with `@` (example: `--payload @/tmp/payload.json`)

Security behavior:

- The CLI determines the command tier **first**.
- For UNSAFE commands, the `--unsafe` requirement and the unsafe gate are enforced **before** any `@file` payload is read from disk.

This prevents `@file` from being used to force arbitrary file reads unless the caller is already authorized to run UNSAFE commands.

---

## Related wrapper documentation (do not duplicate)

This repo also includes a bash wrapper at `./artcraft-cli.sh`, with its own documentation:

- `artcraft-cli/README.md`
- `artcraft-cli/QUICK_REFERENCE.md`

Those docs include practical examples and shortcuts for common commands; this document focuses on the **invoke contract** and **tier/exit-code matrix**.
