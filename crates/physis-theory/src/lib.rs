//! # physis-theory
//!
//! Theories as first-class objects. A theory is a bundle of knobs, a
//! projection into a [`physis_model::World`], and a list of claims it
//! is willing to be judged on.
//!
//! The first lab is `critique::string_critique`: superstring constructions,
//! the bosonic string, the Standard Model, GR, and an observer-geometry
//! scaffold that encodes the *shape* of landscape critiques (including
//! those associated with Eric Weinstein) without pretending to implement
//! Geometric Unity.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod claims;
pub mod critique;
pub mod framework;
pub mod geometry;
pub mod relativity;
pub mod standard_model;
pub mod strings;

pub use critique::{string_critique, ExperimentReport, TheoryReport, VerdictDiff};
pub use framework::Theory;
pub use geometry::ObserverGeometry;
pub use relativity::GeneralRelativity;
pub use standard_model::StandardModel;
pub use strings::{StringKind, StringTheory};
