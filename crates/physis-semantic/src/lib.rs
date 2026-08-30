//! Encoding-review promotions. [`SemanticAssurance`] is not an enum an agent
//! can set.
//!
//! The only public mint is [`review`]: it looks up a trusted dossier for the
//! catalog FormalClaim (not merely the slug), binds the record to
//! [`physis_core::formal::FormalClaim::statement_hash`],
//! builds a [`physis_provenance::SourceRecord`], parses a second encoding, and
//! *runs* the red-team corpus. [`SemanticAssurance::Canonical`] is never assigned.
//! A review of one identity is not P3S for a later identity that kept the slug,
//! and a Physlib dossier will not mint against a different FormalClaim.
//!
//! External crates cannot construct [`SemanticRecord`] by struct literal:
//!
//! ```compile_fail
//! use physis_semantic::SemanticRecord;
//! let _ = SemanticRecord {
//!     claim_id: String::new(),
//!     statement_hash: todo!(),
//!     assurance: physis_core::SemanticAssurance::Canonical,
//!     evidence_hash: todo!(),
//!     source_hash: todo!(),
//! };
//! ```
//!
//! Serde cannot mint one either:
//!
//! ```compile_fail
//! fn needs_deserialize<'de, T: serde::Deserialize<'de>>() {}
//! fn _blocked() {
//!     needs_deserialize::<physis_semantic::SemanticRecord>();
//! }
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use physis_core::artifact::ArtifactId;
use physis_core::assurance::SemanticAssurance;
use physis_core::formal::FormalClaim;
use physis_ir::parse_package;
use physis_proof::{lookup, lookup_matching, parse_expr};
use physis_provenance::{Citation, SourceLocator, SourceRecord};
use serde::Serialize;

/// Why a semantic review refused to raise the tag.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SemanticError {
    /// No trusted dossier for this claim (conjectures stay unreviewed).
    NoDossier,
    /// Source record rejected as a slogan locator.
    VagueSource(String),
    /// IR package did not parse.
    Ir(String),
    /// Second encoding is missing, copied, or not the catalog identity tree.
    Encoding(String),
    /// A dossier exists for the slug, but this FormalClaim is not that identity.
    WrongIdentity,
}

impl std::fmt::Display for SemanticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SemanticError::NoDossier => {
                write!(f, "no semantic dossier; Unreviewed is the honest default")
            }
            SemanticError::VagueSource(s) => write!(f, "source: {s}"),
            SemanticError::Ir(s) => write!(f, "ir: {s}"),
            SemanticError::Encoding(s) => write!(f, "encoding: {s}"),
            SemanticError::WrongIdentity => write!(
                f,
                "catalog identity does not match this FormalClaim; Unreviewed is the honest default"
            ),
        }
    }
}

impl std::error::Error for SemanticError {}

/// In-tree review evidence. Not agent-supplied `accepted: true`.
struct Dossier {
    claim_id: &'static str,
    work: &'static str,
    edition: &'static str,
    version: &'static str,
    section: Option<&'static str>,
    equation: Option<&'static str>,
    ir: &'static str,
}

/// Discrete coboundary: Desbrun–Hirani–Leok–Marsden, not a slogan "textbook".
const D2: Dossier = Dossier {
    claim_id: "dec.d-squared-zero",
    work: "Discrete Exterior Calculus (Desbrun, Hirani, Leok, Marsden)",
    edition: "arXiv:math/0508341v2",
    version: "2005",
    section: Some("The coboundary operator"),
    equation: Some("delta^2 = 0"),
    ir: r#"
id = discrete-d2
name = Discrete coboundary nilpotence
assumption coboundary-on-oriented-simplex
equation (b - a) - (c - a) + (c - b)
claim mathematical mathematical dec.d-squared-zero : d composed with d is the zero operator
lean_ref ∀ (a b c : Int), (b - a) - (c - a) + (c - b) = 0
"#,
};

/// Minkowski interval identity under a boost (c = 1).
const LORENTZ: Dossier = Dossier {
    claim_id: "sr.invariant-interval",
    work: "On the Electrodynamics of Moving Bodies (Einstein)",
    edition: "Annalen der Physik 17 (1905)",
    version: "1905",
    section: Some("3"),
    equation: Some("tau"),
    ir: r#"
id = lorentz-interval
name = Minkowski interval under a boost
assumption minkowski-signature
equation (t - beta * x)^2 - (x - beta * t)^2 - (1 - beta^2) * (t^2 - x^2)
claim model-internal spacetime sr.invariant-interval : the interval is boost-invariant
lean_ref ∀ (t x β : Int), (t - β*x)^2 - (x - β*t)^2 = (1 - β^2)*(t^2 - x^2)
"#,
};

