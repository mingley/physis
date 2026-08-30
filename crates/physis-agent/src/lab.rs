//! The laboratory: theories, knobs, experiments, journal.

use std::collections::BTreeMap;

use physis_core::assurance::{ClaimClass, SemanticAssurance};
use physis_core::claim::VerdictKind;
use physis_core::error::CoreError;
use physis_core::formal::FormalClaim;
use physis_core::id::LayerId;
use physis_core::judgment::{
    GapReason, Judgment, ParameterOrigin, TrustEvidence, TrustProfile, TrustTier,
};
use physis_core::knob::{KnobDomain, KnobValue};
use physis_core::AxiomLedger;
use physis_proof::{lookup_matching, Challenge, UntrustedProof, CATALOG};
use physis_semantic::SemanticStore;
use physis_store::{ArtifactStore, Node, NodeKind};
use physis_theory::blackbody::Blackbody;
use physis_theory::computation::{CombinationalCircuit, LandauerEngine, TuringMachine};
use physis_theory::continuum::KleinGordonField;
use physis_theory::critique::diff_verdicts;
use physis_theory::dec::DeRham;
use physis_theory::em::{LinearMedium, MaxwellVacuum, OhmCircuit};
use physis_theory::gauge_field::{WilsonSun, WilsonU1};
use physis_theory::gravity::NewtonianGravity;
use physis_theory::olbers::OlbersSky;
use physis_theory::quantum::BellTest;
use physis_theory::solid::EinsteinSolid;
use physis_theory::thermo::IdealGas;
use physis_theory::{
    string_critique, ExperimentReport, GeneralRelativity, ObserverGeometry, SpecialRelativity,
    StandardModel, StringTheory, Su5Gut, Theory, VerdictDiff,
};

use crate::journal::{Journal, JournalEvent};
use crate::protocol::{Command, Response};
use crate::replay::replay_journal;
use crate::role::{ResearchBudget, Role};
use physis_verifier::{discover_tools, verify, ReceiptStore, Verified, VerifyError};

/// Named snapshot of every theory's knobs.
#[derive(Clone, Debug)]
struct BranchState {
    knobs: BTreeMap<String, Vec<(String, KnobValue)>>,
}

/// An agent-operable collection of theories.
pub struct Lab {
    theories: BTreeMap<String, Box<dyn Theory>>,
    journal: Journal,
    receipts: ReceiptStore,
    reviews: SemanticStore,
    branches: BTreeMap<String, BranchState>,
    store: ArtifactStore,
    axioms: AxiomLedger,
    role: Role,
    budget: ResearchBudget,
}

/// The experiments the lab can run, with one-line descriptions.
pub const EXPERIMENTS: &[(&str, &str)] = &[
    (
        "string-critique",
        "string constructions vs the Standard Model, GR, and observer-geometry",
    ),
    (
        "em-vacuum",
        "electromagnetism: vacuum, a linear medium, and the lumped-circuit limit",
    ),
    (
        "computation",
        "computation: a combinational circuit vs a Turing machine",
    ),
    (
        "field-modes",
        "a Klein–Gordon scalar field's computed spectrum on a lattice",
    ),
    (
        "gauge-lattice",
        "lattice gauge theory: compact U(1) vs non-abelian SU(2)/SU(3)",
    ),
    (
        "thermo",
        "thermodynamics: a classical ideal gas and its three laws",
    ),
    (
        "blackbody",
        "cavity radiation: Rayleigh–Jeans vs Planck (ultraviolet catastrophe)",
    ),
    (
        "solid",
        "solid heat capacity: Dulong–Petit vs Einstein vs Debye (T³)",
    ),
    (
        "gravity",
        "solar-system gravity: Newton vs GR (Eddington 1.75″, Mercury 43″)",
    ),
    (
        "olbers",
        "night sky: infinite static Euclidean vs a finite-age horizon",
    ),
    (
        "bell",
        "quantum foundations: a CHSH Bell test refuting local realism",
    ),
];

impl Lab {
    /// Empty lab with an in-memory journal.
    pub fn empty() -> Self {
        Self {
            theories: BTreeMap::new(),
            journal: Journal::memory(),
            receipts: ReceiptStore::empty(),
            reviews: SemanticStore::empty(),
            branches: BTreeMap::new(),
            store: ArtifactStore::empty(),
            axioms: AxiomLedger::physis_defaults(),
            role: Role::Lab,
            budget: ResearchBudget::unlimited(),
        }
    }

    /// Default lab: SM, GR, Type IIB, heterotic E₈×E₈, bosonic, observer-geometry.
    pub fn standard() -> Self {
        let mut lab = Self::empty();
        lab.insert(Box::new(StandardModel::default()));
        lab.insert(Box::new(GeneralRelativity::default()));
        lab.insert(Box::new(NewtonianGravity));
        lab.insert(Box::new(SpecialRelativity::default()));
        lab.insert(Box::new(StringTheory::type_iib()));
        lab.insert(Box::new(StringTheory::type_iia()));
        lab.insert(Box::new(StringTheory::type_i()));
        lab.insert(Box::new(StringTheory::heterotic_e8()));
        lab.insert(Box::new(StringTheory::heterotic_so32()));
        lab.insert(Box::new(StringTheory::bosonic()));
        lab.insert(Box::new(StringTheory::m_theory()));
        lab.insert(Box::new(ObserverGeometry::default()));
        // Grand unification: SU(5) sits one layer above the SM.
        lab.insert(Box::new(Su5Gut::default()));
        // Second domain: electromagnetism shares the same lab and protocol.
        lab.insert(Box::new(MaxwellVacuum));
        lab.insert(Box::new(LinearMedium::default()));
        lab.insert(Box::new(OhmCircuit::default()));
        // Third domain: computation.
        lab.insert(Box::new(CombinationalCircuit::default()));
        lab.insert(Box::new(TuringMachine::default()));
        // Computation ↔ thermodynamics bridge: Landauer's principle.
        lab.insert(Box::new(LandauerEngine::default()));
        // M4 continuum: a scalar field and lattice gauge fields as local objects.
        lab.insert(Box::new(KleinGordonField::default()));
        lab.insert(Box::new(WilsonU1::default()));
        lab.insert(Box::new(WilsonSun::su2()));
        lab.insert(Box::new(WilsonSun::su3()));
        // Fourth domain: thermodynamics on the statistical layer.
        lab.insert(Box::new(IdealGas::default()));
        // Standing 19th-c theory on trial: Rayleigh–Jeans vs Planck.
        lab.insert(Box::new(Blackbody::rayleigh_jeans()));
        lab.insert(Box::new(Blackbody::planck()));
        // Standing 1819 theory on trial: Dulong–Petit vs Einstein vs Debye.
        lab.insert(Box::new(EinsteinSolid::dulong_petit()));
        lab.insert(Box::new(EinsteinSolid::einstein()));
        lab.insert(Box::new(EinsteinSolid::debye()));
        // Standing 19th-c cosmology: Olbers' paradox vs a finite-age horizon.
        lab.insert(Box::new(OlbersSky::static_euclidean()));
        lab.insert(Box::new(OlbersSky::finite_age()));
        // Fifth domain: quantum foundations (a CHSH Bell test).
        lab.insert(Box::new(BellTest::default()));
        // Pure mathematics: discrete exterior calculus / de Rham cohomology.
        lab.insert(Box::new(DeRham::default()));
        let ids = lab.theories.keys().cloned().collect();
        lab.journal.record(JournalEvent::boot(ids));
        lab
    }

    /// Insert a theory under its `id()`.
    pub fn insert(&mut self, theory: Box<dyn Theory>) {
        self.theories.insert(theory.id().to_string(), theory);
    }

    /// Journal.
    pub fn journal(&self) -> &Journal {
        &self.journal
    }

    /// Mutable journal (for file persistence).
    pub fn journal_mut(&mut self) -> &mut Journal {
        &mut self.journal
    }

    /// Active role. Default is [`Role::Lab`].
    pub fn role(&self) -> Role {
        self.role
    }

    /// Restrict which commands [`Self::exec`] will dispatch.
    pub fn set_role(&mut self, role: Role) {
        self.role = role;
    }

    /// Remaining research actions. Default is unlimited.
    pub fn set_budget(&mut self, budget: ResearchBudget) {
        self.budget = budget;
    }

    /// Theory ids.
    pub fn theory_ids(&self) -> Vec<String> {
        self.theories.keys().cloned().collect()
    }

    /// Borrow a theory.
    pub fn theory(&self, id: &str) -> Result<&dyn Theory, CoreError> {
        self.theories
            .get(id)
            .map(|t| t.as_ref())
            .ok_or_else(|| CoreError::UnknownTheory { id: id.into() })
    }

    /// Borrow a theory mutably.
    pub fn theory_mut(&mut self, id: &str) -> Result<&mut dyn Theory, CoreError> {
        self.theories
            .get_mut(id)
            .map(|t| t.as_mut() as &mut dyn Theory)
            .ok_or_else(|| CoreError::UnknownTheory { id: id.into() })
    }

    /// Turn a knob, returning (old, new, verdict diffs).
    pub fn set_knob(
        &mut self,
        theory: &str,
        knob: &str,
        raw: &str,
    ) -> Result<(KnobValue, KnobValue, Vec<VerdictDiff>), CoreError> {
        if !self.theories.contains_key(theory) {
            return Err(CoreError::UnknownTheory { id: theory.into() });
        }
        let before = self.theories[theory].evaluate_all();
        let spec = self.theories[theory].spec(knob)?;
        let value = KnobValue::parse_in_domain(raw, &spec.domain).map_err(|mut e| {
            match &mut e {
                CoreError::TypeMismatch { name, .. } | CoreError::Domain { name, .. } => {
                    *name = knob.into();
                }
                _ => {}
            }
            e
        })?;
        let old = self
            .theories
            .get_mut(theory)
            .unwrap()
            .set(knob, value.clone())?;
        let after = self.theories[theory].evaluate_all();
        let diffs = diff_verdicts(&before, &after);
        self.journal.record(JournalEvent::set_knob(
            theory,
            knob,
            old.clone(),
            value.clone(),
            diffs.clone(),
        ));
        Ok((old, value, diffs))
    }

    /// Re-apply journaled `set-knob` events and remint prove / review /
    /// evidence from live state, **without** recording them again.
    ///
    /// This resumes a persisted session: after loading a journal from a file,
    /// call this so subsequent turns build on the prior ones instead of on
    /// fresh defaults. It is what makes a multi-process `--journal` session a
    /// single coherent, replayable session rather than a bag of independent
    /// one-shot diffs.
    ///
    /// Evidence restore rebuilds the DAG from live evaluations. The recorded
    /// `graph_hash` is not deserialized as the snapshot: a forged hash cannot
    /// mint an Evidence node. [`crate::replay::replay_journal`] still certifies
    /// only `set-knob` diffs.
    pub fn restore_from_journal(&mut self) {
        for ev in self.journal.events().to_vec() {
            match ev {
                JournalEvent::SetKnob {
                    theory, knob, to, ..
                } => {
                    if let Ok(t) = self.theory_mut(&theory) {
                        let _ = t.set(&knob, to);
                    }
                }
                JournalEvent::Prove {
                    claim,
                    challenge_hash,
                    statement_hash,
                    ..
                } => {
                    // Restore must remint the recorded identity. A slug
                    // whose live FormalClaim is not that identity, or whose
                    // live challenge hash drifted, is not this prove.
                    self.restore_prove(&claim, &challenge_hash, &statement_hash);
                }
                JournalEvent::Review {
                    claim,
                    statement_hash,
                    ..
                } => {
                    self.restore_review(&claim, &statement_hash);
                }
                JournalEvent::Evidence { claim, .. } => {
                    // Rebuild from live evaluations. The recorded graph
                    // hash is not deserialized as the DAG.
                    let _ = self.build_evidence_graph(&claim);
                }
                _ => {}
            }
        }
    }

    /// Canonical experiment (fresh default knobs).
    pub fn experiment_canonical(&mut self, id: &str) -> Result<ExperimentReport, CoreError> {
        match id {
            "string-critique" => {
                let report = string_critique();
                self.journal.record(JournalEvent::experiment(id));
                Ok(report)
            }
            "em-vacuum" => {
                let report = physis_theory::em_vacuum();
                self.journal.record(JournalEvent::experiment(id));
                Ok(report)
            }
            "computation" => {
                let report = physis_theory::computation();
                self.journal.record(JournalEvent::experiment(id));
                Ok(report)
            }
            "field-modes" => {
                let report = physis_theory::field_modes();
                self.journal.record(JournalEvent::experiment(id));
                Ok(report)
            }
            "gauge-lattice" => {
                let report = physis_theory::gauge_lattice();
                self.journal.record(JournalEvent::experiment(id));
                Ok(report)
            }
            "thermo" => {
                let report = physis_theory::thermodynamics();
                self.journal.record(JournalEvent::experiment(id));
                Ok(report)
            }
            "blackbody" => {
                let report = physis_theory::blackbody();
                self.journal.record(JournalEvent::experiment(id));
                Ok(report)
            }
            "solid" => {
                let report = physis_theory::solid();
                self.journal.record(JournalEvent::experiment(id));
                Ok(report)
            }
            "gravity" => {
                let report = physis_theory::gravity();
                self.journal.record(JournalEvent::experiment(id));
                Ok(report)
            }
            "olbers" => {
                let report = physis_theory::olbers();
                self.journal.record(JournalEvent::experiment(id));
                Ok(report)
            }
            "bell" => {
                let report = physis_theory::bell();
                self.journal.record(JournalEvent::experiment(id));
                Ok(report)
            }
            other => Err(CoreError::UnknownTheory {
                id: format!("experiment:{other}"),
            }),
        }
    }

    /// Dispatch a protocol command.
    ///
    /// Three independent gates, in order: [`Role`] (who), trust (what
    /// evidence the op *consumes*), then [`ResearchBudget`] (how many).
    /// A refusal is not a mint. `reproduce` and the loop's review step
    /// require P3F. Standalone `review` is encoding-axis and does not.
    pub fn exec(&mut self, cmd: Command) -> Response {
        if !self.role.permits(&cmd) {
            return Response::err(format!(
                "role {} cannot {}; proposers do not mint Verified",
                self.role.as_str(),
                cmd.verb()
            ));
        }
        if let Err(e) = self.trust_permits(&cmd) {
            return Response::err(e);
        }
        if let Err(e) = self.budget.try_consume(&cmd) {
            return Response::err(e);
        }
        match cmd {
            Command::Layers => {
                let mut text = String::from("layers (finest → coarsest)\n");
                for layer in LayerId::ALL {
                    text.push_str(&format!("  {:<14} {}\n", layer.as_str(), layer.doc()));
                }
                Response::ok(text)
            }
            Command::Theories => {
                let mut text = String::from("theories\n");
                for (id, t) in &self.theories {
                    text.push_str(&format!("  {id:<22} {}\n", t.name()));
                    text.push_str(&format!("    {}\n", t.summary()));
                }
                Response::ok(text)
            }
            Command::Knobs { theory } => match theory {
                Some(id) => match self.theory(&id) {
                    Ok(t) => Response::ok(render_knobs(t)),
                    Err(e) => Response::err(e.to_string()),
                },
                None => {
                    let mut text = String::new();
                    for t in self.theories.values() {
                        text.push_str(&render_knobs(t.as_ref()));
                        text.push('\n');
                    }
                    Response::ok(text)
                }
            },
            Command::Run { theory } => {
                if !self.theories.contains_key(&theory) {
                    return Response::err(format!("unknown theory '{theory}'"));
                }
                let eval = self.theories[&theory].evaluate_all();
                let mut holds = 0;
                let mut fails = 0;
                let mut other = 0;
                let mut text = format!("run {theory}\n");
                for (c, v) in &eval {
                    match v.kind {
                        VerdictKind::Holds => holds += 1,
                        VerdictKind::Fails => fails += 1,
                        _ => other += 1,
                    }
                    text.push_str(&format!(
                        "  {:<32} {:<13} {:<16} {}\n",
                        c.id_str(),
                        v.kind.as_str(),
                        v.derivation().as_str(),
                        v.summary
                    ));
                }
                text.push_str(&format!("\nholds={holds} fails={fails} other={other}\n"));
                self.journal
                    .record(JournalEvent::run(theory.clone(), holds, fails, other));
                Response::ok(text)
            }
            Command::Set {
                theory,
                knob,
                value,
            } => match self.set_knob(&theory, &knob, &value) {
                Ok((from, to, diffs)) => {
                    let mut text =
                        format!("{theory}  {knob}: {} → {}\n", from.display(), to.display());
                    if diffs.is_empty() {
                        text.push_str("no scientific axes changed\n");
                    } else {
                        text.push_str("verdict changes:\n");
                        for d in &diffs {
                            text.push_str(&d.render());
                        }
                    }
                    Response::Ok {
                        text,
                        report: None,
                        diffs: Some(diffs),
                    }
                }
                Err(e) => Response::err(e.to_string()),
            },
            Command::Epistemics => {
                // Three orthogonal ledgers. There is no "theorem" row:
                // executed model-internal claims are not kernel proofs.
                let mut by_derivation: BTreeMap<&'static str, [usize; 4]> = BTreeMap::new();
                let mut by_class: BTreeMap<&'static str, [usize; 4]> = BTreeMap::new();
                let mut by_semantic: BTreeMap<&'static str, [usize; 4]> = BTreeMap::new();
                let mut by_trust: BTreeMap<&'static str, usize> = BTreeMap::new();
                let mut total = 0usize;
                for t in self.theories.values() {
                    for (c, v) in t.evaluate_all() {
                        total += 1;
                        let idx = match v.kind {
                            VerdictKind::Holds => 0,
                            VerdictKind::Fails => 1,
                            VerdictKind::Undecidable => 2,
                            VerdictKind::Inapplicable => 3,
                        };
                        let semantic = self.semantic_tag(&c);
                        by_derivation.entry(v.derivation().as_str()).or_default()[idx] += 1;
                        by_class.entry(v.class.as_str()).or_default()[idx] += 1;
                        by_semantic.entry(semantic.as_str()).or_default()[idx] += 1;
                        let profile = self.profile_for(&c, v.derivation(), semantic);
                        for tier in TrustTier::ALL {
                            if profile.has(tier) {
                                *by_trust.entry(tier.as_str()).or_default() += 1;
                            }
                        }
                    }
                }
                let mut text = String::from(
                    "assurance ledger (all lab theories at current knobs)\n\
                     derivation is not a kernel proof: MachineProved is minted only by physis-verifier\n",
                );
                fn dump(
                    text: &mut String,
                    title: &str,
                    order: &[&str],
                    tally: &BTreeMap<&str, [usize; 4]>,
                ) {
                    text.push_str(title);
                    for e in order {
                        if let Some(s) = tally.get(e) {
                            let sum: usize = s.iter().sum();
                            text.push_str(&format!(
                                "  {e:<22} {sum:>3}   holds {} fails {} undecidable {} inapplicable {}\n",
                                s[0], s[1], s[2], s[3]
                            ));
                        }
                    }
                }
                dump(
                    &mut text,
                    "\nderivation\n",
                    &["asserted", "executed", "cross-checked", "certified-numeric"],
                    &by_derivation,
                );
                text.push_str(&format!(
                    "  machine-proved          {}   (receipts minted by physis-verifier)\n",
                    self.receipts.len()
                ));
                text.push_str("\ntrust (derived; a claim may sit in several rows)\n");
                for tier in TrustTier::ALL {
                    let n = by_trust.get(tier.as_str()).copied().unwrap_or(0);
                    text.push_str(&format!("  {:<22} {n:>3}\n", tier.as_str()));
                }
                text.push_str("  P4 is not assigned from an in-process remint\n");
                dump(
                    &mut text,
                    "\nclass\n",
                    &[
                        "mathematical",
                        "model-internal",
                        "phenomenological",
                        "empirical-prediction",
                        "measurement",
                        "conjecture",
                        "heuristic",
                        "open-problem",
                    ],
                    &by_class,
                );
                dump(
                    &mut text,
                    "\nsemantic\n",
                    &[
                        "unreviewed",
                        "source-anchored",
                        "independently-encoded",
                        "adversarially-reviewed",
                        "canonical",
                    ],
                    &by_semantic,
                );
                text.push_str(&format!("\ntotal claim-evaluations: {total}\n"));
                Response::ok(text)
            }
            Command::Why { claim } => {
                let mut text = format!("why {claim}\n");
                let mut found = 0usize;
                for t in self.theories.values() {
                    for (c, v) in t.evaluate_all() {
                        if c.id_str() == claim {
                            found += 1;
                            text.push_str(&format!("theory {}\n", t.id()));
                            text.push_str(&format!("  statement:  {}\n", c.statement()));
                            text.push_str(&format!("  class:      {}\n", v.class.as_str()));
                            text.push_str(&format!("  derivation: {}\n", v.derivation().as_str()));
                            text.push_str(&format!("  empirical:  {}\n", v.empirical().as_str()));
                            let semantic = self.semantic_tag(&c);
                            text.push_str(&format!("  semantic:   {}\n", semantic.as_str()));
                            let profile = self.profile_for(&c, v.derivation(), semantic);
                            let judgment = self.projected_judgment(&c, &v);
                            text.push_str(&format!("  judgment:   {}\n", judgment.label()));
                            if let (Some(lo), Some(hi)) = (v.numeric_lo(), v.numeric_hi()) {
                                text.push_str(&format!("  enclosure:  [{lo}, {hi}]\n"));
                            }
                            if let Some(nll) = v.statistical_nll() {
                                text.push_str(&format!("  nll:        {nll}\n"));
                            }
                            text.push_str(&format!("  trust:      {}\n", profile.display()));
                            if profile.unreviewed_proof_is_dangerous(semantic) {
                                text.push_str(
                                    "  trust note: kernel proof with unreviewed encoding is dangerous\n",
                                );
                            }
                            text.push_str(&format!("  identity:   {}\n", c.statement_hash()));
                            for line in c.commitments().why_lines() {
                                text.push_str(&line);
                                text.push('\n');
                            }
                            for line in c.domain().why_lines() {
                                text.push_str(&line);
                                text.push('\n');
                            }
                            if !c.depends_on.is_empty() {
                                text.push_str("  lemmas:\n");
                                for dep in &c.depends_on {
                                    let status = if self.has_live_receipt(&dep.0) {
                                        "have receipt"
                                    } else {
                                        "needs receipt"
                                    };
                                    text.push_str(&format!("    - {}  {}\n", dep.0, status));
                                }
                            }
                            text.push_str("  assumptions:\n");
                            for a in &c.assumptions().items {
                                text.push_str(&format!(
                                    "    - {} [{}]: {}\n",
                                    a.id,
                                    a.class.as_str(),
                                    a.statement
                                ));
                            }
                            text.push_str(&format!(
                                "  verdict:    {} — {}\n",
                                v.kind.as_str(),
                                v.summary
                            ));
                            match self.receipts.by_statement(c.statement_hash()) {
                                Some(r) => {
                                    text.push_str(&format!(
                                        "  kernel proof: receipt {} / {} + {} (backend {:?})\n",
                                        r.challenge_hash,
                                        r.primary_checker.checker,
                                        r.secondary_checker.checker,
                                        r.formal_backend
                                    ));
                                    text.push_str("  axiom closure:\n");
                                    for (id, rec) in self.axioms.closure(&r.axioms_used) {
                                        match rec {
                                            Some(rec) => text.push_str(&format!(
                                                "    - {} [{}] {}: {}\n",
                                                rec.id.0,
                                                rec.class.as_str(),
                                                rec.review_status.as_str(),
                                                rec.provenance
                                            )),
                                            None => text.push_str(&format!(
                                                "    - {} [missing from ledger]\n",
                                                id.0
                                            )),
                                        }
                                    }
                                }
                                None => {
                                    text.push_str(
                                        "  kernel proof: none (MachineProved is not an enum this lab can set)\n",
                                    );
                                }
                            }
                        }
                    }
                }
                if found == 0 {
                    Response::err(format!("unknown claim '{claim}'"))
                } else {
                    Response::ok(text)
                }
            }
            Command::Evidence { claim } => self.evidence_claim(&claim),
            Command::Experiments => {
                let mut text = String::from("experiments\n");
                for (id, desc) in EXPERIMENTS {
                    text.push_str(&format!("  {id:<16} {desc}\n"));
                }
                Response::ok(text)
            }
            Command::Experiment { id } => match self.experiment_canonical(&id) {
                Ok(report) => Response::Ok {
                    text: report.render(),
                    report: Some(Box::new(report)),
                    diffs: None,
                },
                Err(e) => Response::err(e.to_string()),
            },
            Command::Journal => Response::ok(self.journal.to_string()),
            Command::Score { theory } => match self.theory(&theory) {
                Ok(t) => {
                    let card = physis_theory::score(&physis_theory::empirical_target(), t);
                    Response::ok(card.render())
                }
                Err(e) => Response::err(e.to_string()),
            },
            Command::Prove { claim } => self.prove_claim(&claim),
            Command::Falsify { claim } => self.falsify_claim(&claim),
            Command::Sweep {
                theory,
                knob,
                values,
            } => self.sweep(&theory, &knob, &values),
            Command::Branch { name } => self.branch(&name),
            Command::Checkout { name } => self.checkout(&name),
            Command::Compare { a, b } => self.compare_theories(&a, &b),
            Command::Audit => match physis_audit::attack() {
                Ok(()) => Response::ok("audit: red-team corpus caught (no mutation promoted)\n"),
                Err(e) => Response::err(e),
            },
            Command::Design { theories } => self.design(&theories),
            Command::Sensitivity { theory, knob } => self.sensitivity(&theory, &knob),
            Command::Hypothesize { theory } => self.hypothesize(theory.as_deref()),
            Command::Review { claim } => self.review_claim(&claim),
            Command::Loop => self.research_loop(),
            Command::Inspect { axis, value } => self.inspect(axis.as_deref(), value.as_deref()),
            Command::Formalize { claim } => self.formalize_claim(&claim),
            Command::Reproduce { claim } => self.reproduce_claim(&claim),
            Command::Gaps => self.gaps(),
            Command::Replay { path } => match std::fs::read_to_string(&path) {
                Ok(contents) => {
                    let (journal, malformed) = Journal::from_jsonl_counting(&contents);
                    // Refuse to certify a journal we could not fully parse:
                    // dropped lines would make an incomplete replay look faithful.
                    if malformed > 0 {
                        return Response::err(format!(
                            "journal '{path}': {malformed} malformed line(s); refusing to certify replay"
                        ));
                    }
                    let report = replay_journal(&journal);
                    if report.is_empty() {
                        return Response::err(format!(
                            "journal '{path}': no set-knob events to replay"
                        ));
                    }
                    // A non-faithful replay is a verification failure: exit non-zero.
                    if report.faithful() {
                        Response::ok(report.render())
                    } else {
                        Response::err(report.render())
                    }
                }
                Err(e) => Response::err(format!("cannot read journal '{path}': {e}")),
            },
        }
    }

