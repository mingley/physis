//! Serial campaign coordinator (R4, spec 022).
//!
//! Dispatch on top of the R1 manifest, the R2 bundle, and the C7.1 gap
//! selector. [`Campaign::open`] pins one manifest revision to one bundle
//! directory (create-or-open; a changed manifest is a new revision and is
//! refused, never a silent budget reset). [`Campaign::step_once`]
//! reconciles interrupted attempts, checks persisted stop state and
//! budgets, selects one eligible gap deterministically (C7.1 priority
//! plus statement-hash tie-break, filtered by campaign scope,
//! operations, and the persisted blocked set), reserves an attempt,
//! executes at most one action through the existing role/trust/budget
//! gates, commits the result, and checkpoints usage. Blocked work is
//! not retried until a relevant input changes; repeats grow the
//! no-progress streak toward the manifest's finite limit.
//!
//! [`status_text`] renders a read-only projection from the bundle alone:
//! status needs no lab and takes no lock.

use std::collections::BTreeMap;
use std::path::Path;
use std::time::Instant;

use physis_core::judgment::GapReason;
use physis_core::ArtifactId;
use serde::{Deserialize, Serialize};

use crate::bundle::{AttemptOutcome, Bundle};
use crate::campaign::{op_command, op_name, parse_manifest, validate, CampaignOp, Manifest};
use crate::lab::{step_priority, Lab, StepCandidate};
use crate::protocol::{Command, Response};

/// Status rendering version.
pub const CAMPAIGN_STATUS_VERSION: u32 = 1;

/// Explicit stop reasons (spec 022 stop policy). `objective-met` is not
/// among them: R4 cannot judge free-text success conditions, so it stops
/// on budgets, eligibility, and progress — never a claimed result.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StopReason {
    /// A finite budget ran out.
    BudgetExhausted,
    /// No gap in scope, or every remaining gap is blocked with
    /// unchanged inputs and the streak has no room left to report it.
    NoEligibleAction,
    /// The manifest's no-progress limit stopped repeated blocked work.
    NoProgress,
    /// Bundle, worker, or dispatch failure. Never a scientific verdict.
    OperationalFailure,
    /// The manifest requests a human pause.
    HumanPause,
}

impl StopReason {
    /// Wire spelling.
    pub fn as_str(self) -> &'static str {
        match self {
            StopReason::BudgetExhausted => "budget-exhausted",
            StopReason::NoEligibleAction => "no-eligible-action",
            StopReason::NoProgress => "no-progress",
            StopReason::OperationalFailure => "operational-failure",
            StopReason::HumanPause => "human-pause",
        }
    }
}

/// Persisted stop record: the reason plus a human detail.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StopRecord {
    /// Why the campaign stopped.
    pub reason: StopReason,
    /// Human detail (which budget, which inputs, what failed).
    pub detail: String,
}

/// Coordinator state appended to the bundle journal after every step.
/// Everything the next process needs to resume deterministically.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoordinatorCheckpoint {
    /// Cumulative measured dispatch wall time in milliseconds.
    pub wall_ms_used: u64,
    /// Cumulative hypothesize candidates evaluated.
    pub candidates_used: u64,
    /// Consecutive steps without progress.
    pub no_progress_streak: u64,
    /// Blocked action key to relevant-input identity hash. A blocked
    /// action is not retried until its inputs hash changes.
    pub blocked: BTreeMap<String, String>,
    /// Sticky stop record. A stopped campaign stays stopped.
    pub stopped: Option<StopRecord>,
}

/// Budget usage compared against the manifest.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Usage {
    /// Charged actions (sum of intent costs).
    pub actions: u64,
    /// Started attempts (charged even when they fail).
    pub attempts: u64,
    /// Measured dispatch wall time in milliseconds.
    pub wall_ms: u64,
    /// Hypothesize candidates evaluated.
    pub candidates: u64,
}

/// Pure stop decision: sticky stop first, then human pause, then budgets,
/// then the no-progress streak. Returns the record to persist, if any.
pub fn stop_reason(
    manifest: &Manifest,
    usage: &Usage,
    streak: u64,
    stopped: Option<&StopRecord>,
) -> Option<StopRecord> {
    if let Some(record) = stopped {
        return Some(record.clone());
    }
    if manifest.stop.human_pause {
        return Some(StopRecord {
            reason: StopReason::HumanPause,
            detail: "manifest requests a human pause".into(),
        });
    }
    let budget = &manifest.budget;
    if usage.actions >= budget.actions {
        return Some(StopRecord {
            reason: StopReason::BudgetExhausted,
            detail: format!("actions {}/{}", usage.actions, budget.actions),
        });
    }
    if usage.attempts >= budget.attempts {
        return Some(StopRecord {
            reason: StopReason::BudgetExhausted,
            detail: format!("attempts {}/{}", usage.attempts, budget.attempts),
        });
    }
    if usage.wall_ms >= budget.wall_time_secs.saturating_mul(1000) {
        return Some(StopRecord {
            reason: StopReason::BudgetExhausted,
            detail: format!(
                "wall time {}ms/{}ms",
                usage.wall_ms,
                budget.wall_time_secs.saturating_mul(1000)
            ),
        });
    }
    if usage.candidates >= budget.candidates {
        return Some(StopRecord {
            reason: StopReason::BudgetExhausted,
            detail: format!("candidates {}/{}", usage.candidates, budget.candidates),
        });
    }
    if streak >= manifest.stop.no_progress_limit {
        return Some(StopRecord {
            reason: StopReason::NoProgress,
            detail: format!(
                "no progress for {streak} steps (limit {})",
                manifest.stop.no_progress_limit
            ),
        });
    }
    None
}

