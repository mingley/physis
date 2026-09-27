# Checker allowlist record (C6.2)

Maintainer-reviewed assessment binding the checker implementation to
its independence claim. A worker cannot approve its own
implementation: updating this record is a human review decision.

- Checker id: `dec-int-checker`
- Checker version: `1`
- Scope: integer obligations per `WIRE.md` v1 (`expect: zero` only).
- Algorithm: strict JSON parse, syntactic degree recomputation, full
  `{0..=d}^n` grid evaluation in checked i128. See `RATIONALE.md`.
- Shared with production: wire format, `serde`/`serde_json`, `sha2`.
  No `physis-*` dependency in any section (enforced by
  `tests/boundary.rs`, both directions).
- Unsafe: none (`#![forbid(unsafe_code)]`, enforced by test).
- Build recipe: `cargo build -p physis-repro-check` (workspace).
- Executable digest: measured by the host at spawn time and bound to
  the run; pinned digests live in deployment configuration, not here,
  because digests churn with every toolchain rebuild.
- Common-mode risks: wire-format misreading, integer semantics, grid
  theorem, JSON parsing. See `RATIONALE.md`.
- Assessment: independently implemented parser and checking
  algorithm; eligible as the second implementation for
  `dec.d-squared-zero` reproduction, pending per-run host binding.
- Review status: initial implementation, unreviewed.
- Reviewer: (maintainer signature on review)
- Reviewed source: (commit hash on review)
