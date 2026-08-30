//! Immutable formal claim identity.
//!
//! A proof generated for one statement must not attach to another.
//! [`FormalClaim::statement_hash`] commits to the sentence, class, layer,
//! assumptions, domain, quantifiers, units, constants, boundary conditions,
//! conventions, theory version, definitions, datasets, and formal-library
//! identity. Changing ∀ to ∃, a sign, a unit, a constant, or a boundary
//! condition yields a new content-addressed hash. The lab slug ([`ClaimId`])
//! is a stable name, not that hash.
//!
//! Constructed only by [`FormalClaim::from_claim`], which recomputes the
//! statement hash from the live sentence. JSON cannot mint an identity:
//!
//! ```compile_fail
//! use physis_core::formal::FormalClaim;
//! let _ = FormalClaim {
//!     id: physis_core::ClaimId::new("x"),
//!     statement: String::new(),
//!     statement_hash: todo!(),
//!     assumptions: todo!(),
//!     domain: todo!(),
//!     class: physis_core::ClaimClass::Mathematical,
//!     layer: physis_core::LayerId::Mathematical,
//!     commitments: physis_core::ClaimCommitments::unspecified(),
//! };
//! ```
//!
//! ```compile_fail
//! fn needs_deserialize<'de, T: serde::Deserialize<'de>>() {}
//! fn _blocked() {
//!     needs_deserialize::<physis_core::formal::FormalClaim>();
//! }
//! ```

use serde::{Deserialize, Serialize};

use crate::artifact::ArtifactId;
use crate::assumption::{AssumptionSet, AssumptionSetId, DomainOfValidity};
use crate::assurance::ClaimClass;
use crate::claim::Claim;
use crate::id::{ClaimId, LayerId};

/// How the sentence is quantified. Distinct from the English wording:
/// the same prose with [`Quantifier::ForAll`] vs [`Quantifier::Exists`]
/// is a different identity.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Quantifier {
    /// Not yet a first-class quantifier. The English statement is still
    /// hashed; filling this in later is a new identity.
    #[default]
    Unspecified,
    /// Holds for all values in the domain of validity.
    ForAll,
    /// A witness is claimed.
    Exists,
}

impl Quantifier {
    /// Stable kebab-case name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Quantifier::Unspecified => "unspecified",
            Quantifier::ForAll => "forall",
            Quantifier::Exists => "exists",
        }
    }
}

/// First-class identity fields beyond the English sentence.
///
/// Empty / unspecified entries are hashed as such. Filling them in later
/// is a new identity, not a silent annotation. The default theory version
/// is the explicit label `unversioned-encoding`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimCommitments {
    /// Quantifier. Default [`Quantifier::Unspecified`].
    pub quantifier: Quantifier,
    /// Unit strings the sentence is committed to (`1`, `m/s`).
    pub units: Vec<String>,
    /// Versioned constants (`c=1`, `c@si-2019-codata-2018`).
    pub constants: Vec<String>,
    /// Boundary conditions (`unification-scale`, `|k a| < 1`).
    pub boundary: Vec<String>,
    /// Sign and other conventions (`minkowski-mostly-minus`).
    pub conventions: Vec<String>,
    /// Theory encoding version. Default `unversioned-encoding`.
    pub theory_version: String,
    /// Named definitions (`Q = T3 + Y`).
    pub definitions: Vec<String>,
    /// Dataset ids the sentence is compared against.
    pub datasets: Vec<String>,
    /// Formal-library identity (`physlib:unversioned`). Not a Lean
    /// compiler version: that lives on the receipt.
    pub formal_libraries: Vec<String>,
}

impl Default for ClaimCommitments {
    fn default() -> Self {
        Self::unspecified()
    }
}

impl ClaimCommitments {
    /// Explicit placeholders: nothing first-class except the unversioned
    /// encoding label. Not a licence to treat the sentence as unit-free.
    pub fn unspecified() -> Self {
        Self {
            quantifier: Quantifier::Unspecified,
            units: Vec::new(),
            constants: Vec::new(),
            boundary: Vec::new(),
            conventions: Vec::new(),
            theory_version: "unversioned-encoding".into(),
            definitions: Vec::new(),
            datasets: Vec::new(),
            formal_libraries: Vec::new(),
        }
    }

    /// Universal dimensionless identity in unversioned Physlib.
    pub fn physlib_forall() -> Self {
        Self {
            quantifier: Quantifier::ForAll,
            units: vec!["1".into()],
            formal_libraries: vec!["physlib:unversioned".into()],
            ..Self::unspecified()
        }
    }