    /// Evidence an op *consumes*. Observation is free. Standalone encoding
    /// review does not require P3F (the semantic axis is orthogonal).
    /// `reproduce` does: it is a remint, not a first proof and not P4.
    fn trust_permits(&self, cmd: &Command) -> Result<(), String> {
        match cmd {
            Command::Reproduce { claim } => {
                if !self.has_live_receipt(claim) {
                    Err(format!(
                        "reproduce {claim}: trust P3F required (no dual-checked receipt for this identity); not prove and not P4"
                    ))
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }
    }

    fn snapshot_knobs(&self) -> BranchState {
        let mut knobs = BTreeMap::new();
        for (id, t) in &self.theories {
            knobs.insert(
                id.clone(),
                t.snapshot()
                    .into_iter()
                    .map(|(s, v)| (s.name.to_string(), v))
                    .collect(),
            );
        }
        BranchState { knobs }
    }

    fn restore_knobs(&mut self, state: &BranchState) {
        for (id, pairs) in &state.knobs {
            if let Some(t) = self.theories.get_mut(id) {
                for (name, val) in pairs {
                    let _ = t.set(name, val.clone());
                }
            }
        }
    }

    fn find_claim(&self, claim_id: &str) -> Option<physis_core::claim::Claim> {
        for t in self.theories.values() {
            for (c, _) in t.evaluate_all() {
                if c.id_str() == claim_id {
                    return Some(c);
                }
            }
        }
        None
    }

    /// Encoding-review overlay for this live identity. A review recorded
    /// against a different `statement_hash` of the same slug is not P3S.
    /// The evaluator's `Verdict.semantic` is not consulted: P3S is a
    /// review-store tag, not a field a theory can set.
    fn semantic_tag(&self, claim: &physis_core::claim::Claim) -> SemanticAssurance {
        self.reviews
            .by_statement(claim.statement_hash())
            .map(|r| r.assurance())
            .unwrap_or(SemanticAssurance::Unreviewed)
    }

    fn profile_for(
        &self,
        claim: &physis_core::claim::Claim,
        derivation: physis_core::DerivationAssurance,
        semantic: SemanticAssurance,
    ) -> TrustProfile {
        TrustProfile::derive(TrustEvidence {
            derivation,
            semantic,
            dual_checked_receipt: self.receipts.by_statement(claim.statement_hash()).is_some(),
            numeric_certificate: derivation == physis_core::DerivationAssurance::CertifiedNumeric,
        })
    }

    fn projected_judgment(
        &self,
        claim: &physis_core::claim::Claim,
        v: &physis_core::claim::Verdict,
    ) -> Judgment {
        Judgment::from_lab(
            v.class,
            v.kind,
            v.empirical(),
            v.derivation(),
            self.receipts.by_statement(claim.statement_hash()).is_some(),
            v.numeric_lo(),
            v.numeric_hi(),
            v.statistical_nll(),
        )
    }

    /// Competing encodings and evaluations of a lab slug. Groups by
    /// statement hash: a shared id is not one FormalClaim. Inserts a
    /// rebuilt [`NodeKind::Evidence`] snapshot (Statement and Evaluation
    /// parents). Confidence is the derived TrustProfile, not a numeric
    /// score. The graph is not deserialized as authority and does not mint.
    fn evidence_claim(&mut self, claim_id: &str) -> Response {
        match self.build_evidence_graph(claim_id) {
            Ok((out, graph)) => {
                self.journal
                    .record(JournalEvent::evidence(claim_id, graph.to_hex()));
                Response::ok(out)
            }
            Err(e) => Response::err(e),
        }
    }

    /// Rebuild the Evidence DAG from live evaluations. Does not journal
    /// and does not deserialize a recorded graph hash as the snapshot.
    fn build_evidence_graph(
        &mut self,
        claim_id: &str,
    ) -> Result<(String, physis_core::artifact::ArtifactId), String> {
        let mut rows: Vec<EvidenceRow> = Vec::new();
        for t in self.theories.values() {
            for (c, v) in t.evaluate_all() {
                if c.id_str() != claim_id {
                    continue;
                }
                let semantic = self.semantic_tag(&c);
                let dual = self.receipts.by_statement(c.statement_hash()).is_some();
                let profile = self.profile_for(&c, v.derivation(), semantic);
                let judgment = self.projected_judgment(&c, &v);
                let domain = if c.domain().is_encoding_wide() {
                    "encoding-wide".to_string()
                } else if !c.domain().regimes.is_empty() {
                    c.domain().regimes.join(", ")
                } else {
                    "named".to_string()
                };
                rows.push(EvidenceRow {
                    theory: t.id().to_string(),
                    hash: c.statement_hash().to_hex(),
                    kind: v.kind.as_str(),
                    class: v.class.as_str(),
                    derivation: v.derivation().as_str(),
                    empirical: v.empirical().as_str(),
                    judgment: judgment.label(),
                    trust: profile.display(),
                    encoding_wide: c.domain().is_encoding_wide(),
                    domain,
                    receipt: dual,
                });
            }
        }
        if rows.is_empty() {
            return Err(format!("unknown claim '{claim_id}'"));
        }

        let mut by_hash: BTreeMap<String, Vec<EvidenceRow>> = BTreeMap::new();
        for row in rows {
            by_hash.entry(row.hash.clone()).or_default().push(row);
        }
        let mut identities: Vec<(String, Vec<EvidenceRow>)> = by_hash.into_iter().collect();
        identities.sort_by(|a, b| b.1.len().cmp(&a.1.len()).then_with(|| a.0.cmp(&b.0)));

        let mut text = format!("evidence  {claim_id}\n");
        text.push_str(&format!(
            "  encodings  {}  {}\n",
            identities.len(),
            if identities.len() == 1 {
                "this slug is one FormalClaim"
            } else {
                "distinct FormalClaims share this slug"
            }
        ));

        for (hash, nodes) in &identities {
            text.push_str(&format!("  identity  {hash}\n"));
            let domain = &nodes[0].domain;
            let wide = nodes[0].encoding_wide;
            text.push_str(&format!(
                "    domain    {}{}\n",
                domain,
                if wide { " (not a named regime)" } else { "" }
            ));
            text.push_str(&format!(
                "    receipt   {}\n",
                if nodes[0].receipt {
                    "dual-checked (P3F of this identity)"
                } else {
                    "none"
                }
            ));
            for n in nodes {
                text.push_str(&format!(
                    "    {:<20} {:<12} {:<28} trust {:<12} {} / {} / {}\n",
                    n.theory, n.kind, n.judgment, n.trust, n.class, n.derivation, n.empirical
                ));
            }
        }

        if identities.len() > 1 {
            text.push_str("  competing encodings: yes  the lab slug is not one FormalClaim\n");
        } else {
            text.push_str("  competing encodings: no\n");
        }

        text.push_str("  competing evaluations:\n");
        let mut any_eval = false;
        for (hash, nodes) in &identities {
            let mut by_kind: BTreeMap<(&str, &str), Vec<&str>> = BTreeMap::new();
            for n in nodes {
                by_kind
                    .entry((n.kind, n.judgment.as_str()))
                    .or_default()
                    .push(n.theory.as_str());
            }
            if by_kind.len() <= 1 && identities.len() == 1 {
                let ((kind, judgment), theories) = by_kind.iter().next().unwrap();
                text.push_str(&format!(
                    "    consensus  {kind} / {judgment}  theories={}\n",
                    theories.join(",")
                ));
            } else if by_kind.len() > 1 {
                any_eval = true;
                text.push_str(&format!("    identity {}:\n", &hash[..8]));
                for ((kind, judgment), theories) in &by_kind {
                    text.push_str(&format!(
                        "      {kind} / {judgment}  ×{}  ({})\n",
                        theories.len(),
                        theories.join(", ")
                    ));
                }
            }
        }
        if identities.len() > 1 {
            // Interpretations of the slug across encodings.
            let mut slug_kinds: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
            for nodes in identities.iter().map(|(_, n)| n) {
                for n in nodes {
                    slug_kinds
                        .entry(n.kind)
                        .or_default()
                        .push(n.theory.as_str());
                }
            }
            if slug_kinds.len() > 1 {
                any_eval = true;
                text.push_str("    across encodings of this slug:\n");
                for (kind, theories) in &slug_kinds {
                    text.push_str(&format!(
                        "      {kind}  ×{}  ({})\n",
                        theories.len(),
                        theories.join(", ")
                    ));
                }
            }
        }
        if !any_eval && identities.len() > 1 {
            text.push_str("    (each encoding is internally unanimous)\n");
        }

        text.push_str(
            "  confidence  derived from TrustProfile; no numeric score; not Canonical; not P4\n",
        );

        let mut eval_ids: Vec<physis_core::artifact::ArtifactId> = Vec::new();
        for (hash, nodes) in &identities {
            let stmt = self
                .store
                .insert(Node::new(NodeKind::Statement, vec![], hash.as_bytes()));
            for n in nodes {
                let payload = format!(
                    "{}\t{}\t{}\t{}\t{}\t{}\t{}",
                    n.theory, n.kind, n.judgment, n.trust, n.class, n.derivation, n.empirical
                );
                eval_ids.push(self.store.insert(Node::new(
                    NodeKind::Evaluation,
                    vec![stmt],
                    payload.as_bytes(),
                )));
            }
        }
        eval_ids.sort();
        let graph = self
            .store
            .insert(Node::new(NodeKind::Evidence, eval_ids, text.as_bytes()));
        let mut out = format!("evidence  {claim_id}  graph {}\n", graph.to_hex());
        let prefix = format!("evidence  {claim_id}\n");
        if let Some(rest) = text.strip_prefix(&prefix) {
            out.push_str(rest);
        } else {
            out.push_str(&text);
        }
        Ok((out, graph))
    }

    /// A dual-checked receipt for this slug counts only when it matches the
    /// live [`physis_core::claim::Claim::statement_hash`]. A stale receipt
    /// for an older identity is not P3F.
    fn has_live_receipt(&self, claim_id: &str) -> bool {
        self.find_claim(claim_id)
            .is_some_and(|c| self.receipts.by_statement(c.statement_hash()).is_some())
    }

    fn accept_verified<T>(&mut self, v: &Verified<T>) -> physis_verifier::ProofReceipt {
        let r = v.receipt().clone();
        self.receipts.record(v);
        self.store.insert(Node::new(
            NodeKind::VerificationReceipt,
            vec![r.statement_hash],
            r.challenge_hash.to_hex().as_bytes(),
        ));
        r
    }

    fn inspect(&self, axis: Option<&str>, value: Option<&str>) -> Response {
        match (axis, value) {
            (None, None) | (Some("help"), _) => Response::ok(
                "inspect <trust|class|origin|gap|judgment> <value>\n\
                 examples: inspect trust P0 | inspect class conjecture | inspect origin chosen | inspect gap missing-theorem | inspect judgment statistical-computed\n",
            ),
            (Some("trust"), Some(v)) => {
                let want = v.to_ascii_uppercase();
                let Some(tier) = TrustTier::ALL
                    .iter()
                    .copied()
                    .find(|t| t.as_str() == want)
                else {
                    return Response::err(format!("unknown trust tier '{v}'"));
                };
                let mut text = format!("inspect trust {}\n", tier.as_str());
                let mut n = 0usize;
                for (id, t) in &self.theories {
                    for (c, verdict) in t.evaluate_all() {
                        let semantic = self.semantic_tag(&c);
                        let profile = self.profile_for(&c, verdict.derivation(), semantic);
                        if profile.has(tier) {
                            n += 1;
                            text.push_str(&format!(
                                "  {id:<20} {:<36} {}\n",
                                c.id_str(),
                                profile.display()
                            ));
                        }
                    }
                }
                text.push_str(&format!("count {n}\n"));
                Response::ok(text)
            }
            (Some("class"), Some(v)) => {
                let want = v.to_ascii_lowercase();
                let Some(class) = ClaimClass::ALL
                    .iter()
                    .copied()
                    .find(|c| c.as_str() == want)
                else {
                    return Response::err(format!("unknown claim class '{v}'"));
                };
                let mut text = format!("inspect class {}\n", class.as_str());
                let mut n = 0usize;
                for (id, t) in &self.theories {
                    for (c, verdict) in t.evaluate_all() {
                        if verdict.class == class {
                            n += 1;
                            text.push_str(&format!(
                                "  {id:<20} {:<36} {}\n",
                                c.id_str(),
                                verdict.kind.as_str()
                            ));
                        }
                    }
                }
                text.push_str(&format!("count {n}\n"));
                Response::ok(text)
            }
            (Some("origin"), Some(v)) => {
                let want = v.to_ascii_lowercase();
                let Some(origin) = ParameterOrigin::ALL
                    .iter()
                    .copied()
                    .find(|o| o.as_str() == want)
                else {
                    return Response::err(format!("unknown parameter origin '{v}'"));
                };
                let mut text = format!("inspect origin {}\n", origin.as_str());
                let mut n = 0usize;
                for (id, t) in &self.theories {
                    for (spec, val) in t.snapshot() {
                        if spec.origin == origin {
                            n += 1;
                            text.push_str(&format!(
                                "  {id:<20} {:<24} {:<10} {}\n",
                                spec.name,
                                val.display(),
                                spec.origin.as_str()
                            ));
                        }
                    }
                }
                text.push_str(&format!("count {n}\n"));
                Response::ok(text)
            }
            (Some("gap"), Some(v)) => {
                let want = v.to_ascii_lowercase();
                let Some(gap) = GapReason::ALL
                    .iter()
                    .copied()
                    .find(|g| g.as_str() == want)
                else {
                    return Response::err(format!("unknown gap reason '{v}'"));
                };
                let mut text = format!("inspect gap {}\n", gap.as_str());
                let mut n = 0usize;
                for (id, t) in &self.theories {
                    for (c, verdict) in t.evaluate_all() {
                        let dual = self.receipts.by_statement(c.statement_hash()).is_some();
                        if let Some(g) = gap_for(
                            verdict.class,
                            verdict.derivation(),
                            verdict.kind,
                            verdict.empirical(),
                            dual,
                            c.layer(),
                            verdict.intractable(),
                        ) {
                            if g == gap {
                                n += 1;
                                text.push_str(&format!(
                                    "  {id:<20} {:<36} {}\n",
                                    c.id_str(),
                                    g.as_str()
                                ));
                            }
                        }
                    }
                }
                text.push_str(&format!("count {n}\n"));
                Response::ok(text)
            }
            (Some("judgment"), Some(v)) => {
                let Some(want) = Judgment::parse_label(v) else {
                    return Response::err(format!(
                        "unknown judgment label '{v}' (logical-proved|statistical-computed|empirical-excluded|…)"
                    ));
                };
                let mut text = format!("inspect judgment {want}\n");
                let mut n = 0usize;
                for (id, t) in &self.theories {
                    for (c, verdict) in t.evaluate_all() {
                        let judgment = self.projected_judgment(&c, &verdict);
                        if judgment.label() == want {
                            n += 1;
                            text.push_str(&format!(
                                "  {id:<20} {:<36} {}\n",
                                c.id_str(),
                                want
                            ));
                        }
                    }
                }
                text.push_str(&format!("count {n}\n"));
                Response::ok(text)
            }
            (Some(axis), None) => Response::err(format!(
                "inspect {axis} needs a value (trust P0 | class conjecture | origin chosen | gap missing-theorem | judgment statistical-computed)"
            )),
            (None, Some(_)) => {
                Response::err(
                    "inspect needs an axis (trust|class|origin|gap|judgment) before a value",
                )
            }
            (Some(other), _) => Response::err(format!("unknown inspect axis '{other}'")),
        }
    }

    /// Rebuild the knowledge-gap graph from live verdicts and declared
    /// lemma edges. The snapshot is content-addressed; it is not
    /// deserialized as scientific authority.
    fn gaps(&mut self) -> Response {
        let mut buckets: BTreeMap<&'static str, Vec<String>> = BTreeMap::new();
        let mut n = 0usize;
        for (id, t) in &self.theories {
            for (c, verdict) in t.evaluate_all() {
                let dual = self.receipts.by_statement(c.statement_hash()).is_some();
                if let Some(g) = gap_for(
                    verdict.class,
                    verdict.derivation(),
                    verdict.kind,
                    verdict.empirical(),
                    dual,
                    c.layer(),
                    verdict.intractable(),
                ) {
                    n += 1;
                    let mut row = format!("  {id:<20} {:<36} needs {}\n", c.id_str(), need_for(g));
                    for dep in &c.depends_on {
                        let status = if self.has_live_receipt(&dep.0) {
                            "have receipt"
                        } else {
                            "needs receipt"
                        };
                        row.push_str(&format!("    lemma {:<36} {status}\n", dep.0));
                    }
                    buckets.entry(g.as_str()).or_default().push(row);
                }
            }
        }
        let mut body = String::from("gaps\n");
        for reason in GapReason::ALL {
            let rows = buckets.remove(reason.as_str()).unwrap_or_default();
            body.push_str(&format!("{}  {}\n", reason.as_str(), rows.len()));
            for row in rows {
                body.push_str(&row);
            }
        }
        body.push_str(&format!("count {n}\n"));
        let id = self
            .store
            .insert(Node::new(NodeKind::KnowledgeGap, vec![], body.as_bytes()));
        let mut text = format!("gaps  graph {}\n", id.to_hex());
        // Skip the leading "gaps\n" — the graph id is the authority line.
        if let Some(rest) = body.strip_prefix("gaps\n") {
            text.push_str(rest);
        } else {
            text.push_str(&body);
        }
        Response::ok(text)
    }

    /// Replay a prove event only when the recorded identity is the live
    /// FormalClaim and the recorded challenge is that claim's generate-only
    /// obligation. A stale slug is not P3F.
    fn restore_prove(&mut self, claim_id: &str, challenge_hash: &str, statement_hash: &str) {
        let Some(live) = self.find_claim(claim_id) else {
            return;
        };
        if !statement_hash.is_empty() && live.statement_hash().to_hex() != statement_hash {
            return;
        }
        let expected = Challenge::generate(&FormalClaim::from_claim(&live)).challenge_hash();
        if expected.to_hex() != challenge_hash {
            return;
        }
        if self.remint_preferred(claim_id).is_err() {
            let _ = self.remint_exact(claim_id);
        }
    }

    /// Replay a review event only when the recorded identity is the live
    /// FormalClaim. A slug-only (empty) statement hash is not P3S.
    fn restore_review(&mut self, claim_id: &str, statement_hash: &str) {
        let Some(live) = self.find_claim(claim_id) else {
            return;
        };
        if statement_hash.is_empty() || live.statement_hash().to_hex() != statement_hash {
            return;
        }
        let _ = self.remint_review(claim_id);
    }

    /// Re-run the dual checkers. Never deserializes a `Verified` value.
    fn remint_exact(&mut self, claim_id: &str) -> Result<physis_verifier::ProofReceipt, String> {
        let claim = self
            .find_claim(claim_id)
            .ok_or_else(|| format!("unknown claim '{claim_id}'"))?;
        let challenge = Challenge::generate(&FormalClaim::from_claim(&claim));
        let v = verify(&challenge, &UntrustedProof::ExactIdentity).map_err(|e| e.to_string())?;
        Ok(self.accept_verified(&v))
    }

    /// Lean kernel + nanoda on in-tree Physlib. Missing tools is not a mint.
    fn remint_lean(
        &mut self,
        claim_id: &str,
    ) -> Result<physis_verifier::ProofReceipt, VerifyError> {
        let claim = self
            .find_claim(claim_id)
            .ok_or(VerifyError::NoExactIdentity)?;
        let challenge = Challenge::generate(&FormalClaim::from_claim(&claim));
        let v = verify(
            &challenge,
            &UntrustedProof::LeanSource {
                source: physis_proof::PHYSLIB_SOURCE.to_string(),
            },
        )?;
        Ok(self.accept_verified(&v))
    }

    /// Re-run semantic review against the live FormalClaim. Never
    /// deserializes a `SemanticAssurance` tag.
    fn remint_review(&mut self, claim_id: &str) -> Result<physis_semantic::SemanticRecord, String> {
        let claim = self
            .find_claim(claim_id)
            .ok_or_else(|| format!("unknown claim {claim_id}"))?;
        let rec =
            physis_semantic::review(&FormalClaim::from_claim(&claim)).map_err(|e| e.to_string())?;
        self.reviews.record(&rec);
        self.store.insert(Node::new(
            NodeKind::SemanticReview,
            vec![rec.statement_hash(), rec.source_hash()],
            rec.evidence_hash().to_hex().as_bytes(),
        ));
        Ok(rec)
    }

    fn review_claim(&mut self, claim_id: &str) -> Response {
        match self.remint_review(claim_id) {
            Ok(r) => {
                self.journal.record(JournalEvent::review(
                    claim_id,
                    r.evidence_hash().to_hex(),
                    r.statement_hash().to_hex(),
                ));
                Response::ok(format!(
                    "review {claim_id}\n  semantic {}\n  identity {}\n  evidence {}\n  canonical reserved (not agent-mintable)\n",
                    r.assurance().as_str(),
                    r.statement_hash(),
                    r.evidence_hash()
                ))
            }
            Err(e) => Response::err(e),
        }
    }

    /// Prefer Lean kernel + nanoda when the pipeline is wired; otherwise
    /// the exact dual expanders. Never deserializes a `Verified` value.
    fn remint_preferred(
        &mut self,
        claim_id: &str,
    ) -> Result<physis_verifier::ProofReceipt, String> {
        let claim = self
            .find_claim(claim_id)
            .ok_or_else(|| format!("unknown claim '{claim_id}'"))?;
        if lookup_matching(&FormalClaim::from_claim(&claim)).is_some() && discover_tools().is_some()
        {
            match self.remint_lean(claim_id) {
                Err(VerifyError::LeanPipelineNotWired) => self.remint_exact(claim_id),
                Ok(r) => Ok(r),
                Err(e) => Err(e.to_string()),
            }
        } else {
            self.remint_exact(claim_id)
        }
    }

    fn prove_claim(&mut self, claim_id: &str) -> Response {
        match self.remint_preferred(claim_id) {
            Ok(r) => {
                self.journal.record(JournalEvent::prove(
                    claim_id,
                    r.challenge_hash.to_hex(),
                    r.statement_hash.to_hex(),
                ));
                Response::ok(format!(
                    "prove {claim_id}\n  challenge {} \n  backend {:?}\n  checkers {} + {}\n  axioms {}\n",
                    r.challenge_hash,
                    r.formal_backend,
                    r.primary_checker.checker,
                    r.secondary_checker.checker,
                    r.axioms_used
                        .iter()
                        .map(|a| a.0.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ))
            }
            Err(e) => Response::err(e),
        }
    }

    /// Catalog encoding as untrusted bytes. Never calls `verify`.
    fn formalize_claim(&self, claim_id: &str) -> Response {
        let Some(claim) = self.find_claim(claim_id) else {
            return Response::err(format!("formalize {claim_id}: unknown claim"));
        };
        let Some(spec) = lookup_matching(&FormalClaim::from_claim(&claim)) else {
            return Response::err(format!(
                "formalize {claim_id}: no catalog identity; this is not a mint"
            ));
        };
        let expr = (spec.identity)();
        Response::ok(format!(
            "formalize {claim_id}\n\
             status     untrusted encoding (not a receipt)\n\
             lean       {}\n\
             type       {}\n\
             identity   {expr} ≡ 0\n\
             axioms     {}\n\
             note       Physlib.lean bytes remain untrusted until physis-verifier::verify runs\n",
            spec.lean_theorem,
            spec.lean_type,
            spec.axioms.join(", "),
        ))
    }

    /// Same-process remint against a stored receipt. Never assigns P4.
    fn reproduce_claim(&mut self, claim_id: &str) -> Response {
        let Some(live) = self.find_claim(claim_id) else {
            return Response::err(format!("reproduce {claim_id}: claim not in this lab"));
        };
        let Some(prior) = self.receipts.by_statement(live.statement_hash()).cloned() else {
            return Response::err(format!(
                "reproduce {claim_id}: no prior receipt for this identity; this is not prove and not P4"
            ));
        };
        match self.remint_preferred(claim_id) {
            Ok(r) => {
                let same_hash = r.challenge_hash == prior.challenge_hash;
                let same_checkers = r.primary_checker.checker == prior.primary_checker.checker
                    && r.secondary_checker.checker == prior.secondary_checker.checker;
                if same_hash && same_checkers {
                    Response::ok(format!(
                        "reproduce {claim_id}\n\
                         status     in-process remint matched\n\
                         challenge  {}\n\
                         checkers   {} + {}\n\
                         trust      not P4 (same binary, same process)\n",
                        r.challenge_hash, r.primary_checker.checker, r.secondary_checker.checker,
                    ))
                } else {
                    Response::err(format!(
                        "reproduce {claim_id}: remint diverged (hash={same_hash} checkers={same_checkers}); not P4"
                    ))
                }
            }
            Err(e) => Response::err(e),
        }
    }

    fn research_loop(&mut self) -> Response {
        let snap = self.snapshot_knobs();
        let mut text = String::from(
            "loop observe → hypothesize → prove → falsify → replicate → design → audit → review\n",
        );

        let mut holds = 0usize;
        let mut fails = 0usize;
        let mut asserted = 0usize;
        for t in self.theories.values() {
            for (_c, v) in t.evaluate_all() {
                match v.kind {
                    VerdictKind::Holds => holds += 1,
                    VerdictKind::Fails => fails += 1,
                    _ => {}
                }
                if v.derivation() == physis_core::DerivationAssurance::Asserted {
                    asserted += 1;
                }
            }
        }
        text.push_str(&format!(
            "observe  holds={holds} fails={fails} asserted={asserted} receipts={}\n",
            self.receipts.len()
        ));

        let hypo_resp = self.hypothesize(None);
        text.push_str(
            "hypothesize  constrained structural mutation (chosen/fitted knobs; IR package forks; measured frozen)\n",
        );
        for line in hypo_resp.text().lines().skip(1).take(20) {
            text.push_str(&format!("{line}\n"));
        }

        let mut proved = Vec::new();
        for spec in CATALOG {
            let spend = Command::Prove {
                claim: spec.claim_id.to_string(),
            };
            if let Err(e) = self.budget.try_consume(&spend) {
                text.push_str(&format!("prove  {}  {e}\n", spec.claim_id));
                continue;
            }
            match self.remint_preferred(spec.claim_id) {
                Ok(r) => {
                    self.journal.record(JournalEvent::prove(
                        spec.claim_id,
                        r.challenge_hash.to_hex(),
                        r.statement_hash.to_hex(),
                    ));
                    proved.push(spec.claim_id.to_string());
                    text.push_str(&format!("prove  {}  {}\n", spec.claim_id, r.challenge_hash));
                }
                Err(e) => text.push_str(&format!("prove  {}  error: {e}\n", spec.claim_id)),
            }
        }

        let falsify = self.falsify_claim("consistency.critical-dimension");
        text.push_str("falsify  consistency.critical-dimension\n");
        for line in falsify.text().lines().skip(1) {
            text.push_str(&format!("  {line}\n"));
        }

        let mut replicate_ok = true;
        for spec in CATALOG {
            let spend = Command::Prove {
                claim: spec.claim_id.to_string(),
            };
            if let Err(e) = self.budget.try_consume(&spend) {
                replicate_ok = false;
                text.push_str(&format!("replicate  {}  {e}\n", spec.claim_id));
                continue;
            }
            let before = self.find_claim(spec.claim_id).and_then(|c| {
                self.receipts
                    .by_statement(c.statement_hash())
                    .map(|r| r.challenge_hash)
            });
            match self.remint_preferred(spec.claim_id) {
                Ok(r) if Some(r.challenge_hash) == before => {
                    text.push_str(&format!("replicate  {}  ok\n", spec.claim_id));
                }
                Ok(r) => {
                    replicate_ok = false;
                    text.push_str(&format!(
                        "replicate  {}  hash changed {} → {}\n",
                        spec.claim_id,
                        before.map(|h| h.to_hex()).unwrap_or_else(|| "none".into()),
                        r.challenge_hash
                    ));
                }
                Err(e) => {
                    replicate_ok = false;
                    text.push_str(&format!("replicate  {}  error: {e}\n", spec.claim_id));
                }
            }
        }

        let design = self.design(&["olbers-static".into(), "olbers-horizon".into()]);
        text.push_str("design\n");
        for line in design.text().lines().skip(1) {
            if !line.is_empty() {
                text.push_str(&format!("  {line}\n"));
            }
        }

        if let Err(e) = physis_audit::attack() {
            self.restore_knobs(&snap);
            return Response::err(format!("loop audit failed: {e}"));
        }
        text.push_str("audit  red-team corpus caught\n");

        let mut reviewed = Vec::new();
        for spec in CATALOG {
            if !self.has_live_receipt(spec.claim_id) {
                text.push_str(&format!(
                    "review  {}  trust P3F required (no receipt)\n",
                    spec.claim_id
                ));
                continue;
            }
            let spend = Command::Review {
                claim: spec.claim_id.to_string(),
            };
            if let Err(e) = self.budget.try_consume(&spend) {
                text.push_str(&format!("review  {}  {e}\n", spec.claim_id));
                continue;
            }
            match self.remint_review(spec.claim_id) {
                Ok(r) => {
                    self.journal.record(JournalEvent::review(
                        spec.claim_id,
                        r.evidence_hash().to_hex(),
                        r.statement_hash().to_hex(),
                    ));
                    reviewed.push(spec.claim_id.to_string());
                    text.push_str(&format!(
                        "review  {}  {}\n",
                        spec.claim_id,
                        r.assurance().as_str()
                    ));
                }
                Err(e) => text.push_str(&format!("review  {}  error: {e}\n", spec.claim_id)),
            }
        }

        self.restore_knobs(&snap);
        let dim = self
            .theory("type-iib")
            .ok()
            .and_then(|t| t.get("total_dim").ok())
            .map(|v| v.display())
            .unwrap_or_default();
        text.push_str(&format!(
            "restore  type-iib total_dim={dim}  replicate_ok={replicate_ok}\n"
        ));
        self.journal
            .record(JournalEvent::research_loop(proved, reviewed));
        Response::ok(text)
    }

    fn falsify_claim(&mut self, claim_id: &str) -> Response {
        let mut text = format!("falsify {claim_id}\n");
        let mut found_any = false;
        let mut counter = None;
        let ids: Vec<String> = self.theories.keys().cloned().collect();
        for id in ids {
            let t = self.theories[&id].as_ref();
            let Some((claim, verdict)) = t
                .evaluate_all()
                .into_iter()
                .find(|(c, _)| c.id_str() == claim_id)
            else {
                continue;
            };
            found_any = true;
            if verdict.kind != VerdictKind::Holds {
                text.push_str(&format!("  {id}: already {}\n", verdict.kind.as_str()));
                continue;
            }
            let mut snapshot = t
                .snapshot()
                .into_iter()
                .map(|(s, v)| (s.name.to_string(), s.domain.clone(), v))
                .collect::<Vec<_>>();
            snapshot.sort_by_key(|(_, domain, _)| match domain {
                KnobDomain::UInt { .. } => 0,
                KnobDomain::Int { .. } => 1,
                KnobDomain::Float { .. } => 2,
                KnobDomain::Bool => 3,
                KnobDomain::Choice(_) => 4,
            });
            for (name, domain, current) in &snapshot {
                for cand in domain_probes(domain, current) {
                    if cand == *current {
                        continue;
                    }
                    {
                        let t = self.theories.get_mut(&id).unwrap();
                        if t.set(name, cand.clone()).is_err() {
                            continue;
                        }
                    }
                    let now = self.theories[&id]
                        .evaluate_all()
                        .into_iter()
                        .find(|(c, _)| c.id_str() == claim_id)
                        .map(|(_, v)| v.kind);
                    {
                        let t = self.theories.get_mut(&id).unwrap();
                        let _ = t.set(name, current.clone());
                    }
                    if now == Some(VerdictKind::Fails) {
                        counter = Some(format!(
                            "  counterexample: {id} {name} {} → {}  ({} holds → fails)\n",
                            current.display(),
                            cand.display(),
                            claim.id_str()
                        ));
                        break;
                    }
                }
                if counter.is_some() {
                    break;
                }
            }
            if counter.is_some() {
                break;
            }
        }
        if !found_any {
            return Response::err(format!("unknown claim '{claim_id}'"));
        }
        match counter {
            Some(line) => {
                text.push_str(&line);
                Response::ok(text)
            }
            None => {
                text.push_str("  no counterexample in local knob probes\n");
                Response::ok(text)
            }
        }
    }

    fn sweep(&mut self, theory: &str, knob: &str, values: &[String]) -> Response {
        if !self.theories.contains_key(theory) {
            return Response::err(format!("unknown theory '{theory}'"));
        }
        let spec = match self.theories[theory].spec(knob) {
            Ok(s) => s,
            Err(e) => return Response::err(e.to_string()),
        };
        let original = match self.theories[theory].get(knob) {
            Ok(v) => v,
            Err(e) => return Response::err(e.to_string()),
        };
        let baseline = self.theories[theory].evaluate_all();
        let mut text = format!("sweep {theory} {knob}\n");
        for raw in values {
            let value = match KnobValue::parse_in_domain(raw, &spec.domain) {
                Ok(v) => v,
                Err(e) => {
                    text.push_str(&format!("  {raw:<8} error: {e}\n"));
                    continue;
                }
            };
            {
                let t = self.theories.get_mut(theory).unwrap();
                if let Err(e) = t.set(knob, value) {
                    text.push_str(&format!("  {raw:<8} error: {e}\n"));
                    let _ = t.set(knob, original.clone());
                    continue;
                }
            }
            let after = self.theories[theory].evaluate_all();
            let diffs = diff_verdicts(&baseline, &after);
            let changed: Vec<_> = diffs.iter().map(|d| d.claim.as_str()).collect();
            text.push_str(&format!(
                "  {raw:<8} changed_claims={} {:?}\n",
                changed.len(),
                changed
            ));
            let _ = self
                .theories
                .get_mut(theory)
                .unwrap()
                .set(knob, original.clone());
        }
        Response::ok(text)
    }

    fn branch(&mut self, name: &str) -> Response {
        let snap = self.snapshot_knobs();
        self.branches.insert(name.to_string(), snap);
        Response::ok(format!("branch {name}  theories={}\n", self.theories.len()))
    }

    fn checkout(&mut self, name: &str) -> Response {
        let Some(state) = self.branches.get(name).cloned() else {
            return Response::err(format!("unknown branch '{name}'"));
        };
        self.restore_knobs(&state);
        Response::ok(format!("checkout {name}\n"))
    }

    fn compare_theories(&self, a: &str, b: &str) -> Response {
        let ta = match self.theory(a) {
            Ok(t) => t,
            Err(e) => return Response::err(e.to_string()),
        };
        let tb = match self.theory(b) {
            Ok(t) => t,
            Err(e) => return Response::err(e.to_string()),
        };
        let ea = ta.evaluate_all();
        let eb = tb.evaluate_all();
        let diffs = diff_verdicts(&ea, &eb);
        let mut text = format!("compare {a} vs {b}\n");
        for d in &diffs {
            text.push_str(&format!(
                "  {:<32} {} vs {}\n",
                d.claim,
                d.from.as_str(),
                d.to.as_str()
            ));
            if let (Some(fa), Some(ta_)) = (&d.from_empirical, &d.to_empirical) {
                if fa != ta_ {
                    text.push_str(&format!("    empirical:  {fa} → {ta_}\n"));
                }
            }
            if let (Some(fa), Some(ta_)) = (&d.from_judgment, &d.to_judgment) {
                if fa != ta_ {
                    text.push_str(&format!("    judgment:   {fa} → {ta_}\n"));
                }
            }
            if let (Some(fa), Some(ta_)) = (&d.from_derivation, &d.to_derivation) {
                if fa != ta_ {
                    text.push_str(&format!("    derivation: {fa} → {ta_}\n"));
                }
            }
        }
        text.push_str(&format!("discriminating_claims={}\n", diffs.len()));
        Response::ok(text)
    }

    fn design(&self, theories: &[String]) -> Response {
        if theories.len() < 2 {
            return Response::err("design needs at least two theories");
        }
        let mut text = String::from("design (rank by discriminating claim count)\n");
        let mut rows: Vec<(String, String, usize)> = Vec::new();
        for i in 0..theories.len() {
            for j in (i + 1)..theories.len() {
                let a = &theories[i];
                let b = &theories[j];
                let cmp = self.compare_theories(a, b);
                let n = cmp
                    .text()
                    .lines()
                    .filter(|l| l.contains(" vs ") && l.starts_with("  "))
                    .count();
                rows.push((a.clone(), b.clone(), n));
            }
        }
        rows.sort_by_key(|x| std::cmp::Reverse(x.2));
        for (a, b, n) in rows {
            text.push_str(&format!("  {a} vs {b}: {n} discriminating claims\n"));
        }
        Response::ok(text)
    }

    fn sensitivity(&mut self, theory: &str, knob: &str) -> Response {
        if !self.theories.contains_key(theory) {
            return Response::err(format!("unknown theory '{theory}'"));
        }
        let spec = match self.theories[theory].spec(knob) {
            Ok(s) => s,
            Err(e) => return Response::err(e.to_string()),
        };
        let current = match self.theories[theory].get(knob) {
            Ok(v) => v,
            Err(e) => return Response::err(e.to_string()),
        };
        let probes = domain_probes(&spec.domain, &current);
        let baseline = self.theories[theory].evaluate_all();
        let mut text = format!("sensitivity {theory} {knob} (from {})\n", current.display());
        let mut max_flips = 0usize;
        for cand in probes {
            if cand == current {
                continue;
            }
            {
                let t = self.theories.get_mut(theory).unwrap();
                if let Err(e) = t.set(knob, cand.clone()) {
                    text.push_str(&format!("  → {}  error: {e}\n", cand.display()));
                    let _ = t.set(knob, current.clone());
                    continue;
                }
            }
            let after = self.theories[theory].evaluate_all();
            let diffs = diff_verdicts(&baseline, &after);
            max_flips = max_flips.max(diffs.len());
            text.push_str(&format!("  → {}  flips={}\n", cand.display(), diffs.len()));
            let _ = self
                .theories
                .get_mut(theory)
                .unwrap()
                .set(knob, current.clone());
        }
        text.push_str(&format!("max_flips={max_flips}\n"));
        Response::ok(text)
    }

    /// Constrained structural mutation: probe chosen and fitted knobs, then
    /// IR package forks (`Theory::structural_mutations`). Measured / derived
    /// / fundamental-input / nuisance knobs stay frozen. Mutants are not
    /// installed. Does not journal. Does not mint.
    fn hypothesize(&mut self, theory: Option<&str>) -> Response {
        const SHOW: usize = 12;
        let ids: Vec<String> = match theory {
            Some(id) => {
                if !self.theories.contains_key(id) {
                    return Response::err(format!("unknown theory '{id}'"));
                }
                vec![id.to_string()]
            }
            None => self.theories.keys().cloned().collect(),
        };

        let mut frozen = 0usize;
        let mut hits: Vec<HypothesisHit> = Vec::new();
        for id in &ids {
            let knobs: Vec<(String, ParameterOrigin, KnobDomain, KnobValue)> = self.theories[id]
                .snapshot()
                .into_iter()
                .map(|(s, v)| (s.name.to_string(), s.origin, s.domain.clone(), v))
                .collect();
            let baseline = self.theories[id].evaluate_all();
            for (name, origin, domain, current) in knobs {
                if !matches!(origin, ParameterOrigin::Chosen | ParameterOrigin::Fitted) {
                    frozen += 1;
                    continue;
                }
                for cand in domain_probes(&domain, &current) {
                    if cand == current {
                        continue;
                    }
                    {
                        let t = self.theories.get_mut(id).unwrap();
                        if t.set(&name, cand.clone()).is_err() {
                            continue;
                        }
                    }
                    let after = self.theories[id].evaluate_all();
                    let diffs = diff_verdicts(&baseline, &after);
                    {
                        let t = self.theories.get_mut(id).unwrap();
                        let _ = t.set(&name, current.clone());
                    }
                    if !diffs.is_empty() {
                        hits.push(HypothesisHit {
                            theory: id.clone(),
                            knob: name.clone(),
                            origin,
                            from: current.display(),
                            to: cand.display(),
                            diffs,
                            ir: false,
                        });
                    }
                }
            }
            for (label, mutant) in self.theories[id].structural_mutations() {
                let after = mutant.evaluate_all();
                let diffs = diff_verdicts(&baseline, &after);
                if !diffs.is_empty() {
                    hits.push(HypothesisHit {
                        theory: id.clone(),
                        knob: label.clone(),
                        origin: ParameterOrigin::Chosen,
                        from: "package".into(),
                        to: label,
                        diffs,
                        ir: true,
                    });
                }
            }
        }

        hits.sort_by(|a, b| {
            b.diffs
                .len()
                .cmp(&a.diffs.len())
                .then_with(|| origin_rank(a.origin).cmp(&origin_rank(b.origin)))
                .then_with(|| a.theory.cmp(&b.theory))
                .then_with(|| a.knob.cmp(&b.knob))
                .then_with(|| a.to.cmp(&b.to))
        });

        let mut text = String::from(
            "hypothesize  constrained structural mutation\n  measured/derived/fundamental-input/nuisance knobs are frozen\n  ir package mutations are not knobs and are not persisted\n",
        );
        let shown = if theory.is_some() {
            hits.len()
        } else {
            hits.len().min(SHOW)
        };
        for hit in hits.iter().take(shown) {
            let tag = if hit.ir {
                "ir structural"
            } else {
                match hit.origin {
                    ParameterOrigin::Fitted => "fitted accommodate",
                    _ => "chosen structural",
                }
            };
            text.push_str(&format!(
                "  {}  {}: {} → {}  origin={}  flips={}\n",
                hit.theory,
                hit.knob,
                hit.from,
                hit.to,
                tag,
                hit.diffs.len()
            ));
            for d in &hit.diffs {
                for line in d.render().lines() {
                    text.push_str("  ");
                    text.push_str(line);
                    text.push('\n');
                }
            }
        }
        text.push_str(&format!(
            "candidates={} shown={} frozen_knobs={frozen}\n",
            hits.len(),
            shown
        ));
        Response::ok(text)
    }
}

struct HypothesisHit {
    theory: String,
    knob: String,
    origin: ParameterOrigin,
    from: String,
    to: String,
    diffs: Vec<VerdictDiff>,
    ir: bool,
}

struct EvidenceRow {
    theory: String,
    hash: String,
    kind: &'static str,
    class: &'static str,
    derivation: &'static str,
    empirical: &'static str,
    judgment: String,
    trust: String,
    encoding_wide: bool,
    domain: String,
    receipt: bool,
}

fn origin_rank(origin: ParameterOrigin) -> u8 {
    match origin {
        ParameterOrigin::Chosen => 0,
        ParameterOrigin::Fitted => 1,
        _ => 2,
    }
}

fn domain_probes(domain: &KnobDomain, current: &KnobValue) -> Vec<KnobValue> {
    match domain {
        KnobDomain::Bool => vec![KnobValue::Bool(true), KnobValue::Bool(false)],
        KnobDomain::Int { min, max } => {
            vec![KnobValue::Int(*min), KnobValue::Int(*max), current.clone()]
        }
        KnobDomain::UInt { min, max } => {
            let mut v = vec![KnobValue::UInt(*min), KnobValue::UInt(*max)];
            if let KnobValue::UInt(x) = current {
                if *x > *min {
                    v.push(KnobValue::UInt(*x - 1));
                }
                if *x < *max {
                    v.push(KnobValue::UInt(*x + 1));
                }
            }
            v
        }
        KnobDomain::Float { min, max } => vec![
            KnobValue::Float(*min),
            KnobValue::Float(*max),
            current.clone(),
        ],
        KnobDomain::Choice(opts) => opts
            .iter()
            .map(|s| KnobValue::Choice((*s).into()))
            .collect(),
    }
}

fn gap_for(
    class: physis_core::ClaimClass,
    derivation: physis_core::DerivationAssurance,
    kind: VerdictKind,
    empirical: physis_core::EmpiricalStatus,
    dual_checked: bool,
    layer: LayerId,
    intractable: bool,
) -> Option<GapReason> {
    if dual_checked {
        return None;
    }
    use physis_core::ClaimClass::*;
    use physis_core::EmpiricalStatus;
    // Open / conjectural / heuristic stay scientific gaps even when the
    // evaluator reports Undecidable (P vs NP). Information-layer
    // Undecidable is computability (halting, Rice) unless the evaluator
    // marked a resource bound (coNP-complete search, exponential
    // configuration graphs). Other undecidable evaluations are encoding
    // gaps, not logical undecidability.
    // MissingTheorem is only for Holds: a Fails evaluation is already
    // decided by the encoding, not a missing lemma.
    // Empirical Inconclusive is a precision gap, even when the evaluator
    // reports Undecidable: overlap is not containment.
    match class {
        Conjecture | OpenProblem | Heuristic => Some(GapReason::ScientificOpenProblem),
        EmpiricalPrediction | Measurement => match empirical {
            EmpiricalStatus::Untested => Some(GapReason::MissingDataset),
            EmpiricalStatus::Inconclusive => Some(GapReason::InsufficientPrecision),
            EmpiricalStatus::Compatible
            | EmpiricalStatus::Supported
            | EmpiricalStatus::Excluded
            | EmpiricalStatus::NotApplicable
            | EmpiricalStatus::Tension => None,
        },
        // A resolution/enclosure gap is InsufficientPrecision even when the
        // claim is model-internal: too coarse is not a missing encoding and
        // not a failed stencil.
        _ if empirical == EmpiricalStatus::Inconclusive => Some(GapReason::InsufficientPrecision),
        _ if kind == VerdictKind::Undecidable => {
            if intractable {
                Some(GapReason::ComputationallyIntractable)
            } else if layer == LayerId::Information {
                Some(GapReason::LogicallyUndecidable)
            } else {
                Some(GapReason::UnsupportedFormalPrimitive)
            }
        }
        // Fails and Inapplicable are already decided by the encoding.
        // A kernel receipt would not turn a failing evaluation into a lemma.
        Mathematical | ModelInternal | Phenomenological
            if kind == VerdictKind::Holds
                && derivation != physis_core::DerivationAssurance::Asserted =>
        {
            Some(GapReason::MissingTheorem)
        }
        _ => None,
    }
}

fn need_for(g: GapReason) -> &'static str {
    match g {
        GapReason::MissingTheorem => "receipt",
        GapReason::MissingDataset => "dataset",
        GapReason::InsufficientPrecision => "tighter-enclosure",
        GapReason::UnsupportedFormalPrimitive => "encoding",
        GapReason::ComputationallyIntractable => "resources",
        GapReason::LogicallyUndecidable => "computability",
        GapReason::ScientificOpenProblem => "science",
    }
}

fn render_knobs(t: &dyn Theory) -> String {
    let mut text = format!("knobs  {}\n", t.id());
    for (spec, val) in t.snapshot() {
        text.push_str(&format!(
            "  {:<24} {:<10} {:<18} {}\n    {}\n",
            spec.name,
            val.display(),
            spec.origin.as_str(),
            spec.layer.as_str(),
            spec.doc
        ));
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Slice of a `why` dump for one theory. Shared claim ids (SM vs
    /// Green–Schwarz) must not be judged as a single string.
    fn why_theory_block<'a>(why: &'a str, theory: &str) -> &'a str {
        why.split("theory ")
            .find(|b| b.starts_with(&format!("{theory}\n")))
            .unwrap_or_else(|| panic!("missing theory {theory} in:\n{why}"))
    }

    #[test]
    fn turning_iib_dimension_flips_critical_claim() {
        let mut lab = Lab::standard();
        let diffs = lab.set_knob("type-iib", "total_dim", "9").unwrap().2;
        let crit = diffs
            .iter()
            .find(|d| d.claim == "consistency.critical-dimension")
            .expect("critical-dimension");
        assert_eq!(crit.from, VerdictKind::Holds);
        assert_eq!(crit.to, VerdictKind::Fails);
        assert_eq!(crit.from_judgment.as_deref(), Some("logical undetermined"));
        assert_eq!(crit.to_judgment.as_deref(), Some("logical disproved"));
        let text = lab
            .exec(Command::Set {
                theory: "klein-gordon".into(),
                knob: "spacing".into(),
                value: "100".into(),
            })
            .text()
            .to_string();
        assert!(text.contains("holds → undecidable"), "{text}");
        assert!(
            text.contains("not-applicable → inconclusive"),
            "empirical axis must be in the causal diff: {text}"
        );
        assert!(
            text.contains("logical undetermined → numeric unresolved"),
            "judgment axis must be in the causal diff: {text}"
        );
        assert!(
            !text.contains("theorem"),
            "a coarse lattice is not a failed theorem: {text}"
        );
    }

    #[test]
    fn sm_generation_knob_flips_claim() {
        let mut lab = Lab::standard();
        let diffs = lab
            .set_knob("standard-model", "generations", "2")
            .unwrap()
            .2;
        assert!(diffs
            .iter()
            .any(|d| d.claim == "empirical.three-generations" && d.to == VerdictKind::Fails));
    }

    #[test]
    fn epistemic_ledger_has_no_theorem_tag() {
        let mut lab = Lab::standard();
        let text = lab.exec(Command::Epistemics).text().to_string();
        assert!(
            !text.contains("theorem"),
            "Level-3 forbids a forgeable theorem tag, got {text}"
        );
        assert!(text.contains("executed"));
        assert!(text.contains("machine-proved"));
        assert!(text.contains("trust (derived"));
        assert!(text.contains("P3F"));
        assert!(text.contains("P4 is not assigned"));
        assert!(text.contains("total claim-evaluations:"));
        assert!(text.contains("open-problem") || text.contains("conjecture"));
        assert!(
            text.contains("certified-numeric"),
            "P3N must appear as a derivation row once SM anomalies are exact: {text}"
        );
        assert!(
            text.contains("cross-checked"),
            "P2 two-path identities must appear as a derivation row: {text}"
        );
    }

    #[test]
    fn sm_anomaly_cancellation_earns_p3n_not_p3f() {
        let mut lab = Lab::standard();
        let p3n = lab
            .exec(Command::Inspect {
                axis: Some("trust".into()),
                value: Some("P3N".into()),
            })
            .text()
            .to_string();
        assert!(
            p3n.lines()
                .any(|l| l.contains("standard-model")
                    && l.contains("consistency.anomaly-cancellation")),
            "{p3n}"
        );
        assert!(
            p3n.lines()
                .any(|l| l.contains("standard-model") && l.contains("sm.hypercharge-derivation")),
            "exact Ratio hypercharge roots must mint P3N: {p3n}"
        );
        assert!(
            p3n.lines().any(
                |l| l.contains("standard-model") && l.contains("empirical.charge-quantization")
            ),
            "hydrogen neutrality from T3+Y must mint P3N: {p3n}"
        );
        assert!(
            p3n.lines().any(|l| l.trim() == "count 4"),
            "P3N is three SM cells plus GUT-scale 3/8, not Tr Q: {p3n}"
        );
        assert!(
            !p3n.contains("gut.charge-quantization"),
            "Tr Q is ΣY, already certified as the grav anomaly: {p3n}"
        );
        assert!(
            p3n.lines().any(|l| l.contains("su5-gut")
                && l.contains("gut.weinberg-angle")
                && !l.contains("mz")),
            "GUT-scale 3/8 is P3N; GQW at M_Z is not: {p3n}"
        );
        assert!(
            !p3n.contains("gut.weinberg-angle-mz"),
            "GQW running and the 3% band are not P3N: {p3n}"
        );
        assert!(
            !p3n.lines()
                .any(|l| l.contains("type-iib") && l.contains("consistency.anomaly-cancellation")),
            "Green-Schwarz stays encoded, not a Ratio certificate: {p3n}"
        );
        assert!(!p3n.contains("predictivity.unique-vacuum"), "{p3n}");
        assert!(
            !p3n.contains("gut.proton-lifetime-sk"),
            "Super-K interval comparison is not P3N: {p3n}"
        );

        let why = lab
            .exec(Command::Why {
                claim: "consistency.anomaly-cancellation".into(),
            })
            .text()
            .to_string();
        let sm_anom = why_theory_block(&why, "standard-model");
        assert!(
            sm_anom.contains("derivation: certified-numeric"),
            "{sm_anom}"
        );
        assert!(
            sm_anom.contains("judgment:   numeric certified"),
            "{sm_anom}"
        );
        assert!(sm_anom.contains("enclosure:  [0, 0]"), "{sm_anom}");
        assert!(!sm_anom.contains("logical undetermined"), "{sm_anom}");
        assert!(sm_anom.contains("P3N"), "{sm_anom}");
        assert!(!sm_anom.contains("P3F"), "{sm_anom}");
        assert!(sm_anom.contains("kernel proof: none"), "{sm_anom}");
        assert!(sm_anom.contains("one SM generation"), "{sm_anom}");
        assert!(
            !sm_anom.contains("not yet a machine-checked regime"),
            "SM anomalies must not be encoding-wide: {sm_anom}"
        );
        let gs = why_theory_block(&why, "type-iib");
        assert!(gs.contains("derivation: executed"), "{gs}");
        assert!(gs.contains("judgment:   logical undetermined"), "{gs}");
        assert!(!gs.contains("numeric certified"), "{gs}");
        let why_y = lab
            .exec(Command::Why {
                claim: "sm.hypercharge-derivation".into(),
            })
            .text()
            .to_string();
        let sm_y = why_theory_block(&why_y, "standard-model");
        assert!(sm_y.contains("derivation: certified-numeric"), "{sm_y}");
        assert!(sm_y.contains("judgment:   numeric certified"), "{sm_y}");
        assert!(sm_y.contains("enclosure:  [-1/2, -1/2]"), "{sm_y}");
        assert!(!sm_y.contains("logical undetermined"), "{sm_y}");
        assert!(sm_y.contains("P3N"), "{sm_y}");
        assert!(!sm_y.contains("P3F"), "{sm_y}");
        assert!(sm_y.contains("kernel proof: none"), "{sm_y}");
        assert!(sm_y.contains("one SM generation"), "{sm_y}");
        assert!(
            !sm_y.contains("not yet a machine-checked regime"),
            "hypercharge solve must not be encoding-wide: {sm_y}"
        );
        let why_q = lab
            .exec(Command::Why {
                claim: "empirical.charge-quantization".into(),
            })
            .text()
            .to_string();
        let sm_q = why_theory_block(&why_q, "standard-model");
        assert!(sm_q.contains("derivation: certified-numeric"), "{sm_q}");
        assert!(sm_q.contains("judgment:   numeric certified"), "{sm_q}");
        assert!(sm_q.contains("enclosure:  [0, 0]"), "{sm_q}");
        assert!(!sm_q.contains("logical undetermined"), "{sm_q}");
        assert!(sm_q.contains("P3N"), "{sm_q}");
        assert!(!sm_q.contains("P3F"), "{sm_q}");
        assert!(sm_q.contains("kernel proof: none"), "{sm_q}");
        assert!(sm_q.contains("hydrogen atom"), "{sm_q}");
        assert!(
            !sm_q.contains("not yet a machine-checked regime"),
            "hydrogen neutrality must not be encoding-wide: {sm_q}"
        );
        let why_s2 = lab
            .exec(Command::Why {
                claim: "gut.weinberg-angle".into(),
            })
            .text()
            .to_string();
        let gut_s2 = why_theory_block(&why_s2, "su5-gut");
        assert!(gut_s2.contains("derivation: certified-numeric"), "{gut_s2}");
        assert!(gut_s2.contains("judgment:   numeric certified"), "{gut_s2}");
        assert!(gut_s2.contains("enclosure:  [3/8, 3/8]"), "{gut_s2}");
        assert!(!gut_s2.contains("logical undetermined"), "{gut_s2}");
        assert!(gut_s2.contains("P3N"), "{gut_s2}");
        assert!(!gut_s2.contains("P3F"), "{gut_s2}");
        assert!(gut_s2.contains("kernel proof: none"), "{gut_s2}");
        assert!(gut_s2.contains("unification-scale"), "{gut_s2}");
        assert!(
            !gut_s2.contains("not yet a machine-checked regime"),
            "GUT-scale 3/8 must not be encoding-wide: {gut_s2}"
        );
        let why_trq = lab
            .exec(Command::Why {
                claim: "gut.charge-quantization".into(),
            })
            .text()
            .to_string();
        assert!(why_trq.contains("derivation: executed"), "{why_trq}");
        assert!(
            why_trq.contains("judgment:   logical undetermined"),
            "{why_trq}"
        );
        assert!(
            !why_trq.contains("derivation: certified-numeric"),
            "Tr Q is the grav anomaly, not a second P3N: {why_trq}"
        );
        assert!(!why_trq.contains("numeric certified"), "{why_trq}");
        assert!(
            why_trq.contains("not yet a machine-checked regime"),
            "Tr Q stays encoding-wide: {why_trq}"
        );
        let why_mz = lab
            .exec(Command::Why {
                claim: "gut.weinberg-angle-mz".into(),
            })
            .text()
            .to_string();
        assert!(why_mz.contains("derivation: asserted"), "{why_mz}");
        assert!(
            !why_mz.contains("derivation: certified-numeric"),
            "{why_mz}"
        );
        assert!(why_mz.contains("M_Z"), "{why_mz}");
        assert!(
            !why_mz.contains("not yet a machine-checked regime"),
            "GQW at M_Z must name the pole, not encoding-wide: {why_mz}"
        );
        let gut_run = lab
            .exec(Command::Run {
                theory: "su5-gut".into(),
            })
            .text()
            .to_string();
        assert!(
            gut_run.lines().any(|l| l.contains("gut.weinberg-angle")
                && !l.contains("mz")
                && l.contains("certified-numeric")),
            "GUT-scale 3/8 must not stay executed: {gut_run}"
        );
        assert!(
            gut_run
                .lines()
                .any(|l| l.contains("gut.charge-quantization")
                    && l.contains("executed")
                    && !l.contains("certified-numeric")),
            "Tr Q is ΣY, not a second P3N: {gut_run}"
        );
        assert!(
            gut_run.lines().any(|l| l.contains("gut.weinberg-angle-mz")
                && l.contains("asserted")
                && !l.contains("certified-numeric")),
            "GQW at M_Z stays asserted: {gut_run}"
        );
        let run = lab
            .exec(Command::Run {
                theory: "standard-model".into(),
            })
            .text()
            .to_string();
        assert!(run.contains("certified-numeric"), "{run}");
        assert!(run.contains("exact Ratio"), "{run}");
        assert!(
            run.lines().any(|l| l.contains("sm.hypercharge-derivation")
                && l.contains("certified-numeric")),
            "hypercharge derivation must not stay executed: {run}"
        );
        assert!(
            run.lines()
                .any(|l| l.contains("empirical.charge-quantization")
                    && l.contains("certified-numeric")),
            "hydrogen neutrality must not stay executed: {run}"
        );
        // Heterotic GS remains executed in the same why dump.
        assert!(
            why.contains("derivation: executed"),
            "string GS must not inherit P3N: {why}"
        );
    }

    #[test]
    fn euler_poincare_and_hodge_earn_p2_not_p3f() {
        let mut lab = Lab::standard();
        let p2 = lab
            .exec(Command::Inspect {
                axis: Some("trust".into()),
                value: Some("P2".into()),
            })
            .text()
            .to_string();
        assert!(
            p2.lines()
                .any(|l| l.contains("de-rham") && l.contains("dec.hodge-harmonic")),
            "{p2}"
        );
        assert!(
            !p2.contains("dec.euler-poincare"),
            "Euler-Poincaré is rank-cancellation, not a second path: {p2}"
        );
        assert!(
            p2.lines().any(|l| l.trim() == "count 1"),
            "P2 is Hodge Laplacian vs b1, not Euler-Poincaré: {p2}"
        );
        assert!(
            !p2.contains("dec.d-squared-zero"),
            "d² needs a receipt for P3F, not a two-path overlay: {p2}"
        );
        assert!(
            !p2.contains("dec.closed-equals-exact"),
            "Poincaré is b₁ = 0, not χ cross-checked: {p2}"
        );
        assert!(!p2.contains("gut.weinberg"), "GQW is not P2: {p2}");
        assert!(!p2.contains("predictivity.unique-vacuum"), "{p2}");
        assert!(!p2.contains("sm.hypercharge-derivation"), "{p2}");

        let why = lab
            .exec(Command::Why {
                claim: "dec.euler-poincare".into(),
            })
            .text()
            .to_string();
        assert!(why.contains("derivation: executed"), "{why}");
        assert!(!why.contains("derivation: cross-checked"), "{why}");
        assert!(
            why.contains("not yet a machine-checked regime"),
            "Euler–Poincaré stays encoding-wide: {why}"
        );

        let why_h = lab
            .exec(Command::Why {
                claim: "dec.hodge-harmonic".into(),
            })
            .text()
            .to_string();
        assert!(why_h.contains("derivation: cross-checked"), "{why_h}");
        assert!(why_h.contains("P2"), "{why_h}");
        assert!(!why_h.contains("P3F"), "{why_h}");
        assert!(why_h.contains("kernel proof: none"), "{why_h}");
        assert!(why_h.contains("finite simplicial 1-cochains"), "{why_h}");
        assert!(
            !why_h.contains("not yet a machine-checked regime"),
            "Hodge P2 must name the discrete Laplacian, not encoding-wide: {why_h}"
        );

        let run = lab
            .exec(Command::Run {
                theory: "de-rham".into(),
            })
            .text()
            .to_string();
        assert!(
            run.lines()
                .any(|l| l.contains("dec.euler-poincare") && l.contains("executed")),
            "{run}"
        );
        assert!(
            run.lines()
                .any(|l| l.contains("dec.hodge-harmonic") && l.contains("cross-checked")),
            "{run}"
        );
        assert!(
            run.lines()
                .any(|l| l.contains("dec.closed-equals-exact") && l.contains("executed")),
            "Poincaré must stay executed: {run}"
        );
    }

    #[test]
    fn why_prints_assumptions_and_denies_a_kernel_proof() {
        let mut lab = Lab::standard();
        let text = lab
            .exec(Command::Why {
                claim: "consistency.critical-dimension".into(),
            })
            .text()
            .to_string();
        assert!(text.contains("derivation: executed"));
        assert!(text.contains("encoding-is-the-model"));
        assert!(text.contains("kernel proof: none"));
        assert!(!text.contains("quantifier: forall"), "{text}");
        assert!(!text.contains("physlib:unversioned"), "{text}");
        assert!(text.contains("judgment:   logical undetermined"), "{text}");
        assert!(text.contains("trust:      P1"), "{text}");
        assert!(!text.contains("P3F"));
        assert!(!text.contains("theorem"));
    }

    #[test]
    fn why_prints_first_class_identity_fields() {
        let mut lab = Lab::standard();
        let d2 = lab
            .exec(Command::Why {
                claim: "dec.d-squared-zero".into(),
            })
            .text()
            .to_string();
        let d2b = why_theory_block(&d2, "de-rham");
        assert!(d2b.contains("quantifier: forall"), "{d2b}");
        assert!(d2b.contains("physlib:unversioned"), "{d2b}");
        assert!(d2b.contains("  units:"), "{d2b}");
        assert!(d2b.contains("regimes:"), "{d2b}");
        assert!(
            d2b.contains("oriented 2-simplex coboundary over Z"),
            "{d2b}"
        );
        assert!(
            !d2b.contains("not yet a machine-checked regime"),
            "catalog d² must not be encoding-wide: {d2b}"
        );
        assert!(
            d2b.contains("discrete-coboundary"),
            "catalog d² must name the coboundary hypothesis before prove: {d2b}"
        );
        assert!(
            d2b.contains("integer-arithmetic"),
            "catalog d² must name integer-arithmetic before prove: {d2b}"
        );

        let poincare = lab
            .exec(Command::Why {
                claim: "dec.closed-equals-exact".into(),
            })
            .text()
            .to_string();
        let pb = why_theory_block(&poincare, "de-rham");
        assert!(!pb.contains("quantifier: forall"), "{pb}");
        assert!(
            !pb.contains("physlib:unversioned"),
            "Poincaré is not a catalog polynomial: {pb}"
        );
        assert!(
            pb.contains("not yet a machine-checked regime"),
            "Poincaré stays encoding-wide: {pb}"
        );
        assert!(
            !pb.contains("discrete-coboundary"),
            "Poincaré is not the catalog coboundary identity: {pb}"
        );

        let gut = lab
            .exec(Command::Why {
                claim: "gut.weinberg-angle".into(),
            })
            .text()
            .to_string();
        let gb = why_theory_block(&gut, "su5-gut");
        assert!(
            gb.contains("  boundary:") && gb.contains("unification-scale"),
            "{gb}"
        );
        assert!(gb.contains("Tr(T3^2)/Tr(Q^2)"), "{gb}");
        assert!(gb.contains("regimes:"), "{gb}");
        assert!(
            !gb.contains("not yet a machine-checked regime"),
            "GUT-scale 3/8 must name unification-scale, not encoding-wide: {gb}"
        );
        assert!(!gb.contains("pdg-2024-sin2theta"), "{gb}");

        let mz = lab
            .exec(Command::Why {
                claim: "gut.weinberg-angle-mz-interval".into(),
            })
            .text()
            .to_string();
        let mzb = why_theory_block(&mz, "su5-gut");
        assert!(
            mzb.contains("  datasets:") && mzb.contains("pdg-2024-sin2theta"),
            "{mzb}"
        );
        assert!(mzb.contains("M_Z"), "{mzb}");
        assert!(mzb.contains("regimes:"), "{mzb}");
        assert!(
            !mzb.contains("not yet a machine-checked regime"),
            "PDG interval cell must name M_Z, not encoding-wide: {mzb}"
        );

        let sk = lab
            .exec(Command::Why {
                claim: "gut.proton-lifetime-sk".into(),
            })
            .text()
            .to_string();
        let skb = why_theory_block(&sk, "su5-gut");
        assert!(
            skb.contains("  datasets:") && skb.contains("sk-2020-p-e-pi0"),
            "{skb}"
        );
        assert!(skb.contains("p→e+π0"), "{skb}");
        assert!(skb.contains("regimes:"), "{skb}");
        assert!(
            !skb.contains("not yet a machine-checked regime"),
            "Super-K p→e+π0 must name a regime, not encoding-wide: {skb}"
        );
        assert!(skb.contains("empirical:  excluded"), "{skb}");
        assert!(skb.contains("judgment:   empirical excluded"), "{skb}");
        assert!(!skb.contains("nll:"), "{skb}");
        assert!(skb.contains("trust:      P1"), "{skb}");
        assert!(
            !skb.contains("certified-numeric"),
            "Super-K exclusion is executed, not P3N: {skb}"
        );
        assert!(skb.contains("kernel proof: none"), "{skb}");

        let q = lab
            .exec(Command::Why {
                claim: "empirical.charge-quantization".into(),
            })
            .text()
            .to_string();
        let qb = why_theory_block(&q, "standard-model");
        assert!(
            qb.contains("  definitions:") && qb.contains("Q = T3 + Y"),
            "{qb}"
        );
        assert!(qb.contains("hydrogen atom"), "{qb}");
        assert!(
            !qb.contains("not yet a machine-checked regime"),
            "hydrogen neutrality must not be encoding-wide: {qb}"
        );

        let interval = lab
            .exec(Command::Why {
                claim: "sr.invariant-interval".into(),
            })
            .text()
            .to_string();
        let ib = why_theory_block(&interval, "special-relativity");
        assert!(ib.contains("quantifier: forall"), "{ib}");
        assert!(ib.contains("c=1"), "{ib}");
        assert!(ib.contains("minkowski-mostly-minus"), "{ib}");
        assert!(ib.contains("|β| < 1"), "{ib}");
        assert!(ib.contains("1+1 Minkowski"), "{ib}");
        assert!(ib.contains("minkowski-interval-signature"), "{ib}");
    }

    #[test]
    fn slug_receipt_is_not_p3f_for_a_changed_identity() {
        let mut lab = Lab::standard();
        let stale = physis_core::claim::Claim::new(
            "dec.d-squared-zero",
            "The exterior derivative is nilpotent: d ∘ d = 0.",
            LayerId::Mathematical,
            physis_core::ClaimClass::Mathematical,
        );
        let live = lab.find_claim("dec.d-squared-zero").unwrap();
        assert_ne!(
            stale.statement_hash(),
            live.statement_hash(),
            "physlib forall must not be the unspecified default identity"
        );
        let challenge = Challenge::generate(&FormalClaim::from_claim(&stale));
        let err = verify(&challenge, &UntrustedProof::ExactIdentity).unwrap_err();
        assert_eq!(err, physis_verifier::VerifyError::NoExactIdentity);
        assert!(lab.receipts.by_statement(stale.statement_hash()).is_none());
        assert!(lab.receipts.by_statement(live.statement_hash()).is_none());

        let why = lab
            .exec(Command::Why {
                claim: "dec.d-squared-zero".into(),
            })
            .text()
            .to_string();
        let d2 = why_theory_block(&why, "de-rham");
        assert!(d2.contains("judgment:   logical undetermined"), "{d2}");
        assert!(!d2.contains("P3F"), "{d2}");
        assert!(d2.contains("kernel proof: none"), "{d2}");

        let p3f = lab
            .exec(Command::Inspect {
                axis: Some("trust".into()),
                value: Some("P3F".into()),
            })
            .text()
            .to_string();
        assert!(
            !p3f.contains("dec.d-squared-zero"),
            "stale slug receipt must not inspect as P3F: {p3f}"
        );

        lab.exec(Command::Prove {
            claim: "dec.d-squared-zero".into(),
        });
        let why2 = lab
            .exec(Command::Why {
                claim: "dec.d-squared-zero".into(),
            })
            .text()
            .to_string();
        let d2b = why_theory_block(&why2, "de-rham");
        assert!(d2b.contains("judgment:   logical proved"), "{d2b}");
        assert!(d2b.contains("P3F"), "{d2b}");
    }

    #[test]
    fn live_catalog_claims_are_the_catalog_identities() {
        let lab = Lab::standard();
        for spec in CATALOG {
            let live = lab.find_claim(spec.claim_id).unwrap();
            assert_eq!(
                live.statement_hash(),
                spec.formal_claim().statement_hash(),
                "{} live hash must be the catalog FormalClaim",
                spec.claim_id
            );
        }
    }

    #[test]
    fn slug_review_is_not_p3s_for_a_changed_identity() {
        let mut lab = Lab::standard();
        let stale = physis_core::claim::Claim::new(
            "dec.d-squared-zero",
            "The exterior derivative is nilpotent: d ∘ d = 0.",
            LayerId::Mathematical,
            physis_core::ClaimClass::Mathematical,
        );
        let live = lab.find_claim("dec.d-squared-zero").unwrap();
        assert_ne!(
            stale.statement_hash(),
            live.statement_hash(),
            "physlib forall must not be the unspecified default identity"
        );
        let err = physis_semantic::review(&FormalClaim::from_claim(&stale)).unwrap_err();
        assert!(
            err.to_string().contains("catalog identity does not match"),
            "{err}"
        );
        assert!(lab.reviews.by_statement(stale.statement_hash()).is_none());
        assert!(lab.reviews.by_statement(live.statement_hash()).is_none());

        let why = lab
            .exec(Command::Why {
                claim: "dec.d-squared-zero".into(),
            })
            .text()
            .to_string();
        let d2 = why_theory_block(&why, "de-rham");
        assert!(d2.contains("semantic:   unreviewed"), "{d2}");
        assert!(!d2.contains("P3S"), "{d2}");

        let p3s = lab
            .exec(Command::Inspect {
                axis: Some("trust".into()),
                value: Some("P3S".into()),
            })
            .text()
            .to_string();
        assert!(
            !p3s.contains("dec.d-squared-zero"),
            "stale slug review must not inspect as P3S: {p3s}"
        );

        lab.exec(Command::Review {
            claim: "dec.d-squared-zero".into(),
        });
        let why2 = lab
            .exec(Command::Why {
                claim: "dec.d-squared-zero".into(),
            })
            .text()
            .to_string();
        let d2b = why_theory_block(&why2, "de-rham");
        assert!(d2b.contains("semantic:   adversarially-reviewed"), "{d2b}");
        assert!(d2b.contains("P3S"), "{d2b}");
        assert!(lab.reviews.by_statement(live.statement_hash()).is_some());
        assert!(lab.reviews.by_statement(stale.statement_hash()).is_none());
    }

    #[test]
    fn no_lab_verdict_is_a_kernel_proof() {
        let lab = Lab::standard();
        for id in lab.theory_ids() {
            let t = lab.theory(&id).unwrap();
            for (c, v) in t.evaluate_all() {
                assert!(
                    matches!(
                        v.derivation(),
                        physis_core::DerivationAssurance::Asserted
                            | physis_core::DerivationAssurance::Executed
                            | physis_core::DerivationAssurance::CrossChecked
                            | physis_core::DerivationAssurance::CertifiedNumeric
                    ),
                    "{} / {} derivation {:?}",
                    id,
                    c.id_str(),
                    v.derivation()
                );
                assert_eq!(v.semantic(), physis_core::SemanticAssurance::Unreviewed);
                assert!(!c.assumptions().items.is_empty());
            }
        }
    }

    #[test]
    fn inspect_p3s_is_empty_until_review() {
        let mut lab = Lab::standard();
        let before = lab
            .exec(Command::Inspect {
                axis: Some("trust".into()),
                value: Some("P3S".into()),
            })
            .text()
            .to_string();
        assert!(
            before.lines().any(|l| l.trim() == "count 0"),
            "evaluator semantic must not mint P3S: {before}"
        );
        assert!(!before.contains("dec.d-squared-zero"), "{before}");
        lab.exec(Command::Review {
            claim: "dec.d-squared-zero".into(),
        });
        let after = lab
            .exec(Command::Inspect {
                axis: Some("trust".into()),
                value: Some("P3S".into()),
            })
            .text()
            .to_string();
        assert!(
            after
                .lines()
                .any(|l| l.contains("de-rham") && l.contains("dec.d-squared-zero")),
            "{after}"
        );
        assert!(after.lines().any(|l| l.trim() == "count 1"), "{after}");
    }

    #[test]
    fn turning_planck_quantum_off_restores_the_uv_catastrophe() {
        let mut lab = Lab::standard();
        let diffs = lab.set_knob("planck", "quantum", "false").unwrap().2;
        assert!(
            diffs.iter().any(|d| d.claim == "thermo.uv-finite"
                && d.from == VerdictKind::Holds
                && d.to == VerdictKind::Fails),
            "expected uv-finite Holds→Fails, got {diffs:?}"
        );
        assert!(
            diffs.iter().any(|d| d.claim == "thermo.mode-equipartition"
                && d.from == VerdictKind::Fails
                && d.to == VerdictKind::Holds),
            "expected mode-equipartition Fails→Holds, got {diffs:?}"
        );
    }

    #[test]
    fn raising_einstein_temperature_recovers_dulong_petit() {
        let mut lab = Lab::standard();
        let diffs = lab
            .set_knob("einstein-solid", "temperature", "4000")
            .unwrap()
            .2;
        assert!(
            diffs.iter().any(|d| d.claim == "thermo.dulong-petit"
                && d.from == VerdictKind::Fails
                && d.to == VerdictKind::Holds),
            "expected dulong-petit Fails→Holds, got {diffs:?}"
        );
    }

    #[test]
    fn turning_einstein_spectrum_to_debye_flips_t3() {
        let mut lab = Lab::standard();
        let diffs = lab
            .set_knob("einstein-solid", "spectrum", "debye")
            .unwrap()
            .2;
        assert!(
            diffs.iter().any(|d| d.claim == "thermo.debye-t3"
                && d.from == VerdictKind::Fails
                && d.to == VerdictKind::Holds),
            "expected debye-t3 Fails→Holds, got {diffs:?}"
        );
    }

    #[test]
    fn turning_de_rham_to_sphere_gains_a_fundamental_class() {
        let mut lab = Lab::standard();
        let diffs = lab.set_knob("de-rham", "shape", "sphere").unwrap().2;
        assert!(
            diffs.iter().any(|d| d.claim == "dec.fundamental-class"
                && d.from == VerdictKind::Fails
                && d.to == VerdictKind::Holds),
            "expected fundamental-class Fails→Holds, got {diffs:?}"
        );
        assert!(
            !diffs.iter().any(|d| d.claim == "dec.closed-equals-exact"),
            "S² shares b₁ = 0 with the disk; Poincaré must not flip, got {diffs:?}"
        );
    }

    #[test]
    fn raising_gr_dimension_makes_solar_tests_inapplicable() {
        let mut lab = Lab::standard();
        let diffs = lab.set_knob("general-relativity", "dim", "5").unwrap().2;
        assert!(
            diffs.iter().any(|d| d.claim == "gr.eddington-deflection"
                && d.from == VerdictKind::Holds
                && d.to == VerdictKind::Inapplicable),
            "expected eddington Holds→Inapplicable, got {diffs:?}"
        );
    }

    #[test]
    fn turning_on_susy_flips_gqw_weinberg_angle() {
        let mut lab = Lab::standard();
        let diffs = lab.set_knob("su5-gut", "supersymmetric", "true").unwrap().2;
        assert!(
            diffs.iter().any(|d| d.claim == "gut.weinberg-angle-mz"
                && d.from == VerdictKind::Fails
                && d.to == VerdictKind::Holds),
            "expected weinberg-angle-mz Fails→Holds, got {diffs:?}"
        );
        assert!(
            diffs
                .iter()
                .any(|d| d.claim == "gut.weinberg-angle-mz-interval"
                    && d.from == VerdictKind::Fails
                    && d.to == VerdictKind::Undecidable),
            "expected weinberg-angle-mz-interval Fails→Undecidable, got {diffs:?}"
        );
        assert!(
            diffs.iter().any(|d| d.claim == "gut.proton-lifetime-sk"
                && d.from == VerdictKind::Fails
                && d.to == VerdictKind::Holds
                && d.from_empirical.as_deref() == Some("excluded")
                && d.to_empirical.as_deref() == Some("compatible")),
            "expected proton-lifetime-sk excluded→compatible, got {diffs:?}"
        );
        assert!(
            !diffs
                .iter()
                .any(|d| d.claim == "gut.weinberg-angle" || d.claim == "gut.charge-quantization"),
            "GUT-scale 3/8 and Tr Q must not move with the SUSY knob: {diffs:?}"
        );
        let why_mz = lab
            .exec(Command::Why {
                claim: "gut.weinberg-angle-mz".into(),
            })
            .text()
            .to_string();
        assert!(why_mz.contains("verdict:    holds"), "{why_mz}");
        assert!(why_mz.contains("derivation: asserted"), "{why_mz}");
        assert!(
            !why_mz.contains("derivation: certified-numeric"),
            "a 3% GQW hit is not P3N: {why_mz}"
        );
        let p3n = lab
            .exec(Command::Inspect {
                axis: Some("trust".into()),
                value: Some("P3N".into()),
            })
            .text()
            .to_string();
        assert!(
            !p3n.contains("gut.weinberg-angle-mz"),
            "SUSY GQW Holds must not mint P3N: {p3n}"
        );
        assert!(
            p3n.lines().any(|l| l.trim() == "count 4"),
            "P3N stays three SM cells plus GUT-scale 3/8: {p3n}"
        );
    }

    #[test]
    fn susy_gqw_interval_is_insufficient_precision() {
        let mut lab = Lab::standard();
        let class = lab
            .exec(Command::Inspect {
                axis: Some("class".into()),
                value: Some("empirical-prediction".into()),
            })
            .text()
            .to_string();
        assert!(class.contains("gut.weinberg-angle-mz-interval"), "{class}");
        assert!(class.contains("gut.proton-lifetime-sk"), "{class}");

        let before = lab
            .exec(Command::Inspect {
                axis: Some("gap".into()),
                value: Some("insufficient-precision".into()),
            })
            .text()
            .to_string();
        assert!(
            before.contains("count 0"),
            "minimal SU(5) is excluded by the PDG hull, not too coarse: {before}"
        );

        let why_min = lab
            .exec(Command::Why {
                claim: "gut.weinberg-angle-mz-interval".into(),
            })
            .text()
            .to_string();
        assert!(why_min.contains("empirical:  excluded"), "{why_min}");
        assert!(
            why_min.contains("judgment:   statistical computed"),
            "{why_min}"
        );
        assert!(why_min.contains("nll:"), "{why_min}");
        assert!(
            !why_min.contains("certified-numeric"),
            "GQW NLL is not P3N: {why_min}"
        );

        lab.set_knob("su5-gut", "supersymmetric", "true").unwrap();
        let after = lab
            .exec(Command::Inspect {
                axis: Some("gap".into()),
                value: Some("insufficient-precision".into()),
            })
            .text()
            .to_string();
        assert!(after.contains("gut.weinberg-angle-mz-interval"), "{after}");
        assert!(after.contains("count 1"), "{after}");

        let gaps = lab.exec(Command::Gaps).text().to_string();
        assert!(
            gaps.lines()
                .any(|l| l.contains("gut.weinberg-angle-mz-interval")
                    && l.contains("tighter-enclosure")),
            "{gaps}"
        );

        let why = lab
            .exec(Command::Why {
                claim: "gut.weinberg-angle-mz-interval".into(),
            })
            .text()
            .to_string();
        assert!(why.contains("empirical:  inconclusive"), "{why}");
        assert!(why.contains("judgment:   statistical computed"), "{why}");
        assert!(why.contains("nll:"), "{why}");
        let folklore = lab
            .exec(Command::Why {
                claim: "gut.weinberg-angle-mz".into(),
            })
            .text()
            .to_string();
        assert!(folklore.contains("class:      heuristic"), "{folklore}");
    }

    #[test]
    fn proton_lifetime_sk_is_compared_to_super_k() {
        let mut lab = Lab::standard();
        let missing = lab
            .exec(Command::Inspect {
                axis: Some("gap".into()),
                value: Some("missing-dataset".into()),
            })
            .text()
            .to_string();
        assert!(
            !missing.contains("gut.proton-lifetime-sk"),
            "Super-K is a Dataset; this cell is decided: {missing}"
        );
        assert!(
            !missing.contains("gut.weinberg-angle-mz-interval"),
            "the GQW interval cell has a PDG dataset: {missing}"
        );

        let why = lab
            .exec(Command::Why {
                claim: "gut.proton-lifetime-sk".into(),
            })
            .text()
            .to_string();
        assert!(why.contains("class:      empirical-prediction"), "{why}");
        assert!(why.contains("empirical:  excluded"), "{why}");
        assert!(why.contains("judgment:   empirical excluded"), "{why}");
        assert!(why.contains("derivation: executed"), "{why}");
        assert!(
            !why.contains("derivation: certified-numeric"),
            "dim-6 scaling is not P3N: {why}"
        );

        lab.set_knob("su5-gut", "supersymmetric", "true").unwrap();
        let after = lab
            .exec(Command::Inspect {
                axis: Some("gap".into()),
                value: Some("missing-dataset".into()),
            })
            .text()
            .to_string();
        assert!(
            !after.contains("gut.proton-lifetime-sk"),
            "SUSY dim-6 compatible is a decision, not a missing dataset: {after}"
        );
        let why_susy = lab
            .exec(Command::Why {
                claim: "gut.proton-lifetime-sk".into(),
            })
            .text()
            .to_string();
        assert!(why_susy.contains("empirical:  compatible"), "{why_susy}");
        assert!(
            why_susy.contains("judgment:   empirical compatible"),
            "{why_susy}"
        );
        let p3n = lab
            .exec(Command::Inspect {
                axis: Some("trust".into()),
                value: Some("P3N".into()),
            })
            .text()
            .to_string();
        assert!(
            !p3n.contains("gut.proton-lifetime-sk"),
            "Super-K compatible is not P3N: {p3n}"
        );
        assert!(
            p3n.lines().any(|l| l.trim() == "count 4"),
            "P3N stays three SM cells plus GUT-scale 3/8: {p3n}"
        );
    }

    #[test]
    fn coarse_lattice_is_insufficient_precision_not_a_failed_theorem() {
        let mut lab = Lab::standard();
        let before = lab
            .exec(Command::Inspect {
                axis: Some("gap".into()),
                value: Some("insufficient-precision".into()),
            })
            .text()
            .to_string();
        assert!(
            !before.contains("field.second-order-accurate"),
            "default spacing resolves the probe: {before}"
        );
        let why_ok = lab
            .exec(Command::Why {
                claim: "field.second-order-accurate".into(),
            })
            .text()
            .to_string();
        assert!(why_ok.contains("verdict:    holds"), "{why_ok}");
        assert!(why_ok.contains("derivation: executed"), "{why_ok}");
        assert!(why_ok.contains("|k a| < 1"), "{why_ok}");
        assert!(
            why_ok.contains("judgment:   logical undetermined"),
            "{why_ok}"
        );
        assert!(
            !why_ok.contains("derivation: certified-numeric"),
            "the 1.8–2.2 f64 window is not P3N: {why_ok}"
        );
        assert!(!why_ok.contains("numeric certified"), "{why_ok}");
        assert!(
            why_ok.contains("|k a| < 1"),
            "domain must name the long-wavelength regime: {why_ok}"
        );

        let diffs = lab.set_knob("klein-gordon", "spacing", "100").unwrap().2;
        let order = diffs
            .iter()
            .find(|d| d.claim == "field.second-order-accurate")
            .expect("second-order row");
        assert_eq!(order.from, VerdictKind::Holds);
        assert_eq!(order.to, VerdictKind::Undecidable);
        assert_eq!(order.from_empirical.as_deref(), Some("not-applicable"));
        assert_eq!(order.to_empirical.as_deref(), Some("inconclusive"));
        assert_eq!(order.from_judgment.as_deref(), Some("logical undetermined"));
        assert_eq!(order.to_judgment.as_deref(), Some("numeric unresolved"));
        assert!(
            !diffs
                .iter()
                .any(|d| d.claim == "field.second-order-accurate" && d.to == VerdictKind::Fails),
            "too coarse is not a failed stencil: {diffs:?}"
        );

        let gap = lab
            .exec(Command::Inspect {
                axis: Some("gap".into()),
                value: Some("insufficient-precision".into()),
            })
            .text()
            .to_string();
        assert!(
            gap.lines()
                .any(|l| l.contains("klein-gordon") && l.contains("field.second-order-accurate")),
            "{gap}"
        );
        let why = lab
            .exec(Command::Why {
                claim: "field.second-order-accurate".into(),
            })
            .text()
            .to_string();
        assert!(why.contains("verdict:    undecidable"), "{why}");
        assert!(why.contains("empirical:  inconclusive"), "{why}");
        assert!(why.contains("judgment:   numeric unresolved"), "{why}");
        assert!(why.contains("derivation: executed"), "{why}");
        assert!(!why.contains("P3N"), "{why}");
        assert!(why.contains("kernel proof: none"), "{why}");
    }

    #[test]
    fn long_wavelength_cells_name_a_domain() {
        let mut lab = Lab::standard();
        let disp = lab
            .exec(Command::Why {
                claim: "field.dispersion-continuum-limit".into(),
            })
            .text()
            .to_string();
        let db = why_theory_block(&disp, "klein-gordon");
        assert!(db.contains("longest non-zero lattice mode"), "{db}");
        assert!(
            !db.contains("not yet a machine-checked regime"),
            "dispersion must not be encoding-wide: {db}"
        );
        assert!(
            !db.contains("|k a| < 1 at the Richardson probe"),
            "dispersion is not the Richardson cell: {db}"
        );

        let order = lab
            .exec(Command::Why {
                claim: "field.second-order-accurate".into(),
            })
            .text()
            .to_string();
        let ob = why_theory_block(&order, "klein-gordon");
        assert!(ob.contains("|k a| < 1"), "{ob}");
        assert!(
            !ob.contains("longest non-zero lattice mode"),
            "Richardson probe is not the longest-mode cell: {ob}"
        );

        let qs = lab
            .exec(Command::Why {
                claim: "em.quasi-static-valid".into(),
            })
            .text()
            .to_string();
        let ohm = why_theory_block(&qs, "ohm-circuit");
        assert!(ohm.contains("λ > 100"), "{ohm}");
        assert!(
            !ohm.contains("not yet a machine-checked regime"),
            "ohm-circuit quasi-static must name λ >> circuit: {ohm}"
        );
        let mx = why_theory_block(&qs, "maxwell-vacuum");
        assert!(
            mx.contains("not yet a machine-checked regime"),
            "Maxwell's inapplicable copy stays encoding-wide: {mx}"
        );
    }

    #[test]
    fn bounding_the_tape_does_not_make_search_feasible() {
        let mut lab = Lab::standard();
        let diffs = lab
            .set_knob("turing-machine", "tape_bound", "1000")
            .unwrap()
            .2;
        assert!(
            diffs.iter().any(|d| d.claim == "comp.halts"
                && d.from == VerdictKind::Undecidable
                && d.to == VerdictKind::Holds),
            "expected halts Undecidable→Holds, got {diffs:?}"
        );
        assert!(
            diffs.iter().any(|d| d.claim == "comp.feasible-decision"
                && d.from == VerdictKind::Inapplicable
                && d.to == VerdictKind::Undecidable),
            "expected feasible-decision Inapplicable→Undecidable, got {diffs:?}"
        );
        let expensive = lab
            .exec(Command::Inspect {
                axis: Some("gap".into()),
                value: Some("computationally-intractable".into()),
            })
            .text()
            .to_string();
        assert!(
            expensive
                .lines()
                .any(|l| l.contains("turing-machine") && l.contains("comp.feasible-decision")),
            "{expensive}"
        );
        let why = lab
            .exec(Command::Why {
                claim: "comp.feasible-decision".into(),
            })
            .text()
            .to_string();
        assert!(why.contains("turing-machine"), "{why}");
        assert!(why.contains("undecidable"), "{why}");
    }

    #[test]
    fn finite_age_flips_olbers_catastrophe() {
        let mut lab = Lab::standard();
        let diffs = lab
            .set_knob("olbers-static", "finite_age", "true")
            .unwrap()
            .2;
        assert!(
            diffs.iter().any(|d| d.claim == "astro.sky-finite"
                && d.from == VerdictKind::Fails
                && d.to == VerdictKind::Holds),
            "expected sky-finite Fails→Holds, got {diffs:?}"
        );
        assert!(
            diffs.iter().any(|d| d.claim == "astro.night-sky-dark"
                && d.from == VerdictKind::Fails
                && d.to == VerdictKind::Holds),
            "expected night-sky-dark Fails→Holds, got {diffs:?}"
        );
        assert!(
            !diffs.iter().any(|d| d.claim == "astro.shell-cancellation"),
            "finite age must not touch shell cancellation, got {diffs:?}"
        );
    }

    #[test]
    fn expanding_flips_olbers_cancellation_and_saves_the_sky() {
        let mut lab = Lab::standard();
        let diffs = lab
            .set_knob("olbers-static", "expanding", "true")
            .unwrap()
            .2;
        assert!(
            diffs.iter().any(|d| d.claim == "astro.shell-cancellation"
                && d.from == VerdictKind::Holds
                && d.to == VerdictKind::Fails),
            "expected shell-cancellation Holds→Fails, got {diffs:?}"
        );
        assert!(
            diffs.iter().any(|d| d.claim == "astro.sky-finite"
                && d.from == VerdictKind::Fails
                && d.to == VerdictKind::Holds),
            "expected sky-finite Fails→Holds, got {diffs:?}"
        );
        assert!(
            diffs.iter().any(|d| d.claim == "astro.night-sky-dark"
                && d.from == VerdictKind::Fails
                && d.to == VerdictKind::Holds),
            "expected night-sky-dark Fails→Holds, got {diffs:?}"
        );
    }

    #[test]
    fn response_serializes_to_json_for_agents() {
        let mut lab = Lab::standard();
        let resp = lab.exec(Command::Experiment {
            id: "string-critique".into(),
        });
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("\"status\":\"ok\""));
        assert!(json.contains("matrix"));
        // A knob turn exposes structured diffs.
        let set = lab.exec(Command::Set {
            theory: "type-iib".into(),
            knob: "total_dim".into(),
            value: "9".into(),
        });
        let json = serde_json::to_string(&set).unwrap();
        assert!(json.contains("consistency.critical-dimension"));
    }

    #[test]
    fn all_listed_experiments_are_runnable() {
        let mut lab = Lab::standard();
        for (id, _) in EXPERIMENTS {
            assert!(
                lab.experiment_canonical(id).is_ok(),
                "listed experiment '{id}' should run"
            );
        }
        let text = Lab::standard()
            .exec(Command::Experiments)
            .text()
            .to_string();
        for (id, _) in EXPERIMENTS {
            assert!(text.contains(id), "experiments list should mention '{id}'");
        }
    }

    fn temp_path(tag: &str) -> std::path::PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        std::env::temp_dir().join(format!("physis_{tag}_{}_{nanos}.jsonl", std::process::id()))
    }

