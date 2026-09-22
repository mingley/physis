# 007 — Lab improvement (engineering track)

Status: active (plan). Audience: agents.
Does not weaken `specs/020-proof-carrying.md`. Does not fork TODO.md/plan 006.
Calendar estimates are not used.

## Goal

Raise the engineering quality of physis — hygiene, modularity, test
architecture, docs/spec consistency — without touching the science queue,
trust semantics, or behavior. Land every unit on main as atomic commits.

## Success Criteria

- Full CI gate baseline (fmt, clippy, test, experiment, prove) re-verified
  locally and recorded.
- `cosmological_constant` dead knob resolved (wired or removed) with docs
  and tests updated.
- Stale counts/wording in docs/specs/plans fixed with evidence, or recorded
  as verified-current.
- Every default-lab theory has a knob-to-verdict-diff test, or the gap is
  recorded with an owner.
- Constants-ledger duplication/drift findings recorded; tooling built only
  if the audit justifies it.
- `lab.rs` module map recorded; a split lands only under
  zero-behavior-change guardrails.
- CLI smoke tests for help/unknown-command/role-refusal merged.
- Every changing unit pushed to main as its own atomic commit with a
  CHANGELOG line; `cargo test --workspace` green at each push.

## Context And Current Facts

- Workspace: 15 crates, 74 `.rs` files, ~175.7k LOC (measured 2026-09-22).
  Three files hold ~135k: `crates/physis-constants/src/lib.rs` (90,630
  lines, single-file hand-maintained SI/CODATA ledger),
  `crates/physis-agent/src/lab.rs` (29,553 lines, ~196 fns),
  `crates/physis-model/src/constants.rs` (14,645 lines, Qty lockstep
  mirror). `physis-theory` is well split (20+ modules, 20 test files).
- Green this session: `cargo test --workspace` (exit 0),
  `cargo fmt --all -- --check` (clean). Clippy not run locally this
  session; CI gates it (last push 2026-09-04).
- CI (`.github/workflows/ci.yml`): fmt check, clippy `-D warnings`,
  workspace tests, `experiment string-critique`, `prove dec.d-squared-zero`;
  Lean 4.34 + lean4export installed with cache.
- 11 live experiments verified from the binary; docs polished in `83d4b5c`,
  on main.
- Science queue lives in TODO.md/plan 006 (next: C2.1). Repo rule: one
  mechanical idea per commit, direct to main, one CHANGELOG line each
  (CHANGELOG header, git log shape).
- Known rough edges: weakly-dead `cosmological_constant` (docs/KNOBS.md
  "Dead knobs" plus a stale "M1 should…" note); `crates/physis/` facade
  has 0 test files (grep); "holds + theorem" wording vs
  no-`Epistemic::Theorem` consistency unverified.

## Constraints And Non-goals

- Science queue untouched: no C*/R* task work; TODO.md/plan 006 stay
  authoritative for physics.
- No trust-semantics change: no spec 020 edits, no AGENTS.md rewrite, no
  new proof obligations, no trust promotion, no P4 claims.
- No behavior change except U1's dead-knob resolution; no new
  theories/knobs/constants/datasets/dependencies.
- No CHANGELOG history rewrite (append-only contract); no `lab.rs` API
  changes in U5.
- Pushes: atomic commits to main, each green, each with a CHANGELOG line.

## Key Decisions

- Separate engineering track (this plan) from science track (TODO.md/plan
  006): avoids forking the pickup queue; lanes never claim C*/R* IDs.
  Rejected: folding into TODO.md (mixes physics acceptance with hygiene)
  and a long-lived branch (repo ships direct-to-main atomics).
- Evidence-first on god files: U4/U5 audit and map before any split; a
  split lands only with zero behavior diff and green tests, else the unit
  ends as a recorded map. Rejected: upfront rewrite (regression risk on
  load-bearing code that plan 006 protects).
