//! The `Theory` trait: knobs in, world and verdicts out.

use physis_core::claim::{Claim, Verdict};
use physis_core::knob::Knobbed;
use physis_model::World;

/// A falsifiable (inside the model) bundle of knobs, world, and claims.
pub trait Theory: Knobbed + Send + Sync {
    /// Stable id (`type-iib`, `standard-model`, …).
    fn id(&self) -> &'static str;
    /// Human title.
    fn name(&self) -> &'static str;
    /// What this object is, and what it is not.
    fn summary(&self) -> &'static str;
    /// Projection into mechanical layers.
    fn world(&self) -> World;
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
}
