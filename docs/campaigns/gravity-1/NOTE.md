# Gravity-1: Newton/Soldner vs Schwarzschild on shared datasets

Acceptance campaign for R4–R7 (TODO.md row 2). Question: do Newton
and Schwarzschild GR evaluated against the same deflection and
perihelion datasets discriminate, with measured inputs frozen?

## Result

Two runs, both honest:

- `manifest-blocked.json` (no formalize op): both empirical claims
  are unsupported theorems, so both gaps became blocked
  formalization work items with zero attempts, then a clean
  no-progress stop. The coordinator refused to fake progress.
- `manifest.json` + `bundle/` (formalize op added): 2 formalize
  attempts committed, both failed with "no catalog identity; this is
  not a mint", retained as negative results, then a clean
  no-progress stop. Usage: actions 2/8, attempts 2/16.
- Lab-level discrimination (`compare.txt`): GR holds/compatible vs
  Newton fails/excluded on both `gr.eddington-deflection` and
  `gr.mercury-perihelion`, plus the mirrored half-angle control —
  3 discriminating claims on the same datasets.
- Dataset dossier (`cite.txt`): Sobral 4-inch 1.98″ ± 0.12 p.e.,
  σ = 0.18, public-domain, compiled-in (no download).
- Acceptance: the data discriminates (no inconclusive verdict
  needed); measured inputs stayed frozen (no parameter range can
  touch them; the campaign's only range is the chosen `r_min`
  domain edge); numerical error is bounded at the verdict level
  against fixed datasets, not fitted.

## Serial baseline (R8 gate input)

Useful results: 2 retained failed attempts with precise blockers +
3 discriminating claims + 1 dataset dossier. Campaign spend:
2 actions, 2 attempts, ~1.3s wall. Failed attempts are
first-class results here: they prove the lab will not mint
empirical claims as theorems.

## Replay

```sh
physis research plan docs/campaigns/gravity-1/manifest.json
physis research step docs/campaigns/gravity-1/manifest.json /tmp/g1-replay
# ... repeat step until stop ...
physis research export /tmp/g1-replay markdown
physis compare general-relativity newtonian-gravity
physis cite gr.eddington-deflection
```

Manifest id
`b2d34cc2025807777e2b0fb924210ea166a153eb1af620c8317deee79d78588e`
(formalize run);
`0945ce960a01699c0bd047bc9bc69df5a80e9506460b65112885ad1ab19cffc4`
(blocked run).
