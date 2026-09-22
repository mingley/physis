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
fn allowed_commands_still_succeed() {
    // Positive control: the refusal gate must not swallow permitted ops.
    let out = run(&["layers"]);
    assert_eq!(out.status.code(), Some(0), "stderr: {}", stderr_of(&out));
    let out = run(&["--role", "explorer", "layers"]);
    assert_eq!(out.status.code(), Some(0), "stderr: {}", stderr_of(&out));
}