    /// Lines `why` prints for non-default fields.
    pub fn why_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        if self.quantifier != Quantifier::Unspecified {
            lines.push(format!("  quantifier: {}", self.quantifier.as_str()));
        }
        push_why_list(&mut lines, "units", &self.units);
        push_why_list(&mut lines, "constants", &self.constants);
        push_why_list(&mut lines, "boundary", &self.boundary);
        push_why_list(&mut lines, "conventions", &self.conventions);
        if self.theory_version != "unversioned-encoding" {
            lines.push(format!("  theory:     {}", self.theory_version));
        }
        push_why_list(&mut lines, "definitions", &self.definitions);
        push_why_list(&mut lines, "datasets", &self.datasets);
        push_why_list(&mut lines, "libraries", &self.formal_libraries);
        lines
    }

    fn append_canonical(&self, s: &mut String) {
        s.push_str("quantifier:");
        s.push_str(self.quantifier.as_str());
        s.push('\n');
        push_canon_list(s, "unit", &self.units);
        push_canon_list(s, "constant", &self.constants);
        push_canon_list(s, "boundary", &self.boundary);
        push_canon_list(s, "convention", &self.conventions);
        s.push_str("theory-version:");
        s.push_str(&self.theory_version);
        s.push('\n');
        push_canon_list(s, "definition", &self.definitions);
        push_canon_list(s, "dataset", &self.datasets);
        push_canon_list(s, "library", &self.formal_libraries);
    }
}

fn push_canon_list(s: &mut String, key: &str, items: &[String]) {
    for item in items {
        s.push_str(key);
        s.push(':');
        s.push_str(item);
        s.push('\n');
    }
}

fn push_why_list(lines: &mut Vec<String>, key: &str, items: &[String]) {
    if items.is_empty() {
        return;
    }
    lines.push(format!("  {key}:      {}", items.join(", ")));
}

/// Immutable identity of a scientific sentence.
///
/// Constructed only by [`FormalClaim::from_claim`]. There is no
/// [`serde::Deserialize`] impl: JSON cannot mint a catalog hash.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct FormalClaim {
    id: ClaimId,
    statement: String,
    statement_hash: ArtifactId,
    assumptions: AssumptionSetId,
    domain: DomainOfValidity,
    class: ClaimClass,
    layer: LayerId,
    commitments: ClaimCommitments,
}

impl FormalClaim {
    /// Canonical bytes the statement hash commits to.
    pub fn canonical_bytes(
        id: &str,
        statement: &str,
        class: ClaimClass,
        layer: LayerId,
        assumptions: &AssumptionSet,
        domain: &DomainOfValidity,
        commitments: &ClaimCommitments,
    ) -> Vec<u8> {
        Self::identity_bytes(
            id,
            statement,
            class,
            layer,
            &assumptions.id,
            domain,
            commitments,
        )
    }

    fn identity_bytes(
        id: &str,
        statement: &str,
        class: ClaimClass,
        layer: LayerId,
        assumptions: &AssumptionSetId,
        domain: &DomainOfValidity,
        commitments: &ClaimCommitments,
    ) -> Vec<u8> {
        let mut s = String::new();
        s.push_str("id:");
        s.push_str(id);
        s.push('\n');
        s.push_str("statement:");
        s.push_str(statement);
        s.push('\n');
        s.push_str("class:");
        s.push_str(class.as_str());
        s.push('\n');
        s.push_str("layer:");
        s.push_str(layer.as_str());
        s.push('\n');
        s.push_str("assumptions:");
        s.push_str(&assumptions.0.to_hex());
        s.push('\n');
        s.push_str("domain:");
        s.push_str(&domain.id.to_hex());
        s.push('\n');
        commitments.append_canonical(&mut s);
        s.into_bytes()
    }

    /// True when [`Self::statement_hash`] matches [`Self::canonical_bytes`].
    pub fn hash_is_consistent(&self) -> bool {
        ArtifactId::of(Self::identity_bytes(
            &self.id.0,
            &self.statement,
            self.class,
            self.layer,
            &self.assumptions,
            &self.domain,
            &self.commitments,
        )) == self.statement_hash
    }

    /// Lab claim id (`astro.sky-finite`, …).
    pub fn id(&self) -> &ClaimId {
        &self.id
    }

    /// The sentence as encoded today (English until physis-ir exists).
    pub fn statement(&self) -> &str {
        &self.statement
    }

    /// Hash of the canonical identity bytes.
    pub fn statement_hash(&self) -> ArtifactId {
        self.statement_hash
    }

    /// Assumption-set id.
    pub fn assumptions(&self) -> &AssumptionSetId {
        &self.assumptions
    }

    /// Domain of validity.
    pub fn domain(&self) -> &DomainOfValidity {
        &self.domain
    }

    /// Claim class.
    pub fn class(&self) -> ClaimClass {
        self.class
    }

