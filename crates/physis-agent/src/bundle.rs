//! Durable campaign bundles with crash recovery (R2, spec 022).
//!
//! A bundle is a directory with a frozen `meta.json`, an append-only
//! hash-chained `journal.jsonl` of action intents and results, and an
//! `objects/` blob store ([`DurableStore`]). The write protocol is:
//! reserve the attempt budget and append a fsynced intent *before*
//! dispatch; store output objects; validate them and append a fsynced
//! result with the same attempt id *after* completion.
//!
//! A kill between any two boundaries restarts cleanly: completed attempts
//! stay completed (a repeated commit of the same result is ignored, never
//! double-charged), attempts with an intent but no result resume as
//! interrupted with their charge kept, and attempts with no intent never
//! started. Opening refuses corrupt journals, dangling object references,
//! unknown bundle versions, and concurrent writers. Objects are rehashed
//! on read: loading bytes never restores trust in them.

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use physis_core::ArtifactId;
use physis_store::DurableStore;
use serde::{Deserialize, Serialize};

use crate::campaign::CampaignOp;
use crate::coordinator::CoordinatorCheckpoint;

/// Bundle metadata schema tag.
pub const BUNDLE_SCHEMA: &str = "physis-campaign-bundle";
/// The only bundle version R2 understands.
pub const BUNDLE_VERSION: u32 = 1;

/// Previous-hash of the first journal record.
fn genesis_tip() -> ArtifactId {
    ArtifactId::of(b"physis-bundle-genesis")
}

/// Frozen bundle metadata, written once at creation.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct BundleMeta {
    schema: String,
    version: u32,
    manifest: Option<String>,
    created_ms: u64,
}

/// One hash-chained journal record.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Record {
    seq: u64,
    prev: String,
    body: RecordBody,
}

/// Intent or result body.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
enum RecordBody {
    Intent {
        attempt: String,
        op: CampaignOp,
        parent: String,
        inputs: Vec<String>,
        cost_actions: u64,
    },
    Result {
        attempt: String,
        outcome: StoredOutcome,
        outputs: Vec<String>,
        note: String,
    },
    /// Coordinator usage snapshot (R4). Monotonic within one campaign;
    /// the latest record is the resume state.
    Checkpoint { state: CoordinatorCheckpoint },
}

/// Outcome as stored in the journal.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
enum StoredOutcome {
    Completed,
    Failed,
}

/// Chain hash over one record: previous tip, sequence, canonical body.
fn chain_hash(seq: u64, prev: &str, body: &RecordBody) -> ArtifactId {
    let body_json = serde_json::to_string(body).unwrap_or_default();
    ArtifactId::of(format!("{prev}\n{seq}\n{body_json}").as_bytes())
}

/// Verified intent, held in the in-memory index. The R4 coordinator reads
/// these to dispatch attempts.
#[derive(Debug, Clone)]
pub struct AttemptIntent {
    /// Operation to dispatch.
    pub op: CampaignOp,
    /// Expected parent-state identity.
    pub parent: String,
    /// Input object ids (verified present at intent time).
    pub inputs: Vec<ArtifactId>,
    /// Action cost charged by this intent.
    pub cost_actions: u64,
}

/// Verified result, held in the in-memory index.
#[derive(Debug, Clone)]
pub struct AttemptResult {
    /// Committed outcome.
    pub outcome: AttemptOutcome,
    /// Output object ids (verified present at commit time).
    pub outputs: Vec<ArtifactId>,
    /// Coordinator note.
    pub note: String,
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

/// Write bytes fully and fsync them plus the parent directory.
fn write_synced(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    {
        let mut f = File::create(path)?;
        f.write_all(bytes)?;
        f.sync_all()?;
    }
    if let Some(parent) = path.parent() {
        File::open(parent)?.sync_all()?;
    }
    Ok(())
}

/// Outcome committed for one attempt.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttemptOutcome {
    /// The action ran to completion.
    Completed,
    /// The action ran and failed (worker failure, resource failure).
    /// The attempt is still charged.
    Failed,
}

/// Resume state of one attempt id.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttemptState {
    /// No intent was ever recorded.
    NeverStarted,
    /// Intent recorded, no result yet: the worker may have died mid-action.
    Interrupted,
    /// A completed result was committed.
    Completed,
    /// A failed result was committed.
    Failed,
}

