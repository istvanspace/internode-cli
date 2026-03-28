---
name: use-internode-cli
description: Interface with Internode Organizational Intelligence (OI) via the internode CLI. Use when the user asks to read, update tasks, search, or browse knowledge entities (topics, sub-topics, tasks, decisions, intents, teams, projects, statuses), or when bootstrapping context for a work session.
---

# Using the Internode CLI

The `internode` CLI is your interface to a user's **Organizational Intelligence (OI)** — a persistent knowledge graph of topics, sub-topics, tasks, decisions, intents, teams, projects, and statuses. Use it as long-term memory: read context before starting work, browse entities, update task properties, and search the knowledge graph.

## Running the CLI (Docker)

The CLI runs inside a Docker container. To use it, you **must** execute commands inside the container — not on the host.

**Starting the container** (if not already running):

```bash
./internode-cli/docker-up.sh
```

This builds the image and drops into an interactive shell. If the container is already running (check with `docker ps --filter name=internode-cli`), exec into it instead:

```bash
docker exec -it internode-cli bash
```

**All `internode` commands below must be run inside this container shell.** When scripting from the host, prefix with `docker exec`:

```bash
docker exec internode-cli internode topics list
docker exec internode-cli internode search "deployment"
```

## Prerequisites

The CLI must be configured with an API key before use. **Run these inside the container:**

```bash
internode configure <api-key>   # one-time setup; key starts with ink_
internode auth status            # verify the key works
```

Config lives at `~/.config/internode/config.toml` inside the container.

## Output Format

**Every command** prints a single JSON line to stdout.

Success:
```json
{"ok":true,"data":{...}}
```

Error (exit code > 0):
```json
{"ok":false,"error":{"code":"AUTH_ERROR","message":"..."}}
```

Error codes: `BAD_INPUT` (exit 1), `AUTH_ERROR` (exit 2), `SERVER_ERROR` (exit 3), `NETWORK_ERROR` (exit 4).

Human-readable messages go to stderr. **Always parse stdout JSON, ignore stderr.**

## Permissions Model

The CLI is **read-heavy with limited writes**:

- **Read all**: topics, sub-topics, tasks, decisions, intents, teams, projects, statuses
- **Update**: task properties only (title, description, priority, assignee, due date, status, team, project, type, user-notes, blocked-by-reason)
- **Create**: projects only
- **No delete/archive** via CLI

All other entity creation and mutation must be done through the web portal or agent conversations.

## Recommended Workflow

1. **Bootstrap context** at the start of a session:
   ```bash
   internode context --max-tokens 4000
   ```
   This returns a pre-formatted OI summary optimized for LLM consumption.

2. **Search** when you need specific knowledge:
   ```bash
   internode search "deployment pipeline" --top-k 5
   ```

3. **Browse** entity lists to find what you need:
   ```bash
   internode topics list --category 3
   internode tasks list --team <id> --status "In Progress"
   internode subtopics list --type Idea
   ```

4. **Get details** on one or more entities by ID (returns knowledge molecule for tasks/decisions/sub-topics, full properties for everything else; max 20 IDs per call):
   ```bash
   internode entity get <id1> [<id2> ...]
   ```

5. **Update tasks** as you work — change status, assignee, team, project:
   ```bash
   internode tasks update <id> --status <status-id> --assignee "user@example.com"
   ```

## Entity Types

| Entity | What it represents | Key property |
|---|---|---|
| **Topic** | A knowledge area, discussion, or theme | `topic_title` |
| **Sub-topic** | A typed conclusion under a topic (Idea, Problem, Solution, etc.) | `topic_conclusion` |
| **Task** | An actionable work item | `task_title` |
| **Decision** | A resolved choice with rationale | `decision_title` |
| **Intent** | A strategic intent or goal | `intent_title` |
| **Team** | An organizational group | `name` |
| **Project** | A body of work under a team | `name` |
| **Status** | A workflow state for tasks | `name` |

## Command Reference

### Context & Discovery

