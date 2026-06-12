//! HTTP Host for CleverScript fetch — remote relay or embedded loopback.
//! Local checkout overlay serves gitignored text files before HTTP (Phase 2.5).

use relay_runtime::host::{host_fetch_object, Host};
use relay_runtime::value::Value;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

pub struct RelayHttpHost {
    base_url: String,
    repo: String,
    branch: String,
    client: reqwest::blocking::Client,
    /// When set, GET for repo-relative paths reads the working tree first.
    checkout_root: Option<PathBuf>,
}

impl RelayHttpHost {
    pub fn new(base_url: impl Into<String>, repo: impl Into<String>, branch: impl Into<String>) -> Rc<RefCell<Self>> {
        Self::with_checkout(base_url, repo, branch, None)
    }

    pub fn with_checkout(
        base_url: impl Into<String>,
        repo: impl Into<String>,
        branch: impl Into<String>,
        checkout_root: Option<PathBuf>,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            repo: repo.into(),
            branch: branch.into(),
            client: reqwest::blocking::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("reqwest client"),
            checkout_root,
        }))
    }

    fn host_header(&self) -> String {
        if self.base_url.contains("127.0.0.1") || self.base_url.contains("localhost") {
            format!("{}.localhost", self.repo)
        } else {
            format!("{}.relay.local", self.repo)
        }
    }

    fn resolve_url(&self, url: &str) -> String {
        if url.starts_with("http://") || url.starts_with("https://") {
            url.to_string()
        } else {
            format!("{}/{}", self.base_url, url.trim_start_matches('/'))
        }
    }

    fn try_checkout_get(&self, url: &str) -> Option<String> {
        if url.starts_with("http://") || url.starts_with("https://") {
            return None;
        }
        let root = self.checkout_root.as_ref()?;
        let rel = url.split('?').next()?.trim_start_matches('/');
        if rel.is_empty() || rel.contains("..") {
            return None;
        }
        let path = root.join(rel);
        if !path.is_file() {
            return None;
        }
        let root_canon = root.canonicalize().ok()?;
        let file_canon = path.canonicalize().ok()?;
        if !file_canon.starts_with(&root_canon) {
            return None;
        }
        let bytes = std::fs::read(&path).ok()?;
        // Text and subtitle/reel paths; binary media falls through to gateway IPFS GET.
        String::from_utf8(bytes).ok()
    }
}

impl Host for RelayHttpHost {
    fn fetch(&self, url: &str, options: Option<&Value>) -> Result<Value, String> {
        let method = options
            .and_then(|o| match o {
                Value::Object(m) => m.get("method"),
                _ => None,
            })
            .and_then(|v| match v {
                Value::String(s) => Some(s.to_uppercase()),
                _ => None,
            })
            .unwrap_or_else(|| "GET".to_string());

        if method == "GET" {
            if let Some(text) = self.try_checkout_get(url) {
                tracing::debug!(url, "fetch from checkout overlay");
                return Ok(host_fetch_object(true, 200.0, text, None));
            }
        }

        let resolved = self.resolve_url(url);
        tracing::debug!(method = %method, url = %resolved, "fetch");

        let mut req = self.client.request(
            reqwest::Method::from_bytes(method.as_bytes()).unwrap_or(reqwest::Method::GET),
            &resolved,
        );
        req = req
            .header("Host", self.host_header())
            .header("X-Relay-Branch", &self.branch);

        if method == "PUT" {
            let body = options
                .and_then(|o| match o {
                    Value::Object(m) => m.get("body"),
                    _ => None,
                })
                .and_then(|v| match v {
                    Value::String(s) => Some(s.clone()),
                    _ => None,
                })
                .unwrap_or_default();
            req = req.body(body);
        }

        match req.send() {
            Ok(resp) => {
                let status = resp.status().as_u16() as f64;
                let ok = resp.status().is_success();
                let text = resp.text().unwrap_or_default();
                Ok(host_fetch_object(ok, status, text, None))
            }
            Err(e) => Ok(host_fetch_object(false, 0.0, String::new(), Some(e.to_string()))),
        }
    }

    fn navigate(&self, path: &str) -> Result<(), String> {
        tracing::info!(path, "navigate");
        Ok(())
    }

    fn create_element(
        &self,
        tag: &str,
        props: &Value,
        children: &[Value],
    ) -> Result<Value, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("type".to_string(), Value::String("element".to_string()));
        m.insert("tag".to_string(), Value::String(tag.to_string()));
        m.insert("props".to_string(), props.clone());
        m.insert("children".to_string(), Value::Array(children.to_vec()));
        Ok(Value::Object(m))
    }

    fn load_module(&self, path: &str) -> Result<Value, String> {
        Err(format!(
            "load_module({path}) — preload via --repo import scan or ModuleMapHost"
        ))
    }

    fn log(&self, level: &str, message: &str) {
        tracing::debug!(level, message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn checkout_overlay_serves_local_readme() {
        let dir = tempdir().unwrap();
        let readme = dir.path().join("FanMixes/demo/README.md");
        std::fs::create_dir_all(readme.parent().unwrap()).unwrap();
        let mut f = std::fs::File::create(&readme).unwrap();
        writeln!(f, "# Local readme").unwrap();

        let host = RelayHttpHost::with_checkout(
            "http://127.0.0.1:9",
            "fan-edits",
            "main",
            Some(dir.path().to_path_buf()),
        );
        let out = host.borrow().fetch("/FanMixes/demo/README.md", None).unwrap();
        match out {
            Value::Object(m) => {
                let ok = m.get("ok").and_then(|v| match v {
                    Value::Bool(b) => Some(*b),
                    _ => None,
                });
                assert_eq!(ok, Some(true));
                let text = m.get("text").and_then(|v| match v {
                    Value::String(s) => Some(s.as_str()),
                    _ => None,
                });
                assert!(text.unwrap_or("").contains("Local readme"));
            }
            _ => panic!("expected object"),
        }
    }
}
