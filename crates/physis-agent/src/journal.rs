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
        /// Claims whose verdict kind changed.
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
    /// A dual-checked receipt was minted. Restore re-verifies; it does not
    /// deserialize a `Verified` value.
    Prove {
        /// Unix millis.
        t: u64,
        /// Claim id.
        claim: String,
        /// Challenge hash hex.
        challenge_hash: String,
    },
    /// Semantic review ran. Restore re-runs the dossier review; it does not
    /// deserialize a semantic-assurance tag as authority.
    Review {
        /// Unix millis.
        t: u64,
        /// Claim id.
        claim: String,
        /// Evidence hash hex (informational; restore re-runs review).
        evidence_hash: String,
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
    pub fn prove(claim: impl Into<String>, challenge_hash: impl Into<String>) -> Self {
        JournalEvent::Prove {
            t: now_ms(),
            claim: claim.into(),
            challenge_hash: challenge_hash.into(),
        }
    }

    /// A successful semantic review, stamped with the current time.
    pub fn review(claim: impl Into<String>, evidence_hash: impl Into<String>) -> Self {
        JournalEvent::Review {
            t: now_ms(),
            claim: claim.into(),
            evidence_hash: evidence_hash.into(),
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
}
