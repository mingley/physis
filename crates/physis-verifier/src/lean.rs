//! Lean kernel compile + nanoda replay of a `lean4export` file.
//!
//! Both checkers must run. Missing tools is [`crate::VerifyError::LeanPipelineNotWired`],
//! not a mint.

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::time::Duration;

use physis_core::worker::{
    run_worker, WorkerFile, WorkerLimits, WorkerOutcome, WorkerRequest, WorkerTrust,
};
use physis_proof::{compact_lean_type, extract_theorems, Challenge};

use crate::{CheckerReceipt, VerifyError};

const LEAN_TOOLCHAIN: &str = "leanprover/lean4:v4.34.0-rc2";
const NANODA_VERSION: &str = "0.4.16";
const EXPORTER_VERSION: &str = "lean4export-3.1.0";

/// Locations of the two checkers. Absence of either means the pipeline is
/// not wired.
#[derive(Clone, Debug)]
pub struct LeanTools {
    lean: PathBuf,
    lake: PathBuf,
    lean4export: PathBuf,
}

/// Discover `lean`, `lake`, and `lean4export` (`LEAN4EXPORT` or PATH).
pub fn discover_tools() -> Option<LeanTools> {
    let lean = which("lean")?;
    let lake = which("lake")?;
    let lean4export = std::env::var_os("LEAN4EXPORT")
        .map(PathBuf::from)
        .filter(|p| p.is_file())
        .or_else(|| which("lean4export"))
        .or_else(|| {
            let p = PathBuf::from("/tmp/lean4export/.lake/build/bin/lean4export");
            p.is_file().then_some(p)
        })?;
    Some(LeanTools {
        lean,
        lake,
        lean4export,
    })
}

fn which(name: &str) -> Option<PathBuf> {
    let mut dirs: Vec<PathBuf> = std::env::var_os("PATH")
        .map(|p| std::env::split_paths(&p).collect())
        .unwrap_or_default();
    if let Some(home) = std::env::var_os("HOME") {
        dirs.push(PathBuf::from(home).join(".elan/bin"));
    }
    for dir in dirs {
        let p = dir.join(name);
        if p.is_file() {
            return Some(p);
        }
    }
    None
}

/// Compile untrusted Lean with the Lean kernel and replay the export with
/// nanoda. The source must contain a theorem whose type matches the challenge.
pub(crate) fn check_source(
    challenge: &Challenge,
    source: &str,
) -> Result<(CheckerReceipt, CheckerReceipt), VerifyError> {
    let want = compact_lean_type(challenge.lean_type());
    let thm = extract_theorems(source)
        .into_iter()
        .find(|t| compact_lean_type(&t.ty) == want)
        .ok_or(VerifyError::StatementMismatch)?;
    let tools = discover_tools().ok_or(VerifyError::LeanPipelineNotWired)?;
    let sh = ["/bin/sh", "/usr/bin/sh"]
        .iter()
        .map(PathBuf::from)
        .find(|p| p.is_file())
        .ok_or_else(|| VerifyError::LeanKernelRejected("no shell for lake pipeline".into()))?;

    // One worker run: build, then export the theorem to stdout. The lake
    // project files are worker inputs (integrity-checked); the theorem
    // name travels via `$THM`, never interpolated into the script.
    let mut env_extra = lake_env_extra();
    env_extra.push(("LAKE".into(), tools.lake.to_string_lossy().into_owned()));
    env_extra.push((
        "LEAN4EXPORT".into(),
        tools.lean4export.to_string_lossy().into_owned(),
    ));
    env_extra.push(("THM".into(), thm.name.clone()));
    let outcome = run_tool(
        &sh,
        vec!["-c".into(), lake_script().into()],
        vec![
            (
                "lean-toolchain".to_string(),
                format!("{LEAN_TOOLCHAIN}\n").into_bytes(),
            ),
            (
                "lakefile.toml".to_string(),
                LAKEFILE_TOML.as_bytes().to_vec(),
            ),
            ("Physlib.lean".to_string(), source.as_bytes().to_vec()),
        ],
        env_extra,
        LEAN_BUILD_WALL_SECS + LEAN_EXPORT_WALL_SECS,
        LEAN_EXPORT_OUTPUT,
    )?;
    if let Some(reason) = worker_failure("lake build + lean4export", &outcome) {
        return Err(VerifyError::LeanKernelRejected(reason));
    }
    let WorkerOutcome::Completed { stdout, .. } = outcome else {
        return Err(VerifyError::LeanKernelRejected(
            "lake pipeline misclassified".into(),
        ));
    };
    let primary = CheckerReceipt::ran("lean-kernel", &lean_version(&tools.lean), true);

    let tmp = tempfile_dir().map_err(VerifyError::LeanKernelRejected)?;
    let export_path = tmp.join("export.ndjson");
    std::fs::write(&export_path, &stdout)
        .map_err(|e| VerifyError::LeanKernelRejected(e.to_string()))?;

    nanoda_check(&export_path).inspect_err(|_| {
        let _ = std::fs::remove_dir_all(&tmp);
    })?;
    let secondary = CheckerReceipt::ran(
        "nanoda",
        &format!("{NANODA_VERSION}+{EXPORTER_VERSION}"),
        true,
    );

    let _ = std::fs::remove_dir_all(&tmp);
    Ok((primary, secondary))
}

