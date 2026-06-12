//! Minimal demo: render a CleverScript hook in a desktop window.

use relay_runtime::host::{Host, HostHandle, MockHost};
use relay_runtime::value::Value;
use relay_runtime_desktop::run_desktop_window;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

const DEMO_HOOK: &str = r#"
export default function Demo() {
  return <div className="p-4"><span>Relay desktop ok</span></div>
}
"#;

const DEMO_THEME: &str = r#"
themes:
  light:
    variables:
      border-color: "#e5e7eb"
    selectors:
      ".p-4":
        padding: "16px"
current_theme: light
"#;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let host: HostHandle = Rc::new(RefCell::new(MockHost::new()));
    let modules = HashMap::new();
    if let Err(e) = run_desktop_window("Relay Desktop Demo", DEMO_HOOK, DEMO_THEME, modules, host) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
