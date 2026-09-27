//! Isolated worker boundary (R3, spec 022).
//!
//! One home for the host's process boundary, shared by the verifier's
//! formal-tool subprocesses today and the campaign coordinator's workers
//! in R4 (no new crate, per spec 022). Everything untrusted goes through
//! [`run_worker`]; nothing else in the workspace should spawn processes
//! for untrusted work.
//!
//! Two trust modes. [`WorkerTrust::ReviewedLocal`] runs a host-selected
//! tool with the enforceable pure-Rust subset: wall-time kill, per-stream
//! and output-dir byte caps, a scrubbed environment, a private scratch
//! working directory, null stdin, and input-integrity refusal (a worker
//! that modifies its inputs invalidates its own result). Memory limits,
//! network denial, and process-group containment need an approved OS
//! backend; R3 ships none (see [`approved_backends`]), so a request that
//! sets a memory limit is refused rather than silently unenforced, and
//! [`WorkerTrust::Untrusted`] is always refused. Reviewed-local runs keep
//! host networking: reviewing the tool is the operator's burden, and the
//! boundary never claims otherwise.
//!
//! A future backend plugs in by extending [`approved_backends`] with a
//! platform prober plus an enforcement path that the refusal sites consult.
//! Until then the absence of a backend is a loud refusal, never ambient
//! execution.

use std::io::Read;
use std::path::PathBuf;
use std::time::Duration;

/// Trust level of a worker program.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerTrust {
    /// Host-selected reviewed tool (for example the pinned Lean
    /// toolchain). Runs with the enforceable subset documented above.
    ReviewedLocal,
    /// Generated code or unattended workers. Requires an approved OS
    /// backend; R3 ships none, so this is always refused.
    Untrusted,
}

/// Enforceable limits for one worker run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorkerLimits {
    /// Kill the worker after this wall time. Must be positive.
    pub wall_time: Duration,
    /// Byte cap per captured stream and total for collected output files.
    pub output_bytes: u64,
    /// Memory cap in megabytes. Pure Rust cannot enforce it, so `Some`
    /// is always refused. `None` means the caller accepts host memory
    /// (reviewed tools only).
    pub memory_mb: Option<u64>,
}

/// One input or output file: a scratch-relative path plus bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkerFile {
    /// Scratch-relative path (`outputs/r.txt`). Absolute paths and
    /// `..` components are refused.
    pub path: String,
    /// File bytes.
    pub bytes: Vec<u8>,
}

/// One worker invocation. Program, argv, files, and environment additions
/// are all host-selected: the boundary takes no proposer-supplied
/// executable paths.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkerRequest {
    /// Host-selected absolute program path (verified to be a file).
    pub program: PathBuf,
    /// Host-selected argv (no proposer-supplied arguments in R3).
    pub args: Vec<String>,
    /// Files materialized under the private scratch directory.
    pub files: Vec<WorkerFile>,
    /// Environment additions on top of the scrubbed base (`PATH`,
    /// `SCRATCH_DIR`). Everything else is cleared.
    pub env_extra: Vec<(String, String)>,
    /// Trust level of the program.
    pub trust: WorkerTrust,
    /// Enforced limits.
    pub limits: WorkerLimits,
}

/// Outcome of a spawned worker.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WorkerOutcome {
    /// The worker ran to an exit status.
    Completed {
        /// Whether the exit status signals success.
        success: bool,
        /// Exit code, when the platform reports one.
        code: Option<i32>,
        /// Captured stdout (capped).
        stdout: Vec<u8>,
        /// Captured stderr (capped).
        stderr: Vec<u8>,
        /// Files collected from `outputs/` (capped).
        files: Vec<WorkerFile>,
    },
    /// The worker outlived its wall time and was killed, with partial output.
    TimedOut {
        /// Partial captured stdout.
        stdout: Vec<u8>,
        /// Partial captured stderr.
        stderr: Vec<u8>,
    },
    /// A stream or the output directory exceeded its byte cap; killed, with partial output.
    OutputLimit {
        /// Partial captured stdout.
        stdout: Vec<u8>,
        /// Partial captured stderr.
        stderr: Vec<u8>,
    },
    /// The worker modified one of its input files: the result is refused.
    InputsMutated {
        /// Scratch-relative path of the first modified input.
        file: String,
    },
}

