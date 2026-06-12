#!/usr/bin/env bash
# PUT allowlisted .md succeeds; PUT .mkv rejected (catalog fixture has pre-commit hook).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
RELAY_ROOT="$(cd "$ROOT/.." && pwd)"
GATEWAY="${RELAY_GATEWAY_BIN:-$RELAY_ROOT/relay-gateway/target/debug/relay-gateway}"
MIRROR_ROOT="${HOME}/.relay/desktop-repos"
FIXTURE="$ROOT/fixtures/catalog"

"$ROOT/scripts/smoke-http.sh" >/dev/null

git -C "$FIXTURE" push --force "$MIRROR_ROOT/catalog.git" HEAD:refs/heads/main 2>/dev/null || true

PORT="$(python3 -c "import socket; s=socket.socket(); s.bind(('127.0.0.1',0)); print(s.getsockname()[1]); s.close()")"
export RELAY_PUBLIC_HOSTNAME=localhost
export RELAY_DESKTOP_MODE=1

"$GATEWAY" serve --repo "$MIRROR_ROOT" --bind "127.0.0.1:$PORT" >/dev/null 2>&1 &
GWPID=$!
trap 'kill "$GWPID" 2>/dev/null || true' EXIT
sleep 0.5

# Allowed: markdown under FanMixes/
MD_CODE="$(curl -s -o /tmp/relay-put-md.txt -w "%{http_code}" -X PUT \
  -H "Host: catalog.localhost" \
  -H "X-Relay-Branch: main" \
  -H "Content-Type: text/markdown" \
  --data "# smoke test" \
  "http://127.0.0.1:$PORT/FanMixes/Sample/SMOKE.md")"

if [[ "$MD_CODE" != "200" ]]; then
  echo "FAIL: PUT .md expected 200, got $MD_CODE" >&2
  cat /tmp/relay-put-md.txt >&2
  exit 1
fi

# Rejected: video extension
VID_CODE="$(curl -s -o /tmp/relay-put-vid.txt -w "%{http_code}" -X PUT \
  -H "Host: catalog.localhost" \
  -H "X-Relay-Branch: main" \
  --data "fake" \
  "http://127.0.0.1:$PORT/FanMixes/Sample/rip.mkv")"

if [[ "$VID_CODE" == "200" ]]; then
  echo "FAIL: PUT .mkv should be rejected, got 200" >&2
  exit 1
fi

echo "OK: PUT allowlist smoke passed (md=$MD_CODE video=$VID_CODE)"