- Parallel lanes with disjoint file ownership; `lab.rs`/`protocol.rs`/
  claim-ids single-owner (U5) per the TODO.md coordination rule. Rejected:
  free-for-all parallelism (merge conflicts on shared files).
- U0 baseline first: clippy plus full gates re-verified locally before
  lanes start, so a red baseline can't be blamed on lanes.

## Recommended Approach

Three waves. U0 records the green baseline. Wave 1 runs six independent
lanes (U1–U6) in parallel with disjoint files. Wave 2 (U7–U8) acts only on
Wave-1 findings. Each unit: implement, then fmt plus clippy plus targeted
tests, then full `cargo test --workspace`, then one atomic commit (repo
voice plus CHANGELOG line) pushed to main. A lane whose premise proves
wrong stops and records instead of forcing a change.

## Work Plan

- U0 — Gate baseline (depends: none; owns: nothing, read-only). Re-run
  fmt, clippy, workspace tests, the string-critique experiment, and the
  `dec.d-squared-zero` prove locally; report baseline commit plus outputs.
  No commit.
- U1 — Dead-knob resolution (depends: U0; owns: knob owner file(s),
  docs/KNOBS.md, specs/003-knobs-and-causality.md). Call-graph check on
  `cosmological_constant`, then wire a claim or remove; update docs,
  tests, changelog.
- U2 — Docs/spec consistency audit (depends: U0; owns: docs/, specs/,
  plans/000-roadmap.md, README.md; docs-only). Verify each stale-suspect
  count/wording against code; fix or record-verified.
- U3 — Test-coverage audit (depends: U0; owns: crates/physis-theory
  tests). List default-lab theories, check CONTRIBUTING rule 5 (knob-diff
  test each); add missing tests, no behavior change.
- U4 — Constants-ledger audit (depends: U0; owns: nothing, read-only).
  Map duplication/drift between the 90k-line ledger and the 14k-line Qty
  mirror; judge generator feasibility; report findings plus recommendation.
- U5 — `lab.rs` modularization probe (depends: U0; owns:
  crates/physis-agent/src/). Map ~196 fns to candidate modules; split only
  with zero API/behavior change and green tests, else record the map.
- U6 — Facade CLI smoke tests (depends: U0; owns: crates/physis/). Cover
  help, unknown-command, and role-refusal exit codes.
- U7 — Constants drift tooling (depends: U4; owns: per U4). Generator or
  lint, built only if U4 recommends it.
- U8 — CI/MSRV verification (depends: U0; owns: .github/workflows/,
  CONTRIBUTING.md or docs/). Reconcile `rust-version 1.85` with stable CI;
  document or minimal CI tweak.

U1–U6 run in parallel (separate agents, disjoint files). U7 is gated on
U4's recommendation; U8 is small and parallel-safe.

## Validation Plan

- Every unit: `cargo fmt --all -- --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`, the smallest
  targeted test selector, then full `cargo test --workspace`; push only
  when all green.
- U0 additionally: `cargo run -p physis -- experiment string-critique` and
  `cargo run -p physis -- prove dec.d-squared-zero` (CI parity).
- U1: new/removed-knob test shows the verdict diff or clean absence;
  `physis knobs` output inspected.
- U2: every touched claim cites file:line evidence; link check re-run.
- U5: identical behavior proven by the full suite green before/after plus
  a move-only `git diff` review; any API diff aborts the split.
- Highest-risk step: the U5 split — guardrailed to map-only on any doubt.

## Risks / Rollback

- U5 regression risk: guardrails above; rollback is reverting the single
  atomic commit (each unit is independently revertible).
- Lane conflict on shared files: disjoint ownership plus U5 as sole owner
  of physis-agent/src; the coordinator serializes pushes.
- New-lints red baseline (stable moved since Sep 4): U0 owns it — fix
  lints or record evidence; lanes don't start red.
- Scope creep into the science queue: units are U*, never C*/R*; the
  coordinator rejects out-of-scope diffs.

## Open Questions

None. All material facts were verified in the tree (see Context); the U4
and U5 questions are answered by their own audits during execution.
