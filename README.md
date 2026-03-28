# Internode CLI

Agent-native CLI for Internode Organizational Intelligence. Designed for AI agents (Claude Code, Cursor, etc.) to use as long-term memory.

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/istvanspace/internode-cli/main/install.sh | sh
```

Or build from source:

```bash
cd cli
cargo build --release
# Binary: target/release/internode
```

## Download `SKILL.md`

If you want the agent skill file locally:

```bash
curl -fsSL https://raw.githubusercontent.com/istvanspace/internode-cli/main/SKILL.md -o SKILL.md
```

Install it directly into Cursor skills:

```bash
mkdir -p ~/.cursor/skills/use-internode-cli
curl -fsSL https://raw.githubusercontent.com/istvanspace/internode-cli/main/SKILL.md -o ~/.cursor/skills/use-internode-cli/SKILL.md
```

Install it into Claude Code skills (personal scope):

```bash
mkdir -p ~/.claude/skills/use-internode-cli
curl -fsSL https://raw.githubusercontent.com/istvanspace/internode-cli/main/SKILL.md -o ~/.claude/skills/use-internode-cli/SKILL.md
```

Install it into Codex skills:

```bash
# User scope (available in all repos)
mkdir -p ~/.agents/skills/use-internode-cli
curl -fsSL https://raw.githubusercontent.com/istvanspace/internode-cli/main/SKILL.md -o ~/.agents/skills/use-internode-cli/SKILL.md

# Repo scope (available only in current repo)
mkdir -p .agents/skills/use-internode-cli
curl -fsSL https://raw.githubusercontent.com/istvanspace/internode-cli/main/SKILL.md -o .agents/skills/use-internode-cli/SKILL.md
```

## Setup

1. Log in to [app.internode.ai](https://app.internode.ai)
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

## Permissions Model

| Action | Allowed |
|--------|---------|
| Read / list all entities | Yes |
| Update task properties | Yes |
| Create projects | Yes |
| Create other entities | No |
| Update non-task entities | No |
| Delete any entity | No |

## Usage

All commands output structured JSON on stdout. Diagnostics go to stderr.

### Topics

```bash
internode topics list
internode topics list --search "authentication" --category 2 --limit 20
```

### Sub-topics

```bash
internode subtopics list
internode subtopics list --type Idea --topic <topic_id>
internode subtopics list --type Problem --limit 10
```

### Tasks

```bash
internode tasks list
internode tasks list --team <team_id> --status <status_id> --priority high
internode tasks list --topic <topic_id> --intent <intent_id>
internode tasks list --topic-category "Technology & Engineering"
internode tasks update <id> --priority medium --assignee user@example.com
internode tasks update <id> --team <team_id> --project <project_id>
internode tasks update <id> --status <status_id> --due-date 2026-04-01
internode tasks update <id> --user-notes "Blocked on review" --type action_item
```

### Decisions

```bash
internode decisions list
internode decisions list --search "pricing model" --limit 10
```

### Intents

```bash
internode intents list
internode intents list --limit 50
```

### Entity Details

Retrieve full knowledge molecules (tasks, sub-topics, decisions) or property details (other entity types). Accepts up to 20 IDs.

```bash
internode entity get <id>
internode entity get <id1> <id2> <id3>
```

### Teams / Projects / Statuses

```bash
internode teams list
internode projects list --team <team_id>
internode projects create --name "v2" --team <team_id>
internode projects create --name "v2" --team <team_id> --key PRJ --description "Version 2"
internode statuses list --team <team_id>
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
api_url = "https://api.internode.ai"
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