```bash
internode context [--max-tokens N]
# Full OI context dump for LLM consumption. Use --max-tokens to budget.

internode search "<query>" [--top-k N] [--min-score 0.0-1.0]
# Semantic search across all entity types.

internode entity get <id1> [<id2> ... <idN>]
# Get full details for up to 20 entities by ID. Returns knowledge molecule for
# tasks, decisions, sub-topics. Returns full properties for topics, intents,
# teams, projects, statuses. Response is keyed by entity ID.
```

### List Endpoints

All list commands return lightweight results: `{ items: [{ id, label }], total, limit, offset }` where `label` is the key property capped to 200 characters.

```bash
internode topics list [--category N] [--search "text"] [--limit N] [--offset N]
# List main topics. Filter by topic category index.

internode subtopics list [--type "Idea|Problem|Solution|..."] [--topic ID] [--limit N] [--offset N]
# List sub-topics (topic versions). Filter by type: Outcome, Problem, Constraint,
# Solution, Opportunity, Idea, Information.

internode tasks list [--team ID] [--project ID] [--status "name"] [--priority "..."] [--assignee "email"] [--search "text"] [--topic ID] [--intent ID] [--topic-category "index"] [--limit N] [--offset N]
# List tasks with PM and OI filters. topic, intent, and topic-category
# filter tasks through the decision graph.

internode decisions list [--search "text"] [--limit N] [--offset N]
# List decisions.

internode intents list [--limit N] [--offset N]
# List intents.

internode teams list
# List teams.

internode projects list [--team ID]
# List projects, optionally filtered by team.

internode statuses list [--team ID]
# List statuses, optionally filtered by team.
```

### Task Update

Tasks are the only entity that can be mutated via the CLI.

```bash
internode tasks update <id> [--title "..."] [--description "..."] [--priority "..."] [--assignee "email"] [--due-date "YYYY-MM-DD"] [--status ID] [--type "..."] [--team ID] [--project ID] [--user-notes "..."] [--blocked-by-reason "..."]
```

**Team/project changes:** When changing a task's team, incompatible project, status, and assignee are automatically cleared. The response includes a `cleared_fields` list when this happens. Projects are dependent on teams — ensure the target project belongs to the target team.

### Project Create

```bash
internode projects create --name "..." --team <team-id> [--key "..."] [--description "..."]
```

A project always belongs to a team (`--team` is required on create).

## Key Patterns

### Parse output reliably

From inside the container:
```bash
result=$(internode topics list 2>/dev/null)
if echo "$result" | jq -e '.ok' > /dev/null 2>&1; then
  echo "$result" | jq '.data'
fi
```

From the host (agent scripting):
```bash
result=$(docker exec internode-cli internode topics list 2>/dev/null)
echo "$result" | jq '.data'
```

### Entity detail returns knowledge molecules

For tasks, decisions, and sub-topics, `entity get` returns a **knowledge molecule** — the entity plus its decision-centric neighborhood (related decisions, topics, intents, tasks). For other types, it returns the full property set. You can pass up to 20 IDs in a single call; the response is a dict keyed by entity ID. Entities that fail to resolve return an `error` field instead.

### Mutations are validated server-side

The API enforces allowed fields and entity types. If you send an invalid field, you get a `422` with a descriptive error. Read the error message — it tells you exactly what went wrong.

### IDs are UUIDs

All entity IDs are UUIDs returned in `data` from list/get commands. Store and reuse them.

### Sub-topic types

Sub-topics are typed conclusions attached to topics. Valid types: `Outcome`, `Problem`, `Constraint`, `Solution`, `Opportunity`, `Idea`, `Information`. Filter with `--type` on `subtopics list`.

### Topic categories

Topics are grouped into business categories (index 1-11): Strategy & Leadership, Product & Innovation, Technology & Engineering, People & Talent, Finance & Business Operations, Marketing & Brand, Sales & Revenue, Customer Success & Support, Legal & Regulatory, Data & Analytics, Other. Filter with `--category` on the topics list.