/// Einstein 1905 §5: composition of velocities, polynomial content.
const COMPOSITION: Dossier = Dossier {
    claim_id: "sr.subluminal-composition",
    work: "On the Electrodynamics of Moving Bodies (Einstein)",
    edition: "Annalen der Physik 17 (1905)",
    version: "1905",
    section: Some("5"),
    equation: Some("V"),
    ir: r#"
id = einstein-composition
name = Einstein velocity addition algebraic identity
assumption einstein-velocity-addition
equation (1 + u * v)^2 - (u + v)^2 - (1 - u^2) * (1 - v^2)
claim model-internal spacetime sr.subluminal-composition : composing subluminal velocities stays below c
lean_ref ∀ (u v : Int), (1 + u*v)^2 - (u + v)^2 = (1 - u^2)*(1 - v^2)
"#,
};

/// Minkowski 1908: 4-momentum as a space-time vector of the first kind.
/// The polynomial is the interval identity on (E, p), not a new postulate.
const MASS_SHELL: Dossier = Dossier {
    claim_id: "sr.energy-momentum-invariant",
    work: "Die Grundgleichungen fuer die elektromagnetischen Vorgaenge in bewegten Koerpern (Minkowski)",
    edition: "Nachr. Ges. Wiss. Goettingen 1908",
    version: "1908",
    section: Some("space-time vectors of the first kind"),
    equation: Some("rest-mass vector"),
    ir: r#"
id = energy-momentum
name = Mass shell under a boost
assumption minkowski-signature
equation (E - beta * p)^2 - (p - beta * E)^2 - (1 - beta^2) * (E^2 - p^2)
claim model-internal particle sr.energy-momentum-invariant : the mass shell is frame-independent
lean_ref ∀ (E p β : Int), (E - β*p)^2 - (p - β*E)^2 = (1 - β^2)*(E^2 - p^2)
"#,
};

const DOSSIERS: &[Dossier] = &[D2, LORENTZ, COMPOSITION, MASS_SHELL];

fn dossier(claim_id: &str) -> Option<&'static Dossier> {
    DOSSIERS.iter().find(|d| d.claim_id == claim_id)
}

/// A justified semantic tag. Constructor is private to this crate.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct SemanticRecord {
    claim_id: String,
    statement_hash: ArtifactId,
    assurance: SemanticAssurance,
    evidence_hash: ArtifactId,
    source_hash: ArtifactId,
}

impl SemanticRecord {
    fn mint(
        claim_id: String,
        statement_hash: ArtifactId,
        assurance: SemanticAssurance,
        evidence_hash: ArtifactId,
        source_hash: ArtifactId,
    ) -> Self {
        debug_assert_ne!(assurance, SemanticAssurance::Canonical);
        debug_assert_ne!(assurance, SemanticAssurance::Unreviewed);
        Self {
            claim_id,
            statement_hash,
            assurance,
            evidence_hash,
            source_hash,
        }
    }

    /// Claim slug this record is about. Not P3S: a changed statement
    /// identity keeps the slug and must use [`Self::statement_hash`].
    pub fn claim_id(&self) -> &str {
        &self.claim_id
    }

    /// Live identity this review covers. P3S is a tag of this hash.
    pub fn statement_hash(&self) -> ArtifactId {
        self.statement_hash
    }

    /// Justified tag. Never [`SemanticAssurance::Canonical`].
    pub fn assurance(&self) -> SemanticAssurance {
        self.assurance
    }

    /// Hash of the evidence bundle (source + encodings + resulting tag).
    pub fn evidence_hash(&self) -> ArtifactId {
        self.evidence_hash
    }

    /// Hash of the locked source record.
    pub fn source_hash(&self) -> ArtifactId {
        self.source_hash
    }
}

/// Lab-wide store of semantic records. Inserts only through
/// [`SemanticStore::record`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SemanticStore {
    records: Vec<SemanticRecord>,
}