/// What a commit call did.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommitDisposition {
    /// The result was appended and fsynced.
    Committed,
    /// The identical result was already committed for this attempt
    /// (transport duplication): ignored, not double-charged.
    DuplicateIgnored,
}

/// A durable campaign bundle.
#[derive(Debug)]
pub struct Bundle {
    dir: PathBuf,
    store: DurableStore,
    journal: Option<File>,
    intents: HashMap<String, AttemptIntent>,
    results: HashMap<String, AttemptResult>,
    intent_order: Vec<String>,
    checkpoints: Vec<CoordinatorCheckpoint>,
    manifest: Option<String>,
    next_attempt: u64,
    tip: ArtifactId,
    records: u64,
    writable: bool,
    lock_held: bool,
}

/// Verified journal state shared by the writable and read-only opens.
struct Loaded {
    store: DurableStore,
    intents: HashMap<String, AttemptIntent>,
    results: HashMap<String, AttemptResult>,
    intent_order: Vec<String>,
    checkpoints: Vec<CoordinatorCheckpoint>,
    manifest: Option<String>,
    next_attempt: u64,
    tip: ArtifactId,
    records: u64,
}

impl Bundle {
    /// Create a new bundle directory. Refuses when the path already exists:
    /// a new history is never silently started over an old one.
    pub fn create(dir: impl AsRef<Path>, manifest: Option<ArtifactId>) -> Result<Self, String> {
        let dir = dir.as_ref().to_path_buf();
        if dir.exists() {
            return Err(format!(
                "bundle '{}' already exists; open it",
                dir.display()
            ));
        }
        std::fs::create_dir_all(&dir).map_err(|e| format!("bundle io: {e}"))?;
        let store =
            DurableStore::open(dir.join("objects")).map_err(|e| format!("bundle io: {e}"))?;
        let meta = BundleMeta {
            schema: BUNDLE_SCHEMA.into(),
            version: BUNDLE_VERSION,
            manifest: manifest.map(|m| m.to_hex()),
            created_ms: now_ms(),
        };
        let meta_json = serde_json::to_string(&meta).map_err(|e| format!("bundle io: {e}"))?;
        write_synced(&dir.join("meta.json"), meta_json.as_bytes())
            .map_err(|e| format!("bundle io: {e}"))?;
        write_synced(&dir.join("journal.jsonl"), b"").map_err(|e| format!("bundle io: {e}"))?;
        Self::take_lock(&dir)?;
        let journal = Self::append_handle(&dir)?;
        Ok(Self {
            dir,
            store,
            journal: Some(journal),
            intents: HashMap::new(),
            results: HashMap::new(),
            intent_order: Vec::new(),
            checkpoints: Vec::new(),
            manifest: manifest.map(|m| m.to_hex()),
            next_attempt: 1,
            tip: genesis_tip(),
            records: 0,
            writable: true,
            lock_held: true,
        })
    }

    /// Open an existing bundle for writing. Verifies the meta version, the
    /// journal chain, and every referenced object. Refuses when another
    /// writer holds the bundle. A failed open releases the lock it took.
    pub fn open(dir: impl AsRef<Path>) -> Result<Self, String> {
        let dir = dir.as_ref().to_path_buf();
        Self::take_lock(&dir)?;
        match Self::load(&dir) {
            Ok(loaded) => {
                let journal = Self::append_handle(&dir)?;
                Ok(Self {
                    dir,
                    store: loaded.store,
                    journal: Some(journal),
                    intents: loaded.intents,
                    results: loaded.results,
                    intent_order: loaded.intent_order,
                    checkpoints: loaded.checkpoints,
                    manifest: loaded.manifest,
                    next_attempt: loaded.next_attempt,
                    tip: loaded.tip,
                    records: loaded.records,
                    writable: true,
                    lock_held: true,
                })
            }
            Err(e) => {
                std::fs::remove_file(dir.join("LOCK")).ok();
                Err(e)
            }
        }
    }

