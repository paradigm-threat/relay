mod embedded_server;
mod git_mirror;
mod http_host;
mod ipfs_kubo;
mod verify;

use anyhow::{Context, Result};
use clap::Parser;
use relay_runtime::host::HostHandle;
use relay_runtime_desktop::run_desktop_window;
use std::collections::HashMap;
use std::path::PathBuf;

use embedded_server::EmbeddedRelayServer;
use git_mirror::GitMirror;
use http_host::RelayHttpHost;
use verify::{verify_local_repo, VerifyOptions};

#[derive(Parser, Debug)]
#[command(name = "relay-client-desktop", about = "Relay desktop client")]
struct Args {
    /// Local git checkout path (e.g. ~/dev/fan-edits)
    #[arg(long)]
    repo: Option<PathBuf>,

    /// Remote relay base URL (e.g. https://fan-edits.node.example)
    #[arg(long)]
    remote: Option<String>,

    /// Headless verify: mirror + embedded server + hook render (no window)
    #[arg(long)]
    verify: bool,

    /// Render manifest for --verify
    #[arg(long)]
    manifest: Option<PathBuf>,

    /// Repo name for Host header
    #[arg(long, default_value = "fan-edits")]
    repo_name: String,

    /// Branch
    #[arg(long, default_value = "main")]
    branch: String,

    /// Client GET hook path
    #[arg(long, default_value = "hooks/client/get.jsx")]
    hook_path: String,

    /// Start Kubo (ipfs daemon) if needed and point gateway at local IPFS HTTP API
    #[arg(long)]
    ipfs: bool,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    if args.verify {
        let checkout = args
            .repo
            .clone()
            .context("--verify requires --repo PATH")?;
        let manifest = args.manifest.or_else(|| {
            let fixture =
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/catalog/render-manifest.json");
            if fixture.is_file() && checkout.ends_with("fixtures/catalog") {
                Some(fixture)
            } else {
                None
            }
        });
        verify_local_repo(&VerifyOptions {
            checkout,
            hook_path: Some(args.hook_path),
            manifest_path: manifest,
            branch: Some(args.branch),
        })?;
        eprintln!("verify: OK");
        return Ok(());
    }

    let (base_url, repo_name, branch, host_handle): (String, String, String, HostHandle) =
        if let Some(checkout) = &args.repo {
            git_mirror::ensure_is_git_repo(checkout)?;
            git_mirror::warn_if_dirty(checkout)?;
            let ipfs_sidecar = ipfs_kubo::IpfsSidecar::ensure(args.ipfs)?;
            if let Some(sidecar) = ipfs_sidecar {
                Box::leak(Box::new(sidecar));
            }
            let ipfs_env: Vec<(String, String)> = if args.ipfs || std::env::var("RELAY_DESKTOP_IPFS").ok().as_deref() == Some("1") {
                ipfs_kubo::IpfsSidecar::gateway_env()
            } else {
                Vec::new()
            };
            let ipfs_env_refs: Vec<(&str, &str)> =
                ipfs_env.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
            let mirror = GitMirror::ensure(checkout)?;
            let gateway_repo_path = mirror.repo_path_for_gateway();
            let server = EmbeddedRelayServer::start(
                &gateway_repo_path,
                &mirror.repo_name,
                &mirror.branch,
                &ipfs_env_refs,
            )?;
            let base = server.base_url.clone();
            let rn = server.repo_name.clone();
            let br = server.branch.clone();
            let host =
                RelayHttpHost::with_checkout(&base, &rn, &br, Some(checkout.clone())) as HostHandle;
            Box::leak(Box::new(server));
            (base, rn, br, host)
        } else if let Some(remote) = &args.remote {
            let host = RelayHttpHost::new(remote, &args.repo_name, &args.branch) as HostHandle;
            (
                remote.clone(),
                args.repo_name.clone(),
                args.branch.clone(),
                host,
            )
        } else {
            anyhow::bail!("pass --repo PATH or --remote URL");
        };

    let hook_source = http_get_text(&base_url, &repo_name, &branch, &args.hook_path)?;
    let theme_yaml = http_get_text(
        &base_url,
        &repo_name,
        &branch,
        "hooks/client/theme.yaml",
    )
    .unwrap_or_else(|_| default_theme());
    let modules = load_hook_modules(&base_url, &repo_name, &branch, &hook_source)?;

    run_desktop_window(
        &format!("Relay — {repo_name}"),
        &hook_source,
        &theme_yaml,
        modules,
        host_handle,
    )
    .map_err(|e| anyhow::anyhow!(e))?;

    Ok(())
}

fn http_get_text(base: &str, repo: &str, branch: &str, path: &str) -> Result<String> {
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;
    let url = format!(
        "{}/{}",
        base.trim_end_matches('/'),
        path.trim_start_matches('/')
    );
    let host = if base.contains("127.0.0.1") || base.contains("localhost") {
        format!("{repo}.localhost")
    } else {
        format!("{repo}.relay.local")
    };
    let resp = client
        .get(&url)
        .header("Host", host)
        .header("X-Relay-Branch", branch)
        .send()
        .with_context(|| format!("GET {url}"))?;
    if !resp.status().is_success() {
        anyhow::bail!("GET {url} returned {}", resp.status());
    }
    resp.text().context("response body")
}

fn load_hook_modules(
    base: &str,
    repo: &str,
    branch: &str,
    hook_source: &str,
) -> Result<HashMap<String, String>> {
    let mut modules = HashMap::new();
    for import in simple_import_paths(hook_source) {
        if import.starts_with("react") {
            continue;
        }
        if let Ok(src) = http_get_text(base, repo, branch, &import) {
            modules.insert(import, src);
        }
    }
    Ok(modules)
}

fn simple_import_paths(source: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in source.lines() {
        let line = line.trim();
        if line.starts_with("import ") && line.contains("from ") {
            if let Some(q) = line.rsplit('\'').nth(1).or_else(|| line.rsplit('"').nth(1)) {
                if q.ends_with(".jsx") || q.ends_with(".js") {
                    out.push(q.to_string());
                }
            }
        }
    }
    out
}

fn default_theme() -> String {
    r#"themes:
  light:
    selectors:
      ".p-4":
        padding: "16px"
current_theme: light
"#
    .to_string()
}