impl SemanticStore {
    /// Empty store.
    pub fn empty() -> Self {
        Self::default()
    }

    /// True when nothing has been reviewed.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Number of records.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// The only way a record enters the store. Replaces a prior record of
    /// the same statement hash; a different identity of the same slug is
    /// kept.
    pub fn record(&mut self, rec: &SemanticRecord) {
        self.records
            .retain(|r| r.statement_hash != rec.statement_hash);
        self.records.push(rec.clone());
    }

    /// Lookup by statement hash. This is P3S.
    pub fn by_statement(&self, statement_hash: ArtifactId) -> Option<&SemanticRecord> {
        self.records
            .iter()
            .rev()
            .find(|r| r.statement_hash == statement_hash)
    }

    /// Lookup by claim slug (last record wins). Not P3S: a changed
    /// statement identity keeps the slug and must use [`Self::by_statement`].
    pub fn by_claim(&self, claim_id: &str) -> Option<&SemanticRecord> {
        self.records.iter().rev().find(|r| r.claim_id == claim_id)
    }
}

/// Dual-check encodings and provenance, then (if independent) run the
/// red-team corpus. The minted record is bound to `claim.statement_hash`.
/// Never returns [`SemanticAssurance::Canonical`].
pub fn review(claim: &FormalClaim) -> Result<SemanticRecord, SemanticError> {
    let d = dossier(&claim.id().0).ok_or(SemanticError::NoDossier)?;
    if lookup_matching(claim).is_none() {
        return Err(SemanticError::WrongIdentity);
    }
    review_dossier(d, d.ir, claim)
}

fn review_dossier(
    d: &Dossier,
    ir_src: &str,
    claim: &FormalClaim,
) -> Result<SemanticRecord, SemanticError> {
    let spec = lookup(d.claim_id).ok_or(SemanticError::NoDossier)?;
    let source = SourceRecord::new(
        Citation {
            work: d.work.into(),
            edition: d.edition.into(),
        },
        d.version,
        SourceLocator {
            page: None,
            section: d.section.map(|s| s.into()),
            equation: d.equation.map(|s| s.into()),
            figure: None,
            table: None,
            dataset_range: None,
            experiment: None,
        },
        ArtifactId::of(format!("{}|{}|{}", d.work, d.edition, d.version).as_bytes()),
        None,
    )
    .map_err(|e| SemanticError::VagueSource(e.to_string()))?;

    let catalog_expr = (spec.identity)();
    let catalog_hash = ArtifactId::of(catalog_expr.canonical().as_bytes());
    let ir_hash = ArtifactId::of(ir_src.as_bytes());
    independent_ir(d.claim_id, spec.lean_type, &catalog_expr, ir_src)?;
    if catalog_hash == ir_hash {
        return Err(SemanticError::Encoding(
            "catalog identity and IR package have the same artifact hash".into(),
        ));
    }

    let mut level = SemanticAssurance::IndependentlyEncoded;
    if physis_audit::attack().is_ok() {
        level = SemanticAssurance::AdversariallyReviewed;
    }

    let evidence_hash = ArtifactId::of(
        format!(
            "claim:{}\nstatement:{}\nsource:{}\ncatalog:{}\nir:{}\nlevel:{}",
            d.claim_id,
            claim.statement_hash(),
            source.source_hash,
            catalog_hash,
            ir_hash,
            level.as_str()
        )
        .as_bytes(),
    );
    Ok(SemanticRecord::mint(
        d.claim_id.into(),
        claim.statement_hash(),
        level,
        evidence_hash,
        source.source_hash,
    ))
}

