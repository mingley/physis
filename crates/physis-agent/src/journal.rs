//! Append-only journal of agent actions and verdict diffs.

use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use physis_core::artifact::ArtifactId;
use physis_core::knob::KnobValue;
use physis_theory::VerdictDiff;
use serde::{Deserialize, Serialize};

/// One journal record.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "kebab-case")]
pub enum JournalEvent {
    /// Lab created with these theory ids.
    Boot {
        /// Unix millis.
        t: u64,
        /// Theory ids.
        theories: Vec<String>,
    },
    /// A knob was turned.
    SetKnob {
        /// Unix millis.
        t: u64,
        /// Theory lab id.
        theory: String,
        /// Knob name.
        knob: String,
        /// Previous value.
        from: KnobValue,
        /// New value.
        to: KnobValue,
        /// Claims whose scientific axes changed (kind, derivation,
        /// empirical, or projected judgment).
        diffs: Vec<VerdictDiff>,
    },
    /// An experiment was run.
    Experiment {
        /// Unix millis.
        t: u64,
        /// Experiment id.
        id: String,
    },
    /// A theory was fully evaluated.
    Run {
        /// Unix millis.
        t: u64,
        /// Theory id.
        theory: String,
        /// Holds count.
        holds: usize,
        /// Fails count.
        fails: usize,
        /// Other counts.
        other: usize,
    },
    /// A dual-checked receipt was minted. Restore re-verifies the recorded
    /// identity; it does not deserialize a `Verified` value. A matching
    /// slug with a different `statement_hash` is not this prove.
    Prove {
        /// Unix millis.
        t: u64,
        /// Claim id.
        claim: String,
        /// Challenge hash hex. Restore remints only when this matches
        /// [`physis_proof::Challenge::generate`] on the live FormalClaim.
        challenge_hash: String,
        /// FormalClaim identity hex. Empty on pre-identity journals;
        /// restore then still requires a live challenge-hash match.
        #[serde(default)]
        statement_hash: String,
    },
    /// Semantic review ran against the live statement identity. Restore
    /// re-runs the dossier review of that hash; it does not deserialize a
    /// semantic-assurance tag as authority. A matching slug with a
    /// different `statement_hash` is not this review.
    Review {
        /// Unix millis.
        t: u64,
        /// Claim id.
        claim: String,
        /// Evidence hash hex (informational; restore re-runs review).
        evidence_hash: String,
        /// FormalClaim identity hex. Restore remints only when this
        /// matches the live claim. Empty (legacy slug-only) is not P3S.
        #[serde(default)]
        statement_hash: String,
    },
    /// One research-cycle summary. Restore is a no-op; inner prove/review
    /// events re-run their checkers.
    Loop {
        /// Unix millis.
        t: u64,
        /// Catalog claims proved this cycle.
        proved: Vec<String>,
        /// Catalog claims reviewed this cycle.
        reviewed: Vec<String>,
    },
    /// An evidence graph was rebuilt from live evaluations. Restore
    /// rebuilds the DAG from live state; the recorded graph hash is not
    /// deserialized as authority and is not Canonical or P4.
    Evidence {
        /// Unix millis.
        t: u64,
        /// Claim id (lab slug).
        claim: String,
        /// Content-addressed Evidence node hex. Restore rebuilds; a
        /// forged hash cannot mint the graph.
        graph_hash: String,
    },
    /// An independent Ratio parse of a `CertifiedNumeric` enclosure.
    /// Restore rebuilds from live overlay strings. The recorded
    /// certificate hash is not deserialized as authority and is not a
    /// kernel receipt, Canonical, or P4.
    Enclose {
        /// Unix millis.
        t: u64,
        /// Claim id (lab slug).
        claim: String,
        /// Content-addressed NumericCertificate node hex. Restore
        /// rebuilds; a forged hash cannot mint the certificate.
        certificate_hash: String,
    },
    /// An independent SourceRecord rebuild. Restore rebuilds from live
    /// dataset or dossier fields. The recorded source hash is not
    /// deserialized as authority, is not P3S, and is not Canonical or P4.
    Cite {
        /// Unix millis.
        t: u64,
        /// Claim id (lab slug).
        claim: String,
        /// Content-addressed Source node hex. Restore rebuilds; a
        /// forged hash cannot mint the record.
        source_hash: String,
    },
    /// An independent IR package round-trip. Restore rebuilds from the
    /// live theory package. The recorded package hash is not
    /// deserialized as authority, is not P3S, and is not Canonical or P4.
    Encode {
        /// Unix millis.
        t: u64,
        /// Theory id.
        theory: String,
        /// Content-addressed EncodingPackage node hex. Restore rebuilds;
        /// a forged hash cannot mint the package.
        package_hash: String,
    },
}

