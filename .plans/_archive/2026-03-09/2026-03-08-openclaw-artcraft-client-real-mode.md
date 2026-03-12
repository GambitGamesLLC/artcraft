# OpenClaw ↔ ArtCraft Client — Switch From Simulation to Real `artcraft invoke`

**Date:** 2026-03-08  
**Status:** Draft  
**Agent:** Chip 🐱‍💻

---

## Goal

Update the OpenClaw ArtCraft client (`projects/openclaw-artcraft-client/`) to use the **installed** ArtCraft CLI:

```bash
artcraft invoke <command> --json
```

…so OpenClaw workflows can drive ArtCraft for real (no simulation mode), while respecting:
- safe allowlist by default
- optional gated `--unsafe`

---

## Context

- Installed ArtCraft verified at `/usr/bin/artcraft`.
- Generic dispatcher commit: `8d85b6cae...`
- Unsafe gate commit: `4463ab1a3...` (installed binary reports `git_commit_short_id: 4463ab1`).

---

## Tasks

### Task 1: Audit current client assumptions

**SubAgent:** `primary`

**Prompt:**
- Read `projects/openclaw-artcraft-client/` and identify where it assumes `artcraft-cli.sh` / `cli invoke`.
- List client methods and map them to new contract (`artcraft invoke <tauri_command> --json`).

**Status:** ⏳ Pending

---

### Task 2: Implement real-mode execution path

**SubAgent:** `coder`

**Prompt:**
- Update client to call `artcraft invoke ... --json` via subprocess.
- Parse JSON response `{status,payload,error_message}` and raise typed exceptions.
- Add support for optional unsafe flag + gating (env var pass-through):
  - e.g., `unsafe=True` uses `--unsafe`
  - Document that caller must set `ARTCRAFT_ENABLE_UNSAFE_INVOKE=1` or config file.

**Status:** ⏳ Pending

---

### Task 3: Real smoke tests

**SubAgent:** `primary`

**Prompt:**
- Run the client against installed `artcraft`:
  - platform info
  - app info
  - queue list
- Add/adjust tests accordingly.

**Status:** ⏳ Pending

---

### Task 4: Commit + push

**SubAgent:** `primary`

**Prompt:**
- Commit changes in `projects/openclaw-artcraft-client` and push to origin/main.

**Status:** ⏳ Pending

---

## Final Results

**Status:** Draft

---

*Created: 2026-03-08*