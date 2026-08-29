# M1 — String lab, for real

Goal: make the string-critique experiment less of a skeleton and more of a place an agent can live for a long time.

## Work

1. **Journal replay.** ✅ *Done (M1.1).* `Journal` `set-knob` events applied to a fresh `Lab::standard()` reproduce the same diffs. `physis_agent::replay::replay_journal` recomputes and verifies; `physis replay <file.jsonl>` exits non-zero on mismatch. Round-trip and tamper-detection tests in `crates/physis-agent/src/replay.rs`.
2. **More constructions as first-class:** ✅ *Done (M1.2).* Type I, Type IIA, heterotic SO(32), and M-theory are now constructed (`StringTheory::type_i/type_iia/heterotic_so32/m_theory`) and registered in both `Lab::standard()` and the `string-critique` matrix (ten objects total). Tests in `crates/physis-theory/src/strings.rs` pin their distinctive cells (SO(32) embeds SM; Type IIA/M gauge undecidable; M-theory critical dim 11; all default string knobs fail uniqueness).
3. **Anomaly cancellation as a claim.** ✅ *Done (M1.3).* `consistency.anomaly-cancellation` is a matrix row. The Green–Schwarz condition is a mechanical predicate on the gauge group (`GaugeGroup::gs_anomaly_free_10d`, backed by `GaugeGroup::dimension`): it holds for exactly SO(32) and E₈×E₈ (dimension 496) and fails for a fake `SU(3)`/`E8`/SM choice — Green–Schwarz is the reason, not a menu. Encoded as `EncodedFact`; a typed anomaly polynomial is left for later. Knob-sensitive (off critical dimension → `undecidable`).
4. **Moduli as knobs.** ✅ *Done (M1.4).* Added `dilaton` (g_s = e^φ) and heuristic moduli counts `h11` (Kähler) / `h21` (complex structure). `predictivity.unique-vacuum` now depends on `flux_bits × (h11 + h21)` (zero flux **or** zero moduli restores uniqueness), and `empirical.hidden-extra-dims` depends on the effective radius `compact_radius_planck · √g_s`, so the dilaton and the Kähler volume can both expose extra dimensions. Knob-diff tests in `crates/physis-theory/src/strings.rs`.
5. **Observer-geometry: kill the magic 14 or justify it.** ✅ *Done (M1.5).* The total dimension is now composed: `total_dim = observed_dim + fibre_dim`, with `fibre_dim` a knob (default 10). The 10 is justified by a toy constraint — Spin(10) acts on a 10-dimensional space, so a smaller fibre cannot host the conjectured gauge group. Setting `fibre_dim < 10` with `derive_gauge=true` flips `empirical.sm-gauge` to `fails`. So `14 = 4 + 10` is the minimal carrier, not a magic literal. Test: `shrinking_the_fibre_below_ten_starves_the_gauge_assignment`.
6. **Agent session file.** ✅ *Done (M1.6).* `physis --journal path.jsonl` persists across process runs, restoring prior state each run (`Lab::restore_from_journal`) so the accumulated session stays coherent and replayable.

## Tests that must exist before calling M1 done

- ✅ Replay of a recorded session matches live diffs (`replay::tests::recorded_session_replays_faithfully`, `resumed_multi_run_session_replays_faithfully`)
- ✅ Switching `kind` from `type-iib` to `bosonic` flips fermions and tachyon (`strings::tests::switching_kind_to_bosonic_flips_fermions_and_tachyon`)
- ✅ Heterotic anomaly claim holds for E₈×E₈ and SO(32), fails for a fake `SU(3)` kind (`gauge::tests::green_schwarz_solutions_are_exactly_so32_and_e8e8`, `strings::tests::green_schwarz_constructions_cancel_anomalies`)

**M1 status: all work items (1–6) and required tests are complete.** Next: M2 (`plans/003`).

## Not in M1

Computing actual Calabi–Yau Hodge numbers. If an agent needs topology, add a *knob* `h21`, `h11` first (heuristic), not a CY scanner.
