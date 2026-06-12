//! Headless hook verification (no winit window).

use crate::hook::run_hook_to_vnode_stable;
use relay_runtime::host::HostHandle;
use relay_runtime::vdom::VNode;
use std::collections::HashMap;

#[derive(Debug)]
pub struct VerifyResult {
    pub text: String,
    pub vnode: VNode,
}

/// Run hook + theme until stable; return normalized text for assertions.
pub fn verify_hook_render(
    hook_source: &str,
    theme_yaml: &str,
    modules: HashMap<String, String>,
    host: HostHandle,
) -> Result<VerifyResult, String> {
    let result = run_hook_to_vnode_stable(hook_source, host, theme_yaml, modules, true)?;
    let vnode = result.vnode.clone();
    let text = vnode.normalize().text_content();
    Ok(VerifyResult { text, vnode: result.vnode })
}

/// Assert every needle appears in rendered text (case-sensitive).
pub fn assert_text_contains(text: &str, needles: &[&str]) -> Result<(), String> {
    for needle in needles {
        if !text.contains(needle) {
            return Err(format!("expected text to contain {needle:?}"));
        }
    }
    Ok(())
}