    /// Open an existing bundle read-only: same verification, no writer
    /// lock, and every mutating call is refused.
    pub fn open_read_only(dir: impl AsRef<Path>) -> Result<Self, String> {
        let dir = dir.as_ref().to_path_buf();
        let loaded = Self::load(&dir)?;
        Ok(Self {
            dir,
            store: loaded.store,
            journal: None,
            intents: loaded.intents,
            results: loaded.results,
            intent_order: loaded.intent_order,
            checkpoints: loaded.checkpoints,
            manifest: loaded.manifest,
            next_attempt: loaded.next_attempt,
            tip: loaded.tip,
            records: loaded.records,
            writable: false,
            lock_held: false,
        })
    }

    /// Remove a stale writer lock left by a crashed process. Fails when
    /// the bundle is not locked, so breaking a live writer's lock is an
    /// explicit operator decision with a visible error when misused.
    pub fn break_lock(dir: impl AsRef<Path>) -> Result<(), String> {
        let lock = dir.as_ref().join("LOCK");
        if !lock.exists() {
            return Err(format!("bundle '{}' is not locked", dir.as_ref().display()));
        }
        std::fs::remove_file(&lock).map_err(|e| format!("bundle io: {e}"))?;
        Ok(())
    }

    /// Reserve budget and append an intent. All input objects must already
    /// exist and rehash cleanly. Returns the new attempt id.
    pub fn begin_attempt(
        &mut self,
        op: CampaignOp,
        parent: &str,
        inputs: &[ArtifactId],
        cost_actions: u64,
    ) -> Result<String, String> {
        self.require_writable()?;
        if parent.trim().is_empty() {
            return Err("parent state identity must not be empty".into());
        }
        for id in inputs {
            self.store.get(*id).map_err(|e| e.to_string())?;
        }
        let mut n = self.next_attempt;
        while self.intents.contains_key(&format!("a{n}")) {
            n += 1;
        }
        let attempt = format!("a{n}");
        self.next_attempt = n + 1;
        let body = RecordBody::Intent {
            attempt: attempt.clone(),
            op,
            parent: parent.into(),
            inputs: inputs.iter().map(|id| id.to_hex()).collect(),
            cost_actions,
        };
        self.append(body)?;
        self.intents.insert(
            attempt.clone(),
            AttemptIntent {
                op,
                parent: parent.into(),
                inputs: inputs.to_vec(),
                cost_actions,
            },
        );
        self.intent_order.push(attempt.clone());
        Ok(attempt)
    }

    /// Validate output objects and commit the result for an attempt that
    /// has an intent. Repeating the identical result is ignored (transport
    /// duplication never double-charges); a conflicting result is refused.
    pub fn commit_result(
        &mut self,
        attempt: &str,
        outcome: AttemptOutcome,
        outputs: &[ArtifactId],
        note: &str,
    ) -> Result<CommitDisposition, String> {
        self.require_writable()?;
        if !self.intents.contains_key(attempt) {
            return Err(format!("no intent for attempt '{attempt}'"));
        }
        for id in outputs {
            self.store.get(*id).map_err(|e| e.to_string())?;
        }
        if let Some(existing) = self.results.get(attempt) {
            if existing.outcome == outcome && existing.outputs == outputs && existing.note == note {
                return Ok(CommitDisposition::DuplicateIgnored);
            }
            return Err(format!("conflicting result for attempt '{attempt}'"));
        }
        let body = RecordBody::Result {
            attempt: attempt.into(),
            outcome: match outcome {
                AttemptOutcome::Completed => StoredOutcome::Completed,
                AttemptOutcome::Failed => StoredOutcome::Failed,
            },
            outputs: outputs.iter().map(|id| id.to_hex()).collect(),
            note: note.into(),
        };
        self.append(body)?;
        self.results.insert(
            attempt.into(),
            AttemptResult {
                outcome,
                outputs: outputs.to_vec(),
                note: note.into(),
            },
        );
        Ok(CommitDisposition::Committed)
    }

    /// Resume state of one attempt id.
    pub fn attempt_state(&self, attempt: &str) -> AttemptState {
        if let Some(result) = self.results.get(attempt) {
            return match result.outcome {
                AttemptOutcome::Completed => AttemptState::Completed,
                AttemptOutcome::Failed => AttemptState::Failed,
            };
        }
        if self.intents.contains_key(attempt) {
            return AttemptState::Interrupted;
        }
        AttemptState::NeverStarted
    }

