# 006 — Agent protocol

Status: active
Layer: agent

## Commands

| op | meaning |
|---|---|
| `layers` | list `LayerId` with docs |
| `theories` | list lab theories |
| `knobs [theory]` | snapshot knobs |
| `run <theory>` | evaluate all claims |
| `set <theory> <knob> <value>` | turn a knob, print Δverdicts (kind plus derivation / empirical / judgment when those axes move) |
| `epistemics` | tally every verdict by class, derivation, and semantic axes (no theorem row) |
| `why <claim>` | assumptions, typed judgment, derived trust, axiom closure, kernel receipt or none |
| `evidence <claim>` | competing encodings (distinct statement hashes of one slug) and competing evaluations; inserts a content-addressed Evidence DAG (not deserialized); confidence is a derived TrustProfile, not a numeric score; never Canonical or P4 |
| `prove <claim>` | dual-check a catalogued identity; only `physis-verifier` mints |
| `falsify <claim>` | search knobs for a failing evaluation |
| `hypothesize [theory]` | constrained structural mutation: chosen/fitted knob probes **and** IR package forks (`Theory::structural_mutations`); measured/derived knobs stay frozen; package mutants are not installed; does not persist or mint. `add-feedback` (NAND cycle), `add-contention` (NAND multi-driven net), `add-next-nearest` (Klein–Gordon locality and Dirac hopping), `add-quartic` (Klein–Gordon minus-φ⁴), `add-rectangle` (Wilson U(1)/SU(2)/SU(3) 2×1 loops), `add-higgs` (Wilson fundamental Higgs screening), `add-tline` (ohm-circuit transmission-line delay), `add-flux` (ohm-circuit unlumped mesh flux), `add-product` (bell-test product ket), `add-pr-box` (bell-test PR-box correlator), `add-schwarzschild` (newtonian-gravity 3GM u² Binet term), `add-yukawa` (newtonian-gravity Yukawa Soldner suppression), `add-tellegen` (linear-medium magnetoelectric mixing), `add-chiral` (linear-medium Pasteur chirality), `add-monopole` (maxwell-vacuum magnetic current), `add-proca` (maxwell-vacuum Proca mass), `add-bose` (ideal-gas Bose statistics), `add-fermi` (ideal-gas degenerate Fermi), `add-kt` (landauer-engine dropped ln2), `add-demon` (landauer-engine Maxwell demon), and `add-wilson` (dirac-fermion Wilson r term), and `add-r-squared` (general-relativity Starobinsky R²), and `add-brans-dicke` (general-relativity Brans-Dicke PPN), and `add-binomial-gamma` (special-relativity truncated γ), and `add-minus-uv` (special-relativity minus-uv composition), and `add-wien` (planck Wien occupation), and `add-zero-point` (planck zero-point vacuum), and `add-sign-flip` (de-rham coboundary first minus), and `add-down-laplacian` (de-rham down Laplacian), and `add-oracle` (turing-machine halt oracle), and `add-tired-light` (olbers-static exponential energy loss), and `add-missing-10` (su5-gut incomplete generation), and `add-2d` (debye-solid 2D phonon continuum), and `add-missing-eR` (standard-model incomplete generation), and `add-missing-spin10` (observer-geometry missing Spin(10)), and `add-quartic` (dulong-petit quartic virial), and `add-missing-e8` (heterotic-e8e8 missing E8), and `add-so16` (heterotic-so32 SO(16)), and `add-chan-paton-16` (type-i Chan-Paton SO(16)) are not knobs |
| `sweep <theory> <knob> <v,v,…>` | evaluate many values; report changed claims |
| `branch` / `checkout` | snapshot / restore knob state |
| `compare` / `design` | discriminating claims; rank theory pairs |
| `sensitivity` | perturb one knob, count flips |
| `review <claim>` | raise semantic assurance from a trusted dossier (never Canonical) |
| `inspect <axis> <value>` | inverse query over `trust`, `class`, knob `origin`, knowledge `gap`, or projected `judgment` (`statistical-computed`, `empirical-excluded`, `logical-proved`, …). Super-K is `empirical-excluded`; the PDG GQW cell is `statistical-computed`; `logical-proved` is empty until a dual-checked receipt exists |
| `formalize <claim>` | emit the catalog encoding as untrusted bytes (not a receipt) |
| `reproduce <claim>` | remint a stored receipt in-process; **not P4** |
| `gaps` | rebuild the knowledge-gap graph from live verdicts and lemma edges (not deserialized). Failing evaluations are not listed as missing theorems. Overlap without containment is `insufficient-precision`. coNP-complete search is `computationally-intractable`, not Rice. An empirical prediction with no registered dataset is `missing-dataset`. Super-K `p→e+π0` is a Dataset; `gut.proton-lifetime-sk` is decided (excluded / compatible), not that hole |
| `enclose <claim>` | independently parse a `CertifiedNumeric` enclosure as `Ratio` and store a content-addressed NumericCertificate. Succeeds on the four P3N cells; refuses unique-vacuum, Super-K, GQW NLL, and Poincaré. Not a kernel receipt, not Canonical, not P4; P3N count stays 4. Restore rebuilds from live overlay strings |
| `cite <claim>` | independently rebuild a live `SourceRecord` (PDG/Super-K datasets or catalog dossiers). Not P3S, not a kernel receipt, not Canonical, not P4. Unique-vacuum and GUT-scale 3/8 refuse (no precise source artifact). Restore rebuilds from live fields |
| `encode <theory>` | independently parse, round-trip, and reconstruct a live theory IR package (`combinational-circuit`, `klein-gordon`, `wilson-u1`, `wilson-su2`, `wilson-su3`, `ohm-circuit`, `bell-test`, `newtonian-gravity`, `linear-medium`, `maxwell-vacuum`, `ideal-gas`, `landauer-engine`, `dirac-fermion`, `general-relativity`, `special-relativity`, `planck`, `de-rham`, `turing-machine`, `olbers-static`, `su5-gut`, `debye-solid`, `standard-model`, `observer-geometry`, `dulong-petit`, `heterotic-e8e8`, `heterotic-so32`, `type-i`). A package with `lean_ref` must bind the catalog identity tree (de-rham coboundary, special-relativity interval); the live SR package also carries the composition and mass-shell trees. Encode lists each bound identity by claim id. Token packages skip. A catalog type without the tree, or a Physlib pointer that is not a catalog type, fails closed. Stores a content-addressed EncodingPackage. Refuses theories with no package. Not P3S, not a kernel receipt, not Canonical, not P4. Restore rebuilds from the live package |
| `judge <claim>` | independently rebuild `Judgment::from_lab` from live evaluator axes and receipts. Stores a content-addressed JudgmentProjection. Unique-vacuum is heuristic failed (JSON cannot mint `logical proved`). Super-K is empirical excluded. GQW NLL is statistical computed. Not Canonical, not P4. Restore rebuilds from live `from_lab` |
| `loop` | one research cycle: observe → hypothesize (chosen/fitted knobs and IR package forks; measured frozen) → prove → falsify → enclose → cite → encode → judge → replicate → design → audit → review |
| `audit` | red-team corpus must not promote |
| `experiments` | list the available experiments |
| `experiment <id>` | canonical experiment (fresh defaults) |
| `score <theory>` | grade a theory against the empirical-target fixture |
| `journal` | dump JSONL |
| `replay <path>` | replay a recorded JSONL journal and verify it reproduces |

