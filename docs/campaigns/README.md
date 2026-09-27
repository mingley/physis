# Recorded campaigns

Acceptance scenarios for R4–R7 (TODO.md), each with a manifest, a
stopped bundle, JSON + Markdown exports, lab-level evidence, and a
note with replay commands. Every campaign stopped cleanly; no
campaign minted anything it should not have.

| Campaign | Bundle result | Lab evidence | Spend |
|---|---|---|---|
| kinematics-1 | 2 proves minted, no-progress stop | 4 refuted + 2 rejected, explained | 2 actions, 2 attempts, ~1.7s |
| gravity-1 | 2 formalize failed+retained (blocked run: 0 attempts) | 3 discriminating claims + dossier | 2 actions, 2 attempts, ~1.3s |
| field-1 | 1 prove minted + 1 blocked, no-progress stop | 6 refuted + 2 rejected, explained | 1 action, 1 attempt, ~1.2s |
| thermal-1 | 3 blocked, 0 attempts, no-progress stop | frozen design + 4 claims + dossier | 0 actions, 0 attempts, ~1.5s |
| sweep-1 | 9 proves minted, no-progress stop | timing evidence for R8 | 9 actions, 9 attempts, ~4.6s |

## Serial baseline for the R8 gate

Totals across the four campaigns: 3 minted receipts, 2 retained
failed attempts, 7 precise blocked items, 10 explained refutations,
4 explained rejections, 10 discriminating claims, 1 frozen design,
2 dataset dossiers. Campaign spend: 5 actions, 5 attempts, ~6s
wall. Refusal is fast and free: two campaigns spent zero actions
because every gap was already blocked or unsupported.

R8 (bounded parallel workers) may proceed only after showing
better useful-results-per-budget than this serial baseline. The
gate was evaluated in `r8-gate.md` and did not clear: serial is
1 result per action at ~2 results/s with ms-scale attempts and a
single-writer bundle, so parallelism cannot win until a
wall-dominated campaign op exists. Reopen criteria are in that
note.