    /// Verified intent for one attempt, if any.
    pub fn intent(&self, attempt: &str) -> Option<&AttemptIntent> {
        self.intents.get(attempt)
    }

    /// Verified result for one attempt, if any.
    pub fn result(&self, attempt: &str) -> Option<&AttemptResult> {
        self.results.get(attempt)
    }

    /// Attempts with an intent but no result, in attempt order.
    pub fn interrupted(&self) -> Vec<String> {
        self.intent_order
            .iter()
            .filter(|a| !self.results.contains_key(*a))
            .cloned()
            .collect()
    }

    /// Total action cost charged across all intents.
    pub fn total_charged(&self) -> u64 {
        self.intents.values().map(|i| i.cost_actions).sum()
    }

    /// Attempt ids in dispatch order.
    pub fn attempts_in_order(&self) -> &[String] {
        &self.intent_order
    }

    /// Manifest content id pinned at creation, if any.
    pub fn manifest_id(&self) -> Option<&str> {
        self.manifest.as_deref()
    }

    /// Append a coordinator usage snapshot. The latest checkpoint is the
    /// resume state; older ones are audit history.
    pub fn checkpoint(&mut self, state: &CoordinatorCheckpoint) -> Result<(), String> {
        self.require_writable()?;
        self.append(RecordBody::Checkpoint {
            state: state.clone(),
        })?;
        self.checkpoints.push(state.clone());
        Ok(())
    }

    /// Latest coordinator snapshot, if any step has checkpointed.
    pub fn latest_checkpoint(&self) -> Option<&CoordinatorCheckpoint> {
        self.checkpoints.last()
    }

    /// Journal hash-chain tip: the integrity anchor for reports.
    pub fn tip(&self) -> ArtifactId {
        self.tip
    }

    /// Store raw bytes as a content-addressed object.
    pub fn put_object(&self, bytes: impl AsRef<[u8]>) -> Result<ArtifactId, String> {
        self.require_writable()?;
        self.store.put(bytes).map_err(|e| e.to_string())
    }

    /// Read and rehash-verify an object.
    pub fn get_object(&self, id: ArtifactId) -> Result<Vec<u8>, String> {
        self.store.get(id).map_err(|e| e.to_string())
    }

    /// Flush and release the writer lock.
    pub fn close(mut self) -> Result<(), String> {
        if self.lock_held {
            if let Some(journal) = &self.journal {
                journal.sync_all().map_err(|e| format!("bundle io: {e}"))?;
            }
            std::fs::remove_file(self.dir.join("LOCK")).map_err(|e| format!("bundle io: {e}"))?;
            self.lock_held = false;
        }
        Ok(())
    }

    /// Take the writer lock, refusing concurrent writers.
    fn take_lock(dir: &Path) -> Result<(), String> {
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(dir.join("LOCK"))
        {
            Ok(_) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => Err(format!(
                "bundle '{}' is locked by another writer",
                dir.display()
            )),
            Err(e) => Err(format!("bundle io: {e}")),
        }
    }

    /// Append handle for the journal file.
    fn append_handle(dir: &Path) -> Result<File, String> {
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(dir.join("journal.jsonl"))
            .map_err(|e| format!("bundle io: {e}"))
    }

    /// Append one record and fsync it before updating the in-memory index.
    fn append(&mut self, body: RecordBody) -> Result<(), String> {
        let seq = self.records + 1;
        let prev = self.tip.to_hex();
        let id = chain_hash(seq, &prev, &body);
        let line = serde_json::to_string(&Record { seq, prev, body })
            .map_err(|e| format!("bundle io: {e}"))?;
        let journal = self
            .journal
            .as_mut()
            .ok_or_else(|| "bundle opened read-only".to_string())?;
        journal
            .write_all(format!("{line}\n").as_bytes())
            .map_err(|e| format!("bundle io: {e}"))?;
        journal.sync_all().map_err(|e| format!("bundle io: {e}"))?;
        self.tip = id;
        self.records = seq;
        Ok(())
    }