    #[test]
    fn replay_command_rejects_malformed_journal() {
        let path = temp_path("malformed");
        std::fs::write(
            &path,
            "this is not json\n{\"event\":\"boot\",\"t\":1,\"theories\":[]}\n",
        )
        .unwrap();
        let mut lab = Lab::standard();
        let resp = lab.exec(Command::Replay {
            path: path.to_string_lossy().into_owned(),
        });
        assert_eq!(resp.exit_code(), 1, "malformed journal must not certify");
        assert!(resp.text().contains("malformed"));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn replay_command_rejects_journal_with_no_turns() {
        let path = temp_path("noturns");
        // Only a boot event — nothing to verify.
        std::fs::write(&path, "{\"event\":\"boot\",\"t\":1,\"theories\":[]}\n").unwrap();
        let mut lab = Lab::standard();
        let resp = lab.exec(Command::Replay {
            path: path.to_string_lossy().into_owned(),
        });
        assert_eq!(resp.exit_code(), 1, "empty session must not certify");
        assert!(resp.text().contains("no set-knob events"));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn prove_d2_mints_a_receipt_and_why_shows_it() {
        let mut lab = Lab::standard();
        let text = lab
            .exec(Command::Prove {
                claim: "dec.d-squared-zero".into(),
            })
            .text()
            .to_string();
        if physis_verifier::discover_tools().is_some() {
            assert!(text.contains("lean-kernel"), "{text}");
            assert!(text.contains("nanoda"), "{text}");
        } else {
            assert!(text.contains("expand-recursive"), "{text}");
            assert!(text.contains("expand-postfix"), "{text}");
        }
        let why = lab
            .exec(Command::Why {
                claim: "dec.d-squared-zero".into(),
            })
            .text()
            .to_string();
        assert!(why.contains("kernel proof: receipt"), "{why}");
        assert!(why.contains("judgment:   logical proved"), "{why}");
        assert!(why.contains("P3F"), "{why}");
        assert!(why.contains("unreviewed encoding is dangerous"), "{why}");
        assert!(why.contains("axiom closure:"), "{why}");
        assert!(why.contains("discrete-coboundary"), "{why}");
        assert!(why.contains("[model-assumption]"), "{why}");
        assert!(why.contains("quantifier: forall"), "{why}");
        assert!(why.contains("physlib:unversioned"), "{why}");
        let epi = lab.exec(Command::Epistemics).text().to_string();
        assert!(epi.contains("machine-proved          1"), "{epi}");
    }

    #[test]
    fn prove_conjecture_is_refused() {
        let mut lab = Lab::standard();
        let resp = lab.exec(Command::Prove {
            claim: "predictivity.unique-vacuum".into(),
        });
        assert_eq!(resp.exit_code(), 1);
        assert!(resp.text().contains("no exact identity"));
    }

    #[test]
    fn prove_restores_by_reverify_not_by_deserialize() {
        let mut lab = Lab::standard();
        lab.exec(Command::Prove {
            claim: "dec.d-squared-zero".into(),
        });
        let live = lab.find_claim("dec.d-squared-zero").unwrap();
        let jsonl = lab.journal().to_string();
        assert!(jsonl.contains("\"event\":\"prove\""));
        assert!(
            jsonl.contains(&format!("\"statement_hash\":\"{}\"", live.statement_hash())),
            "{jsonl}"
        );
        let mut lab2 = Lab::standard();
        *lab2.journal_mut() = Journal::from_jsonl(&jsonl);
        lab2.restore_from_journal();
        let why = lab2
            .exec(Command::Why {
                claim: "dec.d-squared-zero".into(),
            })
            .text()
            .to_string();
        assert!(why.contains("kernel proof: receipt"), "{why}");
        assert!(why.contains("P3F"), "{why}");
    }

    #[test]
    fn journal_prove_of_a_stale_identity_is_not_p3f() {
        let mut lab = Lab::standard();
        let stale = physis_core::claim::Claim::new(
            "dec.d-squared-zero",
            "The exterior derivative is nilpotent: d ∘ d = 0.",
            LayerId::Mathematical,
            ClaimClass::Mathematical,
        );
        let live = lab.find_claim("dec.d-squared-zero").unwrap();
        assert_ne!(stale.statement_hash(), live.statement_hash());
        let challenge = Challenge::generate(&FormalClaim::from_claim(&stale));
        let jsonl = format!(
            r#"{{"event":"prove","t":1,"claim":"dec.d-squared-zero","challenge_hash":"{}","statement_hash":"{}"}}"#,
            challenge.challenge_hash(),
            stale.statement_hash(),
        );
        *lab.journal_mut() = Journal::from_jsonl(&jsonl);
        lab.restore_from_journal();
        assert!(lab.receipts.by_statement(live.statement_hash()).is_none());
        let why = lab
            .exec(Command::Why {
                claim: "dec.d-squared-zero".into(),
            })
            .text()
            .to_string();
        let block = why_theory_block(&why, "de-rham");
        assert!(block.contains("kernel proof: none"), "{block}");
        assert!(!block.contains("P3F"), "{block}");
    }

    #[test]
    fn journal_prove_with_wrong_challenge_is_not_p3f() {
        let mut lab = Lab::standard();
        let live = lab.find_claim("dec.d-squared-zero").unwrap();
        let jsonl = format!(
            r#"{{"event":"prove","t":1,"claim":"dec.d-squared-zero","challenge_hash":"{}","statement_hash":"{}"}}"#,
            "0".repeat(64),
            live.statement_hash(),
        );
        *lab.journal_mut() = Journal::from_jsonl(&jsonl);
        lab.restore_from_journal();
        assert!(lab.receipts.by_statement(live.statement_hash()).is_none());
        let why = lab
            .exec(Command::Why {
                claim: "dec.d-squared-zero".into(),
            })
            .text()
            .to_string();
        let block = why_theory_block(&why, "de-rham");
        assert!(block.contains("kernel proof: none"), "{block}");
        assert!(!block.contains("P3F"), "{block}");
    }

    #[test]
    fn falsify_critical_dimension_finds_total_dim() {
        let mut lab = Lab::standard();
        let text = lab
            .exec(Command::Falsify {
                claim: "consistency.critical-dimension".into(),
            })
            .text()
            .to_string();
        assert!(text.contains("counterexample"), "{text}");
        assert!(text.contains("total_dim"), "{text}");
    }

    #[test]
    fn hypothesize_mutates_chosen_not_measured() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        let iib = lab
            .exec(Command::Hypothesize {
                theory: Some("type-iib".into()),
            })
            .text()
            .to_string();
        assert!(iib.contains("constrained structural mutation"), "{iib}");
        assert!(
            iib.contains("total_dim") && iib.contains("consistency.critical-dimension"),
            "{iib}"
        );
        assert!(
            iib.contains("logical undetermined → logical disproved"),
            "scientific-axis judgment must be in the hypothesis: {iib}"
        );
        assert!(
            !iib.contains("observed_dim"),
            "measured observed_dim is not a structural hypothesis: {iib}"
        );
        assert_eq!(
            lab.theory("type-iib")
                .unwrap()
                .get("total_dim")
                .unwrap()
                .display(),
            "10",
            "hypothesize must restore knobs"
        );
        assert_eq!(
            lab.journal().len(),
            journal_len,
            "hypothesize must not journal"
        );

        let sm = lab
            .exec(Command::Hypothesize {
                theory: Some("standard-model".into()),
            })
            .text()
            .to_string();
        assert!(
            !sm.contains("generations"),
            "measured generations is nature, not a hypothesis: {sm}"
        );
        assert!(
            sm.contains("chosen structural") || sm.contains("include_"),
            "{sm}"
        );

        let kg = lab
            .exec(Command::Hypothesize {
                theory: Some("klein-gordon".into()),
            })
            .text()
            .to_string();
        assert!(
            kg.contains("field.second-order-accurate") && kg.contains("numeric unresolved"),
            "coarse spacing is inconclusive, not a failed theorem: {kg}"
        );
        assert!(
            !kg.contains("theorem"),
            "hypothesize must not print a theorem tag: {kg}"
        );
        assert_eq!(
            lab.theory("klein-gordon")
                .unwrap()
                .get("spacing")
                .unwrap()
                .display(),
            "1"
        );

        let unknown = lab.exec(Command::Hypothesize {
            theory: Some("no-such-theory".into()),
        });
        assert_eq!(unknown.exit_code(), 1);
    }

    #[test]
    fn hypothesize_circuit_feedback_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        let blocked = lab.exec(Command::Set {
            theory: "combinational-circuit".into(),
            knob: "feedback".into(),
            value: "true".into(),
        });
        assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
        assert!(
            blocked.text().contains("unknown knob") || blocked.text().contains("feedback"),
            "{}",
            blocked.text()
        );

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("combinational-circuit".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-feedback") && text.contains("ir structural"),
            "{text}"
        );
        assert!(
            text.contains("comp.acyclic") && text.contains("holds → fails"),
            "{text}"
        );
        assert!(
            text.contains("comp.halts") && text.contains("holds → inapplicable"),
            "{text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("combinational-circuit").unwrap();
        assert!(
            live.evaluate_all()
                .iter()
                .any(|(c, v)| c.id_str() == "comp.acyclic" && v.kind == VerdictKind::Holds),
            "IR mutant must not be installed"
        );
    }

    #[test]
    fn evidence_graph_separates_encodings_from_evaluations() {
        let mut lab = Lab::standard();
        let uniq = lab
            .exec(Command::Evidence {
                claim: "predictivity.unique-vacuum".into(),
            })
            .text()
            .to_string();
        assert!(
            uniq.contains("evidence  predictivity.unique-vacuum"),
            "{uniq}"
        );
        assert!(
            uniq.lines().next().unwrap_or("").contains("graph "),
            "evidence must print a store graph id: {uniq}"
        );
        assert!(
            uniq.contains("distinct FormalClaims share this slug"),
            "unique-vacuum is not one identity: {uniq}"
        );
        assert!(uniq.contains("competing encodings: yes"), "{uniq}");
        assert!(
            uniq.contains("type-iib") && uniq.contains("fails"),
            "{uniq}"
        );
        assert!(
            uniq.contains("observer-geometry") && uniq.contains("holds"),
            "{uniq}"
        );
        assert!(
            uniq.contains("derived from TrustProfile") && uniq.contains("no numeric score"),
            "{uniq}"
        );
        assert!(
            uniq.contains("not Canonical") && uniq.contains("not P4"),
            "{uniq}"
        );
        assert!(
            !uniq.contains("0.95") && !uniq.contains("%"),
            "confidence is not a invented number: {uniq}"
        );
        assert!(!uniq.contains("theorem"), "{uniq}");
        assert!(
            uniq.contains("flux/moduli landscape"),
            "string unique-vacuum must name the landscape: {uniq}"
        );
        assert!(
            uniq.contains("unique_vacuum program axiom"),
            "observer-geometry unique-vacuum must name the axiom: {uniq}"
        );
        assert!(
            uniq.contains("Einstein-Hilbert") && uniq.contains("Higgs vacuum"),
            "GR and SM unique-vacuum must name their regimes: {uniq}"
        );
        assert!(
            !uniq.contains("encoding-wide"),
            "unique-vacuum encodings name regimes, not the placeholder: {uniq}"
        );

        let qs = lab
            .exec(Command::Evidence {
                claim: "em.quasi-static-valid".into(),
            })
            .text()
            .to_string();
        assert!(
            qs.contains("distinct FormalClaims share this slug"),
            "ohm-circuit and Maxwell are different identities: {qs}"
        );
        assert!(qs.contains("ohm-circuit"), "{qs}");
        assert!(qs.contains("maxwell-vacuum"), "{qs}");
        assert!(
            qs.contains("encoding-wide") && qs.contains("λ > 100"),
            "Maxwell stays encoding-wide; ohm names λ: {qs}"
        );

        let sk = lab
            .exec(Command::Evidence {
                claim: "gut.proton-lifetime-sk".into(),
            })
            .text()
            .to_string();
        assert!(
            sk.contains("this slug is one FormalClaim"),
            "Super-K lives on su5-gut only: {sk}"
        );
        assert!(sk.contains("competing encodings: no"), "{sk}");
        assert!(sk.contains("p→e+π0"), "{sk}");
        assert!(!sk.contains("encoding-wide"), "{sk}");
        assert!(sk.contains("excluded"), "{sk}");
        assert!(sk.contains("empirical excluded"), "{sk}");
        assert!(
            sk.contains("not Canonical") && sk.contains("not P4"),
            "{sk}"
        );
        assert!(!sk.contains("theorem"), "{sk}");

        let unknown = lab.exec(Command::Evidence {
            claim: "no.such.claim".into(),
        });
        assert_eq!(unknown.exit_code(), 1);
    }

    fn evidence_graph_id(text: &str) -> physis_core::artifact::ArtifactId {
        let line = text.lines().next().expect("empty evidence");
        let hex = line.split_whitespace().last().expect("graph hex");
        physis_core::artifact::ArtifactId::from_hex(hex)
            .unwrap_or_else(|| panic!("expected 64 hex graph id in {line}"))
    }

    fn unique_vacuum_statement(lab: &Lab, theory: &str) -> physis_core::artifact::ArtifactId {
        let (c, _) = lab
            .theory(theory)
            .unwrap()
            .evaluate_all()
            .into_iter()
            .find(|(c, _)| c.id_str() == physis_theory::claims::UNIQUE_VACUUM)
            .unwrap_or_else(|| panic!("{theory} unique-vacuum"));
        Node::new(
            NodeKind::Statement,
            vec![],
            c.statement_hash().to_hex().as_bytes(),
        )
        .id
    }

    fn store_kind<'a>(
        lab: &'a Lab,
        ids: impl IntoIterator<Item = &'a physis_core::artifact::ArtifactId>,
        kind: NodeKind,
    ) -> Vec<physis_core::artifact::ArtifactId> {
        ids.into_iter()
            .copied()
            .filter(|id| lab.store.get(*id).is_some_and(|n| n.kind == kind))
            .collect()
    }

