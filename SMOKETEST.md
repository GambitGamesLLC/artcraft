# CLI Smoke Test (chip/cli-config-cross-platform)

Date: 2026-03-09

## Binary under test

Preferred binary would have been the installed one at `/usr/bin/artcraft`, but it did **not** match the current branch tip.

- Repo HEAD: `b628abd35e9d217bd8f91fa35b103e1237d5479a` (branch `chip/cli-config-cross-platform`)
- `/usr/bin/artcraft --version` reported git commit: `4463ab1a3fa4d4eb87c32023485fe08762546c19`

So this smoke test uses the locally built binary:

- `./target/release/artcraft`

Build command used:

```bash
cd /home/derrick/.openclaw/workspace/projects/gambit-artcraft
~/.cargo/bin/cargo build -p artcraft --release
```

## Exit code / behavior validation

All commands were run from:

```bash
cd /home/derrick/.openclaw/workspace/projects/gambit-artcraft
```

### 1) Allowlisted safe command succeeds (exit 0)

Command:

```bash
./target/release/artcraft invoke --json platform_info_command
```

Output:

```json
{"status":"success","payload":{"os_platform":"linux","webview_runtime":"webkit_gtk"}}
```

Exit code: `0`

### 2) Disallowed safe command returns exit 3 (no `--unsafe`)

Command:

```bash
./target/release/artcraft invoke --json get_provider_order_command
```

Output:

```json
{"status":"bad_request","error_message":"unknown or disallowed command: get_provider_order_command. Safe allowlist: [\"platform_info_command\", \"get_app_info_command\", \"get_task_queue_command\"]. Unsafe allowlist (requires --unsafe + gate): [\"get_provider_order_command\"]"}
```

Exit code: `3`

### 3) `--unsafe` without gate returns exit 2

Command:

```bash
env -u ARTCRAFT_ENABLE_UNSAFE_INVOKE \
  ./target/release/artcraft invoke --json --unsafe get_provider_order_command
```

Output:

```json
{"status":"bad_request","error_message":"--unsafe requested but gate is disabled; set ARTCRAFT_ENABLE_UNSAFE_INVOKE=1 or /home/derrick/.config/artcraft/cli.json with {\"enableUnsafeInvoke\":true}"}
```

Exit code: `2`

### 4) `--unsafe` with `ARTCRAFT_ENABLE_UNSAFE_INVOKE=1` returns exit 0

Command:

```bash
ARTCRAFT_ENABLE_UNSAFE_INVOKE=1 \
  ./target/release/artcraft invoke --json --unsafe get_provider_order_command
```

Output:

```json
{"status":"success","payload":{"providers":["artcraft","sora","fal"]}}
```

Exit code: `0`
