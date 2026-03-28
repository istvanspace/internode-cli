#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "${SCRIPT_DIR}"

WORKFLOW_NAME="Release CLI Binaries"
FORCE_RECREATE=false
WATCH_RUN=true
VERSION_INPUT=""

usage() {
  cat <<'EOF'
Usage:
  ./release.sh [version] [--force] [--no-watch]

Examples:
  ./release.sh
  ./release.sh 0.3.0
  ./release.sh v0.3.0
  ./release.sh v0.3.0 --force

Behavior:
  - If version is omitted, reads version from cli/Cargo.toml
  - Creates and pushes an annotated git tag (vX.Y.Z)
  - Watches the GitHub Actions release workflow
  - Prints the published GitHub Release URL and asset names

Options:
  --force     Delete existing release/tag with the same version, then recreate
  --no-watch  Do not wait for workflow/release completion
  -h, --help  Show this help
EOF
}

log() {
  printf '[release] %s\n' "$*"
}

fail() {
  printf '[release][error] %s\n' "$*" >&2
  exit 1
}

require_cmd() {
  command -v "$1" >/dev/null 2>&1 || fail "Missing required command: $1"
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --force)
      FORCE_RECREATE=true
      shift
      ;;
    --no-watch)
      WATCH_RUN=false
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    -*)
      fail "Unknown option: $1"
      ;;
    *)
      if [[ -n "${VERSION_INPUT}" ]]; then
        fail "Only one version argument is allowed."
      fi
      VERSION_INPUT="$1"
      shift
      ;;
  esac
done

require_cmd git
require_cmd gh
require_cmd sed

git rev-parse --is-inside-work-tree >/dev/null 2>&1 || fail "Run this script inside the repo."
gh auth status >/dev/null 2>&1 || fail "GitHub CLI is not authenticated. Run: gh auth login"

if [[ -z "${VERSION_INPUT}" ]]; then
  VERSION_INPUT="$(sed -nE 's/^version[[:space:]]*=[[:space:]]*"([^"]+)".*/\1/p' cli/Cargo.toml | head -n 1)"
  [[ -n "${VERSION_INPUT}" ]] || fail "Could not read version from cli/Cargo.toml"
fi

TAG="${VERSION_INPUT}"
if [[ "${TAG}" != v* ]]; then
  TAG="v${TAG}"
fi

if [[ ! "${TAG}" =~ ^v[0-9]+\.[0-9]+\.[0-9]+([.-][0-9A-Za-z.-]+)?$ ]]; then
  fail "Invalid version/tag '${TAG}'. Expected semver-like tag (example: v0.3.0)."
fi

CURRENT_BRANCH="$(git rev-parse --abbrev-ref HEAD)"
[[ "${CURRENT_BRANCH}" == "main" ]] || fail "Current branch is '${CURRENT_BRANCH}'. Switch to main first."

git fetch origin main --tags >/dev/null

HEAD_SHA="$(git rev-parse HEAD)"
LOCAL_MAIN_SHA="$(git rev-parse main)"
REMOTE_MAIN_SHA="$(git rev-parse origin/main)"

[[ "${HEAD_SHA}" == "${LOCAL_MAIN_SHA}" ]] || fail "HEAD is not at local main tip."
[[ "${LOCAL_MAIN_SHA}" == "${REMOTE_MAIN_SHA}" ]] || fail "Local main is not in sync with origin/main."

if [[ -n "$(git status --porcelain)" ]]; then
  fail "Working tree is not clean. Commit/stash changes before releasing."
fi

LOCAL_TAG_EXISTS=false
REMOTE_TAG_EXISTS=false
RELEASE_EXISTS=false

if git rev-parse "${TAG}" >/dev/null 2>&1; then
  LOCAL_TAG_EXISTS=true
fi

if [[ -n "$(git ls-remote --tags origin "refs/tags/${TAG}")" ]]; then
  REMOTE_TAG_EXISTS=true
fi

if gh release view "${TAG}" >/dev/null 2>&1; then
  RELEASE_EXISTS=true
fi

if [[ "${FORCE_RECREATE}" == false && ( "${LOCAL_TAG_EXISTS}" == true || "${REMOTE_TAG_EXISTS}" == true || "${RELEASE_EXISTS}" == true ) ]]; then
  fail "Tag/release '${TAG}' already exists. Use --force to recreate it."
fi

if [[ "${FORCE_RECREATE}" == true ]]; then
  log "Force mode enabled; deleting existing release/tag if present."

  if [[ "${RELEASE_EXISTS}" == true ]]; then
    gh release delete "${TAG}" --yes
  fi

  if [[ "${LOCAL_TAG_EXISTS}" == true ]]; then
    git tag -d "${TAG}" >/dev/null
  fi

  if [[ "${REMOTE_TAG_EXISTS}" == true ]]; then
    git push origin ":refs/tags/${TAG}" >/dev/null
  fi
fi

log "Creating annotated tag ${TAG} at ${HEAD_SHA}."
git tag -a "${TAG}" -m "Release ${TAG}"

log "Pushing tag ${TAG} to origin."
git push origin "${TAG}"

if [[ "${WATCH_RUN}" == false ]]; then
  log "Tag pushed. Skipping workflow watch (--no-watch)."
  exit 0
fi

log "Waiting for workflow '${WORKFLOW_NAME}' to appear..."
RUN_ID=""
for _ in $(seq 1 45); do
  RUN_ID="$(gh run list --workflow "${WORKFLOW_NAME}" --event push --limit 30 --json databaseId,headSha --jq ".[] | select(.headSha==\"${HEAD_SHA}\") | .databaseId" | head -n 1 || true)"
  if [[ -n "${RUN_ID}" ]]; then
    break
  fi
  sleep 2
done

[[ -n "${RUN_ID}" ]] || fail "Could not find a workflow run for ${TAG}."

log "Watching workflow run ${RUN_ID}..."
gh run watch "${RUN_ID}" --exit-status

log "Waiting for GitHub Release ${TAG}..."
RELEASE_URL=""
for _ in $(seq 1 60); do
  RELEASE_URL="$(gh release view "${TAG}" --json url --jq .url 2>/dev/null || true)"
  if [[ -n "${RELEASE_URL}" ]]; then
    break
  fi
  sleep 2
done

[[ -n "${RELEASE_URL}" ]] || fail "Workflow finished but release ${TAG} was not found."

echo
log "Release published: ${RELEASE_URL}"
log "Assets:"
gh release view "${TAG}" --json assets --jq '.assets[].name'