    #[test]
    fn evidence_graph_is_a_store_dag() {
        let mut lab = Lab::standard();
        let uniq = lab
            .exec(Command::Evidence {
                claim: "predictivity.unique-vacuum".into(),
            })
            .text()
            .to_string();
        let graph = evidence_graph_id(&uniq);
        assert_eq!(
            lab.store
                .iter()
                .filter(|n| n.kind == NodeKind::Statement)
                .count(),
            4,
            "four FormalClaims of unique-vacuum"
        );
        assert_eq!(
            lab.store
                .iter()
                .filter(|n| n.kind == NodeKind::Evaluation)
                .count(),
            10,
            "seven string constructions plus GR, SM, observer-geometry"
        );
        assert_eq!(
            lab.store
                .iter()
                .filter(|n| n.kind == NodeKind::Evidence)
                .count(),
            1
        );
        assert_eq!(
            lab.store.get(graph).map(|n| n.kind),
            Some(NodeKind::Evidence)
        );
        assert_eq!(lab.store.get(graph).map(|n| n.parents.len()), Some(10));

        let again = lab
            .exec(Command::Evidence {
                claim: "predictivity.unique-vacuum".into(),
            })
            .text()
            .to_string();
        assert_eq!(
            evidence_graph_id(&again),
            graph,
            "same live lab ⇒ same graph"
        );
        assert_eq!(lab.store.len(), 15, "re-evidence is content-addressed");

        let string_stmt = unique_vacuum_statement(&lab, "type-iib");
        assert_eq!(
            unique_vacuum_statement(&lab, "bosonic"),
            string_stmt,
            "string constructions share one Statement node"
        );
        let sm_stmt = unique_vacuum_statement(&lab, "standard-model");
        let og_stmt = unique_vacuum_statement(&lab, "observer-geometry");
        assert_ne!(string_stmt, sm_stmt);
        assert_ne!(string_stmt, og_stmt);

        let string_desc = lab.store.descendants(string_stmt);
        let string_evals = store_kind(&lab, &string_desc, NodeKind::Evaluation);
        assert_eq!(string_evals.len(), 7);
        assert!(string_desc.contains(&graph));

        let sm_desc = lab.store.descendants(sm_stmt);
        let sm_evals = store_kind(&lab, &sm_desc, NodeKind::Evaluation);
        assert_eq!(sm_evals.len(), 1);
        assert!(!string_desc.contains(&sm_evals[0]));
        assert!(sm_desc.contains(&graph));

        let kept = lab.store.preserved_if_changed(string_stmt);
        assert!(kept.contains(&sm_stmt));
        assert!(kept.contains(&sm_evals[0]));
        assert!(!kept.contains(&graph));
        for e in &string_evals {
            assert!(!kept.contains(e));
        }

        let og_eval_old = store_kind(&lab, &lab.store.descendants(og_stmt), NodeKind::Evaluation);
        assert_eq!(og_eval_old.len(), 1);
        let og_old = og_eval_old[0];

        let set = lab
            .exec(Command::Set {
                theory: "observer-geometry".into(),
                knob: "unique_vacuum".into(),
                value: "false".into(),
            })
            .text()
            .to_string();
        assert!(
            set.contains("holds → fails") || set.contains("predictivity.unique-vacuum"),
            "{set}"
        );

        let after = lab
            .exec(Command::Evidence {
                claim: "predictivity.unique-vacuum".into(),
            })
            .text()
            .to_string();
        let graph2 = evidence_graph_id(&after);
        assert_ne!(graph, graph2, "a verdict flip is a new evidence graph");
        assert!(
            after.contains("observer-geometry") && after.contains("fails"),
            "{after}"
        );
        assert_eq!(
            lab.store
                .iter()
                .filter(|n| n.kind == NodeKind::Evaluation)
                .count(),
            11,
            "old observer-geometry eval remains"
        );
        assert_eq!(
            lab.store
                .iter()
                .filter(|n| n.kind == NodeKind::Evidence)
                .count(),
            2
        );
        assert!(lab.store.descendants(og_old).contains(&graph));
        assert!(!lab.store.descendants(og_old).contains(&graph2));

        let n = lab.store.len();
        let unknown = lab.exec(Command::Evidence {
            claim: "no.such.claim".into(),
        });
        assert_eq!(unknown.exit_code(), 1);
        assert_eq!(lab.store.len(), n, "unknown slug must not insert");

        lab.set_role(Role::Explorer);
        let observed = lab
            .exec(Command::Evidence {
                claim: "predictivity.unique-vacuum".into(),
            })
            .text()
            .to_string();
        assert_eq!(evidence_graph_id(&observed), graph2);
        assert_eq!(lab.store.len(), n);

        let qs = lab
            .exec(Command::Evidence {
                claim: "em.quasi-static-valid".into(),
            })
            .text()
            .to_string();
        assert!(
            qs.contains("encoding-wide") && qs.contains("λ > 100"),
            "{qs}"
        );
        let qs_graph = lab.store.get(evidence_graph_id(&qs)).unwrap();
        assert_eq!(qs_graph.kind, NodeKind::Evidence);
        assert_eq!(
            qs_graph.parents.len(),
            3,
            "Maxwell, linear-medium, and ohm-circuit each evaluate the slug: {qs}"
        );

        let sk = lab
            .exec(Command::Evidence {
                claim: "gut.proton-lifetime-sk".into(),
            })
            .text()
            .to_string();
        assert!(sk.contains("this slug is one FormalClaim"), "{sk}");
        let sk_graph = lab.store.get(evidence_graph_id(&sk)).unwrap();
        assert_eq!(sk_graph.kind, NodeKind::Evidence);
        assert_eq!(sk_graph.parents.len(), 1);
    }

