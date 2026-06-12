//! Integration test: headless verify against catalog fixture.

use std::path::PathBuf;

#[test]
fn verify_catalog_fixture() {
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/catalog");
    let manifest = fixture.join("render-manifest.json");
    relay_client_desktop::verify::verify_local_repo(&relay_client_desktop::verify::VerifyOptions {
        checkout: fixture,
        hook_path: None,
        manifest_path: Some(manifest),
        branch: None,
    })
    .expect("verify catalog fixture");
}
