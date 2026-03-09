# CLI safety model ("invoke" allowlists)

ArtCraft exposes a generic headless entrypoint:

```bash
artcraft invoke <command> [--payload <json|@file>] [--json] [--unsafe]
```

This is intentionally split into **two dispatch tiers**.

## Safe tier (default)

**Safe = read-only only (by design).**

Safe commands must be limited to local introspection (e.g., platform/app info, task queue). They must **not**:

- call external providers / make network requests that can spend tokens or money
- generate images/video/audio (generation is always unsafe)
- write files or mutate application state

This is intentionally conservative so automation (and agent usage) can query state without surprising side effects.

## Unsafe tier (explicit opt-in + gated)

**Unsafe includes anything that can cause side effects**, including:

- *generation* (token/cost spending and provider calls)
- state mutation, writes, or privileged access

Unsafe is an intentional escalation. If you enable it, **you are acknowledging and accepting the risks** (cost, data exposure, side effects) similarly to enabling agentic tools.

### Requirements to run unsafe commands

Unsafe commands require **both**:

1) `--unsafe` flag
2) an enabled gate:

- env var: `ARTCRAFT_ENABLE_UNSAFE_INVOKE=1`
- OR config file: `<app_config_dir>/artcraft/cli.json` with:

```json
{ "enableUnsafeInvoke": true }
```

Where `<app_config_dir>` resolves per-platform (examples):

- Linux: `~/.config`
- macOS: `~/Library/Application Support`
- Windows: `%APPDATA%`

If `--unsafe` is passed without the gate, the CLI exits with code `2` and prints a JSON error.

## Where to edit the allowlists

The allowlists and gating logic live here:

- `crates/desktop/artcraft/src/core/cli/invoke_dispatcher.rs`

Look for these symbols:

- `SAFE_INVOKE_ALLOWLIST`
- `UNSAFE_INVOKE_ALLOWLIST`

When adding a new CLI-invokable command, decide which tier it belongs to:

- If it can *ever* spend tokens/money, hit a provider, mutate state, write files, or access sensitive data → **UNSAFE**
- Otherwise (pure introspection) → **SAFE**
