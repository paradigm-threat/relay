//! Run CleverScript hooks and produce styled VNodes for desktop.

use relay_runtime::host::{Host, HostHandle, ModuleMapHost};
use relay_runtime::markdown_host;
use relay_runtime::run_state::{
    clear_handler_registry_for_vnode_pass, clear_run_state, consume_render_requested,
    register_handler, run_hook_persistent,
};
use relay_runtime::style_resolve::{resolve_vnode_styles, Platform};
use relay_runtime::value::Value;
use relay_runtime::vdom::{value_to_vnode_with_handlers, VNode};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct HookRunResult {
    pub vnode: VNode,
    pub theme_yaml: String,
}

/// Composite host: module loading via ModuleMapHost, fetch/navigate via outer host.
struct CompositeHost {
    modules: Rc<RefCell<ModuleMapHost>>,
    outer: HostHandle,
}

impl Host for CompositeHost {
    fn fetch(&self, url: &str, options: Option<&Value>) -> Result<Value, String> {
        self.outer.borrow().fetch(url, options)
    }

    fn navigate(&self, path: &str) -> Result<(), String> {
        self.outer.borrow().navigate(path)
    }

    fn create_element(
        &self,
        tag: &str,
        props: &Value,
        children: &[Value],
    ) -> Result<Value, String> {
        self.outer.borrow().create_element(tag, props, children)
    }

    fn load_module(&self, path: &str) -> Result<Value, String> {
        if path == "react" || path == "react.js" {
            return Ok(react_module());
        }
        if path == "react/jsx-runtime" || path == "react/jsx-runtime.js" {
            return self.modules.borrow().load_module("react/jsx-runtime");
        }
        if path == "@clevertree/markdown"
            || path == "@clevertree/markdown.js"
            || path == "@clevertree/hook-transpiler"
            || path == "@clevertree/hook-transpiler.js"
        {
            return Ok(markdown_host::markdown_module());
        }
        self.modules.borrow().load_module(path)
    }

    fn log(&self, level: &str, message: &str) {
        tracing::debug!(level, message);
    }
}

/// Clear hook component state (call when opening a new repo or hook source).
pub fn reset_hook_session() {
    clear_run_state();
}

fn react_module() -> Value {
    let mut obj = HashMap::new();
    for name in ["useState", "useEffect", "useCallback", "useRef"] {
        obj.insert(name.to_string(), Value::ReactHook(name.to_string()));
    }
    Value::Object(obj)
}

fn build_composite(host: HostHandle, modules: HashMap<String, String>) -> Rc<RefCell<CompositeHost>> {
    let module_host = ModuleMapHost::new(modules);
    inject_jsx_runtime(&module_host);
    let composite = Rc::new(RefCell::new(CompositeHost {
        modules: module_host.clone(),
        outer: host,
    }));
    module_host
        .borrow()
        .set_parent_host(composite.clone() as HostHandle);
    composite
}

fn vnode_from_hook_result(result: Value, theme_yaml: &str) -> Result<(VNode, String), String> {
    clear_handler_registry_for_vnode_pass();
    let vnode = value_to_vnode_with_handlers(&result, &mut |v| register_handler(v));
    let (resolved, _, _) = resolve_vnode_styles(
        vnode,
        theme_yaml,
        Platform::Desktop,
        1.0,
        1.0,
        Some("light"),
        false,
    );
    Ok((resolved, theme_yaml.to_string()))
}

fn run_one_pass(
    source: &str,
    composite: Rc<RefCell<CompositeHost>>,
    theme_yaml: &str,
) -> Result<HookRunResult, String> {
    let result = run_hook_persistent(source, composite as HostHandle).map_err(|e| format!("{e}"))?;
    let (vnode, theme_yaml) = vnode_from_hook_result(result, theme_yaml)?;
    Ok(HookRunResult { vnode, theme_yaml })
}

pub fn run_hook_to_vnode(
    source: &str,
    host: HostHandle,
    theme_yaml: &str,
    modules: HashMap<String, String>,
) -> Result<HookRunResult, String> {
    run_hook_to_vnode_stable(source, host, theme_yaml, modules, true)
}

/// Run hook until useEffect/setState quiesce (max 16 passes).
pub fn run_hook_to_vnode_stable(
    source: &str,
    host: HostHandle,
    theme_yaml: &str,
    modules: HashMap<String, String>,
    reset: bool,
) -> Result<HookRunResult, String> {
    if reset {
        reset_hook_session();
    }
    let composite = build_composite(host, modules);
    let mut last = run_one_pass(source, composite.clone(), theme_yaml)?;
    for _ in 0..16 {
        if !consume_render_requested() {
            break;
        }
        last = run_one_pass(source, composite.clone(), theme_yaml)?;
    }
    Ok(last)
}

fn inject_jsx_runtime(host: &Rc<RefCell<ModuleMapHost>>) {
    host.borrow_mut().add_module(
        "react/jsx-runtime",
        r#"export function jsx(type, props) {
  const p = props || {};
  const ch = p.children;
  const children = ch == null ? [] : (Array.isArray(ch) ? ch : [ch]);
  delete p.children;
  const tag = typeof type === 'string' ? type : 'div';
  return { type: 'element', tag, props: p, children };
}
export function jsxs(type, props) { return jsx(type, props); }
export function Fragment(props) { return props.children; }"#
            .to_string(),
    );
}
