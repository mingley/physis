//! Encoding-review promotions. [`SemanticAssurance`] is not an enum an agent
//! can set.
//!
//! The only public mint is [`review`]: it looks up a trusted dossier, builds a
//! [`physis_provenance::SourceRecord`], parses a second encoding, and *runs*
//! the red-team corpus. [`SemanticAssurance::Canonical`] is never assigned.
//!
//! External crates cannot construct [`SemanticRecord`] by struct literal:
//!
//! ```compile_fail
//! use physis_semantic::SemanticRecord;
//! let _ = SemanticRecord {
//!     claim_id: String::new(),
//!     assurance: physis_core::SemanticAssurance::Canonical,
//!     evidence_hash: todo!(),
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
use physis_ir::parse_package;
use physis_proof::{lookup, parse_expr};
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

const DOSSIERS: &[Dossier] = &[D2, LORENTZ, COMPOSITION];

fn dossier(claim_id: &str) -> Option<&'static Dossier> {
    DOSSIERS.iter().find(|d| d.claim_id == claim_id)
}

/// A justified semantic tag. Constructor is private to this crate.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct SemanticRecord {
    claim_id: String,
    assurance: SemanticAssurance,
    evidence_hash: ArtifactId,
    source_hash: ArtifactId,
}

impl SemanticRecord {
    fn mint(
        claim_id: String,
        assurance: SemanticAssurance,
        evidence_hash: ArtifactId,
        source_hash: ArtifactId,
    ) -> Self {
        debug_assert_ne!(assurance, SemanticAssurance::Canonical);
        debug_assert_ne!(assurance, SemanticAssurance::Unreviewed);
        Self {
            claim_id,
            assurance,
            evidence_hash,
            source_hash,
        }
    }

    /// Claim this record is about.
    pub fn claim_id(&self) -> &str {
        &self.claim_id
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

    /// The only way a record enters the store.
    pub fn record(&mut self, rec: &SemanticRecord) {
        self.records.retain(|r| r.claim_id != rec.claim_id);
        self.records.push(rec.clone());
    }

    /// Lookup by claim id.
    pub fn by_claim(&self, claim_id: &str) -> Option<&SemanticRecord> {
        self.records.iter().rev().find(|r| r.claim_id == claim_id)
    }
}

/// Dual-check encodings and provenance, then (if independent) run the
/// red-team corpus. Never returns [`SemanticAssurance::Canonical`].
pub fn review(claim_id: &str) -> Result<SemanticRecord, SemanticError> {
    let d = dossier(claim_id).ok_or(SemanticError::NoDossier)?;
    review_dossier(d, d.ir)
}

fn review_dossier(d: &Dossier, ir_src: &str) -> Result<SemanticRecord, SemanticError> {
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
            "claim:{}\nsource:{}\ncatalog:{}\nir:{}\nlevel:{}",
            d.claim_id,
            source.source_hash,
            catalog_hash,
            ir_hash,
            level.as_str()
        )
        .as_bytes(),
    );
    Ok(SemanticRecord::mint(
        d.claim_id.into(),
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
    use physis_core::assurance::SemanticAssurance;
    use physis_proof::CATALOG;

    use super::*;

    #[test]
    fn catalog_dossiers_reach_adversarial_review() {
        for spec in CATALOG {
            let rec = review(spec.claim_id).unwrap();
            assert_eq!(rec.assurance(), SemanticAssurance::AdversariallyReviewed);
            assert_ne!(rec.assurance(), SemanticAssurance::Canonical);
            assert_ne!(rec.assurance(), SemanticAssurance::Unreviewed);
        }
    }

    #[test]
    fn conjecture_has_no_dossier() {
        let err = review("predictivity.unique-vacuum").unwrap_err();
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
        let err = review_dossier(d, ir).unwrap_err();
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
        let err = review_dossier(d, ir).unwrap_err();
        assert!(matches!(err, SemanticError::Encoding(_)), "{err:?}");
    }

    #[test]
    fn store_only_grows_via_record() {
        let mut store = SemanticStore::empty();
        assert!(store.is_empty());
        store.record(&review("dec.d-squared-zero").unwrap());
        assert_eq!(store.len(), 1);
        assert_eq!(
            store.by_claim("dec.d-squared-zero").unwrap().assurance(),
            SemanticAssurance::AdversariallyReviewed
        );
    }
}