/// Parse the `explored=N` candidate count from a hypothesize footer.
/// Unparseable output counts zero, never a fabricated count.
fn parse_explored(text: &str) -> u64 {
    let Some(at) = text.find("explored=") else {
        return 0;
    };
    text[at + "explored=".len()..]
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap_or(0)
}

/// What the selector decided for one candidate.
#[derive(Clone, Copy)]
enum PlannedAction {
    /// Dispatch this registered operation (budget already checked).
    Dispatch(CampaignOp),
    /// Emit the C7.1 work item without dispatching.
    Blocked,
    /// Preserve without dispatching (computability / open problem).
    None,
    /// Not eligible for this campaign's operation set; skip silently.
    Ineligible,
}

/// Map one sorted candidate to a campaign action.
fn plan_for(sel: &StepCandidate, ops: &[CampaignOp]) -> PlannedAction {
    let allowed = |op: CampaignOp| ops.contains(&op);
    match sel.gap {
        GapReason::MissingTheorem if sel.supported => {
            if allowed(CampaignOp::Prove) {
                PlannedAction::Dispatch(CampaignOp::Prove)
            } else {
                PlannedAction::Ineligible
            }
        }
        GapReason::MissingTheorem => {
            if allowed(CampaignOp::Formalize) {
                PlannedAction::Dispatch(CampaignOp::Formalize)
            } else {
                PlannedAction::Blocked
            }
        }
        GapReason::MissingDataset if sel.registered_dataset => {
            if allowed(CampaignOp::Cite) {
                PlannedAction::Dispatch(CampaignOp::Cite)
            } else {
                PlannedAction::Ineligible
            }
        }
        GapReason::MissingDataset => PlannedAction::Blocked,
        GapReason::InsufficientPrecision if sel.encloseable => {
            if allowed(CampaignOp::Enclose) {
                PlannedAction::Dispatch(CampaignOp::Enclose)
            } else {
                PlannedAction::Ineligible
            }
        }
        GapReason::InsufficientPrecision => PlannedAction::Blocked,
        GapReason::UnsupportedFormalPrimitive | GapReason::ComputationallyIntractable => {
            PlannedAction::Blocked
        }
        GapReason::LogicallyUndecidable => PlannedAction::None,
        GapReason::ScientificOpenProblem => {
            if allowed(CampaignOp::Hypothesize) {
                PlannedAction::Dispatch(CampaignOp::Hypothesize)
            } else {
                PlannedAction::None
            }
        }
    }
}

/// Blocked-set key for one candidate and action name.
fn blocked_key(sel: &StepCandidate, action: &str) -> String {
    format!(
        "{}:{}:{}:{}",
        action,
        sel.theory,
        sel.claim.id_str(),
        sel.gap.as_str()
    )
}

/// Relevant-input identity: statement hash plus the eligibility flags
/// that would justify a retry if they flipped, plus per-action extras
/// (lab role for authority blocks, knob state for hypothesize).
fn inputs_hash(sel: &StepCandidate, extra: &str) -> String {
    ArtifactId::of(
        format!(
            "{}:{}:{}:{}:{}",
            sel.claim.statement_hash().to_hex(),
            sel.supported,
            sel.registered_dataset,
            sel.encloseable,
            extra
        )
        .as_bytes(),
    )
    .to_hex()
}

/// One manifest pinned to one bundle, dispatching against one lab.
pub struct Campaign<'l> {
    lab: &'l mut Lab,
    bundle: Bundle,
    manifest: Manifest,
    manifest_id: ArtifactId,
}

impl<'l> Campaign<'l> {
    /// Open the campaign in `dir` for `manifest_text`: create the bundle
    /// on first use (recording the manifest id and bytes), or open the
    /// existing bundle and refuse when its pinned manifest differs.
    /// Validates the manifest against the live lab before returning.
    pub fn open(lab: &'l mut Lab, manifest_text: &str, dir: &Path) -> Result<Self, String> {
        let (manifest, manifest_id) = parse_manifest(manifest_text)?;
        validate(&manifest, lab)?;
        let bundle = if dir.exists() {
            let bundle = Bundle::open(dir)?;
            match bundle.manifest_id() {
                Some(pinned) if pinned == manifest_id.to_hex() => bundle,
                Some(pinned) => {
                    return Err(format!(
                        "manifest revision change: bundle pins {pinned}, manifest is {}; \
                         start a new campaign revision in a new directory",
                        manifest_id.to_hex()
                    ));
                }
                None => {
                    return Err("bundle has no pinned manifest; start a new campaign".into());
                }
            }
        } else {
            let bundle = Bundle::create(dir, Some(manifest_id))?;
            let stored = bundle.put_object(manifest_text.as_bytes())?;
            debug_assert_eq!(stored, manifest_id);
            bundle
        };
        Ok(Self {
            lab,
            bundle,
            manifest,
            manifest_id,
        })
    }

    /// Current usage: actions charged and attempts started from the
    /// journal, wall time and candidates from the latest checkpoint.
    fn usage(&self) -> Usage {
        let checkpoint = self.bundle.latest_checkpoint().cloned().unwrap_or_default();
        Usage {
            actions: self.bundle.total_charged(),
            attempts: self.bundle.attempts_in_order().len() as u64,
            wall_ms: checkpoint.wall_ms_used,
            candidates: checkpoint.candidates_used,
        }
    }

    /// Latest persisted coordinator state, or a fresh one.
    fn checkpoint_state(&self) -> CoordinatorCheckpoint {
        self.bundle.latest_checkpoint().cloned().unwrap_or_default()
    }