/// Maximum files collected from `outputs/`.
pub const MAX_OUTPUT_FILES: usize = 64;

/// Approved OS isolation backends on this platform. Empty: R3 ships none,
/// so [`WorkerTrust::Untrusted`] is always refused. A backend joins this
/// list only with a platform prober plus enforced memory, network, and
/// process-group containment.
pub fn approved_backends() -> &'static [&'static str] {
    &[]
}

/// Guarantees the boundary enforces for [`WorkerTrust::ReviewedLocal`]
/// without a backend. Anything else (memory caps, network denial,
/// process-group kill) is a documented residual, not a silent promise.
pub fn enforced_guarantees() -> &'static [&'static str] {
    &[
        "wall-time kill",
        "output caps",
        "scrubbed env",
        "private scratch cwd",
        "null stdin",
        "input integrity",
    ]
}

/// Run one worker to an outcome. Refusals (untrusted without a backend,
/// unenforceable limits, unknown program, bad paths) are `Err` before any
/// spawn; everything else is a classified [`WorkerOutcome`]. The scratch
/// directory is always removed afterwards.
pub fn run_worker(req: &WorkerRequest) -> Result<WorkerOutcome, String> {
    if req.trust == WorkerTrust::Untrusted {
        return Err(
            "refused: unattended untrusted execution needs an approved isolation backend; \
             none is available (R3 ships no backends)"
                .into(),
        );
    }
    if let Some(mb) = req.limits.memory_mb {
        return Err(format!(
            "refused: memory limit of {mb}MB needs an approved isolation backend; \
             pure Rust cannot enforce it"
        ));
    }
    if req.limits.wall_time.is_zero() {
        return Err("refused: worker wall_time must be positive".into());
    }
    if !req.program.is_file() {
        return Err(format!(
            "refused: unknown program '{}': not a host-selected binary",
            req.program.display()
        ));
    }
    for file in &req.files {
        check_relpath(&file.path)?;
    }

    let scratch = make_scratch()?;
    let _cleanup = Cleanup {
        dir: scratch.clone(),
    };
    let mut input_hashes: Vec<(String, u64)> = Vec::with_capacity(req.files.len());
    for file in &req.files {
        let path = scratch.join(&file.path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("worker io: {e}"))?;
        }
        std::fs::write(&path, &file.bytes).map_err(|e| format!("worker io: {e}"))?;
        input_hashes.push((file.path.clone(), fnv1a(&file.bytes)));
    }
    let outputs = scratch.join("outputs");
    std::fs::create_dir_all(&outputs).map_err(|e| format!("worker io: {e}"))?;

    let mut cmd = std::process::Command::new(&req.program);
    cmd.args(&req.args)
        .current_dir(&scratch)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .env("SCRATCH_DIR", &scratch);
    for (k, v) in &req.env_extra {
        cmd.env(k, v);
    }
    let mut child = cmd.spawn().map_err(|e| format!("worker spawn: {e}"))?;
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();

    let shared = std::sync::Arc::new(std::sync::Mutex::new(child));
    let done = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let timed_out = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let wall = req.limits.wall_time;
    let watchdog = std::thread::spawn({
        let shared = std::sync::Arc::clone(&shared);
        let done = std::sync::Arc::clone(&done);
        let timed_out = std::sync::Arc::clone(&timed_out);
        move || {
            let start = std::time::Instant::now();
            loop {
                if done.load(std::sync::atomic::Ordering::SeqCst) {
                    return;
                }
                let elapsed = start.elapsed();
                if elapsed >= wall {
                    break;
                }
                let remaining = wall - elapsed;
                std::thread::sleep(remaining.min(Duration::from_millis(50)));
            }
            if !done.load(std::sync::atomic::Ordering::SeqCst) && unlock(&shared).kill().is_ok() {
                timed_out.store(true, std::sync::atomic::Ordering::SeqCst);
            }
        }
    });

    let cap = req.limits.output_bytes;
    let err_thread = std::thread::spawn(move || read_capped(stderr, cap));
    let (out_bytes, out_hit) = read_capped(stdout, cap);
    let (err_bytes, err_hit) = err_thread.join().unwrap_or_default();
    if out_hit || err_hit {
        unlock(&shared).kill().ok();
    }
    let status = unlock(&shared)
        .wait()
        .map_err(|e| format!("worker wait: {e}"))?;
    done.store(true, std::sync::atomic::Ordering::SeqCst);
    let _ = watchdog.join();

    if timed_out.load(std::sync::atomic::Ordering::SeqCst) {
        return Ok(WorkerOutcome::TimedOut {
            stdout: out_bytes,
            stderr: err_bytes,
        });
    }
    if out_hit || err_hit {
        return Ok(WorkerOutcome::OutputLimit {
            stdout: out_bytes,
            stderr: err_bytes,
        });
    }
    for (path, before) in &input_hashes {
        let after = std::fs::read(scratch.join(path))
            .map_err(|e| format!("worker io: {e}"))
            .map(|b| fnv1a(&b))?;
        if after != *before {
            return Ok(WorkerOutcome::InputsMutated { file: path.clone() });
        }
    }
    let files = collect_outputs(&outputs, cap)?;
    if files.is_none() {
        return Ok(WorkerOutcome::OutputLimit {
            stdout: out_bytes,
            stderr: err_bytes,
        });
    }
    Ok(WorkerOutcome::Completed {
        success: status.success(),
        code: status.code(),
        stdout: out_bytes,
        stderr: err_bytes,
        files: files.unwrap_or_default(),
    })
}

