//! Two reciprocal triangles sharing the visible state 0. Fluxes are expected
//! edge traversals per excursion. All exit rates are one per model time unit.
//! See docs/research/excursion-activity/RESULT.md for the all-order bound.

use physis_core::assumption::DomainOfValidity;
use physis_core::claim::{Claim, ClaimClass, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
use physis_core::ParameterOrigin;
use physis_model::World;
use physis_numeric::Ratio;

use crate::framework::Theory;

const ONE: Ratio = Ratio::int(1);
const ZERO: Ratio = Ratio::int(0);
const SPECS: &[KnobSpec] = &[
    KnobSpec {
        name: "second_mean_jumps",
        doc: "Mean number of internal jumps per excursion on the second triangle.",
        layer: LayerId::Statistical,
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::UInt { min: 1, max: 9999 },
    },
    KnobSpec {
        name: "second_winding_quarters",
        doc: "Signed mean winding per second-branch excursion in quarters of a cycle.",
        layer: LayerId::Statistical,
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Int { min: -2, max: 2 },
    },
    KnobSpec {
        name: "first_percent",
        doc: "Percentage of departures entering the first triangle, whose mean is 1 and winding is 1/2.",
        layer: LayerId::Statistical,
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::UInt { min: 1, max: 99 },
    },
];

const POSITIVE: &str = "excursion.positive-floor";
const ATTAINED: &str = "excursion.floor-attained";
const MEAN_MATCH: &str = "excursion.triangle-mean";

/// The default is the five-state counterexample with a rare driven branch.
#[derive(Clone)]
pub struct ExcursionNetwork {
    second_mean_jumps: u64,
    second_winding_quarters: i64,
    first_percent: u64,
}

impl Default for ExcursionNetwork {
    fn default() -> Self {
        Self {
            second_mean_jumps: 9999,
            second_winding_quarters: 0,
            first_percent: 1,
        }
    }
}

struct Observations {
    immediate_return: Ratio,
    two_jump_tail: Ratio,
    hidden_visits: Ratio,
    entropy: f64,
    attains_floor: bool,
}

impl Observations {
    fn deficit(&self) -> Ratio {
        ONE - self.immediate_return * self.hidden_visits
    }

    // Only the displayed rate uses floating point. Every verdict uses exact ratios.
    fn floor_display(&self) -> f64 {
        if self.deficit() <= ZERO {
            return 0.0;
        }
        let d = self.deficit().to_f64().sqrt();
        let n = (self.hidden_visits - ONE).to_f64();
        d * (4.0 * d.atanh() + 2.0 * (d / n).atanh())
    }

    fn mean_matches(&self) -> bool {
        self.hidden_visits * (ONE - self.two_jump_tail) == Ratio::int(2) - self.immediate_return
    }

    fn summary(&self) -> String {
        format!(
            "q0={} P(N>=2)={} E[N]={} excursions/time={} entropy/excursion~{:.8} floor/excursion~{:.8}",
            self.immediate_return, self.two_jump_tail, self.hidden_visits - ONE,
            ONE / (ONE + self.hidden_visits), self.entropy, self.floor_display()
        )
    }
}

impl ExcursionNetwork {
    fn fluxes(&self) -> [[Ratio; 5]; 5] {
        let weight = Ratio::new(self.first_percent.into(), 100);
        let mut flux = [[ZERO; 5]; 5];
        for (left, mass, mean, winding) in [
            (1, weight, ONE, Ratio::new(1, 2)),
            (
                3,
                ONE - weight,
                Ratio::int(self.second_mean_jumps.into()),
                Ratio::new(self.second_winding_quarters.into(), 4),
            ),
        ] {
            let right = left + 1;
            let half = mass / Ratio::int(2);
            flux[0][left] = half * (ONE + winding);
            flux[right][0] = flux[0][left];
            flux[left][0] = half * (ONE - winding);
            flux[0][right] = flux[left][0];
            flux[left][right] = half * (mean + winding);
            flux[right][left] = half * (mean - winding);
        }
        flux
    }

    fn observe(&self) -> Observations {
        let flux = self.fluxes();
        let visits = flux.map(|row| row.into_iter().fold(ZERO, |a, b| a + b));
        let hidden_visits = visits[1..].iter().copied().fold(ZERO, |a, b| a + b);
        let mut immediate_return = ZERO;
        let mut two_jump_tail = ZERO;
        let mut entropy = 0.0;
        for i in 1..5 {
            immediate_return = immediate_return + flux[0][i] * flux[i][0] / visits[i];
            for j in 1..5 {
                two_jump_tail = two_jump_tail
                    + flux[0][i] * flux[i][j] / visits[i] * (ONE - flux[j][0] / visits[j]);
            }
            for (j, &forward) in flux[i][..i].iter().enumerate() {
                if forward > ZERO {
                    let reverse = flux[j][i];
                    entropy += (forward - reverse).to_f64() * (forward / reverse).to_f64().ln();
                }
            }
        }
        // On these triangles each hidden row has exactly one internal neighbour,
        // so reducing the internal edge matrix to its rows loses no information.
        // The driven first triangle precludes attaining a zero floor.
        let attains_floor = immediate_return * hidden_visits < ONE
            && (1..5).all(|i| {
                let entrance = flux[0][i] / visits[i];
                let exit = flux[i][0] / visits[i];
                (entrance + exit) * hidden_visits == Ratio::int(2)
                    && entrance * exit * hidden_visits == immediate_return
            });
        Observations {
            immediate_return,
            two_jump_tail,
            hidden_visits,
            entropy,
            attains_floor,
        }
    }
}

impl Knobbed for ExcursionNetwork {
    fn specs(&self) -> &'static [KnobSpec] {
        SPECS
    }

    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        match name {
            "second_mean_jumps" => Ok(KnobValue::UInt(self.second_mean_jumps)),
            "second_winding_quarters" => Ok(KnobValue::Int(self.second_winding_quarters)),
            "first_percent" => Ok(KnobValue::UInt(self.first_percent)),
            _ => Err(CoreError::UnknownKnob { name: name.into() }),
        }
    }

    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError> {
        let old = self.get(name)?;
        self.spec(name)?.domain.check(name, &value)?;
        match (name, value) {
            ("second_mean_jumps", KnobValue::UInt(v)) => self.second_mean_jumps = v,
            ("second_winding_quarters", KnobValue::Int(v)) => self.second_winding_quarters = v,
            ("first_percent", KnobValue::UInt(v)) => self.first_percent = v,
            _ => unreachable!("validated against the knob spec"),
        }
        Ok(old)
    }
}

