# ArtCraft Installation

**Date:** 2026-03-07  
**Status:** Draft → In Progress → Complete  
**Agent:** Chip 🐱‍💻

---

## Goal

Build and install ArtCraft from source on Zorin OS 18 Pro as a desktop application that can be pinned to the dashboard.

---

## Overview

ArtCraft is a Rust/Tauri desktop application for AI image and video creation. According to the README and dev setup docs, it requires:

1. Rust toolchain
2. Node.js (v24.13.0)
3. Nx (v22.4.5)
4. Tauri CLI (v2.10.0)

The build process involves:
- Building the frontend (React/TypeScript via Nx)
- Building the Rust/Tauri desktop app
- Creating a production build that installs as a native Linux application

We'll use the existing build scripts in `script/artcraft/unix_build.sh` for production build, then create a `.desktop` file for dashboard pinning.

---

## Tasks

### Task 1: Verify Prerequisites

**SubAgent:** `primary`  
**Prompt:** Check if the required tools are installed on the system:
1. Check if Rust is installed (`rustc --version`, `cargo --version`)
2. Check if Node.js is installed (`node --version`, `npm --version`)
3. Check if Tauri CLI is installed (`cargo tauri --version`)
4. Report what's missing and needs installation

**Status:** ✅ Complete

**Results:** 
- ✅ Node.js: v25.4.0 (meets requirement)
- ✅ npm: 11.7.0
- ❌ Rust: Not installed
- ❌ Tauri CLI: Not installed
- ❌ Nx: Not installed 

---

### Task 2: Install Missing Dependencies

**SubAgent:** `primary`  
**Prompt:** Install any missing prerequisites:
- If Rust missing: Install via rustup (https://rustup.rs)
- If Node.js missing: Install via nvm or system package manager
- Install Nx globally: `npm install -g nx@22.4.5`
- Install Tauri CLI: `cargo install tauri-cli --version 2.10.0`
- Install any system dependencies Tauri needs on Linux (libwebkit2gtk, libgtk-3, etc.)

**Status:** ✅ Complete

**Results:** 
- ✅ Rust: Installed (rustc 1.94.0, cargo 1.94.0)
- ✅ Tauri CLI: Installed (v2.10.0)
- ✅ Nx: Installed globally (v22.4.5)
- ⚠️ System deps: Cannot install without sudo/elevated permissions. Will attempt build anyway; Tauri may fail if deps missing. 

---

### Task 3: Build Production ArtCraft

**SubAgent:** `primary`  
**Prompt:** Build ArtCraft for production:
1. Navigate to `/home/derrick/.openclaw/workspace/projects/artcraft/`
2. Run the build script: `./script/artcraft/unix_build.sh`
3. This will build both frontend and Rust components
4. Wait for build to complete (may take 10-30 minutes)
5. Report the build output location

**Status:** ❌ Failed

**Results:** 
Build failed due to OS file watch limit being reached (inotify limit). The system has 65536 watches available but Tauri CLI requires more. Increasing the limit requires sudo (`sysctl fs.inotify.max_user_watches=524288`) which is not available without elevated permissions. Frontend npm install completed successfully (2066 packages). Tauri build step could not start.

**Blocker:** Need elevated permissions to increase inotify watch limit or close other file-watching processes. 

---

### Task 4: Install as Desktop Application

**SubAgent:** `primary`  
**Prompt:** Install ArtCraft as a desktop application:
1. Find the built binary (likely in `target/release/` or `target/*/release/`)
2. Create installation directory: `~/.local/share/artcraft/`
3. Copy binary and assets to installation directory
4. Create a `.desktop` file at `~/.local/share/applications/artcraft.desktop` with:
   - Proper name, icon, and executable path
   - Categories for desktop menu
5. Make the desktop file executable
6. Update desktop database: `update-desktop-database ~/.local/share/applications/`

**Status:** ❌ Skipped

**Results:** 
Skipped - build did not complete successfully. Cannot install without build artifacts. 

---

### Task 5: Create Desktop Launcher Icon

**SubAgent:** `primary`  
**Prompt:** Create or extract an app icon for ArtCraft:
1. Look for existing icons in the repo (check `frontend/`, `crates/desktop/artcraft/`, `.idea/`, `.vscode/`)
2. If found, copy to `~/.local/share/icons/artcraft.png` (or .svg)
3. If not found, create a simple placeholder or download from the repo's README/assets
4. Update the `.desktop` file to reference the icon path
5. Verify the icon appears in the application menu

**Status:** ❌ Skipped

**Results:** 
Skipped - build did not complete. Icons exist in repo (`crates/desktop/artcraft/icons/`) but cannot be installed without successful build. 

---

### Task 6: Test Launch

**SubAgent:** `primary`  
**Prompt:** Test the ArtCraft installation:
1. Try launching via command line: `artcraft` or full path to binary
2. Verify the application starts without errors
3. Check if it appears in the application menu
4. Report any issues or errors

**Status:** ❌ Skipped

**Results:** 
Skipped - no binary was built. 

---

## Final Results

**Status:** ❌ Blocked

**What We Built:** 
Installation was blocked at the build stage. Prerequisites were successfully installed (Rust 1.94.0, Tauri CLI 2.10.0, Nx 22.4.5, Node.js v25.4.0). Frontend dependencies installed successfully (2066 packages). However, the Tauri build step failed due to hitting the OS file watch limit (inotify).

**Blocker:** 
The system's inotify watch limit is set to 65536, which is insufficient for Tauri CLI to monitor all workspace files during build. Increasing this limit requires elevated (sudo) permissions:
```bash
sudo sysctl fs.inotify.max_user_watches=524288
```

Alternatively, closing other file-watching processes (IDEs, file managers, cloud sync tools) may free up enough watches.

**To Complete Installation:**
1. Increase inotify limit: `echo fs.inotify.max_user_watches=524288 | sudo tee -a /etc/sysctl.conf && sudo sysctl -p`
2. Re-run build: `cd /home/derrick/.openclaw/workspace/projects/artcraft/ && ./script/artcraft/unix_build.sh`
3. Once built, the binary will be in `target/release/artcraft`
4. Complete Tasks 4-6 to install as desktop app

**Commits:**
- N/A (build artifact, not source changes)

**Lessons Learned:** 
- Tauri CLI v2 requires significant file watchers for workspace monitoring
- Linux systems may need inotify limits adjusted for large monorepos
- System-level Tauri dependencies (libwebkit2gtk-dev, etc.) couldn't be tested due to sudo requirement

---

*Completed on 2026-03-07*