    /// Run at most one coordinated action: reconcile interrupted
    /// attempts, enforce stop state and budgets, select one eligible
    /// gap, reserve the attempt, execute, commit, and checkpoint.
    pub fn step_once(&mut self) -> Response {
        let started = Instant::now();
        let mut head = String::from("campaign step\n");
        let mut state = self.checkpoint_state();

        for id in self.bundle.interrupted() {
            match self.bundle.commit_result(
                &id,
                AttemptOutcome::Failed,
                &[],
                "interrupted: intent without result; reconciled unresolved before dispatch (R4)",
            ) {
                Ok(_) => head.push_str(&format!("reconciled  {id}  interrupted\n")),
                Err(e) => {
                    return self.stop_now(
                        &mut state,
                        StopReason::OperationalFailure,
                        format!("reconcile of {id} failed: {e}"),
                        started,
                    );
                }
            }
        }

        let usage = self.usage();
        if let Some(record) = stop_reason(
            &self.manifest,
            &usage,
            state.no_progress_streak,
            state.stopped.as_ref(),
        ) {
            if state.stopped.is_none() {
                return self.stop_now(&mut state, record.reason, record.detail, started);
            }
            head.push_str(&format!(
                "stop  {}: {}\n",
                record.reason.as_str(),
                record.detail
            ));
            return Response::ok(head);
        }

        let candidates = match self.lab.collect_step_candidates(None) {
            Ok(c) => c,
            Err(e) => {
                return self.stop_now(
                    &mut state,
                    StopReason::OperationalFailure,
                    format!("candidate collection failed: {e}"),
                    started,
                );
            }
        };
        let ops = &self.manifest.candidate_space.operations;
        let in_scope = candidates
            .iter()
            .filter(|c| {
                (self.manifest.objective.theories.is_empty()
                    || self.manifest.objective.theories.contains(&c.theory))
                    && (self.manifest.objective.statements.is_empty()
                        || self
                            .manifest
                            .objective
                            .statements
                            .contains(&c.claim.id_str().to_string()))
            })
            .collect::<Vec<_>>();
        // First unblocked eligible candidate; first repeat-blocked as the
        // fallback report; op-ineligible candidates are skipped.
        let mut selected: Option<(&StepCandidate, PlannedAction, String, String)> = None;
        let mut repeat: Option<&StepCandidate> = None;
        let mut eligible = 0usize;
        for sel in in_scope {
            let planned = plan_for(sel, ops);
            let action_name = match planned {
                PlannedAction::Dispatch(op) => op_name(op),
                PlannedAction::Blocked => "blocked",
                PlannedAction::None => "none",
                PlannedAction::Ineligible => continue,
            };
            eligible += 1;
            let key = blocked_key(sel, action_name);
            // Dispatch inputs include the lab role: an authority block
            // repeats until the role changes, and completed work is not
            // retried under the same role. Hypothesize additionally
            // binds the knob state it searched.
            let extra = match planned {
                PlannedAction::Dispatch(CampaignOp::Hypothesize) => format!(
                    "{}:{}",
                    self.lab.role().as_str(),
                    self.lab.knob_state_identity()
                ),
                PlannedAction::Dispatch(_) => self.lab.role().as_str().to_string(),
                _ => String::new(),
            };
            let inputs = inputs_hash(sel, &extra);
            if state.blocked.get(&key) == Some(&inputs) {
                if repeat.is_none() {
                    repeat = Some(sel);
                }
                continue;
            }
            selected = Some((sel, planned, key, inputs));
            break;
        }

        let Some((sel, planned, key, inputs)) = selected else {
            if let Some(sel) = repeat {
                state.no_progress_streak += 1;
                let limit = self.manifest.stop.no_progress_limit;
                let streak = state.no_progress_streak;
                if streak >= limit {
                    return self.stop_now(
                        &mut state,
                        StopReason::NoProgress,
                        format!("no progress for {streak} steps (limit {limit})"),
                        started,
                    );
                }
                head.push_str(&format!(
                    "selected-repeat  {}  {}  {}  (all {eligible} eligible gaps blocked with unchanged inputs)\n",
                    sel.theory,
                    sel.claim.id_str(),
                    sel.gap.as_str(),
                ));
                return self.checkpoint_ok(
                    &mut state,
                    &mut head,
                    started,
                    0,
                    "repeat-blocked report; streak grew",
                );
            }
            return self.stop_now(
                &mut state,
                StopReason::NoEligibleAction,
                if eligible == 0 && !candidates.is_empty() {
                    "gaps exist but none match the campaign scope and operations".into()
                } else {
                    "no gap in scope".into()
                },
                started,
            );
        };

        let (prio, prio_name) = step_priority(sel);
        head.push_str(&format!(
            "selected  {}  {}  {}  {}\n",
            sel.theory,
            sel.claim.id_str(),
            sel.gap.as_str(),
            sel.claim.statement_hash().to_hex()
        ));
        head.push_str(&format!(
            "reason  priority {prio} ({prio_name}); {eligible} eligible candidates; tie-break statement-hash ascending\n"
        ));

        match planned {
            PlannedAction::Dispatch(op) => {
                self.dispatch(sel, op, &key, &inputs, &mut head, &mut state, started)
            }
            PlannedAction::Blocked | PlannedAction::None => {
                let (action, result) = match self.lab.respond_to_gap(sel) {
                    Ok(ok) => ok,
                    Err(e) => {
                        return self.stop_now(
                            &mut state,
                            StopReason::OperationalFailure,
                            format!("work-item emission failed: {e}"),
                            started,
                        );
                    }
                };
                head.push_str(&format!("action  {action}\nresult  {result}\n"));
                // New blocked information is progress; the streak only
                // grows on repeats, which selection already skipped.
                state.no_progress_streak = 0;
                state.blocked.insert(key, inputs);
                self.checkpoint_ok(
                    &mut state,
                    &mut head,
                    started,
                    0,
                    "blocked work item recorded",
                )
            }
            PlannedAction::Ineligible => unreachable!("ineligible candidates are skipped"),
        }
    }

