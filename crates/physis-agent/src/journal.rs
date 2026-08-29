//! Append-only journal of agent actions and verdict diffs.

use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

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
}

/// JSONL journal, optionally persisted.
#[derive(Clone, Debug, Default)]
pub struct Journal {
    events: Vec<JournalEvent>,
    path: Option<PathBuf>,
}

impl Journal {
    /// In-memory only.
    pub fn memory() -> Self {
        Self {
            events: Vec::new(),
            path: None,
        }
    }

    /// Parse a JSONL string into an in-memory journal (no file backing).
    ///
    /// Malformed lines are skipped; blank lines are ignored. This is the
    /// lenient path; use [`Journal::from_jsonl_counting`] when the number of
    /// dropped lines matters (e.g. before certifying a replay).
    pub fn from_jsonl(s: &str) -> Self {
        let (events, _) = parse_jsonl_lines(s);
        Self { events, path: None }
    }

    /// Like [`Journal::from_jsonl`], but also returns how many non-blank lines
    /// failed to parse. A non-zero count means the journal is corrupted,
    /// truncated, or schema-incompatible and must not be trusted as complete.
    pub fn from_jsonl_counting(s: &str) -> (Self, usize) {
        let (events, malformed) = parse_jsonl_lines(s);
        (Self { events, path: None }, malformed)
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
        Ok(Self {
            events,
            path: Some(path),
        })
    }

    /// Record an event.
    pub fn record(&mut self, event: JournalEvent) {
        if let Some(path) = &self.path {
            if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(path) {
                if let Ok(line) = serde_json::to_string(&event) {
                    let _ = writeln!(f, "{line}");
                }
            }
        }
        self.events.push(event);
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
