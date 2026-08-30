//! Scientific provenance. `source: "textbook"` is not an authoritative locator.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use physis_core::artifact::ArtifactId;
use serde::{Deserialize, Serialize};

/// Error when a source record is too vague to be authoritative.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProvenanceError {
    /// The locator is a slogan, not a page/equation/dataset row.
    VagueLocator(String),
    /// Stored `source_hash` does not match a reconstruction of the fields.
    HashMismatch,
}

impl std::fmt::Display for ProvenanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProvenanceError::VagueLocator(s) => {
                write!(
                    f,
                    "refusing vague source locator '{s}'; need work/edition/page/equation"
                )
            }
            ProvenanceError::HashMismatch => {
                write!(
                    f,
                    "stored source_hash does not match the reconstructed SourceRecord"
                )
            }
        }
    }
}

impl std::error::Error for ProvenanceError {}

/// Bibliographic citation (work, not a proof).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Citation {
    /// Work title or key.
    pub work: String,
    /// Edition / year / arXiv id.
    pub edition: String,
}

/// Where in the work the fact lives.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceLocator {
    /// Page number, if any.
    pub page: Option<u32>,
    /// Section heading.
    pub section: Option<String>,
    /// Equation number or tag.
    pub equation: Option<String>,
    /// Figure id.
    pub figure: Option<String>,
    /// Table id.
    pub table: Option<String>,
    /// Dataset row/range.
    pub dataset_range: Option<String>,
    /// Experiment identifier.
    pub experiment: Option<String>,
}

impl SourceLocator {
    /// True when at least one precise anchor is present.
    pub fn is_precise(&self) -> bool {
        self.page.is_some()
            || self.equation.is_some()
            || self.figure.is_some()
            || self.table.is_some()
            || self.dataset_range.is_some()
            || self.experiment.is_some()
            || self.section.is_some()
    }
}

/// Optional record of an extraction (AI or human). Never itself trusted.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtractionRecord {
    /// Who/what extracted.
    pub agent: String,
    /// Hash of the extracted bytes.
    pub extracted_hash: ArtifactId,
}

/// Immutable source record.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceRecord {
    /// Hash of the canonical record.
    pub source_hash: ArtifactId,
    /// Citation.
    pub citation: Citation,
    /// Version / release of the source.
    pub version: String,
    /// Precise locator.
    pub locator: SourceLocator,
    /// Hash of the source artifact (pdf bytes, dataset file, …).
    pub artifact_hash: ArtifactId,
    /// Extraction, if any. Provenance-bearing input, not authority.
    pub extraction: Option<ExtractionRecord>,
}

impl SourceRecord {
    /// Build a record. Rejects slogan locators such as `"textbook"`.
    pub fn new(
        citation: Citation,
        version: impl Into<String>,
        locator: SourceLocator,
        artifact_hash: ArtifactId,
        extraction: Option<ExtractionRecord>,
    ) -> Result<Self, ProvenanceError> {
        let work = citation.work.to_lowercase();
        if work == "textbook" || work == "wikipedia" || work.trim().is_empty() {
            return Err(ProvenanceError::VagueLocator(citation.work.clone()));
        }
        if !locator.is_precise() {
            return Err(ProvenanceError::VagueLocator(
                "no page/section/equation/figure/table/dataset/experiment".into(),
            ));
        }
        let version = version.into();
        let mut canonical = String::new();
        canonical.push_str(&citation.work);
        canonical.push('\n');
        canonical.push_str(&citation.edition);
        canonical.push('\n');
        canonical.push_str(&version);
        canonical.push('\n');
        canonical.push_str(&format!("{locator:?}\n"));
        canonical.push_str(&artifact_hash.to_hex());
        Ok(Self {
            source_hash: ArtifactId::of(canonical.as_bytes()),
            citation,
            version,
            locator,
            artifact_hash,
            extraction,
        })
    }

    /// Rebuild this record from its fields. Rejects slogan locators and
    /// a stored hash that does not match the reconstruction. The stored
    /// `source_hash` is not authority.
    pub fn recheck(&self) -> Result<Self, ProvenanceError> {
        let rebuilt = Self::new(
            self.citation.clone(),
            self.version.clone(),
            self.locator.clone(),
            self.artifact_hash,
            self.extraction.clone(),
        )?;
        if rebuilt.source_hash != self.source_hash {
            return Err(ProvenanceError::HashMismatch);
        }
        Ok(rebuilt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn textbook_is_not_a_source() {
        let err = SourceRecord::new(
            Citation {
                work: "textbook".into(),
                edition: "any".into(),
            },
            "1",
            SourceLocator {
                page: Some(1),
                section: None,
                equation: None,
                figure: None,
                table: None,
                dataset_range: None,
                experiment: None,
            },
            ArtifactId::of(b"pdf"),
            None,
        )
        .unwrap_err();
        assert!(matches!(err, ProvenanceError::VagueLocator(_)));
    }

    #[test]
    fn equation_anchor_is_accepted() {
        let rec = SourceRecord::new(
            Citation {
                work: "Jackson Classical Electrodynamics".into(),
                edition: "3rd".into(),
            },
            "1999",
            SourceLocator {
                page: Some(239),
                section: None,
                equation: Some("6.6".into()),
                figure: None,
                table: None,
                dataset_range: None,
                experiment: None,
            },
            ArtifactId::of(b"jackson-3ed"),
            None,
        )
        .unwrap();
        assert_eq!(rec.locator.equation.as_deref(), Some("6.6"));
        assert!(rec.recheck().is_ok());
        let mut forged = rec.clone();
        forged.source_hash = ArtifactId::of(b"forged-source");
        assert!(matches!(
            forged.recheck(),
            Err(ProvenanceError::HashMismatch)
        ));
    }
}
