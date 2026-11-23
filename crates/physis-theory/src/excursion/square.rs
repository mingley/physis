//! A four-state cycle with uniform stationary mass and unit exit rates.
//! Two independent jump biases tune dissipation at a fixed observed clock.
use super::entropy::one_jump_absent_minimum;
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

const POSITIVE: &str = "excursion.square-forces-dissipation";
const MINIMUM: &str = "excursion.square-attains-minimum";

const fn bias(name: &'static str, doc: &'static str) -> KnobSpec {
    KnobSpec {
        name,
        doc,
        layer: LayerId::Statistical,
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Int { min: -11, max: 11 },
    }
}

const BIASES: [KnobSpec; 2] = [
    bias("entry_bias", "Offset v from one half on entry to B+, in units of 1/24. C returns to B+ with probability 1/2-v."),
    bias("continuation_bias", "Offset u from one half for B+ to continue to C, in units of 1/24. B- continues with probability 1/2-u."),
];

/// Unit-rate square 0, B+, C, B-. A change in either bias preserves all
/// four occupancies. Their product determines the complete observed jump clock.
#[derive(Clone, Default)]
pub struct ExcursionSquare {
    biases: [i64; 2],
}

impl ExcursionSquare {
    fn index(name: &str) -> Result<usize, CoreError> {
        BIASES
            .iter()
            .position(|s| s.name == name)
            .ok_or_else(|| CoreError::UnknownKnob { name: name.into() })
    }

    fn counts(&self) -> (Ratio, Ratio, Ratio) {
        let [v, u] = self.biases.map(|i| Ratio::new(i.into(), 24));
        let m = Ratio::new(1, 2) + Ratio::int(2) * u * v;
        (m, u, v)
    }
}

impl Knobbed for ExcursionSquare {
    fn specs(&self) -> &'static [KnobSpec] {
        &BIASES
    }
    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        Ok(KnobValue::Int(self.biases[Self::index(name)?]))
    }
    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError> {
        let i = Self::index(name)?;
        BIASES[i].domain.check(name, &value)?;
        let KnobValue::Int(v) = value else {
            unreachable!("validated signed bias")
        };
        Ok(KnobValue::Int(std::mem::replace(&mut self.biases[i], v)))
    }
}

impl Theory for ExcursionSquare {
    fn id(&self) -> &'static str {
        "excursion-square"
    }
    fn name(&self) -> &'static str {
        "Hidden-jump counter on a four-state cycle"
    }
    fn summary(&self) -> &'static str {
        "Sharp dissipation from a missing one-jump excursion and the mean count."
    }
    fn world(&self) -> Option<World> {
        None
    }
    fn fork(&self) -> Box<dyn Theory> {
        Box::new(self.clone())
    }
    fn note(&self) -> String {
        let (m, u, v) = self.counts();
        let current = u + v;
        let entropy =
            4.0 * current.to_f64() * ((2.0 * u.to_f64()).atanh() + (2.0 * v.to_f64()).atanh());
        let floor = one_jump_absent_minimum(m, Ratio::int(2)).expect("square counts are physical");
        format!("P(N>=1)=P(N>=2)={m} E[N]=2 paired_tail_ratio={} winding={current} entropy/excursion~{entropy:.8} minimum/excursion~{floor:.8}", ONE - m)
    }
    fn claims(&self) -> Vec<Claim> {
        let domain = DomainOfValidity::new(
            vec!["finite stationary irreducible CTMC, reciprocal edges, one effective channel per ordered pair".into()],
            vec!["one visible state, all physical jumps detected, exact population counts".into()],
            "Unit-rate four-state square. Competing networks may have arbitrary finite hidden order. Entropy in k_B per excursion.",
        );
        vec![
            Claim::new(POSITIVE, "The two count tails and the mean force strictly positive entropy production in every compatible network.", LayerId::Statistical, ClaimClass::ModelInternal).with_domain(domain.clone()),
            Claim::new(MINIMUM, "This square attains the least entropy per excursion compatible with its two count tails and mean.", LayerId::Statistical, ClaimClass::ModelInternal).with_domain(domain),
        ]
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        let (m, u, v) = self.counts();
        let holds = match claim.id_str() {
            POSITIVE => Ratio::int(2) * m > ONE,
            // Detailed balance is u=-v. For a positive minimum equality in
            // the boundary Cauchy inequality requires equal biases u=v.
            MINIMUM => u + v == ZERO || u == v,
            _ => return Verdict::inapplicable(claim, "claim outside the excursion square"),
        };
        let summary = self.note();
        if holds {
            Verdict::holds(claim, summary)
        } else {
            Verdict::fails(claim, summary)
        }
    }
}
