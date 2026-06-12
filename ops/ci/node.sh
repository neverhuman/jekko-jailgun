#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

# Node workspace lane for the ported jailgun apps
# (browser-adapter, chrome-bridge, dashboard, fake-chatgpt).
if ! command -v npm >/dev/null 2>&1; then
  echo "npm is required for the node workspace lane" >&2
  exit 1
fi

npm ci
npm run typecheck --workspaces --if-present
npm run test --workspaces --if-present
npm run build --workspaces --if-present
