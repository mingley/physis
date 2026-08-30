//! Observer-geometry: a scaffold for *unique-geometry* unification programs.
//!
//! This is **not** Geometric Unity, not a quantization of it, and not a
//! claim that Eric Weinstein's program is correct. It exists so that
//! agents can compare a *shape of theory* — "start from geometry, demand
//! uniqueness, try to derive the gauge group" — against string constructions
//! whose distinctive failure mode (in this lab) is a landscape of vacua.
//!
//! The total dimension is not a magic number: it is `observed_dim + fibre_dim`.
//! The default `fibre_dim = 10` is the *smallest* fibre that can host the
//! conjectured Spin(10) gauge group (which acts on a 10-dimensional space), so
//! the default total `14 = 4 + 10` is a toy constraint — the minimal geometric
//! room for the assignment — rather than an unexplained choice. It is still a
//! scaffold, not a derivation of Geometric Unity.
//!
//! Spin(10) on a 10-fibre lives on the IR package of `observer-geometry`.
//! A missing Spin(10) (`add-missing-spin10`) is a package mutation, not the
//! `unique_vacuum` or `derive_gauge` knob: `empirical.sm-gauge` fails because
//! the fibre has no assigned group that contains SM, while uniqueness stays
//! the program axiom. That is not Geometric Unity.

