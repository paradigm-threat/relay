//! Maintain bare git mirror under ~/.relay/desktop-repos for embedded gateway.

use anyhow::{Context, Result};
use git2::Repository;
use std::path::{Path, PathBuf};

pub struct GitMirror {
    pub bare_path: PathBuf,
    pub repo_name: String,
    pub branch: String,
}

impl GitMirror {
    pub fn ensure(checkout: &Path) -> Result<Self> {
        ensure_is_git_repo(checkout)?;
        let repo = Repository::open(checkout).context("open checkout")?;
        let name = checkout
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("repo")
            .to_string();
        let branch = current_branch(&repo).unwrap_or_else(|| "main".to_string());

        let mirror_root = mirror_root_dir();
        std::fs::create_dir_all(&mirror_root)?;
        let bare_path = mirror_root.join(format!("{name}.git"));

        if !bare_path.exists() {
            Repository::init_bare(&bare_path).context("init bare mirror")?;
        }

        let mirror = Self {
            bare_path,
            repo_name: name,
            branch: branch.clone(),
        };
        mirror.sync_from_checkout(checkout)?;
        Ok(mirror)
    }

    pub fn sync_from_checkout(&self, checkout: &Path) -> Result<()> {
        let refspec = format!("HEAD:refs/heads/{}", self.branch);
        let bare = self.bare_path.to_string_lossy();
        let status = std::process::Command::new("git")
            .current_dir(checkout)
            .args(["push", "--force", bare.as_ref(), &refspec])
            .status()
            .context("git push to bare mirror")?;
        if !status.success() {
            anyhow::bail!("git push to bare mirror failed with {status}");
        }
        Ok(())
    }

    /// Parent directory containing `{name}.git` bare repos (RELAY_REPO_PATH for gateway).
    pub fn repo_path_for_gateway(&self) -> PathBuf {
        self.bare_path.parent().unwrap().to_path_buf()
    }
}

fn mirror_root_dir() -> PathBuf {
    home_dir().join(".relay/desktop-repos")
}

fn home_dir() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp"))
}

fn current_branch(repo: &Repository) -> Option<String> {
    let head = repo.head().ok()?;
    head.shorthand().map(|s| s.to_string())
}

pub fn ensure_is_git_repo(checkout: &Path) -> Result<()> {
    if !checkout.join(".git").exists() {
        anyhow::bail!(
            "{} is not a git repository (missing .git); Relay desktop requires git repos only",
            checkout.display()
        );
    }
    Ok(())
}

pub fn warn_if_dirty(checkout: &Path) -> Result<()> {
    let repo = Repository::open(checkout)?;
    let mut opts = git2::StatusOptions::new();
    opts.include_untracked(true);
    let statuses = repo.statuses(Some(&mut opts))?;
    if !statuses.is_empty() {
        tracing::warn!(
            count = statuses.len(),
            "checkout has uncommitted changes; embedded gateway serves committed HEAD only — commit hook files before testing"
        );
    }
    Ok(())
}
