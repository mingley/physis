//! Facade crate: re-exports the workspace and houses the `physis` CLI.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub use physis_agent::{Command, Journal, Lab, Response};
pub use physis_core as core;
pub use physis_model as model;
pub use physis_theory as theory;
