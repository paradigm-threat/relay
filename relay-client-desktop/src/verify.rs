//! Headless verification: mirror → embedded gateway → hook render.

use crate::embedded_server::EmbeddedRelayServer;
use crate::git_mirror::GitMirror;
use crate::http_host::RelayHttpHost;
use anyhow::{Context, Result};
use relay_runtime_desktop::{assert_text_contains, verify_hook_render};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
struct RenderManifest {
    entry: String,
    cases: Vec<RenderCase>,
}

#[derive(Debug, Deserialize)]
struct RenderCase {
    name: String,
    #[serde(default)]
    assert_text: Vec<String>,
}

pub struct VerifyOptions {
    pub checkout: PathBuf,
    pub hook_path: Option<String>,
    pub manifest_path: Option<PathBuf>,
    pub branch: Option<String>,
}

pub fn verify_local_repo(opts: &VerifyOptions) -> Result<()> {
    crate::git_mirror::ensure_is_git_repo(&opts.checkout)?;
    crate::git_mirror::warn_if_dirty(&opts.checkout)?;

    let mirror = GitMirror::ensure(&opts.checkout)?;
    let gateway_repo_path = mirror.repo_path_for_gateway();
    let server = EmbeddedRelayServer::start(
        &gateway_repo_path,
        &mirror.repo_name,
        &mirror.branch,
        &[],
    )?;

    let host = RelayHttpHost::with_checkout(
        &server.base_url,
        &mirror.repo_name,
        &mirror.branch,
        Some(opts.checkout.clone()),
    );
    let hook_path = opts
        .hook_path
        .clone()
        .unwrap_or_else(|| "hooks/client/get.jsx".to_string());
    let branch = opts.branch.clone().unwrap_or(mirror.branch.clone());

    let hook_source = http_get_text(&server.base_url, &mirror.repo_name, &branch, &hook_path)?;
    let theme_yaml = http_get_text(
        &server.base_url,
        &mirror.repo_name,
        &branch,
        "hooks/client/theme.yaml",
    )
    .unwrap_or_else(|_| default_theme());
    let modules = load_hook_modules(
        &server.base_url,
        &mirror.repo_name,
        &branch,
        &hook_source,
    )?;

    let rendered = verify_hook_render(&hook_source, &theme_yaml, modules, host)
        .map_err(|e| anyhow::anyhow!(e))?;

    if let Some(manifest_path) = &opts.manifest_path {
        run_manifest(&rendered.text, manifest_path)?;
    } else {
        assert_text_contains(&rendered.text, &["Relay", "catalog"])
            .map_err(|e| anyhow::anyhow!(e))?;
    }

    drop(server);
    Ok(())
}

fn run_manifest(text: &str, path: &Path) -> Result<()> {
    let raw = std::fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    let manifest: RenderManifest =
        serde_json::from_str(&raw).context("parse render-manifest.json")?;
    for case in &manifest.cases {
        let needles: Vec<&str> = case.assert_text.iter().map(String::as_str).collect();
        assert_text_contains(text, &needles)
            .map_err(|e| anyhow::anyhow!("case {}: {e}", case.name))?;
        eprintln!("PASS: {}", case.name);
    }
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
    let host = format!("{repo}.localhost");
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
