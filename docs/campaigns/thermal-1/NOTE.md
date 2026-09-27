# Thermal-1: Planck vs Rayleigh–Jeans with calibration separated

Acceptance campaign for R4–R7 (TODO.md row 4). Question: does
Planck hold the thermal obligations with calibration separated
from evaluation while Rayleigh–Jeans is retained as the failed
classical alternative?

## Result

- Campaign (`research step` ×6): all three obligations are
  unsupported theorems, so all three gaps became blocked
  formalization work items with zero attempts, then a clean
  no-progress stop. The coordinator spent nothing proving what
  has no catalog obligation.
- Calibration/evaluation split (`manifest.json` data policy):
  calibration is empty, evaluation lists only
  `quinn-martin-1985-sigma`, and the manifest validator refuses
  any input appearing in both. The T⁴ ratio, Wien peak, and
  UV-finiteness are computed independent of the sigma value —
  out-of-calibration predictions, not fits.
- Frozen design (`design.txt`): decisive contrast on
  `thermo.stefan-boltzmann` against the sigma dataset (Gaussian,
  two-sigma + exact-nll rule), RJ fails retained, no downloads,
  fitted/nuisance/covariance/adaptive-use disclosures stated.
- Comparison (`compare.txt`): 4 discriminating claims; RJ keeps
  mode-equipartition and the IR limit while failing finiteness,
  T⁴, and the Wien peak — the catastrophe explained per claim.
- Dataset dossier (`cite.txt`): quinn-martin-1985-sigma.
- Acceptance: calibration separated from evaluation by
  construction; out-of-calibration predictions reproduced
  (Wien peak, T⁴, finiteness with zero dataset inputs);
  failed alternative retained with explanations.

## Serial baseline (R8 gate input)

Useful results: 3 precise blocked items + 1 frozen design +
4 discriminating claims + 1 dataset dossier. Campaign spend:
0 actions, 0 attempts, ~1.5s wall — the cheapest campaign,
because refusal is fast and free.

## Replay

```sh
physis research plan docs/campaigns/thermal-1/manifest.json
physis research step docs/campaigns/thermal-1/manifest.json /tmp/t1-replay
# ... repeat step until stop ...
physis research export /tmp/t1-replay markdown
physis compare planck rayleigh-jeans
physis design planck rayleigh-jeans
physis cite thermo.stefan-boltzmann
```

Manifest id
`cce41e91757818481aa40eea9819df88ce585ad4b760b57832174902885a27ea`.
