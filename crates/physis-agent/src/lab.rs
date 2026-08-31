//! The laboratory: theories, knobs, experiments, journal.

use std::collections::{BTreeMap, BTreeSet};

use physis_core::assurance::{ClaimClass, DerivationAssurance, SemanticAssurance};
use physis_core::claim::VerdictKind;
use physis_core::error::CoreError;
use physis_core::formal::FormalClaim;
use physis_core::id::LayerId;
use physis_core::judgment::{
    GapReason, Judgment, ParameterOrigin, TrustEvidence, TrustProfile, TrustTier,
};
use physis_core::knob::{KnobDomain, KnobValue};
use physis_core::AxiomLedger;
use physis_numeric::Ratio;
use physis_proof::{
    catalog_tree_binding, catalog_trees_in, lookup_matching, Challenge, UntrustedProof, CATALOG,
};
use physis_semantic::SemanticStore;
use physis_store::{ArtifactStore, Node, NodeKind};
use physis_theory::blackbody::Blackbody;
use physis_theory::computation::{CombinationalCircuit, LandauerEngine, TuringMachine};
use physis_theory::continuum::{DiracFermion, KleinGordonField};
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
    /// Independent Ratio parses keyed by statement hash. Not a kernel
    /// receipt store and not the `CertifiedNumeric` overlay that earns P3N.
    numeric_certs: BTreeMap<physis_core::artifact::ArtifactId, physis_core::artifact::ArtifactId>,
    /// Independent SourceRecord rebuilds keyed by statement hash. Not P3S.
    cited_sources: BTreeMap<physis_core::artifact::ArtifactId, physis_core::artifact::ArtifactId>,
    /// Independent IR package round-trips keyed by theory id. Not P3S.
    encoded_packages: BTreeMap<String, physis_core::artifact::ArtifactId>,
    /// Independent from_lab projections keyed by statement hash. JSON
    /// cannot mint `logical proved`.
    judged_projections:
        BTreeMap<physis_core::artifact::ArtifactId, physis_core::artifact::ArtifactId>,
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
        "a Klein–Gordon scalar and a 1D Dirac fermion on a lattice",
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
            numeric_certs: BTreeMap::new(),
            cited_sources: BTreeMap::new(),
            encoded_packages: BTreeMap::new(),
            judged_projections: BTreeMap::new(),
        }
    }

    /// Default lab: SM, GR, Type IIB, heterotic E₈×E₈, bosonic, observer-geometry.
    pub fn standard() -> Self {
        let mut lab = Self::empty();
        lab.insert(Box::new(StandardModel::default()));
        lab.insert(Box::new(GeneralRelativity::default()));
        lab.insert(Box::new(NewtonianGravity::default()));
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
        lab.insert(Box::new(MaxwellVacuum::default()));
        lab.insert(Box::new(LinearMedium::default()));
        lab.insert(Box::new(OhmCircuit::default()));
        // Third domain: computation.
        lab.insert(Box::new(CombinationalCircuit::default()));
        lab.insert(Box::new(TuringMachine::default()));
        // Computation ↔ thermodynamics bridge: Landauer's principle.
        lab.insert(Box::new(LandauerEngine::default()));
        // M4 continuum: a scalar field and lattice gauge fields as local objects.
        lab.insert(Box::new(KleinGordonField::default()));
        lab.insert(Box::new(DiracFermion::default()));
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
    /// evidence / enclose / cite / constant / encode / judge from live state, **without** recording them again.
    ///
    /// This resumes a persisted session: after loading a journal from a file,
    /// call this so subsequent turns build on the prior ones instead of on
    /// fresh defaults. It is what makes a multi-process `--journal` session a
    /// single coherent, replayable session rather than a bag of independent
    /// one-shot diffs.
    ///
    /// Evidence restore rebuilds the DAG from live evaluations. Enclose
    /// restore rebuilds numeric certificates from live overlay strings.
    /// Cite restore rebuilds source records from live fields. Constant
    /// restore rebuilds VersionedConstant nodes from live constructors.
    /// Encode
    /// restore rebuilds EncodingPackage nodes from live IR packages.
    /// Recorded hashes are not deserialized as the snapshot: a forged hash
    /// cannot mint an Evidence, NumericCertificate, Source, VersionedConstant, EncodingPackage, or JudgmentProjection node.
    /// [`crate::replay::replay_journal`] still certifies only `set-knob`
    /// diffs.
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
                JournalEvent::Enclose { claim, .. } => {
                    // Rebuild from live CertifiedNumeric overlay strings.
                    // The recorded certificate hash is not deserialized.
                    let _ = self.build_numeric_certificates(&claim);
                }
                JournalEvent::Cite { claim, .. } => {
                    // Rebuild from live dataset / dossier SourceRecords.
                    // The recorded source hash is not deserialized.
                    let _ = self.build_cite(&claim);
                }
                JournalEvent::Constant { name, .. } => {
                    // Rebuild from live constructors. The recorded node
                    // hash is not deserialized. Empty name is the ledger.
                    if name.is_empty() {
                        let _ = self.build_constant_ledger();
                    } else {
                        let _ = self.build_constant(&name);
                    }
                }
                JournalEvent::Encode { theory, .. } => {
                    // Rebuild from the live IR package. The recorded
                    // package hash is not deserialized.
                    let _ = self.build_encoding(&theory);
                }
                JournalEvent::Judge { claim, .. } => {
                    // Rebuild from live from_lab. The recorded
                    // projection hash is not deserialized.
                    let _ = self.build_judgment(&claim);
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
                            if let Some(id) = self.numeric_certs.get(&c.statement_hash()) {
                                text.push_str(&format!("  enclose:     {id}\n"));
                            } else if v.derivation() == DerivationAssurance::CertifiedNumeric {
                                text.push_str(
                                    "  enclose:     none (overlay is not an independent Ratio parse)\n",
                                );
                            }
                            if let Some(id) = self.cited_sources.get(&c.statement_hash()) {
                                text.push_str(&format!("  source:      {id}\n"));
                            } else if physis_data::dataset_for_claim(c.id_str()).is_some()
                                || physis_semantic::cite_source(c.id_str()).is_ok()
                            {
                                text.push_str(
                                    "  source:      none (locator is not an independent SourceRecord rebuild)\n",
                                );
                            }
                            if let Some(id) = self.encoded_packages.get(t.id()) {
                                text.push_str(&format!("  encoding:    {id}\n"));
                            } else if t.ir_package().is_some() {
                                text.push_str(
                                    "  encoding:    none (IR package is not an independent round-trip)\n",
                                );
                            }
                            if let Some(id) = self.judged_projections.get(&c.statement_hash()) {
                                text.push_str(&format!("  projection:  {id}\n"));
                            } else {
                                text.push_str(
                                    "  projection:  none (judgment is not an independent from_lab rebuild)\n",
                                );
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
            Command::Enclose { claim } => self.enclose_claim(&claim),
            Command::Cite { claim } => self.cite_claim(&claim),
            Command::Constant { name } => self.constant_entry(name.as_deref()),
            Command::Encode { theory } => self.encode_theory(&theory),
            Command::Judge { claim } => self.judge_claim(&claim),
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

    /// Independently parse live `CertifiedNumeric` enclosure strings as
    /// [`Ratio`]. Stores a content-addressed [`NodeKind::NumericCertificate`].
    /// Does not mint a kernel receipt, Canonical, or P4, and does not
    /// change the P3N overlay.
    fn enclose_claim(&mut self, claim_id: &str) -> Response {
        match self.build_numeric_certificates(claim_id) {
            Ok((out, cert)) => {
                self.journal
                    .record(JournalEvent::enclose(claim_id, cert.to_hex()));
                Response::ok(out)
            }
            Err(e) => Response::err(e),
        }
    }

    /// Rebuild numeric certificates from live overlay strings. Does not
    /// journal and does not deserialize a recorded certificate hash.
    fn build_numeric_certificates(
        &mut self,
        claim_id: &str,
    ) -> Result<(String, physis_core::artifact::ArtifactId), String> {
        let mut found = false;
        let mut rows = Vec::new();
        for t in self.theories.values() {
            for (c, v) in t.evaluate_all() {
                if c.id_str() != claim_id {
                    continue;
                }
                found = true;
                rows.push((
                    t.id().to_string(),
                    c.statement_hash(),
                    v.derivation(),
                    v.numeric_lo().map(str::to_string),
                    v.numeric_hi().map(str::to_string),
                ));
            }
        }
        if !found {
            return Err(format!("unknown claim '{claim_id}'"));
        }

        let mut text = format!("enclose  {claim_id}\n");
        let mut cert_ids: Vec<physis_core::artifact::ArtifactId> = Vec::new();
        for (theory, hash, derivation, lo, hi) in rows {
            if derivation != DerivationAssurance::CertifiedNumeric {
                text.push_str(&format!(
                    "  skipped  {theory}  derivation {} (not certified-numeric)\n",
                    derivation.as_str()
                ));
                continue;
            }
            let (Some(lo), Some(hi)) = (lo, hi) else {
                return Err(format!(
                    "enclose {claim_id}: {theory} certified-numeric overlay has no enclosure strings"
                ));
            };
            let Some(lo_r) = Ratio::parse_display(&lo) else {
                return Err(format!(
                    "enclose {claim_id}: {theory} lower bound '{lo}' is not a canonical Ratio"
                ));
            };
            let Some(hi_r) = Ratio::parse_display(&hi) else {
                return Err(format!(
                    "enclose {claim_id}: {theory} upper bound '{hi}' is not a canonical Ratio"
                ));
            };
            if lo_r > hi_r {
                return Err(format!(
                    "enclose {claim_id}: {theory} reversed enclosure [{lo}, {hi}]"
                ));
            }
            let stmt = self.store.insert(Node::new(
                NodeKind::Statement,
                vec![],
                hash.to_hex().as_bytes(),
            ));
            let payload = format!("{lo}\t{hi}");
            let cert = self.store.insert(Node::new(
                NodeKind::NumericCertificate,
                vec![stmt],
                payload.as_bytes(),
            ));
            self.numeric_certs.insert(hash, cert);
            cert_ids.push(cert);
            text.push_str(&format!("  identity  {}\n", hash.to_hex()));
            text.push_str(&format!("    theory       {theory}\n"));
            text.push_str(&format!("    enclosure    [{lo}, {hi}]\n"));
            text.push_str(&format!("    certificate  {}\n", cert.to_hex()));
            text.push_str("    not a kernel proof; not P4; not Canonical\n");
        }

        if cert_ids.is_empty() {
            return Err(format!(
                "enclose {claim_id}: no certified-numeric enclosure to parse independently"
            ));
        }

        cert_ids.sort();
        let bundle = if cert_ids.len() == 1 {
            cert_ids[0]
        } else {
            self.store.insert(Node::new(
                NodeKind::NumericCertificate,
                cert_ids,
                claim_id.as_bytes(),
            ))
        };
        let mut out = format!("enclose  {claim_id}  certificate {}\n", bundle.to_hex());
        let prefix = format!("enclose  {claim_id}\n");
        if let Some(rest) = text.strip_prefix(&prefix) {
            out.push_str(rest);
        } else {
            out.push_str(&text);
        }
        Ok((out, bundle))
    }

    /// Independently rebuild a live [`physis_provenance::SourceRecord`].
    /// Datasets and catalog dossiers. Does not raise P3S, does not mint
    /// a kernel receipt, Canonical, or P4.
    fn cite_claim(&mut self, claim_id: &str) -> Response {
        match self.build_cite(claim_id) {
            Ok((out, id)) => {
                self.journal
                    .record(JournalEvent::cite(claim_id, id.to_hex()));
                Response::ok(out)
            }
            Err(e) => Response::err(e),
        }
    }

    /// Rebuild a Source node from live dataset or dossier fields. Does
    /// not journal and does not deserialize a recorded source hash.
    fn build_cite(
        &mut self,
        claim_id: &str,
    ) -> Result<(String, physis_core::artifact::ArtifactId), String> {
        let mut hashes = Vec::new();
        for t in self.theories.values() {
            for (c, _) in t.evaluate_all() {
                if c.id_str() == claim_id {
                    hashes.push((t.id().to_string(), c.statement_hash()));
                }
            }
        }
        if hashes.is_empty() {
            return Err(format!("unknown claim '{claim_id}'"));
        }

        let (kind, label, rec) = if let Some(ds) = physis_data::dataset_for_claim(claim_id) {
            let rec = ds
                .source
                .recheck()
                .map_err(|e| format!("cite {claim_id}: {e}"))?;
            ("dataset", ds.id, rec)
        } else {
            match physis_semantic::cite_source(claim_id) {
                Ok(rec) => {
                    let rec = rec.recheck().map_err(|e| format!("cite {claim_id}: {e}"))?;
                    ("dossier", "catalog".to_string(), rec)
                }
                Err(_) => {
                    return Err(format!("cite {claim_id}: no precise source artifact"));
                }
            }
        };

        let mut text = format!("cite  {claim_id}\n");
        text.push_str(&format!("  kind     {kind}  {label}\n"));
        text.push_str(&format!("  work     {}\n", rec.citation.work));
        text.push_str(&format!("  edition  {}\n", rec.citation.edition));
        text.push_str(&format!("  version  {}\n", rec.version));
        if let Some(eq) = &rec.locator.equation {
            text.push_str(&format!("  equation {eq}\n"));
        }
        if let Some(section) = &rec.locator.section {
            text.push_str(&format!("  section  {section}\n"));
        }
        if let Some(table) = &rec.locator.table {
            text.push_str(&format!("  table    {table}\n"));
        }
        if let Some(range) = &rec.locator.dataset_range {
            text.push_str(&format!("  range    {range}\n"));
        }
        if let Some(exp) = &rec.locator.experiment {
            text.push_str(&format!("  experiment {exp}\n"));
        }
        text.push_str(&format!("  record    {}\n", rec.source_hash));
        text.push_str("  not P3S; not a kernel proof; not P4; not Canonical\n");

        let payload = format!(
            "{}\t{}\t{}\t{}",
            rec.source_hash, rec.citation.work, rec.citation.edition, rec.version
        );
        let mut node_ids = Vec::new();
        for (theory, hash) in &hashes {
            let stmt = self.store.insert(Node::new(
                NodeKind::Statement,
                vec![],
                hash.to_hex().as_bytes(),
            ));
            let node =
                self.store
                    .insert(Node::new(NodeKind::Source, vec![stmt], payload.as_bytes()));
            self.cited_sources.insert(*hash, node);
            node_ids.push(node);
            text.push_str(&format!("  identity  {}  {theory}\n", hash.to_hex()));
        }
        node_ids.sort();
        let bundle = if node_ids.len() == 1 {
            node_ids[0]
        } else {
            self.store
                .insert(Node::new(NodeKind::Source, node_ids, claim_id.as_bytes()))
        };
        let mut out = format!("cite  {claim_id}  source {}\n", bundle.to_hex());
        let prefix = format!("cite  {claim_id}\n");
        if let Some(rest) = text.strip_prefix(&prefix) {
            out.push_str(rest);
        } else {
            out.push_str(&text);
        }
        Ok((out, bundle))
    }

    /// Independently rebuild a versioned physical constant from live
    /// constructors. Does not raise P3N or P3S, does not mint a kernel
    /// receipt, Canonical, or P4. Omitted name rebuilds the full ledger.
    fn constant_entry(&mut self, name: Option<&str>) -> Response {
        let result = match name.filter(|n| !n.is_empty()) {
            Some(name) => self.build_constant(name).map(|(out, id)| (out, id, name)),
            None => self.build_constant_ledger().map(|(out, id)| (out, id, "")),
        };
        match result {
            Ok((out, id, journal_name)) => {
                self.journal
                    .record(JournalEvent::constant(journal_name, id.to_hex()));
                Response::ok(out)
            }
            Err(e) => Response::err(e),
        }
    }

    /// Rebuild a VersionedConstant node from live constructors. Does not
    /// journal and does not deserialize a recorded node hash.
    fn build_constant(
        &mut self,
        name: &str,
    ) -> Result<(String, physis_core::artifact::ArtifactId), String> {
        let live =
            physis_constants::lookup(name).ok_or_else(|| format!("unknown constant '{name}'"))?;
        live.source
            .recheck()
            .map_err(|e| format!("constant {name}: {e}"))?;
        let again = physis_constants::lookup(name)
            .ok_or_else(|| format!("constant {name}: lookup failed on rebuild"))?;
        if again.hash != live.hash {
            return Err(format!("constant {name}: rebuilt hash does not match"));
        }
        if again.source.source_hash != live.source.source_hash {
            return Err(format!("constant {name}: rebuilt source does not match"));
        }
        let payload = format!(
            "{}\n{}\n{}\n{}\n{}",
            live.name,
            live.kind,
            live.value,
            live.unit,
            live.hash.to_hex()
        );
        let node = self.store.insert(Node::new(
            NodeKind::VersionedConstant,
            vec![],
            payload.as_bytes(),
        ));
        let mut text = format!("constant  {name}  node {}\n", node.to_hex());
        text.push_str(&format!("  hash     {}\n", live.hash.to_hex()));
        text.push_str(&format!("  kind     {}\n", live.kind));
        text.push_str(&format!("  value    {}\n", live.value));
        text.push_str(&format!("  unit     {}\n", live.unit));
        text.push_str(&format!("  release  {}\n", live.release.as_str()));
        if let Some(table) = &live.table {
            text.push_str(&format!("  table    {table}\n"));
        }
        if let Some(range) = &live.range {
            text.push_str(&format!("  range    {range}\n"));
        }
        text.push_str("  rebuild  ok\n");
        text.push_str("  not P3S; not P3N; not a kernel proof; not P4; not Canonical\n");
        Ok((text, node))
    }

    /// Rebuild every [`physis_constants::LEDGER`] entry and bundle the
    /// VersionedConstant nodes. Does not journal and does not deserialize
    /// a recorded node hash. Not P3N, not Canonical, not P4.
    fn build_constant_ledger(
        &mut self,
    ) -> Result<(String, physis_core::artifact::ArtifactId), String> {
        let mut payload = String::new();
        let mut blocks = String::new();
        let mut parents = Vec::new();
        for name in physis_constants::LEDGER {
            let (block, node) = self.build_constant(name)?;
            payload.push_str(&format!("{name} {}\n", node.to_hex()));
            parents.push(node);
            blocks.push_str(&block);
        }
        let bundle = self.store.insert(Node::new(
            NodeKind::VersionedConstant,
            parents,
            payload.as_bytes(),
        ));
        let mut text = format!("constant  ledger  node {}\n", bundle.to_hex());
        text.push_str(&blocks);
        Ok((text, bundle))
    }

    /// Independently parse, round-trip, and reconstruct a live theory
    /// IR package. A `lean_ref` must bind the catalog identity tree;
    /// encode lists each bound identity by claim id; token packages
    /// skip. Does not raise P3S, does not install mutants, and does
    /// not mint a kernel receipt, Canonical, or P4.
    fn encode_theory(&mut self, theory_id: &str) -> Response {
        match self.build_encoding(theory_id) {
            Ok((out, id)) => {
                self.journal
                    .record(JournalEvent::encode(theory_id, id.to_hex()));
                Response::ok(out)
            }
            Err(e) => Response::err(e),
        }
    }

    /// Rebuild an EncodingPackage from the live IR package. Does not
    /// journal and does not deserialize a recorded package hash.
    fn build_encoding(
        &mut self,
        theory_id: &str,
    ) -> Result<(String, physis_core::artifact::ArtifactId), String> {
        if !self.theories.contains_key(theory_id) {
            return Err(format!("unknown theory '{theory_id}'"));
        }
        let pkg = self.theories[theory_id]
            .ir_package()
            .ok_or_else(|| format!("encode {theory_id}: no IR package"))?;
        if pkg.id != theory_id {
            return Err(format!(
                "encode {theory_id}: package id '{}' is not the live theory",
                pkg.id
            ));
        }
        if pkg.equations.is_empty() {
            return Err(format!("encode {theory_id}: IR package has no equations"));
        }
        let canonical =
            physis_ir::certify_round_trip(&pkg).map_err(|e| format!("encode {theory_id}: {e}"))?;
        let parsed =
            physis_ir::parse_package(&canonical).map_err(|e| format!("encode {theory_id}: {e}"))?;
        let live_ids: BTreeSet<String> = self.theories[theory_id]
            .claims()
            .into_iter()
            .map(|c| c.id_str().to_string())
            .collect();
        for decl in &parsed.claims {
            if !live_ids.contains(&decl.id) {
                return Err(format!(
                    "encode {theory_id}: IR claim '{}' is not a live claim",
                    decl.id
                ));
            }
        }
        let rebuilt = self.theories[theory_id]
            .reparse_package(&parsed)
            .map_err(|e| format!("encode {theory_id}: {e}"))?;
        let rebuilt_pkg = rebuilt.ir_package().ok_or_else(|| {
            format!("encode {theory_id}: reconstructed encoding has no IR package")
        })?;
        if rebuilt_pkg != pkg {
            return Err(format!(
                "encode {theory_id}: reconstructed package does not match the live package"
            ));
        }
        catalog_tree_binding(parsed.lean_ref.as_deref(), &parsed.equations)
            .map_err(|e| format!("encode {theory_id}: {e}"))?;
        let node = self.store.insert(Node::new(
            NodeKind::EncodingPackage,
            vec![],
            canonical.as_bytes(),
        ));
        self.encoded_packages.insert(theory_id.to_string(), node);
        let mut text = format!("encode  {theory_id}  package {}\n", node.to_hex());
        text.push_str(&format!("  equations  {}\n", parsed.equations.len()));
        text.push_str(&format!("  claims     {}\n", parsed.claims.len()));
        text.push_str("  round-trip canonical\n");
        text.push_str("  reconstruct  ok\n");
        for spec in catalog_trees_in(&parsed.equations) {
            text.push_str(&format!("  catalog identity tree  {}\n", spec.claim_id));
        }
        text.push_str("  not P3S; not a kernel proof; not P4; not Canonical\n");
        Ok((text, node))
    }

    /// Independently rebuild [`Judgment::from_lab`] from live evaluator
    /// axes and receipts. Does not mint, does not raise P3S, and cannot
    /// deserialize `logical proved`.
    fn judge_claim(&mut self, claim_id: &str) -> Response {
        match self.build_judgment(claim_id) {
            Ok((out, id)) => {
                self.journal
                    .record(JournalEvent::judge(claim_id, id.to_hex()));
                Response::ok(out)
            }
            Err(e) => Response::err(e),
        }
    }

    /// Rebuild JudgmentProjection nodes from live from_lab. Does not
    /// journal and does not deserialize a recorded projection hash.
    fn build_judgment(
        &mut self,
        claim_id: &str,
    ) -> Result<(String, physis_core::artifact::ArtifactId), String> {
        let mut rows = Vec::new();
        for t in self.theories.values() {
            for (c, v) in t.evaluate_all() {
                if c.id_str() != claim_id {
                    continue;
                }
                let label = self.projected_judgment(&c, &v).label();
                rows.push((t.id().to_string(), c.statement_hash(), label));
            }
        }
        if rows.is_empty() {
            return Err(format!("unknown claim '{claim_id}'"));
        }

        let mut text = format!("judge  {claim_id}\n");
        let mut node_ids = Vec::new();
        for (theory, hash, label) in &rows {
            let stmt = self.store.insert(Node::new(
                NodeKind::Statement,
                vec![],
                hash.to_hex().as_bytes(),
            ));
            let node = self.store.insert(Node::new(
                NodeKind::JudgmentProjection,
                vec![stmt],
                label.as_bytes(),
            ));
            self.judged_projections.insert(*hash, node);
            node_ids.push(node);
            text.push_str(&format!("  identity  {}\n", hash.to_hex()));
            text.push_str(&format!("    theory      {theory}\n"));
            text.push_str(&format!("    judgment    {label}\n"));
            text.push_str(&format!("    projection  {node}\n"));
            text.push_str("    not P3S; not a kernel proof; not P4; not Canonical\n");
        }
        node_ids.sort();
        let bundle = if node_ids.len() == 1 {
            node_ids[0]
        } else {
            self.store.insert(Node::new(
                NodeKind::JudgmentProjection,
                node_ids,
                claim_id.as_bytes(),
            ))
        };
        let mut out = format!("judge  {claim_id}  projection {}\n", bundle.to_hex());
        let prefix = format!("judge  {claim_id}\n");
        if let Some(rest) = text.strip_prefix(&prefix) {
            out.push_str(rest);
        } else {
            out.push_str(&text);
        }
        Ok((out, bundle))
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
            "loop observe → hypothesize → prove → falsify → enclose → cite → constant → encode → judge → replicate → design → audit → review\n",
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

        let mut enclose_slugs = BTreeSet::new();
        for t in self.theories.values() {
            for (c, v) in t.evaluate_all() {
                if v.derivation() == DerivationAssurance::CertifiedNumeric {
                    enclose_slugs.insert(c.id_str().to_string());
                }
            }
        }
        for slug in enclose_slugs {
            match self.build_numeric_certificates(&slug) {
                Ok((_, cert)) => {
                    self.journal
                        .record(JournalEvent::enclose(&slug, cert.to_hex()));
                    text.push_str(&format!("enclose  {slug}  {}\n", cert.to_hex()));
                }
                Err(e) => text.push_str(&format!("enclose  {slug}  {e}\n")),
            }
        }

        let mut cite_slugs = BTreeSet::new();
        for spec in CATALOG {
            cite_slugs.insert(spec.claim_id.to_string());
        }
        cite_slugs.insert("gut.weinberg-angle-mz-interval".into());
        cite_slugs.insert("gut.proton-lifetime-sk".into());
        for slug in cite_slugs {
            match self.build_cite(&slug) {
                Ok((_, id)) => {
                    self.journal.record(JournalEvent::cite(&slug, id.to_hex()));
                    text.push_str(&format!("cite  {slug}  {}\n", id.to_hex()));
                }
                Err(e) => text.push_str(&format!("cite  {slug}  {e}\n")),
            }
        }

        match self.build_constant_ledger() {
            Ok((_, id)) => {
                self.journal.record(JournalEvent::constant("", id.to_hex()));
                text.push_str(&format!("constant  ledger  {}\n", id.to_hex()));
            }
            Err(e) => text.push_str(&format!("constant  ledger  {e}\n")),
        }

        let mut encode_ids = Vec::new();
        for (id, t) in &self.theories {
            if t.ir_package().is_some() {
                encode_ids.push(id.clone());
            }
        }
        for id in encode_ids {
            match self.build_encoding(&id) {
                Ok((_, pkg)) => {
                    self.journal.record(JournalEvent::encode(&id, pkg.to_hex()));
                    text.push_str(&format!("encode  {id}  {}\n", pkg.to_hex()));
                }
                Err(e) => text.push_str(&format!("encode  {id}  {e}\n")),
            }
        }

        let mut judge_slugs = BTreeSet::new();
        for spec in CATALOG {
            judge_slugs.insert(spec.claim_id.to_string());
        }
        judge_slugs.insert("predictivity.unique-vacuum".into());
        judge_slugs.insert("gut.proton-lifetime-sk".into());
        judge_slugs.insert("gut.weinberg-angle-mz-interval".into());
        judge_slugs.insert("gut.weinberg-angle".into());
        judge_slugs.insert("dec.closed-equals-exact".into());
        for slug in judge_slugs {
            match self.build_judgment(&slug) {
                Ok((_, id)) => {
                    self.journal.record(JournalEvent::judge(&slug, id.to_hex()));
                    text.push_str(&format!("judge  {slug}  {}\n", id.to_hex()));
                }
                Err(e) => text.push_str(&format!("judge  {slug}  {e}\n")),
            }
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
            "GQW running, the input-interval enclosure, and the 3% hit are not P3N: {p3n}"
        );
        assert!(
            !p3n.lines()
                .any(|l| l.contains("type-iib") && l.contains("consistency.anomaly-cancellation")),
            "Green-Schwarz stays encoded, not a Ratio certificate: {p3n}"
        );
        assert!(
            !p3n.lines()
                .any(|l| l.contains("heterotic-e8e8")
                    && l.contains("consistency.anomaly-cancellation")),
            "heterotic Green-Schwarz stays encoded, not P3N: {p3n}"
        );
        assert!(
            !p3n.lines()
                .any(|l| l.contains("heterotic-so32")
                    && l.contains("consistency.anomaly-cancellation")),
            "heterotic-so32 Green-Schwarz stays encoded, not P3N: {p3n}"
        );
        assert!(
            !p3n.lines().any(|l| {
                l.contains("type-i")
                    && !l.contains("type-iia")
                    && !l.contains("type-iib")
                    && l.contains("consistency.anomaly-cancellation")
            }),
            "Type I Green-Schwarz stays encoded, not P3N: {p3n}"
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
        assert!(sm_anom.contains("complete Q_L"), "{sm_anom}");
        assert!(
            !sm_anom.contains("not yet a machine-checked regime"),
            "SM anomalies must not be encoding-wide: {sm_anom}"
        );
        let gs = why_theory_block(&why, "type-iib");
        assert!(gs.contains("derivation: executed"), "{gs}");
        assert!(gs.contains("judgment:   logical undetermined"), "{gs}");
        assert!(!gs.contains("numeric certified"), "{gs}");
        let het_gs = why_theory_block(&why, "heterotic-e8e8");
        assert!(het_gs.contains("E8 x E8"), "{het_gs}");
        assert!(
            !het_gs.contains("not yet a machine-checked regime"),
            "heterotic Green-Schwarz must not be encoding-wide: {het_gs}"
        );
        assert!(het_gs.contains("derivation: executed"), "{het_gs}");
        assert!(!het_gs.contains("numeric certified"), "{het_gs}");
        let so32_gs = why_theory_block(&why, "heterotic-so32");
        assert!(so32_gs.contains("SO(32)"), "{so32_gs}");
        assert!(
            !so32_gs.contains("not yet a machine-checked regime"),
            "heterotic-so32 Green-Schwarz must not be encoding-wide: {so32_gs}"
        );
        assert!(so32_gs.contains("derivation: executed"), "{so32_gs}");
        assert!(!so32_gs.contains("numeric certified"), "{so32_gs}");
        let type_i_gs = why_theory_block(&why, "type-i");
        assert!(
            type_i_gs.contains("Chan-Paton SO(32)"),
            "Type I Green-Schwarz must name Chan-Paton SO(32): {type_i_gs}"
        );
        assert!(
            !type_i_gs.contains("not yet a machine-checked regime"),
            "Type I Green-Schwarz must not be encoding-wide: {type_i_gs}"
        );
        assert!(type_i_gs.contains("derivation: executed"), "{type_i_gs}");
        assert!(!type_i_gs.contains("numeric certified"), "{type_i_gs}");
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
        assert!(
            mzb.contains("pdg-2022-alpha-s-mz") && mzb.contains("pdg-2022-inv-alpha-em-mz"),
            "input listings must be on the identity: {mzb}"
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
        assert!(folklore.contains("verdict:    holds"), "{folklore}");
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
    fn correspondence_cells_name_a_domain() {
        let mut lab = Lab::standard();
        let hi = lab
            .exec(Command::Why {
                claim: "thermo.high-t-classical".into(),
            })
            .text()
            .to_string();
        let hb = why_theory_block(&hi, "debye-solid");
        assert!(hb.contains("T/Θ ≥ 8"), "{hb}");
        assert!(
            !hb.contains("not yet a machine-checked regime"),
            "high-T correspondence must not be encoding-wide: {hb}"
        );

        let t3 = lab
            .exec(Command::Why {
                claim: "thermo.debye-t3".into(),
            })
            .text()
            .to_string();
        let tb = why_theory_block(&t3, "debye-solid");
        assert!(tb.contains("Θ/20"), "{tb}");
        assert!(
            !tb.contains("not yet a machine-checked regime"),
            "Debye T³ must name the low-T probe: {tb}"
        );

        let dp = lab
            .exec(Command::Why {
                claim: "thermo.dulong-petit".into(),
            })
            .text()
            .to_string();
        let dpb = why_theory_block(&dp, "debye-solid");
        assert!(
            dpb.contains("not yet a machine-checked regime"),
            "Dulong–Petit on debye-solid at the current T stays encoding-wide: {dpb}"
        );
        let dpb_dp = why_theory_block(&dp, "dulong-petit");
        assert!(
            dpb_dp.contains("U = 3 N k T") || dpb_dp.contains("harmonic"),
            "dulong-petit must name harmonic U = 3 N k T: {dpb_dp}"
        );
        assert!(
            !dpb_dp.contains("not yet a machine-checked regime"),
            "dulong-petit Dulong–Petit must not be encoding-wide: {dpb_dp}"
        );

        let rj = lab
            .exec(Command::Why {
                claim: "thermo.rj-ir-limit".into(),
            })
            .text()
            .to_string();
        let rjb = why_theory_block(&rj, "planck");
        assert!(rjb.contains("0.01 kT") || rjb.contains("hν"), "{rjb}");
        assert!(
            !rjb.contains("not yet a machine-checked regime"),
            "RJ infrared correspondence must not be encoding-wide: {rjb}"
        );

        let area = lab
            .exec(Command::Why {
                claim: "gauge.exact-area-law-2d".into(),
            })
            .text()
            .to_string();
        let ab = why_theory_block(&area, "wilson-u1");
        assert!(ab.contains("2D"), "{ab}");
        assert!(
            !ab.contains("not yet a machine-checked regime"),
            "exact area law must name 2D: {ab}"
        );
        let poincare = lab
            .exec(Command::Why {
                claim: "dec.closed-equals-exact".into(),
            })
            .text()
            .to_string();
        let pb = why_theory_block(&poincare, "de-rham");
        assert!(
            pb.contains("not yet a machine-checked regime"),
            "Poincaré stays encoding-wide: {pb}"
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
        let marker = "add-feedback: package → add-feedback";
        let start = text.find(marker).expect("add-feedback hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  combinational-circuit  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let feedback_block = &rest[..end];
        assert!(
            feedback_block.contains("comp.acyclic") && feedback_block.contains("holds → fails"),
            "add-feedback must flip acyclic holds to fails: {feedback_block}"
        );
        assert!(
            feedback_block.contains("comp.halts")
                && feedback_block.contains("holds → inapplicable"),
            "add-feedback must make halts inapplicable: {feedback_block}"
        );
        assert!(
            !feedback_block.contains("comp.deterministic"),
            "add-feedback is not the contention fork: {feedback_block}"
        );
        assert!(
            text.contains("add-contention"),
            "contention must still be an IR fork: {text}"
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
    fn hypothesize_circuit_contention_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        for knob in ["contention", "bus", "nondeterministic"] {
            let blocked = lab.exec(Command::Set {
                theory: "combinational-circuit".into(),
                knob: knob.into(),
                value: "true".into(),
            });
            assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
            assert!(
                blocked.text().contains("unknown knob") || blocked.text().contains(knob),
                "{}",
                blocked.text()
            );
        }

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
            text.contains("add-contention") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-contention: package → add-contention";
        let start = text.find(marker).expect("add-contention hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  combinational-circuit  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let contention_block = &rest[..end];
        assert!(
            contention_block.contains("comp.deterministic")
                && contention_block.contains("holds → fails"),
            "add-contention must flip deterministic holds to fails: {contention_block}"
        );
        assert!(
            !contention_block.contains("comp.acyclic"),
            "add-contention is not the feedback cycle fork: {contention_block}"
        );
        assert!(
            !contention_block.contains("comp.halts"),
            "add-contention is not the feedback halts fork: {contention_block}"
        );
        assert!(
            text.contains("add-feedback"),
            "feedback must still be an IR fork: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("combinational-circuit").unwrap();
        assert!(
            live.evaluate_all()
                .iter()
                .any(|(c, v)| c.id_str() == "comp.deterministic" && v.kind == VerdictKind::Holds),
            "IR mutant must not be installed"
        );
        assert!(
            live.evaluate_all()
                .iter()
                .any(|(c, v)| c.id_str() == "comp.acyclic" && v.kind == VerdictKind::Holds),
            "feedback mutant must not be installed"
        );
        let tm = lab.theory("turing-machine").unwrap();
        assert_eq!(
            tm.get("nondeterministic").unwrap().display(),
            "false",
            "NAND contention IR must not convert the Turing-machine nondeterministic knob"
        );
        let why = lab
            .exec(Command::Why {
                claim: "comp.deterministic".into(),
            })
            .text()
            .to_string();
        let nand = why_theory_block(&why, "combinational-circuit");
        assert!(
            nand.contains("unique NAND drivers"),
            "combinational determinism must name unique NAND drivers: {nand}"
        );
        assert!(
            !nand.contains("not yet a machine-checked regime"),
            "combinational determinism must not be encoding-wide: {nand}"
        );
        let tm_why = why_theory_block(&why, "turing-machine");
        assert!(
            tm_why.contains("not yet a machine-checked regime"),
            "TM determinism stays encoding-wide: {tm_why}"
        );
    }

    #[test]
    fn hypothesize_klein_gordon_next_nearest_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        let blocked = lab.exec(Command::Set {
            theory: "klein-gordon".into(),
            knob: "next_nearest".into(),
            value: "true".into(),
        });
        assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
        assert!(
            blocked.text().contains("unknown knob") || blocked.text().contains("next_nearest"),
            "{}",
            blocked.text()
        );

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("klein-gordon".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-next-nearest") && text.contains("ir structural"),
            "{text}"
        );
        assert!(
            text.contains("field.local") && text.contains("holds → fails"),
            "{text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("klein-gordon").unwrap();
        assert!(
            live.evaluate_all()
                .iter()
                .any(|(c, v)| c.id_str() == "field.local" && v.kind == VerdictKind::Holds),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("spacing").unwrap().display(),
            "1",
            "hypothesize must restore knobs"
        );
    }

    #[test]
    fn hypothesize_klein_gordon_quartic_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        for knob in ["quartic", "phi4", "lambda"] {
            let blocked = lab.exec(Command::Set {
                theory: "klein-gordon".into(),
                knob: knob.into(),
                value: "true".into(),
            });
            assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
            assert!(
                blocked.text().contains("unknown knob") || blocked.text().contains(knob),
                "{}",
                blocked.text()
            );
        }

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("klein-gordon".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-quartic") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-quartic: package → add-quartic";
        let start = text.find(marker).expect("add-quartic hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  klein-gordon  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let quartic_block = &rest[..end];
        assert!(
            quartic_block.contains("field.stable") && quartic_block.contains("holds → fails"),
            "add-quartic must flip field.stable holds to fails: {quartic_block}"
        );
        assert!(
            !quartic_block.contains("field.causal"),
            "add-quartic is not the mass_squared tachyon: {quartic_block}"
        );
        assert!(
            !quartic_block.contains("field.local"),
            "add-quartic is not the next-nearest fork: {quartic_block}"
        );
        assert!(
            text.contains("add-next-nearest"),
            "next-nearest must still be an IR fork: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("klein-gordon").unwrap();
        assert!(
            live.evaluate_all()
                .iter()
                .any(|(c, v)| c.id_str() == "field.stable" && v.kind == VerdictKind::Holds),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("mass_squared").unwrap().display(),
            "1",
            "hypothesize must restore knobs"
        );
        let df = lab.theory("dirac-fermion").unwrap();
        assert_eq!(
            df.get("mass").unwrap().display(),
            "1",
            "KG quartic IR must not convert the Dirac mass knob"
        );
        let why = lab
            .exec(Command::Why {
                claim: "field.stable".into(),
            })
            .text()
            .to_string();
        let kg = why_theory_block(&why, "klein-gordon");
        assert!(
            kg.contains("quadratic Klein-Gordon potential"),
            "KG stability must name quadratic potential: {kg}"
        );
        assert!(
            !kg.contains("not yet a machine-checked regime"),
            "KG stability must not be encoding-wide: {kg}"
        );
    }

    #[test]
    fn hypothesize_dirac_fermion_wilson_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        for knob in ["wilson", "r", "wilson_r"] {
            let blocked = lab.exec(Command::Set {
                theory: "dirac-fermion".into(),
                knob: knob.into(),
                value: "true".into(),
            });
            assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
            assert!(
                blocked.text().contains("unknown knob") || blocked.text().contains(knob),
                "{}",
                blocked.text()
            );
        }
        let ms_blocked = lab.exec(Command::Set {
            theory: "dirac-fermion".into(),
            knob: "mass_squared".into(),
            value: "-1".into(),
        });
        assert_eq!(ms_blocked.exit_code(), 1, "{}", ms_blocked.text());

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("dirac-fermion".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-wilson") && text.contains("ir structural"),
            "{text}"
        );
        assert!(
            text.contains("add-next-nearest"),
            "next-nearest must still be an IR fork: {text}"
        );
        let marker = "add-wilson: package → add-wilson";
        let start = text.find(marker).expect("add-wilson hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  dirac-fermion  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let wilson_block = &rest[..end];
        assert!(
            wilson_block.contains("fermion.no-doublers") && wilson_block.contains("fails → holds"),
            "add-wilson must flip no-doublers fails to holds: {wilson_block}"
        );
        assert!(
            !wilson_block.contains("field.local"),
            "add-wilson is not the next-nearest hopping fork: {wilson_block}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("dirac-fermion").unwrap();
        assert!(
            live.evaluate_all()
                .iter()
                .any(|(c, v)| c.id_str() == "fermion.no-doublers" && v.kind == VerdictKind::Fails),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("mass").unwrap().display(),
            "1",
            "hypothesize must restore knobs"
        );
        assert_eq!(
            live.get("sites").unwrap().display(),
            "16",
            "hypothesize must restore knobs"
        );
        assert_eq!(
            live.get("spacing").unwrap().display(),
            "1",
            "hypothesize must restore knobs"
        );
        let kg = lab.theory("klein-gordon").unwrap();
        assert_eq!(
            kg.get("mass_squared").unwrap().display(),
            "1",
            "Dirac IR must not convert the Klein-Gordon mass_squared knob"
        );
        assert!(
            kg.evaluate_all()
                .iter()
                .any(|(c, v)| c.id_str() == "field.local" && v.kind == VerdictKind::Holds),
            "Klein-Gordon must stay the live nearest-neighbour object"
        );
        let why = lab
            .exec(Command::Why {
                claim: "fermion.no-doublers".into(),
            })
            .text()
            .to_string();
        let df = why_theory_block(&why, "dirac-fermion");
        assert!(
            df.contains("naive 1D lattice Dirac"),
            "dirac no-doublers must name naive Dirac: {df}"
        );
        assert!(
            !df.contains("not yet a machine-checked regime"),
            "dirac no-doublers must not be encoding-wide: {df}"
        );
        let local = lab
            .exec(Command::Why {
                claim: "field.local".into(),
            })
            .text()
            .to_string();
        let kg_local = why_theory_block(&local, "klein-gordon");
        assert!(
            kg_local.contains("nearest-neighbour 1D periodic lattice"),
            "KG locality must stay named: {kg_local}"
        );
        let df_local = why_theory_block(&local, "dirac-fermion");
        assert!(
            df_local.contains("nearest-neighbour 1D lattice Dirac"),
            "Dirac locality must name nearest-neighbour hopping: {df_local}"
        );
        assert!(
            !df_local.contains("not yet a machine-checked regime"),
            "Dirac locality must not be encoding-wide: {df_local}"
        );
        assert!(
            !df_local.contains("nearest-neighbour 1D periodic lattice"),
            "Dirac locality is not the KG Laplacian cell: {df_local}"
        );
    }

    #[test]
    fn hypothesize_dirac_fermion_next_nearest_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        for knob in ["next_nearest", "nnn", "hopping"] {
            let blocked = lab.exec(Command::Set {
                theory: "dirac-fermion".into(),
                knob: knob.into(),
                value: "true".into(),
            });
            assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
            assert!(
                blocked.text().contains("unknown knob") || blocked.text().contains(knob),
                "{}",
                blocked.text()
            );
        }

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("dirac-fermion".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-next-nearest") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-next-nearest: package → add-next-nearest";
        let start = text.find(marker).expect("add-next-nearest hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  dirac-fermion  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let nnn_block = &rest[..end];
        assert!(
            nnn_block.contains("field.local") && nnn_block.contains("holds → fails"),
            "add-next-nearest must flip field.local holds to fails: {nnn_block}"
        );
        assert!(
            !nnn_block.contains("fermion.no-doublers"),
            "add-next-nearest is not the Wilson doubling fork: {nnn_block}"
        );
        assert!(
            !nnn_block.contains("field.stable"),
            "add-next-nearest is not the KG quartic fork: {nnn_block}"
        );
        assert!(
            text.contains("add-wilson"),
            "wilson must still be an IR fork: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("dirac-fermion").unwrap();
        assert!(
            live.evaluate_all()
                .iter()
                .any(|(c, v)| c.id_str() == "field.local" && v.kind == VerdictKind::Holds),
            "IR mutant must not be installed"
        );
        assert!(
            live.evaluate_all().iter().any(|(c, v)| {
                c.id_str() == "fermion.no-doublers" && v.kind == VerdictKind::Fails
            }),
            "Wilson mutant must not be installed"
        );
        assert_eq!(
            live.get("mass").unwrap().display(),
            "1",
            "hypothesize must restore knobs"
        );
        let kg = lab.theory("klein-gordon").unwrap();
        assert_eq!(
            kg.get("mass_squared").unwrap().display(),
            "1",
            "Dirac nnn IR must not convert the Klein-Gordon mass_squared knob"
        );
        let why = lab
            .exec(Command::Why {
                claim: "field.local".into(),
            })
            .text()
            .to_string();
        let df = why_theory_block(&why, "dirac-fermion");
        assert!(
            df.contains("nearest-neighbour 1D lattice Dirac"),
            "Dirac locality must name nearest-neighbour hopping: {df}"
        );
        assert!(
            !df.contains("not yet a machine-checked regime"),
            "Dirac locality must not be encoding-wide: {df}"
        );
        let kg_local = why_theory_block(&why, "klein-gordon");
        assert!(
            kg_local.contains("nearest-neighbour 1D periodic lattice"),
            "KG locality must stay the Laplacian cell: {kg_local}"
        );
    }

    #[test]
    fn hypothesize_wilson_u1_rectangle_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        let blocked = lab.exec(Command::Set {
            theory: "wilson-u1".into(),
            knob: "rectangle".into(),
            value: "true".into(),
        });
        assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
        assert!(
            blocked.text().contains("unknown knob") || blocked.text().contains("rectangle"),
            "{}",
            blocked.text()
        );

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("wilson-u1".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-rectangle") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-rectangle: package → add-rectangle";
        let start = text.find(marker).expect("add-rectangle hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  wilson-u1  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let rect_block = &rest[..end];
        assert!(
            rect_block.contains("gauge.local") && rect_block.contains("holds → fails"),
            "add-rectangle must flip gauge.local holds to fails: {rect_block}"
        );
        assert!(
            text.contains("add-higgs"),
            "higgs must still be an IR fork: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("wilson-u1").unwrap();
        assert!(
            live.evaluate_all()
                .iter()
                .any(|(c, v)| c.id_str() == "gauge.local" && v.kind == VerdictKind::Holds),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("beta").unwrap().display(),
            "1",
            "hypothesize must restore knobs"
        );
    }

    #[test]
    fn hypothesize_wilson_su3_rectangle_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        let blocked = lab.exec(Command::Set {
            theory: "wilson-su3".into(),
            knob: "rectangle".into(),
            value: "true".into(),
        });
        assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
        assert!(
            blocked.text().contains("unknown knob") || blocked.text().contains("rectangle"),
            "{}",
            blocked.text()
        );

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("wilson-su3".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-rectangle") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-rectangle: package → add-rectangle";
        let start = text.find(marker).expect("add-rectangle hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  wilson-su3  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let rect_block = &rest[..end];
        assert!(
            rect_block.contains("gauge.local") && rect_block.contains("holds → fails"),
            "add-rectangle must flip gauge.local holds to fails: {rect_block}"
        );
        assert!(
            text.contains("add-higgs"),
            "higgs must still be an IR fork: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("wilson-su3").unwrap();
        assert!(
            live.evaluate_all()
                .iter()
                .any(|(c, v)| c.id_str() == "gauge.local" && v.kind == VerdictKind::Holds),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("beta").unwrap().display(),
            "6",
            "hypothesize must restore knobs"
        );
    }

    #[test]
    fn hypothesize_wilson_u1_higgs_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        for knob in ["higgs", "scalar", "vev"] {
            let blocked = lab.exec(Command::Set {
                theory: "wilson-u1".into(),
                knob: knob.into(),
                value: "true".into(),
            });
            assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
            assert!(
                blocked.text().contains("unknown knob") || blocked.text().contains(knob),
                "{}",
                blocked.text()
            );
        }

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("wilson-u1".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-higgs") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-higgs: package → add-higgs";
        let start = text.find(marker).expect("add-higgs hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  wilson-u1  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let higgs_block = &rest[..end];
        assert!(
            higgs_block.contains("gauge.confining") && higgs_block.contains("holds → fails"),
            "add-higgs must flip gauge.confining holds to fails: {higgs_block}"
        );
        assert!(
            !higgs_block.contains("gauge.local"),
            "add-higgs is not the rectangle locality fork: {higgs_block}"
        );
        assert!(
            text.contains("add-rectangle"),
            "add-rectangle must still be an IR fork: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("wilson-u1").unwrap();
        assert!(
            live.evaluate_all()
                .iter()
                .any(|(c, v)| c.id_str() == "gauge.confining" && v.kind == VerdictKind::Holds),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("beta").unwrap().display(),
            "1",
            "hypothesize must restore knobs"
        );
        let beta = lab.exec(Command::Set {
            theory: "wilson-u1".into(),
            knob: "beta".into(),
            value: "2".into(),
        });
        assert_eq!(beta.exit_code(), 0, "{}", beta.text());
        assert!(
            beta.text().contains("gauge.confining") && beta.text().contains("holds → fails"),
            "{}",
            beta.text()
        );
        let _ = lab.exec(Command::Set {
            theory: "wilson-u1".into(),
            knob: "beta".into(),
            value: "1".into(),
        });
        let why = lab
            .exec(Command::Why {
                claim: "gauge.confining".into(),
            })
            .text()
            .to_string();
        let u1 = why_theory_block(&why, "wilson-u1");
        assert!(
            u1.contains("pure Wilson gauge field"),
            "confining must name pure Wilson gauge: {u1}"
        );
        assert!(
            !u1.contains("not yet a machine-checked regime"),
            "confining must not be encoding-wide: {u1}"
        );
    }

    #[test]
    fn hypothesize_wilson_su3_higgs_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        let blocked = lab.exec(Command::Set {
            theory: "wilson-su3".into(),
            knob: "higgs".into(),
            value: "true".into(),
        });
        assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("wilson-su3".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("add-higgs") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-higgs: package → add-higgs";
        let start = text.find(marker).expect("add-higgs hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  wilson-su3  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let higgs_block = &rest[..end];
        assert!(
            higgs_block.contains("gauge.confining") && higgs_block.contains("holds → fails"),
            "add-higgs must flip gauge.confining holds to fails: {higgs_block}"
        );
        assert!(
            !higgs_block.contains("gauge.local"),
            "add-higgs is not the rectangle locality fork: {higgs_block}"
        );
        assert!(
            text.contains("add-rectangle"),
            "add-rectangle must still be an IR fork: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("wilson-su3").unwrap();
        assert!(
            live.evaluate_all()
                .iter()
                .any(|(c, v)| c.id_str() == "gauge.confining" && v.kind == VerdictKind::Holds),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("beta").unwrap().display(),
            "6",
            "hypothesize must restore knobs"
        );
        let why = lab
            .exec(Command::Why {
                claim: "gauge.confining".into(),
            })
            .text()
            .to_string();
        let su3 = why_theory_block(&why, "wilson-su3");
        assert!(
            su3.contains("pure Wilson gauge field"),
            "confining must name pure Wilson gauge: {su3}"
        );
        assert!(
            !su3.contains("not yet a machine-checked regime"),
            "confining must not be encoding-wide: {su3}"
        );
    }

    #[test]
    fn hypothesize_ohm_circuit_tline_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        let blocked = lab.exec(Command::Set {
            theory: "ohm-circuit".into(),
            knob: "tline".into(),
            value: "true".into(),
        });
        assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
        assert!(
            blocked.text().contains("unknown knob") || blocked.text().contains("tline"),
            "{}",
            blocked.text()
        );

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("ohm-circuit".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-tline") && text.contains("ir structural"),
            "{text}"
        );
        assert!(
            text.contains("add-flux"),
            "flux must still be an IR fork: {text}"
        );
        let marker = "add-tline: package → add-tline";
        let start = text.find(marker).expect("add-tline hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  ohm-circuit  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let tline_block = &rest[..end];
        assert!(
            tline_block.contains("em.charge-conservation") && tline_block.contains("holds → fails"),
            "add-tline must flip charge-conservation holds to fails: {tline_block}"
        );
        assert!(
            !tline_block.contains("em.faraday"),
            "add-tline is not the mesh-flux fork: {tline_block}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("ohm-circuit").unwrap();
        assert!(
            live.evaluate_all().iter().any(
                |(c, v)| c.id_str() == "em.charge-conservation" && v.kind == VerdictKind::Holds
            ),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("frequency_hz").unwrap().display(),
            "1000",
            "hypothesize must restore knobs"
        );
        assert!(
            live.evaluate_all()
                .iter()
                .any(|(c, v)| c.id_str() == "em.faraday" && v.kind == VerdictKind::Holds),
            "flux mutant must not be installed"
        );
    }

    #[test]
    fn hypothesize_ohm_circuit_flux_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        for knob in ["flux", "dPhi", "dphi"] {
            let blocked = lab.exec(Command::Set {
                theory: "ohm-circuit".into(),
                knob: knob.into(),
                value: "true".into(),
            });
            assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
            assert!(
                blocked.text().contains("unknown knob") || blocked.text().contains(knob),
                "{}",
                blocked.text()
            );
        }

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("ohm-circuit".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-flux") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-flux: package → add-flux";
        let start = text.find(marker).expect("add-flux hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  ohm-circuit  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let flux_block = &rest[..end];
        assert!(
            flux_block.contains("em.faraday") && flux_block.contains("holds → fails"),
            "add-flux must flip em.faraday holds to fails: {flux_block}"
        );
        assert!(
            !flux_block.contains("em.charge-conservation"),
            "add-flux is not the tline KCL fork: {flux_block}"
        );
        assert!(
            !flux_block.contains("em.constitutive-linear"),
            "add-flux is not the Tellegen fork: {flux_block}"
        );
        assert!(
            !flux_block.contains("em.quasi-static-valid"),
            "add-flux is not the frequency_hz probe: {flux_block}"
        );
        assert!(
            text.contains("add-tline"),
            "tline must still be an IR fork: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("ohm-circuit").unwrap();
        assert!(
            live.evaluate_all()
                .iter()
                .any(|(c, v)| c.id_str() == "em.faraday" && v.kind == VerdictKind::Holds),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("frequency_hz").unwrap().display(),
            "1000",
            "hypothesize must restore knobs"
        );
        let why = lab
            .exec(Command::Why {
                claim: "em.faraday".into(),
            })
            .text()
            .to_string();
        let ohm = why_theory_block(&why, "ohm-circuit");
        assert!(
            ohm.contains("lumped Kirchhoff voltage"),
            "ohm Faraday must name lumped KVL: {ohm}"
        );
        assert!(
            !ohm.contains("not yet a machine-checked regime"),
            "ohm Faraday must not be encoding-wide: {ohm}"
        );
        assert!(
            !ohm.contains("source-free homogeneous dF=0"),
            "ohm Faraday is not Maxwell dF=0: {ohm}"
        );
        let mx = why_theory_block(&why, "maxwell-vacuum");
        assert!(
            mx.contains("source-free homogeneous dF=0"),
            "Maxwell Faraday must name dF=0: {mx}"
        );
        let lm = why_theory_block(&why, "linear-medium");
        assert!(
            lm.contains("not yet a machine-checked regime"),
            "linear-medium Faraday stays encoding-wide: {lm}"
        );
    }

    #[test]
    fn hypothesize_bell_test_product_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        let blocked = lab.exec(Command::Set {
            theory: "bell-test".into(),
            knob: "product".into(),
            value: "true".into(),
        });
        assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
        assert!(
            blocked.text().contains("unknown knob") || blocked.text().contains("product"),
            "{}",
            blocked.text()
        );

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("bell-test".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-product") && text.contains("ir structural"),
            "{text}"
        );
        assert!(
            text.contains("quantum.bell-violation") && text.contains("holds → fails"),
            "{text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("bell-test").unwrap();
        assert!(
            live.evaluate_all().iter().any(|(c, v)| {
                c.id_str() == "quantum.bell-violation" && v.kind == VerdictKind::Holds
            }),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("visibility").unwrap().display(),
            "1",
            "hypothesize must restore knobs"
        );
        assert!(
            text.contains("add-pr-box"),
            "PR-box must still be an IR fork: {text}"
        );
    }

    #[test]
    fn hypothesize_bell_test_prbox_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        for knob in ["prbox", "pr-box", "pr_box"] {
            let blocked = lab.exec(Command::Set {
                theory: "bell-test".into(),
                knob: knob.into(),
                value: "true".into(),
            });
            assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
            assert!(
                blocked.text().contains("unknown knob") || blocked.text().contains(knob),
                "{}",
                blocked.text()
            );
        }

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("bell-test".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-pr-box") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-pr-box: package → add-pr-box";
        let start = text.find(marker).expect("add-pr-box hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  bell-test  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let prbox_block = &rest[..end];
        assert!(
            prbox_block.contains("quantum.tsirelson-bound")
                && prbox_block.contains("holds → fails"),
            "add-pr-box must flip tsirelson holds to fails: {prbox_block}"
        );
        assert!(
            !prbox_block.contains("quantum.bell-violation"),
            "add-pr-box is not the product-ket fork: {prbox_block}"
        );
        assert!(
            text.contains("add-product"),
            "product must still be an IR fork: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("bell-test").unwrap();
        assert!(
            live.evaluate_all().iter().any(|(c, v)| {
                c.id_str() == "quantum.tsirelson-bound" && v.kind == VerdictKind::Holds
            }),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("visibility").unwrap().display(),
            "1",
            "hypothesize must restore knobs"
        );
        let why = lab
            .exec(Command::Why {
                claim: "quantum.tsirelson-bound".into(),
            })
            .text()
            .to_string();
        let bell = why_theory_block(&why, "bell-test");
        assert!(
            bell.contains("Hilbert-space CHSH (Tsirelson 2√2)"),
            "Bell Tsirelson must name Hilbert-space CHSH: {bell}"
        );
        assert!(
            !bell.contains("not yet a machine-checked regime"),
            "Bell Tsirelson must not be encoding-wide: {bell}"
        );
    }

    #[test]
    fn hypothesize_newtonian_gravity_schwarzschild_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        let blocked = lab.exec(Command::Set {
            theory: "newtonian-gravity".into(),
            knob: "schwarzschild".into(),
            value: "true".into(),
        });
        assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
        assert!(
            blocked.text().contains("unknown knob") || blocked.text().contains("schwarzschild"),
            "{}",
            blocked.text()
        );
        let dim_blocked = lab.exec(Command::Set {
            theory: "newtonian-gravity".into(),
            knob: "dim".into(),
            value: "5".into(),
        });
        assert_eq!(dim_blocked.exit_code(), 1, "{}", dim_blocked.text());

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("newtonian-gravity".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-schwarzschild") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-schwarzschild: package → add-schwarzschild";
        let start = text.find(marker).expect("add-schwarzschild hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  newtonian-gravity  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let schwarzschild_block = &rest[..end];
        assert!(
            schwarzschild_block.contains("gr.newton-half-deflection")
                && schwarzschild_block.contains("holds → fails"),
            "add-schwarzschild must flip newton-half holds to fails: {schwarzschild_block}"
        );
        assert!(
            schwarzschild_block.contains("gr.eddington-deflection")
                && schwarzschild_block.contains("fails → holds"),
            "add-schwarzschild must flip Eddington fails to holds: {schwarzschild_block}"
        );
        assert!(
            text.contains("add-yukawa"),
            "yukawa must still be an IR fork: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("newtonian-gravity").unwrap();
        assert!(
            live.evaluate_all().iter().any(|(c, v)| {
                c.id_str() == "gr.newton-half-deflection" && v.kind == VerdictKind::Holds
            }),
            "IR mutant must not be installed"
        );
        assert_eq!(live.id(), "newtonian-gravity");
        let gr = lab.theory("general-relativity").unwrap();
        assert_eq!(
            gr.get("dim").unwrap().display(),
            "4",
            "Newton IR must not convert GR dim"
        );
        assert!(
            gr.evaluate_all().iter().any(|(c, v)| {
                c.id_str() == "gr.eddington-deflection" && v.kind == VerdictKind::Holds
            }),
            "GR must stay the live Schwarzschild object"
        );
    }

    #[test]
    fn hypothesize_newtonian_gravity_yukawa_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        for knob in ["yukawa", "mu"] {
            let blocked = lab.exec(Command::Set {
                theory: "newtonian-gravity".into(),
                knob: knob.into(),
                value: "true".into(),
            });
            assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
            assert!(
                blocked.text().contains("unknown knob") || blocked.text().contains(knob),
                "{}",
                blocked.text()
            );
        }

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("newtonian-gravity".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-yukawa") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-yukawa: package → add-yukawa";
        let start = text.find(marker).expect("add-yukawa hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  newtonian-gravity  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let yukawa_block = &rest[..end];
        assert!(
            yukawa_block.contains("gr.newton-half-deflection")
                && yukawa_block.contains("holds → fails"),
            "add-yukawa must flip newton-half holds to fails: {yukawa_block}"
        );
        assert!(
            !yukawa_block.contains("gr.eddington-deflection"),
            "add-yukawa is not the Schwarzschild Eddington fork: {yukawa_block}"
        );
        assert!(
            !yukawa_block.contains("gr.mercury-perihelion"),
            "add-yukawa is not the Schwarzschild Mercury fork: {yukawa_block}"
        );
        assert!(
            !yukawa_block.contains("em.constitutive-linear"),
            "add-yukawa is not the Tellegen fork: {yukawa_block}"
        );
        assert!(
            text.contains("add-schwarzschild"),
            "schwarzschild must still be an IR fork: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("newtonian-gravity").unwrap();
        assert!(
            live.evaluate_all().iter().any(|(c, v)| {
                c.id_str() == "gr.newton-half-deflection" && v.kind == VerdictKind::Holds
            }),
            "IR mutant must not be installed"
        );
        assert_eq!(live.id(), "newtonian-gravity");
        let gr = lab.theory("general-relativity").unwrap();
        assert_eq!(
            gr.get("dim").unwrap().display(),
            "4",
            "Newton IR must not convert GR dim"
        );
        let why = lab
            .exec(Command::Why {
                claim: "gr.newton-half-deflection".into(),
            })
            .text()
            .to_string();
        let newton = why_theory_block(&why, "newtonian-gravity");
        assert!(
            newton.contains("inverse-square Binet rhs"),
            "Newton half-angle must name inverse-square Binet: {newton}"
        );
        assert!(
            !newton.contains("not yet a machine-checked regime"),
            "Newton half-angle must not be encoding-wide: {newton}"
        );
        let gr_why = why_theory_block(&why, "general-relativity");
        assert!(
            gr_why.contains("not yet a machine-checked regime"),
            "GR solar cells stay encoding-wide: {gr_why}"
        );
    }

    #[test]
    fn hypothesize_general_relativity_r_squared_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        for knob in ["r_squared", "starobinsky", "quadratic"] {
            let blocked = lab.exec(Command::Set {
                theory: "general-relativity".into(),
                knob: knob.into(),
                value: "true".into(),
            });
            assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
            assert!(
                blocked.text().contains("unknown knob") || blocked.text().contains(knob),
                "{}",
                blocked.text()
            );
        }
        let newton_blocked = lab.exec(Command::Set {
            theory: "general-relativity".into(),
            knob: "schwarzschild".into(),
            value: "true".into(),
        });
        assert_eq!(newton_blocked.exit_code(), 1, "{}", newton_blocked.text());

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("general-relativity".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-r-squared") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-r-squared: package → add-r-squared";
        let start = text.find(marker).expect("add-r-squared hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  general-relativity  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let r2_block = &rest[..end];
        assert!(
            r2_block.contains("predictivity.unique-vacuum") && r2_block.contains("holds → fails"),
            "add-r-squared must flip unique-vacuum holds to fails: {r2_block}"
        );
        assert!(
            !r2_block.contains("empirical.observed-4d"),
            "add-r-squared is not the dim knob: {r2_block}"
        );
        assert!(
            !r2_block.contains("gr.eddington-deflection"),
            "add-r-squared is not the Newton Schwarzschild solar fork: {r2_block}"
        );
        assert!(
            !r2_block.contains("gr.newton-half-deflection"),
            "add-r-squared is not the inverse-square Binet fork: {r2_block}"
        );
        assert!(
            text.contains("add-brans-dicke"),
            "add-brans-dicke must still be an IR fork: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("general-relativity").unwrap();
        assert!(
            live.evaluate_all().iter().any(|(c, v)| {
                c.id_str() == "predictivity.unique-vacuum" && v.kind == VerdictKind::Holds
            }),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("dim").unwrap().display(),
            "4",
            "hypothesize must restore knobs"
        );
        assert_eq!(
            live.get("cosmological_constant").unwrap().display(),
            "0",
            "hypothesize must restore knobs"
        );
        let newton = lab.theory("newtonian-gravity").unwrap();
        assert_eq!(
            newton.id(),
            "newtonian-gravity",
            "GR IR must not convert Newton"
        );
        let sm = lab.theory("standard-model").unwrap();
        assert_eq!(
            sm.get("include_higgs").unwrap().display(),
            "true",
            "GR IR must not convert the Standard-Model include_higgs knob"
        );
        let dim = lab.exec(Command::Set {
            theory: "general-relativity".into(),
            knob: "dim".into(),
            value: "5".into(),
        });
        assert_eq!(dim.exit_code(), 0, "{}", dim.text());
        assert!(
            dim.text().contains("empirical.observed-4d") && dim.text().contains("holds → fails"),
            "{}",
            dim.text()
        );
        let live = lab.theory("general-relativity").unwrap();
        assert!(
            live.evaluate_all().iter().any(|(c, v)| {
                c.id_str() == "predictivity.unique-vacuum" && v.kind == VerdictKind::Holds
            }),
            "dim still Holds uniqueness on the live Einstein-Hilbert encoding"
        );
        let _ = lab.exec(Command::Set {
            theory: "general-relativity".into(),
            knob: "dim".into(),
            value: "4".into(),
        });
        let why = lab
            .exec(Command::Why {
                claim: "predictivity.unique-vacuum".into(),
            })
            .text()
            .to_string();
        assert!(
            why.contains("classical Einstein-Hilbert plus Λ"),
            "GR unique-vacuum must name Einstein-Hilbert: {why}"
        );
        assert!(
            !why.contains("not yet a machine-checked regime"),
            "unique-vacuum encodings name regimes: {why}"
        );
    }

    #[test]
    fn hypothesize_general_relativity_brans_dicke_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        for knob in ["brans_dicke", "omega", "add-brans-dicke"] {
            let blocked = lab.exec(Command::Set {
                theory: "general-relativity".into(),
                knob: knob.into(),
                value: "true".into(),
            });
            assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
            assert!(
                blocked.text().contains("unknown knob") || blocked.text().contains(knob),
                "{}",
                blocked.text()
            );
        }

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("general-relativity".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-brans-dicke") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-brans-dicke: package → add-brans-dicke";
        let start = text.find(marker).expect("add-brans-dicke hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  general-relativity  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let bd_block = &rest[..end];
        assert!(
            bd_block.contains("predictivity.unique-vacuum") && bd_block.contains("holds → fails"),
            "add-brans-dicke must flip unique-vacuum holds to fails: {bd_block}"
        );
        assert!(
            bd_block.contains("gr.eddington-deflection") && bd_block.contains("holds → fails"),
            "add-brans-dicke must flip Eddington holds to fails: {bd_block}"
        );
        assert!(
            bd_block.contains("gr.mercury-perihelion") && bd_block.contains("holds → fails"),
            "add-brans-dicke must flip Mercury holds to fails: {bd_block}"
        );
        assert!(
            !bd_block.contains("empirical.observed-4d"),
            "add-brans-dicke is not the dim knob: {bd_block}"
        );
        assert!(
            !bd_block.contains("gr.newton-half-deflection"),
            "add-brans-dicke is not the Newton half-angle fork: {bd_block}"
        );
        assert!(
            bd_block.matches("holds → fails").count() == 3,
            "add-brans-dicke should flip uniqueness, Eddington, and Mercury: {bd_block}"
        );
        assert!(
            text.contains("add-r-squared"),
            "add-r-squared must still be an IR fork: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert!(!text.contains("receipt"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("general-relativity").unwrap();
        assert!(
            live.evaluate_all().iter().any(|(c, v)| {
                c.id_str() == "predictivity.unique-vacuum" && v.kind == VerdictKind::Holds
            }),
            "IR mutant must not be installed"
        );
        assert!(
            live.evaluate_all().iter().any(|(c, v)| {
                c.id_str() == "gr.eddington-deflection" && v.kind == VerdictKind::Holds
            }),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("dim").unwrap().display(),
            "4",
            "hypothesize must restore knobs"
        );
        let newton = lab.theory("newtonian-gravity").unwrap();
        assert_eq!(
            newton.id(),
            "newtonian-gravity",
            "GR IR must not convert Newton"
        );
        let dim = lab.exec(Command::Set {
            theory: "general-relativity".into(),
            knob: "dim".into(),
            value: "5".into(),
        });
        assert_eq!(dim.exit_code(), 0, "{}", dim.text());
        assert!(
            dim.text().contains("empirical.observed-4d") && dim.text().contains("holds → fails"),
            "dim still flips observed-4d: {}",
            dim.text()
        );
        let live = lab.theory("general-relativity").unwrap();
        assert!(
            live.evaluate_all().iter().any(|(c, v)| {
                c.id_str() == "predictivity.unique-vacuum" && v.kind == VerdictKind::Holds
            }),
            "dim still Holds uniqueness on the live Einstein-Hilbert encoding"
        );
        let _ = lab.exec(Command::Set {
            theory: "general-relativity".into(),
            knob: "dim".into(),
            value: "4".into(),
        });
        let why = lab
            .exec(Command::Why {
                claim: "predictivity.unique-vacuum".into(),
            })
            .text()
            .to_string();
        assert!(
            why.contains("classical Einstein-Hilbert plus Λ"),
            "GR unique-vacuum must name Einstein-Hilbert: {why}"
        );
        assert!(
            !why.contains("not yet a machine-checked regime"),
            "unique-vacuum encodings name regimes: {why}"
        );
    }

    #[test]
    fn hypothesize_special_relativity_binomial_gamma_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        for knob in ["binomial_gamma", "gamma", "truncated"] {
            let blocked = lab.exec(Command::Set {
                theory: "special-relativity".into(),
                knob: knob.into(),
                value: "true".into(),
            });
            assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
            assert!(
                blocked.text().contains("unknown knob") || blocked.text().contains(knob),
                "{}",
                blocked.text()
            );
        }

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("special-relativity".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-binomial-gamma") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-binomial-gamma: package → add-binomial-gamma";
        let start = text.find(marker).expect("add-binomial-gamma hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  special-relativity  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let bin_block = &rest[..end];
        assert!(
            bin_block.contains("sr.invariant-interval") && bin_block.contains("holds → fails"),
            "add-binomial-gamma must flip interval holds to fails: {bin_block}"
        );
        assert!(
            bin_block.contains("sr.energy-momentum-invariant")
                && bin_block.contains("holds → fails"),
            "add-binomial-gamma must flip mass-shell holds to fails: {bin_block}"
        );
        assert!(
            !bin_block.contains("sr.subluminal-composition"),
            "add-binomial-gamma is not the Galilean composition fork: {bin_block}"
        );
        assert!(
            text.contains("add-minus-uv"),
            "add-minus-uv must still be an IR fork: {text}"
        );
        assert!(
            text.contains("absolute_time"),
            "absolute_time must still be a knob probe: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("special-relativity").unwrap();
        assert!(
            live.evaluate_all().iter().any(|(c, v)| {
                c.id_str() == "sr.invariant-interval" && v.kind == VerdictKind::Holds
            }),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("absolute_time").unwrap().display(),
            "false",
            "hypothesize must restore knobs"
        );
        let gal = lab.exec(Command::Set {
            theory: "special-relativity".into(),
            knob: "absolute_time".into(),
            value: "true".into(),
        });
        assert_eq!(gal.exit_code(), 0, "{}", gal.text());
        assert!(
            gal.text().contains("sr.subluminal-composition")
                && gal.text().contains("holds → fails"),
            "absolute_time still flips composition: {}",
            gal.text()
        );
        let _ = lab.exec(Command::Set {
            theory: "special-relativity".into(),
            knob: "absolute_time".into(),
            value: "false".into(),
        });
        let why = lab
            .exec(Command::Why {
                claim: "sr.invariant-interval".into(),
            })
            .text()
            .to_string();
        let sr = why_theory_block(&why, "special-relativity");
        assert!(
            sr.contains("1+1 Minkowski"),
            "interval must keep the catalog domain: {sr}"
        );
        assert!(
            !sr.contains("not yet a machine-checked regime"),
            "interval must not be encoding-wide: {sr}"
        );
    }

    #[test]
    fn hypothesize_special_relativity_minus_uv_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        for knob in ["minus_uv", "compose", "add-minus-uv"] {
            let blocked = lab.exec(Command::Set {
                theory: "special-relativity".into(),
                knob: knob.into(),
                value: "true".into(),
            });
            assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
            assert!(
                blocked.text().contains("unknown knob") || blocked.text().contains(knob),
                "{}",
                blocked.text()
            );
        }

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("special-relativity".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-minus-uv") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-minus-uv: package → add-minus-uv";
        let start = text.find(marker).expect("add-minus-uv hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  special-relativity  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let minus_block = &rest[..end];
        assert!(
            minus_block.contains("sr.subluminal-composition")
                && minus_block.contains("holds → fails"),
            "add-minus-uv must flip composition holds to fails: {minus_block}"
        );
        assert!(
            !minus_block.contains("sr.invariant-interval"),
            "add-minus-uv is not the binomial γ interval fork: {minus_block}"
        );
        assert!(
            !minus_block.contains("sr.energy-momentum-invariant"),
            "add-minus-uv is not the binomial γ mass-shell fork: {minus_block}"
        );
        assert!(
            minus_block.matches("holds → fails").count() == 1,
            "add-minus-uv should flip only composition: {minus_block}"
        );
        assert!(
            text.contains("add-binomial-gamma"),
            "add-binomial-gamma must still be an IR fork: {text}"
        );
        assert!(
            text.contains("absolute_time"),
            "absolute_time must still be a knob probe: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert!(!text.contains("receipt"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("special-relativity").unwrap();
        assert!(
            live.evaluate_all().iter().any(|(c, v)| {
                c.id_str() == "sr.subluminal-composition" && v.kind == VerdictKind::Holds
            }),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("absolute_time").unwrap().display(),
            "false",
            "hypothesize must restore knobs"
        );
        let gal = lab.exec(Command::Set {
            theory: "special-relativity".into(),
            knob: "absolute_time".into(),
            value: "true".into(),
        });
        assert_eq!(gal.exit_code(), 0, "{}", gal.text());
        assert!(
            gal.text().contains("sr.subluminal-composition")
                && gal.text().contains("holds → fails"),
            "absolute_time still flips composition: {}",
            gal.text()
        );
        assert!(
            gal.text().contains("sr.invariant-interval") && gal.text().contains("holds → fails"),
            "absolute_time still flips interval: {}",
            gal.text()
        );
        let _ = lab.exec(Command::Set {
            theory: "special-relativity".into(),
            knob: "absolute_time".into(),
            value: "false".into(),
        });
        let why = lab
            .exec(Command::Why {
                claim: "sr.subluminal-composition".into(),
            })
            .text()
            .to_string();
        let sr = why_theory_block(&why, "special-relativity");
        assert!(
            sr.contains("collinear Einstein") || sr.contains("|u| < 1"),
            "composition must keep the catalog domain: {sr}"
        );
        assert!(
            !sr.contains("not yet a machine-checked regime"),
            "composition must not be encoding-wide: {sr}"
        );
        assert!(
            !sr.contains("theory "),
            "why none-lines must not split why_theory_block: {sr}"
        );
    }

    #[test]
    fn hypothesize_planck_wien_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        for knob in ["wien", "occupation", "binomial_gamma"] {
            let blocked = lab.exec(Command::Set {
                theory: "planck".into(),
                knob: knob.into(),
                value: "true".into(),
            });
            assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
            assert!(
                blocked.text().contains("unknown knob") || blocked.text().contains(knob),
                "{}",
                blocked.text()
            );
        }

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("planck".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-wien") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-wien: package → add-wien";
        let start = text.find(marker).expect("add-wien hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  planck  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let wien_block = &rest[..end];
        assert!(
            wien_block.contains("thermo.rj-ir-limit") && wien_block.contains("holds → fails"),
            "add-wien must flip IR correspondence holds to fails: {wien_block}"
        );
        assert!(
            !wien_block.contains("thermo.uv-finite"),
            "add-wien is not the quantum catastrophe fork: {wien_block}"
        );
        assert!(
            wien_block.matches("holds → fails").count() == 1,
            "add-wien should flip only the IR correspondence: {wien_block}"
        );
        assert!(
            text.contains("add-zero-point"),
            "add-zero-point must still be an IR fork: {text}"
        );
        assert!(
            text.contains("quantum"),
            "quantum must still be a knob probe: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("planck").unwrap();
        assert!(
            live.evaluate_all().iter().any(|(c, v)| {
                c.id_str() == "thermo.rj-ir-limit" && v.kind == VerdictKind::Holds
            }),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("quantum").unwrap().display(),
            "true",
            "hypothesize must restore knobs"
        );
        let cat = lab.exec(Command::Set {
            theory: "planck".into(),
            knob: "quantum".into(),
            value: "false".into(),
        });
        assert_eq!(cat.exit_code(), 0, "{}", cat.text());
        assert!(
            cat.text().contains("thermo.uv-finite") && cat.text().contains("holds → fails"),
            "quantum still restores the catastrophe: {}",
            cat.text()
        );
        let _ = lab.exec(Command::Set {
            theory: "planck".into(),
            knob: "quantum".into(),
            value: "true".into(),
        });
        let why = lab
            .exec(Command::Why {
                claim: "thermo.rj-ir-limit".into(),
            })
            .text()
            .to_string();
        let pb = why_theory_block(&why, "planck");
        assert!(
            pb.contains("0.01 kT") || pb.contains("hν"),
            "IR correspondence must keep the catalog domain: {pb}"
        );
        assert!(
            !pb.contains("not yet a machine-checked regime"),
            "IR correspondence must not be encoding-wide: {pb}"
        );
    }

    #[test]
    fn hypothesize_planck_zero_point_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        for knob in ["zero_point", "vacuum", "add-zero-point"] {
            let blocked = lab.exec(Command::Set {
                theory: "planck".into(),
                knob: knob.into(),
                value: "true".into(),
            });
            assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
            assert!(
                blocked.text().contains("unknown knob") || blocked.text().contains(knob),
                "{}",
                blocked.text()
            );
        }

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("planck".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-zero-point") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-zero-point: package → add-zero-point";
        let start = text.find(marker).expect("add-zero-point hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  planck  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let zp_block = &rest[..end];
        assert!(
            zp_block.contains("thermo.uv-finite") && zp_block.contains("holds → fails"),
            "add-zero-point must flip UV-finite holds to fails: {zp_block}"
        );
        assert!(
            zp_block.contains("thermo.stefan-boltzmann") && zp_block.contains("holds → fails"),
            "add-zero-point must flip Stefan-Boltzmann holds to fails: {zp_block}"
        );
        assert!(
            zp_block.contains("thermo.wien-displacement") && zp_block.contains("holds → fails"),
            "add-zero-point must flip Wien displacement holds to fails: {zp_block}"
        );
        assert!(
            !zp_block.contains("thermo.rj-ir-limit"),
            "add-zero-point is not the Wien infrared fork: {zp_block}"
        );
        assert!(
            !zp_block.contains("thermo.mode-equipartition"),
            "add-zero-point is not the quantum equipartition fork: {zp_block}"
        );
        assert!(
            zp_block.matches("holds → fails").count() == 3,
            "add-zero-point should flip UV, Stefan, and Wien: {zp_block}"
        );
        assert!(
            text.contains("add-wien"),
            "add-wien must still be an IR fork: {text}"
        );
        assert!(
            text.contains("quantum"),
            "quantum must still be a knob probe: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert!(!text.contains("receipt"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("planck").unwrap();
        assert!(
            live.evaluate_all()
                .iter()
                .any(|(c, v)| { c.id_str() == "thermo.uv-finite" && v.kind == VerdictKind::Holds }),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("quantum").unwrap().display(),
            "true",
            "hypothesize must restore knobs"
        );
        let cat = lab.exec(Command::Set {
            theory: "planck".into(),
            knob: "quantum".into(),
            value: "false".into(),
        });
        assert_eq!(cat.exit_code(), 0, "{}", cat.text());
        assert!(
            cat.text().contains("thermo.uv-finite") && cat.text().contains("holds → fails"),
            "quantum still restores the catastrophe: {}",
            cat.text()
        );
        assert!(
            cat.text().contains("thermo.mode-equipartition")
                && cat.text().contains("fails → holds"),
            "quantum still restores equipartition: {}",
            cat.text()
        );
        let _ = lab.exec(Command::Set {
            theory: "planck".into(),
            knob: "quantum".into(),
            value: "true".into(),
        });
        let why = lab
            .exec(Command::Why {
                claim: "thermo.uv-finite".into(),
            })
            .text()
            .to_string();
        let pb = why_theory_block(&why, "planck");
        assert!(
            pb.contains("holds") || pb.contains("finite"),
            "live Planck UV-finite must still hold: {pb}"
        );
        assert!(
            !pb.contains("theory "),
            "why none-lines must not split why_theory_block: {pb}"
        );
    }

    #[test]
    fn hypothesize_de_rham_sign_flip_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        for knob in ["sign_flip", "wien", "occupation"] {
            let blocked = lab.exec(Command::Set {
                theory: "de-rham".into(),
                knob: knob.into(),
                value: "true".into(),
            });
            assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
            assert!(
                blocked.text().contains("unknown knob") || blocked.text().contains(knob),
                "{}",
                blocked.text()
            );
        }

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("de-rham".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-sign-flip") && text.contains("ir structural"),
            "{text}"
        );
        assert!(
            text.contains("add-down-laplacian"),
            "add-down-laplacian must still be an IR fork: {text}"
        );
        let marker = "add-sign-flip: package → add-sign-flip";
        let start = text.find(marker).expect("add-sign-flip hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  de-rham  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let flip_block = &rest[..end];
        assert!(
            flip_block.contains("dec.d-squared-zero") && flip_block.contains("holds → fails"),
            "add-sign-flip must flip d² holds to fails: {flip_block}"
        );
        assert!(
            !flip_block.contains("dec.closed-equals-exact"),
            "add-sign-flip is not the shape knob: {flip_block}"
        );
        assert!(
            flip_block.matches("holds → fails").count() == 1,
            "add-sign-flip should flip only the coboundary identity: {flip_block}"
        );
        assert!(
            text.contains("shape"),
            "shape must still be a knob probe: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("de-rham").unwrap();
        assert!(
            live.evaluate_all().iter().any(|(c, v)| {
                c.id_str() == "dec.d-squared-zero" && v.kind == VerdictKind::Holds
            }),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("shape").unwrap().display(),
            "disk",
            "hypothesize must restore knobs"
        );
        let circle = lab.exec(Command::Set {
            theory: "de-rham".into(),
            knob: "shape".into(),
            value: "circle".into(),
        });
        assert_eq!(circle.exit_code(), 0, "{}", circle.text());
        assert!(
            circle.text().contains("dec.closed-equals-exact")
                && circle.text().contains("holds → fails"),
            "shape still flips Poincaré: {}",
            circle.text()
        );
        let _ = lab.exec(Command::Set {
            theory: "de-rham".into(),
            knob: "shape".into(),
            value: "disk".into(),
        });
        let why = lab
            .exec(Command::Why {
                claim: "dec.d-squared-zero".into(),
            })
            .text()
            .to_string();
        let d2b = why_theory_block(&why, "de-rham");
        assert!(
            d2b.contains("oriented 2-simplex coboundary over Z"),
            "catalog d² must keep the coboundary domain: {d2b}"
        );
        assert!(
            !d2b.contains("not yet a machine-checked regime"),
            "catalog d² must not be encoding-wide: {d2b}"
        );
        assert!(
            d2b.contains("encoding:    none"),
            "hypothesize must not encode: {d2b}"
        );
    }

    #[test]
    fn hypothesize_de_rham_down_laplacian_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        for knob in ["down_laplacian", "laplacian", "add-down-laplacian"] {
            let blocked = lab.exec(Command::Set {
                theory: "de-rham".into(),
                knob: knob.into(),
                value: "true".into(),
            });
            assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
            assert!(
                blocked.text().contains("unknown knob") || blocked.text().contains(knob),
                "{}",
                blocked.text()
            );
        }

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("de-rham".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-down-laplacian") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-down-laplacian: package → add-down-laplacian";
        let start = text.find(marker).expect("add-down-laplacian hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  de-rham  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let down_block = &rest[..end];
        assert!(
            down_block.contains("dec.hodge-harmonic") && down_block.contains("holds → fails"),
            "add-down-laplacian must flip Hodge holds to fails: {down_block}"
        );
        assert!(
            !down_block.contains("dec.d-squared-zero"),
            "add-down-laplacian is not the coboundary sign flip: {down_block}"
        );
        assert!(
            !down_block.contains("dec.closed-equals-exact"),
            "add-down-laplacian is not the shape knob: {down_block}"
        );
        assert!(
            down_block.matches("holds → fails").count() == 1,
            "add-down-laplacian should flip only Hodge: {down_block}"
        );
        assert!(
            text.contains("add-sign-flip"),
            "add-sign-flip must still be an IR fork: {text}"
        );
        assert!(
            text.contains("shape"),
            "shape must still be a knob probe: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert!(!text.contains("receipt"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("de-rham").unwrap();
        assert!(
            live.evaluate_all().iter().any(|(c, v)| {
                c.id_str() == "dec.hodge-harmonic" && v.kind == VerdictKind::Holds
            }),
            "IR mutant must not be installed"
        );
        assert!(
            live.evaluate_all().iter().any(|(c, v)| {
                c.id_str() == "dec.d-squared-zero" && v.kind == VerdictKind::Holds
            }),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("shape").unwrap().display(),
            "disk",
            "hypothesize must restore knobs"
        );
        let circle = lab.exec(Command::Set {
            theory: "de-rham".into(),
            knob: "shape".into(),
            value: "circle".into(),
        });
        assert_eq!(circle.exit_code(), 0, "{}", circle.text());
        assert!(
            circle.text().contains("dec.closed-equals-exact")
                && circle.text().contains("holds → fails"),
            "shape still flips Poincaré: {}",
            circle.text()
        );
        let live = lab.theory("de-rham").unwrap();
        assert!(
            live.evaluate_all().iter().any(|(c, v)| {
                c.id_str() == "dec.hodge-harmonic" && v.kind == VerdictKind::Holds
            }),
            "shape still Holds Hodge on the live full Laplacian"
        );
        let _ = lab.exec(Command::Set {
            theory: "de-rham".into(),
            knob: "shape".into(),
            value: "disk".into(),
        });
        let why = lab
            .exec(Command::Why {
                claim: "dec.d-squared-zero".into(),
            })
            .text()
            .to_string();
        let d2b = why_theory_block(&why, "de-rham");
        assert!(
            d2b.contains("oriented 2-simplex coboundary over Z"),
            "catalog d² must keep the coboundary domain: {d2b}"
        );
        assert!(
            !d2b.contains("not yet a machine-checked regime"),
            "catalog d² must not be encoding-wide: {d2b}"
        );
        assert!(
            d2b.contains("encoding:    none"),
            "hypothesize must not encode: {d2b}"
        );
    }

    #[test]
    fn hypothesize_turing_machine_oracle_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        for knob in ["oracle", "halt_oracle", "add-oracle"] {
            let blocked = lab.exec(Command::Set {
                theory: "turing-machine".into(),
                knob: knob.into(),
                value: "true".into(),
            });
            assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
            assert!(
                blocked.text().contains("unknown knob") || blocked.text().contains(knob),
                "{}",
                blocked.text()
            );
        }

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("turing-machine".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-oracle") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-oracle: package → add-oracle";
        let start = text.find(marker).expect("add-oracle hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  turing-machine  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let oracle_block = &rest[..end];
        assert!(
            oracle_block.contains("comp.halts") && oracle_block.contains("undecidable → holds"),
            "add-oracle must flip halts undecidable to holds: {oracle_block}"
        );
        assert!(
            !oracle_block.contains("comp.turing-complete"),
            "add-oracle is not the tape_bound completeness fork: {oracle_block}"
        );
        assert!(
            !oracle_block.contains("comp.deterministic"),
            "add-oracle is not the nondeterministic knob: {oracle_block}"
        );
        assert!(
            oracle_block.matches("undecidable → holds").count() == 1,
            "add-oracle should flip only unrelativized halt: {oracle_block}"
        );
        assert!(
            text.contains("tape_bound"),
            "tape_bound must still be a knob probe: {text}"
        );
        assert!(
            text.contains("nondeterministic"),
            "nondeterministic must still be a knob probe: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("turing-machine").unwrap();
        assert!(
            live.evaluate_all()
                .iter()
                .any(|(c, v)| { c.id_str() == "comp.halts" && v.kind == VerdictKind::Undecidable }),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("tape_bound").unwrap().display(),
            "0",
            "hypothesize must restore knobs"
        );
        assert_eq!(
            live.get("nondeterministic").unwrap().display(),
            "false",
            "hypothesize must restore knobs"
        );
        let bound = lab.exec(Command::Set {
            theory: "turing-machine".into(),
            knob: "tape_bound".into(),
            value: "1000".into(),
        });
        assert_eq!(bound.exit_code(), 0, "{}", bound.text());
        assert!(
            bound.text().contains("comp.halts") && bound.text().contains("undecidable → holds"),
            "tape_bound still flips halt: {}",
            bound.text()
        );
        assert!(
            bound.text().contains("comp.turing-complete") && bound.text().contains("holds → fails"),
            "tape_bound still drops Turing completeness: {}",
            bound.text()
        );
        let _ = lab.exec(Command::Set {
            theory: "turing-machine".into(),
            knob: "tape_bound".into(),
            value: "0".into(),
        });
        let why = lab
            .exec(Command::Why {
                claim: "comp.halts".into(),
            })
            .text()
            .to_string();
        let tm = why_theory_block(&why, "turing-machine");
        assert!(
            tm.contains("unrelativized Turing machine") || tm.contains("no halt oracle"),
            "TM halt must name the unrelativized machine: {tm}"
        );
        assert!(
            !tm.contains("not yet a machine-checked regime"),
            "TM halt must not be encoding-wide: {tm}"
        );
        assert!(
            tm.contains("encoding:    none"),
            "hypothesize must not encode: {tm}"
        );
        let nand = why_theory_block(&why, "combinational-circuit");
        assert!(
            nand.contains("not yet a machine-checked regime"),
            "combinational halt stays encoding-wide: {nand}"
        );
    }

    #[test]
    fn hypothesize_olbers_tired_light_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        for knob in ["tired", "tired_light", "add-tired-light"] {
            let blocked = lab.exec(Command::Set {
                theory: "olbers-static".into(),
                knob: knob.into(),
                value: "true".into(),
            });
            assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
            assert!(
                blocked.text().contains("unknown knob") || blocked.text().contains(knob),
                "{}",
                blocked.text()
            );
        }

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("olbers-static".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-tired-light") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-tired-light: package → add-tired-light";
        let start = text.find(marker).expect("add-tired-light hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  olbers-static  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let tired_block = &rest[..end];
        assert!(
            tired_block.contains("astro.shell-cancellation")
                && tired_block.contains("holds → fails"),
            "add-tired-light must flip cancellation holds to fails: {tired_block}"
        );
        assert!(
            tired_block.contains("astro.sky-finite") && tired_block.contains("fails → holds"),
            "add-tired-light must cap the energy integral: {tired_block}"
        );
        assert!(
            !tired_block.contains("astro.night-sky-dark"),
            "tired light is not Hubble dimming or finite_age: covering still diverges: {tired_block}"
        );
        assert!(
            text.contains("finite_age"),
            "finite_age must still be a knob probe: {text}"
        );
        assert!(
            text.contains("expanding"),
            "expanding must still be a knob probe: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("olbers-static").unwrap();
        assert!(
            live.evaluate_all().iter().any(|(c, v)| {
                c.id_str() == "astro.shell-cancellation" && v.kind == VerdictKind::Holds
            }),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("finite_age").unwrap().display(),
            "false",
            "hypothesize must restore knobs"
        );
        assert_eq!(
            live.get("expanding").unwrap().display(),
            "false",
            "hypothesize must restore knobs"
        );
        let expanding = lab.exec(Command::Set {
            theory: "olbers-static".into(),
            knob: "expanding".into(),
            value: "true".into(),
        });
        assert_eq!(expanding.exit_code(), 0, "{}", expanding.text());
        assert!(
            expanding.text().contains("astro.shell-cancellation")
                && expanding.text().contains("holds → fails"),
            "expanding still flips cancellation: {}",
            expanding.text()
        );
        assert!(
            expanding.text().contains("astro.night-sky-dark")
                && expanding.text().contains("fails → holds"),
            "expanding still darkens the sky: {}",
            expanding.text()
        );
        let _ = lab.exec(Command::Set {
            theory: "olbers-static".into(),
            knob: "expanding".into(),
            value: "false".into(),
        });
        let why = lab
            .exec(Command::Why {
                claim: "astro.shell-cancellation".into(),
            })
            .text()
            .to_string();
        let ob = why_theory_block(&why, "olbers-static");
        assert!(
            ob.contains("inverse-square Euclidean shells"),
            "shell cancellation must name inverse-square Euclidean shells: {ob}"
        );
        assert!(
            !ob.contains("not yet a machine-checked regime"),
            "shell cancellation must not be encoding-wide: {ob}"
        );
        assert!(
            ob.contains("encoding:    none"),
            "hypothesize must not encode: {ob}"
        );
    }

    #[test]
    fn hypothesize_su5_missing_10_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        for knob in ["missing_10", "missing-10", "add-missing-10"] {
            let blocked = lab.exec(Command::Set {
                theory: "su5-gut".into(),
                knob: knob.into(),
                value: "true".into(),
            });
            assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
            assert!(
                blocked.text().contains("unknown knob") || blocked.text().contains(knob),
                "{}",
                blocked.text()
            );
        }

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("su5-gut".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-missing-10") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-missing-10: package → add-missing-10";
        let start = text.find(marker).expect("add-missing-10 hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  su5-gut  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let missing_block = &rest[..end];
        assert!(
            missing_block.contains("gut.sm-embedding") && missing_block.contains("holds → fails"),
            "add-missing-10 must flip embedding holds to fails: {missing_block}"
        );
        assert!(
            !missing_block.contains("gut.weinberg-angle-mz"),
            "missing 10 is not the supersymmetric knob: {missing_block}"
        );
        assert!(
            !missing_block.contains("gut.weinberg-angle ")
                && !missing_block.contains("gut.weinberg-angle\n"),
            "GUT-scale 3/8 must still hold on a missing 10: {missing_block}"
        );
        assert!(
            text.contains("supersymmetric"),
            "supersymmetric must still be a knob probe: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("su5-gut").unwrap();
        assert!(
            live.evaluate_all()
                .iter()
                .any(|(c, v)| { c.id_str() == "gut.sm-embedding" && v.kind == VerdictKind::Holds }),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("supersymmetric").unwrap().display(),
            "false",
            "hypothesize must restore knobs"
        );
        let susy = lab.exec(Command::Set {
            theory: "su5-gut".into(),
            knob: "supersymmetric".into(),
            value: "true".into(),
        });
        assert_eq!(susy.exit_code(), 0, "{}", susy.text());
        assert!(
            susy.text().contains("gut.weinberg-angle-mz") && susy.text().contains("fails → holds"),
            "supersymmetric still flips GQW: {}",
            susy.text()
        );
        let _ = lab.exec(Command::Set {
            theory: "su5-gut".into(),
            knob: "supersymmetric".into(),
            value: "false".into(),
        });
        let why = lab
            .exec(Command::Why {
                claim: "gut.sm-embedding".into(),
            })
            .text()
            .to_string();
        let gut = why_theory_block(&why, "su5-gut");
        assert!(
            gut.contains("complete 5bar + 10"),
            "embedding must name complete 5bar + 10: {gut}"
        );
        assert!(
            !gut.contains("not yet a machine-checked regime"),
            "embedding must not be encoding-wide: {gut}"
        );
        assert!(
            gut.contains("encoding:    none"),
            "hypothesize must not encode: {gut}"
        );
    }

    #[test]
    fn hypothesize_debye_two_d_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        for knob in ["two_d", "2d", "add-2d"] {
            let blocked = lab.exec(Command::Set {
                theory: "debye-solid".into(),
                knob: knob.into(),
                value: "true".into(),
            });
            assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
            assert!(
                blocked.text().contains("unknown knob") || blocked.text().contains(knob),
                "{}",
                blocked.text()
            );
        }

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("debye-solid".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-2d") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-2d: package → add-2d";
        let start = text.find(marker).expect("add-2d hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  debye-solid  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let two_d_block = &rest[..end];
        assert!(
            two_d_block.contains("thermo.debye-t3") && two_d_block.contains("holds → fails"),
            "add-2d must flip T³ holds to fails: {two_d_block}"
        );
        assert!(
            !two_d_block.contains("thermo.third-law"),
            "2d freeze-out must still hold: {two_d_block}"
        );
        assert!(
            !two_d_block.contains("thermo.dulong-petit"),
            "2d is not the quantum knob: {two_d_block}"
        );
        assert!(
            text.contains("spectrum"),
            "spectrum must still be a knob probe: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("debye-solid").unwrap();
        assert!(
            live.evaluate_all()
                .iter()
                .any(|(c, v)| { c.id_str() == "thermo.debye-t3" && v.kind == VerdictKind::Holds }),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("spectrum").unwrap().display(),
            "debye",
            "hypothesize must restore knobs"
        );
        let einstein = lab.exec(Command::Set {
            theory: "debye-solid".into(),
            knob: "spectrum".into(),
            value: "einstein".into(),
        });
        assert_eq!(einstein.exit_code(), 0, "{}", einstein.text());
        assert!(
            einstein.text().contains("thermo.debye-t3")
                && einstein.text().contains("holds → fails"),
            "spectrum still flips T³: {}",
            einstein.text()
        );
        let _ = lab.exec(Command::Set {
            theory: "debye-solid".into(),
            knob: "spectrum".into(),
            value: "debye".into(),
        });
        let why = lab
            .exec(Command::Why {
                claim: "thermo.debye-t3".into(),
            })
            .text()
            .to_string();
        let debye = why_theory_block(&why, "debye-solid");
        assert!(
            debye.contains("3D ω²") || debye.contains("Θ/20"),
            "T³ must name 3D ω²: {debye}"
        );
        assert!(
            !debye.contains("not yet a machine-checked regime"),
            "T³ must not be encoding-wide: {debye}"
        );
        assert!(
            debye.contains("encoding:    none"),
            "hypothesize must not encode: {debye}"
        );
    }

    #[test]
    fn hypothesize_sm_missing_e_r_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        for knob in ["missing_e_r", "missing-eR", "add-missing-eR"] {
            let blocked = lab.exec(Command::Set {
                theory: "standard-model".into(),
                knob: knob.into(),
                value: "true".into(),
            });
            assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
            assert!(
                blocked.text().contains("unknown knob") || blocked.text().contains(knob),
                "{}",
                blocked.text()
            );
        }

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("standard-model".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-missing-eR") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-missing-eR: package → add-missing-eR";
        let start = text.find(marker).expect("add-missing-eR hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  standard-model  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let missing_block = &rest[..end];
        assert!(
            missing_block.contains("consistency.anomaly-cancellation")
                && missing_block.contains("holds → fails"),
            "add-missing-eR must flip anomaly holds to fails: {missing_block}"
        );
        assert!(
            !missing_block.contains("sm.hypercharge-derivation"),
            "missing e_R is not the hypercharge quadratic: {missing_block}"
        );
        assert!(
            !missing_block.contains("empirical.charge-quantization")
                && !missing_block.contains("consistency.charge-quantization"),
            "hydrogen Q=T3+Y must still hold: {missing_block}"
        );
        assert!(
            text.contains("include_higgs") || text.contains("neutrino_masses"),
            "chosen knobs must still be probed: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("standard-model").unwrap();
        assert!(
            live.evaluate_all().iter().any(|(c, v)| {
                c.id_str() == "consistency.anomaly-cancellation" && v.kind == VerdictKind::Holds
            }),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("generations").unwrap().display(),
            "3",
            "hypothesize must restore knobs"
        );
        let gens = lab.exec(Command::Set {
            theory: "standard-model".into(),
            knob: "generations".into(),
            value: "2".into(),
        });
        assert_eq!(gens.exit_code(), 0, "{}", gens.text());
        assert!(
            gens.text().contains("empirical.three-generations")
                && gens.text().contains("holds → fails"),
            "generations still flips three-generations: {}",
            gens.text()
        );
        let _ = lab.exec(Command::Set {
            theory: "standard-model".into(),
            knob: "generations".into(),
            value: "3".into(),
        });
        let why = lab
            .exec(Command::Why {
                claim: "consistency.anomaly-cancellation".into(),
            })
            .text()
            .to_string();
        let sm = why_theory_block(&why, "standard-model");
        assert!(
            sm.contains("complete Q_L") || sm.contains("eRc"),
            "anomaly must name complete Weyl content: {sm}"
        );
        assert!(
            !sm.contains("not yet a machine-checked regime"),
            "anomaly must not be encoding-wide: {sm}"
        );
        assert!(
            sm.contains("encoding:    none"),
            "hypothesize must not encode: {sm}"
        );
    }

    #[test]
    fn hypothesize_og_missing_spin10_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        for knob in ["missing_spin10", "missing-spin10", "add-missing-spin10"] {
            let blocked = lab.exec(Command::Set {
                theory: "observer-geometry".into(),
                knob: knob.into(),
                value: "true".into(),
            });
            assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
            assert!(
                blocked.text().contains("unknown knob") || blocked.text().contains(knob),
                "{}",
                blocked.text()
            );
        }

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("observer-geometry".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-missing-spin10") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-missing-spin10: package → add-missing-spin10";
        let start = text.find(marker).expect("add-missing-spin10 hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  observer-geometry  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let missing_block = &rest[..end];
        assert!(
            missing_block.contains("empirical.sm-gauge") && missing_block.contains("holds → fails"),
            "add-missing-spin10 must flip sm-gauge holds to fails: {missing_block}"
        );
        assert!(
            !missing_block.contains("predictivity.unique-vacuum"),
            "missing Spin(10) is not the unique_vacuum knob: {missing_block}"
        );
        assert!(
            text.contains("unique_vacuum") || text.contains("derive_gauge"),
            "chosen knobs must still be probed: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("observer-geometry").unwrap();
        assert!(
            live.evaluate_all().iter().any(|(c, v)| {
                c.id_str() == "empirical.sm-gauge" && v.kind == VerdictKind::Holds
            }),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("unique_vacuum").unwrap().display(),
            "true",
            "hypothesize must restore knobs"
        );
        let uniq = lab.exec(Command::Set {
            theory: "observer-geometry".into(),
            knob: "unique_vacuum".into(),
            value: "false".into(),
        });
        assert_eq!(uniq.exit_code(), 0, "{}", uniq.text());
        assert!(
            uniq.text().contains("predictivity.unique-vacuum")
                && uniq.text().contains("holds → fails"),
            "unique_vacuum still flips unique-vacuum: {}",
            uniq.text()
        );
        let _ = lab.exec(Command::Set {
            theory: "observer-geometry".into(),
            knob: "unique_vacuum".into(),
            value: "true".into(),
        });
        let fibre = lab.exec(Command::Set {
            theory: "observer-geometry".into(),
            knob: "fibre_dim".into(),
            value: "9".into(),
        });
        assert_eq!(fibre.exit_code(), 0, "{}", fibre.text());
        assert!(
            fibre.text().contains("empirical.sm-gauge") && fibre.text().contains("holds → fails"),
            "fibre_dim still starves Spin(10): {}",
            fibre.text()
        );
        let _ = lab.exec(Command::Set {
            theory: "observer-geometry".into(),
            knob: "fibre_dim".into(),
            value: "10".into(),
        });
        let why = lab
            .exec(Command::Why {
                claim: "empirical.sm-gauge".into(),
            })
            .text()
            .to_string();
        let og = why_theory_block(&why, "observer-geometry");
        assert!(
            og.contains("Spin(10)") || og.contains("10-fibre"),
            "sm-gauge must name Spin(10) on 10-fibre: {og}"
        );
        assert!(
            !og.contains("not yet a machine-checked regime"),
            "sm-gauge must not be encoding-wide: {og}"
        );
        assert!(
            og.contains("encoding:    none"),
            "hypothesize must not encode: {og}"
        );
    }

    #[test]
    fn hypothesize_dulong_quartic_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        for knob in ["anharmonic", "quartic", "add-quartic"] {
            let blocked = lab.exec(Command::Set {
                theory: "dulong-petit".into(),
                knob: knob.into(),
                value: "true".into(),
            });
            assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
            assert!(
                blocked.text().contains("unknown knob") || blocked.text().contains(knob),
                "{}",
                blocked.text()
            );
        }

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("dulong-petit".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-quartic") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-quartic: package → add-quartic";
        let start = text.find(marker).expect("add-quartic hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  dulong-petit  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let quartic_block = &rest[..end];
        assert!(
            quartic_block.contains("thermo.dulong-petit")
                && quartic_block.contains("holds → fails"),
            "add-quartic must flip dulong-petit holds to fails: {quartic_block}"
        );
        assert!(
            !quartic_block.contains("thermo.third-law"),
            "quartic freeze-out must still fail the third law without flipping it: {quartic_block}"
        );
        assert!(
            text.contains("quantum"),
            "quantum must still be a knob probe: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("dulong-petit").unwrap();
        assert!(
            live.evaluate_all().iter().any(|(c, v)| {
                c.id_str() == "thermo.dulong-petit" && v.kind == VerdictKind::Holds
            }),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("quantum").unwrap().display(),
            "false",
            "hypothesize must restore knobs"
        );
        let quantum = lab.exec(Command::Set {
            theory: "dulong-petit".into(),
            knob: "quantum".into(),
            value: "true".into(),
        });
        assert_eq!(quantum.exit_code(), 0, "{}", quantum.text());
        assert!(
            quantum.text().contains("thermo.dulong-petit")
                && quantum.text().contains("holds → fails"),
            "quantum still flips Dulong–Petit: {}",
            quantum.text()
        );
        let _ = lab.exec(Command::Set {
            theory: "dulong-petit".into(),
            knob: "quantum".into(),
            value: "false".into(),
        });
        let why = lab
            .exec(Command::Why {
                claim: "thermo.dulong-petit".into(),
            })
            .text()
            .to_string();
        let dp = why_theory_block(&why, "dulong-petit");
        assert!(
            dp.contains("U = 3 N k T") || dp.contains("harmonic"),
            "dulong-petit must name harmonic U = 3 N k T: {dp}"
        );
        assert!(
            !dp.contains("not yet a machine-checked regime"),
            "dulong-petit must not be encoding-wide: {dp}"
        );
        assert!(
            dp.contains("encoding:    none"),
            "hypothesize must not encode: {dp}"
        );
    }

    #[test]
    fn hypothesize_heterotic_missing_e8_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        for knob in ["missing_e8", "missing-e8", "add-missing-e8"] {
            let blocked = lab.exec(Command::Set {
                theory: "heterotic-e8e8".into(),
                knob: knob.into(),
                value: "true".into(),
            });
            assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
            assert!(
                blocked.text().contains("unknown knob") || blocked.text().contains(knob),
                "{}",
                blocked.text()
            );
        }

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("heterotic-e8e8".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-missing-e8") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-missing-e8: package → add-missing-e8";
        let start = text.find(marker).expect("add-missing-e8 hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  heterotic-e8e8  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let missing_block = &rest[..end];
        assert!(
            missing_block.contains("consistency.anomaly-cancellation")
                && missing_block.contains("holds → fails"),
            "add-missing-e8 must flip anomaly-cancellation holds to fails: {missing_block}"
        );
        assert!(
            !missing_block.contains("empirical.sm-gauge"),
            "missing E8 still embeds SM: {missing_block}"
        );
        assert!(
            !missing_block.contains("predictivity.unique-vacuum"),
            "missing E8 is not the landscape: {missing_block}"
        );
        assert!(
            !missing_block.contains("consistency.critical-dimension"),
            "missing E8 is not the total_dim knob: {missing_block}"
        );
        assert!(
            text.contains("kind") || text.contains("total_dim"),
            "chosen knobs must still be probed: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert!(!text.contains("receipt"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("heterotic-e8e8").unwrap();
        assert!(
            live.evaluate_all().iter().any(|(c, v)| {
                c.id_str() == "consistency.anomaly-cancellation" && v.kind == VerdictKind::Holds
            }),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("kind").unwrap().display(),
            "heterotic-e8e8",
            "hypothesize must restore knobs"
        );
        let bosonic = lab.exec(Command::Set {
            theory: "heterotic-e8e8".into(),
            knob: "kind".into(),
            value: "bosonic".into(),
        });
        assert_eq!(bosonic.exit_code(), 0, "{}", bosonic.text());
        assert!(
            bosonic.text().contains("empirical.fermions")
                && bosonic.text().contains("holds → fails"),
            "kind still flips fermions: {}",
            bosonic.text()
        );
        let _ = lab.exec(Command::Set {
            theory: "heterotic-e8e8".into(),
            knob: "kind".into(),
            value: "heterotic-e8e8".into(),
        });
        let dim = lab.exec(Command::Set {
            theory: "heterotic-e8e8".into(),
            knob: "total_dim".into(),
            value: "9".into(),
        });
        assert_eq!(dim.exit_code(), 0, "{}", dim.text());
        assert!(
            dim.text().contains("consistency.anomaly-cancellation")
                && dim.text().contains("holds → undecidable"),
            "total_dim still opens Green-Schwarz: {}",
            dim.text()
        );
        let _ = lab.exec(Command::Set {
            theory: "heterotic-e8e8".into(),
            knob: "total_dim".into(),
            value: "10".into(),
        });
        let why = lab
            .exec(Command::Why {
                claim: "consistency.anomaly-cancellation".into(),
            })
            .text()
            .to_string();
        let het = why_theory_block(&why, "heterotic-e8e8");
        assert!(
            het.contains("E8 x E8"),
            "anomaly must name complete E8 x E8: {het}"
        );
        assert!(
            !het.contains("not yet a machine-checked regime"),
            "heterotic Green-Schwarz must not be encoding-wide: {het}"
        );
        assert!(
            het.contains("encoding:    none"),
            "hypothesize must not encode: {het}"
        );
        let iib = why_theory_block(&why, "type-iib");
        assert!(
            iib.contains("not yet a machine-checked regime"),
            "Type II Green-Schwarz stays encoding-wide: {iib}"
        );
    }

    #[test]
    fn hypothesize_heterotic_so16_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        for knob in ["so16", "so-16", "add-so16"] {
            let blocked = lab.exec(Command::Set {
                theory: "heterotic-so32".into(),
                knob: knob.into(),
                value: "true".into(),
            });
            assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
            assert!(
                blocked.text().contains("unknown knob") || blocked.text().contains(knob),
                "{}",
                blocked.text()
            );
        }

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("heterotic-so32".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-so16") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-so16: package → add-so16";
        let start = text.find(marker).expect("add-so16 hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  heterotic-so32  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let so16_block = &rest[..end];
        assert!(
            so16_block.contains("consistency.anomaly-cancellation")
                && so16_block.contains("holds → fails"),
            "add-so16 must flip anomaly-cancellation holds to fails: {so16_block}"
        );
        assert!(
            !so16_block.contains("empirical.sm-gauge"),
            "SO(16) still embeds SM: {so16_block}"
        );
        assert!(
            !so16_block.contains("predictivity.unique-vacuum"),
            "SO(16) is not the landscape: {so16_block}"
        );
        assert!(
            !so16_block.contains("consistency.critical-dimension"),
            "SO(16) is not the total_dim knob: {so16_block}"
        );
        assert!(
            text.contains("kind") || text.contains("total_dim"),
            "chosen knobs must still be probed: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert!(!text.contains("receipt"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("heterotic-so32").unwrap();
        assert!(
            live.evaluate_all().iter().any(|(c, v)| {
                c.id_str() == "consistency.anomaly-cancellation" && v.kind == VerdictKind::Holds
            }),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("kind").unwrap().display(),
            "heterotic-so32",
            "hypothesize must restore knobs"
        );
        let bosonic = lab.exec(Command::Set {
            theory: "heterotic-so32".into(),
            knob: "kind".into(),
            value: "bosonic".into(),
        });
        assert_eq!(bosonic.exit_code(), 0, "{}", bosonic.text());
        assert!(
            bosonic.text().contains("empirical.fermions")
                && bosonic.text().contains("holds → fails"),
            "kind still flips fermions: {}",
            bosonic.text()
        );
        let _ = lab.exec(Command::Set {
            theory: "heterotic-so32".into(),
            knob: "kind".into(),
            value: "heterotic-so32".into(),
        });
        let dim = lab.exec(Command::Set {
            theory: "heterotic-so32".into(),
            knob: "total_dim".into(),
            value: "9".into(),
        });
        assert_eq!(dim.exit_code(), 0, "{}", dim.text());
        assert!(
            dim.text().contains("consistency.anomaly-cancellation")
                && dim.text().contains("holds → undecidable"),
            "total_dim still opens Green-Schwarz: {}",
            dim.text()
        );
        let _ = lab.exec(Command::Set {
            theory: "heterotic-so32".into(),
            knob: "total_dim".into(),
            value: "10".into(),
        });
        let why = lab
            .exec(Command::Why {
                claim: "consistency.anomaly-cancellation".into(),
            })
            .text()
            .to_string();
        let het = why_theory_block(&why, "heterotic-so32");
        assert!(
            het.contains("SO(32)"),
            "anomaly must name complete SO(32): {het}"
        );
        assert!(
            !het.contains("not yet a machine-checked regime"),
            "heterotic-so32 Green-Schwarz must not be encoding-wide: {het}"
        );
        assert!(
            het.contains("encoding:    none"),
            "hypothesize must not encode: {het}"
        );
        let type_i = why_theory_block(&why, "type-i");
        assert!(
            type_i.contains("Chan-Paton SO(32)"),
            "Type I still names Chan-Paton SO(32): {type_i}"
        );
        assert!(
            !type_i.contains("not yet a machine-checked regime"),
            "Type I Green-Schwarz must not be encoding-wide: {type_i}"
        );
        let iib = why_theory_block(&why, "type-iib");
        assert!(
            iib.contains("not yet a machine-checked regime"),
            "Type II Green-Schwarz stays encoding-wide: {iib}"
        );
        let e8e8 = why_theory_block(&why, "heterotic-e8e8");
        assert!(
            e8e8.contains("E8 x E8"),
            "heterotic-e8e8 still names E8 x E8: {e8e8}"
        );
    }

    #[test]
    fn hypothesize_type_i_chan_paton_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        for knob in ["chan_paton_16", "chan-paton-16", "add-chan-paton-16"] {
            let blocked = lab.exec(Command::Set {
                theory: "type-i".into(),
                knob: knob.into(),
                value: "true".into(),
            });
            assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
            assert!(
                blocked.text().contains("unknown knob") || blocked.text().contains(knob),
                "{}",
                blocked.text()
            );
        }

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("type-i".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-chan-paton-16") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-chan-paton-16: package → add-chan-paton-16";
        let start = text.find(marker).expect("add-chan-paton-16 hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  type-i  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let cp_block = &rest[..end];
        assert!(
            cp_block.contains("consistency.anomaly-cancellation")
                && cp_block.contains("holds → fails"),
            "add-chan-paton-16 must flip anomaly-cancellation holds to fails: {cp_block}"
        );
        assert!(
            !cp_block.contains("empirical.sm-gauge"),
            "Chan-Paton SO(16) still embeds SM: {cp_block}"
        );
        assert!(
            !cp_block.contains("predictivity.unique-vacuum"),
            "Chan-Paton SO(16) is not the landscape: {cp_block}"
        );
        assert!(
            !cp_block.contains("consistency.critical-dimension"),
            "Chan-Paton SO(16) is not the total_dim knob: {cp_block}"
        );
        assert!(
            text.contains("kind") || text.contains("total_dim"),
            "chosen knobs must still be probed: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert!(!text.contains("receipt"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("type-i").unwrap();
        assert!(
            live.evaluate_all().iter().any(|(c, v)| {
                c.id_str() == "consistency.anomaly-cancellation" && v.kind == VerdictKind::Holds
            }),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("kind").unwrap().display(),
            "type-i",
            "hypothesize must restore knobs"
        );
        let bosonic = lab.exec(Command::Set {
            theory: "type-i".into(),
            knob: "kind".into(),
            value: "bosonic".into(),
        });
        assert_eq!(bosonic.exit_code(), 0, "{}", bosonic.text());
        assert!(
            bosonic.text().contains("empirical.fermions")
                && bosonic.text().contains("holds → fails"),
            "kind still flips fermions: {}",
            bosonic.text()
        );
        let _ = lab.exec(Command::Set {
            theory: "type-i".into(),
            knob: "kind".into(),
            value: "type-i".into(),
        });
        let dim = lab.exec(Command::Set {
            theory: "type-i".into(),
            knob: "total_dim".into(),
            value: "9".into(),
        });
        assert_eq!(dim.exit_code(), 0, "{}", dim.text());
        assert!(
            dim.text().contains("consistency.anomaly-cancellation")
                && dim.text().contains("holds → undecidable"),
            "total_dim still opens Green-Schwarz: {}",
            dim.text()
        );
        let _ = lab.exec(Command::Set {
            theory: "type-i".into(),
            knob: "total_dim".into(),
            value: "10".into(),
        });
        let why = lab
            .exec(Command::Why {
                claim: "consistency.anomaly-cancellation".into(),
            })
            .text()
            .to_string();
        let het = why_theory_block(&why, "type-i");
        assert!(
            het.contains("Chan-Paton SO(32)"),
            "anomaly must name complete Chan-Paton SO(32): {het}"
        );
        assert!(
            !het.contains("not yet a machine-checked regime"),
            "Type I Green-Schwarz must not be encoding-wide: {het}"
        );
        assert!(
            het.contains("encoding:    none"),
            "hypothesize must not encode: {het}"
        );
        let iib = why_theory_block(&why, "type-iib");
        assert!(
            iib.contains("not yet a machine-checked regime"),
            "Type II Green-Schwarz stays encoding-wide: {iib}"
        );
        let so32 = why_theory_block(&why, "heterotic-so32");
        assert!(
            so32.contains("SO(32)"),
            "heterotic-so32 still names SO(32): {so32}"
        );
        assert!(
            !so32.contains("Chan-Paton"),
            "heterotic-so32 is not Chan-Paton: {so32}"
        );
    }

    #[test]
    fn hypothesize_linear_medium_tellegen_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        let blocked = lab.exec(Command::Set {
            theory: "linear-medium".into(),
            knob: "tellegen".into(),
            value: "true".into(),
        });
        assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
        assert!(
            blocked.text().contains("unknown knob") || blocked.text().contains("tellegen"),
            "{}",
            blocked.text()
        );

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("linear-medium".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-tellegen") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-tellegen: package → add-tellegen";
        let start = text.find(marker).expect("add-tellegen hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  linear-medium  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let tellegen_block = &rest[..end];
        assert!(
            tellegen_block.contains("em.constitutive-linear")
                && tellegen_block.contains("holds → fails"),
            "add-tellegen must flip constitutive-linear holds to fails: {tellegen_block}"
        );
        assert!(
            text.contains("add-chiral"),
            "chiral must still be an IR fork: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("linear-medium").unwrap();
        assert!(
            live.evaluate_all().iter().any(|(c, v)| {
                c.id_str() == "em.constitutive-linear" && v.kind == VerdictKind::Holds
            }),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("epsilon_r").unwrap().display(),
            "2.25",
            "hypothesize must restore knobs"
        );
        assert_eq!(
            live.get("mu_r").unwrap().display(),
            "1",
            "hypothesize must restore knobs"
        );
        let why = lab
            .exec(Command::Why {
                claim: "em.constitutive-linear".into(),
            })
            .text()
            .to_string();
        let medium = why_theory_block(&why, "linear-medium");
        assert!(
            medium.contains("isotropic linear D = εE, B = μH"),
            "constitutive-linear must name D=εE: {medium}"
        );
        assert!(
            !medium.contains("not yet a machine-checked regime"),
            "constitutive-linear must not be encoding-wide: {medium}"
        );
    }

    #[test]
    fn hypothesize_linear_medium_chiral_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        for knob in ["chiral", "pasteur", "kappa"] {
            let blocked = lab.exec(Command::Set {
                theory: "linear-medium".into(),
                knob: knob.into(),
                value: "true".into(),
            });
            assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
            assert!(
                blocked.text().contains("unknown knob") || blocked.text().contains(knob),
                "{}",
                blocked.text()
            );
        }
        let freq_blocked = lab.exec(Command::Set {
            theory: "linear-medium".into(),
            knob: "frequency_hz".into(),
            value: "1e10".into(),
        });
        assert_eq!(freq_blocked.exit_code(), 1, "{}", freq_blocked.text());

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("linear-medium".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-chiral") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-chiral: package → add-chiral";
        let start = text.find(marker).expect("add-chiral hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  linear-medium  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let chiral_block = &rest[..end];
        assert!(
            chiral_block.contains("em.constitutive-linear")
                && chiral_block.contains("holds → fails"),
            "add-chiral must flip constitutive-linear holds to fails: {chiral_block}"
        );
        assert!(
            !chiral_block.contains("em.wave-speed-c"),
            "add-chiral on glass must not be the ε_r wave-speed probe: {chiral_block}"
        );
        assert!(
            text.contains("add-tellegen"),
            "add-tellegen must still be an IR fork: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("linear-medium").unwrap();
        assert!(
            live.evaluate_all().iter().any(|(c, v)| {
                c.id_str() == "em.constitutive-linear" && v.kind == VerdictKind::Holds
            }),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("epsilon_r").unwrap().display(),
            "2.25",
            "hypothesize must restore knobs"
        );
        let ohm = lab.theory("ohm-circuit").unwrap();
        assert_eq!(
            ohm.get("frequency_hz").unwrap().display(),
            "1000",
            "linear-medium IR must not convert the ohm-circuit frequency_hz knob"
        );
        let eps = lab.exec(Command::Set {
            theory: "linear-medium".into(),
            knob: "epsilon_r".into(),
            value: "1".into(),
        });
        assert_eq!(eps.exit_code(), 0, "{}", eps.text());
        assert!(
            eps.text().contains("em.wave-speed-c") && eps.text().contains("fails → holds"),
            "{}",
            eps.text()
        );
        let live = lab.theory("linear-medium").unwrap();
        assert!(
            live.evaluate_all().iter().any(|(c, v)| {
                c.id_str() == "em.constitutive-linear" && v.kind == VerdictKind::Holds
            }),
            "epsilon_r still Holds constitutive on the live isotropic encoding"
        );
        let _ = lab.exec(Command::Set {
            theory: "linear-medium".into(),
            knob: "epsilon_r".into(),
            value: "2.25".into(),
        });
        let why = lab
            .exec(Command::Why {
                claim: "em.constitutive-linear".into(),
            })
            .text()
            .to_string();
        let medium = why_theory_block(&why, "linear-medium");
        assert!(
            medium.contains("isotropic linear D = εE, B = μH"),
            "constitutive-linear must name D=εE: {medium}"
        );
        assert!(
            !medium.contains("not yet a machine-checked regime"),
            "constitutive-linear must not be encoding-wide: {medium}"
        );
        let maxwell = why_theory_block(&why, "maxwell-vacuum");
        assert!(
            maxwell.contains("not yet a machine-checked regime"),
            "Maxwell constitutive stays encoding-wide: {maxwell}"
        );
    }

    #[test]
    fn hypothesize_maxwell_vacuum_monopole_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        let blocked = lab.exec(Command::Set {
            theory: "maxwell-vacuum".into(),
            knob: "monopole".into(),
            value: "true".into(),
        });
        assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
        assert!(
            blocked.text().contains("unknown knob") || blocked.text().contains("monopole"),
            "{}",
            blocked.text()
        );
        let eps_blocked = lab.exec(Command::Set {
            theory: "maxwell-vacuum".into(),
            knob: "epsilon_r".into(),
            value: "1".into(),
        });
        assert_eq!(eps_blocked.exit_code(), 1, "{}", eps_blocked.text());

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("maxwell-vacuum".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-monopole") && text.contains("ir structural"),
            "{text}"
        );
        assert!(
            text.contains("add-proca"),
            "proca must still be an IR fork: {text}"
        );
        assert!(
            text.contains("em.faraday") && text.contains("holds → fails"),
            "{text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("maxwell-vacuum").unwrap();
        assert!(
            live.evaluate_all()
                .iter()
                .any(|(c, v)| c.id_str() == "em.faraday" && v.kind == VerdictKind::Holds),
            "IR mutant must not be installed"
        );
        assert_eq!(live.id(), "maxwell-vacuum");
        let medium = lab.theory("linear-medium").unwrap();
        assert_eq!(
            medium.get("epsilon_r").unwrap().display(),
            "2.25",
            "Maxwell IR must not convert linear-medium ε_r"
        );
        assert!(
            medium.evaluate_all().iter().any(|(c, v)| {
                c.id_str() == "em.constitutive-linear" && v.kind == VerdictKind::Holds
            }),
            "linear-medium must stay the live isotropic-linear object"
        );
        let why = lab
            .exec(Command::Why {
                claim: "em.faraday".into(),
            })
            .text()
            .to_string();
        let mx = why_theory_block(&why, "maxwell-vacuum");
        assert!(
            mx.contains("source-free homogeneous dF=0"),
            "Maxwell Faraday must name dF=0: {mx}"
        );
        assert!(
            !mx.contains("not yet a machine-checked regime"),
            "Maxwell Faraday must not be encoding-wide: {mx}"
        );
        let lm = why_theory_block(&why, "linear-medium");
        assert!(
            lm.contains("not yet a machine-checked regime"),
            "linear-medium Faraday stays encoding-wide: {lm}"
        );
        let ohm = why_theory_block(&why, "ohm-circuit");
        assert!(
            ohm.contains("lumped Kirchhoff voltage"),
            "ohm Faraday must name lumped KVL: {ohm}"
        );
        assert!(
            !ohm.contains("not yet a machine-checked regime"),
            "ohm Faraday must not be encoding-wide: {ohm}"
        );
    }

    #[test]
    fn hypothesize_maxwell_vacuum_proca_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        for knob in ["proca", "mass", "epsilon_r"] {
            let blocked = lab.exec(Command::Set {
                theory: "maxwell-vacuum".into(),
                knob: knob.into(),
                value: "true".into(),
            });
            assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
            assert!(
                blocked.text().contains("unknown knob") || blocked.text().contains(knob),
                "{}",
                blocked.text()
            );
        }

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("maxwell-vacuum".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-proca") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-proca: package → add-proca";
        let start = text.find(marker).expect("add-proca hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  maxwell-vacuum  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let proca_block = &rest[..end];
        assert!(
            proca_block.contains("em.gauss") && proca_block.contains("holds → fails"),
            "add-proca must flip em.gauss holds to fails: {proca_block}"
        );
        assert!(
            !proca_block.contains("em.faraday"),
            "add-proca is not the magnetic-current fork: {proca_block}"
        );
        assert!(
            !proca_block.contains("em.constitutive-linear"),
            "add-proca is not the Tellegen fork: {proca_block}"
        );
        assert!(
            text.contains("add-monopole"),
            "monopole must still be an IR fork: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("maxwell-vacuum").unwrap();
        assert!(
            live.evaluate_all()
                .iter()
                .any(|(c, v)| c.id_str() == "em.gauss" && v.kind == VerdictKind::Holds),
            "IR mutant must not be installed"
        );
        assert_eq!(live.id(), "maxwell-vacuum");
        let medium = lab.theory("linear-medium").unwrap();
        assert_eq!(
            medium.get("epsilon_r").unwrap().display(),
            "2.25",
            "Maxwell Proca IR must not convert linear-medium ε_r"
        );
        let why = lab
            .exec(Command::Why {
                claim: "em.gauss".into(),
            })
            .text()
            .to_string();
        let mx = why_theory_block(&why, "maxwell-vacuum");
        assert!(
            mx.contains("source-free massless Maxwell"),
            "Maxwell Gauss must name massless vacuum: {mx}"
        );
        assert!(
            !mx.contains("not yet a machine-checked regime"),
            "Maxwell Gauss must not be encoding-wide: {mx}"
        );
        let lm = why_theory_block(&why, "linear-medium");
        assert!(
            lm.contains("not yet a machine-checked regime"),
            "linear-medium Gauss stays encoding-wide: {lm}"
        );
        let ohm = why_theory_block(&why, "ohm-circuit");
        assert!(
            ohm.contains("not yet a machine-checked regime"),
            "ohm-circuit Gauss stays encoding-wide: {ohm}"
        );
    }

    #[test]
    fn hypothesize_ideal_gas_bose_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        let blocked = lab.exec(Command::Set {
            theory: "ideal-gas".into(),
            knob: "bose".into(),
            value: "true".into(),
        });
        assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
        assert!(
            blocked.text().contains("unknown knob") || blocked.text().contains("bose"),
            "{}",
            blocked.text()
        );
        let q_blocked = lab.exec(Command::Set {
            theory: "ideal-gas".into(),
            knob: "quantum".into(),
            value: "true".into(),
        });
        assert_eq!(q_blocked.exit_code(), 1, "{}", q_blocked.text());

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("ideal-gas".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-bose") && text.contains("ir structural"),
            "{text}"
        );
        assert!(
            text.contains("add-fermi"),
            "fermi must still be an IR fork: {text}"
        );
        assert!(
            text.contains("thermo.third-law") && text.contains("fails → holds"),
            "{text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("ideal-gas").unwrap();
        assert!(
            live.evaluate_all()
                .iter()
                .any(|(c, v)| c.id_str() == "thermo.third-law" && v.kind == VerdictKind::Fails),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("temperature").unwrap().display(),
            "300",
            "hypothesize must restore knobs"
        );
        assert_eq!(
            live.get("volume_ratio").unwrap().display(),
            "2",
            "hypothesize must restore knobs"
        );
        let einstein = lab.theory("einstein-solid").unwrap();
        assert_eq!(
            einstein.get("quantum").unwrap().display(),
            "true",
            "ideal-gas IR must not convert the Einstein-solid quantum knob"
        );
        assert!(
            einstein
                .evaluate_all()
                .iter()
                .any(|(c, v)| { c.id_str() == "thermo.third-law" && v.kind == VerdictKind::Holds }),
            "Einstein-solid must stay the live Bose-oscillator object"
        );
        let why = lab
            .exec(Command::Why {
                claim: "thermo.third-law".into(),
            })
            .text()
            .to_string();
        let gas = why_theory_block(&why, "ideal-gas");
        assert!(
            gas.contains("classical Maxwell-Boltzmann Sackur-Tetrode"),
            "ideal-gas third law must name Sackur-Tetrode: {gas}"
        );
        assert!(
            !gas.contains("not yet a machine-checked regime"),
            "ideal-gas third law must not be encoding-wide: {gas}"
        );
        let es = why_theory_block(&why, "einstein-solid");
        assert!(
            es.contains("not yet a machine-checked regime"),
            "Einstein-solid third law stays encoding-wide: {es}"
        );
    }

    #[test]
    fn hypothesize_ideal_gas_fermi_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        for knob in ["fermi", "fermi_temp", "lambda"] {
            let blocked = lab.exec(Command::Set {
                theory: "ideal-gas".into(),
                knob: knob.into(),
                value: "true".into(),
            });
            assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
            assert!(
                blocked.text().contains("unknown knob") || blocked.text().contains(knob),
                "{}",
                blocked.text()
            );
        }
        let q_blocked = lab.exec(Command::Set {
            theory: "ideal-gas".into(),
            knob: "quantum".into(),
            value: "true".into(),
        });
        assert_eq!(q_blocked.exit_code(), 1, "{}", q_blocked.text());

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("ideal-gas".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-fermi") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-fermi: package → add-fermi";
        let start = text.find(marker).expect("add-fermi hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  ideal-gas  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let fermi_block = &rest[..end];
        assert!(
            fermi_block.contains("thermo.equipartition") && fermi_block.contains("holds → fails"),
            "add-fermi must flip equipartition holds to fails: {fermi_block}"
        );
        assert!(
            !fermi_block.contains("thermo.second-law"),
            "add-fermi is not the volume_ratio knob: {fermi_block}"
        );
        assert!(
            text.contains("add-bose"),
            "bose must still be an IR fork: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("ideal-gas").unwrap();
        assert!(
            live.evaluate_all()
                .iter()
                .any(|(c, v)| c.id_str() == "thermo.equipartition" && v.kind == VerdictKind::Holds),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("temperature").unwrap().display(),
            "300",
            "hypothesize must restore knobs"
        );
        let einstein = lab.theory("einstein-solid").unwrap();
        assert_eq!(
            einstein.get("quantum").unwrap().display(),
            "true",
            "ideal-gas Fermi IR must not convert the Einstein-solid quantum knob"
        );
        let why = lab
            .exec(Command::Why {
                claim: "thermo.equipartition".into(),
            })
            .text()
            .to_string();
        let gas = why_theory_block(&why, "ideal-gas");
        assert!(
            gas.contains("classical C_V = 3/2 Nk"),
            "ideal-gas equipartition must name classical C_V: {gas}"
        );
        assert!(
            !gas.contains("not yet a machine-checked regime"),
            "ideal-gas equipartition must not be encoding-wide: {gas}"
        );
    }

    #[test]
    fn hypothesize_landauer_engine_kt_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        for knob in ["kt", "drop_ln2", "ln2"] {
            let blocked = lab.exec(Command::Set {
                theory: "landauer-engine".into(),
                knob: knob.into(),
                value: "true".into(),
            });
            assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
            assert!(
                blocked.text().contains("unknown knob") || blocked.text().contains(knob),
                "{}",
                blocked.text()
            );
        }
        let tape_blocked = lab.exec(Command::Set {
            theory: "landauer-engine".into(),
            knob: "tape_bound".into(),
            value: "1000".into(),
        });
        assert_eq!(tape_blocked.exit_code(), 1, "{}", tape_blocked.text());

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("landauer-engine".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-kt") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-kt: package → add-kt";
        let start = text.find(marker).expect("add-kt hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  landauer-engine  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let kt_block = &rest[..end];
        assert!(
            kt_block.contains("info.landauer-cost") && kt_block.contains("holds → fails"),
            "add-kt must flip landauer-cost holds to fails: {kt_block}"
        );
        assert!(
            !kt_block.contains("info.thermodynamically-free"),
            "add-kt is not the reversible Bennett probe: {kt_block}"
        );
        assert!(
            text.contains("add-demon"),
            "demon must still be an IR fork: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("landauer-engine").unwrap();
        assert!(
            live.evaluate_all()
                .iter()
                .any(|(c, v)| c.id_str() == "info.landauer-cost" && v.kind == VerdictKind::Holds),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("temperature_k").unwrap().display(),
            "300",
            "hypothesize must restore knobs"
        );
        assert_eq!(
            live.get("bits_erased").unwrap().display(),
            "1",
            "hypothesize must restore knobs"
        );
        assert_eq!(
            live.get("reversible").unwrap().display(),
            "false",
            "hypothesize must restore knobs"
        );
        let tm = lab.theory("turing-machine").unwrap();
        assert_eq!(
            tm.get("tape_bound").unwrap().display(),
            "0",
            "landauer IR must not convert the Turing-machine tape_bound knob"
        );
        assert_eq!(
            tm.get("nondeterministic").unwrap().display(),
            "false",
            "landauer IR must not convert the Turing-machine nondeterministic knob"
        );
        let rev = lab.exec(Command::Set {
            theory: "landauer-engine".into(),
            knob: "reversible".into(),
            value: "true".into(),
        });
        assert_eq!(rev.exit_code(), 0, "{}", rev.text());
        assert!(
            rev.text().contains("info.thermodynamically-free")
                && rev.text().contains("fails → holds"),
            "{}",
            rev.text()
        );
        let live = lab.theory("landauer-engine").unwrap();
        assert!(
            live.evaluate_all()
                .iter()
                .any(|(c, v)| c.id_str() == "info.landauer-cost" && v.kind == VerdictKind::Holds),
            "reversible still Holds cost on the live ln2 encoding"
        );
        let _ = lab.exec(Command::Set {
            theory: "landauer-engine".into(),
            knob: "reversible".into(),
            value: "false".into(),
        });
        let why = lab
            .exec(Command::Why {
                claim: "info.landauer-cost".into(),
            })
            .text()
            .to_string();
        let le = why_theory_block(&why, "landauer-engine");
        assert!(
            le.contains("kT ln2 Landauer bound"),
            "landauer-cost must name kT ln2: {le}"
        );
        assert!(
            !le.contains("not yet a machine-checked regime"),
            "landauer-cost must not be encoding-wide: {le}"
        );
    }

    #[test]
    fn hypothesize_landauer_engine_demon_is_ir_not_a_knob() {
        let mut lab = Lab::standard();
        let journal_len = lab.journal().len();
        for knob in ["demon", "memory"] {
            let blocked = lab.exec(Command::Set {
                theory: "landauer-engine".into(),
                knob: knob.into(),
                value: "true".into(),
            });
            assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
            assert!(
                blocked.text().contains("unknown knob") || blocked.text().contains(knob),
                "{}",
                blocked.text()
            );
        }

        let text = lab
            .exec(Command::Hypothesize {
                theory: Some("landauer-engine".into()),
            })
            .text()
            .to_string();
        assert!(
            text.contains("ir package mutations are not knobs"),
            "{text}"
        );
        assert!(
            text.contains("add-demon") && text.contains("ir structural"),
            "{text}"
        );
        let marker = "add-demon: package → add-demon";
        let start = text.find(marker).expect("add-demon hit");
        let rest = &text[start..];
        let end = rest[marker.len()..]
            .find("\n  landauer-engine  ")
            .map(|i| marker.len() + i)
            .unwrap_or(rest.len());
        let demon_block = &rest[..end];
        assert!(
            demon_block.contains("info.landauer-cost") && demon_block.contains("holds → fails"),
            "add-demon must flip landauer-cost holds to fails: {demon_block}"
        );
        assert!(
            !demon_block.contains("info.thermodynamically-free"),
            "add-demon is not the reversible Bennett probe: {demon_block}"
        );
        assert!(
            text.contains("add-kt"),
            "add-kt must still be an IR fork: {text}"
        );
        assert!(!text.contains("theorem"), "{text}");
        assert_eq!(lab.journal().len(), journal_len);
        let live = lab.theory("landauer-engine").unwrap();
        assert!(
            live.evaluate_all()
                .iter()
                .any(|(c, v)| c.id_str() == "info.landauer-cost" && v.kind == VerdictKind::Holds),
            "IR mutant must not be installed"
        );
        assert_eq!(
            live.get("reversible").unwrap().display(),
            "false",
            "hypothesize must restore knobs"
        );
        let tm = lab.theory("turing-machine").unwrap();
        assert_eq!(
            tm.get("tape_bound").unwrap().display(),
            "0",
            "landauer IR must not convert the Turing-machine tape_bound knob"
        );
        let why = lab
            .exec(Command::Why {
                claim: "info.landauer-cost".into(),
            })
            .text()
            .to_string();
        let le = why_theory_block(&why, "landauer-engine");
        assert!(
            le.contains("kT ln2 Landauer bound"),
            "landauer-cost must name kT ln2: {le}"
        );
        assert!(
            !le.contains("not yet a machine-checked regime"),
            "landauer-cost must not be encoding-wide: {le}"
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

    fn numeric_certificate_id(text: &str) -> physis_core::artifact::ArtifactId {
        let line = text.lines().next().expect("empty enclose");
        let hex = line.split_whitespace().last().expect("certificate hex");
        physis_core::artifact::ArtifactId::from_hex(hex)
            .unwrap_or_else(|| panic!("expected 64 hex certificate id in {line}"))
    }

    fn source_node_id(text: &str) -> physis_core::artifact::ArtifactId {
        let line = text.lines().next().expect("empty cite");
        let hex = line.split_whitespace().last().expect("source hex");
        physis_core::artifact::ArtifactId::from_hex(hex)
            .unwrap_or_else(|| panic!("expected 64 hex source id in {line}"))
    }

    fn constant_node_id(text: &str) -> physis_core::artifact::ArtifactId {
        let line = text.lines().next().expect("empty constant");
        let hex = line.split_whitespace().last().expect("node hex");
        physis_core::artifact::ArtifactId::from_hex(hex)
            .unwrap_or_else(|| panic!("expected 64 hex node id in {line}"))
    }

    fn encoding_package_id(text: &str) -> physis_core::artifact::ArtifactId {
        let line = text.lines().next().expect("empty encode");
        let hex = line.split_whitespace().last().expect("package hex");
        physis_core::artifact::ArtifactId::from_hex(hex)
            .unwrap_or_else(|| panic!("expected 64 hex package id in {line}"))
    }

    fn judgment_projection_id(text: &str) -> physis_core::artifact::ArtifactId {
        let line = text.lines().next().expect("empty judge");
        let hex = line.split_whitespace().last().expect("projection hex");
        physis_core::artifact::ArtifactId::from_hex(hex)
            .unwrap_or_else(|| panic!("expected 64 hex projection id in {line}"))
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
        assert!(
            text.contains("encode → judge → replicate"),
            "loop must project from_lab after encode: {text}"
        );
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
        assert!(
            text.contains("enclose  gut.weinberg-angle"),
            "loop must independently parse P3N Ratio strings: {text}"
        );
        assert!(
            text.contains("enclose  consistency.anomaly-cancellation"),
            "{text}"
        );
        assert!(
            !text.contains("enclose  predictivity.unique-vacuum"),
            "Asserted unique-vacuum is not CertifiedNumeric: {text}"
        );
        assert!(
            !text.contains("enclose  gut.proton-lifetime-sk"),
            "Super-K is not CertifiedNumeric: {text}"
        );
        assert!(
            text.contains("cite  gut.proton-lifetime-sk"),
            "loop must independently rebuild Super-K SourceRecord: {text}"
        );
        assert!(
            text.contains("cite  dec.d-squared-zero"),
            "loop must cite catalog dossiers without that being P3S by itself: {text}"
        );
        assert!(
            !text.contains("cite  predictivity.unique-vacuum"),
            "unique-vacuum has no precise source artifact: {text}"
        );
        assert!(
            text.contains("cite → constant → encode"),
            "loop must rebuild the constants ledger after cite: {text}"
        );
        assert!(
            text.contains("constant  ledger  044a027898acd4fbe72cfb6f012d248e24f95be834da6c9f5598cabc268a52c1"),
            "loop must independently rebuild the LEDGER bundle: {text}"
        );
        assert!(
            !text.contains("constant  hbar"),
            "loop must not invent ħ: {text}"
        );

        assert!(
            text.contains("encode  combinational-circuit"),
            "loop must independently round-trip the NAND netlist: {text}"
        );
        assert!(
            text.contains("encode  klein-gordon"),
            "loop must independently round-trip the Klein-Gordon stencil: {text}"
        );
        assert!(
            text.contains("encode  wilson-u1"),
            "loop must independently round-trip the Wilson U(1) stencil: {text}"
        );
        assert!(
            text.contains("encode  wilson-su2"),
            "loop must independently round-trip the Wilson SU(2) stencil: {text}"
        );
        assert!(
            text.contains("encode  wilson-su3"),
            "loop must independently round-trip the Wilson SU(3) stencil: {text}"
        );
        assert!(
            text.contains("encode  ohm-circuit"),
            "loop must independently round-trip the lumped Kirchhoff netlist: {text}"
        );
        assert!(
            text.contains("encode  bell-test"),
            "loop must independently round-trip the singlet ket: {text}"
        );
        assert!(
            text.contains("encode  newtonian-gravity"),
            "loop must independently round-trip the inverse-square Binet rhs: {text}"
        );
        assert!(
            text.contains("encode  linear-medium"),
            "loop must independently round-trip the isotropic-linear constitutive law: {text}"
        );
        assert!(
            text.contains("encode  maxwell-vacuum"),
            "loop must independently round-trip the homogeneous Faraday encoding: {text}"
        );
        assert!(
            text.contains("encode  ideal-gas"),
            "loop must independently round-trip Maxwell-Boltzmann statistics: {text}"
        );
        assert!(
            text.contains("encode  landauer-engine"),
            "loop must independently round-trip the kT ln2 Landauer bound: {text}"
        );
        assert!(
            text.contains("encode  dirac-fermion"),
            "loop must independently round-trip the naive Dirac operator: {text}"
        );
        assert!(
            text.contains("encode  general-relativity"),
            "loop must independently round-trip the Einstein-Hilbert action: {text}"
        );
        assert!(
            text.contains("encode  special-relativity"),
            "loop must independently round-trip the Lorentz boost: {text}"
        );
        assert!(
            text.contains("encode  planck"),
            "loop must independently round-trip Planck-Bose occupation: {text}"
        );
        assert!(
            text.contains("encode  de-rham"),
            "loop must independently round-trip the discrete coboundary identity: {text}"
        );
        assert!(
            text.contains("encode  turing-machine"),
            "loop must independently round-trip the unrelativized TM: {text}"
        );
        assert!(
            text.contains("encode  olbers-static"),
            "loop must independently round-trip inverse-square Euclidean shells: {text}"
        );
        assert!(
            text.contains("encode  su5-gut"),
            "loop must independently round-trip complete 5bar + 10: {text}"
        );
        assert!(
            text.contains("encode  debye-solid"),
            "loop must independently round-trip the 3D ω² continuum: {text}"
        );
        assert!(
            !text.contains("encode  einstein-solid"),
            "einstein-solid has no IR package: {text}"
        );
        assert!(
            text.contains("encode  standard-model"),
            "loop must independently round-trip complete Weyl content: {text}"
        );
        assert!(
            text.contains("encode  observer-geometry"),
            "loop must independently round-trip Spin(10) on 10-fibre: {text}"
        );
        assert!(
            text.contains("encode  dulong-petit"),
            "loop must independently round-trip harmonic U = 3 N k T: {text}"
        );
        assert!(
            text.contains("encode  heterotic-e8e8"),
            "loop must independently round-trip complete E8 x E8: {text}"
        );
        assert!(
            text.contains("encode  heterotic-so32"),
            "loop must independently round-trip complete SO(32): {text}"
        );
        assert!(
            text.contains("encode  type-i\n") || text.contains("encode  type-i  "),
            "loop must independently round-trip Chan-Paton SO(32): {text}"
        );
        assert!(
            !text.contains("encode  olbers-horizon"),
            "olbers-horizon has no IR package: {text}"
        );
        assert!(
            !text.contains("encode  rayleigh-jeans"),
            "Rayleigh–Jeans has no IR package: {text}"
        );
        assert!(
            !text.contains("encode  type-iib"),
            "type-iib has no IR package: {text}"
        );
        assert!(
            text.contains("judge  predictivity.unique-vacuum"),
            "loop must independently project unique-vacuum from_lab: {text}"
        );
        assert!(
            text.contains("judge  gut.proton-lifetime-sk"),
            "loop must project Super-K as empirical, not logical: {text}"
        );
        assert!(
            text.contains("judge  dec.d-squared-zero"),
            "loop must project catalog identities after prove: {text}"
        );
        assert!(
            text.contains("judge  gut.weinberg-angle"),
            "loop must project GUT-scale 3/8 as numeric certified: {text}"
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
        let p3n = lab
            .exec(Command::Inspect {
                axis: Some("trust".into()),
                value: Some("P3N".into()),
            })
            .text()
            .to_string();
        assert!(
            p3n.contains("count 4"),
            "loop constant rebuild must not mint P3N: {p3n}"
        );
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
    fn numerical_verifier_encloses_ratio_strings_and_cannot_prove() {
        let mut lab = Lab::standard();
        lab.set_role(Role::Explorer);
        let blocked = lab.exec(Command::Enclose {
            claim: "gut.weinberg-angle".into(),
        });
        assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
        assert!(
            blocked.text().contains("explorer cannot enclose"),
            "{}",
            blocked.text()
        );

        lab.set_role(Role::ProofSearcher);
        let blocked_ps = lab.exec(Command::Enclose {
            claim: "gut.weinberg-angle".into(),
        });
        assert!(
            blocked_ps.text().contains("proof-searcher cannot enclose"),
            "{}",
            blocked_ps.text()
        );

        lab.set_role(Role::NumericalVerifier);
        let prove = lab.exec(Command::Prove {
            claim: "dec.d-squared-zero".into(),
        });
        assert!(
            prove.text().contains("numerical-verifier cannot prove"),
            "{}",
            prove.text()
        );

        let gut = lab
            .exec(Command::Enclose {
                claim: "gut.weinberg-angle".into(),
            })
            .text()
            .to_string();
        assert!(gut.contains("enclosure    [3/8, 3/8]"), "{gut}");
        assert!(gut.contains("not a kernel proof"), "{gut}");
        assert!(gut.contains("not P4"), "{gut}");
        assert!(!gut.contains("receipt"), "{gut}");
        let gut_id = numeric_certificate_id(&gut);
        assert_eq!(
            lab.store.get(gut_id).map(|n| n.kind),
            Some(NodeKind::NumericCertificate)
        );

        let anom = lab
            .exec(Command::Enclose {
                claim: "consistency.anomaly-cancellation".into(),
            })
            .text()
            .to_string();
        assert!(anom.contains("standard-model"), "{anom}");
        assert!(anom.contains("enclosure    [0, 0]"), "{anom}");
        assert!(
            anom.contains("skipped") && anom.contains("type-iib"),
            "string Green-Schwarz must not be parsed as Ratio: {anom}"
        );

        let y = lab
            .exec(Command::Enclose {
                claim: "sm.hypercharge-derivation".into(),
            })
            .text()
            .to_string();
        assert!(y.contains("enclosure    [-1/2, -1/2]"), "{y}");

        let h = lab
            .exec(Command::Enclose {
                claim: "empirical.charge-quantization".into(),
            })
            .text()
            .to_string();
        assert!(h.contains("enclosure    [0, 0]"), "{h}");

        for (claim, why_token) in [
            ("predictivity.unique-vacuum", "no certified-numeric"),
            ("gut.proton-lifetime-sk", "no certified-numeric"),
            ("gut.weinberg-angle-mz-interval", "no certified-numeric"),
            ("dec.closed-equals-exact", "no certified-numeric"),
        ] {
            let resp = lab.exec(Command::Enclose {
                claim: claim.into(),
            });
            assert_eq!(resp.exit_code(), 1, "{claim} {}", resp.text());
            assert!(resp.text().contains(why_token), "{claim} {}", resp.text());
        }

        let p3n = lab
            .exec(Command::Inspect {
                axis: Some("trust".into()),
                value: Some("P3N".into()),
            })
            .text()
            .to_string();
        assert!(
            p3n.contains("count 4"),
            "independent enclose must not mint extra P3N: {p3n}"
        );
        assert!(!p3n.contains("gut.proton-lifetime-sk"), "{p3n}");
        assert!(!p3n.contains("gut.weinberg-angle-mz"), "{p3n}");

        let why = lab
            .exec(Command::Why {
                claim: "gut.weinberg-angle".into(),
            })
            .text()
            .to_string();
        assert!(why.contains(&format!("enclose:     {gut_id}")), "{why}");
        assert!(why.contains("P3N"), "{why}");
        assert!(!why.contains("P4"), "{why}");
    }

    #[test]
    fn numeric_certificate_restores_by_rebuild_not_deserialize() {
        let mut lab1 = Lab::standard();
        let first = lab1
            .exec(Command::Enclose {
                claim: "gut.weinberg-angle".into(),
            })
            .text()
            .to_string();
        let live = numeric_certificate_id(&first);
        assert_eq!(
            live.to_hex(),
            "0967e9f42ec9ff0fd8e29fecc5bb5a3ed9aba4974ac77b0e5217a4bb634ec202",
            "journaling must not change the GUT-scale 3/8 certificate payload"
        );
        let jsonl = lab1.journal().to_string();
        assert!(jsonl.contains("\"event\":\"enclose\""), "{jsonl}");
        assert!(
            jsonl.contains(&format!("\"certificate_hash\":\"{}\"", live.to_hex())),
            "{jsonl}"
        );

        let mut lab2 = Lab::standard();
        assert_eq!(
            lab2.store
                .iter()
                .filter(|n| n.kind == NodeKind::NumericCertificate)
                .count(),
            0
        );
        *lab2.journal_mut() = Journal::from_jsonl(&jsonl);
        assert_eq!(
            lab2.store
                .iter()
                .filter(|n| n.kind == NodeKind::NumericCertificate)
                .count(),
            0,
            "from_jsonl must not insert NumericCertificate"
        );
        let journal_len = lab2.journal().len();
        lab2.restore_from_journal();
        assert_eq!(
            lab2.journal().len(),
            journal_len,
            "restore must not journal enclose again"
        );
        assert_eq!(
            lab2.store.get(live).map(|n| n.kind),
            Some(NodeKind::NumericCertificate),
            "restore rebuilds the live certificate"
        );

        let forged_hex = "0".repeat(64);
        let tampered = format!(
            r#"{{"event":"enclose","t":1,"claim":"gut.weinberg-angle","certificate_hash":"{forged_hex}"}}"#
        );
        let mut lab3 = Lab::standard();
        *lab3.journal_mut() = Journal::from_jsonl(&tampered);
        lab3.restore_from_journal();
        assert_eq!(
            lab3.store.get(live).map(|n| n.kind),
            Some(NodeKind::NumericCertificate),
            "tampered certificate_hash is not the DAG"
        );
        let forged = physis_core::artifact::ArtifactId::from_hex(&forged_hex)
            .expect("64 hex zeros is an ArtifactId");
        assert!(
            lab3.store.get(forged).is_none(),
            "a forged hash cannot mint the certificate"
        );
        assert_eq!(lab3.journal().len(), 1, "tampered restore must not append");

        let why = lab2
            .exec(Command::Why {
                claim: "gut.weinberg-angle".into(),
            })
            .text()
            .to_string();
        assert!(why.contains(&format!("enclose:     {live}")), "{why}");
    }

    #[test]
    fn provenance_auditor_cites_source_records_and_cannot_review() {
        let mut lab = Lab::standard();
        lab.set_role(Role::Explorer);
        let blocked = lab.exec(Command::Cite {
            claim: "gut.proton-lifetime-sk".into(),
        });
        assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
        assert!(
            blocked.text().contains("explorer cannot cite"),
            "{}",
            blocked.text()
        );

        lab.set_role(Role::Reviewer);
        let blocked_rev = lab.exec(Command::Cite {
            claim: "gut.proton-lifetime-sk".into(),
        });
        assert!(
            blocked_rev.text().contains("reviewer cannot cite"),
            "{}",
            blocked_rev.text()
        );

        lab.set_role(Role::ProvenanceAuditor);
        let review = lab.exec(Command::Review {
            claim: "dec.d-squared-zero".into(),
        });
        assert!(
            review.text().contains("provenance-auditor cannot review"),
            "{}",
            review.text()
        );
        let prove = lab.exec(Command::Prove {
            claim: "dec.d-squared-zero".into(),
        });
        assert!(
            prove.text().contains("provenance-auditor cannot prove"),
            "{}",
            prove.text()
        );

        let sk = lab
            .exec(Command::Cite {
                claim: "gut.proton-lifetime-sk".into(),
            })
            .text()
            .to_string();
        assert!(sk.contains("kind     dataset  sk-2020-p-e-pi0"), "{sk}");
        assert!(sk.contains("Takenaka"), "{sk}");
        assert!(sk.contains("not P3S"), "{sk}");
        assert!(!sk.contains("receipt"), "{sk}");
        let sk_id = source_node_id(&sk);
        assert_eq!(lab.store.get(sk_id).map(|n| n.kind), Some(NodeKind::Source));

        let pdg = lab
            .exec(Command::Cite {
                claim: "gut.weinberg-angle-mz-interval".into(),
            })
            .text()
            .to_string();
        assert!(pdg.contains("pdg-2024-sin2theta"), "{pdg}");
        assert!(pdg.contains("PDG Review"), "{pdg}");

        let d2 = lab
            .exec(Command::Cite {
                claim: "dec.d-squared-zero".into(),
            })
            .text()
            .to_string();
        assert!(d2.contains("kind     dossier  catalog"), "{d2}");
        assert!(d2.contains("Desbrun"), "{d2}");
        assert!(d2.contains("not P3S"), "{d2}");

        for claim in [
            "predictivity.unique-vacuum",
            "gut.weinberg-angle",
            "dec.closed-equals-exact",
        ] {
            let resp = lab.exec(Command::Cite {
                claim: claim.into(),
            });
            assert_eq!(resp.exit_code(), 1, "{claim} {}", resp.text());
            assert!(
                resp.text().contains("no precise source artifact"),
                "{claim} {}",
                resp.text()
            );
        }

        let p3s = lab
            .exec(Command::Inspect {
                axis: Some("trust".into()),
                value: Some("P3S".into()),
            })
            .text()
            .to_string();
        assert!(p3s.contains("count 0"), "cite must not raise P3S: {p3s}");
        let p3n = lab
            .exec(Command::Inspect {
                axis: Some("trust".into()),
                value: Some("P3N".into()),
            })
            .text()
            .to_string();
        assert!(p3n.contains("count 4"), "cite must not mint P3N: {p3n}");

        let why = lab
            .exec(Command::Why {
                claim: "gut.proton-lifetime-sk".into(),
            })
            .text()
            .to_string();
        assert!(why.contains(&format!("source:      {sk_id}")), "{why}");
        assert!(!why.contains("P3S"), "{why}");
    }

    #[test]
    fn source_record_restores_by_rebuild_not_deserialize() {
        let mut lab1 = Lab::standard();
        let first = lab1
            .exec(Command::Cite {
                claim: "gut.proton-lifetime-sk".into(),
            })
            .text()
            .to_string();
        let live = source_node_id(&first);
        assert_eq!(
            live.to_hex(),
            "26467998781b7d501f90a1dc762d3c16ae636f867ea61152923c505e1ad3bbef",
            "journaling must not change the Super-K source payload"
        );
        let jsonl = lab1.journal().to_string();
        assert!(jsonl.contains("\"event\":\"cite\""), "{jsonl}");
        assert!(
            jsonl.contains(&format!("\"source_hash\":\"{}\"", live.to_hex())),
            "{jsonl}"
        );

        let mut lab2 = Lab::standard();
        *lab2.journal_mut() = Journal::from_jsonl(&jsonl);
        let journal_len = lab2.journal().len();
        lab2.restore_from_journal();
        assert_eq!(lab2.journal().len(), journal_len);
        assert_eq!(lab2.store.get(live).map(|n| n.kind), Some(NodeKind::Source));

        let forged_hex = "0".repeat(64);
        let tampered = format!(
            r#"{{"event":"cite","t":1,"claim":"gut.proton-lifetime-sk","source_hash":"{forged_hex}"}}"#
        );
        let mut lab3 = Lab::standard();
        *lab3.journal_mut() = Journal::from_jsonl(&tampered);
        lab3.restore_from_journal();
        assert_eq!(lab3.store.get(live).map(|n| n.kind), Some(NodeKind::Source));
        let forged = physis_core::artifact::ArtifactId::from_hex(&forged_hex)
            .expect("64 hex zeros is an ArtifactId");
        assert!(lab3.store.get(forged).is_none());
    }

    #[test]
    fn provenance_auditor_rebuilds_versioned_constants_and_cannot_review() {
        let mut lab = Lab::standard();
        lab.set_role(Role::Explorer);
        let blocked = lab.exec(Command::Constant {
            name: Some("G".into()),
        });
        assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
        assert!(
            blocked.text().contains("explorer cannot constant"),
            "{}",
            blocked.text()
        );
        let blocked_ledger = lab.exec(Command::Constant { name: None });
        assert_eq!(blocked_ledger.exit_code(), 1, "{}", blocked_ledger.text());
        assert!(
            blocked_ledger.text().contains("explorer cannot constant"),
            "{}",
            blocked_ledger.text()
        );

        lab.set_role(Role::Reviewer);
        let blocked_rev = lab.exec(Command::Constant {
            name: Some("G".into()),
        });
        assert!(
            blocked_rev.text().contains("reviewer cannot constant"),
            "{}",
            blocked_rev.text()
        );

        lab.set_role(Role::NumericalVerifier);
        let blocked_nv = lab.exec(Command::Constant {
            name: Some("G".into()),
        });
        assert!(
            blocked_nv
                .text()
                .contains("numerical-verifier cannot constant"),
            "{}",
            blocked_nv.text()
        );

        lab.set_role(Role::ProvenanceAuditor);
        let review = lab.exec(Command::Review {
            claim: "dec.d-squared-zero".into(),
        });
        assert!(
            review.text().contains("provenance-auditor cannot review"),
            "{}",
            review.text()
        );

        let g = lab
            .exec(Command::Constant {
                name: Some("G".into()),
            })
            .text()
            .to_string();
        assert!(g.contains("constant  G  node "), "{g}");
        assert!(
            g.contains("hash     ebbfc13ea8fba734da50b679d9eaf236638b244cdcc350c0b14cdd6696850e92"),
            "{g}"
        );
        assert!(g.contains("kind     interval"), "{g}");
        assert!(g.contains("table    XXXI"), "{g}");
        assert!(g.contains("rebuild  ok"), "{g}");
        assert!(g.contains("not P3N"), "{g}");
        assert!(g.contains("not P3S"), "{g}");
        assert!(!g.contains("receipt"), "{g}");
        assert!(!g.contains("theorem"), "{g}");
        let g_id = constant_node_id(&g);
        assert_eq!(
            lab.store.get(g_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let alpha = lab
            .exec(Command::Constant {
                name: Some("alpha".into()),
            })
            .text()
            .to_string();
        assert!(alpha.contains("constant  alpha  node "), "{alpha}");
        assert!(
            alpha.contains(
                "hash     cef64589acdbd1ed4cb5f5f631658978c01477248f334b1d3563e57314644b38"
            ),
            "{alpha}"
        );
        assert!(alpha.contains("kind     interval"), "{alpha}");
        assert!(alpha.contains("table    XXXI"), "{alpha}");
        assert!(
            alpha.contains("range    alpha = 7.2973525693(11)e-3"),
            "{alpha}"
        );
        assert!(alpha.contains("unit     1"), "{alpha}");
        assert!(alpha.contains("rebuild  ok"), "{alpha}");
        assert!(alpha.contains("not P3N"), "{alpha}");
        assert!(!alpha.contains("receipt"), "{alpha}");
        assert!(!alpha.contains("theorem"), "{alpha}");
        let alpha_id = constant_node_id(&alpha);
        assert_eq!(
            alpha_id.to_hex(),
            "b2b54749bb1e674d72e0b1c7ffa688dbd1cabb8a8a481db3f94bfeba9735f073",
            "journaling must not change the alpha constant payload"
        );
        assert_eq!(
            lab.store.get(alpha_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let inv_alpha = lab
            .exec(Command::Constant {
                name: Some("inv_alpha".into()),
            })
            .text()
            .to_string();
        assert!(
            inv_alpha.contains("constant  inv_alpha  node "),
            "{inv_alpha}"
        );
        assert!(
            inv_alpha.contains(
                "hash     4b7050d77da09c5322877eaf83e94ebba7b84c99bad8ba3713b0e5fe91128482"
            ),
            "{inv_alpha}"
        );
        assert!(inv_alpha.contains("kind     interval"), "{inv_alpha}");
        assert!(inv_alpha.contains("table    XXXI"), "{inv_alpha}");
        assert!(
            inv_alpha.contains("range    inv_alpha = 137.035999084(21)"),
            "{inv_alpha}"
        );
        assert!(inv_alpha.contains("unit     1"), "{inv_alpha}");
        assert!(inv_alpha.contains("rebuild  ok"), "{inv_alpha}");
        assert!(inv_alpha.contains("not P3N"), "{inv_alpha}");
        assert!(!inv_alpha.contains("receipt"), "{inv_alpha}");
        assert!(!inv_alpha.contains("theorem"), "{inv_alpha}");
        let inv_alpha_id = constant_node_id(&inv_alpha);
        assert_eq!(
            inv_alpha_id.to_hex(),
            "6943c43fe01b2b9dbde1c0bd147f0293a69cb15bb2e44877ea7e68013f6dce0e",
            "journaling must not change the inv_alpha constant payload"
        );
        assert_eq!(
            lab.store.get(inv_alpha_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let rinf = lab
            .exec(Command::Constant {
                name: Some("Rinf".into()),
            })
            .text()
            .to_string();
        assert!(rinf.contains("constant  Rinf  node "), "{rinf}");
        assert!(
            rinf.contains(
                "hash     fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090"
            ),
            "{rinf}"
        );
        assert!(rinf.contains("kind     interval"), "{rinf}");
        assert!(rinf.contains("table    XXXI"), "{rinf}");
        assert!(
            rinf.contains("range    Rinf = 10973731.568160(21)"),
            "{rinf}"
        );
        assert!(rinf.contains("unit     m^{-1}"), "{rinf}");
        assert!(rinf.contains("rebuild  ok"), "{rinf}");
        assert!(rinf.contains("not P3N"), "{rinf}");
        assert!(!rinf.contains("receipt"), "{rinf}");
        assert!(!rinf.contains("theorem"), "{rinf}");
        let rinf_id = constant_node_id(&rinf);
        assert_eq!(
            rinf_id.to_hex(),
            "0fb78b2d6e881df7b19d8a55878f642e27dc4d51a8f74ffe0c1e28e9d93380ac",
            "journaling must not change the Rinf constant payload"
        );
        assert_eq!(
            lab.store.get(rinf_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let crinf = lab
            .exec(Command::Constant {
                name: Some("cRinf".into()),
            })
            .text()
            .to_string();
        assert!(crinf.contains("constant  cRinf  node "), "{crinf}");
        assert!(
            crinf.contains(
                "hash     c7c49f18cb4f9905decad406f7a835f59588f34483afa3e4751097451d5d9969"
            ),
            "{crinf}"
        );
        assert!(crinf.contains("kind     interval"), "{crinf}");
        assert!(crinf.contains("table    XXXI"), "{crinf}");
        assert!(
            crinf.contains("range    cRinf = 3.2898419602508(64)e15"),
            "{crinf}"
        );
        assert!(crinf.contains("unit     Hz"), "{crinf}");
        assert!(crinf.contains("rebuild  ok"), "{crinf}");
        assert!(crinf.contains("not P3N"), "{crinf}");
        assert!(!crinf.contains("receipt"), "{crinf}");
        assert!(!crinf.contains("theorem"), "{crinf}");
        let crinf_id = constant_node_id(&crinf);
        assert_eq!(
            crinf_id.to_hex(),
            "8fca9d435d8a31d1fafdac9a8825ce7f1535bf04eaf82785a1c62f66c900e60e",
            "journaling must not change the cRinf constant payload"
        );
        assert_eq!(
            lab.store.get(crinf_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let hcrinf = lab
            .exec(Command::Constant {
                name: Some("hcRinf".into()),
            })
            .text()
            .to_string();
        assert!(hcrinf.contains("constant  hcRinf  node "), "{hcrinf}");
        assert!(
            hcrinf.contains(
                "hash     0d0308e874e54cb3d02570c972232b0d26c2d1d64b493880a1bb7ce4ff7827b2"
            ),
            "{hcrinf}"
        );
        assert!(hcrinf.contains("kind     interval"), "{hcrinf}");
        assert!(hcrinf.contains("table    XXXI"), "{hcrinf}");
        assert!(
            hcrinf.contains("range    hcRinf = 2.1798723611035(42)e-18"),
            "{hcrinf}"
        );
        assert!(hcrinf.contains("unit     J"), "{hcrinf}");
        assert!(hcrinf.contains("rebuild  ok"), "{hcrinf}");
        assert!(hcrinf.contains("not P3N"), "{hcrinf}");
        assert!(!hcrinf.contains("receipt"), "{hcrinf}");
        assert!(!hcrinf.contains("theorem"), "{hcrinf}");
        let hcrinf_id = constant_node_id(&hcrinf);
        assert_eq!(
            hcrinf_id.to_hex(),
            "f7c095d695e231cfaee92b74cd8eb2961462727d1068401ee84953d069af4cbd",
            "journaling must not change the hcRinf constant payload"
        );
        assert_eq!(
            lab.store.get(hcrinf_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let a0 = lab
            .exec(Command::Constant {
                name: Some("a0".into()),
            })
            .text()
            .to_string();
        assert!(a0.contains("constant  a0  node "), "{a0}");
        assert!(
            a0.contains(
                "hash     5d5098fcd983d3db221e4b4047e73de5061985c31a91ccdf12cd122b620eaf29"
            ),
            "{a0}"
        );
        assert!(a0.contains("kind     interval"), "{a0}");
        assert!(a0.contains("table    XXXI"), "{a0}");
        assert!(a0.contains("range    a0 = 5.29177210903(80)e-11"), "{a0}");
        assert!(a0.contains("unit     m"), "{a0}");
        assert!(a0.contains("rebuild  ok"), "{a0}");
        assert!(a0.contains("not P3N"), "{a0}");
        assert!(!a0.contains("receipt"), "{a0}");
        assert!(!a0.contains("theorem"), "{a0}");
        let a0_id = constant_node_id(&a0);
        assert_eq!(
            a0_id.to_hex(),
            "01663e8bd28309970cefc37bd3dc5023c54a70ded784fb04d94ace095abdd475",
            "journaling must not change the a0 constant payload"
        );
        assert_eq!(
            lab.store.get(a0_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let eh = lab
            .exec(Command::Constant {
                name: Some("Eh".into()),
            })
            .text()
            .to_string();
        assert!(eh.contains("constant  Eh  node "), "{eh}");
        assert!(
            eh.contains(
                "hash     c4606c77e55763a397f633ef0f3ace1328d3e1e8781428baf97554c97f4fba5a"
            ),
            "{eh}"
        );
        assert!(eh.contains("kind     interval"), "{eh}");
        assert!(eh.contains("table    XXXI"), "{eh}");
        assert!(eh.contains("range    Eh = 4.3597447222071(85)e-18"), "{eh}");
        assert!(eh.contains("unit     J"), "{eh}");
        assert!(eh.contains("rebuild  ok"), "{eh}");
        assert!(eh.contains("not P3N"), "{eh}");
        assert!(!eh.contains("receipt"), "{eh}");
        assert!(!eh.contains("theorem"), "{eh}");
        let eh_id = constant_node_id(&eh);
        assert_eq!(
            eh_id.to_hex(),
            "84818158c407563a9a514c8eedc85ee7303b0d96f09f09610bda6684582cc82e",
            "journaling must not change the Eh constant payload"
        );
        assert_eq!(
            lab.store.get(eh_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let me_mmu = lab
            .exec(Command::Constant {
                name: Some("me_mmu".into()),
            })
            .text()
            .to_string();
        assert!(me_mmu.contains("constant  me_mmu  node "), "{me_mmu}");
        assert!(
            me_mmu.contains(
                "hash     d57979e61fa03bae0a3b0dc5e2cff20df53cdcb76b772cf6ea2589e77c9c3cb2"
            ),
            "{me_mmu}"
        );
        assert!(me_mmu.contains("kind     interval"), "{me_mmu}");
        assert!(me_mmu.contains("table    XXXI"), "{me_mmu}");
        assert!(
            me_mmu.contains("range    me/mmu = 4.83633169(11)e-3"),
            "{me_mmu}"
        );
        assert!(me_mmu.contains("unit     1"), "{me_mmu}");
        assert!(me_mmu.contains("rebuild  ok"), "{me_mmu}");
        assert!(me_mmu.contains("not P3N"), "{me_mmu}");
        assert!(!me_mmu.contains("receipt"), "{me_mmu}");
        assert!(!me_mmu.contains("theorem"), "{me_mmu}");
        let me_mmu_id = constant_node_id(&me_mmu);
        assert_eq!(
            me_mmu_id.to_hex(),
            "60d9b01d547b5ad4307443e4ba7749adb42c4da1343f16a35f194c80bbc35088",
            "journaling must not change the me_mmu constant payload"
        );
        assert_eq!(
            lab.store.get(me_mmu_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let me_mp = lab
            .exec(Command::Constant {
                name: Some("me_mp".into()),
            })
            .text()
            .to_string();
        assert!(me_mp.contains("constant  me_mp  node "), "{me_mp}");
        assert!(
            me_mp.contains(
                "hash     b573fa37eb0080e54bc71e3bf41170421c2bae2911609e1d11ffc129448a2e7b"
            ),
            "{me_mp}"
        );
        assert!(me_mp.contains("kind     interval"), "{me_mp}");
        assert!(me_mp.contains("table    XXXI"), "{me_mp}");
        assert!(
            me_mp.contains("range    me/mp = 5.44617021487(33)e-4"),
            "{me_mp}"
        );
        assert!(me_mp.contains("unit     1"), "{me_mp}");
        assert!(me_mp.contains("rebuild  ok"), "{me_mp}");
        assert!(me_mp.contains("not P3N"), "{me_mp}");
        assert!(!me_mp.contains("receipt"), "{me_mp}");
        assert!(!me_mp.contains("theorem"), "{me_mp}");
        let me_mp_id = constant_node_id(&me_mp);
        assert_eq!(
            me_mp_id.to_hex(),
            "b4fd3e8b7678afd9bb4aea49c3b06c9756ab3d6fced7b4b49b25c322134bf3f2",
            "journaling must not change the me_mp constant payload"
        );
        assert_eq!(
            lab.store.get(me_mp_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let me_mn = lab
            .exec(Command::Constant {
                name: Some("me_mn".into()),
            })
            .text()
            .to_string();
        assert!(me_mn.contains("constant  me_mn  node "), "{me_mn}");
        assert!(
            me_mn.contains(
                "hash     e271d2015c7b39491daebf2a1d532ebe4c4dacf8228b3f7fc4d258be7b79ecba"
            ),
            "{me_mn}"
        );
        assert!(me_mn.contains("kind     interval"), "{me_mn}");
        assert!(me_mn.contains("table    XXXI"), "{me_mn}");
        assert!(
            me_mn.contains("range    me/mn = 5.4386734424(26)e-4"),
            "{me_mn}"
        );
        assert!(me_mn.contains("unit     1"), "{me_mn}");
        assert!(me_mn.contains("rebuild  ok"), "{me_mn}");
        assert!(me_mn.contains("not P3N"), "{me_mn}");
        assert!(!me_mn.contains("receipt"), "{me_mn}");
        assert!(!me_mn.contains("theorem"), "{me_mn}");
        let me_mn_id = constant_node_id(&me_mn);
        assert_eq!(
            me_mn_id.to_hex(),
            "deeb5e2665cabc16ffa607d446a4018cabf8b2b427fdb0b81184384113089bb3",
            "journaling must not change the me_mn constant payload"
        );
        assert_eq!(
            lab.store.get(me_mn_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let me_md = lab
            .exec(Command::Constant {
                name: Some("me_md".into()),
            })
            .text()
            .to_string();
        assert!(me_md.contains("constant  me_md  node "), "{me_md}");
        assert!(
            me_md.contains(
                "hash     2aa5fe69f8cdd03f44e77b006a3b6ea90d48e1b8aec71275e184c4e529f0f76c"
            ),
            "{me_md}"
        );
        assert!(me_md.contains("kind     interval"), "{me_md}");
        assert!(me_md.contains("table    XXXI"), "{me_md}");
        assert!(
            me_md.contains("range    me/md = 2.724437107462(96)e-4"),
            "{me_md}"
        );
        assert!(me_md.contains("unit     1"), "{me_md}");
        assert!(me_md.contains("rebuild  ok"), "{me_md}");
        assert!(me_md.contains("not P3N"), "{me_md}");
        assert!(!me_md.contains("receipt"), "{me_md}");
        assert!(!me_md.contains("theorem"), "{me_md}");
        let me_md_id = constant_node_id(&me_md);
        assert_eq!(
            me_md_id.to_hex(),
            "a2b8e4d5a5cdff854b67986773f186e1f427dc9cfc0d6d92f3a01ee81bdd26e5",
            "journaling must not change the me_md constant payload"
        );
        assert_eq!(
            lab.store.get(me_md_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let me_mt = lab
            .exec(Command::Constant {
                name: Some("me_mt".into()),
            })
            .text()
            .to_string();
        assert!(me_mt.contains("constant  me_mt  node "), "{me_mt}");
        assert!(
            me_mt.contains(
                "hash     2f8187d744269836cf0fbc123f8cb7d60107215e65be109daf2ae67c8116afd1"
            ),
            "{me_mt}"
        );
        assert!(me_mt.contains("kind     interval"), "{me_mt}");
        assert!(me_mt.contains("table    XXXI"), "{me_mt}");
        assert!(
            me_mt.contains("range    me/mt = 1.819200062251(90)e-4"),
            "{me_mt}"
        );
        assert!(me_mt.contains("unit     1"), "{me_mt}");
        assert!(me_mt.contains("rebuild  ok"), "{me_mt}");
        assert!(me_mt.contains("not P3N"), "{me_mt}");
        assert!(!me_mt.contains("receipt"), "{me_mt}");
        assert!(!me_mt.contains("theorem"), "{me_mt}");
        let me_mt_id = constant_node_id(&me_mt);
        assert_eq!(
            me_mt_id.to_hex(),
            "3d9b3ce3c7ecca0e131e0232f308ce878696a268e263286e133c8edc441eb7f0",
            "journaling must not change the me_mt constant payload"
        );
        assert_eq!(
            lab.store.get(me_mt_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let me_mh = lab
            .exec(Command::Constant {
                name: Some("me_mh".into()),
            })
            .text()
            .to_string();
        assert!(me_mh.contains("constant  me_mh  node "), "{me_mh}");
        assert!(
            me_mh.contains(
                "hash     0fb8f5fde9e76fcf2c24d73267f36cd3a5b40ca9f27f24e47d9807ec4206055e"
            ),
            "{me_mh}"
        );
        assert!(me_mh.contains("kind     interval"), "{me_mh}");
        assert!(me_mh.contains("table    XXXI"), "{me_mh}");
        assert!(
            me_mh.contains("range    me/mh = 1.819543074573(79)e-4"),
            "{me_mh}"
        );
        assert!(me_mh.contains("unit     1"), "{me_mh}");
        assert!(me_mh.contains("rebuild  ok"), "{me_mh}");
        assert!(me_mh.contains("not P3N"), "{me_mh}");
        assert!(!me_mh.contains("receipt"), "{me_mh}");
        assert!(!me_mh.contains("theorem"), "{me_mh}");
        let me_mh_id = constant_node_id(&me_mh);
        assert_eq!(
            me_mh_id.to_hex(),
            "b55534bac40b377d7b8c6123de509a2b65cde4d75fe280d46aefa30f83e72890",
            "journaling must not change the me_mh constant payload"
        );
        assert_eq!(
            lab.store.get(me_mh_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let me_malpha = lab
            .exec(Command::Constant {
                name: Some("me_malpha".into()),
            })
            .text()
            .to_string();
        assert!(
            me_malpha.contains("constant  me_malpha  node "),
            "{me_malpha}"
        );
        assert!(
            me_malpha.contains(
                "hash     3407529f38a47a2cf983c5418482d3d9bab5243e08258bbe88c06a2f42e0baa3"
            ),
            "{me_malpha}"
        );
        assert!(me_malpha.contains("kind     interval"), "{me_malpha}");
        assert!(me_malpha.contains("table    XXXI"), "{me_malpha}");
        assert!(
            me_malpha.contains("range    me/malpha = 1.370933554787(45)e-4"),
            "{me_malpha}"
        );
        assert!(me_malpha.contains("unit     1"), "{me_malpha}");
        assert!(me_malpha.contains("rebuild  ok"), "{me_malpha}");
        assert!(me_malpha.contains("not P3N"), "{me_malpha}");
        assert!(!me_malpha.contains("receipt"), "{me_malpha}");
        assert!(!me_malpha.contains("theorem"), "{me_malpha}");
        let me_malpha_id = constant_node_id(&me_malpha);
        assert_eq!(
            me_malpha_id.to_hex(),
            "ddb38fbd88d7250c7aea0e87e0bd2c44b32d5b5b0fd9eb1b0689bb9aa3315545",
            "journaling must not change the me_malpha constant payload"
        );
        assert_eq!(
            lab.store.get(me_malpha_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let e_me = lab
            .exec(Command::Constant {
                name: Some("e_me".into()),
            })
            .text()
            .to_string();
        assert!(e_me.contains("constant  e_me  node "), "{e_me}");
        assert!(
            e_me.contains(
                "hash     bfe24e8de43e90dbc8a28472f99ed206f07566fa1a4fa6c6d14356adf4e89b22"
            ),
            "{e_me}"
        );
        assert!(e_me.contains("kind     interval"), "{e_me}");
        assert!(e_me.contains("table    XXXI"), "{e_me}");
        assert!(
            e_me.contains("range    -e/me = -1.75882001076(53)e11"),
            "{e_me}"
        );
        assert!(e_me.contains("unit     C kg^{-1}"), "{e_me}");
        assert!(
            e_me.contains("value    [-175882001129, -175882001023]"),
            "{e_me}"
        );
        assert!(e_me.contains("rebuild  ok"), "{e_me}");
        assert!(e_me.contains("not P3N"), "{e_me}");
        assert!(!e_me.contains("receipt"), "{e_me}");
        assert!(!e_me.contains("theorem"), "{e_me}");
        let e_me_id = constant_node_id(&e_me);
        assert_eq!(
            e_me_id.to_hex(),
            "4180ebda17cac1399d5888468d4686d9874499a1e6b2c386a3ccbe58f8039f36",
            "journaling must not change the e_me constant payload"
        );
        assert_eq!(
            lab.store.get(e_me_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let molar = lab
            .exec(Command::Constant {
                name: Some("M_e".into()),
            })
            .text()
            .to_string();
        assert!(molar.contains("constant  M_e  node "), "{molar}");
        assert!(
            molar.contains(
                "hash     0a8b3285a4969854567b59db2ebf9449268df86ffdbb461e3b9c1db0955eb804"
            ),
            "{molar}"
        );
        assert!(molar.contains("kind     interval"), "{molar}");
        assert!(molar.contains("table    XXXI"), "{molar}");
        assert!(
            molar.contains("range    Me = 5.4857990888(17)e-7"),
            "{molar}"
        );
        assert!(molar.contains("unit     kg mol^{-1}"), "{molar}");
        assert!(
            molar.contains(
                "value    [54857990871/100000000000000000, 10971598181/20000000000000000]"
            ),
            "{molar}"
        );
        assert!(molar.contains("rebuild  ok"), "{molar}");
        assert!(molar.contains("not P3N"), "{molar}");
        assert!(!molar.contains("receipt"), "{molar}");
        assert!(!molar.contains("theorem"), "{molar}");
        let molar_id = constant_node_id(&molar);
        assert_eq!(
            molar_id.to_hex(),
            "da1692471def8d3d930d45de5d4e089231c2d18fc859d73feeb22ffe89075692",
            "journaling must not change the M_e constant payload"
        );
        assert_eq!(
            lab.store.get(molar_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let rcbar = lab
            .exec(Command::Constant {
                name: Some("lambdabar_C".into()),
            })
            .text()
            .to_string();
        assert!(rcbar.contains("constant  lambdabar_C  node "), "{rcbar}");
        assert!(
            rcbar.contains(
                "hash     0ed48571f065fc19458ea3c8fd493fd00de18a7d196669f81bb93c50779bc625"
            ),
            "{rcbar}"
        );
        assert!(rcbar.contains("kind     interval"), "{rcbar}");
        assert!(rcbar.contains("table    XXXI"), "{rcbar}");
        assert!(
            rcbar.contains("range    lambdabar_C = 3.8615926796(12)e-13"),
            "{rcbar}"
        );
        assert!(rcbar.contains("unit     m"), "{rcbar}");
        assert!(
            rcbar.contains(
                "value    [18855433/48828125000000000000, 4826990851/12500000000000000000000]"
            ),
            "{rcbar}"
        );
        assert!(rcbar.contains("rebuild  ok"), "{rcbar}");
        assert!(rcbar.contains("not P3N"), "{rcbar}");
        assert!(!rcbar.contains("receipt"), "{rcbar}");
        assert!(!rcbar.contains("theorem"), "{rcbar}");
        let rcbar_id = constant_node_id(&rcbar);
        assert_eq!(
            rcbar_id.to_hex(),
            "3fd48f3a014e92dae7062468ea0d7df4e4e1e44da7a6a9a6cccea5a5a4ffcc0d",
            "journaling must not change the lambdabar_C constant payload"
        );
        assert_eq!(
            lab.store.get(rcbar_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let rc = lab
            .exec(Command::Constant {
                name: Some("lambda_C".into()),
            })
            .text()
            .to_string();
        assert!(rc.contains("constant  lambda_C  node "), "{rc}");
        assert!(
            rc.contains(
                "hash     6280f2b2f61adf3ae0fa3e65f3b12cfb4982f6601027d98552f541246198c3d8"
            ),
            "{rc}"
        );
        assert!(rc.contains("kind     interval"), "{rc}");
        assert!(rc.contains("table    XXXI"), "{rc}");
        assert!(
            rc.contains("range    lambda_C = 2.42631023867(73)e-12"),
            "{rc}"
        );
        assert!(rc.contains("unit     m"), "{rc}");
        assert!(
            rc.contains(
                "value    [121315511897/50000000000000000000000, 12131551197/5000000000000000000000]"
            ),
            "{rc}"
        );
        assert!(rc.contains("rebuild  ok"), "{rc}");
        assert!(rc.contains("not P3N"), "{rc}");
        assert!(!rc.contains("receipt"), "{rc}");
        assert!(!rc.contains("theorem"), "{rc}");
        let rc_id = constant_node_id(&rc);
        assert_eq!(
            rc_id.to_hex(),
            "4c83c25a7c4f517afc2e092809b141dffc97ae12307b4676cb01da5ab73716e3",
            "journaling must not change the lambda_C constant payload"
        );
        assert_eq!(
            lab.store.get(rc_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let re = lab
            .exec(Command::Constant {
                name: Some("re".into()),
            })
            .text()
            .to_string();
        assert!(re.contains("constant  re  node "), "{re}");
        assert!(
            re.contains(
                "hash     1b8dfc7aa2f90183fd50dab61cf3361f57c3c906e6a221ffa3b2ef17302a38d4"
            ),
            "{re}"
        );
        assert!(re.contains("kind     interval"), "{re}");
        assert!(re.contains("table    XXXI"), "{re}");
        assert!(re.contains("range    re = 2.8179403262(13)e-15"), "{re}");
        assert!(re.contains("unit     m"), "{re}");
        assert!(
            re.contains(
                "value    [28179403249/10000000000000000000000000, 1127176131/400000000000000000000000]"
            ),
            "{re}"
        );
        assert!(re.contains("rebuild  ok"), "{re}");
        assert!(re.contains("not P3N"), "{re}");
        assert!(!re.contains("receipt"), "{re}");
        assert!(!re.contains("theorem"), "{re}");
        let re_id = constant_node_id(&re);
        assert_eq!(
            re_id.to_hex(),
            "bd8a6f5f629ba9df37a0246f420d98c4bbde1d82cdcaaa8d4f9c7796ba239c23",
            "journaling must not change the re constant payload"
        );
        assert_eq!(
            lab.store.get(re_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let mu_e = lab
            .exec(Command::Constant {
                name: Some("mu_e".into()),
            })
            .text()
            .to_string();
        assert!(mu_e.contains("constant  mu_e  node "), "{mu_e}");
        assert!(
            mu_e.contains(
                "hash     e48d03baa8e8b2f62d1ea5c19a7010b583cdfba3f4f9c3d2b55877817d36c9b8"
            ),
            "{mu_e}"
        );
        assert!(mu_e.contains("kind     interval"), "{mu_e}");
        assert!(mu_e.contains("table    XXXI"), "{mu_e}");
        assert!(
            mu_e.contains("range    mu_e = -9.2847647043(28)e-24"),
            "{mu_e}"
        );
        assert!(mu_e.contains("unit     J T^{-1}"), "{mu_e}");
        assert!(
            mu_e.contains(
                "value    [-92847647071/10000000000000000000000000000000000, -18569529403/2000000000000000000000000000000000]"
            ),
            "{mu_e}"
        );
        assert!(mu_e.contains("rebuild  ok"), "{mu_e}");
        assert!(mu_e.contains("not P3N"), "{mu_e}");
        assert!(!mu_e.contains("receipt"), "{mu_e}");
        assert!(!mu_e.contains("theorem"), "{mu_e}");
        let mu_e_id = constant_node_id(&mu_e);
        assert_eq!(
            mu_e_id.to_hex(),
            "5ed9218a55b4eaa8b15614c412c1454a7be21e3a43a317c39275aa68095d5a0d",
            "journaling must not change the mu_e constant payload"
        );
        assert_eq!(
            lab.store.get(mu_e_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let mu_e_mu_b = lab
            .exec(Command::Constant {
                name: Some("mu_e_muB".into()),
            })
            .text()
            .to_string();
        assert!(
            mu_e_mu_b.contains("constant  mu_e_muB  node "),
            "{mu_e_mu_b}"
        );
        assert!(
            mu_e_mu_b.contains(
                "hash     5d4db81093e3f34e08d258ab214de2fb6649d8e7f07cd37c2f5f625a89b52926"
            ),
            "{mu_e_mu_b}"
        );
        assert!(mu_e_mu_b.contains("kind     interval"), "{mu_e_mu_b}");
        assert!(mu_e_mu_b.contains("table    XXXI"), "{mu_e_mu_b}");
        assert!(
            mu_e_mu_b.contains("range    mu_e/muB = -1.00115965218128(18)"),
            "{mu_e_mu_b}"
        );
        assert!(mu_e_mu_b.contains("unit     1"), "{mu_e_mu_b}");
        assert!(
            mu_e_mu_b.contains(
                "value    [-50057982609073/50000000000000, -10011596521811/10000000000000]"
            ),
            "{mu_e_mu_b}"
        );
        assert!(mu_e_mu_b.contains("rebuild  ok"), "{mu_e_mu_b}");
        assert!(mu_e_mu_b.contains("not P3N"), "{mu_e_mu_b}");
        assert!(!mu_e_mu_b.contains("receipt"), "{mu_e_mu_b}");
        assert!(!mu_e_mu_b.contains("theorem"), "{mu_e_mu_b}");
        let mu_e_mu_b_id = constant_node_id(&mu_e_mu_b);
        assert_eq!(
            mu_e_mu_b_id.to_hex(),
            "2297f4ce64d7c1bd8e9ebdfde769d13acfd03f4334913adcc49a57346b1bbcd8",
            "journaling must not change the mu_e_muB constant payload"
        );
        assert_eq!(
            lab.store.get(mu_e_mu_b_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let mu_e_mu_n = lab
            .exec(Command::Constant {
                name: Some("mu_e_muN".into()),
            })
            .text()
            .to_string();
        assert!(
            mu_e_mu_n.contains("constant  mu_e_muN  node "),
            "{mu_e_mu_n}"
        );
        assert!(
            mu_e_mu_n.contains(
                "hash     2a82c539bc621b71977129a26433da37e94f1afd8b38e50c031da0133e2196ca"
            ),
            "{mu_e_mu_n}"
        );
        assert!(mu_e_mu_n.contains("kind     interval"), "{mu_e_mu_n}");
        assert!(mu_e_mu_n.contains("table    XXXI"), "{mu_e_mu_n}");
        assert!(
            mu_e_mu_n.contains("range    mu_e/muN = -1838.28197188(11)"),
            "{mu_e_mu_n}"
        );
        assert!(mu_e_mu_n.contains("unit     1"), "{mu_e_mu_n}");
        assert!(
            mu_e_mu_n.contains("value    [-183828197199/100000000, -183828197177/100000000]"),
            "{mu_e_mu_n}"
        );
        assert!(mu_e_mu_n.contains("rebuild  ok"), "{mu_e_mu_n}");
        assert!(mu_e_mu_n.contains("not P3N"), "{mu_e_mu_n}");
        assert!(!mu_e_mu_n.contains("receipt"), "{mu_e_mu_n}");
        assert!(!mu_e_mu_n.contains("theorem"), "{mu_e_mu_n}");
        let mu_e_mu_n_id = constant_node_id(&mu_e_mu_n);
        assert_eq!(
            mu_e_mu_n_id.to_hex(),
            "fe37bac9de51edecd6c7fbca4718fe5995cbef58e829b91a03f2875e284db9c0",
            "journaling must not change the mu_e_muN constant payload"
        );
        assert_eq!(
            lab.store.get(mu_e_mu_n_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let ae = lab
            .exec(Command::Constant {
                name: Some("ae".into()),
            })
            .text()
            .to_string();
        assert!(ae.contains("constant  ae  node "), "{ae}");
        assert!(
            ae.contains(
                "hash     0fb8666d816320352cbc8e24b896bbb2adc59a085d3b469659d41c6447c82da5"
            ),
            "{ae}"
        );
        assert!(ae.contains("kind     interval"), "{ae}");
        assert!(ae.contains("table    XXXI"), "{ae}");
        assert!(ae.contains("range    ae = 1.15965218128(18)e-3"), "{ae}");
        assert!(ae.contains("unit     1"), "{ae}");
        assert!(
            ae.contains("value    [11596521811/10000000000000, 57982609073/50000000000000]"),
            "{ae}"
        );
        assert!(ae.contains("rebuild  ok"), "{ae}");
        assert!(ae.contains("not P3N"), "{ae}");
        assert!(!ae.contains("receipt"), "{ae}");
        assert!(!ae.contains("theorem"), "{ae}");
        let ae_id = constant_node_id(&ae);
        assert_eq!(
            ae_id.to_hex(),
            "7ca6857af40ac6cf8f3b25125278adbff8732302c4ef9e8b4eb0889087f312bb",
            "journaling must not change the ae constant payload"
        );
        assert_eq!(
            lab.store.get(ae_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let ge = lab
            .exec(Command::Constant {
                name: Some("ge".into()),
            })
            .text()
            .to_string();
        assert!(ge.contains("constant  ge  node "), "{ge}");
        assert!(
            ge.contains(
                "hash     8e1daf3628381ffa7dce3fafc5e65038038eb74b5537cf7adb95702f5d0e0050"
            ),
            "{ge}"
        );
        assert!(ge.contains("kind     interval"), "{ge}");
        assert!(ge.contains("table    XXXI"), "{ge}");
        assert!(ge.contains("range    ge = -2.00231930436256(35)"), "{ge}");
        assert!(ge.contains("unit     1"), "{ge}");
        assert!(
            ge.contains(
                "value    [-200231930436291/100000000000000, -200231930436221/100000000000000]"
            ),
            "{ge}"
        );
        assert!(ge.contains("rebuild  ok"), "{ge}");
        assert!(ge.contains("not P3N"), "{ge}");
        assert!(!ge.contains("receipt"), "{ge}");
        assert!(!ge.contains("theorem"), "{ge}");
        let ge_id = constant_node_id(&ge);
        assert_eq!(
            ge_id.to_hex(),
            "98a79140e37ef1b8e6df0de890bd7dd704c443d879935fdcd62df8aa232540c1",
            "journaling must not change the ge constant payload"
        );
        assert_eq!(
            lab.store.get(ge_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let mu_e_mmu = lab
            .exec(Command::Constant {
                name: Some("mu_e_mmu".into()),
            })
            .text()
            .to_string();
        assert!(mu_e_mmu.contains("constant  mu_e_mmu  node "), "{mu_e_mmu}");
        assert!(
            mu_e_mmu.contains(
                "hash     125652aec9ee47a2db2df2ae81c39cfeb8d9b4037098829e64b78873deb56559"
            ),
            "{mu_e_mmu}"
        );
        assert!(mu_e_mmu.contains("kind     interval"), "{mu_e_mmu}");
        assert!(mu_e_mmu.contains("table    XXXI"), "{mu_e_mmu}");
        assert!(
            mu_e_mmu.contains("range    mu_e/mmu = 206.7669883(46)"),
            "{mu_e_mmu}"
        );
        assert!(mu_e_mmu.contains("unit     1"), "{mu_e_mmu}");
        assert!(
            mu_e_mmu.contains("value    [2067669837/10000000, 2067669929/10000000]"),
            "{mu_e_mmu}"
        );
        assert!(mu_e_mmu.contains("rebuild  ok"), "{mu_e_mmu}");
        assert!(mu_e_mmu.contains("not P3N"), "{mu_e_mmu}");
        assert!(!mu_e_mmu.contains("receipt"), "{mu_e_mmu}");
        assert!(!mu_e_mmu.contains("theorem"), "{mu_e_mmu}");
        let mu_e_mmu_id = constant_node_id(&mu_e_mmu);
        assert_eq!(
            mu_e_mmu_id.to_hex(),
            "12906f3612b3e923097deac331dfecbe0a8b7a03cf9232065aa0a3408a47b1b6",
            "journaling must not change the mu_e_mmu constant payload"
        );
        assert_eq!(
            lab.store.get(mu_e_mmu_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let mu_e_mup = lab
            .exec(Command::Constant {
                name: Some("mu_e_mup".into()),
            })
            .text()
            .to_string();
        assert!(mu_e_mup.contains("constant  mu_e_mup  node "), "{mu_e_mup}");
        assert!(
            mu_e_mup.contains(
                "hash     13a0d90f76fb16f948196cf56fb9d54e90ccc43ad4ff613f27873de735ba7b5b"
            ),
            "{mu_e_mup}"
        );
        assert!(mu_e_mup.contains("kind     interval"), "{mu_e_mup}");
        assert!(mu_e_mup.contains("table    XXXI"), "{mu_e_mup}");
        assert!(
            mu_e_mup.contains("range    mu_e/mup = -658.21068789(20)"),
            "{mu_e_mup}"
        );
        assert!(mu_e_mup.contains("unit     1"), "{mu_e_mup}");
        assert!(
            mu_e_mup.contains("value    [-65821068809/100000000, -65821068769/100000000]"),
            "{mu_e_mup}"
        );
        assert!(mu_e_mup.contains("rebuild  ok"), "{mu_e_mup}");
        assert!(mu_e_mup.contains("not P3N"), "{mu_e_mup}");
        assert!(!mu_e_mup.contains("receipt"), "{mu_e_mup}");
        assert!(!mu_e_mup.contains("theorem"), "{mu_e_mup}");
        let mu_e_mup_id = constant_node_id(&mu_e_mup);
        assert_eq!(
            mu_e_mup_id.to_hex(),
            "c5b40558043871b42fac243c16485e1fec42d13d48622fe406ce1a65b33a8a3e",
            "journaling must not change the mu_e_mup constant payload"
        );
        assert_eq!(
            lab.store.get(mu_e_mup_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let mu_e_mu0p = lab
            .exec(Command::Constant {
                name: Some("mu_e_mu0p".into()),
            })
            .text()
            .to_string();
        assert!(
            mu_e_mu0p.contains("constant  mu_e_mu0p  node "),
            "{mu_e_mu0p}"
        );
        assert!(
            mu_e_mu0p.contains(
                "hash     a3028069b2f88c67432e3c555655438a64bd7b150b2add2b6539e38b3e2df199"
            ),
            "{mu_e_mu0p}"
        );
        assert!(mu_e_mu0p.contains("kind     interval"), "{mu_e_mu0p}");
        assert!(mu_e_mu0p.contains("table    XXXI"), "{mu_e_mu0p}");
        assert!(
            mu_e_mu0p.contains("range    mu_e/mu0p = -658.2275971(72)"),
            "{mu_e_mu0p}"
        );
        assert!(mu_e_mu0p.contains("unit     1"), "{mu_e_mu0p}");
        assert!(
            mu_e_mu0p.contains("value    [-6582276043/10000000, -6582275899/10000000]"),
            "{mu_e_mu0p}"
        );
        assert!(mu_e_mu0p.contains("rebuild  ok"), "{mu_e_mu0p}");
        assert!(mu_e_mu0p.contains("not P3N"), "{mu_e_mu0p}");
        assert!(!mu_e_mu0p.contains("receipt"), "{mu_e_mu0p}");
        assert!(!mu_e_mu0p.contains("theorem"), "{mu_e_mu0p}");
        let mu_e_mu0p_id = constant_node_id(&mu_e_mu0p);
        assert_eq!(
            mu_e_mu0p_id.to_hex(),
            "444c8953846cb45fe6790497b60c5dc1050cb39edc0f55d4f7c122a26e1d2279",
            "journaling must not change the mu_e_mu0p constant payload"
        );
        assert_eq!(
            lab.store.get(mu_e_mu0p_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let mu_e_mun = lab
            .exec(Command::Constant {
                name: Some("mu_e_mun".into()),
            })
            .text()
            .to_string();
        assert!(mu_e_mun.contains("constant  mu_e_mun  node "), "{mu_e_mun}");
        assert!(
            mu_e_mun.contains(
                "hash     9abd0d4216937c89cafceaa4f418b8e8b65a2216df12b3bbc6a1976b1f5c8df2"
            ),
            "{mu_e_mun}"
        );
        assert!(mu_e_mun.contains("kind     interval"), "{mu_e_mun}");
        assert!(mu_e_mun.contains("table    XXXI"), "{mu_e_mun}");
        assert!(
            mu_e_mun.contains("range    mu_e/mun = 960.92050(23)"),
            "{mu_e_mun}"
        );
        assert!(mu_e_mun.contains("unit     1"), "{mu_e_mun}");
        assert!(
            mu_e_mun.contains("value    [96092027/100000, 96092073/100000]"),
            "{mu_e_mun}"
        );
        assert!(mu_e_mun.contains("rebuild  ok"), "{mu_e_mun}");
        assert!(mu_e_mun.contains("not P3N"), "{mu_e_mun}");
        assert!(!mu_e_mun.contains("receipt"), "{mu_e_mun}");
        assert!(!mu_e_mun.contains("theorem"), "{mu_e_mun}");
        let mu_e_mun_id = constant_node_id(&mu_e_mun);
        assert_eq!(
            mu_e_mun_id.to_hex(),
            "aee3c0c42e091e2c5f26b3d9466846186e6d1e70693c4c67deabf9f3a09bc4dc",
            "journaling must not change the mu_e_mun constant payload"
        );
        assert_eq!(
            lab.store.get(mu_e_mun_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let mu_e_mud = lab
            .exec(Command::Constant {
                name: Some("mu_e_mud".into()),
            })
            .text()
            .to_string();
        assert!(mu_e_mud.contains("constant  mu_e_mud  node "), "{mu_e_mud}");
        assert!(
            mu_e_mud.contains(
                "hash     7db59dc912a6c2a301f669f52d7353b27672a07b917e2f8b92b03c1f9acaaa64"
            ),
            "{mu_e_mud}"
        );
        assert!(mu_e_mud.contains("kind     interval"), "{mu_e_mud}");
        assert!(mu_e_mud.contains("table    XXXI"), "{mu_e_mud}");
        assert!(
            mu_e_mud.contains("range    mu_e/mud = -2143.9234915(56)"),
            "{mu_e_mud}"
        );
        assert!(mu_e_mud.contains("unit     1"), "{mu_e_mud}");
        assert!(
            mu_e_mud.contains("value    [-21439234971/10000000, -21439234859/10000000]"),
            "{mu_e_mud}"
        );
        assert!(mu_e_mud.contains("rebuild  ok"), "{mu_e_mud}");
        assert!(mu_e_mud.contains("not P3N"), "{mu_e_mud}");
        assert!(!mu_e_mud.contains("receipt"), "{mu_e_mud}");
        assert!(!mu_e_mud.contains("theorem"), "{mu_e_mud}");
        let mu_e_mud_id = constant_node_id(&mu_e_mud);
        assert_eq!(
            mu_e_mud_id.to_hex(),
            "7a29b2b885a9c1ec2491ac30d0f7408fc89c2d7319e3bb511ab7a3892fef4d33",
            "journaling must not change the mu_e_mud constant payload"
        );
        assert_eq!(
            lab.store.get(mu_e_mud_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let mu_e_mu0h = lab
            .exec(Command::Constant {
                name: Some("mu_e_mu0h".into()),
            })
            .text()
            .to_string();
        assert!(
            mu_e_mu0h.contains("constant  mu_e_mu0h  node "),
            "{mu_e_mu0h}"
        );
        assert!(
            mu_e_mu0h.contains(
                "hash     3e3e29f0ac633705b8d8467b80b0cd229b07f4d7ba44fe32b84730261c576a9b"
            ),
            "{mu_e_mu0h}"
        );
        assert!(mu_e_mu0h.contains("kind     interval"), "{mu_e_mu0h}");
        assert!(mu_e_mu0h.contains("table    XXXI"), "{mu_e_mu0h}");
        assert!(
            mu_e_mu0h.contains("range    mu_e/mu0h = 864.058257(10)"),
            "{mu_e_mu0h}"
        );
        assert!(mu_e_mu0h.contains("unit     1"), "{mu_e_mu0h}");
        assert!(
            mu_e_mu0h.contains("value    [864058247/1000000, 864058267/1000000]"),
            "{mu_e_mu0h}"
        );
        assert!(mu_e_mu0h.contains("rebuild  ok"), "{mu_e_mu0h}");
        assert!(mu_e_mu0h.contains("not P3N"), "{mu_e_mu0h}");
        assert!(!mu_e_mu0h.contains("receipt"), "{mu_e_mu0h}");
        assert!(!mu_e_mu0h.contains("theorem"), "{mu_e_mu0h}");
        let mu_e_mu0h_id = constant_node_id(&mu_e_mu0h);
        assert_eq!(
            mu_e_mu0h_id.to_hex(),
            "f6b2ab92d421f6139a457f76b4898616573c38cef1e29d29941e0eb41c795e30",
            "journaling must not change the mu_e_mu0h constant payload"
        );
        assert_eq!(
            lab.store.get(mu_e_mu0h_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let m_mu = lab
            .exec(Command::Constant {
                name: Some("m_mu".into()),
            })
            .text()
            .to_string();
        assert!(m_mu.contains("constant  m_mu  node "), "{m_mu}");
        assert!(
            m_mu.contains(
                "hash     b1e0e67d46205c048709815e1215184c1b77afbcb0f197099085fbfc7d3bb016"
            ),
            "{m_mu}"
        );
        assert!(m_mu.contains("kind     interval"), "{m_mu}");
        assert!(m_mu.contains("table    XXXI"), "{m_mu}");
        assert!(
            m_mu.contains("range    mmu = 1.883531627(42)e-28"),
            "{m_mu}"
        );
        assert!(m_mu.contains("unit     kg"), "{m_mu}");
        assert!(m_mu.contains("rebuild  ok"), "{m_mu}");
        assert!(m_mu.contains("not P3N"), "{m_mu}");
        assert!(!m_mu.contains("receipt"), "{m_mu}");
        assert!(!m_mu.contains("theorem"), "{m_mu}");
        let m_mu_id = constant_node_id(&m_mu);
        assert_eq!(
            m_mu_id.to_hex(),
            "3cf58d635727710c293a539a68c0bce2aeadc9d41fa8a8dd43c238dfa58ad890",
            "journaling must not change the m_mu constant payload"
        );
        assert_eq!(
            lab.store.get(m_mu_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let m_mu_u = lab
            .exec(Command::Constant {
                name: Some("m_mu_u".into()),
            })
            .text()
            .to_string();
        assert!(m_mu_u.contains("constant  m_mu_u  node "), "{m_mu_u}");
        assert!(
            m_mu_u.contains(
                "hash     ced234733b80023dd6d8687ce99efc8473defe15f63b74f3ecde00ece485515d"
            ),
            "{m_mu_u}"
        );
        assert!(m_mu_u.contains("kind     interval"), "{m_mu_u}");
        assert!(m_mu_u.contains("table    XXXI"), "{m_mu_u}");
        assert!(
            m_mu_u.contains("range    mmu_u = 0.1134289259(25)"),
            "{m_mu_u}"
        );
        assert!(m_mu_u.contains("unit     u"), "{m_mu_u}");
        assert!(
            m_mu_u.contains("value    [567144617/5000000000, 283572321/2500000000]"),
            "{m_mu_u}"
        );
        assert!(m_mu_u.contains("rebuild  ok"), "{m_mu_u}");
        assert!(m_mu_u.contains("not P3N"), "{m_mu_u}");
        assert!(!m_mu_u.contains("receipt"), "{m_mu_u}");
        assert!(!m_mu_u.contains("theorem"), "{m_mu_u}");
        let m_mu_u_id = constant_node_id(&m_mu_u);
        assert_eq!(
            m_mu_u_id.to_hex(),
            "d9dd36e1db3fe1aa782b3cfb99db87ba10250a4f0d945607d0cfa0ad6b163b78",
            "journaling must not change the m_mu_u constant payload"
        );
        assert_eq!(
            lab.store.get(m_mu_u_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let m_mu_c2 = lab
            .exec(Command::Constant {
                name: Some("m_mu_c2".into()),
            })
            .text()
            .to_string();
        assert!(m_mu_c2.contains("constant  m_mu_c2  node "), "{m_mu_c2}");
        assert!(
            m_mu_c2.contains(
                "hash     d83a5072b8cb4fe869a2aa076aff9c4cd0d8f9f613a41eef52117124acde5854"
            ),
            "{m_mu_c2}"
        );
        assert!(m_mu_c2.contains("kind     interval"), "{m_mu_c2}");
        assert!(m_mu_c2.contains("table    XXXI"), "{m_mu_c2}");
        assert!(
            m_mu_c2.contains("range    mmu_c2 = 1.692833804(38)e-11"),
            "{m_mu_c2}"
        );
        assert!(m_mu_c2.contains("unit     J"), "{m_mu_c2}");
        assert!(
            m_mu_c2.contains(
                "value    [846416883/50000000000000000000, 846416921/50000000000000000000]"
            ),
            "{m_mu_c2}"
        );
        assert!(m_mu_c2.contains("rebuild  ok"), "{m_mu_c2}");
        assert!(m_mu_c2.contains("not P3N"), "{m_mu_c2}");
        assert!(!m_mu_c2.contains("receipt"), "{m_mu_c2}");
        assert!(!m_mu_c2.contains("theorem"), "{m_mu_c2}");
        let m_mu_c2_id = constant_node_id(&m_mu_c2);
        assert_eq!(
            m_mu_c2_id.to_hex(),
            "a451ddc9cfd85f74fc32ddaa156c25b2d60003cac9c3a2c7c60b17d3c2a2544a",
            "journaling must not change the m_mu_c2 constant payload"
        );
        assert_eq!(
            lab.store.get(m_mu_c2_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let m_mu_c2_mev = lab
            .exec(Command::Constant {
                name: Some("m_mu_c2_MeV".into()),
            })
            .text()
            .to_string();
        assert!(
            m_mu_c2_mev.contains("constant  m_mu_c2_MeV  node "),
            "{m_mu_c2_mev}"
        );
        assert!(
            m_mu_c2_mev.contains(
                "hash     292b0524e0f1a160403fe1a2a4998cd4c2690f5d3b344a5f8ba31e9248be0416"
            ),
            "{m_mu_c2_mev}"
        );
        assert!(m_mu_c2_mev.contains("kind     interval"), "{m_mu_c2_mev}");
        assert!(m_mu_c2_mev.contains("table    XXXI"), "{m_mu_c2_mev}");
        assert!(
            m_mu_c2_mev.contains("range    mmu_c2_MeV = 105.6583755(23)"),
            "{m_mu_c2_mev}"
        );
        assert!(m_mu_c2_mev.contains("unit     MeV"), "{m_mu_c2_mev}");
        assert!(
            m_mu_c2_mev.contains("value    [264145933/2500000, 528291889/5000000]"),
            "{m_mu_c2_mev}"
        );
        assert!(m_mu_c2_mev.contains("rebuild  ok"), "{m_mu_c2_mev}");
        assert!(m_mu_c2_mev.contains("not P3N"), "{m_mu_c2_mev}");
        assert!(!m_mu_c2_mev.contains("receipt"), "{m_mu_c2_mev}");
        assert!(!m_mu_c2_mev.contains("theorem"), "{m_mu_c2_mev}");
        let m_mu_c2_mev_id = constant_node_id(&m_mu_c2_mev);
        assert_eq!(
            m_mu_c2_mev_id.to_hex(),
            "b0d03e5dcc8f9174cfebf4d35d2ad0ab0836c6cde6d615cbdc21dd4e720d5dd4",
            "journaling must not change the m_mu_c2_MeV constant payload"
        );
        assert_eq!(
            lab.store.get(m_mu_c2_mev_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let mmu_me = lab
            .exec(Command::Constant {
                name: Some("mmu_me".into()),
            })
            .text()
            .to_string();
        assert!(mmu_me.contains("constant  mmu_me  node "), "{mmu_me}");
        assert!(
            mmu_me.contains(
                "hash     0ac70815382ab74fd46513f298dd351685fcc54ab0e64b6fc00b64b4fccc426f"
            ),
            "{mmu_me}"
        );
        assert!(mmu_me.contains("kind     interval"), "{mmu_me}");
        assert!(mmu_me.contains("table    XXXI"), "{mmu_me}");
        assert!(
            mmu_me.contains("range    mmu/me = 206.7682830(46)"),
            "{mmu_me}"
        );
        assert!(mmu_me.contains("unit     1"), "{mmu_me}");
        assert!(
            mmu_me.contains("value    [64615087/312500, 516920719/2500000]"),
            "{mmu_me}"
        );
        assert!(mmu_me.contains("rebuild  ok"), "{mmu_me}");
        assert!(mmu_me.contains("not P3N"), "{mmu_me}");
        assert!(!mmu_me.contains("receipt"), "{mmu_me}");
        assert!(!mmu_me.contains("theorem"), "{mmu_me}");
        let mmu_me_id = constant_node_id(&mmu_me);
        assert_eq!(
            mmu_me_id.to_hex(),
            "6c0240f5d6812b9ed27e687e73892c43959dc1cac8c2697cd116f4c116f06c74",
            "journaling must not change the mmu_me constant payload"
        );
        assert_eq!(
            lab.store.get(mmu_me_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let mmu_mp = lab
            .exec(Command::Constant {
                name: Some("mmu_mp".into()),
            })
            .text()
            .to_string();
        assert!(mmu_mp.contains("constant  mmu_mp  node "), "{mmu_mp}");
        assert!(
            mmu_mp.contains(
                "hash     1527aa21236682ad99206cf1ef6b6267d7432a5a1975bcc2315af9a510e147d2"
            ),
            "{mmu_mp}"
        );
        assert!(mmu_mp.contains("kind     interval"), "{mmu_mp}");
        assert!(mmu_mp.contains("table    XXXI"), "{mmu_mp}");
        assert!(
            mmu_mp.contains("range    mmu/mp = 0.1126095264(25)"),
            "{mmu_mp}"
        );
        assert!(mmu_mp.contains("unit     1"), "{mmu_mp}");
        assert!(
            mmu_mp.contains("value    [1126095239/10000000000, 1126095289/10000000000]"),
            "{mmu_mp}"
        );
        assert!(mmu_mp.contains("rebuild  ok"), "{mmu_mp}");
        assert!(mmu_mp.contains("not P3N"), "{mmu_mp}");
        assert!(!mmu_mp.contains("receipt"), "{mmu_mp}");
        assert!(!mmu_mp.contains("theorem"), "{mmu_mp}");
        let mmu_mp_id = constant_node_id(&mmu_mp);
        assert_eq!(
            mmu_mp_id.to_hex(),
            "71390580bc3deae9e30e53f10b832a03e64680879ce3f0f076695e3f41024915",
            "journaling must not change the mmu_mp constant payload"
        );
        assert_eq!(
            lab.store.get(mmu_mp_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let mmu_mn = lab
            .exec(Command::Constant {
                name: Some("mmu_mn".into()),
            })
            .text()
            .to_string();
        assert!(mmu_mn.contains("constant  mmu_mn  node "), "{mmu_mn}");
        assert!(
            mmu_mn.contains(
                "hash     f8a9dfb53e84c4a592143e9d17e9e04884b69cc9b2b378dc2a7c099c4d442835"
            ),
            "{mmu_mn}"
        );
        assert!(mmu_mn.contains("kind     interval"), "{mmu_mn}");
        assert!(mmu_mn.contains("table    XXXI"), "{mmu_mn}");
        assert!(
            mmu_mn.contains("range    mmu/mn = 0.1124545170(25)"),
            "{mmu_mn}"
        );
        assert!(mmu_mn.contains("unit     1"), "{mmu_mn}");
        assert!(
            mmu_mn.contains("value    [224909029/2000000000, 224909039/2000000000]"),
            "{mmu_mn}"
        );
        assert!(mmu_mn.contains("rebuild  ok"), "{mmu_mn}");
        assert!(mmu_mn.contains("not P3N"), "{mmu_mn}");
        assert!(!mmu_mn.contains("receipt"), "{mmu_mn}");
        assert!(!mmu_mn.contains("theorem"), "{mmu_mn}");
        let mmu_mn_id = constant_node_id(&mmu_mn);
        assert_eq!(
            mmu_mn_id.to_hex(),
            "4f08dabdf90b0a433f93c5cd653e9d3c5f2f7b02145130049391199780b02e63",
            "journaling must not change the mmu_mn constant payload"
        );
        assert_eq!(
            lab.store.get(mmu_mn_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let m_mu_molar = lab
            .exec(Command::Constant {
                name: Some("M_mu".into()),
            })
            .text()
            .to_string();
        assert!(m_mu_molar.contains("constant  M_mu  node "), "{m_mu_molar}");
        assert!(
            m_mu_molar.contains(
                "hash     b53efc5e339708317e98c92c02ae506bf5b90c6d847e586d716d1631d902c81a"
            ),
            "{m_mu_molar}"
        );
        assert!(m_mu_molar.contains("kind     interval"), "{m_mu_molar}");
        assert!(m_mu_molar.contains("table    XXXI"), "{m_mu_molar}");
        assert!(
            m_mu_molar.contains("range    Mmu = 1.134289259(25)e-4"),
            "{m_mu_molar}"
        );
        assert!(m_mu_molar.contains("unit     kg mol^{-1}"), "{m_mu_molar}");
        assert!(
            m_mu_molar.contains("value    [567144617/5000000000000, 283572321/2500000000000]"),
            "{m_mu_molar}"
        );
        assert!(m_mu_molar.contains("rebuild  ok"), "{m_mu_molar}");
        assert!(m_mu_molar.contains("not P3N"), "{m_mu_molar}");
        assert!(!m_mu_molar.contains("receipt"), "{m_mu_molar}");
        assert!(!m_mu_molar.contains("theorem"), "{m_mu_molar}");
        let m_mu_molar_id = constant_node_id(&m_mu_molar);
        assert_eq!(
            m_mu_molar_id.to_hex(),
            "4fb4263b7231f6a7d4fe1a73eba8b05adeaf94a12ccd7caccc4eb2ff4d0c6a07",
            "journaling must not change the M_mu constant payload"
        );
        assert_eq!(
            lab.store.get(m_mu_molar_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let lambda_c_mu = lab
            .exec(Command::Constant {
                name: Some("lambda_C_mu".into()),
            })
            .text()
            .to_string();
        assert!(
            lambda_c_mu.contains("constant  lambda_C_mu  node "),
            "{lambda_c_mu}"
        );
        assert!(
            lambda_c_mu.contains(
                "hash     6fb48517f2b436bf1ede156c0dd4505692db4e7afe3e5d6f7ed2bfbfdc4198d9"
            ),
            "{lambda_c_mu}"
        );
        assert!(lambda_c_mu.contains("kind     interval"), "{lambda_c_mu}");
        assert!(lambda_c_mu.contains("table    XXXI"), "{lambda_c_mu}");
        assert!(
            lambda_c_mu.contains("range    lambda_C_mu = 1.173444110(26)e-14"),
            "{lambda_c_mu}"
        );
        assert!(lambda_c_mu.contains("unit     m"), "{lambda_c_mu}");
        assert!(
            lambda_c_mu.contains(
                "value    [293361021/25000000000000000000000, 146680517/12500000000000000000000]"
            ),
            "{lambda_c_mu}"
        );
        assert!(lambda_c_mu.contains("rebuild  ok"), "{lambda_c_mu}");
        assert!(lambda_c_mu.contains("not P3N"), "{lambda_c_mu}");
        assert!(!lambda_c_mu.contains("receipt"), "{lambda_c_mu}");
        assert!(!lambda_c_mu.contains("theorem"), "{lambda_c_mu}");
        let lambda_c_mu_id = constant_node_id(&lambda_c_mu);
        assert_eq!(
            lambda_c_mu_id.to_hex(),
            "7927f45a9a1a6944fde9bc270c82a4a5014224d39fbfbf9802d3b5505556d4ec",
            "journaling must not change the lambda_C_mu constant payload"
        );
        assert_eq!(
            lab.store.get(lambda_c_mu_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let mu_mu = lab
            .exec(Command::Constant {
                name: Some("mu_mu".into()),
            })
            .text()
            .to_string();
        assert!(mu_mu.contains("constant  mu_mu  node "), "{mu_mu}");
        assert!(
            mu_mu.contains(
                "hash     3344549ca18b2db388cfff366cc63079f3d3b0b094cac6de12e318fe8531c3e0"
            ),
            "{mu_mu}"
        );
        assert!(mu_mu.contains("kind     interval"), "{mu_mu}");
        assert!(mu_mu.contains("table    XXXI"), "{mu_mu}");
        assert!(
            mu_mu.contains("range    mu_mu = -4.49044830(10)e-26"),
            "{mu_mu}"
        );
        assert!(mu_mu.contains("unit     J T^{-1}"), "{mu_mu}");
        assert!(
            mu_mu.contains("value    [-11226121/250000000000000000000000000000000, -22452241/500000000000000000000000000000000]"),
            "{mu_mu}"
        );
        assert!(mu_mu.contains("rebuild  ok"), "{mu_mu}");
        assert!(mu_mu.contains("not P3N"), "{mu_mu}");
        assert!(!mu_mu.contains("receipt"), "{mu_mu}");
        assert!(!mu_mu.contains("theorem"), "{mu_mu}");
        let mu_mu_id = constant_node_id(&mu_mu);
        assert_eq!(
            mu_mu_id.to_hex(),
            "dbdd3710096e4ae7679dc5238012cec06c3ecc8242e3727978c5c15023b4abb8",
            "journaling must not change the mu_mu constant payload"
        );
        assert_eq!(
            lab.store.get(mu_mu_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let mu_mu_mu_b = lab
            .exec(Command::Constant {
                name: Some("mu_mu_muB".into()),
            })
            .text()
            .to_string();
        assert!(
            mu_mu_mu_b.contains("constant  mu_mu_muB  node "),
            "{mu_mu_mu_b}"
        );
        assert!(
            mu_mu_mu_b.contains(
                "hash     5fa244938a528feff7867ea9ae972d76da59930a932f2a5ac9fe6ef52762c591"
            ),
            "{mu_mu_mu_b}"
        );
        assert!(mu_mu_mu_b.contains("kind     interval"), "{mu_mu_mu_b}");
        assert!(mu_mu_mu_b.contains("table    XXXI"), "{mu_mu_mu_b}");
        assert!(
            mu_mu_mu_b.contains("range    mu_mu/muB = -4.84197047(11)e-3"),
            "{mu_mu_mu_b}"
        );
        assert!(mu_mu_mu_b.contains("unit     1"), "{mu_mu_mu_b}");
        assert!(
            mu_mu_mu_b.contains("value    [-242098529/50000000000, -121049259/25000000000]"),
            "{mu_mu_mu_b}"
        );
        assert!(mu_mu_mu_b.contains("rebuild  ok"), "{mu_mu_mu_b}");
        assert!(mu_mu_mu_b.contains("not P3N"), "{mu_mu_mu_b}");
        assert!(!mu_mu_mu_b.contains("receipt"), "{mu_mu_mu_b}");
        assert!(!mu_mu_mu_b.contains("theorem"), "{mu_mu_mu_b}");
        let mu_mu_mu_b_id = constant_node_id(&mu_mu_mu_b);
        assert_eq!(
            mu_mu_mu_b_id.to_hex(),
            "598222a0188296e5619983efb3343cb0841a482f073854780655991ddbbc57e7",
            "journaling must not change the mu_mu_muB constant payload"
        );
        assert_eq!(
            lab.store.get(mu_mu_mu_b_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let mu_mu_mu_n = lab
            .exec(Command::Constant {
                name: Some("mu_mu_muN".into()),
            })
            .text()
            .to_string();
        assert!(
            mu_mu_mu_n.contains("constant  mu_mu_muN  node "),
            "{mu_mu_mu_n}"
        );
        assert!(
            mu_mu_mu_n.contains(
                "hash     52a97de9669b20480e5729de915cef56ba841da392185b3e5893b10c496ed16b"
            ),
            "{mu_mu_mu_n}"
        );
        assert!(mu_mu_mu_n.contains("kind     interval"), "{mu_mu_mu_n}");
        assert!(mu_mu_mu_n.contains("table    XXXI"), "{mu_mu_mu_n}");
        assert!(
            mu_mu_mu_n.contains("range    mu_mu/muN = -8.89059703(20)"),
            "{mu_mu_mu_n}"
        );
        assert!(mu_mu_mu_n.contains("unit     1"), "{mu_mu_mu_n}");
        assert!(
            mu_mu_mu_n.contains("value    [-889059723/100000000, -889059683/100000000]"),
            "{mu_mu_mu_n}"
        );
        assert!(mu_mu_mu_n.contains("rebuild  ok"), "{mu_mu_mu_n}");
        assert!(mu_mu_mu_n.contains("not P3N"), "{mu_mu_mu_n}");
        assert!(!mu_mu_mu_n.contains("receipt"), "{mu_mu_mu_n}");
        assert!(!mu_mu_mu_n.contains("theorem"), "{mu_mu_mu_n}");
        let mu_mu_mu_n_id = constant_node_id(&mu_mu_mu_n);
        assert_eq!(
            mu_mu_mu_n_id.to_hex(),
            "a2ee12b91517cfe4a6ab8f310e7361b9adb4877f0f69643d9e7f31d983777854",
            "journaling must not change the mu_mu_muN constant payload"
        );
        assert_eq!(
            lab.store.get(mu_mu_mu_n_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let amu = lab
            .exec(Command::Constant {
                name: Some("amu".into()),
            })
            .text()
            .to_string();
        assert!(amu.contains("constant  amu  node "), "{amu}");
        assert!(
            amu.contains(
                "hash     972c93982e6cd84f054db85605b9e7d106d124bd52bac104bb447d788cdc64c4"
            ),
            "{amu}"
        );
        assert!(amu.contains("kind     interval"), "{amu}");
        assert!(amu.contains("table    XXXI"), "{amu}");
        assert!(amu.contains("range    amu = 1.16592089(63)e-3"), "{amu}");
        assert!(amu.contains("unit     1"), "{amu}");
        assert!(
            amu.contains("value    [58296013/50000000000, 14574019/12500000000]"),
            "{amu}"
        );
        assert!(amu.contains("rebuild  ok"), "{amu}");
        assert!(amu.contains("not P3N"), "{amu}");
        assert!(!amu.contains("receipt"), "{amu}");
        assert!(!amu.contains("theorem"), "{amu}");
        let amu_id = constant_node_id(&amu);
        assert_eq!(
            amu_id.to_hex(),
            "8eeb79c20e0bd0766d4afb682a885a006a867378e21dced5d1b485c9a52b06ad",
            "journaling must not change the amu constant payload"
        );
        assert_eq!(
            lab.store.get(amu_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let mp = lab
            .exec(Command::Constant {
                name: Some("m_p".into()),
            })
            .text()
            .to_string();
        assert!(mp.contains("constant  m_p  node "), "{mp}");
        assert!(
            mp.contains(
                "hash     ffd371a69f7ec3d9bac8dcf57e0126709fd3f63c35561e717d9886d2fb1f88c8"
            ),
            "{mp}"
        );
        assert!(mp.contains("kind     interval"), "{mp}");
        assert!(mp.contains("table    XXXI"), "{mp}");
        assert!(mp.contains("range    mp = 1.67262192369(51)e-27"), "{mp}");
        assert!(mp.contains("unit     kg"), "{mp}");
        assert!(mp.contains("rebuild  ok"), "{mp}");
        assert!(mp.contains("not P3N"), "{mp}");
        assert!(!mp.contains("receipt"), "{mp}");
        assert!(!mp.contains("theorem"), "{mp}");
        let mp_id = constant_node_id(&mp);
        assert_eq!(
            mp_id.to_hex(),
            "3fcee64bc46c5e13e69bc0d822e66e701b793910803445e5d4689ead316028a0",
            "journaling must not change the m_p constant payload"
        );
        assert_eq!(
            lab.store.get(mp_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let mu0 = lab
            .exec(Command::Constant {
                name: Some("mu0".into()),
            })
            .text()
            .to_string();
        assert!(mu0.contains("constant  mu0  node "), "{mu0}");
        assert!(
            mu0.contains(
                "hash     fa1264a6ce514520c9c2d9131fee2c71cacd4ce5fe615ea4dd424fd23de35cd7"
            ),
            "{mu0}"
        );
        assert!(mu0.contains("kind     interval"), "{mu0}");
        assert!(mu0.contains("table    XXXI"), "{mu0}");
        assert!(mu0.contains("range    mu0 = 1.25663706212(19)e-6"), "{mu0}");
        assert!(mu0.contains("unit     N A^{-2}"), "{mu0}");
        assert!(mu0.contains("rebuild  ok"), "{mu0}");
        assert!(mu0.contains("not P3N"), "{mu0}");
        assert!(!mu0.contains("receipt"), "{mu0}");
        assert!(!mu0.contains("theorem"), "{mu0}");
        let mu0_id = constant_node_id(&mu0);
        assert_eq!(
            mu0_id.to_hex(),
            "2b652a4d95e792481d77b5358af0426e6425619a32e86965bd6f19cbec20ae9e",
            "journaling must not change the mu0 constant payload"
        );
        assert_eq!(
            lab.store.get(mu0_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let eps = lab
            .exec(Command::Constant {
                name: Some("epsilon0".into()),
            })
            .text()
            .to_string();
        assert!(eps.contains("constant  epsilon0  node "), "{eps}");
        assert!(
            eps.contains(
                "hash     fadaf2a47a8161ba2727a4c2ff6b842f7c9e6add2edd67cd5496a7a753f22d80"
            ),
            "{eps}"
        );
        assert!(eps.contains("kind     interval"), "{eps}");
        assert!(eps.contains("table    XXXI"), "{eps}");
        assert!(
            eps.contains("range    epsilon0 = 8.8541878128(13)e-12"),
            "{eps}"
        );
        assert!(eps.contains("unit     F m^{-1}"), "{eps}");
        assert!(eps.contains("rebuild  ok"), "{eps}");
        assert!(eps.contains("not P3N"), "{eps}");
        assert!(!eps.contains("receipt"), "{eps}");
        assert!(!eps.contains("theorem"), "{eps}");
        let eps_id = constant_node_id(&eps);
        assert_eq!(
            eps_id.to_hex(),
            "0b2262eee35047174ebb10962a4aaf06481a8e18e9b44a3873a129f91873a8b7",
            "journaling must not change the epsilon0 constant payload"
        );
        assert_eq!(
            lab.store.get(eps_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let z0 = lab
            .exec(Command::Constant {
                name: Some("Z0".into()),
            })
            .text()
            .to_string();
        assert!(z0.contains("constant  Z0  node "), "{z0}");
        assert!(
            z0.contains(
                "hash     6f72c1c5833dc722ac6fb5223f982879499ff412157c6e6c9851d77088991316"
            ),
            "{z0}"
        );
        assert!(z0.contains("kind     interval"), "{z0}");
        assert!(z0.contains("table    XXXI"), "{z0}");
        assert!(z0.contains("range    Z0 = 376.730313668(57)"), "{z0}");
        assert!(z0.contains("unit     ohm"), "{z0}");
        assert!(z0.contains("rebuild  ok"), "{z0}");
        assert!(z0.contains("not P3N"), "{z0}");
        assert!(!z0.contains("receipt"), "{z0}");
        assert!(!z0.contains("theorem"), "{z0}");
        let z0_id = constant_node_id(&z0);
        assert_eq!(
            z0_id.to_hex(),
            "e7ebeaa7b1b18ebed7aa269cd7b4b322842d5f674106efb8b1ad1cda4d4ac77e",
            "journaling must not change the Z0 constant payload"
        );
        assert_eq!(
            lab.store.get(z0_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let c = lab
            .exec(Command::Constant {
                name: Some("c".into()),
            })
            .text()
            .to_string();
        assert!(
            c.contains("hash     691eb73ea444f6d10fb223b999a1b37c0b67da92d51e43ca8bd8a6561785a3c1"),
            "{c}"
        );
        assert!(c.contains("kind     ratio"), "{c}");
        assert!(c.contains("table    1"), "{c}");

        let h = lab
            .exec(Command::Constant {
                name: Some("h".into()),
            })
            .text()
            .to_string();
        assert!(
            h.contains("hash     50a96a8715769547a90cba69b0775d8892d79f2fa32465ad13a6d73b2d111eef"),
            "{h}"
        );
        assert!(h.contains("kind     sci-exact"), "{h}");
        assert!(h.contains("662607015e-42"), "{h}");

        let au = lab
            .exec(Command::Constant {
                name: Some("au".into()),
            })
            .text()
            .to_string();
        assert!(au.contains("constant  au  node "), "{au}");
        assert!(
            au.contains(
                "hash     d3441603d75b565016c25cc955783fbb76b4050ee22befcef0c0e3896e873a0b"
            ),
            "{au}"
        );
        assert!(au.contains("kind     ratio"), "{au}");
        assert!(au.contains("table    8"), "{au}");
        assert!(au.contains("149597870700"), "{au}");
        assert!(au.contains("rebuild  ok"), "{au}");
        assert!(!au.contains("receipt"), "{au}");
        assert!(!au.contains("theorem"), "{au}");

        let ev = lab
            .exec(Command::Constant {
                name: Some("eV".into()),
            })
            .text()
            .to_string();
        assert!(ev.contains("constant  eV  node "), "{ev}");
        assert!(
            ev.contains(
                "hash     d5514de9cbef3f6990067899529d34f20b4349ca3b20ba18c9a5932c8c6b6c0f"
            ),
            "{ev}"
        );
        assert!(ev.contains("kind     ratio"), "{ev}");
        assert!(ev.contains("table    8"), "{ev}");
        assert!(ev.contains("release  si-2019-codata-2018"), "{ev}");
        assert!(ev.contains("801088317/"), "{ev}");
        assert!(ev.contains("rebuild  ok"), "{ev}");
        assert!(!ev.contains("receipt"), "{ev}");
        assert!(!ev.contains("theorem"), "{ev}");

        let gm = lab
            .exec(Command::Constant {
                name: Some("GM_sun".into()),
            })
            .text()
            .to_string();
        assert!(gm.contains("constant  GM_sun  node "), "{gm}");
        assert!(
            gm.contains(
                "hash     636001001c4ed9cd5e6661241e5ad5e5db09c8419a3fe79790143162b7af3a58"
            ),
            "{gm}"
        );
        assert!(gm.contains("kind     ratio"), "{gm}");
        assert!(gm.contains("table    1"), "{gm}");
        assert!(gm.contains("release  iau-2015"), "{gm}");
        assert!(gm.contains("132712440000000000000"), "{gm}");
        assert!(gm.contains("rebuild  ok"), "{gm}");
        assert!(!gm.contains("receipt"), "{gm}");
        assert!(!gm.contains("theorem"), "{gm}");

        let r_sun = lab
            .exec(Command::Constant {
                name: Some("R_sun".into()),
            })
            .text()
            .to_string();
        assert!(r_sun.contains("constant  R_sun  node "), "{r_sun}");
        assert!(
            r_sun.contains(
                "hash     cb7f91f2d0663d2d8ff8b0e3009f6e0772a126220d04ed658fc793db7e5cc6b4"
            ),
            "{r_sun}"
        );
        assert!(r_sun.contains("kind     ratio"), "{r_sun}");
        assert!(r_sun.contains("table    1"), "{r_sun}");
        assert!(r_sun.contains("release  iau-2015"), "{r_sun}");
        assert!(r_sun.contains("695700000"), "{r_sun}");
        assert!(r_sun.contains("rebuild  ok"), "{r_sun}");
        assert!(!r_sun.contains("receipt"), "{r_sun}");
        assert!(!r_sun.contains("theorem"), "{r_sun}");

        let l_sun = lab
            .exec(Command::Constant {
                name: Some("L_sun".into()),
            })
            .text()
            .to_string();
        assert!(l_sun.contains("constant  L_sun  node "), "{l_sun}");
        assert!(
            l_sun.contains(
                "hash     444f85fba501ddec8fb08ba403c1b869cc78a2284df5466a56a617043807bbc4"
            ),
            "{l_sun}"
        );
        assert!(l_sun.contains("kind     ratio"), "{l_sun}");
        assert!(l_sun.contains("table    1"), "{l_sun}");
        assert!(l_sun.contains("release  iau-2015"), "{l_sun}");
        assert!(l_sun.contains("382800000000000000000000000"), "{l_sun}");
        assert!(l_sun.contains("rebuild  ok"), "{l_sun}");
        assert!(!l_sun.contains("receipt"), "{l_sun}");
        assert!(!l_sun.contains("theorem"), "{l_sun}");

        let unknown = lab.exec(Command::Constant {
            name: Some("hbar".into()),
        });
        assert_eq!(unknown.exit_code(), 1, "{}", unknown.text());
        assert!(
            unknown.text().contains("unknown constant 'hbar'"),
            "{}",
            unknown.text()
        );

        let unknown_eh = lab.exec(Command::Constant {
            name: Some("E_h".into()),
        });
        assert_eq!(unknown_eh.exit_code(), 1, "{}", unknown_eh.text());
        assert!(
            unknown_eh.text().contains("unknown constant 'E_h'"),
            "{}",
            unknown_eh.text()
        );

        let unknown_hc = lab.exec(Command::Constant {
            name: Some("hcRinf_eV".into()),
        });
        assert_eq!(unknown_hc.exit_code(), 1, "{}", unknown_hc.text());
        assert!(
            unknown_hc.text().contains("unknown constant 'hcRinf_eV'"),
            "{}",
            unknown_hc.text()
        );

        let unknown_ratio = lab.exec(Command::Constant {
            name: Some("sigma_e".into()),
        });
        assert_eq!(unknown_ratio.exit_code(), 1, "{}", unknown_ratio.text());
        assert!(
            unknown_ratio.text().contains("unknown constant 'sigma_e'"),
            "{}",
            unknown_ratio.text()
        );

        let unknown_ge = lab.exec(Command::Constant {
            name: Some("g_e".into()),
        });
        assert_eq!(unknown_ge.exit_code(), 1, "{}", unknown_ge.text());
        assert!(
            unknown_ge.text().contains("unknown constant 'g_e'"),
            "{}",
            unknown_ge.text()
        );

        let unknown_mue_mup = lab.exec(Command::Constant {
            name: Some("mue_mup".into()),
        });
        assert_eq!(unknown_mue_mup.exit_code(), 1, "{}", unknown_mue_mup.text());
        assert!(
            unknown_mue_mup
                .text()
                .contains("unknown constant 'mue_mup'"),
            "{}",
            unknown_mue_mup.text()
        );

        let unknown_mue_mu0p = lab.exec(Command::Constant {
            name: Some("mue_mu0p".into()),
        });
        assert_eq!(
            unknown_mue_mu0p.exit_code(),
            1,
            "{}",
            unknown_mue_mu0p.text()
        );
        assert!(
            unknown_mue_mu0p
                .text()
                .contains("unknown constant 'mue_mu0p'"),
            "{}",
            unknown_mue_mu0p.text()
        );

        let unknown_mue_mun = lab.exec(Command::Constant {
            name: Some("mue_mun".into()),
        });
        assert_eq!(unknown_mue_mun.exit_code(), 1, "{}", unknown_mue_mun.text());
        assert!(
            unknown_mue_mun
                .text()
                .contains("unknown constant 'mue_mun'"),
            "{}",
            unknown_mue_mun.text()
        );

        let unknown_mue_mud = lab.exec(Command::Constant {
            name: Some("mue_mud".into()),
        });
        assert_eq!(unknown_mue_mud.exit_code(), 1, "{}", unknown_mue_mud.text());
        assert!(
            unknown_mue_mud
                .text()
                .contains("unknown constant 'mue_mud'"),
            "{}",
            unknown_mue_mud.text()
        );

        let unknown_mue_mu0h = lab.exec(Command::Constant {
            name: Some("mue_mu0h".into()),
        });
        assert_eq!(
            unknown_mue_mu0h.exit_code(),
            1,
            "{}",
            unknown_mue_mu0h.text()
        );
        assert!(
            unknown_mue_mu0h
                .text()
                .contains("unknown constant 'mue_mu0h'"),
            "{}",
            unknown_mue_mu0h.text()
        );

        let unknown_mmu = lab.exec(Command::Constant {
            name: Some("mmu".into()),
        });
        assert_eq!(unknown_mmu.exit_code(), 1, "{}", unknown_mmu.text());
        assert!(
            unknown_mmu.text().contains("unknown constant 'mmu'"),
            "{}",
            unknown_mmu.text()
        );

        let unknown_mmu_u = lab.exec(Command::Constant {
            name: Some("mmu_u".into()),
        });
        assert_eq!(unknown_mmu_u.exit_code(), 1, "{}", unknown_mmu_u.text());
        assert!(
            unknown_mmu_u.text().contains("unknown constant 'mmu_u'"),
            "{}",
            unknown_mmu_u.text()
        );

        let unknown_m_mu_slash_u = lab.exec(Command::Constant {
            name: Some("m_mu/u".into()),
        });
        assert_eq!(
            unknown_m_mu_slash_u.exit_code(),
            1,
            "{}",
            unknown_m_mu_slash_u.text()
        );
        assert!(
            unknown_m_mu_slash_u
                .text()
                .contains("unknown constant 'm_mu/u'"),
            "{}",
            unknown_m_mu_slash_u.text()
        );

        let unknown_m_mu_hyphen_u = lab.exec(Command::Constant {
            name: Some("m-mu-u".into()),
        });
        assert_eq!(
            unknown_m_mu_hyphen_u.exit_code(),
            1,
            "{}",
            unknown_m_mu_hyphen_u.text()
        );
        assert!(
            unknown_m_mu_hyphen_u
                .text()
                .contains("unknown constant 'm-mu-u'"),
            "{}",
            unknown_m_mu_hyphen_u.text()
        );

        let unknown_ar_mu = lab.exec(Command::Constant {
            name: Some("Ar_mu".into()),
        });
        assert_eq!(unknown_ar_mu.exit_code(), 1, "{}", unknown_ar_mu.text());
        assert!(
            unknown_ar_mu.text().contains("unknown constant 'Ar_mu'"),
            "{}",
            unknown_ar_mu.text()
        );

        let unknown_mmu_c2 = lab.exec(Command::Constant {
            name: Some("mmu_c2".into()),
        });
        assert_eq!(unknown_mmu_c2.exit_code(), 1, "{}", unknown_mmu_c2.text());
        assert!(
            unknown_mmu_c2.text().contains("unknown constant 'mmu_c2'"),
            "{}",
            unknown_mmu_c2.text()
        );

        let unknown_mmuc2 = lab.exec(Command::Constant {
            name: Some("mmuc2".into()),
        });
        assert_eq!(unknown_mmuc2.exit_code(), 1, "{}", unknown_mmuc2.text());
        assert!(
            unknown_mmuc2.text().contains("unknown constant 'mmuc2'"),
            "{}",
            unknown_mmuc2.text()
        );

        let unknown_mmuc2_mev = lab.exec(Command::Constant {
            name: Some("mmuc2_MeV".into()),
        });
        assert_eq!(
            unknown_mmuc2_mev.exit_code(),
            1,
            "{}",
            unknown_mmuc2_mev.text()
        );
        assert!(
            unknown_mmuc2_mev
                .text()
                .contains("unknown constant 'mmuc2_MeV'"),
            "{}",
            unknown_mmuc2_mev.text()
        );

        let unknown_mmu_c2_mev = lab.exec(Command::Constant {
            name: Some("mmu_c2_MeV".into()),
        });
        assert_eq!(
            unknown_mmu_c2_mev.exit_code(),
            1,
            "{}",
            unknown_mmu_c2_mev.text()
        );
        assert!(
            unknown_mmu_c2_mev
                .text()
                .contains("unknown constant 'mmu_c2_MeV'"),
            "{}",
            unknown_mmu_c2_mev.text()
        );

        let unknown_m_mu_me = lab.exec(Command::Constant {
            name: Some("m_mu_me".into()),
        });
        assert_eq!(unknown_m_mu_me.exit_code(), 1, "{}", unknown_m_mu_me.text());
        assert!(
            unknown_m_mu_me
                .text()
                .contains("unknown constant 'm_mu_me'"),
            "{}",
            unknown_m_mu_me.text()
        );

        let unknown_mmu_slash_me = lab.exec(Command::Constant {
            name: Some("mmu/me".into()),
        });
        assert_eq!(
            unknown_mmu_slash_me.exit_code(),
            1,
            "{}",
            unknown_mmu_slash_me.text()
        );
        assert!(
            unknown_mmu_slash_me
                .text()
                .contains("unknown constant 'mmu/me'"),
            "{}",
            unknown_mmu_slash_me.text()
        );

        let unknown_m_dash_mu_me = lab.exec(Command::Constant {
            name: Some("m-mu-me".into()),
        });
        assert_eq!(
            unknown_m_dash_mu_me.exit_code(),
            1,
            "{}",
            unknown_m_dash_mu_me.text()
        );
        assert!(
            unknown_m_dash_mu_me
                .text()
                .contains("unknown constant 'm-mu-me'"),
            "{}",
            unknown_m_dash_mu_me.text()
        );

        let unknown_m_mu_slash_me = lab.exec(Command::Constant {
            name: Some("m_mu/me".into()),
        });
        assert_eq!(
            unknown_m_mu_slash_me.exit_code(),
            1,
            "{}",
            unknown_m_mu_slash_me.text()
        );
        assert!(
            unknown_m_mu_slash_me
                .text()
                .contains("unknown constant 'm_mu/me'"),
            "{}",
            unknown_m_mu_slash_me.text()
        );

        let unknown_m_mu_mp = lab.exec(Command::Constant {
            name: Some("m_mu_mp".into()),
        });
        assert_eq!(unknown_m_mu_mp.exit_code(), 1, "{}", unknown_m_mu_mp.text());
        assert!(
            unknown_m_mu_mp
                .text()
                .contains("unknown constant 'm_mu_mp'"),
            "{}",
            unknown_m_mu_mp.text()
        );

        let unknown_mmu_slash_mp = lab.exec(Command::Constant {
            name: Some("mmu/mp".into()),
        });
        assert_eq!(
            unknown_mmu_slash_mp.exit_code(),
            1,
            "{}",
            unknown_mmu_slash_mp.text()
        );
        assert!(
            unknown_mmu_slash_mp
                .text()
                .contains("unknown constant 'mmu/mp'"),
            "{}",
            unknown_mmu_slash_mp.text()
        );

        let unknown_m_dash_mu_mp = lab.exec(Command::Constant {
            name: Some("m-mu-mp".into()),
        });
        assert_eq!(
            unknown_m_dash_mu_mp.exit_code(),
            1,
            "{}",
            unknown_m_dash_mu_mp.text()
        );
        assert!(
            unknown_m_dash_mu_mp
                .text()
                .contains("unknown constant 'm-mu-mp'"),
            "{}",
            unknown_m_dash_mu_mp.text()
        );

        let unknown_m_mu_slash_mp = lab.exec(Command::Constant {
            name: Some("m_mu/mp".into()),
        });
        assert_eq!(
            unknown_m_mu_slash_mp.exit_code(),
            1,
            "{}",
            unknown_m_mu_slash_mp.text()
        );
        assert!(
            unknown_m_mu_slash_mp
                .text()
                .contains("unknown constant 'm_mu/mp'"),
            "{}",
            unknown_m_mu_slash_mp.text()
        );

        let unknown_mmu_mtau = lab.exec(Command::Constant {
            name: Some("mmu_mtau".into()),
        });
        assert_eq!(
            unknown_mmu_mtau.exit_code(),
            1,
            "{}",
            unknown_mmu_mtau.text()
        );
        assert!(
            unknown_mmu_mtau
                .text()
                .contains("unknown constant 'mmu_mtau'"),
            "{}",
            unknown_mmu_mtau.text()
        );

        let unknown_m_mu_mn = lab.exec(Command::Constant {
            name: Some("m_mu_mn".into()),
        });
        assert_eq!(unknown_m_mu_mn.exit_code(), 1, "{}", unknown_m_mu_mn.text());
        assert!(
            unknown_m_mu_mn
                .text()
                .contains("unknown constant 'm_mu_mn'"),
            "{}",
            unknown_m_mu_mn.text()
        );

        let unknown_mmu_slash_mn = lab.exec(Command::Constant {
            name: Some("mmu/mn".into()),
        });
        assert_eq!(
            unknown_mmu_slash_mn.exit_code(),
            1,
            "{}",
            unknown_mmu_slash_mn.text()
        );
        assert!(
            unknown_mmu_slash_mn
                .text()
                .contains("unknown constant 'mmu/mn'"),
            "{}",
            unknown_mmu_slash_mn.text()
        );

        let unknown_m_dash_mu_mn = lab.exec(Command::Constant {
            name: Some("m-mu-mn".into()),
        });
        assert_eq!(
            unknown_m_dash_mu_mn.exit_code(),
            1,
            "{}",
            unknown_m_dash_mu_mn.text()
        );
        assert!(
            unknown_m_dash_mu_mn
                .text()
                .contains("unknown constant 'm-mu-mn'"),
            "{}",
            unknown_m_dash_mu_mn.text()
        );

        let unknown_m_mu_slash_mn = lab.exec(Command::Constant {
            name: Some("m_mu/mn".into()),
        });
        assert_eq!(
            unknown_m_mu_slash_mn.exit_code(),
            1,
            "{}",
            unknown_m_mu_slash_mn.text()
        );
        assert!(
            unknown_m_mu_slash_mn
                .text()
                .contains("unknown constant 'm_mu/mn'"),
            "{}",
            unknown_m_mu_slash_mn.text()
        );

        let unknown_mmu_molar = lab.exec(Command::Constant {
            name: Some("Mmu".into()),
        });
        assert_eq!(
            unknown_mmu_molar.exit_code(),
            1,
            "{}",
            unknown_mmu_molar.text()
        );
        assert!(
            unknown_mmu_molar.text().contains("unknown constant 'Mmu'"),
            "{}",
            unknown_mmu_molar.text()
        );

        let unknown_m_dash_mu_molar = lab.exec(Command::Constant {
            name: Some("M-mu".into()),
        });
        assert_eq!(
            unknown_m_dash_mu_molar.exit_code(),
            1,
            "{}",
            unknown_m_dash_mu_molar.text()
        );
        assert!(
            unknown_m_dash_mu_molar
                .text()
                .contains("unknown constant 'M-mu'"),
            "{}",
            unknown_m_dash_mu_molar.text()
        );

        let unknown_m_mu_slash_mol = lab.exec(Command::Constant {
            name: Some("M_mu/mol".into()),
        });
        assert_eq!(
            unknown_m_mu_slash_mol.exit_code(),
            1,
            "{}",
            unknown_m_mu_slash_mol.text()
        );
        assert!(
            unknown_m_mu_slash_mol
                .text()
                .contains("unknown constant 'M_mu/mol'"),
            "{}",
            unknown_m_mu_slash_mol.text()
        );

        let unknown_lambda_cmu = lab.exec(Command::Constant {
            name: Some("lambda_Cmu".into()),
        });
        assert_eq!(
            unknown_lambda_cmu.exit_code(),
            1,
            "{}",
            unknown_lambda_cmu.text()
        );
        assert!(
            unknown_lambda_cmu
                .text()
                .contains("unknown constant 'lambda_Cmu'"),
            "{}",
            unknown_lambda_cmu.text()
        );

        let unknown_lambdac_mu = lab.exec(Command::Constant {
            name: Some("lambdaC_mu".into()),
        });
        assert_eq!(
            unknown_lambdac_mu.exit_code(),
            1,
            "{}",
            unknown_lambdac_mu.text()
        );
        assert!(
            unknown_lambdac_mu
                .text()
                .contains("unknown constant 'lambdaC_mu'"),
            "{}",
            unknown_lambdac_mu.text()
        );

        let unknown_lambda_dash = lab.exec(Command::Constant {
            name: Some("lambda-C-mu".into()),
        });
        assert_eq!(
            unknown_lambda_dash.exit_code(),
            1,
            "{}",
            unknown_lambda_dash.text()
        );
        assert!(
            unknown_lambda_dash
                .text()
                .contains("unknown constant 'lambda-C-mu'"),
            "{}",
            unknown_lambda_dash.text()
        );

        let unknown_lambdabar_c_mu = lab.exec(Command::Constant {
            name: Some("lambdabar_C_mu".into()),
        });
        assert_eq!(
            unknown_lambdabar_c_mu.exit_code(),
            1,
            "{}",
            unknown_lambdabar_c_mu.text()
        );
        assert!(
            unknown_lambdabar_c_mu
                .text()
                .contains("unknown constant 'lambdabar_C_mu'"),
            "{}",
            unknown_lambdabar_c_mu.text()
        );

        let unknown_mumu = lab.exec(Command::Constant {
            name: Some("mumu".into()),
        });
        assert_eq!(unknown_mumu.exit_code(), 1, "{}", unknown_mumu.text());
        assert!(
            unknown_mumu.text().contains("unknown constant 'mumu'"),
            "{}",
            unknown_mumu.text()
        );

        let unknown_mu_dash_mu = lab.exec(Command::Constant {
            name: Some("mu-mu".into()),
        });
        assert_eq!(
            unknown_mu_dash_mu.exit_code(),
            1,
            "{}",
            unknown_mu_dash_mu.text()
        );
        assert!(
            unknown_mu_dash_mu
                .text()
                .contains("unknown constant 'mu-mu'"),
            "{}",
            unknown_mu_dash_mu.text()
        );

        let unknown_mu_m_mu = lab.exec(Command::Constant {
            name: Some("mu_m_mu".into()),
        });
        assert_eq!(unknown_mu_m_mu.exit_code(), 1, "{}", unknown_mu_m_mu.text());
        assert!(
            unknown_mu_m_mu
                .text()
                .contains("unknown constant 'mu_m_mu'"),
            "{}",
            unknown_mu_m_mu.text()
        );

        let unknown_mumu_mu_b = lab.exec(Command::Constant {
            name: Some("mumu_muB".into()),
        });
        assert_eq!(
            unknown_mumu_mu_b.exit_code(),
            1,
            "{}",
            unknown_mumu_mu_b.text()
        );
        assert!(
            unknown_mumu_mu_b
                .text()
                .contains("unknown constant 'mumu_muB'"),
            "{}",
            unknown_mumu_mu_b.text()
        );

        let unknown_mu_mu_slash_mu_b = lab.exec(Command::Constant {
            name: Some("mu_mu/muB".into()),
        });
        assert_eq!(
            unknown_mu_mu_slash_mu_b.exit_code(),
            1,
            "{}",
            unknown_mu_mu_slash_mu_b.text()
        );
        assert!(
            unknown_mu_mu_slash_mu_b
                .text()
                .contains("unknown constant 'mu_mu/muB'"),
            "{}",
            unknown_mu_mu_slash_mu_b.text()
        );

        let unknown_mu_dash_mu_mu_b = lab.exec(Command::Constant {
            name: Some("mu-mu-muB".into()),
        });
        assert_eq!(
            unknown_mu_dash_mu_mu_b.exit_code(),
            1,
            "{}",
            unknown_mu_dash_mu_mu_b.text()
        );
        assert!(
            unknown_mu_dash_mu_mu_b
                .text()
                .contains("unknown constant 'mu-mu-muB'"),
            "{}",
            unknown_mu_dash_mu_mu_b.text()
        );

        let unknown_mumu_mu_n = lab.exec(Command::Constant {
            name: Some("mumu_muN".into()),
        });
        assert_eq!(
            unknown_mumu_mu_n.exit_code(),
            1,
            "{}",
            unknown_mumu_mu_n.text()
        );
        assert!(
            unknown_mumu_mu_n
                .text()
                .contains("unknown constant 'mumu_muN'"),
            "{}",
            unknown_mumu_mu_n.text()
        );

        let unknown_mu_mu_slash_mu_n = lab.exec(Command::Constant {
            name: Some("mu_mu/muN".into()),
        });
        assert_eq!(
            unknown_mu_mu_slash_mu_n.exit_code(),
            1,
            "{}",
            unknown_mu_mu_slash_mu_n.text()
        );
        assert!(
            unknown_mu_mu_slash_mu_n
                .text()
                .contains("unknown constant 'mu_mu/muN'"),
            "{}",
            unknown_mu_mu_slash_mu_n.text()
        );

        let unknown_mu_dash_mu_mu_n = lab.exec(Command::Constant {
            name: Some("mu-mu-muN".into()),
        });
        assert_eq!(
            unknown_mu_dash_mu_mu_n.exit_code(),
            1,
            "{}",
            unknown_mu_dash_mu_mu_n.text()
        );
        assert!(
            unknown_mu_dash_mu_mu_n
                .text()
                .contains("unknown constant 'mu-mu-muN'"),
            "{}",
            unknown_mu_dash_mu_mu_n.text()
        );

        let unknown_a_mu = lab.exec(Command::Constant {
            name: Some("a_mu".into()),
        });
        assert_eq!(unknown_a_mu.exit_code(), 1, "{}", unknown_a_mu.text());
        assert!(
            unknown_a_mu.text().contains("unknown constant 'a_mu'"),
            "{}",
            unknown_a_mu.text()
        );

        let unknown_a_dash_mu = lab.exec(Command::Constant {
            name: Some("a-mu".into()),
        });
        assert_eq!(
            unknown_a_dash_mu.exit_code(),
            1,
            "{}",
            unknown_a_dash_mu.text()
        );
        assert!(
            unknown_a_dash_mu.text().contains("unknown constant 'a-mu'"),
            "{}",
            unknown_a_dash_mu.text()
        );

        let unknown_amu_e = lab.exec(Command::Constant {
            name: Some("amu_e".into()),
        });
        assert_eq!(unknown_amu_e.exit_code(), 1, "{}", unknown_amu_e.text());
        assert!(
            unknown_amu_e.text().contains("unknown constant 'amu_e'"),
            "{}",
            unknown_amu_e.text()
        );

        let unknown_mue = lab.exec(Command::Constant {
            name: Some("mue".into()),
        });
        assert_eq!(unknown_mue.exit_code(), 1, "{}", unknown_mue.text());
        assert!(
            unknown_mue.text().contains("unknown constant 'mue'"),
            "{}",
            unknown_mue.text()
        );

        let unknown_z = lab.exec(Command::Constant {
            name: Some("Y0".into()),
        });
        assert_eq!(unknown_z.exit_code(), 1, "{}", unknown_z.text());
        assert!(
            unknown_z.text().contains("unknown constant 'Y0'"),
            "{}",
            unknown_z.text()
        );

        let unknown_mu = lab.exec(Command::Constant {
            name: Some("mu_0".into()),
        });
        assert_eq!(unknown_mu.exit_code(), 1, "{}", unknown_mu.text());
        assert!(
            unknown_mu.text().contains("unknown constant 'mu_0'"),
            "{}",
            unknown_mu.text()
        );

        let ledger = lab
            .exec(Command::Constant { name: None })
            .text()
            .to_string();
        assert!(ledger.starts_with("constant  ledger  node "), "{ledger}");
        let ledger_id = constant_node_id(&ledger);
        assert_eq!(
            ledger_id.to_hex(),
            "044a027898acd4fbe72cfb6f012d248e24f95be834da6c9f5598cabc268a52c1",
            "journaling must not change the LEDGER bundle payload"
        );
        assert_eq!(
            lab.store.get(ledger_id).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        for name in physis_constants::LEDGER {
            assert!(
                ledger.contains(&format!("constant  {name}  node ")),
                "ledger missing {name}: {ledger}"
            );
        }
        assert!(
            ledger.contains(
                "hash     ebbfc13ea8fba734da50b679d9eaf236638b244cdcc350c0b14cdd6696850e92"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     cef64589acdbd1ed4cb5f5f631658978c01477248f334b1d3563e57314644b38"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    alpha = 7.2973525693(11)e-3"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     4b7050d77da09c5322877eaf83e94ebba7b84c99bad8ba3713b0e5fe91128482"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    inv_alpha = 137.035999084(21)"),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    cRinf = 3.2898419602508(64)e15"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     c7c49f18cb4f9905decad406f7a835f59588f34483afa3e4751097451d5d9969"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    hcRinf = 2.1798723611035(42)e-18"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     0d0308e874e54cb3d02570c972232b0d26c2d1d64b493880a1bb7ce4ff7827b2"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    Rinf = 10973731.568160(21)"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     5d5098fcd983d3db221e4b4047e73de5061985c31a91ccdf12cd122b620eaf29"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    a0 = 5.29177210903(80)e-11"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     c4606c77e55763a397f633ef0f3ace1328d3e1e8781428baf97554c97f4fba5a"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    Eh = 4.3597447222071(85)e-18"),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    me/mmu = 4.83633169(11)e-3"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     d57979e61fa03bae0a3b0dc5e2cff20df53cdcb76b772cf6ea2589e77c9c3cb2"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    me/mp = 5.44617021487(33)e-4"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     b573fa37eb0080e54bc71e3bf41170421c2bae2911609e1d11ffc129448a2e7b"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    me/mn = 5.4386734424(26)e-4"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     e271d2015c7b39491daebf2a1d532ebe4c4dacf8228b3f7fc4d258be7b79ecba"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    me/md = 2.724437107462(96)e-4"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     2aa5fe69f8cdd03f44e77b006a3b6ea90d48e1b8aec71275e184c4e529f0f76c"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    me/mt = 1.819200062251(90)e-4"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     2f8187d744269836cf0fbc123f8cb7d60107215e65be109daf2ae67c8116afd1"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    me/mh = 1.819543074573(79)e-4"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     0fb8f5fde9e76fcf2c24d73267f36cd3a5b40ca9f27f24e47d9807ec4206055e"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    me/malpha = 1.370933554787(45)e-4"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     3407529f38a47a2cf983c5418482d3d9bab5243e08258bbe88c06a2f42e0baa3"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    -e/me = -1.75882001076(53)e11"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     bfe24e8de43e90dbc8a28472f99ed206f07566fa1a4fa6c6d14356adf4e89b22"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    Me = 5.4857990888(17)e-7"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     0a8b3285a4969854567b59db2ebf9449268df86ffdbb461e3b9c1db0955eb804"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    lambdabar_C = 3.8615926796(12)e-13"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     0ed48571f065fc19458ea3c8fd493fd00de18a7d196669f81bb93c50779bc625"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    lambda_C = 2.42631023867(73)e-12"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     6280f2b2f61adf3ae0fa3e65f3b12cfb4982f6601027d98552f541246198c3d8"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    re = 2.8179403262(13)e-15"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     1b8dfc7aa2f90183fd50dab61cf3361f57c3c906e6a221ffa3b2ef17302a38d4"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    mu_e = -9.2847647043(28)e-24"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     e48d03baa8e8b2f62d1ea5c19a7010b583cdfba3f4f9c3d2b55877817d36c9b8"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    mu_e/muB = -1.00115965218128(18)"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     5d4db81093e3f34e08d258ab214de2fb6649d8e7f07cd37c2f5f625a89b52926"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    mu_e/muN = -1838.28197188(11)"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     2a82c539bc621b71977129a26433da37e94f1afd8b38e50c031da0133e2196ca"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    ae = 1.15965218128(18)e-3"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     0fb8666d816320352cbc8e24b896bbb2adc59a085d3b469659d41c6447c82da5"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    ge = -2.00231930436256(35)"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     8e1daf3628381ffa7dce3fafc5e65038038eb74b5537cf7adb95702f5d0e0050"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    mu_e/mmu = 206.7669883(46)"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     125652aec9ee47a2db2df2ae81c39cfeb8d9b4037098829e64b78873deb56559"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    mu_e/mup = -658.21068789(20)"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     13a0d90f76fb16f948196cf56fb9d54e90ccc43ad4ff613f27873de735ba7b5b"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    mu_e/mu0p = -658.2275971(72)"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     a3028069b2f88c67432e3c555655438a64bd7b150b2add2b6539e38b3e2df199"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    mu_e/mun = 960.92050(23)"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     9abd0d4216937c89cafceaa4f418b8e8b65a2216df12b3bbc6a1976b1f5c8df2"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    mu_e/mud = -2143.9234915(56)"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     7db59dc912a6c2a301f669f52d7353b27672a07b917e2f8b92b03c1f9acaaa64"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    mu_e/mu0h = 864.058257(10)"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     3e3e29f0ac633705b8d8467b80b0cd229b07f4d7ba44fe32b84730261c576a9b"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    mmu = 1.883531627(42)e-28"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     b1e0e67d46205c048709815e1215184c1b77afbcb0f197099085fbfc7d3bb016"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    mmu_u = 0.1134289259(25)"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     ced234733b80023dd6d8687ce99efc8473defe15f63b74f3ecde00ece485515d"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    mmu_c2 = 1.692833804(38)e-11"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     d83a5072b8cb4fe869a2aa076aff9c4cd0d8f9f613a41eef52117124acde5854"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    mmu_c2_MeV = 105.6583755(23)"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     292b0524e0f1a160403fe1a2a4998cd4c2690f5d3b344a5f8ba31e9248be0416"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    mmu/me = 206.7682830(46)"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     0ac70815382ab74fd46513f298dd351685fcc54ab0e64b6fc00b64b4fccc426f"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    mmu/mp = 0.1126095264(25)"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     1527aa21236682ad99206cf1ef6b6267d7432a5a1975bcc2315af9a510e147d2"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    mmu/mn = 0.1124545170(25)"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     f8a9dfb53e84c4a592143e9d17e9e04884b69cc9b2b378dc2a7c099c4d442835"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    Mmu = 1.134289259(25)e-4"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     b53efc5e339708317e98c92c02ae506bf5b90c6d847e586d716d1631d902c81a"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    lambda_C_mu = 1.173444110(26)e-14"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     6fb48517f2b436bf1ede156c0dd4505692db4e7afe3e5d6f7ed2bfbfdc4198d9"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    mu_mu = -4.49044830(10)e-26"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     3344549ca18b2db388cfff366cc63079f3d3b0b094cac6de12e318fe8531c3e0"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    mu_mu/muB = -4.84197047(11)e-3"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     5fa244938a528feff7867ea9ae972d76da59930a932f2a5ac9fe6ef52762c591"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    mu_mu/muN = -8.89059703(20)"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     52a97de9669b20480e5729de915cef56ba841da392185b3e5893b10c496ed16b"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    amu = 1.16592089(63)e-3"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     972c93982e6cd84f054db85605b9e7d106d124bd52bac104bb447d788cdc64c4"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     ffd371a69f7ec3d9bac8dcf57e0126709fd3f63c35561e717d9886d2fb1f88c8"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    mp = 1.67262192369(51)e-27"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     fa1264a6ce514520c9c2d9131fee2c71cacd4ce5fe615ea4dd424fd23de35cd7"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    mu0 = 1.25663706212(19)e-6"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     fadaf2a47a8161ba2727a4c2ff6b842f7c9e6add2edd67cd5496a7a753f22d80"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    epsilon0 = 8.8541878128(13)e-12"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     6f72c1c5833dc722ac6fb5223f982879499ff412157c6e6c9851d77088991316"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains("range    Z0 = 376.730313668(57)"),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     691eb73ea444f6d10fb223b999a1b37c0b67da92d51e43ca8bd8a6561785a3c1"
            ),
            "{ledger}"
        );
        assert!(
            ledger.contains(
                "hash     50a96a8715769547a90cba69b0775d8892d79f2fa32465ad13a6d73b2d111eef"
            ),
            "{ledger}"
        );
        assert!(ledger.contains("kind     interval"), "{ledger}");
        assert!(ledger.contains("kind     ratio"), "{ledger}");
        assert!(ledger.contains("kind     sci-exact"), "{ledger}");
        assert!(ledger.contains("662607015e-42"), "{ledger}");
        assert!(
            ledger.contains(
                "hash     d3441603d75b565016c25cc955783fbb76b4050ee22befcef0c0e3896e873a0b"
            ),
            "{ledger}"
        );
        assert!(ledger.contains("table    8"), "{ledger}");
        assert!(ledger.contains("149597870700"), "{ledger}");
        assert!(
            ledger.contains(
                "hash     636001001c4ed9cd5e6661241e5ad5e5db09c8419a3fe79790143162b7af3a58"
            ),
            "{ledger}"
        );
        assert!(ledger.contains("release  iau-2015"), "{ledger}");
        assert!(ledger.contains("132712440000000000000"), "{ledger}");
        assert!(
            ledger.contains(
                "hash     cb7f91f2d0663d2d8ff8b0e3009f6e0772a126220d04ed658fc793db7e5cc6b4"
            ),
            "{ledger}"
        );
        assert!(ledger.contains("695700000"), "{ledger}");
        assert!(
            ledger.contains(
                "hash     444f85fba501ddec8fb08ba403c1b869cc78a2284df5466a56a617043807bbc4"
            ),
            "{ledger}"
        );
        assert!(ledger.contains("382800000000000000000000000"), "{ledger}");
        assert!(
            ledger.contains(
                "hash     d5514de9cbef3f6990067899529d34f20b4349ca3b20ba18c9a5932c8c6b6c0f"
            ),
            "{ledger}"
        );
        assert!(!ledger.contains("receipt"), "{ledger}");
        assert!(!ledger.contains("theorem"), "{ledger}");
        assert_eq!(
            ledger.matches("constant  ").count(),
            1 + physis_constants::LEDGER.len(),
            "{ledger}"
        );

        let p3n = lab
            .exec(Command::Inspect {
                axis: Some("trust".into()),
                value: Some("P3N".into()),
            })
            .text()
            .to_string();
        assert!(p3n.contains("count 4"), "constant must not mint P3N: {p3n}");

        lab.set_role(Role::Lab);
        let sr = lab
            .exec(Command::Encode {
                theory: "special-relativity".into(),
            })
            .text()
            .to_string();
        assert!(
            sr.contains("faecac5791ad5650337c61dcb10e45d5eb36ca24c0423df51891673ba3da3ef6"),
            "{sr}"
        );
    }

    #[test]
    fn versioned_constant_restores_by_rebuild_not_deserialize() {
        let mut lab1 = Lab::standard();
        let first = lab1
            .exec(Command::Constant {
                name: Some("G".into()),
            })
            .text()
            .to_string();
        let live = constant_node_id(&first);
        assert_eq!(
            live.to_hex(),
            "f320ea2da0141f16c191acd3001a6fe0b5074fc73d4768fa91f42d8e85abc52c",
            "journaling must not change the G constant payload"
        );
        let jsonl = lab1.journal().to_string();
        assert!(jsonl.contains("\"event\":\"constant\""), "{jsonl}");
        assert!(
            jsonl.contains(&format!("\"node_hash\":\"{}\"", live.to_hex())),
            "{jsonl}"
        );

        let mut lab2 = Lab::standard();
        *lab2.journal_mut() = Journal::from_jsonl(&jsonl);
        let journal_len = lab2.journal().len();
        lab2.restore_from_journal();
        assert_eq!(lab2.journal().len(), journal_len);
        assert_eq!(
            lab2.store.get(live).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let forged_hex = "0".repeat(64);
        let tampered =
            format!(r#"{{"event":"constant","t":1,"name":"G","node_hash":"{forged_hex}"}}"#);
        let mut lab3 = Lab::standard();
        *lab3.journal_mut() = Journal::from_jsonl(&tampered);
        lab3.restore_from_journal();
        assert_eq!(
            lab3.store.get(live).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let forged = physis_core::artifact::ArtifactId::from_hex(&forged_hex)
            .expect("64 hex zeros is an ArtifactId");
        assert!(lab3.store.get(forged).is_none());
    }

    #[test]
    fn versioned_constant_ledger_restores_by_rebuild_not_deserialize() {
        let mut lab1 = Lab::standard();
        let first = lab1
            .exec(Command::Constant { name: None })
            .text()
            .to_string();
        let live = constant_node_id(&first);
        assert_eq!(
            live.to_hex(),
            "044a027898acd4fbe72cfb6f012d248e24f95be834da6c9f5598cabc268a52c1",
            "journaling must not change the LEDGER bundle payload"
        );
        assert!(first.starts_with("constant  ledger  node "), "{first}");
        assert_eq!(
            lab1.store.get(live).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let jsonl = lab1.journal().to_string();
        assert!(jsonl.contains("\"event\":\"constant\""), "{jsonl}");
        assert!(jsonl.contains("\"name\":\"\""), "{jsonl}");
        assert!(
            jsonl.contains(&format!("\"node_hash\":\"{}\"", live.to_hex())),
            "{jsonl}"
        );
        assert!(!jsonl.contains("receipt"), "{jsonl}");

        let mut lab2 = Lab::standard();
        *lab2.journal_mut() = Journal::from_jsonl(&jsonl);
        let journal_len = lab2.journal().len();
        lab2.restore_from_journal();
        assert_eq!(lab2.journal().len(), journal_len);
        assert_eq!(
            lab2.store.get(live).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let g = physis_core::artifact::ArtifactId::from_hex(
            "f320ea2da0141f16c191acd3001a6fe0b5074fc73d4768fa91f42d8e85abc52c",
        )
        .expect("pinned G node");
        assert_eq!(
            lab2.store.get(g).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let alpha = physis_core::artifact::ArtifactId::from_hex(
            "b2b54749bb1e674d72e0b1c7ffa688dbd1cabb8a8a481db3f94bfeba9735f073",
        )
        .expect("pinned alpha node");
        assert_eq!(
            lab2.store.get(alpha).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let mp = physis_core::artifact::ArtifactId::from_hex(
            "3fcee64bc46c5e13e69bc0d822e66e701b793910803445e5d4689ead316028a0",
        )
        .expect("pinned m_p node");
        assert_eq!(
            lab2.store.get(mp).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let mu0 = physis_core::artifact::ArtifactId::from_hex(
            "2b652a4d95e792481d77b5358af0426e6425619a32e86965bd6f19cbec20ae9e",
        )
        .expect("pinned mu0 node");
        assert_eq!(
            lab2.store.get(mu0).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let eps = physis_core::artifact::ArtifactId::from_hex(
            "0b2262eee35047174ebb10962a4aaf06481a8e18e9b44a3873a129f91873a8b7",
        )
        .expect("pinned epsilon0 node");
        assert_eq!(
            lab2.store.get(eps).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let z0 = physis_core::artifact::ArtifactId::from_hex(
            "e7ebeaa7b1b18ebed7aa269cd7b4b322842d5f674106efb8b1ad1cda4d4ac77e",
        )
        .expect("pinned Z0 node");
        assert_eq!(
            lab2.store.get(z0).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let inv_alpha = physis_core::artifact::ArtifactId::from_hex(
            "6943c43fe01b2b9dbde1c0bd147f0293a69cb15bb2e44877ea7e68013f6dce0e",
        )
        .expect("pinned inv_alpha node");
        assert_eq!(
            lab2.store.get(inv_alpha).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let rinf = physis_core::artifact::ArtifactId::from_hex(
            "0fb78b2d6e881df7b19d8a55878f642e27dc4d51a8f74ffe0c1e28e9d93380ac",
        )
        .expect("pinned Rinf node");
        assert_eq!(
            lab2.store.get(rinf).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let a0 = physis_core::artifact::ArtifactId::from_hex(
            "01663e8bd28309970cefc37bd3dc5023c54a70ded784fb04d94ace095abdd475",
        )
        .expect("pinned a0 node");
        assert_eq!(
            lab2.store.get(a0).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let eh = physis_core::artifact::ArtifactId::from_hex(
            "84818158c407563a9a514c8eedc85ee7303b0d96f09f09610bda6684582cc82e",
        )
        .expect("pinned Eh node");
        assert_eq!(
            lab2.store.get(eh).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let me_mmu = physis_core::artifact::ArtifactId::from_hex(
            "60d9b01d547b5ad4307443e4ba7749adb42c4da1343f16a35f194c80bbc35088",
        )
        .expect("pinned me_mmu node");
        assert_eq!(
            lab2.store.get(me_mmu).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let me_mp = physis_core::artifact::ArtifactId::from_hex(
            "b4fd3e8b7678afd9bb4aea49c3b06c9756ab3d6fced7b4b49b25c322134bf3f2",
        )
        .expect("pinned me_mp node");
        assert_eq!(
            lab2.store.get(me_mp).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let me_mn = physis_core::artifact::ArtifactId::from_hex(
            "deeb5e2665cabc16ffa607d446a4018cabf8b2b427fdb0b81184384113089bb3",
        )
        .expect("pinned me_mn node");
        assert_eq!(
            lab2.store.get(me_mn).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let me_md = physis_core::artifact::ArtifactId::from_hex(
            "a2b8e4d5a5cdff854b67986773f186e1f427dc9cfc0d6d92f3a01ee81bdd26e5",
        )
        .expect("pinned me_md node");
        assert_eq!(
            lab2.store.get(me_md).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let me_mt = physis_core::artifact::ArtifactId::from_hex(
            "3d9b3ce3c7ecca0e131e0232f308ce878696a268e263286e133c8edc441eb7f0",
        )
        .expect("pinned me_mt node");
        assert_eq!(
            lab2.store.get(me_mt).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let me_mh = physis_core::artifact::ArtifactId::from_hex(
            "b55534bac40b377d7b8c6123de509a2b65cde4d75fe280d46aefa30f83e72890",
        )
        .expect("pinned me_mh node");
        assert_eq!(
            lab2.store.get(me_mh).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let me_malpha = physis_core::artifact::ArtifactId::from_hex(
            "ddb38fbd88d7250c7aea0e87e0bd2c44b32d5b5b0fd9eb1b0689bb9aa3315545",
        )
        .expect("pinned me_malpha node");
        assert_eq!(
            lab2.store.get(me_malpha).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let e_me = physis_core::artifact::ArtifactId::from_hex(
            "4180ebda17cac1399d5888468d4686d9874499a1e6b2c386a3ccbe58f8039f36",
        )
        .expect("pinned e_me node");
        assert_eq!(
            lab2.store.get(e_me).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let molar = physis_core::artifact::ArtifactId::from_hex(
            "da1692471def8d3d930d45de5d4e089231c2d18fc859d73feeb22ffe89075692",
        )
        .expect("pinned M_e node");
        assert_eq!(
            lab2.store.get(molar).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let rcbar = physis_core::artifact::ArtifactId::from_hex(
            "3fd48f3a014e92dae7062468ea0d7df4e4e1e44da7a6a9a6cccea5a5a4ffcc0d",
        )
        .expect("pinned lambdabar_C node");
        assert_eq!(
            lab2.store.get(rcbar).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let rc = physis_core::artifact::ArtifactId::from_hex(
            "4c83c25a7c4f517afc2e092809b141dffc97ae12307b4676cb01da5ab73716e3",
        )
        .expect("pinned lambda_C node");
        assert_eq!(
            lab2.store.get(rc).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let re = physis_core::artifact::ArtifactId::from_hex(
            "bd8a6f5f629ba9df37a0246f420d98c4bbde1d82cdcaaa8d4f9c7796ba239c23",
        )
        .expect("pinned re node");
        assert_eq!(
            lab2.store.get(re).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let mu_e = physis_core::artifact::ArtifactId::from_hex(
            "5ed9218a55b4eaa8b15614c412c1454a7be21e3a43a317c39275aa68095d5a0d",
        )
        .expect("pinned mu_e node");
        assert_eq!(
            lab2.store.get(mu_e).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let mu_e_mu_b = physis_core::artifact::ArtifactId::from_hex(
            "2297f4ce64d7c1bd8e9ebdfde769d13acfd03f4334913adcc49a57346b1bbcd8",
        )
        .expect("pinned mu_e_muB node");
        assert_eq!(
            lab2.store.get(mu_e_mu_b).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let mu_e_mu_n = physis_core::artifact::ArtifactId::from_hex(
            "fe37bac9de51edecd6c7fbca4718fe5995cbef58e829b91a03f2875e284db9c0",
        )
        .expect("pinned mu_e_muN node");
        assert_eq!(
            lab2.store.get(mu_e_mu_n).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let ae = physis_core::artifact::ArtifactId::from_hex(
            "7ca6857af40ac6cf8f3b25125278adbff8732302c4ef9e8b4eb0889087f312bb",
        )
        .expect("pinned ae node");
        assert_eq!(
            lab2.store.get(ae).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let ge = physis_core::artifact::ArtifactId::from_hex(
            "98a79140e37ef1b8e6df0de890bd7dd704c443d879935fdcd62df8aa232540c1",
        )
        .expect("pinned ge node");
        assert_eq!(
            lab2.store.get(ge).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let mu_e_mmu = physis_core::artifact::ArtifactId::from_hex(
            "12906f3612b3e923097deac331dfecbe0a8b7a03cf9232065aa0a3408a47b1b6",
        )
        .expect("pinned mu_e_mmu node");
        assert_eq!(
            lab2.store.get(mu_e_mmu).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let mu_e_mup = physis_core::artifact::ArtifactId::from_hex(
            "c5b40558043871b42fac243c16485e1fec42d13d48622fe406ce1a65b33a8a3e",
        )
        .expect("pinned mu_e_mup node");
        assert_eq!(
            lab2.store.get(mu_e_mup).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let mu_e_mu0p = physis_core::artifact::ArtifactId::from_hex(
            "444c8953846cb45fe6790497b60c5dc1050cb39edc0f55d4f7c122a26e1d2279",
        )
        .expect("pinned mu_e_mu0p node");
        assert_eq!(
            lab2.store.get(mu_e_mu0p).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let mu_e_mun = physis_core::artifact::ArtifactId::from_hex(
            "aee3c0c42e091e2c5f26b3d9466846186e6d1e70693c4c67deabf9f3a09bc4dc",
        )
        .expect("pinned mu_e_mun node");
        assert_eq!(
            lab2.store.get(mu_e_mun).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let mu_e_mud = physis_core::artifact::ArtifactId::from_hex(
            "7a29b2b885a9c1ec2491ac30d0f7408fc89c2d7319e3bb511ab7a3892fef4d33",
        )
        .expect("pinned mu_e_mud node");
        assert_eq!(
            lab2.store.get(mu_e_mud).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let mu_e_mu0h = physis_core::artifact::ArtifactId::from_hex(
            "f6b2ab92d421f6139a457f76b4898616573c38cef1e29d29941e0eb41c795e30",
        )
        .expect("pinned mu_e_mu0h node");
        assert_eq!(
            lab2.store.get(mu_e_mu0h).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let m_mu = physis_core::artifact::ArtifactId::from_hex(
            "3cf58d635727710c293a539a68c0bce2aeadc9d41fa8a8dd43c238dfa58ad890",
        )
        .expect("pinned m_mu node");
        assert_eq!(
            lab2.store.get(m_mu).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let m_mu_u = physis_core::artifact::ArtifactId::from_hex(
            "d9dd36e1db3fe1aa782b3cfb99db87ba10250a4f0d945607d0cfa0ad6b163b78",
        )
        .expect("pinned m_mu_u node");
        assert_eq!(
            lab2.store.get(m_mu_u).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let m_mu_c2 = physis_core::artifact::ArtifactId::from_hex(
            "a451ddc9cfd85f74fc32ddaa156c25b2d60003cac9c3a2c7c60b17d3c2a2544a",
        )
        .expect("pinned m_mu_c2 node");
        assert_eq!(
            lab2.store.get(m_mu_c2).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let m_mu_c2_mev = physis_core::artifact::ArtifactId::from_hex(
            "b0d03e5dcc8f9174cfebf4d35d2ad0ab0836c6cde6d615cbdc21dd4e720d5dd4",
        )
        .expect("pinned m_mu_c2_MeV node");
        assert_eq!(
            lab2.store.get(m_mu_c2_mev).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let mmu_me = physis_core::artifact::ArtifactId::from_hex(
            "6c0240f5d6812b9ed27e687e73892c43959dc1cac8c2697cd116f4c116f06c74",
        )
        .expect("pinned mmu_me node");
        assert_eq!(
            lab2.store.get(mmu_me).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let mmu_mp = physis_core::artifact::ArtifactId::from_hex(
            "71390580bc3deae9e30e53f10b832a03e64680879ce3f0f076695e3f41024915",
        )
        .expect("pinned mmu_mp node");
        assert_eq!(
            lab2.store.get(mmu_mp).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let mmu_mn = physis_core::artifact::ArtifactId::from_hex(
            "4f08dabdf90b0a433f93c5cd653e9d3c5f2f7b02145130049391199780b02e63",
        )
        .expect("pinned mmu_mn node");
        assert_eq!(
            lab2.store.get(mmu_mn).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let m_mu_molar = physis_core::artifact::ArtifactId::from_hex(
            "4fb4263b7231f6a7d4fe1a73eba8b05adeaf94a12ccd7caccc4eb2ff4d0c6a07",
        )
        .expect("pinned M_mu node");
        assert_eq!(
            lab2.store.get(m_mu_molar).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let lambda_c_mu = physis_core::artifact::ArtifactId::from_hex(
            "7927f45a9a1a6944fde9bc270c82a4a5014224d39fbfbf9802d3b5505556d4ec",
        )
        .expect("pinned lambda_C_mu node");
        assert_eq!(
            lab2.store.get(lambda_c_mu).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let mu_mu = physis_core::artifact::ArtifactId::from_hex(
            "dbdd3710096e4ae7679dc5238012cec06c3ecc8242e3727978c5c15023b4abb8",
        )
        .expect("pinned mu_mu node");
        assert_eq!(
            lab2.store.get(mu_mu).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let mu_mu_mu_b = physis_core::artifact::ArtifactId::from_hex(
            "598222a0188296e5619983efb3343cb0841a482f073854780655991ddbbc57e7",
        )
        .expect("pinned mu_mu_muB node");
        assert_eq!(
            lab2.store.get(mu_mu_mu_b).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let mu_mu_mu_n = physis_core::artifact::ArtifactId::from_hex(
            "a2ee12b91517cfe4a6ab8f310e7361b9adb4877f0f69643d9e7f31d983777854",
        )
        .expect("pinned mu_mu_muN node");
        assert_eq!(
            lab2.store.get(mu_mu_mu_n).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let amu = physis_core::artifact::ArtifactId::from_hex(
            "8eeb79c20e0bd0766d4afb682a885a006a867378e21dced5d1b485c9a52b06ad",
        )
        .expect("pinned amu node");
        assert_eq!(
            lab2.store.get(amu).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let crinf = physis_core::artifact::ArtifactId::from_hex(
            "8fca9d435d8a31d1fafdac9a8825ce7f1535bf04eaf82785a1c62f66c900e60e",
        )
        .expect("pinned cRinf node");
        assert_eq!(
            lab2.store.get(crinf).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let hcrinf = physis_core::artifact::ArtifactId::from_hex(
            "f7c095d695e231cfaee92b74cd8eb2961462727d1068401ee84953d069af4cbd",
        )
        .expect("pinned hcRinf node");
        assert_eq!(
            lab2.store.get(hcrinf).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );

        let forged_hex = "0".repeat(64);
        let tampered =
            format!(r#"{{"event":"constant","t":1,"name":"","node_hash":"{forged_hex}"}}"#);
        let mut lab3 = Lab::standard();
        *lab3.journal_mut() = Journal::from_jsonl(&tampered);
        lab3.restore_from_journal();
        assert_eq!(
            lab3.store.get(live).map(|n| n.kind),
            Some(NodeKind::VersionedConstant)
        );
        let forged = physis_core::artifact::ArtifactId::from_hex(&forged_hex)
            .expect("64 hex zeros is an ArtifactId");
        assert!(lab3.store.get(forged).is_none());
    }

    #[test]
    fn encoding_auditor_round_trips_ir_packages_and_cannot_review() {
        let mut lab = Lab::standard();
        lab.set_role(Role::Explorer);
        let blocked = lab.exec(Command::Encode {
            theory: "combinational-circuit".into(),
        });
        assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
        assert!(
            blocked.text().contains("explorer cannot encode"),
            "{}",
            blocked.text()
        );

        lab.set_role(Role::Reviewer);
        let blocked_rev = lab.exec(Command::Encode {
            theory: "combinational-circuit".into(),
        });
        assert!(
            blocked_rev.text().contains("reviewer cannot encode"),
            "{}",
            blocked_rev.text()
        );

        lab.set_role(Role::Formalizer);
        let blocked_f = lab.exec(Command::Encode {
            theory: "combinational-circuit".into(),
        });
        assert!(
            blocked_f.text().contains("formalizer cannot encode"),
            "{}",
            blocked_f.text()
        );

        lab.set_role(Role::EncodingAuditor);
        let review = lab.exec(Command::Review {
            claim: "dec.d-squared-zero".into(),
        });
        assert!(
            review.text().contains("encoding-auditor cannot review"),
            "{}",
            review.text()
        );
        let prove = lab.exec(Command::Prove {
            claim: "dec.d-squared-zero".into(),
        });
        assert!(
            prove.text().contains("encoding-auditor cannot prove"),
            "{}",
            prove.text()
        );
        let cite = lab.exec(Command::Cite {
            claim: "gut.proton-lifetime-sk".into(),
        });
        assert!(
            cite.text().contains("encoding-auditor cannot cite"),
            "{}",
            cite.text()
        );

        let nand = lab
            .exec(Command::Encode {
                theory: "combinational-circuit".into(),
            })
            .text()
            .to_string();
        assert!(nand.contains("equations  1"), "{nand}");
        assert!(nand.contains("round-trip canonical"), "{nand}");
        assert!(nand.contains("reconstruct  ok"), "{nand}");
        assert!(nand.contains("not P3S"), "{nand}");
        assert!(!nand.contains("receipt"), "{nand}");
        let nand_id = encoding_package_id(&nand);
        assert_eq!(
            nand_id.to_hex(),
            "762aa72d9eace0c61026eca6ebf71b37f26608797a6786c60b92ba06af4ad8ea"
        );
        assert_eq!(
            lab.store.get(nand_id).map(|n| n.kind),
            Some(NodeKind::EncodingPackage)
        );

        let kg = lab
            .exec(Command::Encode {
                theory: "klein-gordon".into(),
            })
            .text()
            .to_string();
        assert!(kg.contains("equations  1"), "{kg}");
        assert!(
            kg.contains("laplacian nn") || kg.contains("round-trip"),
            "{kg}"
        );
        assert!(kg.contains("not P3S"), "{kg}");
        assert!(!kg.contains("receipt"), "{kg}");
        let kg_id = encoding_package_id(&kg);
        assert_eq!(
            kg_id.to_hex(),
            "32b0997d38afb977615e8fc6527ee5d766271e8a31fb5c882912ca740a3b4e4f"
        );
        assert_ne!(nand_id, kg_id);

        let u1 = lab
            .exec(Command::Encode {
                theory: "wilson-u1".into(),
            })
            .text()
            .to_string();
        assert!(u1.contains("equations  1"), "{u1}");
        assert!(u1.contains("round-trip canonical"), "{u1}");
        assert!(u1.contains("not P3S"), "{u1}");
        assert!(!u1.contains("receipt"), "{u1}");
        let u1_id = encoding_package_id(&u1);
        assert_eq!(
            u1_id.to_hex(),
            "d9644435e8775eeb95d5e81638ad61a589686d65ff6929caf0ec3c2769d4423a"
        );
        assert_ne!(u1_id, nand_id);
        assert_ne!(u1_id, kg_id);

        let su2 = lab
            .exec(Command::Encode {
                theory: "wilson-su2".into(),
            })
            .text()
            .to_string();
        assert!(su2.contains("equations  1"), "{su2}");
        assert!(su2.contains("round-trip canonical"), "{su2}");
        assert!(su2.contains("not P3S"), "{su2}");
        assert!(!su2.contains("receipt"), "{su2}");
        let su2_id = encoding_package_id(&su2);
        assert_eq!(
            su2_id.to_hex(),
            "32f36c4b5c3dc442b1c1fa970c1949c12fd0601b640f6c784d2317fcb742897a"
        );
        assert_ne!(su2_id, u1_id);

        let su3 = lab
            .exec(Command::Encode {
                theory: "wilson-su3".into(),
            })
            .text()
            .to_string();
        assert!(su3.contains("equations  1"), "{su3}");
        assert!(su3.contains("round-trip canonical"), "{su3}");
        assert!(su3.contains("not P3S"), "{su3}");
        assert!(!su3.contains("receipt"), "{su3}");
        let su3_id = encoding_package_id(&su3);
        assert_eq!(
            su3_id.to_hex(),
            "03bd82af34a6e36ee04985c243a0e2a35ab9fe56a1b28d3ad0bb63ea8461d8d3"
        );
        assert_ne!(su3_id, su2_id);
        assert_ne!(su3_id, u1_id);

        let ohm = lab
            .exec(Command::Encode {
                theory: "ohm-circuit".into(),
            })
            .text()
            .to_string();
        assert!(ohm.contains("equations  1"), "{ohm}");
        assert!(ohm.contains("round-trip canonical"), "{ohm}");
        assert!(ohm.contains("not P3S"), "{ohm}");
        assert!(!ohm.contains("receipt"), "{ohm}");
        let ohm_id = encoding_package_id(&ohm);
        assert_eq!(
            ohm_id.to_hex(),
            "fb14d2c8a8cf2c51fe67c2f334a9307860c6ebb5cfbeca1c35467d61f1387af1"
        );
        assert_ne!(ohm_id, nand_id);
        assert_ne!(ohm_id, u1_id);

        let bell = lab
            .exec(Command::Encode {
                theory: "bell-test".into(),
            })
            .text()
            .to_string();
        assert!(bell.contains("equations  1"), "{bell}");
        assert!(bell.contains("round-trip canonical"), "{bell}");
        assert!(bell.contains("not P3S"), "{bell}");
        assert!(!bell.contains("receipt"), "{bell}");
        let bell_id = encoding_package_id(&bell);
        assert_eq!(
            bell_id.to_hex(),
            "4a54aa1db88b053ef04a53593732c435331a71dcc0f8ad3749e7cbb6786990dc"
        );
        assert_ne!(bell_id, ohm_id);
        assert_ne!(bell_id, nand_id);

        let newton = lab
            .exec(Command::Encode {
                theory: "newtonian-gravity".into(),
            })
            .text()
            .to_string();
        assert!(newton.contains("equations  1"), "{newton}");
        assert!(newton.contains("round-trip canonical"), "{newton}");
        assert!(newton.contains("not P3S"), "{newton}");
        assert!(!newton.contains("receipt"), "{newton}");
        let newton_id = encoding_package_id(&newton);
        assert_eq!(
            newton_id.to_hex(),
            "e6e7c4222c571adcf6f526a27ab5e0572fb41d92361c7f3ce393e71e23184078"
        );
        assert_ne!(newton_id, bell_id);
        assert_ne!(newton_id, nand_id);

        let medium = lab
            .exec(Command::Encode {
                theory: "linear-medium".into(),
            })
            .text()
            .to_string();
        assert!(medium.contains("equations  1"), "{medium}");
        assert!(medium.contains("round-trip canonical"), "{medium}");
        assert!(medium.contains("not P3S"), "{medium}");
        assert!(!medium.contains("receipt"), "{medium}");
        let medium_id = encoding_package_id(&medium);
        assert_eq!(
            medium_id.to_hex(),
            "35df991eb0911875613084efff07327ed6821b5580bfbccb85dd08387c3722eb"
        );
        assert_ne!(medium_id, ohm_id);
        assert_ne!(medium_id, nand_id);

        let maxwell = lab
            .exec(Command::Encode {
                theory: "maxwell-vacuum".into(),
            })
            .text()
            .to_string();
        assert!(maxwell.contains("equations  1"), "{maxwell}");
        assert!(maxwell.contains("round-trip canonical"), "{maxwell}");
        assert!(maxwell.contains("not P3S"), "{maxwell}");
        assert!(!maxwell.contains("receipt"), "{maxwell}");
        let maxwell_id = encoding_package_id(&maxwell);
        assert_eq!(
            maxwell_id.to_hex(),
            "f6f47f600c798018d8cea30121512950f0066f56406aa7be34575f4fae034cc3"
        );
        assert_ne!(maxwell_id, medium_id);
        assert_ne!(maxwell_id, nand_id);

        let gas = lab
            .exec(Command::Encode {
                theory: "ideal-gas".into(),
            })
            .text()
            .to_string();
        assert!(gas.contains("equations  1"), "{gas}");
        assert!(gas.contains("round-trip canonical"), "{gas}");
        assert!(gas.contains("not P3S"), "{gas}");
        assert!(!gas.contains("receipt"), "{gas}");
        let gas_id = encoding_package_id(&gas);
        assert_eq!(
            gas_id.to_hex(),
            "fb1dbc123bf6f00bc62cb49b4ba5df49a6b22aba81c6d9434e817c714ea18e06"
        );
        assert_ne!(gas_id, maxwell_id);
        assert_ne!(gas_id, nand_id);

        let landauer = lab
            .exec(Command::Encode {
                theory: "landauer-engine".into(),
            })
            .text()
            .to_string();
        assert!(landauer.contains("equations  1"), "{landauer}");
        assert!(landauer.contains("round-trip canonical"), "{landauer}");
        assert!(landauer.contains("not P3S"), "{landauer}");
        assert!(!landauer.contains("receipt"), "{landauer}");
        let landauer_id = encoding_package_id(&landauer);
        assert_eq!(
            landauer_id.to_hex(),
            "94e8b44c1e141f6e4cbff91a409b805361e5fe00a925121348b62cdbc3e187a9"
        );
        assert_ne!(landauer_id, gas_id);
        assert_ne!(landauer_id, nand_id);

        let dirac = lab
            .exec(Command::Encode {
                theory: "dirac-fermion".into(),
            })
            .text()
            .to_string();
        assert!(dirac.contains("equations  1"), "{dirac}");
        assert!(dirac.contains("round-trip canonical"), "{dirac}");
        assert!(dirac.contains("not P3S"), "{dirac}");
        assert!(!dirac.contains("receipt"), "{dirac}");
        let dirac_id = encoding_package_id(&dirac);
        assert_eq!(
            dirac_id.to_hex(),
            "62ea25b78eaf5a7d934db096943e401135acf490c4594fc8a0621478581a521a"
        );
        assert_ne!(dirac_id, landauer_id);
        assert_ne!(dirac_id, nand_id);

        let gr = lab
            .exec(Command::Encode {
                theory: "general-relativity".into(),
            })
            .text()
            .to_string();
        assert!(gr.contains("equations  1"), "{gr}");
        assert!(gr.contains("round-trip canonical"), "{gr}");
        assert!(gr.contains("not P3S"), "{gr}");
        assert!(!gr.contains("receipt"), "{gr}");
        assert!(
            !gr.contains("catalog identity tree"),
            "Einstein-Hilbert must skip the catalog tree: {gr}"
        );
        let gr_id = encoding_package_id(&gr);
        assert_eq!(
            gr_id.to_hex(),
            "8e99553456fa93c2774e4021eb87bb4dd0547f457cf549ec4bf11859313f7be0"
        );
        assert_ne!(gr_id, dirac_id);
        assert_ne!(gr_id, newton_id);

        let sr = lab
            .exec(Command::Encode {
                theory: "special-relativity".into(),
            })
            .text()
            .to_string();
        assert!(sr.contains("equations  4"), "{sr}");
        assert!(sr.contains("claims     3"), "{sr}");
        assert!(sr.contains("round-trip canonical"), "{sr}");
        assert!(
            sr.contains("catalog identity tree  sr.invariant-interval"),
            "{sr}"
        );
        assert!(
            sr.contains("catalog identity tree  sr.subluminal-composition"),
            "{sr}"
        );
        assert!(
            sr.contains("catalog identity tree  sr.energy-momentum-invariant"),
            "{sr}"
        );
        assert!(!sr.contains("catalog identity tree  ok"), "{sr}");
        assert!(sr.contains("not P3S"), "{sr}");
        assert!(!sr.contains("receipt"), "{sr}");
        let sr_id = encoding_package_id(&sr);
        assert_eq!(
            sr_id.to_hex(),
            "faecac5791ad5650337c61dcb10e45d5eb36ca24c0423df51891673ba3da3ef6"
        );
        assert_ne!(sr_id, gr_id);
        assert_ne!(sr_id, nand_id);

        let planck = lab
            .exec(Command::Encode {
                theory: "planck".into(),
            })
            .text()
            .to_string();
        assert!(planck.contains("equations  1"), "{planck}");
        assert!(planck.contains("round-trip canonical"), "{planck}");
        assert!(planck.contains("not P3S"), "{planck}");
        assert!(!planck.contains("receipt"), "{planck}");
        assert!(
            !planck.contains("catalog identity tree"),
            "token Planck-Bose must skip the catalog tree: {planck}"
        );
        let planck_id = encoding_package_id(&planck);
        assert_eq!(
            planck_id.to_hex(),
            "7f7e69662ab0960948a1dc7c965078eddda2687e31ea7eebfdc2ab93aa69807b"
        );
        assert_ne!(planck_id, sr_id);
        assert_ne!(planck_id, nand_id);

        let derham = lab
            .exec(Command::Encode {
                theory: "de-rham".into(),
            })
            .text()
            .to_string();
        assert!(derham.contains("equations  1"), "{derham}");
        assert!(derham.contains("round-trip canonical"), "{derham}");
        assert!(
            derham.contains("catalog identity tree  dec.d-squared-zero"),
            "{derham}"
        );
        assert!(!derham.contains("catalog identity tree  ok"), "{derham}");
        assert!(derham.contains("not P3S"), "{derham}");
        assert!(!derham.contains("receipt"), "{derham}");
        let derham_id = encoding_package_id(&derham);
        assert_eq!(
            derham_id.to_hex(),
            "187ee7fd592ffb31a1e5f31fea50d158f7b67bd97f6fbf292c139683445006a6"
        );
        assert_ne!(derham_id, planck_id);
        assert_ne!(derham_id, nand_id);

        let tm = lab
            .exec(Command::Encode {
                theory: "turing-machine".into(),
            })
            .text()
            .to_string();
        assert!(tm.contains("equations  1"), "{tm}");
        assert!(tm.contains("round-trip canonical"), "{tm}");
        assert!(tm.contains("not P3S"), "{tm}");
        assert!(!tm.contains("receipt"), "{tm}");
        let tm_id = encoding_package_id(&tm);
        assert_eq!(
            tm_id.to_hex(),
            "63961d0b197deadfeb9fbbbfbf8c7c4b27f5d83a29e5e7bc75e66dbab076332f"
        );
        assert_ne!(tm_id, derham_id);
        assert_ne!(tm_id, nand_id);

        let olbers = lab
            .exec(Command::Encode {
                theory: "olbers-static".into(),
            })
            .text()
            .to_string();
        assert!(olbers.contains("equations  1"), "{olbers}");
        assert!(olbers.contains("round-trip canonical"), "{olbers}");
        assert!(olbers.contains("not P3S"), "{olbers}");
        assert!(!olbers.contains("receipt"), "{olbers}");
        let olbers_id = encoding_package_id(&olbers);
        assert_eq!(
            olbers_id.to_hex(),
            "dc1ea0aa82ee79cda7ab53071e43ccb40b56c77a609fc948a8b194864994ffd2"
        );
        assert_ne!(olbers_id, tm_id);
        assert_ne!(olbers_id, nand_id);

        let gut = lab
            .exec(Command::Encode {
                theory: "su5-gut".into(),
            })
            .text()
            .to_string();
        assert!(gut.contains("equations  1"), "{gut}");
        assert!(gut.contains("round-trip canonical"), "{gut}");
        assert!(gut.contains("not P3S"), "{gut}");
        assert!(!gut.contains("receipt"), "{gut}");
        let gut_id = encoding_package_id(&gut);
        assert_eq!(
            gut_id.to_hex(),
            "fc8614b387c901cc2806fbf456e05d5221131de9cb0d5205e5e4e7ea6a10309e"
        );
        assert_ne!(gut_id, olbers_id);
        assert_ne!(gut_id, nand_id);

        let debye = lab
            .exec(Command::Encode {
                theory: "debye-solid".into(),
            })
            .text()
            .to_string();
        assert!(debye.contains("equations  1"), "{debye}");
        assert!(debye.contains("round-trip canonical"), "{debye}");
        assert!(debye.contains("not P3S"), "{debye}");
        assert!(!debye.contains("receipt"), "{debye}");
        let debye_id = encoding_package_id(&debye);
        assert_eq!(
            debye_id.to_hex(),
            "dd817e70efdc2efede016101efe3e7b88558cd95f8260b30fc9a130301892b16"
        );
        assert_ne!(debye_id, gut_id);
        assert_ne!(debye_id, nand_id);

        let sm = lab
            .exec(Command::Encode {
                theory: "standard-model".into(),
            })
            .text()
            .to_string();
        assert!(sm.contains("equations  1"), "{sm}");
        assert!(sm.contains("round-trip canonical"), "{sm}");
        assert!(sm.contains("not P3S"), "{sm}");
        assert!(!sm.contains("receipt"), "{sm}");
        let sm_id = encoding_package_id(&sm);
        assert_eq!(
            sm_id.to_hex(),
            "860f037bdf4e717007487d9539836f5201adc6d456dc475f05e2e8470781013d"
        );
        assert_ne!(sm_id, debye_id);
        assert_ne!(sm_id, nand_id);

        let og = lab
            .exec(Command::Encode {
                theory: "observer-geometry".into(),
            })
            .text()
            .to_string();
        assert!(og.contains("equations  1"), "{og}");
        assert!(og.contains("round-trip canonical"), "{og}");
        assert!(og.contains("not P3S"), "{og}");
        assert!(!og.contains("receipt"), "{og}");
        let og_id = encoding_package_id(&og);
        assert_eq!(
            og_id.to_hex(),
            "fefb1522c8782cc9e2ceee5af785cca9a3c296ee4dfc174ed65e0fd0c51fcd30"
        );
        assert_ne!(og_id, sm_id);
        assert_ne!(og_id, nand_id);

        let dulong = lab
            .exec(Command::Encode {
                theory: "dulong-petit".into(),
            })
            .text()
            .to_string();
        assert!(dulong.contains("equations  1"), "{dulong}");
        assert!(dulong.contains("round-trip canonical"), "{dulong}");
        assert!(dulong.contains("not P3S"), "{dulong}");
        assert!(!dulong.contains("receipt"), "{dulong}");
        let dulong_id = encoding_package_id(&dulong);
        assert_eq!(
            dulong_id.to_hex(),
            "82138399bbfc4f442d125df64e3bc31833ae23f11070f5c8dbd0460b4531eaea"
        );
        assert_ne!(dulong_id, og_id);
        assert_ne!(dulong_id, nand_id);

        let heterotic = lab
            .exec(Command::Encode {
                theory: "heterotic-e8e8".into(),
            })
            .text()
            .to_string();
        assert!(heterotic.contains("equations  1"), "{heterotic}");
        assert!(heterotic.contains("round-trip canonical"), "{heterotic}");
        assert!(heterotic.contains("not P3S"), "{heterotic}");
        assert!(!heterotic.contains("receipt"), "{heterotic}");
        let heterotic_id = encoding_package_id(&heterotic);
        assert_eq!(
            heterotic_id.to_hex(),
            "c6cab84980b2320e96b4393a373de44a6fbbcdb31d54d350003e7294b61a7329"
        );
        assert_ne!(heterotic_id, dulong_id);
        assert_ne!(heterotic_id, nand_id);

        let so32 = lab
            .exec(Command::Encode {
                theory: "heterotic-so32".into(),
            })
            .text()
            .to_string();
        assert!(so32.contains("equations  1"), "{so32}");
        assert!(so32.contains("round-trip canonical"), "{so32}");
        assert!(so32.contains("not P3S"), "{so32}");
        assert!(!so32.contains("receipt"), "{so32}");
        let so32_id = encoding_package_id(&so32);
        assert_eq!(
            so32_id.to_hex(),
            "8931d99fcd313e83cc90e75a76c684853912b1c31fffd279aea84a04d274e9c2"
        );
        assert_ne!(so32_id, heterotic_id);
        assert_ne!(so32_id, nand_id);

        let type_i = lab
            .exec(Command::Encode {
                theory: "type-i".into(),
            })
            .text()
            .to_string();
        assert!(type_i.contains("equations  1"), "{type_i}");
        assert!(type_i.contains("round-trip canonical"), "{type_i}");
        assert!(type_i.contains("not P3S"), "{type_i}");
        assert!(!type_i.contains("receipt"), "{type_i}");
        let type_i_id = encoding_package_id(&type_i);
        assert_eq!(
            type_i_id.to_hex(),
            "87e40657853eb6ccd781d9c69134187f055979177276e4857884741af57e114d"
        );
        assert_ne!(type_i_id, so32_id);
        assert_ne!(type_i_id, nand_id);

        for theory in [
            "type-iib",
            "rayleigh-jeans",
            "olbers-horizon",
            "einstein-solid",
        ] {
            let resp = lab.exec(Command::Encode {
                theory: theory.into(),
            });
            assert_eq!(resp.exit_code(), 1, "{theory} {}", resp.text());
            assert!(
                resp.text().contains("no IR package"),
                "{theory} {}",
                resp.text()
            );
        }

        let hypo = lab
            .exec(Command::Hypothesize {
                theory: Some("combinational-circuit".into()),
            })
            .text()
            .to_string();
        assert!(hypo.contains("add-feedback"), "{hypo}");
        assert!(hypo.contains("add-contention"), "{hypo}");
        let nand2 = lab
            .exec(Command::Encode {
                theory: "combinational-circuit".into(),
            })
            .text()
            .to_string();
        assert_eq!(
            encoding_package_id(&nand2),
            nand_id,
            "hypothesize must not install the feedback mutant"
        );

        let hypo_kg = lab
            .exec(Command::Hypothesize {
                theory: Some("klein-gordon".into()),
            })
            .text()
            .to_string();
        assert!(hypo_kg.contains("add-next-nearest"), "{hypo_kg}");
        assert!(hypo_kg.contains("add-quartic"), "{hypo_kg}");
        let kg_again = lab
            .exec(Command::Encode {
                theory: "klein-gordon".into(),
            })
            .text()
            .to_string();
        assert_eq!(
            encoding_package_id(&kg_again),
            kg_id,
            "hypothesize must not install the Klein-Gordon mutants"
        );

        let hypo_u1 = lab
            .exec(Command::Hypothesize {
                theory: Some("wilson-u1".into()),
            })
            .text()
            .to_string();
        assert!(hypo_u1.contains("add-rectangle"), "{hypo_u1}");
        assert!(hypo_u1.contains("add-higgs"), "{hypo_u1}");
        let u1_again = lab
            .exec(Command::Encode {
                theory: "wilson-u1".into(),
            })
            .text()
            .to_string();
        assert_eq!(
            encoding_package_id(&u1_again),
            u1_id,
            "hypothesize must not install the rectangle mutant"
        );

        let hypo_su2 = lab
            .exec(Command::Hypothesize {
                theory: Some("wilson-su2".into()),
            })
            .text()
            .to_string();
        assert!(hypo_su2.contains("add-rectangle"), "{hypo_su2}");
        assert!(hypo_su2.contains("add-higgs"), "{hypo_su2}");
        let su2_again = lab
            .exec(Command::Encode {
                theory: "wilson-su2".into(),
            })
            .text()
            .to_string();
        assert_eq!(
            encoding_package_id(&su2_again),
            su2_id,
            "hypothesize must not install the SU(2) rectangle mutant"
        );

        let hypo_su3 = lab
            .exec(Command::Hypothesize {
                theory: Some("wilson-su3".into()),
            })
            .text()
            .to_string();
        assert!(hypo_su3.contains("add-rectangle"), "{hypo_su3}");
        assert!(hypo_su3.contains("add-higgs"), "{hypo_su3}");
        let su3_again = lab
            .exec(Command::Encode {
                theory: "wilson-su3".into(),
            })
            .text()
            .to_string();
        assert_eq!(
            encoding_package_id(&su3_again),
            su3_id,
            "hypothesize must not install the SU(3) rectangle mutant"
        );

        let hypo_ohm = lab
            .exec(Command::Hypothesize {
                theory: Some("ohm-circuit".into()),
            })
            .text()
            .to_string();
        assert!(hypo_ohm.contains("add-tline"), "{hypo_ohm}");
        assert!(hypo_ohm.contains("add-flux"), "{hypo_ohm}");
        let ohm_again = lab
            .exec(Command::Encode {
                theory: "ohm-circuit".into(),
            })
            .text()
            .to_string();
        assert_eq!(
            encoding_package_id(&ohm_again),
            ohm_id,
            "hypothesize must not install the tline mutant"
        );

        let hypo_bell = lab
            .exec(Command::Hypothesize {
                theory: Some("bell-test".into()),
            })
            .text()
            .to_string();
        assert!(hypo_bell.contains("add-product"), "{hypo_bell}");
        assert!(hypo_bell.contains("add-pr-box"), "{hypo_bell}");
        let bell_again = lab
            .exec(Command::Encode {
                theory: "bell-test".into(),
            })
            .text()
            .to_string();
        assert_eq!(
            encoding_package_id(&bell_again),
            bell_id,
            "hypothesize must not install the product-state mutant"
        );

        let hypo_newton = lab
            .exec(Command::Hypothesize {
                theory: Some("newtonian-gravity".into()),
            })
            .text()
            .to_string();
        assert!(hypo_newton.contains("add-schwarzschild"), "{hypo_newton}");
        assert!(hypo_newton.contains("add-yukawa"), "{hypo_newton}");
        let newton_again = lab
            .exec(Command::Encode {
                theory: "newtonian-gravity".into(),
            })
            .text()
            .to_string();
        assert_eq!(
            encoding_package_id(&newton_again),
            newton_id,
            "hypothesize must not install the Schwarzschild Binet mutant"
        );

        let hypo_medium = lab
            .exec(Command::Hypothesize {
                theory: Some("linear-medium".into()),
            })
            .text()
            .to_string();
        assert!(hypo_medium.contains("add-tellegen"), "{hypo_medium}");
        assert!(hypo_medium.contains("add-chiral"), "{hypo_medium}");
        let medium_again = lab
            .exec(Command::Encode {
                theory: "linear-medium".into(),
            })
            .text()
            .to_string();
        assert_eq!(
            encoding_package_id(&medium_again),
            medium_id,
            "hypothesize must not install the Tellegen constitutive mutant"
        );

        let hypo_maxwell = lab
            .exec(Command::Hypothesize {
                theory: Some("maxwell-vacuum".into()),
            })
            .text()
            .to_string();
        assert!(hypo_maxwell.contains("add-monopole"), "{hypo_maxwell}");
        assert!(hypo_maxwell.contains("add-proca"), "{hypo_maxwell}");
        let maxwell_again = lab
            .exec(Command::Encode {
                theory: "maxwell-vacuum".into(),
            })
            .text()
            .to_string();
        assert_eq!(
            encoding_package_id(&maxwell_again),
            maxwell_id,
            "hypothesize must not install the magnetic-current mutant"
        );

        let hypo_gas = lab
            .exec(Command::Hypothesize {
                theory: Some("ideal-gas".into()),
            })
            .text()
            .to_string();
        assert!(hypo_gas.contains("add-bose"), "{hypo_gas}");
        assert!(hypo_gas.contains("add-fermi"), "{hypo_gas}");
        let gas_again = lab
            .exec(Command::Encode {
                theory: "ideal-gas".into(),
            })
            .text()
            .to_string();
        assert_eq!(
            encoding_package_id(&gas_again),
            gas_id,
            "hypothesize must not install the Bose-statistics mutant"
        );

        let hypo_landauer = lab
            .exec(Command::Hypothesize {
                theory: Some("landauer-engine".into()),
            })
            .text()
            .to_string();
        assert!(hypo_landauer.contains("add-kt"), "{hypo_landauer}");
        assert!(hypo_landauer.contains("add-demon"), "{hypo_landauer}");
        let landauer_again = lab
            .exec(Command::Encode {
                theory: "landauer-engine".into(),
            })
            .text()
            .to_string();
        assert_eq!(
            encoding_package_id(&landauer_again),
            landauer_id,
            "hypothesize must not install the dropped-ln2 mutant"
        );

        let hypo_dirac = lab
            .exec(Command::Hypothesize {
                theory: Some("dirac-fermion".into()),
            })
            .text()
            .to_string();
        assert!(hypo_dirac.contains("add-wilson"), "{hypo_dirac}");
        assert!(hypo_dirac.contains("add-next-nearest"), "{hypo_dirac}");
        let dirac_again = lab
            .exec(Command::Encode {
                theory: "dirac-fermion".into(),
            })
            .text()
            .to_string();
        assert_eq!(
            encoding_package_id(&dirac_again),
            dirac_id,
            "hypothesize must not install the Wilson-term mutant"
        );

        let hypo_gr = lab
            .exec(Command::Hypothesize {
                theory: Some("general-relativity".into()),
            })
            .text()
            .to_string();
        assert!(hypo_gr.contains("add-r-squared"), "{hypo_gr}");
        assert!(hypo_gr.contains("add-brans-dicke"), "{hypo_gr}");
        let gr_again = lab
            .exec(Command::Encode {
                theory: "general-relativity".into(),
            })
            .text()
            .to_string();
        assert_eq!(
            encoding_package_id(&gr_again),
            gr_id,
            "hypothesize must not install the R-squared mutant"
        );

        let hypo_sr = lab
            .exec(Command::Hypothesize {
                theory: Some("special-relativity".into()),
            })
            .text()
            .to_string();
        assert!(hypo_sr.contains("add-binomial-gamma"), "{hypo_sr}");
        assert!(hypo_sr.contains("add-minus-uv"), "{hypo_sr}");
        let sr_again = lab
            .exec(Command::Encode {
                theory: "special-relativity".into(),
            })
            .text()
            .to_string();
        assert_eq!(
            encoding_package_id(&sr_again),
            sr_id,
            "hypothesize must not install the binomial-gamma mutant"
        );

        let hypo_planck = lab
            .exec(Command::Hypothesize {
                theory: Some("planck".into()),
            })
            .text()
            .to_string();
        assert!(hypo_planck.contains("add-wien"), "{hypo_planck}");
        assert!(hypo_planck.contains("add-zero-point"), "{hypo_planck}");
        let planck_again = lab
            .exec(Command::Encode {
                theory: "planck".into(),
            })
            .text()
            .to_string();
        assert_eq!(
            encoding_package_id(&planck_again),
            planck_id,
            "hypothesize must not install the Wien mutant"
        );

        let hypo_derham = lab
            .exec(Command::Hypothesize {
                theory: Some("de-rham".into()),
            })
            .text()
            .to_string();
        assert!(hypo_derham.contains("add-sign-flip"), "{hypo_derham}");
        assert!(hypo_derham.contains("add-down-laplacian"), "{hypo_derham}");
        let derham_again = lab
            .exec(Command::Encode {
                theory: "de-rham".into(),
            })
            .text()
            .to_string();
        assert_eq!(
            encoding_package_id(&derham_again),
            derham_id,
            "hypothesize must not install the sign-flip mutant"
        );

        let hypo_tm = lab
            .exec(Command::Hypothesize {
                theory: Some("turing-machine".into()),
            })
            .text()
            .to_string();
        assert!(hypo_tm.contains("add-oracle"), "{hypo_tm}");
        let tm_again = lab
            .exec(Command::Encode {
                theory: "turing-machine".into(),
            })
            .text()
            .to_string();
        assert_eq!(
            encoding_package_id(&tm_again),
            tm_id,
            "hypothesize must not install the oracle mutant"
        );

        let hypo_olbers = lab
            .exec(Command::Hypothesize {
                theory: Some("olbers-static".into()),
            })
            .text()
            .to_string();
        assert!(hypo_olbers.contains("add-tired-light"), "{hypo_olbers}");
        let olbers_again = lab
            .exec(Command::Encode {
                theory: "olbers-static".into(),
            })
            .text()
            .to_string();
        assert_eq!(
            encoding_package_id(&olbers_again),
            olbers_id,
            "hypothesize must not install the tired-light mutant"
        );

        let hypo_gut = lab
            .exec(Command::Hypothesize {
                theory: Some("su5-gut".into()),
            })
            .text()
            .to_string();
        assert!(hypo_gut.contains("add-missing-10"), "{hypo_gut}");
        let gut_again = lab
            .exec(Command::Encode {
                theory: "su5-gut".into(),
            })
            .text()
            .to_string();
        assert_eq!(
            encoding_package_id(&gut_again),
            gut_id,
            "hypothesize must not install the missing-10 mutant"
        );

        let hypo_debye = lab
            .exec(Command::Hypothesize {
                theory: Some("debye-solid".into()),
            })
            .text()
            .to_string();
        assert!(hypo_debye.contains("add-2d"), "{hypo_debye}");
        let debye_again = lab
            .exec(Command::Encode {
                theory: "debye-solid".into(),
            })
            .text()
            .to_string();
        assert_eq!(
            encoding_package_id(&debye_again),
            debye_id,
            "hypothesize must not install the 2d mutant"
        );

        let hypo_sm = lab
            .exec(Command::Hypothesize {
                theory: Some("standard-model".into()),
            })
            .text()
            .to_string();
        assert!(hypo_sm.contains("add-missing-eR"), "{hypo_sm}");
        let sm_again = lab
            .exec(Command::Encode {
                theory: "standard-model".into(),
            })
            .text()
            .to_string();
        assert_eq!(
            encoding_package_id(&sm_again),
            sm_id,
            "hypothesize must not install the missing-eR mutant"
        );

        let hypo_og = lab
            .exec(Command::Hypothesize {
                theory: Some("observer-geometry".into()),
            })
            .text()
            .to_string();
        assert!(hypo_og.contains("add-missing-spin10"), "{hypo_og}");
        let og_again = lab
            .exec(Command::Encode {
                theory: "observer-geometry".into(),
            })
            .text()
            .to_string();
        assert_eq!(
            encoding_package_id(&og_again),
            og_id,
            "hypothesize must not install the missing-spin10 mutant"
        );

        let hypo_dulong = lab
            .exec(Command::Hypothesize {
                theory: Some("dulong-petit".into()),
            })
            .text()
            .to_string();
        assert!(hypo_dulong.contains("add-quartic"), "{hypo_dulong}");
        let dulong_again = lab
            .exec(Command::Encode {
                theory: "dulong-petit".into(),
            })
            .text()
            .to_string();
        assert_eq!(
            encoding_package_id(&dulong_again),
            dulong_id,
            "hypothesize must not install the quartic mutant"
        );

        let hypo_het = lab
            .exec(Command::Hypothesize {
                theory: Some("heterotic-e8e8".into()),
            })
            .text()
            .to_string();
        assert!(hypo_het.contains("add-missing-e8"), "{hypo_het}");
        let het_again = lab
            .exec(Command::Encode {
                theory: "heterotic-e8e8".into(),
            })
            .text()
            .to_string();
        assert_eq!(
            encoding_package_id(&het_again),
            heterotic_id,
            "hypothesize must not install the missing-e8 mutant"
        );

        let hypo_so32 = lab
            .exec(Command::Hypothesize {
                theory: Some("heterotic-so32".into()),
            })
            .text()
            .to_string();
        assert!(hypo_so32.contains("add-so16"), "{hypo_so32}");
        let so32_again = lab
            .exec(Command::Encode {
                theory: "heterotic-so32".into(),
            })
            .text()
            .to_string();
        assert_eq!(
            encoding_package_id(&so32_again),
            so32_id,
            "hypothesize must not install the so16 mutant"
        );

        let hypo_type_i = lab
            .exec(Command::Hypothesize {
                theory: Some("type-i".into()),
            })
            .text()
            .to_string();
        assert!(hypo_type_i.contains("add-chan-paton-16"), "{hypo_type_i}");
        let type_i_again = lab
            .exec(Command::Encode {
                theory: "type-i".into(),
            })
            .text()
            .to_string();
        assert_eq!(
            encoding_package_id(&type_i_again),
            type_i_id,
            "hypothesize must not install the Chan-Paton mutant"
        );

        let p3s = lab
            .exec(Command::Inspect {
                axis: Some("trust".into()),
                value: Some("P3S".into()),
            })
            .text()
            .to_string();
        assert!(p3s.contains("count 0"), "encode must not raise P3S: {p3s}");
        let p3n = lab
            .exec(Command::Inspect {
                axis: Some("trust".into()),
                value: Some("P3N".into()),
            })
            .text()
            .to_string();
        assert!(p3n.contains("count 4"), "encode must not mint P3N: {p3n}");

        let why = lab
            .exec(Command::Why {
                claim: "comp.acyclic".into(),
            })
            .text()
            .to_string();
        assert!(why.contains(&format!("encoding:    {nand_id}")), "{why}");
        assert!(!why.contains("P3S"), "{why}");

        let why_d2 = lab
            .exec(Command::Why {
                claim: "dec.d-squared-zero".into(),
            })
            .text()
            .to_string();
        assert!(
            why_d2.contains(&format!("encoding:    {derham_id}")),
            "{why_d2}"
        );
        assert!(!derham.contains("receipt"), "{derham}");
        let d2b = why_theory_block(&why_d2, "de-rham");
        assert!(
            d2b.contains("oriented 2-simplex coboundary over Z"),
            "encode must not change the catalog domain: {d2b}"
        );

        let ev = lab
            .exec(Command::Evidence {
                claim: "predictivity.unique-vacuum".into(),
            })
            .text()
            .to_string();
        assert_eq!(
            evidence_graph_id(&ev).to_hex(),
            "6ee50cdc3de02838465b178b47061d8d5b36d6c135baf40f80988ff640a36bc9",
            "encoding round-trip must not change the unique-vacuum evidence payload"
        );
    }

    #[test]
    fn encoding_package_restores_by_rebuild_not_deserialize() {
        let mut lab1 = Lab::standard();
        let first = lab1
            .exec(Command::Encode {
                theory: "combinational-circuit".into(),
            })
            .text()
            .to_string();
        let live = encoding_package_id(&first);
        assert_eq!(
            live.to_hex(),
            "762aa72d9eace0c61026eca6ebf71b37f26608797a6786c60b92ba06af4ad8ea",
            "journaling must not change the combinational-circuit package payload"
        );
        let jsonl = lab1.journal().to_string();
        assert!(jsonl.contains("\"event\":\"encode\""), "{jsonl}");
        assert!(
            jsonl.contains(&format!("\"package_hash\":\"{}\"", live.to_hex())),
            "{jsonl}"
        );

        let mut lab2 = Lab::standard();
        assert_eq!(
            lab2.store
                .iter()
                .filter(|n| n.kind == NodeKind::EncodingPackage)
                .count(),
            0
        );
        *lab2.journal_mut() = Journal::from_jsonl(&jsonl);
        assert_eq!(
            lab2.store
                .iter()
                .filter(|n| n.kind == NodeKind::EncodingPackage)
                .count(),
            0,
            "from_jsonl must not insert EncodingPackage"
        );
        let journal_len = lab2.journal().len();
        lab2.restore_from_journal();
        assert_eq!(
            lab2.journal().len(),
            journal_len,
            "restore must not journal encode again"
        );
        assert_eq!(
            lab2.store.get(live).map(|n| n.kind),
            Some(NodeKind::EncodingPackage),
            "restore rebuilds the live package"
        );

        let forged_hex = "0".repeat(64);
        let tampered = format!(
            r#"{{"event":"encode","t":1,"theory":"combinational-circuit","package_hash":"{forged_hex}"}}"#
        );
        let mut lab3 = Lab::standard();
        *lab3.journal_mut() = Journal::from_jsonl(&tampered);
        lab3.restore_from_journal();
        assert_eq!(
            lab3.store.get(live).map(|n| n.kind),
            Some(NodeKind::EncodingPackage),
            "tampered package_hash is not the DAG"
        );
        let forged = physis_core::artifact::ArtifactId::from_hex(&forged_hex)
            .expect("64 hex zeros is an ArtifactId");
        assert!(
            lab3.store.get(forged).is_none(),
            "a forged hash cannot mint the package"
        );
        assert_eq!(lab3.journal().len(), 1, "tampered restore must not append");

        let why = lab2
            .exec(Command::Why {
                claim: "comp.acyclic".into(),
            })
            .text()
            .to_string();
        assert!(why.contains(&format!("encoding:    {live}")), "{why}");
    }

    #[test]
    fn judge_projects_from_lab_and_cannot_mint_proved() {
        let mut lab = Lab::standard();
        lab.set_role(Role::Explorer);
        let blocked = lab.exec(Command::Judge {
            claim: "predictivity.unique-vacuum".into(),
        });
        assert_eq!(blocked.exit_code(), 1, "{}", blocked.text());
        assert!(
            blocked.text().contains("explorer cannot judge"),
            "{}",
            blocked.text()
        );

        lab.set_role(Role::EncodingAuditor);
        let blocked_enc = lab.exec(Command::Judge {
            claim: "predictivity.unique-vacuum".into(),
        });
        assert!(
            blocked_enc.text().contains("encoding-auditor cannot judge"),
            "{}",
            blocked_enc.text()
        );

        lab.set_role(Role::Judge);
        let prove = lab.exec(Command::Prove {
            claim: "dec.d-squared-zero".into(),
        });
        assert!(
            prove.text().contains("judge cannot prove"),
            "{}",
            prove.text()
        );

        let before = lab
            .exec(Command::Why {
                claim: "predictivity.unique-vacuum".into(),
            })
            .text()
            .to_string();
        assert!(
            before.contains("none (judgment is not an independent from_lab rebuild)"),
            "judge is a unique op, not an observe print:\n{before}"
        );

        let uniq = lab
            .exec(Command::Judge {
                claim: "predictivity.unique-vacuum".into(),
            })
            .text()
            .to_string();
        assert!(uniq.contains("heuristic failed"), "{uniq}");
        assert!(!uniq.contains("logical proved"), "{uniq}");
        assert!(uniq.contains("not P3S"), "{uniq}");
        assert!(!uniq.contains("receipt"), "{uniq}");
        let uniq_id = judgment_projection_id(&uniq);
        assert_eq!(
            uniq_id.to_hex(),
            "0dadce8d7bfc005efc32e47917f75b4c17ea77900ec9f6592010fd81f0f1ea76"
        );
        assert_eq!(
            lab.store.get(uniq_id).map(|n| n.kind),
            Some(NodeKind::JudgmentProjection)
        );

        let sk = lab
            .exec(Command::Judge {
                claim: "gut.proton-lifetime-sk".into(),
            })
            .text()
            .to_string();
        assert!(sk.contains("empirical excluded"), "{sk}");
        assert!(!sk.contains("logical proved"), "{sk}");

        let pdg = lab
            .exec(Command::Judge {
                claim: "gut.weinberg-angle-mz-interval".into(),
            })
            .text()
            .to_string();
        assert!(pdg.contains("statistical computed"), "{pdg}");

        let gut = lab
            .exec(Command::Judge {
                claim: "gut.weinberg-angle".into(),
            })
            .text()
            .to_string();
        assert!(gut.contains("numeric certified"), "{gut}");

        let poincare = lab
            .exec(Command::Judge {
                claim: "dec.closed-equals-exact".into(),
            })
            .text()
            .to_string();
        assert!(poincare.contains("logical undetermined"), "{poincare}");
        assert!(!poincare.contains("logical proved"), "{poincare}");

        let d2 = lab
            .exec(Command::Judge {
                claim: "dec.d-squared-zero".into(),
            })
            .text()
            .to_string();
        assert!(d2.contains("logical undetermined"), "{d2}");
        assert!(!d2.contains("logical proved"), "{d2}");

        lab.set_role(Role::Lab);
        let _ = lab.exec(Command::Prove {
            claim: "dec.d-squared-zero".into(),
        });
        lab.set_role(Role::Judge);
        let d2p = lab
            .exec(Command::Judge {
                claim: "dec.d-squared-zero".into(),
            })
            .text()
            .to_string();
        assert!(d2p.contains("logical proved"), "{d2p}");
        assert!(!d2p.contains("receipt"), "{d2p}");

        let p3s = lab
            .exec(Command::Inspect {
                axis: Some("trust".into()),
                value: Some("P3S".into()),
            })
            .text()
            .to_string();
        assert!(p3s.contains("count 0"), "judge must not raise P3S: {p3s}");
        let p3n = lab
            .exec(Command::Inspect {
                axis: Some("trust".into()),
                value: Some("P3N".into()),
            })
            .text()
            .to_string();
        assert!(p3n.contains("count 4"), "judge must not mint P3N: {p3n}");

        let why = lab
            .exec(Command::Why {
                claim: "predictivity.unique-vacuum".into(),
            })
            .text()
            .to_string();
        assert!(why.contains("projection:  "), "{why}");
        assert!(
            !why.contains("none (judgment is not an independent from_lab rebuild)"),
            "live from_lab rebuild should bind a projection id:\n{why}"
        );
        assert!(why.contains("heuristic failed"), "{why}");
        assert!(!why.contains("logical proved"), "{why}");

        let ev = lab
            .exec(Command::Evidence {
                claim: "predictivity.unique-vacuum".into(),
            })
            .text()
            .to_string();
        assert_eq!(
            evidence_graph_id(&ev).to_hex(),
            "6ee50cdc3de02838465b178b47061d8d5b36d6c135baf40f80988ff640a36bc9",
            "from_lab projection must not change the unique-vacuum evidence payload"
        );
    }

    #[test]
    fn judgment_projection_restores_by_rebuild_not_deserialize() {
        let mut lab1 = Lab::standard();
        let first = lab1
            .exec(Command::Judge {
                claim: "gut.weinberg-angle".into(),
            })
            .text()
            .to_string();
        let live = judgment_projection_id(&first);
        assert_eq!(
            live.to_hex(),
            "40c991698dbff52a5614093b98edcc3478a3702ddcb5cc545f9818af4a6448ae",
            "journaling must not change the GUT-scale 3/8 projection payload"
        );
        let jsonl = lab1.journal().to_string();
        assert!(jsonl.contains("\"event\":\"judge\""), "{jsonl}");
        assert!(
            jsonl.contains(&format!("\"projection_hash\":\"{}\"", live.to_hex())),
            "{jsonl}"
        );
        assert!(!jsonl.contains("logical proved"), "{jsonl}");

        let mut lab2 = Lab::standard();
        *lab2.journal_mut() = Journal::from_jsonl(&jsonl);
        let journal_len = lab2.journal().len();
        lab2.restore_from_journal();
        assert_eq!(lab2.journal().len(), journal_len);
        assert_eq!(
            lab2.store.get(live).map(|n| n.kind),
            Some(NodeKind::JudgmentProjection)
        );

        let forged_hex = "0".repeat(64);
        let tampered = format!(
            r#"{{"event":"judge","t":1,"claim":"gut.weinberg-angle","projection_hash":"{forged_hex}"}}"#
        );
        let mut lab3 = Lab::standard();
        *lab3.journal_mut() = Journal::from_jsonl(&tampered);
        lab3.restore_from_journal();
        assert_eq!(
            lab3.store.get(live).map(|n| n.kind),
            Some(NodeKind::JudgmentProjection),
            "tampered projection_hash is not the DAG"
        );
        let forged = physis_core::artifact::ArtifactId::from_hex(&forged_hex)
            .expect("64 hex zeros is an ArtifactId");
        assert!(lab3.store.get(forged).is_none());

        let why = lab2
            .exec(Command::Why {
                claim: "gut.weinberg-angle".into(),
            })
            .text()
            .to_string();
        assert!(why.contains(&format!("projection:  {live}")), "{why}");
        assert!(why.contains("numeric certified"), "{why}");
        assert!(!why.contains("logical proved"), "{why}");
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
        assert!(
            text.contains("constant  ledger  044a027898acd4fbe72cfb6f012d248e24f95be834da6c9f5598cabc268a52c1"),
            "a zero prove budget must not skip the constants ledger: {text}"
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
