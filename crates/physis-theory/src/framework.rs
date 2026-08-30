//! The `Theory` trait: knobs in, world and verdicts out.

use physis_core::claim::{Claim, Verdict};
use physis_core::knob::Knobbed;
use physis_ir::TheoryPackage;
use physis_model::World;

/// A falsifiable (inside the model) bundle of knobs, world, and claims.
pub trait Theory: Knobbed + Send + Sync {
    /// Stable id (`type-iib`, `standard-model`, …).
    fn id(&self) -> &'static str;
    /// Human title.
    fn name(&self) -> &'static str;
    /// What this object is, and what it is not.
    fn summary(&self) -> &'static str;
    /// Projection into the physics-shaped mechanical layers, when the object
    /// has one. Non-physics domains (e.g. computation) have no spacetime,
    /// gauge, or spectrum and return `None` rather than borrowing a placeholder.
    fn world(&self) -> Option<World>;
    /// One-line note for reports. Defaults to the world's note; domains without
    /// a world override this to describe themselves.
    fn note(&self) -> String {
        self.world().map(|w| w.note).unwrap_or_default()
    }
    /// Claims this theory is willing to be judged on.
    fn claims(&self) -> Vec<Claim>;
    /// Evaluate one claim against current knobs.
    fn evaluate(&self, claim: &Claim) -> Verdict;
    /// Evaluate every claim.
    fn evaluate_all(&self) -> Vec<(Claim, Verdict)> {
        self.claims()
            .into_iter()
            .map(|c| {
                let v = self.evaluate(&c);
                (c, v)
            })
            .collect()
    }
    /// Constrained structural forks of the encoding (IR package / netlist),
    /// not knob probes. Each pair is `(label, mutant)`. The caller evaluates
    /// the mutant and must not install it as trusted state. Default: none.
    fn structural_mutations(&self) -> Vec<(String, Box<dyn Theory>)> {
        Vec::new()
    }
    /// Live IR package, when this theory is a parsed encoding (NAND
    /// netlist, lattice stencil, …). Default: none. Not a kernel proof
    /// and not semantic review.
    fn ir_package(&self) -> Option<TheoryPackage> {
        None
    }
    /// Reconstruct this encoding from an IR package. The result is a
    /// fork candidate, not trusted lab state. Default: no IR package.
    fn reparse_package(&self, pkg: &TheoryPackage) -> Result<Box<dyn Theory>, String> {
        let _ = pkg;
        Err("no IR package".into())
    }
}
