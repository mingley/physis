//! Inspectable research report (R6, spec 022).
//!
//! Versioned projection over a campaign bundle: objective, assumptions,
//! candidate lineage, all attempts (negative results included), bounds,
//! dataset ids, receipt-bearing notes, and unresolved gaps. Read-only:
//! the bundle journal is the store of truth, and the Markdown rendering
//! says so on its face. A clean checkout replays the embedded manifest
//! to reproduce the scoped result or hits a precise, content-addressed
//! blocker; prose is not evidence.

use std::path::Path;

use physis_core::artifact::ArtifactId;
use serde::Serialize;

use crate::bundle::{AttemptState, Bundle};
use crate::campaign::{op_name, parse_manifest};
use crate::coordinator::StopRecord;

/// Report schema tag.
pub const REPORT_SCHEMA: &str = "physis-campaign-report";
/// Report schema version.
pub const REPORT_VERSION: u32 = 1;

/// Export format for [`export_text`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportFormat {
    /// Versioned JSON: the machine-checkable report.
    Json,
    /// Rendered Markdown: a projection, not the store of truth.
    Markdown,
}

impl ExportFormat {
    /// Parse the optional CLI format word. Anything else is refused.
    pub fn parse(word: Option<&str>) -> Result<Self, String> {
        match word {
            None | Some("json") => Ok(ExportFormat::Json),
            Some("markdown") => Ok(ExportFormat::Markdown),
            Some(other) => Err(format!("unknown export format '{other}': json or markdown")),
        }
    }
}

/// Render the campaign report from the bundle directory alone:
/// read-only open, no lock, no lab.
pub fn export_text(dir: &Path, format: ExportFormat) -> Result<String, String> {
    let bundle = Bundle::open_read_only(dir)?;
    let pinned = bundle
        .manifest_id()
        .ok_or_else(|| "bundle has no pinned manifest".to_string())?;
    let id = ArtifactId::from_hex(pinned)
        .ok_or_else(|| "bundle manifest id is not a hash".to_string())?;
    let bytes = bundle
        .get_object(id)
        .map_err(|e| format!("manifest object missing from bundle: {e}"))?;
    let manifest_text =
        String::from_utf8(bytes).map_err(|_| "manifest object is not utf-8".to_string())?;
    let (manifest, _) = parse_manifest(&manifest_text)?;
    let manifest_json: serde_json::Value = serde_json::from_str(&manifest_text)
        .map_err(|e| format!("pinned manifest is not json: {e}"))?;
    let report = build_report(
        &bundle,
        &manifest,
        pinned,
        &manifest_text,
        manifest_json,
        dir,
    );
    match format {
        ExportFormat::Json => serde_json::to_string_pretty(&report)
            .map_err(|e| format!("report encoding failed: {e}")),
        ExportFormat::Markdown => Ok(render_markdown(&report)),
    }
}

/// One used/max budget pair.
#[derive(Serialize)]
struct BudgetEntry {
    used: u64,
    max: u64,
}

/// Budget usage against the manifest.
#[derive(Serialize)]
struct BudgetView {
    actions: BudgetEntry,
    attempts: BudgetEntry,
    wall_ms: BudgetEntry,
    candidates: BudgetEntry,
}

/// One attempt with its lineage and disposition.
#[derive(Serialize)]
struct AttemptView {
    id: String,
    op: String,
    parent: String,
    inputs: Vec<String>,
    cost_actions: u64,
    state: String,
    outputs: Vec<String>,
    note: String,
}

/// One blocked action with its frozen input identity.
#[derive(Serialize)]
struct BlockedView {
    key: String,
    inputs: String,
}

/// A declared dataset source from the manifest.
#[derive(Serialize)]
struct SourceView {
    id: String,
    locator: String,
    license: String,
}

/// The versioned report: machine-checkable projection over the bundle.
#[derive(Serialize)]
struct Report {
    schema: String,
    version: u32,
    campaign: String,
    manifest_id: String,
    manifest_text: String,
    manifest: serde_json::Value,
    state: String,
    stop: Option<StopRecord>,
    budget: BudgetView,
    no_progress_streak: u64,
    no_progress_limit: u64,
    attempts: Vec<AttemptView>,
    blocked: Vec<BlockedView>,
    declared_datasets: Vec<String>,
    data_sources: Vec<SourceView>,
    journal_tip: String,
}