    /// Load and fully verify a bundle directory: meta version, journal
    /// chain (sequence, previous-hash, no duplicate or dangling records),
    /// and every referenced object.
    fn load(dir: &Path) -> Result<Loaded, String> {
        let meta_path = dir.join("meta.json");
        if !meta_path.exists() {
            return Err(format!("bundle '{}' has no meta.json", dir.display()));
        }
        let meta_text =
            std::fs::read_to_string(&meta_path).map_err(|e| format!("bundle io: {e}"))?;
        let meta: BundleMeta =
            serde_json::from_str(&meta_text).map_err(|e| format!("bundle meta: {e}"))?;
        if meta.schema != BUNDLE_SCHEMA {
            return Err(format!(
                "bundle schema is '{}', want '{BUNDLE_SCHEMA}'",
                meta.schema
            ));
        }
        if meta.version != BUNDLE_VERSION {
            return Err(format!(
                "unknown bundle version {}, want {BUNDLE_VERSION}",
                meta.version
            ));
        }
        let journal_path = dir.join("journal.jsonl");
        if !journal_path.exists() {
            return Err(format!("bundle '{}' journal is missing", dir.display()));
        }
        let store =
            DurableStore::open(dir.join("objects")).map_err(|e| format!("bundle io: {e}"))?;
        let text = std::fs::read_to_string(&journal_path).map_err(|e| format!("bundle io: {e}"))?;
        let mut intents = HashMap::new();
        let mut results = HashMap::new();
        let mut intent_order = Vec::new();
        let mut checkpoints = Vec::new();
        let mut tip = genesis_tip();
        let mut seq = 0u64;
        for (n, line) in text.lines().enumerate() {
            let line_no = n + 1;
            if line.trim().is_empty() {
                continue;
            }
            let record: Record = serde_json::from_str(line)
                .map_err(|e| format!("journal line {line_no} is malformed: {e}"))?;
            seq += 1;
            if record.seq != seq {
                return Err(format!(
                    "journal line {line_no} has seq {}, want {seq}",
                    record.seq
                ));
            }
            if record.prev != tip.to_hex() {
                return Err(format!("journal chain broken at line {line_no}"));
            }
            tip = chain_hash(record.seq, &record.prev, &record.body);
            match record.body {
                RecordBody::Intent {
                    attempt,
                    op,
                    parent,
                    inputs,
                    cost_actions,
                } => {
                    if intents.contains_key(&attempt) {
                        return Err(format!("duplicate intent for attempt '{attempt}'"));
                    }
                    let mut verified = Vec::with_capacity(inputs.len());
                    for hex in &inputs {
                        let id = ArtifactId::from_hex(hex).ok_or_else(|| {
                            format!("journal line {line_no} has invalid hash '{hex}'")
                        })?;
                        if !store.contains(id) {
                            return Err(format!(
                                "journal line {line_no} references missing object {hex}"
                            ));
                        }
                        verified.push(id);
                    }
                    intents.insert(
                        attempt.clone(),
                        AttemptIntent {
                            op,
                            parent,
                            inputs: verified,
                            cost_actions,
                        },
                    );
                    intent_order.push(attempt);
                }
                RecordBody::Result {
                    attempt,
                    outcome,
                    outputs,
                    note,
                } => {
                    if !intents.contains_key(&attempt) {
                        return Err(format!("result without intent for attempt '{attempt}'"));
                    }
                    if results.contains_key(&attempt) {
                        return Err(format!("duplicate result for attempt '{attempt}'"));
                    }
                    let mut verified = Vec::with_capacity(outputs.len());
                    for hex in &outputs {
                        let id = ArtifactId::from_hex(hex).ok_or_else(|| {
                            format!("journal line {line_no} has invalid hash '{hex}'")
                        })?;
                        if !store.contains(id) {
                            return Err(format!(
                                "journal line {line_no} references missing object {hex}"
                            ));
                        }
                        verified.push(id);
                    }
                    results.insert(
                        attempt,
                        AttemptResult {
                            outcome: match outcome {
                                StoredOutcome::Completed => AttemptOutcome::Completed,
                                StoredOutcome::Failed => AttemptOutcome::Failed,
                            },
                            outputs: verified,
                            note,
                        },
                    );
                }
                RecordBody::Checkpoint { state } => {
                    checkpoints.push(state);
                }
            }
        }
        let next_attempt = intents.len() as u64 + 1;
        Ok(Loaded {
            store,
            intents,
            results,
            intent_order,
            checkpoints,
            manifest: meta.manifest,
            next_attempt,
            tip,
            records: seq,
        })
    }

