//! Facade CLI smoke tests: help, unknown-command, and role-refusal paths.
//!
//! These assert exit codes plus stable single-phrase markers only — never
//! prose-golden snapshots of help or response text.

use std::process::{Command, Output};

fn physis() -> Command {
    Command::new(env!("CARGO_BIN_EXE_physis"))
}

fn run(args: &[&str]) -> Output {
    physis()
        .args(args)
        .output()
        .expect("failed to spawn physis binary")
}

fn stderr_of(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

fn stdout_of(out: &Output) -> String {
    String::from_utf8_lossy(&out.stdout).into_owned()
}

#[test]
fn bare_invocation_prints_usage_and_exits_zero() {
    let out = run(&[]);
    assert_eq!(out.status.code(), Some(0), "stderr: {}", stderr_of(&out));
    assert!(
        stderr_of(&out).contains("USAGE:"),
        "stderr: {}",
        stderr_of(&out)
    );
}

#[test]
fn help_flag_prints_usage_and_exits_two() {
    for args in [&["--help"][..], &["help"][..]] {
        let out = run(args);
        assert_eq!(
            out.status.code(),
            Some(2),
            "args {args:?} stderr: {}",
            stderr_of(&out)
        );
        assert!(
            stderr_of(&out).contains("USAGE:"),
            "args {args:?} stderr: {}",
            stderr_of(&out)
        );
    }
}

#[test]
fn unknown_command_exits_two_with_marker() {
    let out = run(&["frobnicate"]);
    assert_eq!(out.status.code(), Some(2), "stderr: {}", stderr_of(&out));
    let stderr = stderr_of(&out);
    assert!(
        stderr.contains("unknown command 'frobnicate'"),
        "stderr: {stderr}"
    );
    assert!(stderr.contains("USAGE:"), "stderr: {stderr}");
}

#[test]
fn unknown_role_exits_two_with_marker() {
    let out = run(&["--role", "no-such-role", "layers"]);
    assert_eq!(out.status.code(), Some(2), "stderr: {}", stderr_of(&out));
    assert!(
        stderr_of(&out).contains("unknown role 'no-such-role'"),
        "stderr: {}",
        stderr_of(&out)
    );
}

#[test]
fn role_refusal_explorer_cannot_prove() {
    let out = run(&["--role", "explorer", "prove", "dec.d-squared-zero"]);
    assert_eq!(out.status.code(), Some(1), "stderr: {}", stderr_of(&out));
    let stderr = stderr_of(&out);
    assert!(
        stderr.contains("role explorer cannot prove"),
        "stderr: {stderr}"
    );
    assert!(stderr.contains("proposers do not mint"), "stderr: {stderr}");
}

#[test]
fn role_refusal_explorer_cannot_set() {
    let out = run(&[
        "--role",
        "explorer",
        "set",
        "standard-model",
        "generations",
        "2",
    ]);
    assert_eq!(out.status.code(), Some(1), "stderr: {}", stderr_of(&out));
    let stderr = stderr_of(&out);
    assert!(
        stderr.contains("role explorer cannot set"),
        "stderr: {stderr}"
    );
    assert!(stderr.contains("proposers do not mint"), "stderr: {stderr}");
}

#[test]
fn role_refusal_proof_searcher_cannot_review() {
    let out = run(&["--role", "proof-searcher", "review", "gut.weinberg-angle"]);
    assert_eq!(out.status.code(), Some(1), "stderr: {}", stderr_of(&out));
    let stderr = stderr_of(&out);
    assert!(
        stderr.contains("role proof-searcher cannot review"),
        "stderr: {stderr}"
    );
    assert!(stderr.contains("proposers do not mint"), "stderr: {stderr}");
}

#[test]
fn step_parses_scope_and_cap() {
    // step with an unknown scope must reach the lab (exit 1), not the
    // usage parser (exit 2): the scope travels through argv intact.
    let out = run(&["step", "nope-nope"]);
    assert_eq!(out.status.code(), Some(1), "stderr: {}", stderr_of(&out));
    assert!(
        stderr_of(&out).contains("unknown step scope"),
        "stderr: {}",
        stderr_of(&out)
    );
    // A non-numeric cap is a usage error.
    let out = run(&["step", "lab", "many"]);
    assert_eq!(out.status.code(), Some(2), "stderr: {}", stderr_of(&out));
    assert!(
        stderr_of(&out).contains("USAGE:"),
        "stderr: {}",
        stderr_of(&out)
    );
    // Role refusal for the lab-only step command.
    let out = run(&["--role", "explorer", "step", "dec.d-squared-zero"]);
    assert_eq!(out.status.code(), Some(1), "stderr: {}", stderr_of(&out));
    assert!(
        stderr_of(&out).contains("role explorer cannot step"),
        "stderr: {}",
        stderr_of(&out)
    );
}

#[test]
fn research_plan_dry_run_succeeds_with_marker() {
    let path = std::env::temp_dir().join(format!("physis-smoke-plan-{}.json", std::process::id()));
    std::fs::write(&path, physis_agent::campaign::EXAMPLE_MANIFEST).unwrap();
    let arg = path.to_string_lossy().into_owned();
    let out = run(&["research", "plan", arg.as_str()]);
    std::fs::remove_file(&path).ok();
    assert_eq!(out.status.code(), Some(0), "stderr: {}", stderr_of(&out));
    assert!(
        stdout_of(&out).contains("dry-run"),
        "stdout: {}",
        stdout_of(&out)
    );
}

#[test]
fn design_ranks_observables_and_freezes() {
    let out = run(&["design", "general-relativity", "newtonian-gravity"]);
    assert_eq!(out.status.code(), Some(0), "stderr: {}", stderr_of(&out));
    let stdout = stdout_of(&out);
    assert!(stdout.contains("discriminating claims"), "{stdout}");
    assert!(stdout.contains("frozen design"), "{stdout}");
    assert!(stdout.contains("gr.mercury-perihelion"), "{stdout}");
    assert!(stdout.contains("independence assumed"), "{stdout}");
}

#[test]
fn research_step_and_status_run_with_markers() {
    let tag = std::process::id();
    let path = std::env::temp_dir().join(format!("physis-smoke-r4-{tag}.json"));
    std::fs::write(&path, physis_agent::campaign::EXAMPLE_MANIFEST).unwrap();
    let camp = std::env::temp_dir().join(format!("physis-smoke-r4-{tag}-camp"));
    std::fs::remove_dir_all(&camp).ok();
    let arg = path.to_string_lossy().into_owned();
    let dir = camp.to_string_lossy().into_owned();
    let out = run(&["research", "step", arg.as_str(), dir.as_str()]);
    assert_eq!(out.status.code(), Some(0), "stderr: {}", stderr_of(&out));
    assert!(
        stdout_of(&out).contains("campaign step"),
        "stdout: {}",
        stdout_of(&out)
    );
    let status = run(&["research", "status", dir.as_str()]);
    assert_eq!(
        status.status.code(),
        Some(0),
        "stderr: {}",
        stderr_of(&status)
    );
    assert!(
        stdout_of(&status).contains("research status"),
        "stdout: {}",
        stdout_of(&status)
    );
    std::fs::remove_file(&path).ok();
    std::fs::remove_dir_all(&camp).ok();
}

#[test]
fn research_step_refuses_explorer_but_status_observes() {
    let tag = std::process::id();
    let path = std::env::temp_dir().join(format!("physis-smoke-r4-role-{tag}.json"));
    std::fs::write(&path, physis_agent::campaign::EXAMPLE_MANIFEST).unwrap();
    let camp = std::env::temp_dir().join(format!("physis-smoke-r4-role-{tag}-camp"));
    std::fs::remove_dir_all(&camp).ok();
    let arg = path.to_string_lossy().into_owned();
    let dir = camp.to_string_lossy().into_owned();
    let stepped = run(&["research", "step", arg.as_str(), dir.as_str()]);
    assert_eq!(
        stepped.status.code(),
        Some(0),
        "stderr: {}",
        stderr_of(&stepped)
    );
    let refused = run(&[
        "--role",
        "explorer",
        "research",
        "step",
        arg.as_str(),
        dir.as_str(),
    ]);
    assert_eq!(
        refused.status.code(),
        Some(1),
        "stderr: {}",
        stderr_of(&refused)
    );
    assert!(
        stderr_of(&refused).contains("role explorer cannot research-step"),
        "stderr: {}",
        stderr_of(&refused)
    );
    let observed = run(&["--role", "explorer", "research", "status", dir.as_str()]);
    assert_eq!(
        observed.status.code(),
        Some(0),
        "stderr: {}",
        stderr_of(&observed)
    );
    std::fs::remove_file(&path).ok();
    std::fs::remove_dir_all(&camp).ok();
}

#[test]
fn research_export_reports_json_and_markdown() {
    let tag = std::process::id();
    let path = std::env::temp_dir().join(format!("physis-smoke-r6-{tag}.json"));
    std::fs::write(&path, physis_agent::campaign::EXAMPLE_MANIFEST).unwrap();
    let camp = std::env::temp_dir().join(format!("physis-smoke-r6-{tag}-camp"));
    std::fs::remove_dir_all(&camp).ok();
    let arg = path.to_string_lossy().into_owned();
    let dir = camp.to_string_lossy().into_owned();
    let stepped = run(&["research", "step", arg.as_str(), dir.as_str()]);
    assert_eq!(
        stepped.status.code(),
        Some(0),
        "stderr: {}",
        stderr_of(&stepped)
    );
    let json = run(&["research", "export", dir.as_str()]);
    assert_eq!(json.status.code(), Some(0), "stderr: {}", stderr_of(&json));
    assert!(
        stdout_of(&json).contains("physis-campaign-report"),
        "stdout: {}",
        stdout_of(&json)
    );
    let md = run(&["research", "export", dir.as_str(), "markdown"]);
    assert_eq!(md.status.code(), Some(0), "stderr: {}", stderr_of(&md));
    assert!(
        stdout_of(&md).contains("# Campaign report"),
        "stdout: {}",
        stdout_of(&md)
    );
    let bad = run(&["research", "export", dir.as_str(), "yaml"]);
    assert_eq!(bad.status.code(), Some(1), "stderr: {}", stderr_of(&bad));
    std::fs::remove_file(&path).ok();
    std::fs::remove_dir_all(&camp).ok();
}

#[test]
fn research_plan_bad_version_is_refusal() {
    let path = std::env::temp_dir().join(format!("physis-smoke-bad-{}.json", std::process::id()));
    let bad = physis_agent::campaign::EXAMPLE_MANIFEST.replace("\"version\": 1", "\"version\": 2");
    std::fs::write(&path, bad).unwrap();
    let arg = path.to_string_lossy().into_owned();
    let out = run(&["research", "plan", arg.as_str()]);
    std::fs::remove_file(&path).ok();
    assert_eq!(out.status.code(), Some(1), "stderr: {}", stderr_of(&out));
    let stderr = stderr_of(&out);
    assert!(stderr.contains("error:"), "stderr: {stderr}");
    assert!(stderr.contains("version"), "stderr: {stderr}");
}

#[test]
fn allowed_commands_still_succeed() {
    // Positive control: the refusal gate must not swallow permitted ops.
    let out = run(&["layers"]);
    assert_eq!(out.status.code(), Some(0), "stderr: {}", stderr_of(&out));
    let out = run(&["--role", "explorer", "layers"]);
    assert_eq!(out.status.code(), Some(0), "stderr: {}", stderr_of(&out));
}