CLI tokens map 1:1 onto `physis_agent::Command`.

`--role explorer|formalizer|proof-searcher|falsifier|reviewer|auditor|replication-agent|empirical-analyst|numerical-verifier|provenance-auditor|encoding-auditor|judge`
gates which ops `exec` will dispatch. Named roles may observe (including `hypothesize`); each may
run one kind of untrusted work. A proof-searcher cannot remint a receipt
it requested; that is `replication-agent` (still not P4). An explorer
cannot score the empirical target; that is `empirical-analyst`. A
proof-searcher cannot independently parse a Ratio enclosure; that is
`numerical-verifier` (not a kernel receipt, not P4). A reviewer cannot
independently rehash a `SourceRecord`; that is `provenance-auditor`
(not P3S). A reviewer cannot independently round-trip a live theory IR
package; that is `encoding-auditor` (not P3S). An explorer cannot
independently rebuild a `from_lab` judgment; that is `judge` (JSON
cannot mint `logical proved`). None of
them mint `Verified` — `prove`
still goes through `physis_verifier::verify`. `loop` and `replay` stay
lab-only. `--budget prove=N,review=N,set=N` is a research cap on the
lab, not a proof. Journal restore reconstitutes as the lab, then the
live command is role-gated.

`exec` then checks trust: `reproduce` and the loop's review step require
a dual-checked receipt (P3F). Standalone `review` is encoding-axis and
does not. Observation is free. A trust refusal does not spend budget.

