# ArtCraft — Install Upgraded .deb + Verify `artcraft invoke` (Installed Binary)

**Date:** 2026-03-08  
**Status:** ✅ Complete  
**Agent:** Chip 🐱‍💻

---

## Goal

Upgrade the system-installed ArtCraft (`art-craft`) to the newly built `.deb` from branch `chip/artcraft-cli-buildfix`, then verify that the **installed** `artcraft` binary supports:

```bash
artcraft invoke <command> --json
```

---

## Artifact

- `.deb` path:
  - `/home/derrick/.openclaw/workspace/projects/gambit-artcraft/target/release/bundle/deb/ArtCraft_0.12.0_amd64.deb`

---

## Tasks

### Task 1: Install/upgrade package from local .deb (sudo)

**Owner:** Derrick (sudo)

```bash
sudo apt-get install -y /home/derrick/.openclaw/workspace/projects/gambit-artcraft/target/release/bundle/deb/ArtCraft_0.12.0_amd64.deb
```

Notes:
- Package name is `art-craft`.
- This should upgrade in-place if same version but different build.

**Status:** ✅ Complete

---

### Task 2: Verify installed binary resolves to /usr/bin/artcraft

```bash
which artcraft
ls -la "$(which artcraft)"
artcraft --version || true
```

**Status:** ✅ Complete

---

### Task 3: Verify invoke works on installed binary (no GUI)

```bash
artcraft invoke platform_info_command --json
artcraft invoke get_app_info_command --json
artcraft invoke get_task_queue_command --json

# negative: unknown command -> exit 3
set +e
artcraft invoke totally_not_a_real_command --json
echo "EXIT:$?"
```

**Status:** ✅ Complete

---

## Final Results

**Status:** ✅ Complete

**Installed path:** `/usr/bin/artcraft`

**Installed build commit:**
- `artcraft invoke get_app_info_command --json` reports `git_commit_short_id: "4463ab1"`

**Quick verification (installed binary):**
- `artcraft invoke platform_info_command --json` → exit `0`
- `artcraft invoke get_task_queue_command --json` → exit `0` (empty queue)
- `artcraft invoke totally_not_a_real_command --json` → exit `3` (expected)

---

*Created: 2026-03-08*  
*Completed: 2026-03-09*