    /// Persist a stop and report it. Checkpoint failure degrades to an
    /// error response: the stop is real but not yet durable.
    fn stop_now(
        &mut self,
        state: &mut CoordinatorCheckpoint,
        reason: StopReason,
        detail: String,
        started: Instant,
    ) -> Response {
        let record = StopRecord { reason, detail };
        state.stopped = Some(record.clone());
        state.wall_ms_used += started.elapsed().as_millis() as u64;
        let mut head = String::from("campaign step\n");
        match self.bundle.checkpoint(state) {
            Ok(()) => {
                head.push_str(&format!(
                    "stop  {}: {}\n",
                    record.reason.as_str(),
                    record.detail
                ));
                Response::ok(head)
            }
            Err(e) => Response::err(format!(
                "campaign stopped ({}: {}) but the checkpoint failed: {e}",
                record.reason.as_str(),
                record.detail
            )),
        }
    }

    /// Checkpoint usage after a non-stopped step and report success.
    fn checkpoint_ok(
        &mut self,
        state: &mut CoordinatorCheckpoint,
        head: &mut String,
        started: Instant,
        candidates: u64,
        note: &str,
    ) -> Response {
        state.wall_ms_used += started.elapsed().as_millis() as u64;
        state.candidates_used += candidates;
        match self.bundle.checkpoint(state) {
            Ok(()) => {
                let usage = self.usage();
                head.push_str(&format!(
                    "budget  actions {}/{}  attempts {}/{}  wall_ms {}/{}  candidates {}/{}\nnote  {note}\n",
                    usage.actions,
                    self.manifest.budget.actions,
                    usage.attempts,
                    self.manifest.budget.attempts,
                    usage.wall_ms,
                    self.manifest.budget.wall_time_secs.saturating_mul(1000),
                    usage.candidates,
                    self.manifest.budget.candidates,
                ));
                Response::ok(std::mem::take(head))
            }
            Err(e) => Response::err(format!("campaign checkpoint failed: {e}")),
        }
    }

    /// Reserve the attempt, execute one action, commit, and checkpoint.
    #[allow(clippy::too_many_arguments)]
    fn dispatch(
        &mut self,
        sel: &StepCandidate,
        op: CampaignOp,
        key: &str,
        inputs: &str,
        head: &mut String,
        state: &mut CoordinatorCheckpoint,
        started: Instant,
    ) -> Response {
        let claim_id = sel.claim.id_str().to_string();
        if !self.lab.role().permits(&op_command(op)) {
            let role = self.lab.role().as_str().to_string();
            head.push_str(&format!(
                "action  blocked\nauthority work item: {} on {claim_id} needs a role permitting {}; lab role is {role}\n",
                op_name(op),
                op_name(op),
            ));
            state.no_progress_streak = 0;
            // Record the selection-time inputs: the block repeats until
            // the role (or, for hypothesize, the knobs) change.
            state.blocked.insert(key.to_string(), inputs.to_string());
            return self.checkpoint_ok(state, head, started, 0, "authority block recorded");
        }
        let selection = serde_json::json!({
            "theory": sel.theory,
            "claim": claim_id,
            "statement_hash": sel.claim.statement_hash().to_hex(),
            "gap": sel.gap.as_str(),
            "op": op_name(op),
        });
        let input_id = match self.bundle.put_object(selection.to_string().as_bytes()) {
            Ok(id) => id,
            Err(e) => {
                return self.stop_now(
                    state,
                    StopReason::OperationalFailure,
                    format!("selection store failed: {e}"),
                    started,
                );
            }
        };
        let parent = self.lab.journal().tip().to_hex();
        let attempt = match self.bundle.begin_attempt(op, &parent, &[input_id], 1) {
            Ok(id) => id,
            Err(e) => {
                return self.stop_now(
                    state,
                    StopReason::OperationalFailure,
                    format!("attempt reservation failed: {e}"),
                    started,
                );
            }
        };
        head.push_str(&format!("attempt  {attempt}\n"));
        let (outcome, action, result, candidates) = self.execute(sel, op);
        head.push_str(&format!("action  {action}\nresult  {result}\n"));
        let output = serde_json::json!({
            "attempt": attempt,
            "action": action,
            "claim": claim_id,
            "statement_hash": sel.claim.statement_hash().to_hex(),
            "detail": result,
        });
        let output_id = match self.bundle.put_object(output.to_string().as_bytes()) {
            Ok(id) => id,
            Err(e) => {
                return self.stop_now(
                    state,
                    StopReason::OperationalFailure,
                    format!("output store failed for {attempt}: {e}"),
                    started,
                );
            }
        };
        let note = first_line(&result);
        match self
            .bundle
            .commit_result(&attempt, outcome, &[output_id], &note)
        {
            Ok(_) => {}
            Err(e) => {
                return self.stop_now(
                    state,
                    StopReason::OperationalFailure,
                    format!("commit failed for {attempt}: {e}"),
                    started,
                );
            }
        }
        // Every dispatch records its blocked entry: a completed proof
        // removes its gap, while cite/enclose/formalize/hypothesize gaps
        // persist and must not spin until an input changes. Selection
        // skipped every repeat, so any dispatch is first-touch progress —
        // including a first failure, which is new information.
        state.blocked.insert(key.to_string(), inputs.to_string());
        state.no_progress_streak = 0;
        self.checkpoint_ok(state, head, started, candidates, "attempt committed")
    }