    /// Layer.
    pub fn layer(&self) -> LayerId {
        self.layer
    }

    /// First-class identity fields committed in the hash.
    pub fn commitments(&self) -> &ClaimCommitments {
        &self.commitments
    }

    /// Identity of an executable lab claim. Recomputes the statement hash
    /// from the live sentence, class, layer, assumptions, domain, and
    /// commitments. The live [`Claim::statement_hash`] is always derived;
    /// there is no stored hash to copy through.
    pub fn from_claim(claim: &Claim) -> Self {
        let statement_hash = ArtifactId::of(Self::canonical_bytes(
            &claim.id.0,
            claim.statement(),
            claim.class,
            claim.layer,
            &claim.assumptions,
            &claim.domain,
            &claim.commitments,
        ));
        Self {
            id: claim.id.clone(),
            statement: claim.statement().to_string(),
            statement_hash,
            assumptions: claim.assumptions.id.clone(),
            domain: claim.domain.clone(),
            class: claim.class,
            layer: claim.layer,
            commitments: claim.commitments.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::claim::Claim;
    use crate::id::LayerId;

    fn hash(statement: &str, commitments: &ClaimCommitments) -> ArtifactId {
        let assumptions = AssumptionSet::encoding_internal();
        let domain = DomainOfValidity::encoding_wide();
        ArtifactId::of(FormalClaim::canonical_bytes(
            "math.example",
            statement,
            ClaimClass::Mathematical,
            LayerId::Mathematical,
            &assumptions,
            &domain,
            commitments,
        ))
    }

    #[test]
    fn forall_to_exists_in_prose_is_a_new_identity() {
        let c = ClaimCommitments::unspecified();
        let a = hash("forall x, P(x)", &c);
        let b = hash("exists x, P(x)", &c);
        assert_ne!(a, b);
    }

    #[test]
    fn quantifier_field_is_identity_apart_from_prose() {
        let stmt = "P(x) holds in the domain";
        let mut forall = ClaimCommitments::unspecified();
        forall.quantifier = Quantifier::ForAll;
        let mut exists = ClaimCommitments::unspecified();
        exists.quantifier = Quantifier::Exists;
        assert_ne!(hash(stmt, &forall), hash(stmt, &exists));
    }

    #[test]
    fn each_commitment_axis_changes_the_hash() {
        let stmt = "the same English sentence";
        let base = ClaimCommitments::unspecified();
        let h0 = hash(stmt, &base);
        let mut units = base.clone();
        units.units = vec!["m".into()];
        let mut constants = base.clone();
        constants.constants = vec!["c=1".into()];
        let mut boundary = base.clone();
        boundary.boundary = vec!["unification-scale".into()];
        let mut conventions = base.clone();
        conventions.conventions = vec!["sign:+".into()];
        let mut sign_minus = base.clone();
        sign_minus.conventions = vec!["sign:-".into()];
        let mut version = base.clone();
        version.theory_version = "encoding-v2".into();
        let mut definitions = base.clone();
        definitions.definitions = vec!["Q = T3 + Y".into()];
        let mut datasets = base.clone();
        datasets.datasets = vec!["pdg-2024-sin2theta".into()];
        let mut libraries = base.clone();
        libraries.formal_libraries = vec!["physlib:unversioned".into()];
        for (name, other) in [
            ("units", units),
            ("constants", constants),
            ("boundary", boundary),
            ("conventions", conventions.clone()),
            ("theory_version", version),
            ("definitions", definitions),
            ("datasets", datasets),
            ("libraries", libraries),
        ] {
            assert_ne!(h0, hash(stmt, &other), "{name} must change the identity");
        }
        assert_ne!(
            hash(stmt, &conventions),
            hash(stmt, &sign_minus),
            "a sign flip is a new identity"
        );
    }

    #[test]
    fn unspecified_why_lines_are_silent() {
        assert!(ClaimCommitments::unspecified().why_lines().is_empty());
        let lines = ClaimCommitments::physlib_forall().why_lines();
        assert!(lines.iter().any(|l| l.contains("quantifier: forall")));
        assert!(lines
            .iter()
            .any(|l| l.contains("units:") && l.contains('1')));
        assert!(lines
            .iter()
            .any(|l| l.contains("libraries:") && l.contains("physlib:unversioned")));
    }

    #[test]
    fn from_claim_matches_the_live_hash() {
        let claim = Claim::new(
            "math.example",
            "P holds",
            LayerId::Mathematical,
            ClaimClass::Mathematical,
        );
        let formal = FormalClaim::from_claim(&claim);
        assert_eq!(formal.statement_hash(), claim.statement_hash());
        assert!(formal.hash_is_consistent());
        assert_eq!(formal.statement(), claim.statement());
    }
}