use physis_core::assumption::DomainOfValidity;
use physis_core::claim::{Claim, ClaimClass, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
use physis_core::ParameterOrigin;
use physis_ir::{apply_mutation, parse_package, render_package, PackageMutation, TheoryPackage};
use physis_model::{GaugeGroup, Manifold, Signature, Spectrum, Topology, World};

use crate::claims;
use crate::framework::Theory;

/// Minimal fibre dimension that can host the conjectured Spin(10) gauge group.
const SPIN10_MIN_FIBRE: u8 = 10;

/// Live assignment on the `observer-geometry` package.
const SPIN10_EQ: &str = "Spin(10) on 10-fibre";
/// Incomplete encoding: the fibre has no Spin(10).
const MISSING_EQ: &str = "missing Spin(10)";

const SPECS: &[KnobSpec] = &[
    KnobSpec {
        name: "fibre_dim",
        layer: LayerId::Spacetime,
        doc: "Dimension of the internal fibre over observed spacetime. Total dimension is observed_dim + fibre_dim. Default 10 is the minimal fibre that can carry Spin(10). A missing Spin(10) is not this knob: add-missing-spin10 is an IR mutation.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::UInt { min: 0, max: 22 },
    },
    KnobSpec {
        name: "observed_dim",
        layer: LayerId::Spacetime,
        doc: "Observed spacetime dimension. Empirical target: 4.",
        origin: ParameterOrigin::Measured,
        domain: KnobDomain::UInt { min: 1, max: 26 },
    },
    KnobSpec {
        name: "derive_gauge",
        layer: LayerId::Interaction,
        doc: "If true, pretends the gauge group is an output (assigned Spin(10) as a conjecture, not a proof). A missing Spin(10) is not this knob: add-missing-spin10 is an IR mutation.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Bool,
    },
    KnobSpec {
        name: "unique_vacuum",
        layer: LayerId::Mathematical,
        doc: "Program-level demand: the geometry selects one vacuum. This is an axiom of the program, not a theorem. A missing Spin(10) is not this knob: add-missing-spin10 is an IR mutation.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Bool,
    },
];

/// Unique-geometry unification scaffold.
///
/// Spin(10) on a 10-fibre lives on the IR package. A missing Spin(10)
/// (`add-missing-spin10`) is a package mutation, not a knob.
/// `unique_vacuum` and `derive_gauge` stay chosen knobs.
#[derive(Clone, Debug, PartialEq)]
pub struct ObserverGeometry {
    fibre_dim: u8,
    observed_dim: u8,
    derive_gauge: bool,
    unique_vacuum: bool,
    /// Whether the encoding is missing the Spin(10) assignment.
    missing_spin10: bool,
}

impl Default for ObserverGeometry {
    fn default() -> Self {
        Self {
            fibre_dim: SPIN10_MIN_FIBRE,
            observed_dim: 4,
            derive_gauge: true,
            unique_vacuum: true,
            missing_spin10: false,
        }
    }
}

impl ObserverGeometry {
    /// Total geometric dimension: observed spacetime plus the internal fibre.
    fn total_dim(&self) -> u8 {
        self.observed_dim.saturating_add(self.fibre_dim)
    }

    /// Whether the (conjectural) Spin(10) assignment has geometric room: the
    /// fibre must be at least 10-dimensional for Spin(10) to act on it.
    fn fibre_can_host_spin10(&self) -> bool {
        self.fibre_dim >= SPIN10_MIN_FIBRE
    }

    fn build_world(&self) -> World {
        let total = self.total_dim();
        let extra = self.fibre_dim;
        let space = total.saturating_sub(1);
        World {
            spacetime: Manifold {
                dim: total,
                signature: Signature { time: 1, space },
                compact_extra: extra,
                compact_radius_planck: if extra == 0 { 0.0 } else { 1.0 },
                topology: Topology::Unspecified,
                convention: physis_model::SignConvention::MostlyPlus,
            },
            gauge: self.gauge(),
            spectrum: Spectrum::standard_model_plus_graviton(),
            has_gravity: true,
            supersymmetric: false,
            free_parameter_count: if self.unique_vacuum { 1 } else { 40 },
            landscape_log10: if self.unique_vacuum { 0.0 } else { 12.0 },
            note: format!(
                "observer-geometry D={}=({}+{}) derive_gauge={} unique={}",
                total, self.observed_dim, self.fibre_dim, self.derive_gauge, self.unique_vacuum
            ),
        }
    }

    fn gauge(&self) -> GaugeGroup {
        if self.missing_spin10 {
            GaugeGroup::trivial()
        } else if self.derive_gauge {
            GaugeGroup::spin10()
        } else {
            GaugeGroup::standard_model()
        }
    }

    /// IR package for this assignment. Equations are `Spin(10) on 10-fibre`
    /// and, when forked, `missing Spin(10)`. `unique_vacuum` stays on the struct.
    pub fn package(&self) -> TheoryPackage {
        let mut equations = vec![SPIN10_EQ.to_string()];
        if self.missing_spin10 {
            equations.push(MISSING_EQ.to_string());
        }
        TheoryPackage {
            id: self.id().to_string(),
            name: self.name().to_string(),
            parameters: vec![],
            assumptions: vec!["spin10-on-10-fibre".into()],
            equations,
            claims: vec![physis_ir::ClaimDecl {
                id: claims::SM_GAUGE.into(),
                statement: "The Standard Model gauge group is derived, not postulated.".into(),
                layer: "interaction".into(),
                class: "conjecture".into(),
            }],
            lean_ref: None,
        }
    }

    /// Load a Spin(10) assignment from a package. Knobs default; overlay them
    /// from a live observer-geometry object when forking.
    pub fn from_package(pkg: &TheoryPackage) -> Result<Self, String> {
        if pkg.id != "observer-geometry" {
            return Err(format!(
                "observer-geometry package id '{}' is not observer-geometry",
                pkg.id
            ));
        }
        let missing_spin10 = parse_spin10_assignment(pkg)?;
        Ok(Self {
            missing_spin10,
            ..Self::default()
        })
    }

    fn missing_equation() -> String {
        MISSING_EQ.to_string()
    }
}

fn parse_spin10_assignment(pkg: &TheoryPackage) -> Result<bool, String> {
    let mut complete = false;
    let mut missing = false;
    for eq in &pkg.equations {
        match eq.trim() {
            SPIN10_EQ => complete = true,
            MISSING_EQ => missing = true,
            _ => {}
        }
    }
    if !complete {
        return Err(format!(
            "{} package has no Spin(10) on 10-fibre assignment",
            pkg.id
        ));
    }
    Ok(missing)
}

impl Knobbed for ObserverGeometry {
    fn specs(&self) -> &'static [KnobSpec] {
        SPECS
    }

    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        match name {
            "fibre_dim" => Ok(KnobValue::UInt(self.fibre_dim as u64)),
            "observed_dim" => Ok(KnobValue::UInt(self.observed_dim as u64)),
            "derive_gauge" => Ok(KnobValue::Bool(self.derive_gauge)),
            "unique_vacuum" => Ok(KnobValue::Bool(self.unique_vacuum)),
            _ => Err(CoreError::UnknownKnob { name: name.into() }),
        }
    }

    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError> {
        let spec = self.spec(name)?;
        spec.domain.check(name, &value)?;
        let old = self.get(name)?;
        match (name, value) {
            ("fibre_dim", KnobValue::UInt(v)) => self.fibre_dim = v as u8,
            ("observed_dim", KnobValue::UInt(v)) => self.observed_dim = v as u8,
            ("derive_gauge", KnobValue::Bool(v)) => self.derive_gauge = v,
            ("unique_vacuum", KnobValue::Bool(v)) => self.unique_vacuum = v,
            _ => {
                return Err(CoreError::TypeMismatch {
                    name: name.into(),
                    expected: spec.domain.kind_name().into(),
                    got: old.kind_name().into(),
                });
            }
        }
        Ok(old)
    }
}

