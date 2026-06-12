#!/usr/bin/env bash
# Smoke test: mirror fixture checkout → embedded gateway → GET hook via Host header.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
RELAY_ROOT="$(cd "$ROOT/.." && pwd)"
FIXTURE="$ROOT/fixtures/catalog"
GATEWAY="${RELAY_GATEWAY_BIN:-$RELAY_ROOT/relay-gateway/target/debug/relay-gateway}"

if [[ ! -x "$GATEWAY" && ! -f "$GATEWAY" ]]; then
  echo "Build relay-gateway first: cd relay-gateway && cargo build --bin relay-gateway" >&2
  exit 1
fi

# Ensure fixture is a git repo with committed hooks
if [[ ! -d "$FIXTURE/.git" ]]; then
  git -C "$FIXTURE" init -b main
  git -C "$FIXTURE" add .
  git -C "$FIXTURE" commit -m "fixture: catalog hooks for desktop smoke test"
fi

MIRROR_ROOT="${HOME}/.relay/desktop-repos"
mkdir -p "$MIRROR_ROOT"
BARE="$MIRROR_ROOT/catalog.git"
if [[ ! -d "$BARE" ]]; then
  git init --bare "$BARE"
fi
git -C "$FIXTURE" push --force "$BARE" HEAD:refs/heads/main

PORT="$(python3 -c "import socket; s=socket.socket(); s.bind(('127.0.0.1',0)); print(s.getsockname()[1]); s.close()")"
export RELAY_PUBLIC_HOSTNAME=localhost
"$GATEWAY" serve --repo "$MIRROR_ROOT" --bind "127.0.0.1:$PORT" >/dev/null 2>&1 &
GWPID=$!
trap 'kill "$GWPID" 2>/dev/null || true' EXIT

for _ in $(seq 1 50); do
  if curl -sf -o /dev/null -H "Host: catalog.localhost" -H "X-Relay-Branch: main" \
    "http://127.0.0.1:$PORT/hooks/client/get.jsx"; then
    break
  fi
  sleep 0.1
done

CODE="$(curl -s -o /tmp/relay-smoke-hook.jsx -w "%{http_code}" \
  -H "Host: catalog.localhost" \
  -H "X-Relay-Branch: main" \
  "http://127.0.0.1:$PORT/hooks/client/get.jsx")"

if [[ "$CODE" != "200" ]]; then
  echo "FAIL: GET hook returned $CODE" >&2
  cat /tmp/relay-smoke-hook.jsx >&2 || true
  exit 1
fi

grep -q "Fan Edits" /tmp/relay-smoke-hook.jsx
echo "OK: smoke-http passed (port=$PORT)"