/// Assemble the report from verified bundle state. Every attempt is a
/// row — failures and reconciled interruptions included — and every id
/// is content-addressed, so a clean checkout replays the manifest or
/// hits the pinned blocker.
fn build_report(
    bundle: &Bundle,
    manifest: &crate::campaign::Manifest,
    pinned: &str,
    manifest_text: &str,
    manifest_json: serde_json::Value,
    dir: &Path,
) -> Report {
    let checkpoint = bundle.latest_checkpoint().cloned().unwrap_or_default();
    let attempts_total = bundle.attempts_in_order().len() as u64;
    let state = match &checkpoint.stopped {
        Some(record) => format!("stopped ({}: {})", record.reason.as_str(), record.detail),
        None if attempts_total == 0 && checkpoint.no_progress_streak == 0 => "planned".into(),
        None => "running".into(),
    };
    let mut attempts = Vec::new();
    for id in bundle.attempts_in_order() {
        let (op, parent, inputs, cost) = match bundle.intent(id) {
            Some(i) => (
                op_name(i.op).to_string(),
                i.parent.clone(),
                i.inputs.iter().map(|h| h.to_hex()).collect(),
                i.cost_actions,
            ),
            None => ("unknown".into(), String::new(), Vec::new(), 0),
        };
        let state = match bundle.attempt_state(id) {
            AttemptState::Completed => "completed",
            AttemptState::Failed => "failed",
            AttemptState::Interrupted => "interrupted",
            AttemptState::NeverStarted => "never-started",
        };
        let (outputs, note) = match bundle.result(id) {
            Some(r) => (
                r.outputs.iter().map(|h| h.to_hex()).collect(),
                r.note.clone(),
            ),
            None => (Vec::new(), String::new()),
        };
        attempts.push(AttemptView {
            id: id.clone(),
            op,
            parent,
            inputs,
            cost_actions: cost,
            state: state.into(),
            outputs,
            note,
        });
    }
    Report {
        schema: REPORT_SCHEMA.into(),
        version: REPORT_VERSION,
        campaign: dir
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default(),
        manifest_id: pinned.into(),
        manifest_text: manifest_text.into(),
        manifest: manifest_json,
        state,
        stop: checkpoint.stopped.clone(),
        budget: BudgetView {
            actions: BudgetEntry {
                used: bundle.total_charged(),
                max: manifest.budget.actions,
            },
            attempts: BudgetEntry {
                used: attempts_total,
                max: manifest.budget.attempts,
            },
            wall_ms: BudgetEntry {
                used: checkpoint.wall_ms_used,
                max: manifest.budget.wall_time_secs.saturating_mul(1000),
            },
            candidates: BudgetEntry {
                used: checkpoint.candidates_used,
                max: manifest.budget.candidates,
            },
        },
        no_progress_streak: checkpoint.no_progress_streak,
        no_progress_limit: manifest.stop.no_progress_limit,
        attempts,
        blocked: checkpoint
            .blocked
            .iter()
            .map(|(key, inputs)| BlockedView {
                key: key.clone(),
                inputs: inputs.clone(),
            })
            .collect(),
        declared_datasets: manifest.baseline.datasets.clone(),
        data_sources: manifest
            .data
            .sources
            .iter()
            .map(|s| SourceView {
                id: s.id.clone(),
                locator: s.locator.clone(),
                license: s.license.clone(),
            })
            .collect(),
        journal_tip: bundle.tip().to_hex(),
    }
}