/// Run a host-selected formal tool through the R3 worker boundary with
/// reviewed-local trust: the tool is pinned and discovered, the input is
/// confined to a private scratch directory, and wall time, output bytes,
/// environment, and input integrity are enforced.
fn run_tool(
    program: &Path,
    args: Vec<String>,
    files: Vec<(String, Vec<u8>)>,
    env_extra: Vec<(String, String)>,
    wall_secs: u64,
    output_bytes: u64,
) -> Result<WorkerOutcome, VerifyError> {
    run_worker(&WorkerRequest {
        program: program.to_path_buf(),
        args,
        files: files
            .into_iter()
            .map(|(path, bytes)| WorkerFile { path, bytes })
            .collect(),
        env_extra,
        trust: WorkerTrust::ReviewedLocal,
        limits: WorkerLimits {
            wall_time: Duration::from_secs(wall_secs),
            output_bytes,
            memory_mb: None,
        },
    })
    .map_err(VerifyError::LeanKernelRejected)
}

fn lean_version(lean: &Path) -> String {
    match run_tool(
        lean,
        vec!["--version".into()],
        Vec::new(),
        vec![("ELAN_TOOLCHAIN".into(), LEAN_TOOLCHAIN.into())],
        LEAN_VERSION_WALL_SECS,
        LEAN_VERSION_OUTPUT,
    ) {
        Ok(WorkerOutcome::Completed {
            success: true,
            stdout,
            ..
        }) => String::from_utf8_lossy(&stdout)
            .lines()
            .next()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .unwrap_or_else(|| LEAN_TOOLCHAIN.to_string()),
        _ => LEAN_TOOLCHAIN.to_string(),
    }
}

fn nanoda_check(export_path: &Path) -> Result<(), VerifyError> {
    let cfg: nanoda_lib::util::Config = serde_json::from_value(serde_json::json!({
        "export_file_path": export_path,
        "permitted_axioms": ["propext", "Quot.sound", "Classical.choice"],
        "unpermitted_axiom_hard_error": true,
        "print_success_message": false,
        "print_axioms": false,
        "nat_extension": true,
        "string_extension": true,
        "num_threads": 1usize
    }))
    .map_err(|e| VerifyError::NanodaRejected(e.to_string()))?;
    let (export_file, skipped) = cfg
        .to_export_file()
        .map_err(|e| VerifyError::NanodaRejected(e.to_string()))?;
    if !skipped.is_empty() {
        return Err(VerifyError::NanodaRejected(format!(
            "skipped unpermitted axioms {skipped:?}"
        )));
    }
    let ok = catch_unwind(AssertUnwindSafe(|| {
        export_file.check_all_declars();
    }));
    match ok {
        Ok(()) => Ok(()),
        Err(_) => Err(VerifyError::NanodaRejected(
            "nanoda panicked while typechecking the export".into(),
        )),
    }
}