    /// Refuse mutation on a read-only open.
    fn require_writable(&self) -> Result<(), String> {
        if self.writable {
            Ok(())
        } else {
            Err("bundle opened read-only".into())
        }
    }
}

impl Drop for Bundle {
    /// Best-effort lock release. A crashed process never runs this, which
    /// is exactly why [`Bundle::break_lock`] exists.
    fn drop(&mut self) {
        if self.lock_held {
            std::fs::remove_file(self.dir.join("LOCK")).ok();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("physis-r2-bundle-{}-{name}", std::process::id()))
    }

    fn cleanup(dir: &Path) {
        std::fs::remove_dir_all(dir).ok();
    }

    #[test]
    fn begin_commit_roundtrip() {
        let dir = scratch("roundtrip");
        cleanup(&dir);
        let mut b = Bundle::create(&dir, None).unwrap();
        let out = b.put_object(b"result-bytes").unwrap();
        let a = b
            .begin_attempt(CampaignOp::Hypothesize, "genesis", &[], 2)
            .unwrap();
        assert_eq!(
            b.commit_result(&a, AttemptOutcome::Completed, &[out], "ok")
                .unwrap(),
            CommitDisposition::Committed
        );
        assert_eq!(b.attempt_state(&a), AttemptState::Completed);
        assert_eq!(b.total_charged(), 2);
        b.close().unwrap();
        cleanup(&dir);
    }

    #[test]
    fn kill_after_begin_leaves_interrupted_charge_kept() {
        let dir = scratch("killbegin");
        cleanup(&dir);
        let attempt = {
            let mut b = Bundle::create(&dir, None).unwrap();
            let a = b
                .begin_attempt(CampaignOp::Prove, "genesis", &[], 3)
                .unwrap();
            b.close().unwrap();
            a
        };
        // Kill before any result: drop without commit, then reopen.
        let b = Bundle::open(&dir).unwrap();
        assert_eq!(b.attempt_state(&attempt), AttemptState::Interrupted);
        assert_eq!(b.interrupted(), vec![attempt]);
        assert_eq!(b.total_charged(), 3);
        cleanup(&dir);
    }

    #[test]
    fn kill_after_commit_keeps_completed() {
        let dir = scratch("killcommit");
        cleanup(&dir);
        let (attempt, obj) = {
            let mut b = Bundle::create(&dir, None).unwrap();
            let o = b.put_object(b"durable").unwrap();
            let a = b
                .begin_attempt(CampaignOp::Cite, "genesis", &[], 1)
                .unwrap();
            b.commit_result(&a, AttemptOutcome::Completed, &[o], "cited")
                .unwrap();
            b.close().unwrap();
            (a, o)
        };
        let b = Bundle::open(&dir).unwrap();
        assert_eq!(b.attempt_state(&attempt), AttemptState::Completed);
        assert_eq!(b.get_object(obj).unwrap(), b"durable");
        assert_eq!(b.total_charged(), 1);
        cleanup(&dir);
    }

    #[test]
    fn duplicate_commit_ignored_not_double_charged() {
        let dir = scratch("dupcommit");
        cleanup(&dir);
        let mut b = Bundle::create(&dir, None).unwrap();
        let out = b.put_object(b"x").unwrap();
        let a = b
            .begin_attempt(CampaignOp::Enclose, "genesis", &[], 1)
            .unwrap();
        assert_eq!(
            b.commit_result(&a, AttemptOutcome::Completed, &[out], "n")
                .unwrap(),
            CommitDisposition::Committed
        );
        assert_eq!(
            b.commit_result(&a, AttemptOutcome::Completed, &[out], "n")
                .unwrap(),
            CommitDisposition::DuplicateIgnored
        );
        assert_eq!(b.total_charged(), 1);
        cleanup(&dir);
    }

