#!/usr/bin/env bash
# Assert PUT is rejected when pre-commit hook is not configured (desktop mode).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
RELAY_ROOT="$(cd "$ROOT/.." && pwd)"
GATEWAY="${RELAY_GATEWAY_BIN:-$RELAY_ROOT/relay-gateway/target/debug/relay-gateway}"
MIRROR_ROOT="${HOME}/.relay/desktop-repos"

"$ROOT/scripts/smoke-http.sh" >/dev/null

PORT="$(python3 -c "import socket; s=socket.socket(); s.bind(('127.0.0.1',0)); print(s.getsockname()[1]); s.close()")"
export RELAY_PUBLIC_HOSTNAME=localhost
export RELAY_DESKTOP_MODE=1
export RELAY_PUT_REQUIRES_PRECOMMIT=1

"$GATEWAY" serve --repo "$MIRROR_ROOT" --bind "127.0.0.1:$PORT" >/dev/null 2>&1 &
GWPID=$!
trap 'kill "$GWPID" 2>/dev/null || true' EXIT
sleep 0.5

CODE="$(curl -s -o /tmp/relay-put.txt -w "%{http_code}" -X PUT \
  -H "Host: catalog.localhost" \
  -H "X-Relay-Branch: main" \
  -H "Content-Type: text/plain" \
  --data "test" \
  "http://127.0.0.1:$PORT/test.txt")"

if [[ "$CODE" == "200" ]]; then
  echo "FAIL: PUT should be rejected without pre-commit hook (got 200)" >&2
  cat /tmp/relay-put.txt >&2
  exit 1
fi

echo "OK: PUT gating smoke passed (HTTP $CODE)"
