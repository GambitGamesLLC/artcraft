# ArtCraft — Generic CLI `invoke` Dispatcher (Tauri v2)

**Date:** 2026-03-08  
**Status:** ✅ Complete  
**Agent:** Chip 🐱‍💻  

---

## Goal

Implement a **generic** external CLI entrypoint for ArtCraft so automation can do:

```bash
artcraft invoke <tauri_command_name> --payload '<json>' --json
```

…and receive a structured JSON response, without needing the GUI.

---

## Context

- Branch: `chip/artcraft-cli-buildfix` (pushed to `origin`)
- `tauri-plugin-cli` provides match parsing but does not implement a generic `invoke` dispatcher.
- Requirement: early-exit behavior for CLI invocations (no main window).

---

## Proposed CLI Contract (Generic)

### Command

```bash
artcraft invoke <command> [--payload <json-or-@file>] [--json]
```

### Exit codes
- `0`: success
- `2`: invalid args / parsing
- `3`: unknown command
- `4`: invoke failed (runtime error)

---

## Tasks

### Task 1: Add `plugins.cli` config for `invoke`

**SubAgent:** `coder`  
**Status:** ✅ Complete

**Results:**
- Added `plugins.cli` configuration to support `invoke` with positional `<command>` and flags.

**Files modified:**
- `crates/desktop/artcraft/tauri.conf.json`

---

### Task 2: Implement Rust-side early dispatcher

**SubAgent:** `coder`  
**Status:** ✅ Complete

**Results:**
- Implemented an early CLI dispatch path that avoids creating the main window for `invoke`.
- Initial allowlist includes:
  - `platform_info_command`
  - `get_app_info_command`
  - `get_task_queue_command`
- JSON response printed to stdout.
- Exit codes follow the contract above.

**Files modified/added:**
- `crates/desktop/artcraft/src/lib.rs`
- `crates/desktop/artcraft/src/core/mod.rs`
- `crates/desktop/artcraft/src/core/cli/mod.rs` (new)
- `crates/desktop/artcraft/src/core/cli/invoke_dispatcher.rs` (new)

---

### Task 3: Update wrapper (`artcraft-cli.sh`) + docs

**SubAgent:** `coder`  
**Status:** ✅ Complete

**Results:**
- Updated wrapper and docs to reflect the new `artcraft invoke <command> ...` contract.

**Files modified:**
- `artcraft-cli.sh`
- `artcraft-cli/README.md`
- `artcraft-cli/QUICK_REFERENCE.md`
- `README.md`

---

### Task 4: Build deb-only + smoke test

**SubAgent:** `primary`  
**Status:** ✅ Complete

**Build:**
- Command: `./script/artcraft/unix_build.sh` (Linux default deb-only bundling)
- Build log:
  - `build-logs/2026-03-09_artcraft_deb_build_chip-artcraft-cli-buildfix.log`

**.deb produced:**
- `/home/derrick/.openclaw/workspace/projects/gambit-artcraft/target/release/bundle/deb/ArtCraft_0.12.0_amd64.deb`

**Smoke tests (log):**
- `build-logs/2026-03-09_artcraft_invoke_smoketest_chip-artcraft-cli-buildfix.log`

**Smoke tests (summary):**
- `./target/release/artcraft invoke platform_info_command --json` → exit `0`
  - `{"status":"success","payload":{"os_platform":"linux","webview_runtime":"webkit_gtk"}}`
- `./target/release/artcraft invoke get_app_info_command --json` → exit `0`
  - returns app + build metadata JSON
- `./target/release/artcraft invoke get_task_queue_command --json` → exit `0`
  - `{"status":"success","payload":{"tasks":[]}}`

**Negative tests:**
- Missing required `<command>`:
  - `./target/release/artcraft invoke --json` → exit `2`
- Unknown/disallowed command:
  - `./target/release/artcraft invoke totally_not_a_real_command --json` → exit `3`

---

### Task 5: Commit + push

**SubAgent:** `primary`  
**Status:** ✅ Complete

**Commit:**
- `8d85b6cae` — `Add generic invoke CLI dispatcher (early exit)`

**Push:**
- Pushed to `origin/chip/artcraft-cli-buildfix`

**Notes:**
- Build artifacts/logs were kept in `build-logs/` and intentionally not committed.

---

## Final Results

**Status:** ✅ Complete

**What We Built:**
- A generic `artcraft invoke <command> --json` CLI entrypoint with early-exit behavior and a small initial allowlist of safe commands.
- Debian-only bundle build confirmed on Linux.

**Commits:**
- `8d85b6cae` — Add generic invoke CLI dispatcher (early exit)

*Updated: 2026-03-09*
