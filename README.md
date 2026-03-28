# Internode CLI

Agent-native CLI for Internode Organizational Intelligence. Designed for AI agents (Claude Code, Cursor, etc.) to use as long-term memory.

## Architecture

```
User Machine                    Hosted Services
┌──────────┐    X-CLI-API-Key   ┌──────────────────────┐
│ internode │ ────────────────── │ api.internode.work    │
│   CLI     │                   │ (agentops-api)        │
└──────────┘                   │  └─ /internode-tools/  │
                                │      cli/oi/*          │
                                │  └─ agents-postgres    │
                                │  └─ Neo4j              │
                                └──────────────────────┘
```

- No gateway or proxy needed
- CLI talks directly to `api.internode.work`
- Per-user API key authentication (no shared secrets on client)

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/internodelabs/internode-cli/main/install.sh | sh
```

Or build from source:

```bash
cd cli
cargo build --release
# Binary: target/release/internode
```

## Setup

1. Log in to [app.internode.work](https://app.internode.work)
2. Go to **Settings > CLI API Key**
3. Click **Generate Key** and copy it
4. Run:

```bash
internode configure ink_your_api_key_here
```

5. Verify:

```bash
internode auth status
```

## Usage

All commands output structured JSON on stdout. Diagnostics go to stderr.

### Topics

```bash
internode topics list --limit 10
internode topics get <id> --with-related
internode topics create --title "My Topic"
internode topics update <id> --title "New Title"
internode topics delete <id>
```

### Tasks

```bash
internode tasks list --status open --team <team_id>
internode tasks get <id> --with-related
internode tasks create --title "My Task" --priority high
internode tasks update <id> --priority medium
internode tasks delete <id>
```

### Decisions

```bash
internode decisions get <id> --with-related
internode decisions update <id> --title "Updated"
internode decisions delete <id>
```

### Teams / Projects / Statuses

```bash
internode teams list
internode teams create --name "Engineering"
internode projects list --team <team_id>
internode projects create --name "v2" --team <team_id>
internode statuses list --team <team_id>
internode statuses create --name "In Progress" --team <team_id>
```

### Search

```bash
internode search "authentication architecture" --top-k 10
```

### Context (LLM-optimized)

```bash
internode context --max-tokens 4000
```

## Configuration

Config file: `~/.config/internode/config.toml`

```toml
api_url = "https://api.internode.work"
api_key = "ink_..."
```

## Output Format

All commands return JSON envelopes:

```json
{"ok": true, "data": { ... }}
```

Errors:

```json
{"ok": false, "error": {"code": "AUTH_ERROR", "message": "..."}}
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Bad input |
| 2 | Auth error |
| 3 | Server error |
| 4 | Network error |

## Release

GitHub Actions cross-compiles on tag push:
- Linux AMD64
- macOS ARM64 (Apple Silicon)
- Windows x64