/// Lock a worker child without ever panicking on poisoning.
fn unlock(
    child: &std::sync::Mutex<std::process::Child>,
) -> std::sync::MutexGuard<'_, std::process::Child> {
    child.lock().unwrap_or_else(|e| e.into_inner())
}

/// Reject anything but a clean scratch-relative path.
fn check_relpath(path: &str) -> Result<(), String> {
    use std::path::Component;
    if path.is_empty() {
        return Err("refused: worker file path must not be empty".into());
    }
    let mut normal = 0;
    for c in std::path::Path::new(path).components() {
        match c {
            Component::Normal(_) => normal += 1,
            _ => {
                return Err(format!(
                    "refused: worker file path '{path}' is not scratch-relative"
                ));
            }
        }
    }
    if normal == 0 {
        return Err(format!("refused: worker file path '{path}' names no file"));
    }
    Ok(())
}

/// Read a piped stream to `cap + 1` bytes, reporting whether the cap was
/// exceeded. A torn read keeps its partial bytes; the outcome flags (not
/// the read error) decide the classification.
fn read_capped<R: std::io::Read>(pipe: Option<R>, cap: u64) -> (Vec<u8>, bool) {
    let Some(mut pipe) = pipe else {
        return (Vec::new(), false);
    };
    let mut buf = Vec::new();
    let limit = cap.saturating_add(1);
    let _ = std::io::Read::take(&mut pipe, limit).read_to_end(&mut buf);
    let hit = buf.len() as u64 > cap;
    (buf, hit)
}

/// Collect top-level files from `outputs/`: at most [`MAX_OUTPUT_FILES`]
/// files and `cap` bytes total. `Ok(None)` means the collection itself
/// exceeded the cap.
fn collect_outputs(dir: &PathBuf, cap: u64) -> Result<Option<Vec<WorkerFile>>, String> {
    let mut files = Vec::new();
    let mut total = 0u64;
    let mut entries: Vec<PathBuf> = std::fs::read_dir(dir)
        .map_err(|e| format!("worker io: {e}"))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    entries.sort();
    for path in entries {
        if !path.is_file() {
            continue;
        }
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| "worker outputs: non-utf8 file name".to_string())?
            .to_string();
        let bytes = std::fs::read(&path).map_err(|e| format!("worker io: {e}"))?;
        total += bytes.len() as u64;
        files.push(WorkerFile { path: name, bytes });
        if files.len() > MAX_OUTPUT_FILES || total > cap {
            return Ok(None);
        }
    }
    Ok(Some(files))
}