    #[test]
    fn conflicting_commit_refused() {
        let dir = scratch("conflict");
        cleanup(&dir);
        let mut b = Bundle::create(&dir, None).unwrap();
        let out = b.put_object(b"x").unwrap();
        let a = b
            .begin_attempt(CampaignOp::Enclose, "genesis", &[], 1)
            .unwrap();
        b.commit_result(&a, AttemptOutcome::Completed, &[out], "n")
            .unwrap();
        let err = b
            .commit_result(&a, AttemptOutcome::Failed, &[out], "n")
            .unwrap_err();
        assert!(err.contains("conflict"), "{err}");
        cleanup(&dir);
    }

    #[test]
    fn commit_without_intent_refused() {
        let dir = scratch("nointent");
        cleanup(&dir);
        let mut b = Bundle::create(&dir, None).unwrap();
        let err = b
            .commit_result("a9", AttemptOutcome::Completed, &[], "n")
            .unwrap_err();
        assert!(err.contains("no intent"), "{err}");
        cleanup(&dir);
    }

    #[test]
    fn missing_input_refused() {
        let dir = scratch("missingin");
        cleanup(&dir);
        let mut b = Bundle::create(&dir, None).unwrap();
        let err = b
            .begin_attempt(CampaignOp::Prove, "genesis", &[ArtifactId::of(b"nope")], 1)
            .unwrap_err();
        assert!(err.contains("missing"), "{err}");
        cleanup(&dir);
    }

    #[test]
    fn missing_output_refused() {
        let dir = scratch("missingout");
        cleanup(&dir);
        let mut b = Bundle::create(&dir, None).unwrap();
        let a = b
            .begin_attempt(CampaignOp::Prove, "genesis", &[], 1)
            .unwrap();
        let err = b
            .commit_result(
                &a,
                AttemptOutcome::Completed,
                &[ArtifactId::of(b"nope")],
                "n",
            )
            .unwrap_err();
        assert!(err.contains("missing"), "{err}");
        cleanup(&dir);
    }

    #[test]
    fn corrupt_journal_refused_on_open() {
        let dir = scratch("corruptjl");
        cleanup(&dir);
        {
            let mut b = Bundle::create(&dir, None).unwrap();
            b.begin_attempt(CampaignOp::Prove, "genesis", &[], 1)
                .unwrap();
            b.close().unwrap();
        }
        std::fs::write(dir.join("journal.jsonl"), "{torn\n").unwrap();
        let err = Bundle::open(&dir).unwrap_err();
        assert!(err.contains("line 1"), "{err}");
        cleanup(&dir);
    }

    #[test]
    fn tampered_object_detected() {
        let dir = scratch("tamperobj");
        cleanup(&dir);
        let b = Bundle::create(&dir, None).unwrap();
        let id = b.put_object(b"pristine").unwrap();
        std::fs::write(dir.join("objects").join(id.to_hex()), b"forged").unwrap();
        let err = b.get_object(id).unwrap_err();
        assert!(err.contains("corrupt"), "{err}");
        cleanup(&dir);
    }

    #[test]
    fn second_writer_refused_until_close() {
        let dir = scratch("locking");
        cleanup(&dir);
        let b = Bundle::create(&dir, None).unwrap();
        let err = Bundle::open(&dir).unwrap_err();
        assert!(err.contains("locked"), "{err}");
        b.close().unwrap();
        Bundle::open(&dir).unwrap().close().unwrap();
        cleanup(&dir);
    }

    #[test]
    fn kill9_leaves_lock_break_recovers() {
        let dir = scratch("kill9");
        cleanup(&dir);
        let attempt = {
            let mut b = Bundle::create(&dir, None).unwrap();
            let a = b
                .begin_attempt(CampaignOp::Prove, "genesis", &[], 3)
                .unwrap();
            std::mem::forget(b);
            a
        };
        let err = Bundle::open(&dir).unwrap_err();
        assert!(err.contains("locked"), "{err}");
        Bundle::break_lock(&dir).unwrap();
        let b = Bundle::open(&dir).unwrap();
        assert_eq!(b.attempt_state(&attempt), AttemptState::Interrupted);
        assert_eq!(b.total_charged(), 3);
        cleanup(&dir);
    }