fn independent_ir(
    claim_id: &str,
    lean_type: &str,
    catalog: &physis_proof::Expr,
    ir_src: &str,
) -> Result<(), SemanticError> {
    let pkg = parse_package(ir_src).map_err(SemanticError::Ir)?;
    let decl = pkg
        .claims
        .iter()
        .find(|c| c.id == claim_id)
        .ok_or_else(|| SemanticError::Encoding(format!("IR missing claim '{claim_id}'")))?;
    if decl.statement.trim().is_empty() {
        return Err(SemanticError::Encoding(
            "IR claim has empty statement".into(),
        ));
    }
    match pkg.lean_ref.as_deref() {
        Some(r) if r == lean_type => {}
        Some(r) => {
            return Err(SemanticError::Encoding(format!(
                "IR lean_ref {r:?} != catalog {lean_type:?}"
            )));
        }
        None => {
            return Err(SemanticError::Encoding("IR missing lean_ref".into()));
        }
    }
    let mut matched = false;
    for eq in &pkg.equations {
        let parsed = parse_expr(eq).map_err(SemanticError::Encoding)?;
        if parsed.canonical() == catalog.canonical() {
            matched = true;
            break;
        }
    }
    if !matched {
        return Err(SemanticError::Encoding(
            "IR equation is not the catalog identity tree (vacuous 0 is not d^2=0)".into(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use physis_core::assurance::{ClaimClass, SemanticAssurance};
    use physis_core::claim::Claim;
    use physis_core::id::LayerId;
    use physis_proof::{lookup, CATALOG};

    use super::*;

    fn unspecified(id: &str) -> FormalClaim {
        FormalClaim::from_claim(&Claim::new(
            id,
            "The exterior derivative is nilpotent: d ∘ d = 0.",
            LayerId::Mathematical,
            ClaimClass::Mathematical,
        ))
    }

    fn catalog_d2() -> FormalClaim {
        lookup("dec.d-squared-zero").unwrap().formal_claim()
    }

    #[test]
    fn catalog_dossiers_reach_adversarial_review() {
        for spec in CATALOG {
            let rec = review(&spec.formal_claim()).unwrap();
            assert_eq!(rec.assurance(), SemanticAssurance::AdversariallyReviewed);
            assert_ne!(rec.assurance(), SemanticAssurance::Canonical);
            assert_ne!(rec.assurance(), SemanticAssurance::Unreviewed);
            assert_eq!(rec.statement_hash(), spec.formal_claim().statement_hash());
        }
    }

    #[test]
    fn conjecture_has_no_dossier() {
        let err = review(&unspecified("predictivity.unique-vacuum")).unwrap_err();
        assert_eq!(err, SemanticError::NoDossier);
    }

    #[test]
    fn vacuous_zero_is_not_an_independent_encoding() {
        let d = dossier("dec.d-squared-zero").unwrap();
        let ir = r#"
id = fake
name = fake
equation 0
claim mathematical mathematical dec.d-squared-zero : d^2 = 0
lean_ref ∀ (a b c : Int), (b - a) - (c - a) + (c - b) = 0
"#;
        let err = review_dossier(d, ir, &catalog_d2()).unwrap_err();
        assert!(matches!(err, SemanticError::Encoding(_)), "{err:?}");
    }

    #[test]
    fn sign_flip_is_not_the_catalog_tree() {
        let d = dossier("dec.d-squared-zero").unwrap();
        let ir = r#"
id = fake
name = fake
equation (b - a) - (c - a) - (c - b)
claim mathematical mathematical dec.d-squared-zero : d^2 = 0
lean_ref ∀ (a b c : Int), (b - a) - (c - a) + (c - b) = 0
"#;
        let err = review_dossier(d, ir, &catalog_d2()).unwrap_err();
        assert!(matches!(err, SemanticError::Encoding(_)), "{err:?}");
    }

    #[test]
    fn store_only_grows_via_record() {
        let mut store = SemanticStore::empty();
        assert!(store.is_empty());
        let rec = review(&catalog_d2()).unwrap();
        store.record(&rec);
        assert_eq!(store.len(), 1);
        assert_eq!(
            store.by_claim("dec.d-squared-zero").unwrap().assurance(),
            SemanticAssurance::AdversariallyReviewed
        );
        assert_eq!(
            store
                .by_statement(rec.statement_hash())
                .unwrap()
                .assurance(),
            SemanticAssurance::AdversariallyReviewed
        );
    }

    #[test]
    fn slug_review_is_not_p3s_for_a_changed_identity() {
        let unspecified = unspecified("dec.d-squared-zero");
        let live = catalog_d2();
        assert_ne!(unspecified.statement_hash(), live.statement_hash());

        let err = review(&unspecified).unwrap_err();
        assert_eq!(err, SemanticError::WrongIdentity);

        let rec = review(&live).unwrap();
        assert_eq!(rec.statement_hash(), live.statement_hash());
        let mut store = SemanticStore::empty();
        store.record(&rec);
        assert!(store.by_statement(live.statement_hash()).is_some());
        assert!(store.by_statement(unspecified.statement_hash()).is_none());
    }
}
