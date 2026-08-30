# 000 — Overview

Status: active
Layer: all

## Purpose

`physis` is a pure-Rust workspace whose job is to make *models of reality* mechanically inspectable:

- every quantity has a type-level dimension
- every theory is a bundle of knobs, a world projection, and a list of claims
- every claim evaluates to a verdict with orthogonal assurance axes (`class`, `derivation`, `empirical`, `semantic`)
- every claim has an explicit assumption set and a content-addressed statement hash
- every knob turn that matters produces a scientific-axis verdict diff, recorded in a journal

The workspace is reusable across scientific domains. The first domain is fundamental physics. The first experiment is a typed comparison of string constructions against empirical controls (Standard Model, GR) and against a unique-geometry scaffold.

## Non-goals (v0)

- A full lattice QFT or a Calabi–Yau compactification engine
- A PDE spacetime simulator
- An implementation of Geometric Unity
- A declaration that string theory is true or false
- Bindings to C/C++ physics libraries (proof *artifacts* from isolated formal checkers are a later Level-3 exception; see `specs/020-proof-carrying.md`)

## Success criteria for v0

1. `cargo test --workspace` is green.
2. Adding mass to length is a compile error.
3. `physis set type-iib total_dim 9` flips `consistency.critical-dimension` holds → fails.
4. The string-critique matrix is printable and includes SM, GR, at least two string constructions, bosonic string, and observer-geometry.
5. Specs, plans, and docs exist and match the code.
6. Assurance axes are present on verdicts, not just in prose. There is no forgeable `theorem` tag.

## Invariants

- No `unsafe`.
- No unverified FFI physics engines.
- Verdicts are internal to the encoding.
- Journals append; they do not rewrite.
- `MachineProved` cannot be manufactured by setting an enum.
- A `Holds` that is `Executed` is not a kernel proof.

## Related

- `specs/001-type-system.md`
- `specs/004-theories-and-claims.md`
- `specs/005-string-critique.md`
- `specs/016-blackbody.md`
- `specs/017-einstein-solid.md`
- `specs/018-light-deflection.md`
- `specs/020-proof-carrying.md`
- `plans/000-roadmap.md`
