# ArtCraft — Canonical Plan (Linux Build/Install + External CLI/IPC Control)

**Project:** GambitGamesLLC/artcraft (fork)

**Date:** 2026-03-08  
**Status:** In Progress (build/install OK; strict SAFE/UNSAFE invoke contract + verifier shipped; upstream PRs pending)  
**Agent:** Chip 🐱‍💻  
**Overseer:** Derrick

---

## Start Here

This is the **single canonical ArtCraft plan**. All other ArtCraft plan docs have been archived for history:
- `plans/artcraft/_archive/2026-03-09/`

### Canonical repo-level specs (source of truth)

These live in the ArtCraft fork repo and should be referenced rather than duplicated:
- `projects/gambit-artcraft/docs/COMMAND_MATRIX.md`
- `projects/gambit-artcraft/docs/COMMAND_VERIFICATION_MATRIX.md`
- `projects/gambit-artcraft/scripts/tools/verify_artcraft_cli_commands.py`

### OpenClaw integration references
- `projects/openclaw-artcraft/packages/client/README.md`
- `projects/openclaw-artcraft/skills/openclaw-artcraft/SKILL.md`

---

## Goal

1) Get the ArtCraft fork building and runnable on Linux (Zorin) so we have a real desktop binary to test against.  
2) Finish external automation support (CLI/IPC via `tauri-plugin-cli`) and perform a **real** smoke test (not simulation).  
3) Document the workflow and upstream the changes.

---

## Current State (What’s true right now)

### Repo / remotes
- Work repo: `~/.openclaw/workspace/projects/gambit-artcraft`
- `origin` remote is SSH: `git@github.com:GambitGamesLLC/artcraft.git`

### Buildfix branch status
- Active build-unblock branch: `chip/fix-sqlite_tasks-build`
- Branch pushed to origin with these commits:
  - `128dc2a5a` — fix(build): include cargo bin path in unix build script
  - `0eb252222` — fix(build): run nx sync before tauri build
  - `38691aa05` — chore(build): nx sync outputs + reduce cargo build parallelism
  - `97de640b0` — fix(build): run tauri beforeBuildCommand/beforeDevCommand Nx steps from `frontend/` workspace

### Build blockers / outcomes
- **Nx root cause** (fixed): running Nx from repo root fails with:
  - `NX The current directory isn't part of an Nx workspace.`
  - Actual Nx workspace lives at `frontend/`
- After the Nx CWD fixes, Linux build progresses further.
- **Current blocker:** bundling appears to stall/hang at RPM packaging step:
  - Stalls after: `Bundling ArtCraft-0.12.0-1.x86_64.rpm ...`
  - `.deb` bundle completes
  - Build was manually killed after prolonged no-output hang

### Log artifact
- Build output captured/appended to:
  - `build-logs/unix_build-fix-sqlite_tasks-build.txt`
  - Note: this was left untracked (but exists locally).

---

## External CLI/IPC Control (Scope + Status)

### Problem
ArtCraft exposes ~45 Tauri commands via JS `invoke()` (frontend-only). We want **external automation** (OpenClaw + scripts) without UI-driving.

### Approach
Use `tauri-plugin-cli` (Tauri v2) and a wrapper script/client library so external processes can invoke Tauri commands.

### Status by phase
- ✅ **Phase 1:** Add `tauri-plugin-cli` dependency + register plugin + config in `tauri.conf.json`
- ✅ **Phase 2:** Create `artcraft-cli.sh` wrapper + examples (supports simulation mode)
- ✅ **Phase 3:** Create OpenClaw Python client (`projects/openclaw-artcraft-client/`) + tests (simulation mode)
- ⏳ **Phase 4 (pending):** Documentation, real binary smoke test, and upstream PR

### Phase 4 is blocked by
Historically: `sqlite_tasks` build errors prevented producing a runnable binary.

Now: the build progresses much further; the immediate new issue is the **RPM bundling hang**. However, for CLI smoke tests we likely only need a runnable binary (not an RPM), so we may be able to continue by **skipping RPM bundling**.

---

## CLI Safety + Command Matrix Workflow

