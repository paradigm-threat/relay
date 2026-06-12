# Relay desktop workspace

Native Relay client crates (Ubuntu + Windows):

- **`relay-runtime-desktop`** — winit + tiny-skia hook renderer
- **`relay-client-desktop`** — embedded loopback gateway, git mirror, `--verify`

```bash
cargo build -p relay-client-desktop
cargo test -p relay-client-desktop
./relay-client-desktop/scripts/smoke-http.sh
relay-client-desktop --repo ~/dev/fan-edits
relay-client-desktop --repo ~/dev/fan-edits --ipfs   # Kubo sidecar + local gateway
```

fan-edits helper: `~/dev/fan-edits/scripts/open-relay-desktop.sh` (pass `--ipfs` through).

Depends on sibling **`relay-runtime`** and **`relay-gateway`** (subprocess) checkouts.