    #[test]
    fn evidence_graph_restores_by_rebuild_not_deserialize() {
        let mut lab1 = Lab::standard();
        let first = lab1
            .exec(Command::Evidence {
                claim: "predictivity.unique-vacuum".into(),
            })
            .text()
            .to_string();
        let live = evidence_graph_id(&first);
        assert_eq!(
            live.to_hex(),
            "6ee50cdc3de02838465b178b47061d8d5b36d6c135baf40f80988ff640a36bc9",
            "journaling must not change the unique-vacuum evidence payload"
        );
        let jsonl = lab1.journal().to_string();
        assert!(jsonl.contains("\"event\":\"evidence\""), "{jsonl}");
        assert!(
            jsonl.contains(&format!("\"graph_hash\":\"{}\"", live.to_hex())),
            "{jsonl}"
        );

        let mut lab2 = Lab::standard();
        assert_eq!(
            lab2.store
                .iter()
                .filter(|n| n.kind == NodeKind::Evidence)
                .count(),
            0
        );
        *lab2.journal_mut() = Journal::from_jsonl(&jsonl);
        assert_eq!(
            lab2.store
                .iter()
                .filter(|n| n.kind == NodeKind::Evidence)
                .count(),
            0,
            "from_jsonl must not insert Evidence"
        );
        let journal_len = lab2.journal().len();
        lab2.restore_from_journal();
        assert_eq!(
            lab2.journal().len(),
            journal_len,
            "restore must not journal evidence again"
        );
        assert_eq!(
            lab2.store.get(live).map(|n| n.kind),
            Some(NodeKind::Evidence),
            "restore rebuilds the live graph"
        );
        assert_eq!(
            lab2.store
                .iter()
                .filter(|n| n.kind == NodeKind::Evidence)
                .count(),
            1
        );

        let forged_hex = "0".repeat(64);
        let tampered = format!(
            r#"{{"event":"evidence","t":1,"claim":"predictivity.unique-vacuum","graph_hash":"{forged_hex}"}}"#
        );
        let mut lab3 = Lab::standard();
        *lab3.journal_mut() = Journal::from_jsonl(&tampered);
        lab3.restore_from_journal();
        assert_eq!(
            lab3.store.get(live).map(|n| n.kind),
            Some(NodeKind::Evidence),
            "tampered graph_hash is not the DAG"
        );
        let forged = physis_core::artifact::ArtifactId::from_hex(&forged_hex)
            .expect("64 hex zeros is an ArtifactId");
        assert!(
            lab3.store.get(forged).is_none(),
            "a forged hash cannot mint the graph"
        );
        assert_eq!(lab3.journal().len(), 1, "tampered restore must not append");
    }

