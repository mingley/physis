//! Untrusted proof artifacts. Treat solver output as hostile.

use serde::{Deserialize, Serialize};

/// What the untrusted side handed the verifier.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum UntrustedProof {
    /// "The catalog identity is the zero polynomial." No solver-chosen
    /// statement; the challenge already contains the identity.
    ExactIdentity,
    /// Lean source. Scanned for `axiom` / `sorry` / `admit` before any
    /// kernel is invoked. Cannot mint without dual kernel replay.
    LeanSource {
        /// Candidate `.lean` text.
        source: String,
    },
    /// `lean4export` bytes. Independent checkers replay these.
    LeanExport {
        /// Export file contents.
        bytes: Vec<u8>,
    },
}

/// Result of scanning untrusted Lean (or tactic) text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScanReport {
    /// Unauthorized `axiom` lines.
    pub axioms: Vec<String>,
    /// `sorry` / `admit` / `sorryAx` occurrences.
    pub holes: Vec<String>,
}

impl ScanReport {
    /// True when the artifact is clean of holes and extra axioms.
    pub fn clean(&self) -> bool {
        self.axioms.is_empty() && self.holes.is_empty()
    }
}

/// Scan Lean-like source for holes and extra axioms.
///
/// Lines whose first non-whitespace token is `axiom`, or which contain
/// `sorry` / `admit` as identifiers, are recorded. This is a conservative
/// gate, not a parser: if it is unsure, it fails closed by treating a
/// match as a hole.
pub fn scan_lean_source(source: &str) -> ScanReport {
    let mut axioms = Vec::new();
    let mut holes = Vec::new();
    for (i, line) in source.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("--") {
            continue;
        }
        if trimmed.starts_with("axiom ") || trimmed.starts_with("axiom\t") {
            axioms.push(format!("L{}: {trimmed}", i + 1));
        }
        for hole in ["sorry", "admit", "sorryAx"] {
            if contains_ident(trimmed, hole) {
                holes.push(format!("L{}: {hole}", i + 1));
            }
        }
    }
    ScanReport { axioms, holes }
}

fn contains_ident(line: &str, ident: &str) -> bool {
    let bytes = line.as_bytes();
    let id = ident.as_bytes();
    let mut i = 0;
    while i + id.len() <= bytes.len() {
        if &bytes[i..i + id.len()] == id {
            let before = if i == 0 {
                true
            } else {
                !is_ident_char(bytes[i - 1])
            };
            let after = i + id.len() == bytes.len() || !is_ident_char(bytes[i + id.len()]);
            if before && after {
                return true;
            }
        }
        i += 1;
    }
    false
}

fn is_ident_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'.'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorry_and_axiom_are_rejected() {
        let src = r#"
axiom answer_is_true : DesiredTheorem
theorem T : DesiredTheorem := sorry
"#;
        let r = scan_lean_source(src);
        assert!(!r.clean());
        assert_eq!(r.axioms.len(), 1);
        assert!(!r.holes.is_empty());
    }

    #[test]
    fn comments_are_ignored() {
        let src = "-- sorry in a comment\ntheorem T : True := trivial\n";
        assert!(scan_lean_source(src).clean());
    }
}
