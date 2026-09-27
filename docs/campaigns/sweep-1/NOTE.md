# Sweep-1: all supported theorems, serial timing evidence

Not an acceptance scenario — a measurement run for the R8 gate.
Question: sweep all 9 catalogued exact identities serially and
record the timing.

## Result

- 9 prove attempts, 9 minted, each exactly once, then a clean
  no-progress stop (repeats blocked with unchanged inputs).
- Usage: actions 9/12, attempts 9/16, wall 4570ms/300000.
  Per-step wall ≈ 0.5s including process boot and lab
  construction; per-attempt compute is milliseconds.
- Single-op timings (built debug binary, wall): prove ≈ 0.36s,
  hypothesize (de-rham, 7 candidates) ≈ 0.02s, cite ≈ 0.06s.
  All sub-second; all boot-dominated.

## Reading for R8

Serial useful-results-per-budget: 1 receipt per action at ~2
receipts per wall-second. See `../r8-gate.md` for the gate
evaluation.

## Replay

```sh
physis research plan docs/campaigns/sweep-1/manifest.json
physis research step docs/campaigns/sweep-1/manifest.json /tmp/s1-replay
# ... repeat step until stop (9 mints, then no-progress) ...
```

Manifest id
`0d02156744096a6a909e0e89b1dbb4526601936043c12fe620b3bc34a3a6296f`.