    #[test]
    fn deleted_object_detected_on_open() {
        let dir = scratch("delobj");
        cleanup(&dir);
        let obj = {
            let mut b = Bundle::create(&dir, None).unwrap();
            let o = b.put_object(b"v").unwrap();
            let a = b
                .begin_attempt(CampaignOp::Cite, "genesis", &[], 1)
                .unwrap();
            b.commit_result(&a, AttemptOutcome::Completed, &[o], "n")
                .unwrap();
            b.close().unwrap();
            o
        };
        std::fs::remove_file(dir.join("objects").join(obj.to_hex())).unwrap();
        let err = Bundle::open(&dir).unwrap_err();
        assert!(err.contains("missing"), "{err}");
        cleanup(&dir);
    }

    #[test]
    fn read_only_sees_writes_and_refuses_mutation() {
        let dir = scratch("readonly");
        cleanup(&dir);
        let attempt = {
            let mut b = Bundle::create(&dir, None).unwrap();
            let a = b
                .begin_attempt(CampaignOp::Cite, "genesis", &[], 1)
                .unwrap();
            b.commit_result(&a, AttemptOutcome::Completed, &[], "n")
                .unwrap();
            a
        };
        let mut r = Bundle::open_read_only(&dir).unwrap();
        assert_eq!(r.attempt_state(&attempt), AttemptState::Completed);
        let err = r
            .begin_attempt(CampaignOp::Cite, "genesis", &[], 1)
            .unwrap_err();
        assert!(err.contains("read-only"), "{err}");
        cleanup(&dir);
    }

    #[test]
    fn create_refuses_existing() {
        let dir = scratch("exists");
        cleanup(&dir);
        Bundle::create(&dir, None).unwrap().close().unwrap();
        let err = Bundle::create(&dir, None).unwrap_err();
        assert!(err.contains("exists"), "{err}");
        cleanup(&dir);
    }

    #[test]
    fn unknown_meta_version_refused() {
        let dir = scratch("metaver");
        cleanup(&dir);
        Bundle::create(&dir, None).unwrap().close().unwrap();
        std::fs::write(
            dir.join("meta.json"),
            r#"{"schema":"physis-campaign-bundle","version":99,"manifest":null,"created_ms":0}"#,
        )
        .unwrap();
        let err = Bundle::open(&dir).unwrap_err();
        assert!(err.contains("version"), "{err}");
        cleanup(&dir);
    }

    #[test]
    fn r7_wrong_meta_schema_refused() {
        let dir = scratch("r7-schema");
        cleanup(&dir);
        Bundle::create(&dir, None).unwrap().close().unwrap();
        std::fs::write(
            dir.join("meta.json"),
            r#"{"schema":"physis-something-else","version":1,"manifest":null,"created_ms":0}"#,
        )
        .unwrap();
        let err = Bundle::open(&dir).unwrap_err();
        assert!(err.contains("schema"), "{err}");
        cleanup(&dir);
    }

    #[test]
    fn failed_attempt_is_charged_and_reported() {
        let dir = scratch("failed");
        cleanup(&dir);
        let mut b = Bundle::create(&dir, None).unwrap();
        let a = b
            .begin_attempt(CampaignOp::Prove, "genesis", &[], 2)
            .unwrap();
        assert_eq!(
            b.commit_result(&a, AttemptOutcome::Failed, &[], "worker died")
                .unwrap(),
            CommitDisposition::Committed
        );
        assert_eq!(b.attempt_state(&a), AttemptState::Failed);
        assert_eq!(b.total_charged(), 2);
        assert!(b.interrupted().is_empty());
        let intent = b.intent(&a).unwrap();
        assert_eq!(intent.op, CampaignOp::Prove);
        assert_eq!(intent.cost_actions, 2);
        assert_eq!(b.result(&a).unwrap().note, "worker died");
        cleanup(&dir);
    }

    #[test]
    fn never_started_is_unknown() {
        let dir = scratch("never");
        cleanup(&dir);
        let b = Bundle::create(&dir, None).unwrap();
        assert_eq!(b.attempt_state("a1"), AttemptState::NeverStarted);
        assert!(b.interrupted().is_empty());
        cleanup(&dir);
    }
}
