# ArtCraft — Install Missing Deps + Install/Run via .deb

**Date:** 2026-03-08  
**Status:** Draft  
**Agent:** Chip 🐱‍💻  

---

## Goal

Install any missing system dependencies needed for ArtCraft’s Linux packaging (notably RPM tooling), then install and run ArtCraft via the generated `.deb` package so we can proceed to the CLI/IPC smoke test.

---

## Context

- Repo: `/home/derrick/.openclaw/workspace/projects/gambit-artcraft`
- Build artifacts already produced:
  - Binary: `target/release/artcraft`
  - Deb: `target/release/bundle/deb/ArtCraft_0.12.0_amd64.deb`
- Prior preflight showed all Tauri build deps installed.
- Missing: `rpm` / `rpmbuild` tooling (RPM bundling step hangs / can’t produce RPM).

---

## Tasks

### Task 1: Install RPM tooling (to unblock RPM bundling)

**SubAgent:** `primary`  
**Prompt:**
- Verify `rpm` and `rpmbuild` are missing.
- Install the minimal Ubuntu/Zorin package set for RPM tooling.
- Verify `rpm --version` and `rpmbuild --version` work.

**Commands (requires sudo):**
```bash
sudo apt-get update
sudo apt-get install -y rpm

rpm --version
rpmbuild --version
```

**Status:** ⏳ Pending

---

### Task 2: Install ArtCraft from the `.deb`

**SubAgent:** `primary`  
**Prompt:**
- Install the `.deb` package.
- If dependency resolution is needed, run the appropriate follow-up.
- Verify the app launches.

**Commands (requires sudo):**
```bash
cd /home/derrick/.openclaw/workspace/projects/gambit-artcraft

# Recommended (handles deps better)
sudo apt-get install -y ./target/release/bundle/deb/ArtCraft_0.12.0_amd64.deb

# Fallback if needed:
# sudo dpkg -i ./target/release/bundle/deb/ArtCraft_0.12.0_amd64.deb
# sudo apt-get -f install -y
```

**Launch/verify:**
- From terminal (if installed into PATH): `artcraft`
- Or locate via application launcher (“ArtCraft”)

**Status:** ⏳ Pending

---

### Task 3 (Optional): Re-run build to confirm RPM bundling no longer hangs

**SubAgent:** `primary`  
**Prompt:**
- Re-run `./script/artcraft/unix_build.sh`
- Confirm RPM artifact is produced, or at least that bundling progresses meaningfully.
- Capture/append log to `build-logs/unix_build-fix-sqlite_tasks-build.txt`

**Status:** ⏳ Pending

---

## Final Results

**Status:** Draft

---

*Created: 2026-03-08*