# ArtCraft — Rebuild + Install .deb after `--unsafe` gate commit

**Date:** 2026-03-08  
**Status:** ✅ Complete  
**Agent:** Chip 🐱‍💻

---

## Goal

Rebuild the Linux `.deb` from the latest `chip/artcraft-cli-buildfix` (including commit `4463ab1a3`), install it system-wide, and verify that the **installed** `/usr/bin/artcraft` supports:
- safe allowlist behavior
- `invoke --unsafe` gated by env/config

---

## Tasks

### Task 1: Rebuild deb-only from latest branch head

**SubAgent:** `primary`

**Prompt:**
- `cd /home/derrick/.openclaw/workspace/projects/gambit-artcraft`
- `git fetch origin`
- `git checkout chip/artcraft-cli-buildfix`
- `git pull --ff-only`
- Confirm `git rev-parse HEAD` (should be >= 4463ab1a3)
- Run `./script/artcraft/unix_build.sh` (deb-only)
- Verify embedded commit:
  - `./target/release/artcraft invoke get_app_info_command --json`
- Report `.deb` path + `git_commit_short_id`

**Status:** ✅ Complete

**Results:**
- Branch head: `4463ab1a3`
- `.deb` produced at:
  - `target/release/bundle/deb/ArtCraft_0.12.0_amd64.deb`
- Built binary reports embedded commit:
  - `./target/release/artcraft invoke get_app_info_command --json` → `git_commit_short_id: "4463ab1"`

---

### Task 2: Install upgraded .deb (sudo) + verify installed behavior

**Owner:** Derrick (sudo)

**Install (copy to /tmp to avoid _apt warning):**
```bash
cp /home/derrick/.openclaw/workspace/projects/gambit-artcraft/target/release/bundle/deb/ArtCraft_0.12.0_amd64.deb /tmp/ArtCraft_0.12.0_amd64.deb
sudo apt-get install -y /tmp/ArtCraft_0.12.0_amd64.deb
```

**Verify installed commit + safe/unsafe behavior:**
```bash
which artcraft
artcraft invoke get_app_info_command --json

# safe allowlist OK
artcraft invoke platform_info_command --json

# blocked without --unsafe (exit 3)
set +e
artcraft invoke get_provider_order_command --json; echo EXIT:$?

# unsafe but gate disabled (exit 2)
artcraft invoke --unsafe get_provider_order_command --json; echo EXIT:$?

# unsafe with env gate (exit 0)
ARTCRAFT_ENABLE_UNSAFE_INVOKE=1 artcraft invoke --unsafe get_provider_order_command --json
```

**Status:** ✅ Complete

**Results (installed `/usr/bin/artcraft`):**
- `which artcraft` → `/usr/bin/artcraft`
- `artcraft invoke get_app_info_command --json` → `git_commit_short_id: "4463ab1"`
- Safe allowlist:
  - `artcraft invoke get_provider_order_command --json` → exit `3`
- Unsafe gate:
  - `artcraft invoke --unsafe get_provider_order_command --json` → exit `2` (gate disabled)
  - `ARTCRAFT_ENABLE_UNSAFE_INVOKE=1 artcraft invoke --unsafe get_provider_order_command --json` → exit `0` (prints JSON)

---

## Final Results

**Status:** ✅ Complete

- Rebuilt `.deb` from `chip/artcraft-cli-buildfix` @ `4463ab1a3`.
- Installed `/usr/bin/artcraft` verified to support:
  - safe allowlist `invoke` calls
  - gated `--unsafe` mode (env gate verified; config gate also implemented per docs)

---

*Created: 2026-03-08*  
*Completed: 2026-03-09*