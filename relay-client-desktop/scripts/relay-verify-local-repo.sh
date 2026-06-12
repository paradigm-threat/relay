#!/usr/bin/env bash
# Verify a local git checkout: mirror → embedded gateway → hook render (headless).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
RELAY_ROOT="$(cd "$ROOT/.." && pwd)"
GATEWAY="${RELAY_GATEWAY_BIN:-$RELAY_ROOT/relay-gateway/target/debug/relay-gateway}"

CHECKOUT="${1:-$ROOT/fixtures/catalog}"
MANIFEST="${2:-}"
if [[ -z "$MANIFEST" && -f "$CHECKOUT/render-manifest.json" ]]; then
  MANIFEST="$CHECKOUT/render-manifest.json"
fi

if [[ ! -x "$GATEWAY" && ! -f "$GATEWAY" ]]; then
  echo "Build relay-gateway first: cd relay-gateway && cargo build --bin relay-gateway" >&2
  exit 1
fi

cd "$RELAY_ROOT"
ARGS=(--verify --repo "$CHECKOUT")
if [[ -n "$MANIFEST" ]]; then
  ARGS+=(--manifest "$MANIFEST")
fi

export RELAY_GATEWAY_BIN="$GATEWAY"
cargo run -q -p relay-client-desktop -- "${ARGS[@]}"