impl Theory for ObserverGeometry {
    fn id(&self) -> &'static str {
        "observer-geometry"
    }
    fn name(&self) -> &'static str {
        "Observer geometry (scaffold)"
    }
    fn summary(&self) -> &'static str {
        "A typed scaffold for programs that try to derive particle physics from a unique \
         geometric starting point. Encodes the *shape* of landscape critiques. \
         Does not implement Geometric Unity. Gauge 'derivation' is a conjectural assignment \
         (Spin(10)), labelled as such."
    }

    fn world(&self) -> Option<World> {
        Some(self.build_world())
    }

    fn claims(&self) -> Vec<Claim> {
        vec![
            claims::c(
                claims::SPACETIME_STRUCTURE,
                "Geometric dimension and observed dimension are consistent.",
                LayerId::Spacetime,
                ClaimClass::ModelInternal,
            ),
            claims::c(
                claims::CRITICAL_DIMENSION,
                "A worldsheet critical dimension applies.",
                LayerId::Spacetime,
                ClaimClass::Phenomenological,
            ),
            claims::c(
                claims::OBSERVED_4D,
                "Observed spacetime is 3+1.",
                LayerId::Spacetime,
                ClaimClass::Phenomenological,
            ),
            claims::c(
                claims::FERMIONS,
                "Fermions arise from the geometry.",
                LayerId::Particle,
                ClaimClass::Conjecture,
            ),
            claims::c(
                claims::SM_GAUGE,
                "The Standard Model gauge group is derived, not postulated.",
                LayerId::Interaction,
                ClaimClass::Conjecture,
            )
            .with_domain(DomainOfValidity::new(
                vec!["Spin(10) on 10-fibre".into()],
                vec!["conjectural assignment, not a geometric derivation".into()],
                "The assigned group is Spin(10) on a 10-fibre. A missing Spin(10) is a new encoding, not a silent unique_vacuum or derive_gauge knob. Not a kernel proof.",
            )),
            claims::c(
                claims::THREE_GENERATIONS,
                "Three generations are selected by the geometry.",
                LayerId::Particle,
                ClaimClass::OpenProblem,
            ),
            claims::c(
                claims::GRAVITY,
                "Gravity is geometric (metric / Einstein sector).",
                LayerId::Spacetime,
                ClaimClass::Conjecture,
            ),
            claims::c(
                claims::UNIQUE_VACUUM,
                "The construction selects a unique vacuum.",
                LayerId::Mathematical,
                ClaimClass::Conjecture,
            )
            .with_domain(DomainOfValidity::new(
                vec!["unique_vacuum program axiom".into()],
                vec!["uniqueness is demanded by a knob, not derived from a theorem".into()],
                "This is the contrast class for the string landscape, not evidence \
                 that geometry has succeeded. Not Geometric Unity. Not a kernel proof.",
            )),
            claims::c(
                claims::UV_COMPLETION,
                "The construction is a UV completion of gravity plus matter.",
                LayerId::Field,
                ClaimClass::OpenProblem,
            ),
        ]
    }

    fn evaluate(&self, claim: &Claim) -> Verdict {
        match claim.id_str() {
            claims::SPACETIME_STRUCTURE => {
                if self.build_world().spacetime.structurally_ok() {
                    Verdict::holds(claim, "dimension numbers fit")
                } else {
                    Verdict::fails(claim,
                        "spacetime numbers are not internally consistent",
                    )
                    .with_evidence([format!(
                        "observed={} fibre={} total={}",
                        self.observed_dim,
                        self.fibre_dim,
                        self.total_dim()
                    )])
                }
            }
            claims::CRITICAL_DIMENSION => Verdict::inapplicable(
                claim,
                "observer-geometry is not a worldsheet theory; it has no Polyakov conformal anomaly",
            ),
            claims::OBSERVED_4D => {
                if self.observed_dim == 4 {
                    Verdict::holds(claim, "observed_dim = 4")
                } else {
                    Verdict::fails(claim,
                        format!("observed_dim = {}", self.observed_dim),
                    )
                }
            }
            claims::FERMIONS => Verdict::undecidable(claim,
                "fermions are *assumed* in the projected spectrum; they are not derived in this encoding",
            ),
            claims::SM_GAUGE => {
                if !self.derive_gauge {
                    Verdict::fails(claim,
                        "derive_gauge is off: SM is postulated, which is the thing this program wanted to avoid",
                    )
                } else if self.missing_spin10 {
                    Verdict::fails(
                        claim,
                        "missing Spin(10): fibre has no assigned group that contains SM",
                    )
                    .with_evidence([
                        "Spin(10) on 10-fibre is the live encoding; missing Spin(10) is not a unique_vacuum knob"
                            .to_string(),
                    ])
                } else if !self.fibre_can_host_spin10() {
                    // Toy geometric constraint: Spin(10) acts on R^10, so a
                    // fibre smaller than 10 has no room for the assignment.
                    Verdict::fails(claim,
                        format!(
                            "fibre_dim = {} < {} has no geometric room for Spin(10)",
                            self.fibre_dim, SPIN10_MIN_FIBRE
                        ),
                    )
                    .with_evidence([
                        "this is why the default fibre is 10 (hence total 14 = 4 + 10): the minimal carrier of Spin(10)".to_string(),
                    ])
                } else if self.gauge().sm_embed().contains_sm() {
                    let chain = self
                        .gauge()
                        .verified_contains_sm()
                        .unwrap_or_default()
                        .join(" ⊃ ");
                    Verdict::holds(claim,
                        "Spin(10) is assigned as a derived group and does contain SM — assignment, not a proof",
                    )
                    .with_evidence([
                        format!("fibre_dim = {} ≥ {} can host Spin(10)", self.fibre_dim, SPIN10_MIN_FIBRE),
                        format!("verified embedding chain: {chain}"),
                        "replace this assignment with an actual geometric derivation before treating it as a theorem".to_string(),
                    ])
                } else {
                    Verdict::fails(claim,
                        "derived group does not contain SM in this encoding",
                    )
                }
            }
            claims::THREE_GENERATIONS => Verdict::undecidable(claim,
                "generation count is not selected by any encoded geometric rule yet",
            ),
            claims::GRAVITY => Verdict::holds(claim,
                "a metric sector is part of the scaffold; not derived from a theorem here",
            ),
            claims::UNIQUE_VACUUM => {
                if self.unique_vacuum {
                    Verdict::holds(claim,
                        "uniqueness is an axiom of this program (knob unique_vacuum=true), not a computed theorem",
                    )
                    .with_evidence([
                        "this is the contrast class for the string landscape, not evidence that geometry has succeeded",
                    ])
                } else {
                    Verdict::fails(claim,
                        "unique_vacuum knob is off; the program has dropped its distinctive demand",
                    )
                }
            }
            claims::UV_COMPLETION => Verdict::undecidable(claim,
                "no quantum construction is encoded; UV completeness is open",
            ),
            claims::SUSY_CONSTRUCTION | claims::NO_TACHYON | claims::HIDDEN_EXTRA_DIMS => {
                Verdict::inapplicable(claim, "not a string construction")
            }
            _ => Verdict::inapplicable(claim, "claim not made by observer-geometry"),
        }
    }
    fn ir_package(&self) -> Option<TheoryPackage> {
        Some(self.package())
    }
    fn reparse_package(&self, pkg: &TheoryPackage) -> Result<Box<dyn Theory>, String> {
        let parsed = Self::from_package(pkg)?;
        let mut fork = self.clone();
        fork.missing_spin10 = parsed.missing_spin10;
        Ok(Box::new(fork))
    }
    fn structural_mutations(&self) -> Vec<(String, Box<dyn Theory>)> {
        if self.missing_spin10 {
            return Vec::new();
        }
        let src = render_package(&self.package());
        let Ok(pkg) = parse_package(&src) else {
            return Vec::new();
        };
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(Self::missing_equation()),
        );
        if let Ok(parsed) = Self::from_package(&mutated) {
            if parsed.missing_spin10 {
                let mut fork = self.clone();
                fork.missing_spin10 = true;
                return vec![("add-missing-spin10".into(), Box::new(fork))];
            }
        }
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use physis_core::claim::VerdictKind;

    #[test]
    fn observed_dim_is_measured_fibre_is_chosen() {
        let t = ObserverGeometry::default();
        assert_eq!(
            t.spec("observed_dim").unwrap().origin,
            ParameterOrigin::Measured
        );
        assert_eq!(t.spec("fibre_dim").unwrap().origin, ParameterOrigin::Chosen);
        assert_eq!(
            t.spec("unique_vacuum").unwrap().origin,
            ParameterOrigin::Chosen
        );
    }

    #[test]
    fn uniqueness_is_conjectural_hold() {
        let t = ObserverGeometry::default();
        let c = t
            .claims()
            .into_iter()
            .find(|c| c.id_str() == claims::UNIQUE_VACUUM)
            .unwrap();
        let v = t.evaluate(&c);
        assert_eq!(v.kind, VerdictKind::Holds);
        assert_eq!(v.class, ClaimClass::Conjecture);
        assert!(
            !c.domain().is_encoding_wide(),
            "observer-geometry unique-vacuum must name the program axiom: {:?}",
            c.domain()
        );
        assert!(
            c.domain()
                .regimes
                .iter()
                .any(|r| r.contains("unique_vacuum program axiom")),
            "observer-geometry regime: {:?}",
            c.domain()
        );
    }

    #[test]
    fn no_critical_dimension() {
        let t = ObserverGeometry::default();
        let c = t
            .claims()
            .into_iter()
            .find(|c| c.id_str() == claims::CRITICAL_DIMENSION)
            .unwrap();
        assert_eq!(t.evaluate(&c).kind, VerdictKind::Inapplicable);
    }

    fn verdict(t: &ObserverGeometry, id: &str) -> VerdictKind {
        let c = t.claims().into_iter().find(|c| c.id_str() == id).unwrap();
        t.evaluate(&c).kind
    }

    #[test]
    fn total_dimension_is_composed_not_magic() {
        // 14 is not a literal: it is observed (4) + fibre (10).
        let t = ObserverGeometry::default();
        assert_eq!(t.total_dim(), 14);
        assert_eq!(t.observed_dim, 4);
        assert_eq!(t.fibre_dim, 10);
    }

    #[test]
    fn shrinking_the_fibre_below_ten_starves_the_gauge_assignment() {
        // The toy constraint: Spin(10) needs a fibre of at least 10 dimensions.
        let mut t = ObserverGeometry::default();
        assert_eq!(verdict(&t, claims::SM_GAUGE), VerdictKind::Holds);
        t.set("fibre_dim", KnobValue::UInt(9)).unwrap();
        assert_eq!(verdict(&t, claims::SM_GAUGE), VerdictKind::Fails);
        // Restoring the minimal fibre restores the (conjectural) assignment.
        t.set("fibre_dim", KnobValue::UInt(10)).unwrap();
        assert_eq!(verdict(&t, claims::SM_GAUGE), VerdictKind::Holds);
    }

    fn kind(t: &dyn Theory, id: &str) -> VerdictKind {
        let c = t.claims().into_iter().find(|c| c.id_str() == id).unwrap();
        t.evaluate(&c).kind
    }

    #[test]
    fn missing_spin10_is_ir_not_a_knob() {
        assert!(
            ObserverGeometry::default()
                .set("missing_spin10", KnobValue::Bool(true))
                .is_err(),
            "missing Spin(10) is an IR mutation, not a knob"
        );
        assert!(
            ObserverGeometry::default()
                .set("missing-spin10", KnobValue::Bool(true))
                .is_err(),
            "missing-spin10 is not a knob"
        );
        assert!(
            ObserverGeometry::default()
                .set("add-missing-spin10", KnobValue::Bool(true))
                .is_err(),
            "add-missing-spin10 is not a knob"
        );
        let og = ObserverGeometry::default();
        assert!(!og.missing_spin10);
        let src = render_package(&og.package());
        let pkg = parse_package(&src).unwrap();
        assert_eq!(pkg.equations.len(), 1, "live package must stay complete");
        assert_eq!(pkg.equations[0], SPIN10_EQ);
        assert_eq!(
            ObserverGeometry::from_package(&pkg).unwrap(),
            og,
            "IR round-trip must preserve Spin(10) on 10-fibre"
        );
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(ObserverGeometry::missing_equation()),
        );
        let parsed = ObserverGeometry::from_package(&mutated).unwrap();
        assert!(parsed.missing_spin10);
        let mut fork = og.clone();
        fork.missing_spin10 = true;
        assert_eq!(fork.id(), "observer-geometry");
        let gauge = fork.evaluate(
            &fork
                .claims()
                .into_iter()
                .find(|c| c.id_str() == claims::SM_GAUGE)
                .unwrap(),
        );
        assert_eq!(gauge.kind, VerdictKind::Fails);
        assert!(
            !gauge.summary.contains("unique_vacuum")
                && !gauge.summary.contains("derive_gauge")
                && !gauge.summary.contains("fibre_dim"),
            "missing Spin(10) is not a knob: {}",
            gauge.summary
        );
        assert_eq!(kind(&fork, claims::UNIQUE_VACUUM), VerdictKind::Holds);
        assert_eq!(kind(&fork, claims::GRAVITY), VerdictKind::Holds);
        assert_eq!(kind(&fork, claims::OBSERVED_4D), VerdictKind::Holds);
        assert_eq!(kind(&og, claims::SM_GAUGE), VerdictKind::Holds);

        let probes = ObserverGeometry::default().structural_mutations();
        assert!(
            probes
                .iter()
                .any(|(label, _)| label == "add-missing-spin10"),
            "live observer-geometry must offer add-missing-spin10: {:?}",
            probes.iter().map(|(l, _)| l.as_str()).collect::<Vec<_>>()
        );
        let probe = probes
            .iter()
            .find(|(label, _)| label == "add-missing-spin10")
            .expect("add-missing-spin10");
        assert_eq!(kind(probe.1.as_ref(), claims::SM_GAUGE), VerdictKind::Fails);
        assert_eq!(
            kind(probe.1.as_ref(), claims::UNIQUE_VACUUM),
            VerdictKind::Holds
        );
        let fork_probes = fork.structural_mutations();
        assert!(
            fork_probes
                .iter()
                .all(|(label, _)| label != "add-missing-spin10"),
            "missing-spin10 fork must not re-offer add-missing-spin10"
        );
        let live = ObserverGeometry::default();
        let canonical = physis_ir::certify_round_trip(&live.ir_package().unwrap()).unwrap();
        let parsed = parse_package(&canonical).unwrap();
        let mut no_unique = ObserverGeometry::default();
        no_unique
            .set("unique_vacuum", KnobValue::Bool(false))
            .unwrap();
        let rebuilt = no_unique.reparse_package(&parsed).unwrap();
        assert_eq!(
            rebuilt.get("unique_vacuum").unwrap(),
            KnobValue::Bool(false),
            "reparse must overlay missing-spin10 IR onto live knobs"
        );
        assert_eq!(
            kind(rebuilt.as_ref(), claims::SM_GAUGE),
            VerdictKind::Holds,
            "live Spin(10) assignment still Holds sm-gauge"
        );
        assert_eq!(
            kind(rebuilt.as_ref(), claims::UNIQUE_VACUUM),
            VerdictKind::Fails
        );
        let live_rebuilt = live.reparse_package(&parsed).unwrap();
        assert_eq!(
            kind(live_rebuilt.as_ref(), claims::SM_GAUGE),
            VerdictKind::Holds
        );
        assert!(
            crate::gut::Su5Gut::default()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-missing-spin10"),
            "su5-gut must not grow add-missing-spin10"
        );
        assert!(
            crate::standard_model::StandardModel::default()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-missing-spin10"),
            "standard-model must not grow add-missing-spin10"
        );
        assert!(
            crate::solid::EinsteinSolid::debye()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-missing-spin10"),
            "debye-solid must not grow add-missing-spin10"
        );
        assert!(
            ObserverGeometry::default()
                .set("unique_vacuum", KnobValue::Bool(false))
                .is_ok(),
            "observer-geometry keeps the unique_vacuum knob"
        );
        let sm = live
            .claims()
            .into_iter()
            .find(|c| c.id_str() == claims::SM_GAUGE)
            .unwrap();
        assert!(
            !sm.domain().is_encoding_wide(),
            "sm-gauge must name Spin(10) on 10-fibre: {:?}",
            sm.domain()
        );
        assert!(
            sm.domain()
                .regimes
                .iter()
                .any(|r| r.contains("Spin(10)") && r.contains("10-fibre")),
            "sm-gauge regime: {:?}",
            sm.domain()
        );
        assert!(
            !sm.domain().notes.contains("theory "),
            "sm-gauge notes must not split why_theory_block: {:?}",
            sm.domain()
        );
    }
}