    #[test]
    fn sweep_and_compare_and_audit() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        let sweep = lab
            .exec(Command::Sweep {
                theory: "type-iib".into(),
                knob: "total_dim".into(),
                values: vec!["9".into(), "10".into()],
            })
            .text()
            .to_string();
        assert_eq!(
            lab.journal().len(),
            journal_len,
            "sweep must not persist knob turns"
        );
        assert_eq!(
            lab.theory("type-iib")
                .unwrap()
                .get("total_dim")
                .unwrap()
                .display(),
            "10"
        );
        assert!(sweep.contains("changed_claims"), "{sweep}");
        assert!(
            sweep.contains("9        changed_claims=2") || sweep.contains("changed_claims=2"),
            "{sweep}"
        );
        let cmp = lab
            .exec(Command::Compare {
                a: "olbers-static".into(),
                b: "olbers-horizon".into(),
            })
            .text()
            .to_string();
        assert!(cmp.contains("discriminating_claims="), "{cmp}");
        let audit = lab.exec(Command::Audit);
        assert_eq!(audit.exit_code(), 0, "{}", audit.text());
        lab.exec(Command::Branch {
            name: "hypothesis-a".into(),
        });
        lab.set_knob("type-iib", "total_dim", "9").unwrap();
        lab.exec(Command::Checkout {
            name: "hypothesis-a".into(),
        });
        let t = lab.theory("type-iib").unwrap();
        assert_eq!(t.get("total_dim").unwrap().display(), "10");
    }

    #[test]
    fn review_raises_semantic_and_why_shows_it() {
        let mut lab = Lab::standard();
        let text = lab
            .exec(Command::Review {
                claim: "dec.d-squared-zero".into(),
            })
            .text()
            .to_string();
        assert!(text.contains("adversarially-reviewed"), "{text}");
        assert!(text.contains("canonical reserved"), "{text}");
        let why = lab
            .exec(Command::Why {
                claim: "dec.d-squared-zero".into(),
            })
            .text()
            .to_string();
        assert!(why.contains("semantic:   adversarially-reviewed"), "{why}");
        assert!(why.contains("P3S"), "{why}");
        assert!(!why.contains("P3F"), "{why}");
        let epi = lab.exec(Command::Epistemics).text().to_string();
        assert!(epi.contains("adversarially-reviewed"), "{epi}");
        let unique = lab
            .exec(Command::Why {
                claim: "predictivity.unique-vacuum".into(),
            })
            .text()
            .to_string();
        assert!(unique.contains("semantic:   unreviewed"), "{unique}");
        assert!(unique.contains("trust:      P0"), "{unique}");
        assert!(
            unique.contains("judgment:   logical undetermined"),
            "{unique}"
        );
    }

    #[test]
    fn prove_and_review_d2_is_p3f_and_p3s_not_p4() {
        let mut lab = Lab::standard();
        lab.exec(Command::Prove {
            claim: "dec.d-squared-zero".into(),
        });
        lab.exec(Command::Review {
            claim: "dec.d-squared-zero".into(),
        });
        let why = lab
            .exec(Command::Why {
                claim: "dec.d-squared-zero".into(),
            })
            .text()
            .to_string();
        assert!(why.contains("P3F"), "{why}");
        assert!(why.contains("P3S"), "{why}");
        assert!(!why.contains("P4"), "{why}");
        assert!(!why.contains("unreviewed encoding is dangerous"), "{why}");
        assert!(why.contains("judgment:   logical proved"), "{why}");
    }

    #[test]
    fn review_conjecture_is_refused() {
        let mut lab = Lab::standard();
        let resp = lab.exec(Command::Review {
            claim: "predictivity.unique-vacuum".into(),
        });
        assert_eq!(resp.exit_code(), 1);
        assert!(resp.text().contains("no semantic dossier"));
    }

    #[test]
    fn review_restores_by_rerun_not_by_deserialize() {
        let mut lab = Lab::standard();
        lab.exec(Command::Review {
            claim: "dec.d-squared-zero".into(),
        });
        let live = lab.find_claim("dec.d-squared-zero").unwrap();
        let jsonl = lab.journal().to_string();
        assert!(jsonl.contains("\"event\":\"review\""));
        assert!(
            jsonl.contains(&format!("\"statement_hash\":\"{}\"", live.statement_hash())),
            "{jsonl}"
        );
        let mut lab2 = Lab::standard();
        *lab2.journal_mut() = Journal::from_jsonl(&jsonl);
        lab2.restore_from_journal();
        let why = lab2
            .exec(Command::Why {
                claim: "dec.d-squared-zero".into(),
            })
            .text()
            .to_string();
        assert!(why.contains("adversarially-reviewed"), "{why}");
        assert!(why.contains("P3S"), "{why}");
    }

    #[test]
    fn journal_review_of_a_stale_identity_is_not_p3s() {
        let mut lab = Lab::standard();
        let stale = physis_core::claim::Claim::new(
            "dec.d-squared-zero",
            "The exterior derivative is nilpotent: d ∘ d = 0.",
            LayerId::Mathematical,
            ClaimClass::Mathematical,
        );
        let live = lab.find_claim("dec.d-squared-zero").unwrap();
        assert_ne!(stale.statement_hash(), live.statement_hash());
        let jsonl = format!(
            r#"{{"event":"review","t":1,"claim":"dec.d-squared-zero","evidence_hash":"{}","statement_hash":"{}"}}"#,
            "0".repeat(64),
            stale.statement_hash(),
        );
        *lab.journal_mut() = Journal::from_jsonl(&jsonl);
        lab.restore_from_journal();
        assert!(lab.reviews.by_statement(live.statement_hash()).is_none());
        let why = lab
            .exec(Command::Why {
                claim: "dec.d-squared-zero".into(),
            })
            .text()
            .to_string();
        let block = why_theory_block(&why, "de-rham");
        assert!(block.contains("semantic:   unreviewed"), "{block}");
        assert!(!block.contains("P3S"), "{block}");
    }

    #[test]
    fn slug_only_journal_review_is_not_p3s() {
        let mut lab = Lab::standard();
        let live = lab.find_claim("dec.d-squared-zero").unwrap();
        let jsonl = r#"{"event":"review","t":1,"claim":"dec.d-squared-zero","evidence_hash":"00"}"#;
        *lab.journal_mut() = Journal::from_jsonl(jsonl);
        lab.restore_from_journal();
        assert!(lab.reviews.by_statement(live.statement_hash()).is_none());
        let why = lab
            .exec(Command::Why {
                claim: "dec.d-squared-zero".into(),
            })
            .text()
            .to_string();
        let block = why_theory_block(&why, "de-rham");
        assert!(block.contains("semantic:   unreviewed"), "{block}");
        assert!(!block.contains("P3S"), "{block}");
    }

    #[test]
    fn research_loop_proves_reviews_and_restores_knobs() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        let text = lab.exec(Command::Loop).text().to_string();
        assert!(text.contains("prove  dec.d-squared-zero"), "{text}");
        assert!(text.contains("prove  sr.invariant-interval"), "{text}");
        assert!(text.contains("prove  sr.subluminal-composition"), "{text}");
        assert!(
            text.contains("prove  sr.energy-momentum-invariant"),
            "{text}"
        );
        assert!(text.contains("counterexample"), "{text}");
        assert!(text.contains("replicate  dec.d-squared-zero  ok"), "{text}");
        assert!(text.contains("audit  red-team corpus caught"), "{text}");
        assert!(
            text.contains("review  dec.d-squared-zero  adversarially-reviewed"),
            "{text}"
        );
        assert!(text.contains("restore  type-iib total_dim=10"), "{text}");
        assert!(
            text.contains("constrained structural mutation"),
            "loop hypothesize must search encodings, not list catalog slugs: {text}"
        );
        assert!(
            !text.contains("unproved_catalog"),
            "catalog membership is not a structural hypothesis: {text}"
        );
        assert_eq!(
            lab.theory("type-iib")
                .unwrap()
                .get("total_dim")
                .unwrap()
                .display(),
            "10"
        );
        assert!(lab.journal().len() > journal_len);
        let why = lab
            .exec(Command::Why {
                claim: "sr.invariant-interval".into(),
            })
            .text()
            .to_string();
        assert!(why.contains("kernel proof: receipt"), "{why}");
        assert!(why.contains("adversarially-reviewed"), "{why}");
    }

    #[test]
    fn knobs_and_inspect_distinguish_chosen_from_derived() {
        let mut lab = Lab::standard();
        let knobs = lab
            .exec(Command::Knobs {
                theory: Some("type-iib".into()),
            })
            .text()
            .to_string();
        assert!(knobs.contains("observed_dim"), "{knobs}");
        assert!(knobs.contains("measured"), "{knobs}");
        assert!(knobs.contains("compact_radius_planck"), "{knobs}");
        assert!(knobs.contains("fitted"), "{knobs}");
        assert!(knobs.contains("euler_number"), "{knobs}");
        assert!(knobs.contains("chosen"), "{knobs}");

        let fitted = lab
            .exec(Command::Inspect {
                axis: Some("origin".into()),
                value: Some("fitted".into()),
            })
            .text()
            .to_string();
        assert!(fitted.contains("type-iib"), "{fitted}");
        assert!(fitted.contains("compact_radius_planck"), "{fitted}");
        assert!(fitted.contains("dilaton"), "{fitted}");
        assert!(
            !fitted.contains("euler_number"),
            "euler_number is chosen, not fitted: {fitted}"
        );

        let chosen = lab
            .exec(Command::Inspect {
                axis: Some("origin".into()),
                value: Some("chosen".into()),
            })
            .text()
            .to_string();
        assert!(chosen.contains("euler_number"), "{chosen}");

        let measured = lab
            .exec(Command::Inspect {
                axis: Some("origin".into()),
                value: Some("measured".into()),
            })
            .text()
            .to_string();
        assert!(measured.contains("standard-model"), "{measured}");
        assert!(measured.contains("generations"), "{measured}");

        let conj = lab
            .exec(Command::Inspect {
                axis: Some("class".into()),
                value: Some("conjecture".into()),
            })
            .text()
            .to_string();
        assert!(conj.contains("predictivity.unique-vacuum"), "{conj}");

        let gap = lab
            .exec(Command::Inspect {
                axis: Some("gap".into()),
                value: Some("missing-theorem".into()),
            })
            .text()
            .to_string();
        assert!(gap.contains("dec.d-squared-zero"), "{gap}");
        assert!(
            !gap.lines()
                .any(|l| l.contains("turing-machine") && l.contains("comp.halts")),
            "unbounded TM halting is not a missing lemma: {gap}"
        );
        assert!(
            !gap.lines()
                .any(|l| l.contains("combinational-circuit") && l.contains("comp.turing-complete")),
            "a failing Turing-completeness evaluation is not a missing lemma: {gap}"
        );
        assert!(
            gap.lines()
                .any(|l| l.contains("combinational-circuit") && l.contains("comp.halts")),
            "combinational halt-on-every-input Holds and still needs a receipt: {gap}"
        );

        let undec = lab
            .exec(Command::Inspect {
                axis: Some("gap".into()),
                value: Some("logically-undecidable".into()),
            })
            .text()
            .to_string();
        assert!(
            undec
                .lines()
                .any(|l| l.contains("turing-machine") && l.contains("comp.halts")),
            "{undec}"
        );
        assert!(
            undec
                .lines()
                .any(|l| l.contains("turing-machine") && l.contains("comp.decidable-equivalence")),
            "Rice's theorem must sit with halting: {undec}"
        );
        assert!(
            !undec.lines().any(|l| l.contains("empirical.sm-gauge")),
            "Type II SM gauge is an encoding gap, not Rice/halting: {undec}"
        );
        assert!(
            !undec.lines().any(|l| l.contains("comp.feasible-decision")),
            "coNP-complete search is not Rice/halting: {undec}"
        );

        let expensive = lab
            .exec(Command::Inspect {
                axis: Some("gap".into()),
                value: Some("computationally-intractable".into()),
            })
            .text()
            .to_string();
        assert!(
            expensive.lines().any(
                |l| l.contains("combinational-circuit") && l.contains("comp.feasible-decision")
            ),
            "{expensive}"
        );
        assert!(
            !expensive
                .lines()
                .any(|l| l.contains("turing-machine") && l.contains("comp.feasible-decision")),
            "unbounded TM has no finite search; cost is inapplicable: {expensive}"
        );
        assert!(
            !expensive.lines().any(|l| l.contains("comp.halts")),
            "halting is computability, not cost: {expensive}"
        );

        let encoding = lab
            .exec(Command::Inspect {
                axis: Some("gap".into()),
                value: Some("unsupported-formal-primitive".into()),
            })
            .text()
            .to_string();
        assert!(
            encoding
                .lines()
                .any(|l| l.contains("type-iia") && l.contains("empirical.sm-gauge")),
            "{encoding}"
        );

        let p0 = lab
            .exec(Command::Inspect {
                axis: Some("trust".into()),
                value: Some("P0".into()),
            })
            .text()
            .to_string();
        assert!(p0.contains("predictivity.unique-vacuum"), "{p0}");

        let p3f_before = lab
            .exec(Command::Inspect {
                axis: Some("trust".into()),
                value: Some("P3F".into()),
            })
            .text()
            .to_string();
        assert!(
            p3f_before.contains("count 0"),
            "P3F must not be earned before prove: {p3f_before}"
        );

        let proved = lab
            .exec(Command::Prove {
                claim: "dec.d-squared-zero".into(),
            })
            .text()
            .to_string();
        assert!(
            proved.contains("lean-kernel") || proved.contains("expand-recursive"),
            "{proved}"
        );

        let p3f = lab
            .exec(Command::Inspect {
                axis: Some("trust".into()),
                value: Some("P3F".into()),
            })
            .text()
            .to_string();
        assert!(p3f.contains("dec.d-squared-zero"), "{p3f}");

        let gap_after = lab
            .exec(Command::Inspect {
                axis: Some("gap".into()),
                value: Some("missing-theorem".into()),
            })
            .text()
            .to_string();
        assert!(
            !gap_after.contains("dec.d-squared-zero"),
            "proved claim must leave the missing-theorem gap: {gap_after}"
        );

        let help = lab
            .exec(Command::Inspect {
                axis: None,
                value: None,
            })
            .text()
            .to_string();
        assert!(help.contains("inspect <trust"), "{help}");
        assert!(help.contains("judgment"), "{help}");

        let bad = lab.exec(Command::Inspect {
            axis: Some("vibes".into()),
            value: Some("yes".into()),
        });
        assert!(
            bad.text().contains("unknown inspect axis"),
            "{}",
            bad.text()
        );
        assert_eq!(bad.exit_code(), 1);
    }

    #[test]
    fn inspect_judgment_inverts_projected_labels() {
        let mut lab = Lab::standard();
        let stat = lab
            .exec(Command::Inspect {
                axis: Some("judgment".into()),
                value: Some("statistical-computed".into()),
            })
            .text()
            .to_string();
        assert!(stat.contains("gut.weinberg-angle-mz-interval"), "{stat}");
        assert!(
            !stat.contains("gut.proton-lifetime-sk"),
            "Super-K is interval-subset, not a Gaussian NLL: {stat}"
        );
        assert!(
            stat.lines().any(|l| l.trim() == "count 1"),
            "only the PDG GQW cell is statistical computed: {stat}"
        );

        let excluded = lab
            .exec(Command::Inspect {
                axis: Some("judgment".into()),
                value: Some("empirical-excluded".into()),
            })
            .text()
            .to_string();
        assert!(excluded.contains("gut.proton-lifetime-sk"), "{excluded}");
        assert!(
            !excluded.contains("gut.weinberg-angle-mz-interval"),
            "PDG NLL is statistical computed, not empirical excluded: {excluded}"
        );

        let proved = lab
            .exec(Command::Inspect {
                axis: Some("judgment".into()),
                value: Some("logical-proved".into()),
            })
            .text()
            .to_string();
        assert!(
            proved.lines().any(|l| l.trim() == "count 0"),
            "no kernel proof until prove: {proved}"
        );

        let minted = lab
            .exec(Command::Prove {
                claim: "dec.d-squared-zero".into(),
            })
            .text()
            .to_string();
        assert!(
            minted.contains("lean-kernel") || minted.contains("expand-recursive"),
            "{minted}"
        );
        let after = lab
            .exec(Command::Inspect {
                axis: Some("judgment".into()),
                value: Some("logical-proved".into()),
            })
            .text()
            .to_string();
        assert!(after.contains("dec.d-squared-zero"), "{after}");
        assert!(
            !after.contains("gut.weinberg-angle-mz-interval"),
            "NLL is not a kernel proof: {after}"
        );

        let unknown = lab.exec(Command::Inspect {
            axis: Some("judgment".into()),
            value: Some("vibes".into()),
        });
        assert_eq!(unknown.exit_code(), 1, "{}", unknown.text());
        assert!(
            unknown.text().contains("unknown judgment label"),
            "{}",
            unknown.text()
        );

        lab.set_role(Role::Explorer);
        assert_eq!(
            lab.exec(Command::Inspect {
                axis: Some("judgment".into()),
                value: Some("statistical-computed".into()),
            })
            .exit_code(),
            0
        );
    }

    #[test]
    fn gap_for_does_not_call_undecidable_a_missing_theorem() {
        let halt = gap_for(
            physis_core::ClaimClass::ModelInternal,
            physis_core::DerivationAssurance::Executed,
            VerdictKind::Undecidable,
            physis_core::EmpiricalStatus::NotApplicable,
            false,
            LayerId::Information,
            false,
        );
        assert_eq!(halt, Some(GapReason::LogicallyUndecidable));

        let rice = gap_for(
            physis_core::ClaimClass::Phenomenological,
            physis_core::DerivationAssurance::Executed,
            VerdictKind::Undecidable,
            physis_core::EmpiricalStatus::NotApplicable,
            false,
            LayerId::Information,
            false,
        );
        assert_eq!(rice, Some(GapReason::LogicallyUndecidable));

        let sm_gauge = gap_for(
            physis_core::ClaimClass::Phenomenological,
            physis_core::DerivationAssurance::Executed,
            VerdictKind::Undecidable,
            physis_core::EmpiricalStatus::NotApplicable,
            false,
            LayerId::Interaction,
            false,
        );
        assert_eq!(sm_gauge, Some(GapReason::UnsupportedFormalPrimitive));

        let p_vs_np = gap_for(
            physis_core::ClaimClass::OpenProblem,
            physis_core::DerivationAssurance::Asserted,
            VerdictKind::Undecidable,
            physis_core::EmpiricalStatus::Untested,
            false,
            LayerId::Mathematical,
            false,
        );
        assert_eq!(p_vs_np, Some(GapReason::ScientificOpenProblem));

        let d2 = gap_for(
            physis_core::ClaimClass::Mathematical,
            physis_core::DerivationAssurance::Executed,
            VerdictKind::Holds,
            physis_core::EmpiricalStatus::NotApplicable,
            false,
            LayerId::Mathematical,
            false,
        );
        assert_eq!(d2, Some(GapReason::MissingTheorem));

        let proved = gap_for(
            physis_core::ClaimClass::Mathematical,
            physis_core::DerivationAssurance::Executed,
            VerdictKind::Holds,
            physis_core::EmpiricalStatus::NotApplicable,
            true,
            LayerId::Mathematical,
            false,
        );
        assert_eq!(proved, None);

        let failing_tc = gap_for(
            physis_core::ClaimClass::Phenomenological,
            physis_core::DerivationAssurance::Executed,
            VerdictKind::Fails,
            physis_core::EmpiricalStatus::NotApplicable,
            false,
            LayerId::Information,
            false,
        );
        assert_eq!(
            failing_tc, None,
            "a failing evaluation is decided, not a missing lemma"
        );

        let combinational_halts = gap_for(
            physis_core::ClaimClass::ModelInternal,
            physis_core::DerivationAssurance::Executed,
            VerdictKind::Holds,
            physis_core::EmpiricalStatus::NotApplicable,
            false,
            LayerId::Information,
            false,
        );
        assert_eq!(combinational_halts, Some(GapReason::MissingTheorem));

        let untested = gap_for(
            physis_core::ClaimClass::EmpiricalPrediction,
            physis_core::DerivationAssurance::Executed,
            VerdictKind::Holds,
            physis_core::EmpiricalStatus::Untested,
            false,
            LayerId::Effective,
            false,
        );
        assert_eq!(untested, Some(GapReason::MissingDataset));

        let excluded = gap_for(
            physis_core::ClaimClass::EmpiricalPrediction,
            physis_core::DerivationAssurance::Executed,
            VerdictKind::Fails,
            physis_core::EmpiricalStatus::Excluded,
            false,
            LayerId::Effective,
            false,
        );
        assert_eq!(
            excluded, None,
            "an exclusion receipt is a decision, not a gap"
        );

        let coarse = gap_for(
            physis_core::ClaimClass::EmpiricalPrediction,
            physis_core::DerivationAssurance::Executed,
            VerdictKind::Undecidable,
            physis_core::EmpiricalStatus::Inconclusive,
            false,
            LayerId::Effective,
            false,
        );
        assert_eq!(coarse, Some(GapReason::InsufficientPrecision));

        let lattice = gap_for(
            physis_core::ClaimClass::ModelInternal,
            physis_core::DerivationAssurance::Executed,
            VerdictKind::Undecidable,
            physis_core::EmpiricalStatus::Inconclusive,
            false,
            LayerId::Field,
            false,
        );
        assert_eq!(
            lattice,
            Some(GapReason::InsufficientPrecision),
            "a coarse lattice is a resolution gap, not an encoding gap"
        );

        let expensive = gap_for(
            physis_core::ClaimClass::Phenomenological,
            physis_core::DerivationAssurance::Executed,
            VerdictKind::Undecidable,
            physis_core::EmpiricalStatus::NotApplicable,
            false,
            LayerId::Information,
            true,
        );
        assert_eq!(expensive, Some(GapReason::ComputationallyIntractable));
    }

    #[test]
    fn explorer_cannot_prove_and_does_not_mint() {
        let mut lab = Lab::standard();
        lab.set_role(Role::Explorer);
        let resp = lab.exec(Command::Prove {
            claim: "dec.d-squared-zero".into(),
        });
        assert_eq!(resp.exit_code(), 1, "{}", resp.text());
        assert!(
            resp.text().contains("explorer cannot prove"),
            "{}",
            resp.text()
        );
        let p3f = lab
            .exec(Command::Inspect {
                axis: Some("trust".into()),
                value: Some("P3F".into()),
            })
            .text()
            .to_string();
        assert!(p3f.contains("count 0"), "{p3f}");
    }

    #[test]
    fn formalizer_emits_untrusted_encoding_without_a_receipt() {
        let mut lab = Lab::standard();
        lab.set_role(Role::Formalizer);
        let text = lab
            .exec(Command::Formalize {
                claim: "dec.d-squared-zero".into(),
            })
            .text()
            .to_string();
        assert!(text.contains("untrusted encoding"), "{text}");
        assert!(text.contains("d_squared_zero"), "{text}");
        assert!(!text.contains("backend"), "{text}");
        let prove = lab.exec(Command::Prove {
            claim: "dec.d-squared-zero".into(),
        });
        assert!(
            prove.text().contains("formalizer cannot prove"),
            "{}",
            prove.text()
        );
        let conj = lab.exec(Command::Formalize {
            claim: "predictivity.unique-vacuum".into(),
        });
        assert_eq!(conj.exit_code(), 1);
        assert!(
            conj.text().contains("no catalog identity"),
            "{}",
            conj.text()
        );
    }

    #[test]
    fn proof_searcher_can_prove_under_a_spent_budget() {
        let mut lab = Lab::standard();
        lab.set_role(Role::ProofSearcher);
        lab.set_budget(ResearchBudget::limited(1, 0, 0));
        let first = lab
            .exec(Command::Prove {
                claim: "dec.d-squared-zero".into(),
            })
            .text()
            .to_string();
        assert!(
            first.contains("lean-kernel") || first.contains("expand-recursive"),
            "{first}"
        );
        let second = lab.exec(Command::Prove {
            claim: "sr.invariant-interval".into(),
        });
        assert_eq!(second.exit_code(), 1, "{}", second.text());
        assert!(
            second.text().contains("budget exhausted"),
            "{}",
            second.text()
        );
        let p3f = lab
            .exec(Command::Inspect {
                axis: Some("trust".into()),
                value: Some("P3F".into()),
            })
            .text()
            .to_string();
        assert!(p3f.contains("dec.d-squared-zero"), "{p3f}");
        assert!(!p3f.contains("sr.invariant-interval"), "{p3f}");
    }

    #[test]
    fn proof_searcher_cannot_remint_a_receipt_it_requested() {
        let mut lab = Lab::standard();
        lab.set_role(Role::ProofSearcher);
        let proved = lab
            .exec(Command::Prove {
                claim: "dec.d-squared-zero".into(),
            })
            .text()
            .to_string();
        assert!(
            proved.contains("lean-kernel") || proved.contains("expand-recursive"),
            "{proved}"
        );
        let remint = lab.exec(Command::Reproduce {
            claim: "dec.d-squared-zero".into(),
        });
        assert_eq!(remint.exit_code(), 1, "{}", remint.text());
        assert!(
            remint.text().contains("proof-searcher cannot reproduce"),
            "{}",
            remint.text()
        );

        lab.set_role(Role::ReplicationAgent);
        let ok = lab
            .exec(Command::Reproduce {
                claim: "dec.d-squared-zero".into(),
            })
            .text()
            .to_string();
        assert!(ok.contains("in-process remint matched"), "{ok}");
        assert!(ok.contains("not P4"), "{ok}");
        let prove = lab.exec(Command::Prove {
            claim: "sr.invariant-interval".into(),
        });
        assert!(
            prove.text().contains("replication-agent cannot prove"),
            "{}",
            prove.text()
        );
    }

    #[test]
    fn empirical_analyst_scores_and_explorer_cannot() {
        let mut lab = Lab::standard();
        lab.set_role(Role::Explorer);
        let blocked = lab.exec(Command::Score {
            theory: "standard-model".into(),
        });
        assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
        assert!(
            blocked.text().contains("explorer cannot score"),
            "{}",
            blocked.text()
        );

        lab.set_role(Role::EmpiricalAnalyst);
        let card = lab
            .exec(Command::Score {
                theory: "standard-model".into(),
            })
            .text()
            .to_string();
        assert!(card.contains("score standard-model"), "{card}");
        assert!(card.contains("has_gravity"), "{card}");
        let prove = lab.exec(Command::Prove {
            claim: "dec.d-squared-zero".into(),
        });
        assert!(
            prove.text().contains("empirical-analyst cannot prove"),
            "{}",
            prove.text()
        );
    }

    #[test]
    fn reviewer_cannot_prove() {
        let mut lab = Lab::standard();
        lab.set_role(Role::Reviewer);
        let resp = lab.exec(Command::Prove {
            claim: "dec.d-squared-zero".into(),
        });
        assert!(
            resp.text().contains("reviewer cannot prove"),
            "{}",
            resp.text()
        );
    }

    #[test]
    fn loop_respects_a_zero_prove_budget() {
        let mut lab = Lab::standard();
        lab.set_budget(ResearchBudget::limited(0, 0, 0));
        let text = lab.exec(Command::Loop).text().to_string();
        assert!(
            text.contains("prove  dec.d-squared-zero  research budget exhausted"),
            "{text}"
        );
        assert!(
            text.contains("replicate  dec.d-squared-zero  research budget exhausted"),
            "{text}"
        );
        assert!(
            text.contains("review  dec.d-squared-zero  trust P3F required"),
            "{text}"
        );
        let p3f = lab
            .exec(Command::Inspect {
                axis: Some("trust".into()),
                value: Some("P3F".into()),
            })
            .text()
            .to_string();
        assert!(
            p3f.contains("count 0"),
            "loop must not mint when prove budget is zero: {p3f}"
        );
    }

    #[test]
    fn loop_review_requires_p3f_and_does_not_spend_review_budget() {
        let mut lab = Lab::standard();
        lab.set_budget(ResearchBudget::limited(0, 2, 0));
        let text = lab.exec(Command::Loop).text().to_string();
        assert!(
            text.contains("review  dec.d-squared-zero  trust P3F required"),
            "{text}"
        );
        assert!(
            text.contains("review  sr.invariant-interval  trust P3F required"),
            "{text}"
        );
        assert!(
            text.contains("review  sr.subluminal-composition  trust P3F required"),
            "{text}"
        );
        assert!(
            text.contains("review  sr.energy-momentum-invariant  trust P3F required"),
            "{text}"
        );
        assert!(
            !text.contains("adversarially-reviewed"),
            "loop must not raise P3S on an unproved identity: {text}"
        );
        let p3s = lab
            .exec(Command::Inspect {
                axis: Some("trust".into()),
                value: Some("P3S".into()),
            })
            .text()
            .to_string();
        assert!(p3s.contains("count 0"), "{p3s}");
        let p3f = lab
            .exec(Command::Inspect {
                axis: Some("trust".into()),
                value: Some("P3F".into()),
            })
            .text()
            .to_string();
        assert!(p3f.contains("count 0"), "{p3f}");

        // Review budget was not spent on the skipped loop step.
        lab.set_role(Role::Reviewer);
        let encoding = lab.exec(Command::Review {
            claim: "dec.d-squared-zero".into(),
        });
        assert_eq!(encoding.exit_code(), 0, "{}", encoding.text());
        assert!(
            encoding.text().contains("adversarially-reviewed"),
            "{}",
            encoding.text()
        );
    }

    #[test]
    fn reproduce_without_receipt_does_not_spend_prove_budget() {
        let mut lab = Lab::standard();
        lab.set_budget(ResearchBudget::limited(1, 0, 0));
        let missing = lab.exec(Command::Reproduce {
            claim: "dec.d-squared-zero".into(),
        });
        assert_eq!(missing.exit_code(), 1, "{}", missing.text());
        let proved = lab.exec(Command::Prove {
            claim: "dec.d-squared-zero".into(),
        });
        assert_eq!(proved.exit_code(), 0, "{}", proved.text());
        assert!(
            proved.text().contains("lean-kernel") || proved.text().contains("expand-recursive"),
            "{}",
            proved.text()
        );
    }

    #[test]
    fn reproduce_matches_in_process_and_is_not_p4() {
        let mut lab = Lab::standard();
        let missing = lab.exec(Command::Reproduce {
            claim: "dec.d-squared-zero".into(),
        });
        assert_eq!(missing.exit_code(), 1, "{}", missing.text());
        assert!(
            missing.text().contains("trust P3F required"),
            "{}",
            missing.text()
        );

        let proved = lab
            .exec(Command::Prove {
                claim: "dec.d-squared-zero".into(),
            })
            .text()
            .to_string();
        assert!(
            proved.contains("lean-kernel") || proved.contains("expand-recursive"),
            "{proved}"
        );

        let text = lab
            .exec(Command::Reproduce {
                claim: "dec.d-squared-zero".into(),
            })
            .text()
            .to_string();
        assert!(text.contains("in-process remint matched"), "{text}");
        assert!(text.contains("not P4"), "{text}");

        let p4 = lab
            .exec(Command::Inspect {
                axis: Some("trust".into()),
                value: Some("P4".into()),
            })
            .text()
            .to_string();
        assert!(p4.contains("count 0"), "{p4}");

        lab.set_role(Role::Explorer);
        let blocked = lab.exec(Command::Reproduce {
            claim: "dec.d-squared-zero".into(),
        });
        assert!(
            blocked.text().contains("explorer cannot reproduce"),
            "{}",
            blocked.text()
        );
    }

    #[test]
    fn gaps_graph_drops_a_proved_identity() {
        let mut lab = Lab::standard();
        let before = lab.exec(Command::Gaps).text().to_string();
        assert!(before.starts_with("gaps  graph "), "{before}");
        assert!(
            before
                .lines()
                .any(|l| l.contains("dec.d-squared-zero") && l.contains("needs receipt")),
            "{before}"
        );
        assert!(
            before
                .lines()
                .any(|l| l.contains("predictivity.unique-vacuum") && l.contains("needs science")),
            "{before}"
        );
        assert!(
            before.lines().any(|l| l.contains("turing-machine")
                && l.contains("comp.halts")
                && l.contains("computability")),
            "{before}"
        );
        assert!(
            !before.lines().any(|l| l.contains("combinational-circuit")
                && l.contains("comp.turing-complete")
                && l.contains("needs receipt")),
            "combinational Turing-completeness Fails; it is not a missing lemma: {before}"
        );
        assert!(
            before.lines().any(|l| l.contains("combinational-circuit")
                && l.contains("comp.halts")
                && l.contains("needs receipt")),
            "{before}"
        );
        assert!(
            before.lines().any(|l| l.contains("combinational-circuit")
                && l.contains("comp.feasible-decision")
                && l.contains("needs resources")),
            "{before}"
        );
        assert!(
            !before
                .lines()
                .any(|l| l.contains("turing-machine") && l.contains("comp.feasible-decision")),
            "unbounded TM feasible-decision is inapplicable, not a cost gap: {before}"
        );
        assert!(
            !before
                .lines()
                .any(|l| l.contains("gut.proton-lifetime-sk") && l.contains("needs dataset")),
            "Super-K is a Dataset; exclusion is a decision: {before}"
        );
        assert!(before.contains("dec.closed-equals-exact"), "{before}");
        assert!(
            before
                .lines()
                .any(|l| l.contains("lemma dec.d-squared-zero") && l.contains("needs receipt")),
            "Poincaré must record an unmet d² lemma: {before}"
        );

        let why = lab
            .exec(Command::Why {
                claim: "dec.closed-equals-exact".into(),
            })
            .text()
            .to_string();
        assert!(why.contains("lemmas:"), "{why}");
        assert!(
            why.contains("dec.d-squared-zero") && why.contains("needs receipt"),
            "{why}"
        );

        let proved = lab
            .exec(Command::Prove {
                claim: "dec.d-squared-zero".into(),
            })
            .text()
            .to_string();
        assert!(
            proved.contains("lean-kernel") || proved.contains("expand-recursive"),
            "{proved}"
        );

        let after = lab.exec(Command::Gaps).text().to_string();
        assert!(
            !after.lines().any(|l| l.contains("dec.d-squared-zero")
                && l.contains("needs receipt")
                && !l.contains("lemma")),
            "proved catalog identity must leave the gap graph: {after}"
        );
        assert!(
            after
                .lines()
                .any(|l| l.contains("lemma dec.d-squared-zero") && l.contains("have receipt")),
            "Poincaré still depends on d² after the receipt exists: {after}"
        );
        assert!(
            after
                .lines()
                .any(|l| l.contains("dec.closed-equals-exact") && l.contains("needs receipt")),
            "Poincaré itself is not a catalog identity: {after}"
        );
        let h1 = before.lines().next().unwrap();
        let h2 = after.lines().next().unwrap();
        assert_ne!(h1, h2, "gap graph hash must move after prove");

        let why_after = lab
            .exec(Command::Why {
                claim: "dec.closed-equals-exact".into(),
            })
            .text()
            .to_string();
        assert!(
            why_after.contains("dec.d-squared-zero") && why_after.contains("have receipt"),
            "{why_after}"
        );

        lab.set_role(Role::Explorer);
        assert_eq!(lab.exec(Command::Gaps).exit_code(), 0);
    }

    #[test]
    fn mass_shell_records_an_interval_lemma_edge() {
        let mut lab = Lab::standard();
        let before = lab.exec(Command::Gaps).text().to_string();
        assert!(
            before
                .lines()
                .any(|l| l.contains("sr.energy-momentum-invariant")
                    && l.contains("needs receipt")
                    && !l.contains("lemma")),
            "{before}"
        );
        assert!(
            before.contains("lemma sr.invariant-interval") && before.contains("needs receipt"),
            "mass shell must record an unmet interval lemma: {before}"
        );

        let proved = lab
            .exec(Command::Prove {
                claim: "sr.invariant-interval".into(),
            })
            .text()
            .to_string();
        assert!(
            proved.contains("lean-kernel") || proved.contains("expand-recursive"),
            "{proved}"
        );

        let after = lab.exec(Command::Gaps).text().to_string();
        assert!(
            after
                .lines()
                .any(|l| l.contains("sr.energy-momentum-invariant")
                    && l.contains("needs receipt")
                    && !l.contains("lemma")),
            "mass shell is not the interval identity: {after}"
        );
        assert!(
            after
                .lines()
                .any(|l| l.contains("lemma sr.invariant-interval") && l.contains("have receipt")),
            "{after}"
        );

        let why = lab
            .exec(Command::Why {
                claim: "sr.energy-momentum-invariant".into(),
            })
            .text()
            .to_string();
        assert!(
            why.contains("sr.invariant-interval") && why.contains("have receipt"),
            "{why}"
        );
    }
}