    /// Execute one action through the existing lab paths. Prove, cite,
    /// and enclose reuse the C7.1 responder; formalize and hypothesize
    /// go through `exec` and its role gate.
    fn execute(
        &mut self,
        sel: &StepCandidate,
        op: CampaignOp,
    ) -> (AttemptOutcome, String, String, u64) {
        let claim_id = sel.claim.id_str().to_string();
        match op {
            CampaignOp::Prove | CampaignOp::Cite | CampaignOp::Enclose => {
                match self.lab.respond_to_gap(sel) {
                    Ok((action, result)) => (AttemptOutcome::Completed, action, result, 0),
                    Err(e) => (
                        AttemptOutcome::Failed,
                        op_name(op).to_string(),
                        format!("{} {claim_id} failed: {e}", op_name(op)),
                        0,
                    ),
                }
            }
            CampaignOp::Formalize => {
                let response = self.lab.exec(Command::Formalize {
                    claim: claim_id.clone(),
                });
                if response.exit_code() == 0 {
                    (
                        AttemptOutcome::Completed,
                        "formalize".into(),
                        response.text().to_string(),
                        0,
                    )
                } else {
                    (
                        AttemptOutcome::Failed,
                        "formalize".into(),
                        format!("formalize {claim_id} failed: {}", response.text()),
                        0,
                    )
                }
            }
            CampaignOp::Hypothesize => {
                let response = self.lab.exec(Command::Hypothesize {
                    theory: Some(sel.theory.clone()),
                });
                if response.exit_code() == 0 {
                    let text = response.text().to_string();
                    let explored = parse_explored(&text);
                    (
                        AttemptOutcome::Completed,
                        "hypothesize".into(),
                        text,
                        explored,
                    )
                } else {
                    (
                        AttemptOutcome::Failed,
                        "hypothesize".into(),
                        format!("hypothesize {} failed: {}", sel.theory, response.text()),
                        0,
                    )
                }
            }
        }
    }

    /// Render the campaign status: state, manifest id, budget usage,
    /// attempts, blocked set, and stop record.
    pub fn status(&self) -> String {
        render_status(
            &self.bundle,
            &self.manifest,
            &self.manifest_id.to_hex(),
            CAMPAIGN_STATUS_VERSION,
        )
    }

    /// Pinned manifest content id.
    pub fn manifest_id(&self) -> ArtifactId {
        self.manifest_id
    }

    /// Campaign operations from the manifest.
    pub fn operations(&self) -> &[CampaignOp] {
        &self.manifest.candidate_space.operations
    }
}

/// Render status from the bundle directory alone: read-only open, no
/// lock, no lab. Refuses corrupt bundles exactly like [`Bundle::open`].
/// The manifest comes from the pinned bundle object, never a path the
/// caller could swap between calls.
pub fn status_text(dir: &Path) -> Result<String, String> {
    let bundle = Bundle::open_read_only(dir)?;
    let pinned = bundle
        .manifest_id()
        .ok_or_else(|| "bundle has no pinned manifest".to_string())?;
    let id = ArtifactId::from_hex(pinned)
        .ok_or_else(|| "bundle manifest id is not a hash".to_string())?;
    let bytes = bundle.get_object(id)?;
    let text = String::from_utf8(bytes).map_err(|_| "manifest object is not utf-8".to_string())?;
    let (manifest, _) = parse_manifest(&text)?;
    Ok(render_status(
        &bundle,
        &manifest,
        pinned,
        CAMPAIGN_STATUS_VERSION,
    ))
}

/// Shared renderer for live and read-only status.
fn render_status(bundle: &Bundle, manifest: &Manifest, manifest_hex: &str, version: u32) -> String {
    let checkpoint = bundle.latest_checkpoint().cloned().unwrap_or_default();
    let usage = Usage {
        actions: bundle.total_charged(),
        attempts: bundle.attempts_in_order().len() as u64,
        wall_ms: checkpoint.wall_ms_used,
        candidates: checkpoint.candidates_used,
    };
    let state = match &checkpoint.stopped {
        Some(record) => format!("stopped ({}: {})", record.reason.as_str(), record.detail),
        None if usage.attempts == 0 && checkpoint.no_progress_streak == 0 => "planned".to_string(),
        None => "running".to_string(),
    };
    let mut out = format!("research status  v{version}\n");
    out.push_str(&format!("  manifest {manifest_hex}\n"));
    out.push_str(&format!("  state {state}\n"));
    out.push_str(&format!(
        "  budget  actions {}/{}  attempts {}/{}  wall_ms {}/{}  candidates {}/{}\n",
        usage.actions,
        manifest.budget.actions,
        usage.attempts,
        manifest.budget.attempts,
        usage.wall_ms,
        manifest.budget.wall_time_secs.saturating_mul(1000),
        usage.candidates,
        manifest.budget.candidates,
    ));
    out.push_str(&format!(
        "  no-progress {}/{}\n",
        checkpoint.no_progress_streak, manifest.stop.no_progress_limit
    ));
    out.push_str("  attempts:\n");
    for id in bundle.attempts_in_order() {
        let op = bundle
            .intent(id)
            .map(|i| op_name(i.op))
            .unwrap_or("unknown");
        let state = match bundle.attempt_state(id) {
            crate::bundle::AttemptState::Completed => "completed",
            crate::bundle::AttemptState::Failed => "failed",
            crate::bundle::AttemptState::Interrupted => "interrupted",
            crate::bundle::AttemptState::NeverStarted => "never-started",
        };
        let note = bundle
            .result(id)
            .map(|r| r.note.clone())
            .unwrap_or_default();
        out.push_str(&format!("    {id}  {op}  {state}  {note}\n"));
    }
    if bundle.attempts_in_order().is_empty() {
        out.push_str("    none\n");
    }
    out.push_str("  blocked:\n");
    if checkpoint.blocked.is_empty() {
        out.push_str("    none\n");
    }
    for (key, inputs) in &checkpoint.blocked {
        out.push_str(&format!("    {key}  inputs {inputs}\n"));
    }
    out
}

