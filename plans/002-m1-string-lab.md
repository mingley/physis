# M1 — String lab, for real

Goal: make the string-critique experiment less of a skeleton and more of a place an agent can live for a long time.

## Work

1. **Journal replay.** ✅ *Done (M1.1).* `Journal` `set-knob` events applied to a fresh `Lab::standard()` reproduce the same diffs. `physis_agent::replay::replay_journal` recomputes and verifies; `physis replay <file.jsonl>` exits non-zero on mismatch. Round-trip and tamper-detection tests in `crates/physis-agent/src/replay.rs`.
2. **More constructions as first-class:** Type I, Type IIA, heterotic SO(32), M-theory already exist as `StringKind` — put them in the default lab and matrix.
3. **Anomaly cancellation as a claim.** Heterotic gauge groups are not a menu; Green–Schwarz is the reason. Encode as `EncodedFact` first, then look at a typed polynomial later.
4. **Moduli as knobs.** Dilaton, a few Kähler/complex-structure stand-ins. Hidden-extra-dims and uniqueness should depend on them.
5. **Observer-geometry: kill the magic 14 or justify it.** Either derive a constraint (even a toy one) or rename the default and document it as “unset.”
6. **Agent session file.** ✅ *Done (M1.6).* `physis --journal path.jsonl` persists across process runs, restoring prior state each run (`Lab::restore_from_journal`) so the accumulated session stays coherent and replayable.

## Tests that must exist before calling M1 done

- ✅ Replay of a recorded session matches live diffs (`replay::tests::recorded_session_replays_faithfully`, `resumed_multi_run_session_replays_faithfully`)
- Switching `kind` from `type-iib` to `bosonic` flips fermions and tachyon
- Heterotic anomaly claim holds for E₈×E₈ and SO(32), fails for a fake `SU(3)` kind if we allow that knob

## Not in M1

Computing actual Calabi–Yau Hodge numbers. If an agent needs topology, add a *knob* `h21`, `h11` first (heuristic), not a CY scanner.