/// Non-cryptographic integrity hash for worker inputs (equality only, not
/// identity: content addresses stay with [`crate::ArtifactId`]).
fn fnv1a(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in bytes {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

static SCRATCH_CTR: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Fresh private scratch directory.
fn make_scratch() -> Result<PathBuf, String> {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let n = SCRATCH_CTR.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let dir =
        std::env::temp_dir().join(format!("physis-worker-{}-{nanos}-{n}", std::process::id()));
    std::fs::create_dir_all(&dir).map_err(|e| format!("worker io: {e}"))?;
    Ok(dir)
}

/// Best-effort scratch removal on every return path.
struct Cleanup {
    dir: PathBuf,
}

impl Drop for Cleanup {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.dir).ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// First existing absolute candidate. The test process is the host:
    /// fixtures are resolved, never proposer-supplied.
    fn fixture(candidates: &[&str]) -> PathBuf {
        for c in candidates {
            let p = PathBuf::from(c);
            if p.is_file() {
                return p;
            }
        }
        panic!("no test fixture found among {candidates:?}");
    }

    fn echo() -> PathBuf {
        fixture(&["/bin/echo", "/usr/bin/echo"])
    }

    fn sh() -> PathBuf {
        fixture(&["/bin/sh", "/usr/bin/sh"])
    }

    fn request(program: PathBuf) -> WorkerRequest {
        WorkerRequest {
            program,
            args: Vec::new(),
            files: Vec::new(),
            env_extra: Vec::new(),
            trust: WorkerTrust::ReviewedLocal,
            limits: WorkerLimits {
                wall_time: Duration::from_secs(30),
                output_bytes: 1 << 20,
                memory_mb: None,
            },
        }
    }

    #[test]
    fn echo_succeeds_with_captured_output() {
        let mut req = request(echo());
        req.args = vec!["hello".into()];
        match run_worker(&req).unwrap() {
            WorkerOutcome::Completed {
                success,
                code,
                stdout,
                files,
                ..
            } => {
                assert!(success);
                assert_eq!(code, Some(0));
                assert_eq!(stdout, b"hello\n");
                assert!(files.is_empty());
            }
            other => panic!("unexpected outcome: {other:?}"),
        }
    }

    #[test]
    fn argv_passthrough_is_host_selected() {
        let mut req = request(echo());
        req.args = vec!["a".into(), "b".into()];
        match run_worker(&req).unwrap() {
            WorkerOutcome::Completed { stdout, .. } => assert_eq!(stdout, b"a b\n"),
            other => panic!("unexpected outcome: {other:?}"),
        }
    }

    #[test]
    fn cwd_is_private_scratch_removed_after() {
        let req = request(fixture(&["/bin/pwd", "/usr/bin/pwd"]));
        let out = match run_worker(&req).unwrap() {
            WorkerOutcome::Completed { stdout, .. } => String::from_utf8(stdout).unwrap(),
            other => panic!("unexpected outcome: {other:?}"),
        };
        let cwd = out.trim();
        assert!(cwd.contains("physis-worker"), "{cwd}");
        assert!(
            !PathBuf::from(cwd).exists(),
            "scratch must be removed after the run: {cwd}"
        );
    }

    #[test]
    fn env_is_scrubbed_to_allowlist() {
        std::env::set_var("PHYSIS_R3_SMOKE_SECRET", "leak-me");
        let req = request(fixture(&["/usr/bin/env", "/bin/env"]));
        let out = match run_worker(&req).unwrap() {
            WorkerOutcome::Completed { stdout, .. } => String::from_utf8(stdout).unwrap(),
            other => panic!("unexpected outcome: {other:?}"),
        };
        std::env::remove_var("PHYSIS_R3_SMOKE_SECRET");
        assert!(!out.contains("PHYSIS_R3_SMOKE_SECRET"), "{out}");
        assert!(out.contains("SCRATCH_DIR="), "{out}");
    }

    #[test]
    fn stdin_is_null() {
        let mut req = request(fixture(&["/bin/cat", "/usr/bin/cat"]));
        req.limits.wall_time = Duration::from_secs(10);
        match run_worker(&req).unwrap() {
            WorkerOutcome::Completed {
                success, stdout, ..
            } => {
                assert!(success);
                assert!(stdout.is_empty());
            }
            other => panic!("unexpected outcome: {other:?}"),
        }
    }

    #[test]
    fn timeout_kills_the_worker() {
        let mut req = request(fixture(&["/bin/sleep", "/usr/bin/sleep"]));
        req.args = vec!["30".into()];
        req.limits.wall_time = Duration::from_secs(1);
        let start = std::time::Instant::now();
        match run_worker(&req).unwrap() {
            WorkerOutcome::TimedOut { .. } => {}
            other => panic!("unexpected outcome: {other:?}"),
        }
        assert!(
            start.elapsed() < Duration::from_secs(25),
            "the worker must be killed, not awaited"
        );
    }

    #[test]
    fn output_cap_kills_the_worker() {
        let mut req = request(fixture(&["/usr/bin/yes", "/bin/yes"]));
        req.limits.output_bytes = 65536;
        match run_worker(&req).unwrap() {
            WorkerOutcome::OutputLimit { stdout, .. } => {
                assert!(stdout.len() <= 65536 + 1, "len {}", stdout.len());
            }
            other => panic!("unexpected outcome: {other:?}"),
        }
    }

    #[test]
    fn missing_program_is_refused() {
        let req = request(PathBuf::from("/nonexistent/physis-r3-smoke"));
        let err = run_worker(&req).unwrap_err();
        assert!(err.contains("program"), "{err}");
    }

    #[test]
    fn untrusted_is_refused_without_backend() {
        let mut req = request(echo());
        req.args = vec!["x".into()];
        req.trust = WorkerTrust::Untrusted;
        let err = run_worker(&req).unwrap_err();
        assert!(err.contains("refused"), "{err}");
        assert!(err.contains("backend"), "{err}");
    }

    #[test]
    fn memory_limit_is_refused_as_unenforceable() {
        let mut req = request(echo());
        req.args = vec!["x".into()];
        req.limits.memory_mb = Some(512);
        let err = run_worker(&req).unwrap_err();
        assert!(err.contains("memory"), "{err}");
    }

    #[test]
    fn zero_wall_time_is_refused() {
        let mut req = request(echo());
        req.limits.wall_time = Duration::ZERO;
        let err = run_worker(&req).unwrap_err();
        assert!(err.contains("wall_time"), "{err}");
    }

    #[test]
    fn input_mutation_is_detected() {
        let mut req = request(sh());
        req.args = vec!["-c".into(), "echo tampered > in.txt".into()];
        req.files = vec![WorkerFile {
            path: "in.txt".into(),
            bytes: b"pristine".to_vec(),
        }];
        match run_worker(&req).unwrap() {
            WorkerOutcome::InputsMutated { file } => assert_eq!(file, "in.txt"),
            other => panic!("unexpected outcome: {other:?}"),
        }
    }

    #[test]
    fn path_traversal_is_refused() {
        for bad in ["../evil", "/abs", "", ".", "a/../../b"] {
            let mut req = request(echo());
            req.files = vec![WorkerFile {
                path: bad.into(),
                bytes: Vec::new(),
            }];
            let err = run_worker(&req).unwrap_err();
            assert!(err.contains("path"), "{bad}: {err}");
        }
    }

    #[test]
    fn output_files_are_collected() {
        let mut req = request(sh());
        req.args = vec!["-c".into(), "echo hi > outputs/r.txt".into()];
        match run_worker(&req).unwrap() {
            WorkerOutcome::Completed { files, .. } => {
                assert_eq!(files.len(), 1);
                assert_eq!(files[0].path, "r.txt");
                assert_eq!(files[0].bytes, b"hi\n");
            }
            other => panic!("unexpected outcome: {other:?}"),
        }
    }

    #[test]
    fn output_file_cap_is_enforced() {
        let mut req = request(sh());
        req.args = vec!["-c".into(), "head -c 262144 /dev/zero > outputs/big".into()];
        req.limits.output_bytes = 65536;
        match run_worker(&req).unwrap() {
            WorkerOutcome::OutputLimit { .. } => {}
            other => panic!("unexpected outcome: {other:?}"),
        }
    }

    #[test]
    fn nonzero_exit_is_completed_not_success() {
        let mut req = request(sh());
        req.args = vec!["-c".into(), "exit 3".into()];
        match run_worker(&req).unwrap() {
            WorkerOutcome::Completed { success, code, .. } => {
                assert!(!success);
                assert_eq!(code, Some(3));
            }
            other => panic!("unexpected outcome: {other:?}"),
        }
    }

    #[test]
    fn status_reports_no_backend_and_guarantees() {
        assert!(approved_backends().is_empty());
        let guarantees = enforced_guarantees();
        for want in ["wall-time", "output", "env", "scratch", "stdin", "input"] {
            assert!(
                guarantees.iter().any(|g| g.contains(want)),
                "missing guarantee for {want}: {guarantees:?}"
            );
        }
    }
}
