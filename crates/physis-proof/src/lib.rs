//! # physis-proof
//!
//! Proof *obligations* and untrusted artifacts. This crate does not implement
//! physics and cannot mint [`physis_core`] claims as machine-proved.
//!
//! Trusted side: [`Challenge::generate`] from a [`physis_core::FormalClaim`].
//! Fields are private; JSON cannot mint a challenge. Untrusted side:
//! [`UntrustedProof`]. The verifier (a different crate) is the only place a
//! receipt can be minted.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod artifact;
pub mod catalog;
pub mod challenge;
pub mod expand;
pub mod expr;
pub mod parse;
pub mod physlib;

pub use artifact::{scan_lean_source, ScanReport, UntrustedProof};
pub use catalog::{lookup, lookup_matching, IdentitySpec, CATALOG};
pub use challenge::Challenge;
pub use expand::{identity_is_zero, Poly};
pub use expr::Expr;
pub use parse::parse_expr;
pub use physlib::{
    compact_lean_type, extract_theorems, source_matches_challenge, ExtractedTheorem, PHYSLIB_SOURCE,
};
