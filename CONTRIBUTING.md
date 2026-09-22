# Contributing

`physis` is a laboratory. Changes that do not move a knob, a claim, a layer, or a test usually do not belong.

Research contracts, accurate architecture docs and actionable task handoffs
also belong. Start from [TODO.md](TODO.md), select one dependency-ready task,
and read [the research workflow](docs/RESEARCH-LAB.md). Do not substitute a
new constant or an unrelated catalog identity for the task's scientific goal.

## Setup

```bash
rustc --version   # 1.85+
cargo test --workspace
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
```

## MSRV story

- Floor: Rust 1.85, declared once as workspace `rust-version` in
  [Cargo.toml](Cargo.toml) and inherited by every crate
  (`rust-version.workspace = true`). Do not use language or library
  features newer than 1.85 in workspace code.
- Day-to-day toolchain: stable (`rust-toolchain.toml`; CI installs
  `dtolnay/rust-toolchain@stable`). CI gates stable only — fmt check,
  `clippy -D warnings`, workspace tests, the `string-critique`
  experiment, and the `dec.d-squared-zero` prove. There is deliberately
  no pinned-1.85 CI job: the edition-2021 workspace rarely risks
  MSRV drift, and a second toolchain job would slow every gate for
  little signal.
- Raising the floor means changing the workspace `rust-version` and this
  section together, in one commit.

## Adding a theory

1. New file under `crates/physis-theory/src/`.
2. Implement `Knobbed` + `Theory`.
3. Shared claim ids live in `claims.rs` — reuse them so matrices stay comparable.
4. Register in `Lab::standard` if it belongs in the default lab.
5. Add a test that turns a knob and checks a verdict diff.
6. Document honesty: theorem / encoded-fact / conjecture / heuristic / open.

## Adding a layer

Layers are `LayerId` variants plus a model type in `physis-model`. Do not skip the spec: `specs/002-ontology-layers.md`.

## Adding a domain (electricity, computation, …)

See `specs/007-reuse-domains.md`. Domain work should not fork the core. It should implement `Theory` (and maybe a new `LayerId` if the existing tower is genuinely missing a stratum).

## Pull requests

- One mechanical idea per PR.
- Name the claims you changed.
- Do not expand the string-critique honesty notes into marketing.
- Include the task ID, assumptions/domain, expected scientific-axis change,
  negative control, evidence and remaining uncertainty.
- Update TODO status only when its acceptance condition is demonstrated.
  Documentation-only changes must not mark planned runtime behavior complete.
  Check their links and consistency; executable changes still follow the
  existing Rust gates.
