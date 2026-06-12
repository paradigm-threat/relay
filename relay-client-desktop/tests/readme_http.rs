//! Gateway serves project README from bare mirror (Phase 2 asset reads).

use relay_client_desktop::git_mirror::GitMirror;
use relay_client_desktop::embedded_server::EmbeddedRelayServer;
use std::path::PathBuf;

#[test]
fn fixture_readme_available_via_gateway() {
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/catalog");
    let mirror = GitMirror::ensure(&fixture).expect("mirror");
    let server = EmbeddedRelayServer::start(
        &mirror.repo_path_for_gateway(),
        &mirror.repo_name,
        &mirror.branch,
    )
    .expect("embedded server");

    let client = reqwest::blocking::Client::new();
    let url = format!(
        "{}/FanMixes/Sample/README.md",
        server.base_url.trim_end_matches('/')
    );
    let resp = client
        .get(&url)
        .header("Host", format!("{}.localhost", mirror.repo_name))
        .header("X-Relay-Branch", &mirror.branch)
        .send()
        .expect("GET README");
    assert!(resp.status().is_success(), "README GET failed: {}", resp.status());
    let body = resp.text().expect("body");
    assert!(
        body.contains("Fixture README"),
        "expected fixture README content, got: {}",
        &body[..body.len().min(200)]
    );
}