/// First line of a result, for attempt notes.
fn first_line(text: &str) -> String {
    text.lines()
        .next()
        .unwrap_or_default()
        .chars()
        .take(160)
        .collect()
}

/// Minimal valid manifest scoping `theories`/`statements`/`ops`.
/// Mirrors EXAMPLE_MANIFEST's live-validated blocks. Shared test fixture.
#[cfg(test)]
pub(crate) fn test_manifest(theories: &[&str], statements: &[&str], ops: &[&str]) -> String {
    let authorities: Vec<String> = ops
        .iter()
        .map(|op| {
            let role = match *op {
                "prove" => "proof-searcher",
                "cite" => "provenance-auditor",
                "enclose" => "numerical-verifier",
                "formalize" => "formalizer",
                "hypothesize" => "explorer",
                _ => panic!("unknown op {op}"),
            };
            format!("\"{op}\": \"{role}\"")
        })
        .collect();
    format!(
        r#"{{
  "schema": "physis-campaign-manifest",
  "version": 1,
  "objective": {{
"question": "r4 test campaign",
"theories": [{}],
"statements": [{}],
"observable": "test verdict",
"units": "verdict-kind",
"domain": "test scope",
"success": "scoped gap closes",
"failure": "scoped gap persists",
"inconclusive": "step is blocked"
  }},
  "baseline": {{
"source_revision": "working-tree",
"theory_package": "type-iib",
"knobs": [{{"theory": "type-iib", "knob": "total_dim", "value": "10"}}],
"assumptions": ["live lab definitions"],
"constants": ["c"],
"datasets": [],
"evidence": []
  }},
  "candidate_space": {{
"operations": [{}],
"parameter_ranges": [{{"theory": "type-iib", "knob": "total_dim", "min": 2.0, "max": 32.0}}],
"structural_mutations": [],
"max_candidates": 8,
"seed": 7
  }},
  "evaluation": {{
"obligations": ["consistency.critical-dimension"],
"numeric_tolerance": 0.000001,
"tolerance_justification": "verdict flips are exact; tolerance is a formality",
"comparison": "diff verdicts against the retained baseline",
"controls": ["unmutated baseline"],
"evidence_required": ["verdict diffs"]
  }},
  "data": {{
"sources": [{{"id": "none", "locator": "synthetic test", "license": "n/a"}}],
"calibration": [],
"evaluation_inputs": [],
"uncertainty": "no empirical inputs",
"access_approval": "none-required: synthetic test",
"adaptive_use": []
  }},
  "authority": {{{}}},
  "budget": {{
"actions": 4,
"candidates": 8,
"attempts": 8,
"wall_time_secs": 60,
"memory_mb": 512,
"output_bytes": 65536,
"external_cost": 0
  }},
  "stop": {{"no_progress_limit": 3, "human_pause": false}}
}}"#,
        theories
            .iter()
            .map(|t| format!("\"{t}\""))
            .collect::<Vec<_>>()
            .join(", "),
        statements
            .iter()
            .map(|s| format!("\"{s}\""))
            .collect::<Vec<_>>()
            .join(", "),
        ops.iter()
            .map(|o| format!("\"{o}\""))
            .collect::<Vec<_>>()
            .join(", "),
        authorities.join(", "),
    )
}

#[cfg(test)]
mod tests {
    use super::test_manifest as manifest_for;
    use super::*;

    fn campaign_dir(name: &str) -> std::path::PathBuf {
        let dir =
            std::env::temp_dir().join(format!("physis-campaign-{name}-{}", std::process::id()));
        std::fs::remove_dir_all(&dir).ok();
        dir
    }

    #[test]
    fn step_proves_supported_theorem_and_reports_usage() {
        let mut lab = Lab::standard();
        let dir = campaign_dir("prove");
        let text = manifest_for(&["de-rham"], &["dec.d-squared-zero"], &["prove"]);
        let mut campaign = Campaign::open(&mut lab, &text, &dir).unwrap();
        let out = campaign.step_once();
        assert_eq!(out.exit_code(), 0, "{}", out.text());
        assert!(
            out.text().contains("prove dec.d-squared-zero minted"),
            "{}",
            out.text()
        );
        let status = campaign.status();
        assert!(status.contains("actions 1/4"), "{status}");
        assert!(status.contains("attempts 1/8"), "{status}");
        assert!(status.contains("state running"), "{status}");
    }

    #[test]
    fn manifest_revision_change_is_refused() {
        let mut lab = Lab::standard();
        let dir = campaign_dir("revision");
        let text = manifest_for(&["de-rham"], &["dec.d-squared-zero"], &["prove"]);
        {
            let mut campaign = Campaign::open(&mut lab, &text, &dir).unwrap();
            assert_eq!(campaign.step_once().exit_code(), 0);
        }
        let changed = text.replace("r4 test campaign", "r4 test campaign revised");
        let err = Campaign::open(&mut lab, &changed, &dir)
            .err()
            .expect("revision change is refused");
        assert!(err.contains("revision"), "{err}");
    }

    #[test]
    fn human_pause_stops_before_dispatch() {
        let mut lab = Lab::standard();
        let dir = campaign_dir("pause");
        let text = manifest_for(&["de-rham"], &["dec.d-squared-zero"], &["prove"]);
        let text = text.replace("\"human_pause\": false", "\"human_pause\": true");
        let mut campaign = Campaign::open(&mut lab, &text, &dir).unwrap();
        let out = campaign.step_once();
        assert_eq!(out.exit_code(), 0, "{}", out.text());
        assert!(out.text().contains("human-pause"), "{}", out.text());
        let status = campaign.status();
        assert!(status.contains("state stopped"), "{status}");
        assert!(status.contains("attempts 0/8"), "{status}");
    }

