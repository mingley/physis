//! CLI tests: obligation file in, verdict file out, exit codes.

use std::path::PathBuf;

fn checker_bin() -> PathBuf {
    // Same-package binary under test; probe both spellings.
    if let Some(p) = option_env!("CARGO_BIN_EXE_physis_repro_check") {
        return PathBuf::from(p);
    }
    if let Some(p) = option_env!("CARGO_BIN_EXE_physis-repro-check") {
        return PathBuf::from(p);
    }
    panic!("checker binary not built");
}

fn scratch(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("physis-r6x-cli-{}-{name}", std::process::id()))
}

fn triangle_obligation() -> Vec<u8> {
    br#"{"expect":"zero","run_id":"cli1","schema":"physis-int-obligation","symbols":["a","b","c"],"tree":{"args":[{"args":[{"args":[{"var":"b"},{"var":"a"}],"op":"sub"},{"args":[{"var":"c"},{"var":"a"}],"op":"sub"}],"op":"sub"},{"args":[{"var":"c"},{"var":"b"}],"op":"sub"}],"op":"add"},"version":1}"#.to_vec()
}

fn run_checker(obligation: &[u8], name: &str) -> (std::process::Output, PathBuf) {
    let dir = scratch(name);
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    let input = dir.join("obligation.json");
    let output = dir.join("verdict.json");
    std::fs::write(&input, obligation).unwrap();
    let out = std::process::Command::new(checker_bin())
        .arg(&input)
        .arg(&output)
        .env("WORKER_EXE_DIGEST", "feedface")
        .output()
        .expect("spawn checker");
    (out, output)
}

fn verdict_of(path: &PathBuf) -> serde_json::Value {
    let text = std::fs::read_to_string(path).expect("verdict file");
    serde_json::from_str(&text).expect("verdict json")
}

#[test]
fn cli_agrees_on_triangle() {
    let (out, verdict) = run_checker(&triangle_obligation(), "agree");
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let v = verdict_of(&verdict);
    assert_eq!(v["disposition"], "agree");
    assert_eq!(v["run_id"], "cli1");
    assert_eq!(v["exe_sha256"], "feedface");
    assert_eq!(v["evaluated_points"], 8);
    assert_eq!(v["schema"], "physis-int-verdict");
}

#[test]
fn cli_disagrees_with_witness() {
    let mut bytes = triangle_obligation();
    let text = String::from_utf8(bytes.clone()).unwrap();
    // Flip the first sub to add: (b+a)-(c-a)+(c-b) = 2a.
    let flipped = text.replacen("\"op\":\"sub\"", "\"op\":\"add\"", 1);
    assert_ne!(flipped, text);
    bytes = flipped.into_bytes();
    let (out, verdict) = run_checker(&bytes, "disagree");
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let v = verdict_of(&verdict);
    assert_eq!(v["disposition"], "disagree");
    assert!(v["witness"]["a"].is_number(), "{v}");
}

#[test]
fn cli_malformed_writes_invalid_verdict() {
    let (out, verdict) = run_checker(b"{torn", "malformed");
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let v = verdict_of(&verdict);
    assert_eq!(v["disposition"], "invalid");
}

#[test]
fn cli_bad_argv_fails_without_verdict() {
    let out = std::process::Command::new(checker_bin())
        .arg("only-one")
        .output()
        .expect("spawn checker");
    assert!(!out.status.success());
}

#[test]
fn cli_missing_input_is_execution_failure() {
    let dir = scratch("missing");
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    let output = dir.join("verdict.json");
    let out = std::process::Command::new(checker_bin())
        .arg(dir.join("nope.json"))
        .arg(&output)
        .output()
        .expect("spawn checker");
    assert!(!out.status.success());
    assert!(!output.exists(), "no verdict on unreadable input");
}
