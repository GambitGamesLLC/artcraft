# ArtCraft — Allowlist + `--unsafe` Invoke Gate (Config / Env)

**Date:** 2026-03-08  
**Status:** Complete  
**Agent:** Chip 🐱‍💻

---

## Goal

Keep the current **allowlist** behavior for `artcraft invoke <command> --json`, and add an **explicit escape hatch**:

```bash
artcraft invoke --unsafe <command> ...
```

…but make `--unsafe` require an additional opt-in gate via **config and/or env var**, so it’s not usable accidentally or by default in agent runs.

---

## Proposed Behavior

### Default (safe)
- `artcraft invoke <command>` works only for allowlisted commands.
- Disallowed commands return exit `3` with JSON error.

### Unsafe mode
- `artcraft invoke --unsafe <command>` attempts to dispatch any command (or a broader list), **only if unsafe is enabled**.

### Unsafe enable gates (recommend both)
Unsafe is allowed iff:
- Config file contains `enableUnsafeInvoke: true`, OR
- Env var `ARTCRAFT_ENABLE_UNSAFE_INVOKE=1` is set

Optional hardening:
- If `--unsafe` is used without gate enabled: exit `2` with JSON error message.
- If running non-interactive (no TTY), optionally require both config + env.

---

## Config Location

Prefer something under the existing ArtCraft app-data root, e.g.
- `~/.config/artcraft/cli.json` (Linux)

Config schema (minimal):
```json
{
  "enableUnsafeInvoke": false
}
```

---

## Tasks

### Task 1: Update CLI contract + docs

**SubAgent:** `primary`

**Prompt:**
- Update README + quick reference to document:
  - safe mode behavior
  - `--unsafe` flag
  - how to enable unsafe via env/config
- Keep docs concise and explicit.

**Status:** ✅ Complete

**Results:**
- Updated `README.md`, `artcraft-cli/README.md`, and `artcraft-cli/QUICK_REFERENCE.md` with `--unsafe` gate docs (env var + `~/.config/artcraft/cli.json`).

---

### Task 2: Implement config/env gate in Rust dispatcher

**SubAgent:** `coder`

**Prompt:**
- On branch `chip/artcraft-cli-buildfix`:
  - Add `--unsafe` flag parsing to the CLI matches.
  - Implement unsafe gating:
    - env var `ARTCRAFT_ENABLE_UNSAFE_INVOKE=1`
    - and/or config file `~/.config/artcraft/cli.json` (or under ArtCraft app data root if that’s already implemented)
  - If `--unsafe` not provided: keep current allowlist behavior unchanged.
  - If `--unsafe` provided but gate not enabled: exit `2` with JSON error.
  - If gate enabled: broaden dispatch (start with “all commands” if feasible, or a larger allowlist tier).

**Status:** ✅ Complete

**Results:**
- Added `--unsafe` bool arg to the `invoke` CLI in `crates/desktop/artcraft/tauri.conf.json`.
- Implemented `ARTCRAFT_ENABLE_UNSAFE_INVOKE=1` and `~/.config/artcraft/cli.json` (`{"enableUnsafeInvoke": true}`) gates in `crates/desktop/artcraft/src/core/cli/invoke_dispatcher.rs`.
- Added gated dispatch for `get_provider_order_command` and bootstraps `ProviderPriorityStore` for CLI runs.

---

### Task 3: Build + smoke tests

**SubAgent:** `primary`

**Prompt:**
- Build deb-only.
- Tests:
  - safe invoke still works for existing allowlist
  - safe invoke still blocks disallowed
  - `--unsafe` without env/config returns error + exit 2
  - `--unsafe` with env var enabled allows at least one previously disallowed command (pick a non-destructive one)

**Status:** ✅ Complete

**Results:**
- Build: `cargo build -p artcraft --release` (log: `build-logs/2026-03-09_cargo-build-release-artcraft_chip-artcraft-cli-buildfix.log`).
- Smoke tests (log: `build-logs/2026-03-09_invoke-smoketest_chip-artcraft-cli-buildfix.log`):
  - `platform_info_command` → exit `0`
  - `get_provider_order_command` (no `--unsafe`) → exit `3`
  - `get_provider_order_command --unsafe` (no gate) → exit `2`
  - `ARTCRAFT_ENABLE_UNSAFE_INVOKE=1 ... --unsafe` → exit `0` and prints JSON

---

### Task 4: Commit + push

**SubAgent:** `primary`

**Prompt:**
- Commit changes with clear message(s).
- Push to `origin/chip/artcraft-cli-buildfix`.

**Status:** ✅ Complete

**Results:**
- Commit: `4463ab1a3` - `artcraft-cli: gate --unsafe invoke + provider order`
- Pushed to `origin/chip/artcraft-cli-buildfix`

---

## Final Results

**Status:** ✅ Complete

**What We Built:**
- Kept strict safe allowlist for `artcraft invoke`.
- Added gated `--unsafe` path requiring `ARTCRAFT_ENABLE_UNSAFE_INVOKE=1` or `~/.config/artcraft/cli.json` with `{ "enableUnsafeInvoke": true }`.
- Implemented one gated command: `get_provider_order_command`.

**Commits:**
- `4463ab1a3` - artcraft-cli: gate --unsafe invoke + provider order

---

*Completed: 2026-03-09*