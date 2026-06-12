//! Desktop renderer for relay-runtime VNodes (Skia + winit + taffy).

pub mod app;
pub mod hook;
pub mod layout;
pub mod paint;
pub mod style_props;
pub mod verify;

pub use app::run_desktop_window;
pub use hook::{reset_hook_session, run_hook_to_vnode, run_hook_to_vnode_stable, HookRunResult};
pub use verify::{assert_text_contains, verify_hook_render, VerifyResult};