## Responses

`Response::Ok { text, report, diffs }` or `Response::Err { message }`.

Text is for humans and for agents that just want a buffer. `report` / `diffs` are structured.

The CLI `--json` flag emits the whole `Response` as JSON (status, text, and the
structured `report`/`diffs`), so an agent gets typed matrices and verdict diffs
rather than parsing prose. Example: `physis --json set type-iib total_dim 9`.

## Journal events

- `boot` — theory ids at lab creation
- `set-knob` — from, to, diffs (kind plus optional scientific axes)
- `run` — holds/fails/other counts
- `experiment` — experiment id
- `prove` — claim id, challenge hash, statement hash; restore remints
  only when those hashes match the live FormalClaim
- `review` — claim id, evidence hash, statement hash; restore remints
  only when the statement hash is the live identity
- `loop` — cycle summary (inner prove/review events are authoritative)
- `evidence` — claim id, graph hash; restore rebuilds the DAG from live
  evaluations. The recorded hash is not deserialized as authority and is
  not Canonical or P4. `replay_journal` ignores this event (it still
  certifies only `set-knob`)
- `enclose` — claim id, certificate hash; restore rebuilds the
  NumericCertificate from live `CertifiedNumeric` overlay strings. The
  recorded hash is not deserialized as authority, is not a kernel
  receipt, and is not Canonical or P4. `replay_journal` ignores this
  event (it still certifies only `set-knob`)
- `cite` — claim id, source hash; restore rebuilds the Source node from
  live dataset or dossier `SourceRecord` fields. The recorded hash is
  not deserialized as authority, is not P3S, and is not Canonical or P4.
  `replay_journal` ignores this event (it still certifies only `set-knob`)
- `encode` — theory id, package hash; restore rebuilds the EncodingPackage
  from the live IR package (parse / round-trip / reconstruct). The
  recorded hash is not deserialized as authority, is not P3S, and is
  not Canonical or P4. `replay_journal` ignores this event (it still
  certifies only `set-knob`)
- `judge` — claim id, projection hash; restore rebuilds the
  JudgmentProjection from live `Judgment::from_lab`. The recorded hash
  is not deserialized as authority and cannot mint `logical proved`.
  `replay_journal` ignores this event (it still certifies only `set-knob`)

Append-only. Optional file backend (`Journal::file`).

## Long-horizon use

Agents should persist the journal, not the vibes. A later agent must be able to replay “what was tried” from JSONL without the original session.

Replay of `set` events onto a fresh `Lab::standard()` is implemented (M1.1).
`replay_journal` re-applies every recorded `set-knob`, recomputes the verdict
diffs, and checks them against the recorded diffs. Kind triples always
compare. Derivation / empirical / judgment strings compare only when the
journal record carries them, so a pre-axis JSONL remains faithful. A
faithful replay is a
mechanical proof that the session reproduces; a mismatch (or a failed turn)
proves the journal or the encoding drifted, and the CLI exits non-zero.

Journals must be **coherent sessions**: the diffs of a `set` are computed
against the state left by the previous `set`. The one-shot CLI evaluates each
line against fresh defaults, so to persist a real session across process runs,
pass `--journal <file.jsonl>`: the lab loads the file and *restores* prior state
(`Lab::restore_from_journal`) before the new turn, so the accumulated file
replays faithfully. Restore remints prove/review of recorded identities and
rebuilds evidence graphs, numeric certificates, source records,
encoding packages, and judgment projections from live
evaluations and packages;
it does not deserialize a
`Verified`, a semantic tag, a `graph_hash`, a `certificate_hash`, a
`source_hash`, a `package_hash`, or a `projection_hash` as the artifact. `physis replay`
still certifies only `set-knob` diffs.

Timestamps (`t`) are Unix milliseconds as `u64` — 128-bit integers do not
survive serde's internally tagged round-trip, which would silently drop every
event on reload.

## Non-goals

- Natural language as a command
- Unbounded Python in the loop
- Networked multi-agent consensus
