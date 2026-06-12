//! Relay desktop client library (verify harness + shared glue).

pub mod embedded_server;
pub mod git_mirror;
pub mod http_host;
pub mod verify;

pub use verify::{VerifyOptions, verify_local_repo};