impl Theory for ExcursionNetwork {
    fn id(&self) -> &'static str {
        "excursion-counts"
    }
    fn name(&self) -> &'static str {
        "Hidden-jump excursion counts"
    }
    fn summary(&self) -> &'static str {
        "Stationary jump counts on two triangles sharing one visible state."
    }
    fn fork(&self) -> Box<dyn Theory> {
        Box::new(self.clone())
    }
    fn world(&self) -> Option<World> {
        None
    }
    fn note(&self) -> String {
        self.observe().summary()
    }

    fn claims(&self) -> Vec<Claim> {
        let domain = DomainOfValidity::new(
            vec!["finite stationary reciprocal CTMC, singleton on state".into()],
            vec!["two triangles; all physical jumps counted; complete excursions".into()],
            "Unit exit rates. Entropy per excursion in units of k_B; natural logarithms.",
        );
        [
            (POSITIVE, "The immediate-return fraction and mean hidden jump count force positive entropy production."),
            (ATTAINED, "This network attains the least entropy production compatible with its immediate-return fraction and mean hidden jump count."),
            (MEAN_MATCH, "The first two count tails predict the actual mean hidden jump count via the triangle recurrence."),
        ].into_iter().map(|(id, statement)| {
            Claim::new(id, statement, LayerId::Statistical, ClaimClass::ModelInternal)
                .with_domain(domain.clone())
        }).collect()
    }

    fn evaluate(&self, claim: &Claim) -> Verdict {
        let counts = self.observe();
        let holds = match claim.id_str() {
            POSITIVE => counts.deficit() > ZERO,
            ATTAINED => counts.attains_floor,
            MEAN_MATCH => counts.mean_matches(),
            _ => return Verdict::inapplicable(claim, "claim outside the excursion model"),
        };
        if holds {
            Verdict::holds(claim, counts.summary())
        } else {
            Verdict::fails(claim, counts.summary())
        }
    }
}
