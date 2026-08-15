# Relay desktop workspace

Native desktop crates live here (**not** the Android tester, **not** the gateway).

- **Android ↔ web testing:** sibling [`relay-runtime/docs/ANDROID_WEB_TEST.md`](relay-runtime/docs/ANDROID_WEB_TEST.md)
- **Gateway:** sibling `relay-gateway/` → GitLab **`clevertree/relay-gateway`**
- **Product path for desktop users:** web client. Native Windows/macOS UI is skipped.

## Crates in this git repo

- **`relay-runtime-desktop`** — winit + tiny-skia (experimental; not the tester path)
- **`relay-client-desktop`** — embedded loopback gateway, git mirror, `--verify`

```bash
cargo build -p relay-client-desktop
cargo test -p relay-client-desktop
./relay-client-desktop/scripts/smoke-http.sh
relay-client-desktop --repo ~/dev/fan-edits
relay-client-desktop --repo ~/dev/fan-edits --ipfs   # Kubo sidecar + local gateway
```

fan-edits helper: `~/dev/fan-edits/scripts/open-relay-desktop.sh` (pass `--ipfs` or `--verify` through).

Headless verify uses `render-manifest.json` in the checkout when present.

Depends on sibling **`relay-runtime`** and **`relay-gateway`** (subprocess) checkouts.
