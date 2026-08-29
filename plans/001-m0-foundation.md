# M0 — Foundation (this commit)

## Delivered

- Public GitHub repo `mingley/physis`
- Workspace: `physis-core`, `physis-model`, `physis-theory`, `physis-agent`, `physis`
- Typed SI quantities via `typenum`
- Layer id tower
- Knob protocol with domains
- Claims + verdicts + epistemic tags
- Worlds: spacetime, spectrum, gauge, gravity, landscape heuristic
- Theories: SM, GR, Type IIB, heterotic E₈×E₈, bosonic, observer-geometry
- Experiment `string-critique`
- CLI
- Specs, plans, docs, `AGENTS.md`
- CI: fmt, clippy `-D warnings`, test, one experiment run

## Done when

- [x] `cargo test --workspace`
- [x] `set type-iib total_dim 9` flips critical dimension
- [x] compile-fail doctest for mass + length
- [x] documentation matches code

## Explicitly incomplete (honest leftovers)

- No journal replay
- No Lie-algebra branching (SM embeddings are a table)
- No compactification topology that selects generations
- No Maxwell / circuits
- Finite Hilbert space only (qubits), not QFT
- Observer-geometry is a scaffold
