# ArtCraft Dev Mode Setup

**Date:** 2026-03-07  
**Status:** ⚠️ Partial - Blocked on system dependencies  
**Agent:** Chip 🐱‍💻

---

## Goal

Get ArtCraft running in development mode on Zorin OS 18 Pro following the official dev setup guide from ArtCraft devs.

---

## Overview

ArtCraft is a Rust/Tauri desktop app. The official dev workflow runs two processes:
1. Frontend dev server (Nx/React on port 5173)
2. Tauri Rust app (connects to frontend)

This approach avoids production build complexity and lets us test functionality first. Once dev mode works, we can explore production packaging.

---

## Prerequisites

Already installed from previous attempt:
- ✅ Rust (v1.94.0)
- ✅ Node.js (v25.4.0)
- ✅ Nx (v22.4.5)
- ✅ Tauri CLI (v2.10.0)

---

## Tasks

### Task 1: Apply Linux System Fix

**SubAgent:** `primary`  
**Prompt:** Apply the inotify watcher fix required for Tauri on Linux:
```bash
echo fs.inotify.max_user_watches=524288 | sudo tee -a /etc/sysctl.conf && sudo sysctl -p
```
Verify the change: `cat /proc/sys/fs/inotify/max_user_watches` should show 524288

**Status:** ✅ Complete

**Results:** The inotify setting was already configured at 524288 (verified via `cat /proc/sys/fs/inotify/max_user_watches`). No action needed. 

---

### Task 2: Install System Dependencies

**SubAgent:** `primary`  
**Prompt:** Install Tauri's Linux system dependencies:
```bash
sudo apt update
sudo apt install -y libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

**Status:** ❌ Failed (requires sudo)

**Results:** Build failed - pkg-config not found. The Tauri Rust build requires system dependencies that need sudo privileges to install. **Manual intervention required:** User needs to run the apt install command above. Error: `glib-sys` build failed because pkg-config package is missing. 

---

### Task 3: Start Frontend Dev Server

**SubAgent:** `primary`  
**Prompt:** Start the frontend dev server in the background:
1. Navigate to `/home/derrick/.openclaw/workspace/projects/artcraft/`
2. Run: `./script/artcraft/unix_frontend_dev.sh &`
3. Wait for it to compile and show "Local: http://localhost:5173/"
4. Capture any errors

**Status:** ✅ Complete

**Results:** Frontend dev server started successfully. Vite v5.4.21 ready in 275ms on http://localhost:5173/. Process running (PID 4123536). 

---

### Task 4: Start Tauri Rust App

**SubAgent:** `primary`  
**Prompt:** Start the Tauri Rust application:
1. Navigate to `/home/derrick/.openclaw/workspace/projects/artcraft/`
2. Run: `./script/artcraft/unix_rust_dev.sh`
3. This should launch the ArtCraft desktop window
4. Monitor for errors

**Status:** ❌ Blocked

**Results:** Blocked on Task 2 - Tauri system dependencies must be installed first. Build failed at `glib-sys v0.18.1` with error: pkg-config not found. Once dependencies are installed, this task can be retried. 

---

### Task 5: Verify Functionality

**SubAgent:** `primary`  
**Prompt:** Test the running ArtCraft app:
1. Does the window appear?
2. Can you interact with the UI?
3. Check both frontend and backend logs for errors
4. Report what works and what doesn't

**Status:** ❌ Blocked

**Results:** Blocked on Task 4 - cannot verify until Tauri app builds and launches. 

---

## Final Results

**Status:** ⚠️ Partial - Frontend works, Rust app blocked on system dependencies

**What We Built:** 
- ✅ Frontend dev server running successfully on http://localhost:5173/
- ❌ Tauri Rust app build failed - missing system dependencies

**Blocker:** Tauri system dependencies require sudo privileges to install. The following command must be run manually by the user:
```bash
sudo apt install -y libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev pkg-config
```

**Next Steps:** 
1. User runs the apt install command above
2. Re-run `./script/artcraft/unix_rust_dev.sh` to build and launch the Tauri app
3. Verify the desktop window appears and is functional

---

*Partially completed on 2026-03-07 - awaiting manual dependency installation*
