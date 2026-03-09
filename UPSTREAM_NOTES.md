# Upstream notes: `artcraft invoke` safety model + cross-platform CLI config

This branch introduces / documents a **generic CLI invoke dispatcher** with an explicit safety model.

> Note: This file is intended as an upstream Issue/PR description outline. It does **not** open an Issue/PR.

## What the CLI feature is (invoke dispatcher)

ArtCraft exposes a generic headless entrypoint:

```bash
artcraft invoke <command> [--payload <json|@file>] [--json] [--unsafe]
```

Internally this routes through an **invoke dispatcher** that:

- parses CLI args (`<command>`, `--payload`, `--json`, `--unsafe`)
- validates payload JSON (inline or `@file`)
- gates execution based on a **two-tier allowlist policy** (safe vs unsafe)
- returns structured JSON responses + appropriate exit codes

Implementation entrypoint:

- `crates/desktop/artcraft/src/core/cli/invoke_dispatcher.rs` (`dispatch_invoke`)

## Safe vs unsafe policy

The dispatcher is intentionally split into **two dispatch tiers**:

### Safe tier (default)

**Safe = read-only only (by design).**

Safe commands must be limited to local introspection (platform/app info, task queue status, etc.). They must **not**:

- call external providers / make network requests that can spend tokens or money
- generate images/video/audio (generation is always unsafe)
- write files or mutate application state

This is intentionally conservative so automation (and agent usage) can query state without surprising side effects.

### Unsafe tier (explicit opt-in)

**Unsafe includes anything that can cause side effects**, including:

- *generation* (token/cost spending and provider calls)
- state mutation, writes, or privileged access

Unsafe is an intentional escalation; enabling it acknowledges the risks.

## Where allowlists live

The allowlists and the core gating logic live in:

- `crates/desktop/artcraft/src/core/cli/invoke_dispatcher.rs`

Look for these symbols:

- `SAFE_INVOKE_ALLOWLIST`
- `UNSAFE_INVOKE_ALLOWLIST`

When adding a new CLI-invokable command, decide which tier it belongs to:

- If it can *ever* spend tokens/money, hit a provider, mutate state, write files, or access sensitive data → **UNSAFE**
- Otherwise (pure introspection) → **SAFE**

## How unsafe gating works (env + config)

Unsafe invocation requires **both**:

1. `--unsafe` flag
2. an enabled gate, via either:
   - env var: `ARTCRAFT_ENABLE_UNSAFE_INVOKE=1`
   - OR config file with `{"enableUnsafeInvoke": true}`

If `--unsafe` is passed without the gate, the CLI exits with code `2` and prints a JSON error.

Code:

- `unsafe_gate_enabled()` in `crates/desktop/artcraft/src/core/cli/invoke_dispatcher.rs`

## Cross-platform config path rationale (and where it lives)

The unsafe gate can be enabled via a config file located at the platform’s standard **user config directory**.

Instead of hard-coding `~/.config` (Linux-only), the code uses:

- `dirs::config_dir()`

This resolves to OS-appropriate locations:

- Linux: `$XDG_CONFIG_HOME` or `~/.config`
- macOS: `~/Library/Application Support`
- Windows: `%APPDATA%` (RoamingAppData)

The ArtCraft CLI config file is then:

```text
{config_dir}/artcraft/cli.json
```

Concrete examples:

- Linux (typical): `~/.config/artcraft/cli.json`
- macOS (typical): `~/Library/Application Support/artcraft/cli.json`
- Windows (typical): `%APPDATA%\\artcraft\\cli.json`

Code location:

- `cli_config_path = config_dir.join("artcraft").join("cli.json")` in `crates/desktop/artcraft/src/core/cli/invoke_dispatcher.rs`

## Docs / references

The policy and editing guidance are documented here:

- `docs/cli-safety.md`

Key sections:

- safe tier definition (read-only)
- unsafe tier definition (explicit opt-in + gating requirements)
- allowlist symbols and file location
