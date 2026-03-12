# ArtCraft External IPC Control

**Date:** 2026-03-07  
**Status:** In Progress  
**Agent:** Chip 🐱‍💻

---

## Goal

Add external IPC control to ArtCraft via Tauri CLI plugin and/or WebSocket API, enabling programmatic automation from OpenClaw and other external tools.

---

## Overview

ArtCraft is a Tauri app with 48+ exposed commands (e.g., `enqueue_text_to_image_command`), but these are only callable from the frontend via `invoke()`. To enable external control:

1. **Tauri CLI Plugin** - Add `tauri-plugin-cli` to allow external command invocation
2. **WebSocket Server** - Optional: Add a WebSocket endpoint in the Rust backend for real-time control
3. **OpenClaw Integration** - Create wrapper functions to call ArtCraft commands from OpenClaw

This gives us:
- ✅ CLI-based automation (`artcraft-cli invoke enqueue_text_to_image_command ...`)
- ✅ WebSocket-based real-time control (optional, for streaming results)
- ✅ Integration with OpenClaw's orchestration system

---

## Tasks

### Task 1: Analyze Current Tauri Configuration

**SubAgent:** `coder`  
**Prompt:** 
```
Analyze the Tauri configuration in /home/derrick/.openclaw/workspace/projects/gambit-artcraft/crates/desktop/artcraft/tauri.conf.json

1. Check if tauri-plugin-cli is already installed
2. Review the "allowlist" or "permissions" section for IPC capabilities
3. Check if there's an existing CLI configuration
4. Look at the main.rs or lib.rs to see how commands are registered

Report back with:
- Current Tauri version
- Existing plugins
- Command registration pattern
- Any security restrictions we need to work around
```

**Files Modified:**
- None (analysis only)

**Status:** ⏳ Pending

**Results:** 

---

### Task 2: Add Tauri CLI Plugin

**SubAgent:** `coder`  
**Prompt:**
```
Add tauri-plugin-cli to the ArtCraft desktop app to enable external command invocation.

1. Add `tauri-plugin-cli` to Cargo.toml dependencies in crates/desktop/artcraft/
2. Update tauri.conf.json to enable CLI plugin with proper permissions
3. Register the plugin in the Tauri app builder (lib.rs or main.rs)
4. Ensure all existing Tauri commands are exposed to CLI

Test by building:
```bash
cd /home/derrick/.openclaw/workspace/projects/gambit-artcraft/
./script/artcraft/unix_build.sh
```

Report any compilation errors or permission issues.
```

**Files Modified:**
- `crates/desktop/artcraft/Cargo.toml`
- `crates/desktop/artcraft/tauri.conf.json`
- `crates/desktop/artcraft/src/lib.rs`

**Status:** ⏳ Pending

**Results:** 

---

### Task 3: Create CLI Command Wrapper Script

**SubAgent:** `coder`  
**Prompt:**
```
Create a wrapper script at /home/derrick/.openclaw/workspace/projects/gambit-artcraft/artcraft-cli.sh that:

1. Uses `cargo-tauri` or the built binary to invoke Tauri commands
2. Supports common operations:
   - `./artcraft-cli.sh generate:text-to-image "prompt here"`
   - `./artcraft-cli.sh generate:image-to-video "image_path" "prompt"`
   - `./artcraft-cli.sh queue:list`
   - `./artcraft-cli.sh queue:dismiss <task_id>`
3. Handles authentication/credentials if needed
4. Returns JSON output for easy parsing

Make it executable and test with a simple command.
```

**Files Created:**
- `artcraft-cli.sh`

**Status:** ⏳ Pending

**Results:** 

---

### Task 4: Optional - Add WebSocket Server Plugin

**SubAgent:** `coder`  
**Prompt:**
```
Add a WebSocket server to the ArtCraft Rust backend for real-time external control.

1. Add `tokio-tungstenite` or `warp` WebSocket dependency to Cargo.toml
2. Create a new thread/service in the Tauri app that listens on localhost:PORT
3. Implement WebSocket message handlers for:
   - `{"command": "enqueue_text_to_image", "params": {...}}`
   - `{"command": "get_task_queue"}`
   - `{"command": "subscribe_to_updates"}`
4. Send task completion events back via WebSocket
5. Add authentication token for security (optional)

Test by starting the app and connecting with a WebSocket client.
```

**Files Created/Modified:**
- `crates/desktop/artcraft/Cargo.toml` (add WebSocket dependency)
- `crates/desktop/artcraft/src/core/websocket_server.rs` (new)
- `crates/desktop/artcraft/src/lib.rs` (integrate WebSocket thread)

**Status:** ⏳ Pending

**Results:** 

---

### Task 5: OpenClaw Integration - Create ArtCraft Client Module

**SubAgent:** `coder`  
**Prompt:**
```
Create an OpenClaw module to control ArtCraft programmatically.

Location: /home/derrick/.openclaw/workspace/projects/openclaw-artcraft-client/

Create:
1. `artcraft_client.py` or `artcraft_client.ts` with functions:
   - `generate_text_to_image(prompt, provider, options)`
   - `generate_image_to_video(image_path, prompt, options)`
   - `get_task_queue()`
   - `dismiss_task(task_id)`
   - `wait_for_completion(task_id, timeout)`

2. Support both CLI and WebSocket backends:
   - CLI: subprocess calls to artcraft-cli.sh
   - WebSocket: async WebSocket connection

3. Add example usage script showing how to integrate with OpenClaw workflows

4. Test with a simple generation request
```

**Files Created:**
- `projects/openclaw-artcraft-client/artcraft_client.py` (or .ts)
- `projects/openclaw-artcraft-client/example_usage.py`
- `projects/openclaw-artcraft-client/README.md`

**Status:** ⏳ Pending

**Results:** 

---

### Task 6: Test End-to-End Workflow

**SubAgent:** `primary`  
**Prompt:**
```
Test the complete ArtCraft external control workflow:

1. Start ArtCraft desktop app
2. From OpenClaw, call the client module to:
   - Generate a text-to-image
   - Monitor the task queue
   - Wait for completion
   - Retrieve the generated image

3. Document any issues or limitations
4. Verify the app doesn't crash or behave unexpectedly

Report results with screenshots or output logs.
```

**Status:** ⏳ Pending

**Results:** 

---

### Task 7: Documentation & Commit

**SubAgent:** `primary`  
**Prompt:**
```
1. Update README.md in gambit-artcraft with CLI usage instructions
2. Create a PR description for upstream ArtCraft repo
3. Commit all changes to GambitGamesLLC/artcraft fork
4. Push to main branch
5. Update this plan with final results and commit hashes
```

**Files Modified:**
- `README.md`
- `docs/external-control.md` (new)

**Status:** ⏳ Pending

**Results:** 

---

## Final Results

**Status:** ⏳ In Progress

**What We Built:** 

**Commits:**

**Lessons Learned:** 

---

*Plan created on 2026-03-07*
