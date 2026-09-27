# Field-1: DEC disk baseline vs orientation/boundary mutations

Acceptance campaign for R4–R7 (TODO.md row 3). Question: does the
disk DEC encoding keep `dec.d-squared-zero` and
`dec.closed-equals-exact` holding while orientation and boundary
mutations fail with the changed encoding identified?

## Result

- Campaign (`research step` ×5): 1 prove minted
  (`dec.d-squared-zero`, dual expanders);
  `dec.closed-equals-exact` is an unsupported theorem, so it became
  a blocked formalization work item with no attempt. Clean
  no-progress stop. Usage: actions 1/8, attempts 1/16.
- Candidate search (`candidates.txt`): 7 explored, 6 refuted, each
  naming the changed encoding — `add-sign-flip` (orientation)
  flips d-squared-zero; `shape: disk → torus/circle/klein`
  (boundary/topology) flips closed-equals-exact; combined
  `add-sign-flip+add-down-laplacian` flips both; 2 invalid
  candidates rejected with reasons.
- Falsifier (`falsify.txt`): no counterexample to d-squared-zero in
  local knob probes — the baseline stands where the mutations fall.
- Acceptance: minimal counterexamples produced (single-flip rows),
  each identifying the changed assumption/encoding, not just a
  failed boolean.

## Serial baseline (R8 gate input)

Useful results: 1 minted receipt + 1 precise blocked item +
6 explained refutations + 2 explained rejections. Campaign spend:
1 action, 1 attempt, ~1.2s wall.

## Replay

```sh
physis research plan docs/campaigns/field-1/manifest.json
physis research step docs/campaigns/field-1/manifest.json /tmp/f1-replay
# ... repeat step until stop ...
physis research export /tmp/f1-replay markdown
physis hypothesize de-rham
physis falsify dec.d-squared-zero
```

Manifest id
`541291ac65bca0d4099900654b93df9f78c44873b880f8d37594a467fbc58a21`.