### CLI safety policy (default-safe)
- **Safe** commands are **read-only**: they may inspect state, enumerate queues, read config, and return structured output.
- **Unsafe** commands are anything that **mutates state** or performs **generation** (e.g., enqueue/execute tasks, write files, modify settings, start jobs).
- Unsafe operations must require **both**:
  1) an explicit `--unsafe` CLI flag, **and**
  2) a runtime **unsafe gate** (e.g., allowlist/confirmation gate) that blocks execution unless consciously enabled.

### Command-matrix-driven workflow (source of truth)
The command inventory + safety classification is tracked in:
- `projects/gambit-artcraft/COMMAND_MATRIX.md` (branch: `chip/cli-config-cross-platform`)

Workflow for adding external CLI/IPC coverage:
1) **Classify** the target command in the matrix (safe/unsafe, inputs/outputs, side effects).
2) **Wire into the dispatcher** (CLI matches → dispatcher → command implementation) and enforce the policy:
   - safe is allowed by default
   - unsafe requires `--unsafe` + gate + explicit allowlist entry
3) **Bootstrap minimal runtime state** required for deterministic behavior (config, app data dir, headless/no-window path if available).
4) Add **tests** (unit tests for classification/gating; integration tests for dispatcher routing and safe commands).
5) **Expand allowlists incrementally**: start with a small safe surface (info/introspection), then broaden in controlled steps with matrix + tests updated alongside.

---

## Milestones / Tasks

### Milestone A — Produce a runnable Linux build artifact (skip RPM if needed)

**Objective:** obtain a local runnable ArtCraft binary (or `.deb` install) to test CLI invocation end-to-end.

**Tasks:**
1. Reproduce current build status on `chip/fix-sqlite_tasks-build`.
2. If RPM bundling hangs again, pivot to one of:
   - Build without bundling (preferred for speed) and run the binary directly.
   - Build only `.deb` bundle (since it completes) and install/run.
3. Capture actionable evidence on the RPM hang (process tree, last output, etc.).

**Concrete next steps (RPM hang triage):**
- When it stalls, in another terminal:
  - `ps -ef | egrep '(rpmbuild|rpm|tar|gzip|xz|zstd|tauri)' | grep -v egrep`
  - `top` (look for a compress process pegged or blocked)
  - `strace -p <pid>` (if allowed) to see where it’s stuck
- Check for output files being produced incrementally:
  - `find crates/desktop/artcraft/target -maxdepth 6 -type f | tail -n 50`
- If the build script supports it, run a command that **does not attempt RPM bundling** (exact flag depends on Tauri version; candidates include `--bundles deb` or `--bundles none`).

**Definition of Done:** we can launch ArtCraft (even from `target/release/…`) and it runs.

---

### Milestone B — Layer CLI changes onto the buildfix branch

**Objective:** create a branch that contains both (1) build fixes and (2) CLI plugin integration so we can smoke-test a real CLI call.

**Plan:**
- Create: `chip/artcraft-cli-buildfix`
- Base: `origin/chip/fix-sqlite_tasks-build`
- Layer in CLI work from: `chip/wip-artcraft-cli` (via rebase/cherry-pick/merge)

**Definition of Done:** branch builds and produces a runnable artifact (even if RPM bundle is skipped), and the CLI work is present.

---

### Milestone C — Real CLI smoke test (end-to-end)

**Objective:** use the built ArtCraft binary and run at least one real external CLI invocation.

**Smoke-test checklist:**
- Invoke a harmless command:
  - `get_app_info_command` or `platform_info_command`
- Invoke queue command:
  - `get_task_queue_command`
- (Optional) enqueue a generation task, verify it appears in UI queue:
  - `enqueue_text_to_image_command`

**Definition of Done:** CLI invocation returns expected JSON/structured output and ArtCraft responds.

---

### Milestone D — Documentation + upstream

**Objective:** finish Phase 4:
- Update repo README with External CLI Control quick-start
- Create `docs/external-control.md`
- Ensure security considerations are documented
- Commit + push to GambitGamesLLC/artcraft
- Prepare upstream PR

---

## Resume Here (Exact Commands)

### 0) Go to repo and confirm branch
```bash
cd /home/derrick/.openclaw/workspace/projects/gambit-artcraft

git status

git fetch origin

git checkout chip/fix-sqlite_tasks-build

git log --oneline -n 20
```

### 1) Re-run the Linux build script
```bash
./script/artcraft/unix_build.sh
```

