//! Spawn loopback relay-gateway subprocess for local git repos.

use anyhow::{Context, Result};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::Duration;

pub struct EmbeddedRelayServer {
    child: Child,
    pub base_url: String,
    pub repo_name: String,
    pub branch: String,
}

impl EmbeddedRelayServer {
    pub fn start(repo_path: &Path, repo_name: &str, branch: &str) -> Result<Self> {
        let port = pick_port()?;
        let bind = format!("127.0.0.1:{port}");
        let gateway = find_gateway_binary()?;

        let mut cmd = Command::new(&gateway);
        cmd.arg("serve")
            .arg("--repo")
            .arg(repo_path)
            .arg("--bind")
            .arg(&bind)
            .env("RELAY_DESKTOP_MODE", "1")
            .env("RELAY_PUT_REQUIRES_PRECOMMIT", "1")
            .env("RELAY_PUBLIC_HOSTNAME", "localhost")
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        let child = cmd.spawn().with_context(|| format!("spawn {gateway:?}"))?;

        // Wait for server to accept hook reads (root may 404 without repo host)
        let health_url = format!("http://{bind}/hooks/client/get.jsx");
        let host = format!("{repo_name}.localhost");
        for _ in 0..50 {
            if reqwest::blocking::Client::new()
                .get(&health_url)
                .header("Host", &host)
                .header("X-Relay-Branch", branch)
                .send()
                .map(|r| r.status().is_success() || r.status().as_u16() == 404)
                .unwrap_or(false)
            {
                break;
            }
            std::thread::sleep(Duration::from_millis(100));
        }

        Ok(Self {
            child,
            base_url: format!("http://{bind}"),
            repo_name: repo_name.to_string(),
            branch: branch.to_string(),
        })
    }
}

impl Drop for EmbeddedRelayServer {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn pick_port() -> Result<u16> {
    let listener = TcpListener::bind("127.0.0.1:0").context("bind ephemeral port")?;
    Ok(listener.local_addr()?.port())
}

fn find_gateway_binary() -> Result<PathBuf> {
    if let Ok(p) = std::env::var("RELAY_GATEWAY_BIN") {
        let path = PathBuf::from(p);
        if path.is_file() {
            return Ok(path);
        }
    }
    let dev = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../relay-gateway/target/debug/relay-gateway");
    if dev.is_file() {
        return Ok(dev);
    }
    let release = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../relay-gateway/target/release/relay-gateway");
    if release.is_file() {
        return Ok(release);
    }
    which_gateway()
}

fn which_gateway() -> Result<PathBuf> {
    let out = Command::new("which")
        .arg("relay-gateway")
        .output()
        .context("which relay-gateway")?;
    if out.status.success() {
        let s = String::from_utf8_lossy(&out.stdout);
        let p = s.trim();
        if !p.is_empty() {
            return Ok(PathBuf::from(p));
        }
    }
    anyhow::bail!(
        "relay-gateway binary not found; build relay-gateway or set RELAY_GATEWAY_BIN"
    )
}
