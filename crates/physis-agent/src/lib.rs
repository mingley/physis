//! # physis-agent
//!
//! The lab: a bag of theories, an append-only journal, and a protocol
//! agents use to observe, turn knobs, and run experiments.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod journal;
pub mod lab;
pub mod protocol;
pub mod replay;

pub use journal::{Journal, JournalEvent};
pub use lab::Lab;
pub use protocol::{Command, Response};
pub use replay::{replay_journal, ReplayReport, ReplayStep};
