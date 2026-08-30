//! The laboratory: theories, knobs, experiments, journal.

use std::collections::BTreeMap;

use physis_core::assurance::SemanticAssurance;
use physis_core::claim::VerdictKind;
use physis_core::error::CoreError;
use physis_core::formal::FormalClaim;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobValue};
use physis_proof::{Challenge, UntrustedProof, CATALOG};
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
        lab.insert(Box::new(CombinationalCircuit));
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

    /// Re-apply the `set-knob` events already in the journal to theory state,
    /// **without** recording them again.
    ///
    /// This resumes a persisted session: after loading a journal from a file,
    /// call this so subsequent turns build on the prior ones instead of on
    /// fresh defaults. It is what makes a multi-process `--journal` session a
    /// single coherent, replayable session rather than a bag of independent
    /// one-shot diffs.
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
                JournalEvent::Prove { claim, .. } => {
                    // Restore must remint; a Lean kernel miss still has the
                    // exact expanders as an independently checkable receipt.
                    if self.remint_preferred(&claim).is_err() {
                        let _ = self.remint_exact(&claim);
                    }
                }
                JournalEvent::Review { claim, .. } => {
                    let _ = self.remint_review(&claim);
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
    pub fn exec(&mut self, cmd: Command) -> Response {
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
                        c.id.0,
                        v.kind.as_str(),
                        v.derivation.as_str(),
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
                        text.push_str("no verdict kinds changed\n");
                    } else {
                        text.push_str("verdict changes:\n");
                        for d in &diffs {
                            text.push_str(&format!(
                                "  {:<32} {} → {}\n",
                                d.claim,
                                d.from.as_str(),
                                d.to.as_str()
                            ));
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
                        let semantic = self.semantic_tag(&c.id.0, v.semantic);
                        by_derivation.entry(v.derivation.as_str()).or_default()[idx] += 1;
                        by_class.entry(v.class.as_str()).or_default()[idx] += 1;
                        by_semantic.entry(semantic.as_str()).or_default()[idx] += 1;
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
                        if c.id.0 == claim {
                            found += 1;
                            text.push_str(&format!("theory {}\n", t.id()));
                            text.push_str(&format!("  statement:  {}\n", c.statement));
                            text.push_str(&format!("  class:      {}\n", v.class.as_str()));
                            text.push_str(&format!("  derivation: {}\n", v.derivation.as_str()));
                            text.push_str(&format!("  empirical:  {}\n", v.empirical.as_str()));
                            text.push_str(&format!(
                                "  semantic:   {}\n",
                                self.semantic_tag(&c.id.0, v.semantic).as_str()
                            ));
                            text.push_str(&format!("  identity:   {}\n", c.statement_hash));
                            text.push_str(&format!("  domain:     {}\n", c.domain.notes));
                            text.push_str("  assumptions:\n");
                            for a in &c.assumptions.items {
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
                            match self.receipts.by_statement(c.statement_hash) {
                                Some(r) => {
                                    text.push_str(&format!(
                                        "  kernel proof: receipt {} / {} + {} (backend {:?})\n",
                                        r.challenge_hash,
                                        r.primary_checker.checker,
                                        r.secondary_checker.checker,
                                        r.formal_backend
                                    ));
                                    text.push_str("  axioms:\n");
                                    for a in &r.axioms_used {
                                        text.push_str(&format!("    - {}\n", a.0));
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
            Command::Review { claim } => self.review_claim(&claim),
            Command::Loop => self.research_loop(),
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
                if c.id.0 == claim_id {
                    return Some(c);
                }
            }
        }
        None
    }

    fn semantic_tag(&self, claim_id: &str, fallback: SemanticAssurance) -> SemanticAssurance {
        self.reviews
            .by_claim(claim_id)
            .map(|r| r.assurance())
            .unwrap_or(fallback)
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

    /// Re-run semantic review. Never deserializes a `SemanticAssurance` tag.
    fn remint_review(&mut self, claim_id: &str) -> Result<physis_semantic::SemanticRecord, String> {
        let rec = physis_semantic::review(claim_id).map_err(|e| e.to_string())?;
        self.reviews.record(&rec);
        self.store.insert(Node::new(
            NodeKind::SemanticReview,
            vec![rec.source_hash()],
            rec.evidence_hash().to_hex().as_bytes(),
        ));
        Ok(rec)
    }

    fn review_claim(&mut self, claim_id: &str) -> Response {
        match self.remint_review(claim_id) {
            Ok(r) => {
                self.journal
                    .record(JournalEvent::review(claim_id, r.evidence_hash().to_hex()));
                Response::ok(format!(
                    "review {claim_id}\n  semantic {}\n  evidence {}\n  canonical reserved (not agent-mintable)\n",
                    r.assurance().as_str(),
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
        if physis_proof::lookup(claim_id).is_some() && discover_tools().is_some() {
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
                self.journal
                    .record(JournalEvent::prove(claim_id, r.challenge_hash.to_hex()));
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
                if v.derivation == physis_core::DerivationAssurance::Asserted {
                    asserted += 1;
                }
            }
        }
        text.push_str(&format!(
            "observe  holds={holds} fails={fails} asserted={asserted} receipts={}\n",
            self.receipts.len()
        ));

        let mut hypo: Vec<&str> = Vec::new();
        for spec in CATALOG {
            if self.receipts.by_claim(spec.claim_id).is_none() {
                hypo.push(spec.claim_id);
            }
        }
        text.push_str(&format!("hypothesize  unproved_catalog={hypo:?}\n"));

        let mut proved = Vec::new();
        for spec in CATALOG {
            match self.remint_preferred(spec.claim_id) {
                Ok(r) => {
                    self.journal.record(JournalEvent::prove(
                        spec.claim_id,
                        r.challenge_hash.to_hex(),
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
            let before = self
                .receipts
                .by_claim(spec.claim_id)
                .map(|r| r.challenge_hash);
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
            match self.remint_review(spec.claim_id) {
                Ok(r) => {
                    self.journal.record(JournalEvent::review(
                        spec.claim_id,
                        r.evidence_hash().to_hex(),
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
                .find(|(c, _)| c.id.0 == claim_id)
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
                        .find(|(c, _)| c.id.0 == claim_id)
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
                            claim.id.0
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
        let eb: BTreeMap<_, _> = tb
            .evaluate_all()
            .into_iter()
            .map(|(c, v)| (c.id.0, v))
            .collect();
        let mut text = format!("compare {a} vs {b}\n");
        let mut n = 0usize;
        for (c, va) in ea {
            if let Some(vb) = eb.get(&c.id.0) {
                if va.kind != vb.kind {
                    n += 1;
                    text.push_str(&format!(
                        "  {:<32} {} vs {}\n",
                        c.id.0,
                        va.kind.as_str(),
                        vb.kind.as_str()
                    ));
                }
            }
        }
        text.push_str(&format!("discriminating_claims={n}\n"));
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

fn render_knobs(t: &dyn Theory) -> String {
    let mut text = format!("knobs  {}\n", t.id());
    for (spec, val) in t.snapshot() {
        text.push_str(&format!(
            "  {:<24} {:<10} {}\n    {}\n",
            spec.name,
            val.display(),
            spec.layer.as_str(),
            spec.doc
        ));
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turning_iib_dimension_flips_critical_claim() {
        let mut lab = Lab::standard();
        let diffs = lab.set_knob("type-iib", "total_dim", "9").unwrap().2;
        assert!(
            diffs
                .iter()
                .any(|d| d.claim == "consistency.critical-dimension"
                    && d.from == VerdictKind::Holds
                    && d.to == VerdictKind::Fails),
            "expected critical-dimension Holds→Fails, got {diffs:?}"
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
        assert!(text.contains("total claim-evaluations:"));
        assert!(text.contains("open-problem") || text.contains("conjecture"));
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
        assert!(!text.contains("theorem"));
    }

    #[test]
    fn no_lab_verdict_is_a_kernel_proof() {
        let lab = Lab::standard();
        for id in lab.theory_ids() {
            let t = lab.theory(&id).unwrap();
            for (c, v) in t.evaluate_all() {
                assert!(
                    matches!(
                        v.derivation,
                        physis_core::DerivationAssurance::Asserted
                            | physis_core::DerivationAssurance::Executed
                            | physis_core::DerivationAssurance::CrossChecked
                            | physis_core::DerivationAssurance::CertifiedNumeric
                    ),
                    "{} / {} derivation {:?}",
                    id,
                    c.id.0,
                    v.derivation
                );
                assert_eq!(v.semantic, physis_core::SemanticAssurance::Unreviewed);
                assert!(!c.assumptions.items.is_empty());
            }
        }
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
        let jsonl = lab.journal().to_string();
        assert!(jsonl.contains("\"event\":\"prove\""));
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
        let epi = lab.exec(Command::Epistemics).text().to_string();
        assert!(epi.contains("adversarially-reviewed"), "{epi}");
        let unique = lab
            .exec(Command::Why {
                claim: "predictivity.unique-vacuum".into(),
            })
            .text()
            .to_string();
        assert!(unique.contains("semantic:   unreviewed"), "{unique}");
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
        let jsonl = lab.journal().to_string();
        assert!(jsonl.contains("\"event\":\"review\""));
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
    }

    #[test]
    fn research_loop_proves_reviews_and_restores_knobs() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        let text = lab.exec(Command::Loop).text().to_string();
        assert!(text.contains("prove  dec.d-squared-zero"), "{text}");
        assert!(text.contains("prove  sr.invariant-interval"), "{text}");
        assert!(text.contains("counterexample"), "{text}");
        assert!(text.contains("replicate  dec.d-squared-zero  ok"), "{text}");
        assert!(text.contains("audit  red-team corpus caught"), "{text}");
        assert!(
            text.contains("review  dec.d-squared-zero  adversarially-reviewed"),
            "{text}"
        );
        assert!(text.contains("restore  type-iib total_dim=10"), "{text}");
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
}