/// Parse JSONL into events, counting non-blank lines that fail to deserialize.
fn parse_jsonl_lines(s: &str) -> (Vec<JournalEvent>, usize) {
    let mut events = Vec::new();
    let mut malformed = 0usize;
    for line in s.lines() {
        if line.trim().is_empty() {
            continue;
        }
        match serde_json::from_str(line) {
            Ok(ev) => events.push(ev),
            Err(_) => malformed += 1,
        }
    }
    (events, malformed)
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

impl JournalEvent {
    /// A boot event stamped with the current time.
    pub fn boot(theories: Vec<String>) -> Self {
        JournalEvent::Boot {
            t: now_ms(),
            theories,
        }
    }

    /// A knob turn stamped with the current time.
    pub fn set_knob(
        theory: impl Into<String>,
        knob: impl Into<String>,
        from: KnobValue,
        to: KnobValue,
        diffs: Vec<VerdictDiff>,
    ) -> Self {
        JournalEvent::SetKnob {
            t: now_ms(),
            theory: theory.into(),
            knob: knob.into(),
            from,
            to,
            diffs,
        }
    }

    /// A full-evaluation event stamped with the current time.
    pub fn run(theory: impl Into<String>, holds: usize, fails: usize, other: usize) -> Self {
        JournalEvent::Run {
            t: now_ms(),
            theory: theory.into(),
            holds,
            fails,
            other,
        }
    }

    /// An experiment event stamped with the current time.
    pub fn experiment(id: impl Into<String>) -> Self {
        JournalEvent::Experiment {
            t: now_ms(),
            id: id.into(),
        }
    }

    /// A successful prove, stamped with the current time.
    pub fn prove(
        claim: impl Into<String>,
        challenge_hash: impl Into<String>,
        statement_hash: impl Into<String>,
    ) -> Self {
        JournalEvent::Prove {
            t: now_ms(),
            claim: claim.into(),
            challenge_hash: challenge_hash.into(),
            statement_hash: statement_hash.into(),
        }
    }

    /// A successful semantic review, stamped with the current time.
    pub fn review(
        claim: impl Into<String>,
        evidence_hash: impl Into<String>,
        statement_hash: impl Into<String>,
    ) -> Self {
        JournalEvent::Review {
            t: now_ms(),
            claim: claim.into(),
            evidence_hash: evidence_hash.into(),
            statement_hash: statement_hash.into(),
        }
    }

    /// A research-cycle summary, stamped with the current time.
    pub fn research_loop(proved: Vec<String>, reviewed: Vec<String>) -> Self {
        JournalEvent::Loop {
            t: now_ms(),
            proved,
            reviewed,
        }
    }

    /// An evidence snapshot, stamped with the current time.
    pub fn evidence(claim: impl Into<String>, graph_hash: impl Into<String>) -> Self {
        JournalEvent::Evidence {
            t: now_ms(),
            claim: claim.into(),
            graph_hash: graph_hash.into(),
        }
    }

    /// An independent Ratio enclose, stamped with the current time.
    pub fn enclose(claim: impl Into<String>, certificate_hash: impl Into<String>) -> Self {
        JournalEvent::Enclose {
            t: now_ms(),
            claim: claim.into(),
            certificate_hash: certificate_hash.into(),
        }
    }

    /// An independent SourceRecord rebuild, stamped with the current time.
    pub fn cite(claim: impl Into<String>, source_hash: impl Into<String>) -> Self {
        JournalEvent::Cite {
            t: now_ms(),
            claim: claim.into(),
            source_hash: source_hash.into(),
        }
    }

    /// An independent IR package round-trip, stamped with the current time.
    pub fn encode(theory: impl Into<String>, package_hash: impl Into<String>) -> Self {
        JournalEvent::Encode {
            t: now_ms(),
            theory: theory.into(),
            package_hash: package_hash.into(),
        }
    }
}

/// JSONL journal, optionally persisted. Events are hash-linked in memory
/// (`tip` changes if history is rewritten).
#[derive(Clone, Debug, Default)]
pub struct Journal {
    events: Vec<JournalEvent>,
    hashes: Vec<ArtifactId>,
    path: Option<PathBuf>,
}

fn genesis() -> ArtifactId {
    ArtifactId::of(b"physis-journal-genesis")
}

fn hash_event(prev: ArtifactId, event: &JournalEvent) -> ArtifactId {
    let body = serde_json::to_string(event).unwrap_or_default();
    ArtifactId::of(format!("{}\n{body}", prev.to_hex()).as_bytes())
}

fn with_chain(events: Vec<JournalEvent>, path: Option<PathBuf>) -> Journal {
    let mut hashes = Vec::with_capacity(events.len());
    let mut prev = genesis();
    for ev in &events {
        prev = hash_event(prev, ev);
        hashes.push(prev);
    }
    Journal {
        events,
        hashes,
        path,
    }
}

impl Journal {
    /// In-memory only.
    pub fn memory() -> Self {
        Self {
            events: Vec::new(),
            hashes: Vec::new(),
            path: None,
        }
    }

    /// Merkle tip (genesis if empty).
    pub fn tip(&self) -> ArtifactId {
        self.hashes.last().copied().unwrap_or_else(genesis)
    }

    /// Parse a JSONL string into an in-memory journal (no file backing).
    ///
    /// Malformed lines are skipped; blank lines are ignored. This is the
    /// lenient path; use [`Journal::from_jsonl_counting`] when the number of
    /// dropped lines matters (e.g. before certifying a replay).
    pub fn from_jsonl(s: &str) -> Self {
        let (events, _) = parse_jsonl_lines(s);
        with_chain(events, None)
    }

    /// Like [`Journal::from_jsonl`], but also returns how many non-blank lines
    /// failed to parse. A non-zero count means the journal is corrupted,
    /// truncated, or schema-incompatible and must not be trusted as complete.
    pub fn from_jsonl_counting(s: &str) -> (Self, usize) {
        let (events, malformed) = parse_jsonl_lines(s);
        (with_chain(events, None), malformed)
    }

    /// Append to a JSONL file (created if needed).
    pub fn file(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let mut events = Vec::new();
        if path.exists() {
            let f = File::open(&path)?;
            for line in BufReader::new(f).lines() {
                let line = line?;
                if line.trim().is_empty() {
                    continue;
                }
                if let Ok(ev) = serde_json::from_str(&line) {
                    events.push(ev);
                }
            }
        }
        Ok(with_chain(events, Some(path)))
    }

    /// Record an event.
    pub fn record(&mut self, event: JournalEvent) {
        let prev = self.tip();
        let h = hash_event(prev, &event);
        if let Some(path) = &self.path {
            if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(path) {
                if let Ok(line) = serde_json::to_string(&event) {
                    let _ = writeln!(f, "{line}");
                }
            }
        }
        self.events.push(event);
        self.hashes.push(h);
    }

    /// All events, oldest first.
    pub fn events(&self) -> &[JournalEvent] {
        &self.events
    }

    /// Number of events.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Empty?
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

impl fmt::Display for Journal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for ev in &self.events {
            writeln!(f, "{}", serde_json::to_string(ev).unwrap_or_default())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn boot_at(t: u64, theories: Vec<String>) -> JournalEvent {
        JournalEvent::Boot { t, theories }
    }

    fn run_at(t: u64, theory: &str, holds: usize, fails: usize, other: usize) -> JournalEvent {
        JournalEvent::Run {
            t,
            theory: theory.into(),
            holds,
            fails,
            other,
        }
    }

    #[test]
    fn rewriting_history_changes_the_tip() {
        // Wall-clock helpers stamp `t`; the Merkle tip includes it, so this
        // comparison has to use a frozen timestamp or it flakes across a ms.
        let mut j = Journal::memory();
        j.record(boot_at(1, vec!["a".into()]));
        j.record(run_at(2, "a", 1, 0, 0));
        let tip = j.tip();
        let mut k = Journal::memory();
        k.record(boot_at(1, vec!["a".into()]));
        k.record(run_at(2, "a", 0, 1, 0));
        assert_ne!(tip, k.tip());
        let mut j2 = Journal::memory();
        j2.record(boot_at(1, vec!["a".into()]));
        j2.record(run_at(2, "a", 1, 0, 0));
        assert_eq!(j.tip(), j2.tip());
    }

    #[test]
    fn prove_event_records_statement_hash() {
        let ev = JournalEvent::prove("dec.d-squared-zero", "aa", "bb");
        let s = serde_json::to_string(&ev).unwrap();
        assert!(s.contains("\"event\":\"prove\""));
        assert!(s.contains("\"statement_hash\":\"bb\""));
        assert!(s.contains("\"challenge_hash\":\"aa\""));
        let back: JournalEvent = serde_json::from_str(&s).unwrap();
        match back {
            JournalEvent::Prove {
                statement_hash,
                challenge_hash,
                claim,
                ..
            } => {
                assert_eq!(claim, "dec.d-squared-zero");
                assert_eq!(challenge_hash, "aa");
                assert_eq!(statement_hash, "bb");
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn legacy_prove_without_statement_hash_deserializes_empty() {
        let s = r#"{"event":"prove","t":1,"claim":"dec.d-squared-zero","challenge_hash":"aa"}"#;
        let ev: JournalEvent = serde_json::from_str(s).unwrap();
        match ev {
            JournalEvent::Prove { statement_hash, .. } => assert!(statement_hash.is_empty()),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn legacy_review_without_statement_hash_deserializes_empty() {
        let s = r#"{"event":"review","t":1,"claim":"dec.d-squared-zero","evidence_hash":"aa"}"#;
        let ev: JournalEvent = serde_json::from_str(s).unwrap();
        match ev {
            JournalEvent::Review { statement_hash, .. } => assert!(statement_hash.is_empty()),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn evidence_event_round_trips_and_is_not_a_graph() {
        let ev = JournalEvent::evidence("predictivity.unique-vacuum", "deadbeef");
        let s = serde_json::to_string(&ev).unwrap();
        assert!(s.contains("\"event\":\"evidence\""));
        assert!(s.contains("\"graph_hash\":\"deadbeef\""));
        let back: JournalEvent = serde_json::from_str(&s).unwrap();
        match back {
            JournalEvent::Evidence {
                claim, graph_hash, ..
            } => {
                assert_eq!(claim, "predictivity.unique-vacuum");
                assert_eq!(graph_hash, "deadbeef");
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn enclose_event_round_trips_and_is_not_a_certificate() {
        let ev = JournalEvent::enclose("gut.weinberg-angle", "deadbeef");
        let s = serde_json::to_string(&ev).unwrap();
        assert!(s.contains("\"event\":\"enclose\""));
        assert!(s.contains("\"certificate_hash\":\"deadbeef\""));
        assert!(!s.contains("receipt"), "{s}");
        let back: JournalEvent = serde_json::from_str(&s).unwrap();
        match back {
            JournalEvent::Enclose {
                claim,
                certificate_hash,
                ..
            } => {
                assert_eq!(claim, "gut.weinberg-angle");
                assert_eq!(certificate_hash, "deadbeef");
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn cite_event_round_trips_and_is_not_p3s() {
        let ev = JournalEvent::cite("gut.proton-lifetime-sk", "deadbeef");
        let s = serde_json::to_string(&ev).unwrap();
        assert!(s.contains("\"event\":\"cite\""));
        assert!(s.contains("\"source_hash\":\"deadbeef\""));
        assert!(!s.contains("receipt"), "{s}");
        let back: JournalEvent = serde_json::from_str(&s).unwrap();
        match back {
            JournalEvent::Cite {
                claim, source_hash, ..
            } => {
                assert_eq!(claim, "gut.proton-lifetime-sk");
                assert_eq!(source_hash, "deadbeef");
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn encode_event_round_trips_and_is_not_p3s() {
        let ev = JournalEvent::encode("combinational-circuit", "deadbeef");
        let s = serde_json::to_string(&ev).unwrap();
        assert!(s.contains("\"event\":\"encode\""));
        assert!(s.contains("\"package_hash\":\"deadbeef\""));
        assert!(!s.contains("receipt"), "{s}");
        let back: JournalEvent = serde_json::from_str(&s).unwrap();
        match back {
            JournalEvent::Encode {
                theory,
                package_hash,
                ..
            } => {
                assert_eq!(theory, "combinational-circuit");
                assert_eq!(package_hash, "deadbeef");
            }
            other => panic!("{other:?}"),
        }
    }
}