    #[test]
    fn selection_is_deterministic_across_labs() {
        let text = manifest_for(&["de-rham"], &["dec.d-squared-zero"], &["prove"]);
        let mut lab_a = Lab::standard();
        let dir_a = campaign_dir("det-a");
        let mut a = Campaign::open(&mut lab_a, &text, &dir_a).unwrap();
        let first_a = a.step_once().text().to_string();
        let mut lab_b = Lab::standard();
        let dir_b = campaign_dir("det-b");
        let mut b = Campaign::open(&mut lab_b, &text, &dir_b).unwrap();
        let first_b = b.step_once().text().to_string();
        let selected = |t: &str| {
            t.lines()
                .find(|l| l.starts_with("selected "))
                .unwrap_or_default()
                .to_string()
        };
        assert_eq!(selected(&first_a), selected(&first_b));
        assert!(selected(&first_a).contains("dec.d-squared-zero"));
    }

    #[test]
    fn exhausted_budgets_stop_without_dispatch() {
        let mut lab = Lab::standard();
        let text = manifest_for(&["de-rham"], &["dec.d-squared-zero"], &["prove"]);
        // One action of budget: the prove consumes it, the next step stops.
        let text = text.replace("\"actions\": 4", "\"actions\": 1");
        let dir = campaign_dir("budget");
        let mut campaign = Campaign::open(&mut lab, &text, &dir).unwrap();
        assert_eq!(campaign.step_once().exit_code(), 0);
        let out = campaign.step_once();
        assert_eq!(out.exit_code(), 0, "{}", out.text());
        assert!(out.text().contains("budget-exhausted"), "{}", out.text());
        let status = campaign.status();
        assert!(status.contains("state stopped"), "{status}");
    }

    #[test]
    fn status_text_needs_no_lab() {
        let mut lab = Lab::standard();
        let dir = campaign_dir("status");
        let text = manifest_for(&["de-rham"], &["dec.d-squared-zero"], &["prove"]);
        {
            let mut campaign = Campaign::open(&mut lab, &text, &dir).unwrap();
            assert_eq!(campaign.step_once().exit_code(), 0);
        }
        let status = status_text(&dir).unwrap();
        assert!(status.contains("attempts 1/8"), "{status}");
        assert!(status.contains("dec.d-squared-zero"), "{status}");
    }

    #[test]
    fn proved_scope_stops_with_no_eligible_action() {
        let mut lab = Lab::standard();
        let dir = campaign_dir("done");
        let text = manifest_for(&["de-rham"], &["dec.d-squared-zero"], &["prove"]);
        let mut campaign = Campaign::open(&mut lab, &text, &dir).unwrap();
        assert_eq!(campaign.step_once().exit_code(), 0);
        let out = campaign.step_once();
        assert_eq!(out.exit_code(), 0, "{}", out.text());
        assert!(out.text().contains("no-eligible-action"), "{}", out.text());
        assert!(
            campaign.status().contains("state stopped"),
            "{}",
            campaign.status()
        );
    }

    #[test]
    fn resume_reconciles_interrupted_attempt_then_dispatches() {
        use crate::bundle::Bundle;
        use crate::campaign::CampaignOp;
        let dir = campaign_dir("resume");
        let text = manifest_for(&["de-rham"], &["dec.d-squared-zero"], &["prove"]);
        {
            let mut lab = Lab::standard();
            let campaign = Campaign::open(&mut lab, &text, &dir).unwrap();
            drop(campaign);
        }
        // Crash between intent and result: the lock stays held, like a
        // killed process that never ran Drop.
        {
            let mut bundle = Bundle::open(&dir).unwrap();
            let input = bundle.put_object(b"selection").unwrap();
            bundle
                .begin_attempt(CampaignOp::Prove, "parent-tip", &[input], 1)
                .unwrap();
            std::mem::forget(bundle);
        }
        Bundle::break_lock(&dir).unwrap();
        let mut lab = Lab::standard();
        let mut campaign = Campaign::open(&mut lab, &text, &dir).unwrap();
        let out = campaign.step_once();
        assert_eq!(out.exit_code(), 0, "{}", out.text());
        assert!(
            out.text().contains("reconciled  a1  interrupted"),
            "{}",
            out.text()
        );
        assert!(out.text().contains("attempt  a2"), "{}", out.text());
        assert!(
            out.text().contains("prove dec.d-squared-zero minted"),
            "{}",
            out.text()
        );
        let status = campaign.status();
        assert!(status.contains("attempts 2/8"), "{status}");
        assert!(status.contains("a1  prove  failed"), "{status}");
        assert!(status.contains("a2  prove  completed"), "{status}");
    }

    #[test]
    fn blocked_work_is_not_retried_until_inputs_change() {
        let mut lab = Lab::standard();
        let dir = campaign_dir("blocked");
        // Unsupported theorem with no formalize op: a work item every
        // time, never a dispatch.
        let text = manifest_for(
            &["general-relativity"],
            &["gr.eddington-deflection"],
            &["prove"],
        );
        let mut campaign = Campaign::open(&mut lab, &text, &dir).unwrap();
        let first = campaign.step_once();
        assert_eq!(first.exit_code(), 0, "{}", first.text());
        assert!(
            first.text().contains("formalization work item"),
            "{}",
            first.text()
        );
        assert!(first.text().contains("attempts 0/8"), "{}", first.text());
        let second = campaign.step_once();
        assert!(
            second.text().contains("selected-repeat"),
            "{}",
            second.text()
        );
        assert!(
            campaign.status().contains("no-progress 1/3"),
            "{}",
            campaign.status()
        );
        assert_eq!(campaign.step_once().exit_code(), 0);
        let fourth = campaign.step_once();
        assert!(fourth.text().contains("no-progress"), "{}", fourth.text());
        let status = campaign.status();
        assert!(status.contains("state stopped"), "{status}");
        assert!(status.contains("attempts 0/8"), "{status}");
        assert!(status.contains("blocked:"), "{status}");
    }