### 2) If it hangs at “Bundling … .rpm”, capture evidence
```bash
ps -ef | egrep '(rpmbuild|rpm|tar|gzip|xz|zstd|tauri)' | grep -v egrep

# optional (if available)
# strace -p <pid>
```

### 3) Proceed even if RPM bundling is broken (get runnable artifact)
Try a build mode that avoids RPM bundling (exact flags depend on the repo’s Tauri build wiring):
```bash
# Try to produce a runnable build without rpm bundling
# (adjust to whichever option is supported in this repo)

# Example candidates:
# cargo tauri build --bundles deb
# cargo tauri build --bundles none
# cargo tauri build --no-bundle
```

### 4) Create the layered branch for CLI smoke test
```bash
# Base from the buildfix branch
git checkout -b chip/artcraft-cli-buildfix

# Bring in CLI work (choose one approach)
# Option A: rebase
# git rebase chip/wip-artcraft-cli

# Option B: cherry-pick (preferred when isolating)
# git cherry-pick <sha1> <sha2> ...

# Option C: merge
# git merge chip/wip-artcraft-cli
```

---

## Notes / Decision Record

- We do **not** need RPM packaging to validate external CLI/IPC. We only need a runnable binary (or deb install).
- Nx steps must be executed from `frontend/` (this is now encoded in the build config via the commits listed above).

---

## Run Log — 2026-03-08 (18:50–19:25 EDT)

### Milestone B (layer CLI onto buildfix) — ✅
- Created layered branch: `chip/artcraft-cli-buildfix`
  - Base: `chip/fix-sqlite_tasks-build`
  - Cherry-picked CLI work from local `chip/wip-artcraft-cli` (single commit): `9696b8617`
- Pushed branch to origin:
  - `origin/chip/artcraft-cli-buildfix`

### Milestone A (produce runnable Linux artifact; skip RPM) — ✅
- Successfully built **deb-only** bundle using Tauri CLI v2.10.0:
  - Command (equivalent of `script/artcraft/unix_build.sh` but with bundles restricted):
    - `cargo tauri build --config crates/desktop/artcraft/tauri.conf.json --bundles deb`
  - Build log captured at:
    - `build-logs/unix_build-artcraft-cli-buildfix-deb-20260308-185201.txt`
  - Output artifact:
    - `/home/derrick/.openclaw/workspace/projects/gambit-artcraft/target/release/bundle/deb/ArtCraft_0.12.0_amd64.deb`
- Updated build script on this branch so Linux defaults to `.deb` bundling (avoids RPM hang):
  - `script/artcraft/unix_build.sh` now uses `--bundles deb` by default on Linux
  - Override examples:
    - `ARTCRAFT_TAURI_BUNDLES="deb,rpm" ./script/artcraft/unix_build.sh`
    - `ARTCRAFT_TAURI_NO_BUNDLE=1 ./script/artcraft/unix_build.sh`

### Milestone C (real CLI smoke test) — ⚠️ Partial / Blocked
- Verified **tauri-plugin-cli** is present in the built binary (symbol/strings evidence):
  - `strings ./target/release/artcraft | grep -E "tauri_plugin_cli|cli_matches" | head`
  - Shows e.g. `plugin:cli|cli_matches` and `tauri_plugin_cli::init`.
- Attempted runtime invocation via the wrapper’s assumed pattern:
  - `./target/release/artcraft cli invoke platform_info_command`
  - Observed behavior: app launches normally; no obvious “invoke” dispatch occurs.
- **Finding:** `tauri-plugin-cli` (Tauri v2) provides a way to **parse/read CLI matches** (via `cli_matches`), but it does **not** automatically provide a `cli invoke <tauri-command>` command.

**Next steps (to unblock real smoke test):**
1) Decide the real CLI contract we want (`artcraft <subcommand> ...`) and define it under `plugins.cli` in `tauri.conf.json`.
2) Add a small dispatcher (Rust and/or JS) that:
   - reads matches (`@tauri-apps/plugin-cli` / `cli_matches`),
   - runs a harmless backend function (e.g., call `platform_info_command`),
   - prints JSON to stdout and exits **without** spinning up the full main window.

---

## Run Log — 2026-03-09

### Milestone C (real CLI smoke test + safety contract) — ✅ Complete