fn tempfile_dir() -> Result<PathBuf, String> {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let dir = std::env::temp_dir().join(format!("physis-lean-{}-{}", std::process::id(), nanos));
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}

/// Minimal lake project manifest for the worker-run build.
const LAKEFILE_TOML: &str =
    "name = \"physlib\"\ndefaultTargets = [\"Physlib\"]\n\n[[lean_lib]]\nname = \"Physlib\"\n";

/// R3 worker limits for the formal-tool pipeline. Finite and generous:
/// the timeout bounds hangs, not slow first builds.
const LEAN_BUILD_WALL_SECS: u64 = 1200;
/// Export wall time: replay input, not a build.
const LEAN_EXPORT_WALL_SECS: u64 = 600;
/// Combined build+export output cap: failures are kilobytes, Physlib
/// exports are kilobytes; megabytes fail closed.
const LEAN_EXPORT_OUTPUT: u64 = 8 << 20;
/// `--version` is instant or broken.
const LEAN_VERSION_WALL_SECS: u64 = 30;
/// `--version` prints one line.
const LEAN_VERSION_OUTPUT: u64 = 65536;
/// Error-text bound: rejection messages never carry megabytes.
const LEAN_ERROR_HEAD: usize = 4096;

/// Shell pipeline for one worker run: build, then export the named
/// theorem to stdout. Tool paths arrive via `$LAKE`/`$LEAN4EXPORT` and
/// the theorem name via `$THM`, always double-quoted, so untrusted
/// source text cannot inject shell commands and no tool is PATH-resolved.
fn lake_script() -> &'static str {
    "set -e\n\"$LAKE\" build\n\"$LAKE\" env \"$LEAN4EXPORT\" Physlib -- \"$THM\"\n"
}

/// Host-selected environment additions for the lake pipeline: pinned
/// toolchain, elan-first PATH (same construction as before the R3
/// boundary: elan prepended to the inherited PATH), and HOME (elan
/// resolves toolchains under it; not a credential). Everything else
/// stays scrubbed.
fn lake_env_extra() -> Vec<(String, String)> {
    let mut env = vec![("ELAN_TOOLCHAIN".to_string(), LEAN_TOOLCHAIN.to_string())];
    if let Some(home) = std::env::var_os("HOME") {
        let elan = PathBuf::from(&home).join(".elan/bin");
        let mut dirs = vec![elan];
        if let Some(path) = std::env::var_os("PATH") {
            dirs.extend(std::env::split_paths(&path));
        }
        if let Ok(joined) = std::env::join_paths(dirs) {
            if let Some(path) = joined.to_str() {
                env.push(("PATH".to_string(), path.to_string()));
            }
        }
        if let Ok(home) = home.into_string() {
            env.push(("HOME".to_string(), home));
        }
    } else if std::env::var_os("PATH").is_some() {
        // No HOME: keep the inherited PATH so the pinned tools resolve.
        if let Some(path) = std::env::var_os("PATH").and_then(|p| p.into_string().ok()) {
            env.push(("PATH".to_string(), path));
        }
    }
    if !env.iter().any(|(k, _)| k == "PATH") {
        env.push(("PATH".to_string(), "/usr/bin:/bin".to_string()));
    }
    env
}

