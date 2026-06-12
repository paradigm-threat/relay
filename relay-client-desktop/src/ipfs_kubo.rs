//! Optional Kubo (IPFS) sidecar for local desktop repos with thin assets.

use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::Duration;

pub struct IpfsSidecar {
    child: Option<Child>,
}

impl IpfsSidecar {
    /// Start when `RELAY_DESKTOP_IPFS=1` or `ipfs_auto` is true. No-op if `ipfs id` already works.
    pub fn ensure(ipfs_auto: bool) -> Result<Option<Self>> {
        let enabled = ipfs_auto
            || std::env::var("RELAY_DESKTOP_IPFS").ok().as_deref() == Some("1");
        if !enabled {
            return Ok(None);
        }
        if ipfs_api_ready() {
            tracing::info!("ipfs daemon already reachable on :5001");
            return Ok(Some(Self { child: None }));
        }
        let ipfs = which_ipfs()?;
        tracing::info!("starting kubo daemon ({})", ipfs.display());
        let child = Command::new(&ipfs)
            .arg("daemon")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .with_context(|| format!("spawn ipfs daemon ({})", ipfs.display()))?;
        for _ in 0..60 {
            if ipfs_api_ready() {
                tracing::info!("kubo daemon ready");
                return Ok(Some(Self {
                    child: Some(child),
                }));
            }
            std::thread::sleep(Duration::from_millis(250));
        }
        anyhow::bail!("ipfs daemon did not become ready within 15s");
    }

    pub fn gateway_env() -> Vec<(String, String)> {
        vec![
            ("RELAY_IPFS_GATEWAY_ALLOW_PRIVATE".into(), "1".into()),
            (
                "RELAY_IPFS_GATEWAY".into(),
                std::env::var("RELAY_IPFS_GATEWAY")
                    .unwrap_or_else(|_| "http://127.0.0.1:8080".into()),
            ),
        ]
    }
}

impl Drop for IpfsSidecar {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

fn ipfs_api_ready() -> bool {
    Command::new("ipfs")
        .args(["id"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn which_ipfs() -> Result<PathBuf> {
    if let Ok(p) = std::env::var("IPFS_BIN") {
        let path = PathBuf::from(p);
        if path.is_file() {
            return Ok(path);
        }
    }
    let out = Command::new("which")
        .arg("ipfs")
        .output()
        .context("which ipfs")?;
    if out.status.success() {
        let s = String::from_utf8_lossy(&out.stdout);
        let p = s.trim();
        if !p.is_empty() {
            return Ok(PathBuf::from(p));
        }
    }
    anyhow::bail!("ipfs CLI not found; install Kubo or set IPFS_BIN")
}