Shipped in `gambit-artcraft` branch `chip/artcraft-cli-buildfix`:
- `artcraft invoke <tauri_command> [--payload ...] [--unsafe] --json`
- Strict SAFE tier (no `--unsafe`, no gate):
  - `platform_info_command`
  - `flip_image`
- UNSAFE tier: everything else, requires:
  - `--unsafe` and
  - unsafe gate enabled via `ARTCRAFT_ENABLE_UNSAFE_INVOKE=1` or `~/.config/artcraft/cli.json` (`{"enableUnsafeInvoke": true}`)
- `artcraft invoke --list-allowed --json` → `{ safe: [...], unsafe: [...], unsafeGateEnabled: bool }`
- Payload hardening: `--payload @file` is not read until after tier/gate enforcement.
- JSON error codes in `--json` mode: `error_details.code` (e.g. `unsafe_required`, `unsafe_gate_disabled`, `invalid_args`).

Verification/tooling:
- Added deterministic verifier + matrix:
  - `docs/COMMAND_VERIFICATION_MATRIX.md`
  - `scripts/tools/verify_artcraft_cli_commands.py`
    - default: non-destructive (gate forced off)
    - opt-in subsets (manual-only; never default/CI):
      - `readonly` (requires `--unsafe-gate-on`)
      - `readonly-network-cost` (may hit network/incur cost): **SKIPPED** unless `--allow-network` or `--allow-network-cost`
      - `readonly-account` (credentialed): **SKIPPED** unless `--allow-credentialed`

Bugfix:
- Fixed `flip_image` stdout contamination so `--json` is JSON-only.

Local install:
- Rebuilt + installed `.deb` so `/usr/bin/artcraft` matches the branch and passes:
  - `./scripts/tools/verify_artcraft_cli_commands.py --binary /usr/bin/artcraft`
  - `ARTCRAFT_ENABLE_UNSAFE_INVOKE=1 ... --run-unsafe-subset readonly --unsafe-gate-on`

### OpenClaw client integration — ✅ Complete

In `openclaw-artcraft` (repo `~/.openclaw/workspace/projects/openclaw-artcraft`):
- Python client moved to tier-based API + allowlist introspection:
  - `tier="safe"|"unsafe"` (no legacy `unsafe=`)
  - `list_allowed()`
  - deterministic exception mapping via CLI JSON `error_details.code`
- Skill docs enabled in workspace via symlink:
  - `~/.openclaw/workspace/skills/openclaw-artcraft` → `projects/openclaw-artcraft/skills/openclaw-artcraft`

### Remaining work
- Prepare upstream Issue(s) + PR(s) for ArtCraft changes (recommend split):
  - PR A: core CLI contract + safety (`invoke_dispatcher.rs`, `tauri.conf.json`, wiring)
  - PR B: verifier + docs (`verify_artcraft_cli_commands.py`, `COMMAND_*MATRIX*.md`, wrapper docs)
- Sanity check against upstream before opening PRs:
  - ensure we’re not shipping irrelevant fixes/docs/plans/config/logs
  - confirm `.gitignore` covers local noise (e.g. `build-logs/`, `.gastown-ignore`)
  - scan PR diff for any workspace-only paths or local build artifacts

Recent “docs reflect reality” commits worth calling out:
- `gambit-artcraft` (`chip/artcraft-cli-buildfix`):
  - `f4b27eb95` — verifier: latch network-cost subset; clarify skips
  - `ca7f35d28` — ignore local build logs + `.gastown-ignore`
  - `3fd658e1a` — docs: sync README CLI automation examples
  - `7d30aeba2` — docs: sync CLI wrapper + verifier flags
- `openclaw-artcraft` (`main`): `0205e6b` — docs: align client + skill with real CLI contract

---

## References (Former plans consolidated into this canonical doc)

This file replaces and consolidates earlier plan docs. As of 2026-03-09, the historical plans were **archived** (not deleted) under:
- `plans/artcraft/_archive/2026-03-09/`

Note: a prior note claimed `artcraft-external-ipc-control-2026-03-07.md` was hard-deleted; the file exists as:
- `plans/gambit-artcraft/artcraft-external-ipc-control-2026-03-07.md`


---

*Created: 2026-03-08*  
*Last updated: 2026-03-09*