/// Explain a non-success worker outcome as a kernel rejection. `None`
/// when the tool ran successfully. Pure: unit-tested without the toolchain.
pub(crate) fn worker_failure(
    tool: &str,
    outcome: &physis_core::worker::WorkerOutcome,
) -> Option<String> {
    match outcome {
        WorkerOutcome::Completed { success: true, .. } => None,
        WorkerOutcome::Completed {
            success: false,
            code,
            stdout,
            stderr,
            ..
        } => Some(format!(
            "{tool} failed (code {code:?}): {}{}",
            head(stderr, LEAN_ERROR_HEAD / 2),
            head(stdout, LEAN_ERROR_HEAD / 2)
        )),
        WorkerOutcome::TimedOut { .. } => Some(format!("{tool} timed out")),
        WorkerOutcome::OutputLimit { .. } => Some(format!("{tool} exceeded its output limit")),
        WorkerOutcome::InputsMutated { file } => {
            Some(format!("{tool} modified its input '{file}'"))
        }
    }
}

/// First `n` bytes lossy, with a truncation marker when cut.
fn head(bytes: &[u8], n: usize) -> String {
    if bytes.len() <= n {
        return String::from_utf8_lossy(bytes).into_owned();
    }
    format!(
        "{}[truncated {} bytes]",
        String::from_utf8_lossy(&bytes[..n]),
        bytes.len() - n
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use physis_core::worker::WorkerOutcome;

    fn completed(success: bool, stdout: &[u8], stderr: &[u8]) -> WorkerOutcome {
        WorkerOutcome::Completed {
            success,
            code: Some(if success { 0 } else { 1 }),
            stdout: stdout.to_vec(),
            stderr: stderr.to_vec(),
            files: Vec::new(),
        }
    }

    #[test]
    fn success_maps_to_no_failure() {
        assert_eq!(
            worker_failure("lake build", &completed(true, b"ok", b"")),
            None
        );
    }

    #[test]
    fn failed_exit_carries_bounded_output() {
        let err = worker_failure("lake build", &completed(false, b"out", b"err")).unwrap();
        assert!(err.contains("lake build"), "{err}");
        assert!(err.contains("err"), "{err}");
        assert!(err.contains("out"), "{err}");
    }

    #[test]
    fn timeout_output_limit_and_mutation_map() {
        let t = worker_failure(
            "lake build",
            &WorkerOutcome::TimedOut {
                stdout: Vec::new(),
                stderr: Vec::new(),
            },
        )
        .unwrap();
        assert!(t.contains("timed out"), "{t}");
        let o = worker_failure(
            "lake build",
            &WorkerOutcome::OutputLimit {
                stdout: Vec::new(),
                stderr: Vec::new(),
            },
        )
        .unwrap();
        assert!(o.contains("output limit"), "{o}");
        let m = worker_failure(
            "lake build",
            &WorkerOutcome::InputsMutated {
                file: "Physlib.lean".into(),
            },
        )
        .unwrap();
        assert!(m.contains("Physlib.lean"), "{m}");
    }

    #[test]
    fn head_truncates_with_marker() {
        assert_eq!(head(b"abc", 10), "abc");
        let cut = head(b"abcdef", 4);
        assert!(cut.starts_with("abcd"), "{cut}");
        assert!(cut.contains("truncated"), "{cut}");
    }

    #[test]
    fn lake_script_is_quoted_and_sequenced() {
        let script = lake_script();
        assert!(script.contains("set -e"), "{script}");
        assert!(script.contains("\"$LAKE\" build"), "{script}");
        assert!(script.contains("\"$THM\""), "{script}");
        for var in ["$THM", "$LAKE", "$LEAN4EXPORT"] {
            let quoted = format!("\"{var}\"");
            assert_eq!(
                script.matches(var).count(),
                script.matches(quoted.as_str()).count(),
                "every {var} use must be double-quoted: {script}"
            );
        }
        assert!(
            !script.contains("lake "),
            "lake must be absolute-pathed via $LAKE, never PATH-resolved: {script}"
        );
    }

    #[test]
    fn lake_env_pins_toolchain() {
        let env = lake_env_extra();
        assert!(
            env.iter()
                .any(|(k, v)| k == "ELAN_TOOLCHAIN" && v == LEAN_TOOLCHAIN),
            "{env:?}"
        );
        assert!(env.iter().any(|(k, _)| k == "PATH"), "{env:?}");
    }
}
