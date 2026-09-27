# Kinematics-1: Lorentz vs Galilean/truncated boost

Acceptance campaign for R4–R7 (TODO.md row 1). Question: does the
Lorentz encoding keep `sr.invariant-interval` and
`sr.subluminal-composition` holding while Galilean and
truncated-boost candidates fail?

## Result

- Campaign (`research step` ×5): 2 prove attempts minted
  (`sr.subluminal-composition`, `sr.invariant-interval`, dual
  expand-recursive + expand-postfix), then a clean no-progress stop.
  Usage: actions 2/8, attempts 2/16, wall 1721ms/300000,
  candidates 0/16. Both post-proof gaps repeat-blocked with unchanged
  inputs — the coordinator did not re-prove.
- Candidate search (`hypothesize special-relativity`, lab-level, in
  `candidates.txt`): 4 explored, 4 refuted with per-claim verdict
  diffs (Galilean `absolute_time=true` flips all three SR claims;
  `add-binomial-gamma` flips 2; `add-minus-uv` flips composition),
  2 invalid candidates rejected with reasons.
- Acceptance: exact algebra (dual-expander prove) and candidate
  search stayed distinct; every invalid candidate or domain is
  explained, none hidden.

Hypothesize never dispatched *inside* the campaign: the C7.1 selector
routes theorems to prove and reserves hypothesize for open-problem
gaps. That is correct routing, not a gap — the candidate half is
lab-level evidence recorded alongside the bundle.

## Serial baseline (R8 gate input)

Useful results: 2 minted receipts + 4 explained refutations +
2 explained rejections. Campaign spend: 2 actions, 2 attempts,
~1.7s wall. Candidate search: 4 explored (one lab command).

## Replay

```sh
physis research plan docs/campaigns/kinematics-1/manifest.json
physis research step docs/campaigns/kinematics-1/manifest.json /tmp/k1-replay
# ... repeat step until stop ...
physis research status /tmp/k1-replay
physis research export /tmp/k1-replay markdown
physis hypothesize special-relativity
```

`bundle/` is the original stopped bundle: `research status` and
`research export` read it without a lab. Manifest id
`827e2dd507c5f231205fcb8de6448b222bf0af30ff22ecc034460348bcd6a678`.
