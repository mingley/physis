//! Pairwise equilibrium fits and joint dissipation detection on a four-state ring.

use super::{ONE, ZERO};
use crate::Theory;
use physis_core::assumption::DomainOfValidity;
use physis_core::claim::{Claim, ClaimClass, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
use physis_core::ParameterOrigin;
use physis_model::World;
use physis_numeric::Ratio;

const CLAIM: &str = "excursion.only-joint-detects";
const WEIGHTS: [KnobSpec; 3] = [
    weight("clockwise", "Weight of a clockwise jump from each site."),
    weight(
        "anticlockwise",
        "Weight of an anticlockwise jump from each site.",
    ),
    weight("opposite", "Weight of a jump to the opposite site."),
];

const fn weight(name: &'static str, doc: &'static str) -> KnobSpec {
    KnobSpec {
        name,
        doc,
        layer: LayerId::Statistical,
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::UInt { min: 1, max: 8 },
    }
}

/// Rate-one ring with one visible site and three hidden sites. Uniform
/// stationarity fixes the mean number of internal jumps per excursion at two.
#[derive(Clone)]
pub struct ExcursionRing {
    weights: [u64; 3],
}

impl Default for ExcursionRing {
    fn default() -> Self {
        Self { weights: [1; 3] }
    }
}

impl ExcursionRing {
    fn index(name: &str) -> Result<usize, CoreError> {
        WEIGHTS
            .iter()
            .position(|w| w.name == name)
            .ok_or_else(|| CoreError::UnknownKnob { name: name.into() })
    }

    fn observation(&self) -> (bool, String) {
        let [a, b, c] = self.weights.map(|w| Ratio::int(w.into()));
        let total = a + b + c;
        // Enumerate closed walks of length two and three from site zero.
        // With no self-jumps these are first returns and account for N=0,1.
        let q = (Ratio::int(2) * a * b + c.pow(2)) / total.pow(2);
        let m = ONE - q;
        let r = m - Ratio::int(3) * c * (a.pow(2) + b.pow(2)) / total.pow(3);
        let gap = (ONE + m).pow(2) - Ratio::int(5) * (ONE - r);
        let pairs_admit_equilibrium =
            m.pow(2) <= r && Ratio::int(3) * q >= ONE && Ratio::int(2) * r <= ONE;
        let summary = format!("P(N>=1)={m} P(N>=2)={r} E[N]=2 joint_gap={gap} each_pair_admits_equilibrium={pairs_admit_equilibrium}");
        (pairs_admit_equilibrium && gap > ZERO, summary)
    }
}

impl Knobbed for ExcursionRing {
    fn specs(&self) -> &'static [KnobSpec] {
        &WEIGHTS
    }

    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        Ok(KnobValue::UInt(self.weights[Self::index(name)?]))
    }

    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError> {
        let index = Self::index(name)?;
        WEIGHTS[index].domain.check(name, &value)?;
        let KnobValue::UInt(w) = value else {
            unreachable!("validated against the weight domain")
        };
        Ok(KnobValue::UInt(std::mem::replace(
            &mut self.weights[index],
            w,
        )))
    }
}

impl Theory for ExcursionRing {
    fn id(&self) -> &'static str {
        "excursion-ring"
    }
    fn name(&self) -> &'static str {
        "Hidden-jump counter on a four-state ring"
    }
    fn summary(&self) -> &'static str {
        "Three count statistics and the conditions for pairwise reversible fits."
    }
    fn world(&self) -> Option<World> {
        None
    }
    fn note(&self) -> String {
        self.observation().1
    }
    fn fork(&self) -> Box<dyn Theory> {
        Box::new(self.clone())
    }

    fn claims(&self) -> Vec<Claim> {
        let domain = DomainOfValidity::new(
            vec!["stationary reciprocal CTMC, one visible state, every physical jump counted".into()],
            vec!["four-state circulant generator with unit exit rates; competing fits may have any finite hidden order".into()],
            "Complete excursions. The joint test is (1+m)^2 > (1+2n)(1-r), with m=P(N>=1), r=P(N>=2), n=E[N].",
        );
        vec![Claim::new(CLAIM, "Every pair of count statistics admits equilibrium and the joint inequality forces positive entropy production.", LayerId::Statistical, ClaimClass::ModelInternal).with_domain(domain)]
    }

    fn evaluate(&self, claim: &Claim) -> Verdict {
        if claim.id_str() != CLAIM {
            return Verdict::inapplicable(claim, "claim outside the excursion ring");
        }
        let (holds, summary) = self.observation();
        if holds {
            Verdict::holds(claim, summary)
        } else {
            Verdict::fails(claim, summary)
        }
    }
}