    #[test]
    fn authority_refusal_is_a_blocked_item_until_role_changes() {
        use crate::role::Role;
        let mut lab = Lab::standard();
        lab.set_role(Role::Explorer);
        let dir = campaign_dir("authority");
        let text = manifest_for(&["de-rham"], &["dec.d-squared-zero"], &["prove"]);
        let mut campaign = Campaign::open(&mut lab, &text, &dir).unwrap();
        let blocked = campaign.step_once();
        assert_eq!(blocked.exit_code(), 0, "{}", blocked.text());
        assert!(
            blocked.text().contains("authority work item"),
            "{}",
            blocked.text()
        );
        assert!(
            blocked.text().contains("attempts 0/8"),
            "{}",
            blocked.text()
        );
        let repeat = campaign.step_once();
        assert!(
            repeat.text().contains("selected-repeat"),
            "{}",
            repeat.text()
        );
        // The role is a relevant input: changing it re-enables dispatch.
        campaign.lab.set_role(Role::Lab);
        let out = campaign.step_once();
        assert!(
            out.text().contains("prove dec.d-squared-zero minted"),
            "{}",
            out.text()
        );
    }

    #[test]
    fn hypothesize_charges_candidates() {
        let mut lab = Lab::standard();
        let dir = campaign_dir("hypothesize");
        let text = manifest_for(
            &["observer-geometry"],
            &["predictivity.unique-vacuum"],
            &["hypothesize"],
        );
        let mut campaign = Campaign::open(&mut lab, &text, &dir).unwrap();
        let out = campaign.step_once();
        assert_eq!(out.exit_code(), 0, "{}", out.text());
        assert!(out.text().contains("action  hypothesize"), "{}", out.text());
        assert!(out.text().contains("attempt  a1"), "{}", out.text());
        let status = campaign.status();
        assert!(status.contains("attempts 1/8"), "{status}");
        assert!(status.contains("candidates "), "{status}");
    }

    #[test]
    fn stop_reason_branches() {
        let text = manifest_for(&["de-rham"], &["dec.d-squared-zero"], &["prove"]);
        let (manifest, _) = crate::campaign::parse_manifest(&text).unwrap();
        let fresh = Usage {
            actions: 0,
            attempts: 0,
            wall_ms: 0,
            candidates: 0,
        };
        assert!(stop_reason(&manifest, &fresh, 0, None).is_none());
        let sticky = StopRecord {
            reason: StopReason::HumanPause,
            detail: "paused".into(),
        };
        assert_eq!(
            stop_reason(&manifest, &fresh, 0, Some(&sticky))
                .unwrap()
                .reason,
            StopReason::HumanPause
        );
        for (usage, streak, reason) in [
            (
                Usage {
                    actions: 4,
                    ..fresh
                },
                0,
                StopReason::BudgetExhausted,
            ),
            (
                Usage {
                    attempts: 8,
                    ..fresh
                },
                0,
                StopReason::BudgetExhausted,
            ),
            (
                Usage {
                    wall_ms: 60_000,
                    ..fresh
                },
                0,
                StopReason::BudgetExhausted,
            ),
            (
                Usage {
                    candidates: 8,
                    ..fresh
                },
                0,
                StopReason::BudgetExhausted,
            ),
            (fresh, 3, StopReason::NoProgress),
        ] {
            let record = stop_reason(&manifest, &usage, streak, None)
                .unwrap_or_else(|| panic!("{usage:?}/{streak} must stop"));
            assert_eq!(record.reason, reason);
        }
        assert!(stop_reason(&manifest, &fresh, 2, None).is_none());
    }

    #[test]
    fn parse_explored_counts_footer() {
        assert_eq!(parse_explored("explored=10 rejected=2"), 10);
        assert_eq!(parse_explored("nothing here"), 0);
        assert_eq!(parse_explored("explored=x"), 0);
    }

    #[test]
    fn r7_explored_sign_and_overflow_count_zero() {
        // A lying footer cannot reduce candidate usage or wrap it: a
        // negative or overflowing count charges zero, never a refund.
        assert_eq!(parse_explored("explored=-3"), 0);
        assert_eq!(parse_explored("explored=99999999999999999999999999"), 0);
        assert_eq!(parse_explored("explored="), 0);
        assert_eq!(parse_explored("explored=7 explored=-7"), 7);
    }

    #[test]
    fn r7_reopen_after_exhaustion_stays_stopped() {
        // Restart bypass: reopening an exhausted campaign restores usage
        // from the journal and refuses to dispatch; it never resets.
        let mut lab = Lab::standard();
        let text = manifest_for(&["de-rham"], &["dec.d-squared-zero"], &["prove"]);
        let text = text.replace("\"actions\": 4", "\"actions\": 1");
        let dir = campaign_dir("r7-restart");
        {
            let mut campaign = Campaign::open(&mut lab, &text, &dir).unwrap();
            assert_eq!(campaign.step_once().exit_code(), 0);
            let out = campaign.step_once();
            assert!(out.text().contains("budget-exhausted"), "{}", out.text());
        }
        {
            let mut campaign = Campaign::open(&mut lab, &text, &dir).unwrap();
            let out = campaign.step_once();
            assert_eq!(out.exit_code(), 0, "{}", out.text());
            assert!(
                out.text().contains("budget-exhausted"),
                "reopen must not reset usage: {}",
                out.text()
            );
            let status = campaign.status();
            assert!(status.contains("state stopped"), "{status}");
        }
    }
}