/// Render the Markdown projection. Same data, human order, labeled as
/// a projection on its face.
fn render_markdown(report: &Report) -> String {
    let mut out = String::from("# Campaign report\n\n");
    out.push_str("> projection, not the store of truth: verify against the bundle journal.\n\n");
    out.push_str(&format!("- campaign: {}\n", report.campaign));
    out.push_str(&format!("- manifest: {}\n", report.manifest_id));
    out.push_str(&format!("- state: {}\n", report.state));
    if let Some(stop) = &report.stop {
        out.push_str(&format!(
            "- stop: {}: {}\n",
            stop.reason.as_str(),
            stop.detail
        ));
    }
    out.push_str(&format!(
        "- budget: actions {}/{}; attempts {}/{}; wall_ms {}/{}; candidates {}/{}\n",
        report.budget.actions.used,
        report.budget.actions.max,
        report.budget.attempts.used,
        report.budget.attempts.max,
        report.budget.wall_ms.used,
        report.budget.wall_ms.max,
        report.budget.candidates.used,
        report.budget.candidates.max,
    ));
    out.push_str(&format!(
        "- no-progress: {}/{}\n",
        report.no_progress_streak, report.no_progress_limit
    ));
    out.push_str(&format!("- journal tip: {}\n", report.journal_tip));
    out.push_str("\n## Attempts (all outcomes, failures included)\n\n");
    if report.attempts.is_empty() {
        out.push_str("none\n");
    }
    for a in &report.attempts {
        out.push_str(&format!(
            "- {} {} {} (cost {}): {}\n",
            a.id, a.op, a.state, a.cost_actions, a.note
        ));
    }
    out.push_str("\n## Blocked (unresolved gaps)\n\n");
    if report.blocked.is_empty() {
        out.push_str("none\n");
    }
    for b in &report.blocked {
        out.push_str(&format!("- {} inputs {}\n", b.key, b.inputs));
    }
    out.push_str("\n## Datasets (declared)\n\n");
    if report.declared_datasets.is_empty() && report.data_sources.is_empty() {
        out.push_str("none\n");
    }
    for d in &report.declared_datasets {
        out.push_str(&format!("- {d}\n"));
    }
    for s in &report.data_sources {
        out.push_str(&format!("- {} ({}, {})\n", s.id, s.locator, s.license));
    }
    out.push_str("\n## Manifest\n\n");
    out.push_str("Pinned bytes verified; replay with the exact text to reproduce or block.\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bundle::{AttemptOutcome, Bundle};
    use crate::campaign::CampaignOp;
    use crate::coordinator::{test_manifest, StopReason, StopRecord};

    fn report_dir(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("physis-report-{name}-{}", std::process::id()));
        std::fs::remove_dir_all(&dir).ok();
        dir
    }

    /// Bundle with one completed attempt, one failed attempt, and a
    /// checkpoint carrying blocked inputs plus a stop record.
    fn history_bundle(dir: &Path, manifest: &str) -> Bundle {
        use crate::coordinator::CoordinatorCheckpoint;
        use physis_core::ArtifactId;
        use std::collections::BTreeMap;
        let manifest_id = ArtifactId::of(manifest.as_bytes());
        let mut bundle = Bundle::create(dir, Some(manifest_id)).unwrap();
        bundle.put_object(manifest.as_bytes()).unwrap();
        let input = bundle.put_object(b"selection").unwrap();
        let a1 = bundle
            .begin_attempt(CampaignOp::Prove, "tip-1", &[input], 1)
            .unwrap();
        let out1 = bundle.put_object(b"result-1").unwrap();
        bundle
            .commit_result(&a1, AttemptOutcome::Completed, &[out1], "prove x minted r1")
            .unwrap();
        let a2 = bundle
            .begin_attempt(CampaignOp::Cite, "tip-2", &[input], 1)
            .unwrap();
        bundle
            .commit_result(&a2, AttemptOutcome::Failed, &[], "cite y failed: no source")
            .unwrap();
        bundle
            .checkpoint(&CoordinatorCheckpoint {
                wall_ms_used: 120,
                candidates_used: 0,
                no_progress_streak: 1,
                blocked: BTreeMap::from([("cite:gr:y:missing-dataset".into(), "inputs-1".into())]),
                stopped: Some(StopRecord {
                    reason: StopReason::NoEligibleAction,
                    detail: "no gap in scope".into(),
                }),
            })
            .unwrap();
        bundle
    }

    #[test]
    fn export_json_carries_versioned_schema() {
        let dir = report_dir("json");
        let manifest = test_manifest(&["de-rham"], &["dec.d-squared-zero"], &["prove"]);
        drop(history_bundle(&dir, &manifest));
        let text = export_text(&dir, ExportFormat::Json).unwrap();
        let v: serde_json::Value = serde_json::from_str(&text).unwrap();
        assert_eq!(v["schema"], REPORT_SCHEMA);
        assert_eq!(v["version"], REPORT_VERSION);
        assert!(v["campaign"]
            .as_str()
            .unwrap()
            .contains("physis-report-json"));
        assert_eq!(v["attempts"].as_array().unwrap().len(), 2);
        // Negative results are first-class rows, not filtered.
        let failed = v["attempts"]
            .as_array()
            .unwrap()
            .iter()
            .find(|a| a["state"] == "failed")
            .expect("failed attempt row");
        assert!(failed["note"].as_str().unwrap().contains("no source"));
        assert_eq!(v["blocked"].as_array().unwrap().len(), 1);
        assert_eq!(v["stop"]["reason"], "no-eligible-action");
        assert_eq!(v["budget"]["actions"]["used"], 2);
        assert_eq!(v["budget"]["actions"]["max"], 4);
        assert!(v["manifest"]["objective"]["question"].is_string());
        assert!(v["manifest_id"].as_str().unwrap().len() == 64);
        assert!(v["journal_tip"].as_str().unwrap().len() == 64);
        assert!(v["declared_datasets"].is_array());
        assert!(v["data_sources"].is_array());
    }

    #[test]
    fn export_refuses_missing_bundle() {
        let dir = report_dir("missing");
        assert!(export_text(&dir, ExportFormat::Json).is_err());
    }

    #[test]
    fn export_refuses_bundle_without_manifest_object() {
        let dir = report_dir("nomanifest");
        // Pinned id present in meta, but the bytes were never stored.
        let manifest = test_manifest(&["de-rham"], &["dec.d-squared-zero"], &["prove"]);
        {
            let bundle =
                Bundle::create(&dir, Some(physis_core::ArtifactId::of(manifest.as_bytes())))
                    .unwrap();
            drop(bundle);
        }
        let err = export_text(&dir, ExportFormat::Json).unwrap_err();
        assert!(err.contains("manifest"), "{err}");
    }

    #[test]
    fn export_markdown_is_a_labeled_projection() {
        let dir = report_dir("markdown");
        let manifest = test_manifest(&["de-rham"], &["dec.d-squared-zero"], &["prove"]);
        drop(history_bundle(&dir, &manifest));
        let text = export_text(&dir, ExportFormat::Markdown).unwrap();
        assert!(text.contains("# Campaign report"), "{text}");
        assert!(
            text.contains("projection, not the store of truth"),
            "{text}"
        );
        assert!(text.contains("a1"), "{text}");
        assert!(text.contains("failed"), "{text}");
        assert!(text.contains("no-eligible-action"), "{text}");
    }

    #[test]
    fn exported_manifest_replays_or_blocks_precisely() {
        use crate::coordinator::Campaign;
        use crate::lab::Lab;
        let dir = report_dir("replay");
        let manifest = test_manifest(&["de-rham"], &["dec.d-squared-zero"], &["prove"]);
        {
            let mut lab = Lab::standard();
            let mut campaign = Campaign::open(&mut lab, &manifest, &dir).unwrap();
            assert_eq!(campaign.step_once().exit_code(), 0);
        }
        let text = export_text(&dir, ExportFormat::Json).unwrap();
        let v: serde_json::Value = serde_json::from_str(&text).unwrap();
        // The embedded manifest text round-trips byte-identically: it
        // replays the same selection in a fresh directory.
        let embedded = v["manifest_text"].as_str().expect("manifest text");
        assert_eq!(embedded, manifest);
        let dir2 = report_dir("replay-2");
        {
            let mut lab = Lab::standard();
            let mut campaign = Campaign::open(&mut lab, embedded, &dir2).unwrap();
            let out = campaign.step_once();
            assert!(
                out.text().contains("selected  de-rham  dec.d-squared-zero"),
                "{}",
                out.text()
            );
        }
        // And the pinned id refuses a tampered revision with a precise blocker.
        let tampered = manifest.replace("r4 test campaign", "r4 test campaign revised");
        {
            let mut lab = Lab::standard();
            let err = Campaign::open(&mut lab, &tampered, &dir)
                .err()
                .expect("revision change is refused");
            assert!(err.contains("revision"), "{err}");
        }
        assert_eq!(
            v["manifest_id"],
            physis_core::ArtifactId::of(manifest.as_bytes()).to_hex()
        );
    }
}
