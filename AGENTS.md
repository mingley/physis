# Standing orders for agents

You are operating inside **physis**, a typed laboratory. You do not have opinions that bypass knobs, claims, journals, and verification receipts.

## What you may do

1. Read `specs/`, `plans/`, `docs/`, and crate-level rustdoc.
2. Run the CLI (`physis layers|theories|knobs|run|set|experiment|journal|epistemics|why|evidence|prove|formalize|reproduce|gaps|enclose|cite|constant|encode|judge|falsify|hypothesize|sweep|branch|audit|review|inspect|loop`). `--role explorer` (and the other named roles) can observe but cannot mint. `proof-searcher` cannot remint; that is `replication-agent` (still not P4). `explorer` cannot `score`; that is `empirical-analyst`. `proof-searcher` cannot independently parse a `CertifiedNumeric` enclosure; that is `numerical-verifier` (`physis enclose`; not a kernel receipt, not Canonical, not P4; `inspect trust P3N` stays count 4). `--journal` restore rebuilds `NumericCertificate` nodes from live overlay strings; a recorded `certificate_hash` is not deserialized. A reviewer cannot independently rehash a `SourceRecord`; that is `provenance-auditor` (`physis cite`; datasets and catalog dossiers; not P3S, not Canonical, not P4). A reviewer cannot independently rebuild a versioned Constant; that is `provenance-auditor` (`physis constant [name]`; omitted name rebuilds the full LEDGER; SI 2019 Ratio/SciExact, CODATA G Interval, CODATA 2018 mu0 Interval 1.25663706212(19)e-6 N A^{-2} (JPCRD table XXXI UNIVERSAL, measured after SI 2019, not exact 4pi x 10^{-7}, not Y0, not P3N), CODATA 2018 epsilon0 Interval 8.8541878128(13)e-12 F m^{-1} (JPCRD table XXXI UNIVERSAL, 1/(mu0 c^2) after SI 2019, not exact, not Y0, not P3N), CODATA 2018 Z0 Interval 376.730313668(57) ohm (JPCRD table XXXI UNIVERSAL, mu0 c after SI 2019, not exact, not Y0, not P3N), CODATA 2018 alpha Interval 7.2973525693(11)e-3 (JPCRD table XXXI ATOMIC AND NUCLEAR, not an SI defining Ratio, not P3N), CODATA 2018 inv_alpha Interval 137.035999084(21) (JPCRD table XXXI ATOMIC AND NUCLEAR, not an SI defining Ratio, not P3N), CODATA 2018 cRinf Interval 3.2898419602508(64)e15 Hz (JPCRD table XXXI ATOMIC AND NUCLEAR, not an SI defining Ratio, not P3N), CODATA 2018 hcRinf Interval 2.1798723611035(42)e-18 J (JPCRD table XXXI ATOMIC AND NUCLEAR, not an SI defining Ratio, not the eV conversion, not P3N), CODATA 2018 Rinf Interval 10973731.568160(21) m^{-1} (JPCRD table XXXI ATOMIC AND NUCLEAR, not an SI defining Ratio, not P3N), CODATA 2018 a0 Interval 5.29177210903(80)e-11 m (JPCRD table XXXI ATOMIC AND NUCLEAR, not an SI defining Ratio, not P3N), CODATA 2018 Eh Interval 4.3597447222071(85)e-18 J (JPCRD table XXXI ATOMIC AND NUCLEAR, not an SI defining Ratio, not the eV conversion, not P3N), CODATA 2018 me_mmu Interval 4.83633169(11)e-3 (JPCRD table XXXI Electron, e-, not an SI defining Ratio, not electron mass, not P3N), CODATA 2018 me_mp Interval 5.44617021487(33)e-4 (JPCRD table XXXI Electron, e-, not an SI defining Ratio, not electron mass, not P3N), CODATA 2018 me_mn Interval 5.4386734424(26)e-4 (JPCRD table XXXI Electron, e-, not an SI defining Ratio, not electron mass, not P3N), CODATA 2018 me_md Interval 2.724437107462(96)e-4 (JPCRD table XXXI Electron, e-, not an SI defining Ratio, not electron mass, not P3N), CODATA 2018 me_mt Interval 1.819200062251(90)e-4 (JPCRD table XXXI Electron, e-, not an SI defining Ratio, not electron mass, not P3N), CODATA 2018 me_mh Interval 1.819543074573(79)e-4 (JPCRD table XXXI Electron, e-, not an SI defining Ratio, not electron mass, not P3N), CODATA 2018 me_malpha Interval 1.370933554787(45)e-4 (JPCRD table XXXI Electron, e-, not an SI defining Ratio, not electron mass, not P3N), CODATA 2018 e_me Interval -1.75882001076(53)e11 C kg^{-1} (JPCRD table XXXI Electron, e-, not an SI defining Ratio, not electron mass, not P3N), CODATA 2018 M_e Interval 5.4857990888(17)e-7 kg mol^{-1} (JPCRD table XXXI Electron, e-, not an SI defining Ratio, not electron mass, not the mass-in-u row, not P3N), CODATA 2018 lambdabar_C Interval 3.8615926796(12)e-13 m (JPCRD table XXXI Electron, e-, not an SI defining Ratio, not a certificate of alpha a0, not P3N), CODATA 2018 lambda_C Interval 2.42631023867(73)e-12 m (JPCRD table XXXI Electron, e-, not an SI defining Ratio, not a certificate of 2pi lambdabar_C, not P3N), CODATA 2018 re Interval 2.8179403262(13)e-15 m (JPCRD table XXXI Electron, e-, not an SI defining Ratio, not a certificate of alpha^2 a0, Thomson skipped because pi, not P3N), CODATA 2018 mu_e Interval -9.2847647043(28)e-24 J T^{-1} (JPCRD table XXXI Electron, e-, not an SI defining Ratio, not P3N), CODATA 2018 mu_e_muB Interval -1.00115965218128(18) (JPCRD table XXXI Electron, e-, not an SI defining Ratio, not the g-factor, not the anomaly, not P3N), CODATA 2018 mu_e_muN Interval -1838.28197188(11) (JPCRD table XXXI Electron, e-, not an SI defining Ratio, not the g-factor, not the anomaly, not P3N), CODATA 2018 ae Interval 1.15965218128(18)e-3 (JPCRD table XXXI Electron, e-, not an SI defining Ratio, not the g-factor, not P3N), CODATA 2018 ge Interval -2.00231930436256(35) (JPCRD table XXXI Electron, e-, not an SI defining Ratio, not the anomaly, not P3N), CODATA 2018 mu_e_mmu Interval 206.7669883(46) (JPCRD table XXXI Electron, e-, not an SI defining Ratio, not the electron-muon mass ratio, not P3N), CODATA 2018 mu_e_mup Interval -658.21068789(20) (JPCRD table XXXI Electron, e-, not an SI defining Ratio, not the electron-proton mass ratio, not P3N), CODATA 2018 mu_e_mu0p Interval -658.2275971(72) (JPCRD table XXXI Electron, e-, not an SI defining Ratio, not the free-proton moment ratio, not vacuum permeability, not P3N), CODATA 2018 mu_e_mun Interval 960.92050(23) (JPCRD table XXXI Electron, e-, not an SI defining Ratio, not the electron-neutron mass ratio, not P3N), CODATA 2018 mu_e_mud Interval -2143.9234915(56) (JPCRD table XXXI Electron, e-, not an SI defining Ratio, not the electron-deuteron mass ratio, not P3N), CODATA 2018 mu_e_mu0h Interval 864.058257(10) (JPCRD table XXXI Electron, e-, not an SI defining Ratio, not the electron-helion mass ratio, not the shielded-proton moment ratio, not vacuum permeability, not P3N), CODATA 2018 m_mu Interval 1.883531627(42)e-28 kg (JPCRD table XXXI Muon, mu-, not an SI defining Ratio, not the electron-muon mass ratio, not the u-row, not P3N), CODATA 2018 m_mu_u Interval 0.1134289259(25) u (JPCRD table XXXI Muon, mu-, not an SI defining Ratio, not the kg hull, not electron molar mass, not P3N), CODATA 2018 m_mu_c2 Interval 1.692833804(38)e-11 J (JPCRD table XXXI Muon, mu-, not an SI defining Ratio, not the kg hull, not the u-row, not the MeV conversion, not hcRinf, not P3N), CODATA 2018 m_mu_c2_MeV Interval 105.6583755(23) MeV (JPCRD table XXXI Muon, mu-, not an SI defining Ratio, not the joule hull, not the exact electronvolt, not Eh, not P3N), CODATA 2018 mmu_me Interval 206.7682830(46) (JPCRD table XXXI Muon, mu-, not an SI defining Ratio, not me_mmu, not P3N), CODATA 2018 mmu_mp Interval 0.1126095264(25) (JPCRD table XXXI Muon, mu-, not an SI defining Ratio, not me_mp, not P3N; muon-tau skipped as PDG reprint), CODATA 2018 mmu_mn Interval 0.1124545170(25) (JPCRD table XXXI Muon, mu-, not an SI defining Ratio, not me_mn, not P3N), CODATA 2018 M_mu Interval 1.134289259(25)e-4 kg mol^{-1} (JPCRD table XXXI Muon, mu-, not an SI defining Ratio, not the u-row, not electron molar mass, not P3N), CODATA 2018 lambda_C_mu Interval 1.173444110(26)e-14 m (JPCRD table XXXI Muon, mu-, not an SI defining Ratio, not electron Compton, reduced muon Compton skipped because hbar, not P3N), CODATA 2018 mu_mu Interval -4.49044830(10)e-26 J T^{-1} (JPCRD table XXXI Muon, mu-, not an SI defining Ratio, not electron magnetic moment, not the electron-muon magnetic-moment ratio, not vacuum permeability, not P3N), CODATA 2018 mu_mu_muB Interval -4.84197047(11)e-3 (JPCRD table XXXI Muon, mu-, not an SI defining Ratio, not electron Bohr-magneton ratio, not the muon magnetic moment, not the g-factor, not the anomaly, not P3N), CODATA 2018 m_p Interval 1.67262192369(51)e-27 kg (JPCRD table XXXI Proton, p, not an SI defining Ratio, not electron mass, not P3N), IAU 2012 au exact Ratio 149597870700 m and SI 2019 eV exact Ratio 1.602176634e-19 J (BIPM table 8), and IAU 2015 GM_sun exact Ratio 1.3271244e20 m3 s-2, R_sun exact Ratio 695700000 m, and L_sun exact Ratio 3.828e26 W (AJ 152 table 1, conversion rulers not measured solar properties); physis_model Qty floats lockstep the ledger: c, au, GM_sun, R_sun, and L_sun via integer to_f64, e/k/eV via IEEE rounding of the SI decimal not reduced Ratio::to_f64, h via SciExact to_f64; G Qty is the CODATA centre inside the hull; mu0 Qty is the CODATA centre inside the hull; epsilon0 Qty is the CODATA centre inside the hull; Z0 Qty is the CODATA centre inside the hull; alpha Qty is the CODATA centre inside the hull; inv_alpha Qty is the CODATA centre inside the hull; cRinf Qty is the CODATA centre inside the hull; hcRinf Qty is the CODATA centre inside the hull; Rinf Qty is the CODATA centre inside the hull; a0 Qty is the CODATA centre inside the hull; Eh Qty is the CODATA centre inside the hull; me_mmu Qty is the CODATA centre inside the hull; me_mp Qty is the CODATA centre inside the hull; me_mn Qty is the CODATA centre inside the hull; me_md Qty is the CODATA centre inside the hull; me_mt Qty is the CODATA centre inside the hull; me_mh Qty is the CODATA centre inside the hull; me_malpha Qty is the CODATA centre inside the hull; e_me Qty is the CODATA centre inside the hull; M_e Qty is the CODATA centre inside the hull; lambdabar_C Qty is the CODATA centre inside the hull; lambda_C Qty is the CODATA centre inside the hull; re Qty is the CODATA centre inside the hull; mu_e Qty is the CODATA centre inside the hull; mu_e_muB Qty is the CODATA centre inside the hull; mu_e_muN Qty is the CODATA centre inside the hull; ae Qty is the CODATA centre inside the hull; ge Qty is the CODATA centre inside the hull; mu_e_mmu Qty is the CODATA centre inside the hull; mu_e_mup Qty is the CODATA centre inside the hull; mu_e_mu0p Qty is the CODATA centre inside the hull; mu_e_mun Qty is the CODATA centre inside the hull; mu_e_mud Qty is the CODATA centre inside the hull; mu_e_mu0h Qty is the CODATA centre inside the hull; m_mu Qty is the CODATA centre inside the hull; m_mu_u Qty is the CODATA centre inside the hull; m_mu_c2 Qty is the CODATA centre inside the hull; m_mu_c2_MeV Qty is the CODATA centre inside the hull; mmu_me Qty is the CODATA centre inside the hull; mmu_mp Qty is the CODATA centre inside the hull; mmu_mn Qty is the CODATA centre inside the hull; M_mu Qty is the CODATA centre inside the hull; lambda_C_mu Qty is the CODATA centre inside the hull; mu_mu Qty is the CODATA centre inside the hull; mu_mu_muB Qty is the CODATA centre inside the hull; m_p Qty is the CODATA centre inside the hull; parsec stays f64 with pi; ħ is not stored; not P3N, not P3S, not Canonical, not P4). `--journal` restore rebuilds `VersionedConstant` nodes from live constructors; a recorded `node_hash` is not deserialized. A reviewer cannot independently round-trip a live theory IR package; that is `encoding-auditor` (`physis encode`; combinational NAND netlist, Klein–Gordon stencil, Wilson U(1)/SU(2)/SU(3) plaquettes, ohm-circuit lumped branches, bell-test singlet ket, newtonian-gravity inverse-square Binet rhs, linear-medium isotropic-linear constitutive law, maxwell-vacuum source-free homogeneous Faraday, ideal-gas Maxwell-Boltzmann statistics, landauer-engine kT ln2 bound, dirac-fermion naive 1D operator, general-relativity Einstein-Hilbert action, special-relativity Lorentz boost plus catalog interval, composition, and mass-shell trees, planck Bose occupation, de-rham discrete coboundary, and turing-machine unrelativized TM, and `olbers-static` inverse-square Euclidean shells, and `su5-gut` complete 5bar + 10, and `debye-solid` 3D ω² continuum, and `standard-model` complete one-generation Weyl, and `observer-geometry` Spin(10) on 10-fibre, and `dulong-petit` harmonic U = 3 N k T, and `heterotic-e8e8` complete E8 x E8, and `heterotic-so32` complete SO(32), and `type-i` Chan-Paton SO(32); a package with lean_ref must bind the catalog identity tree; encode lists each bound identity by claim id, not a kernel proof; not P3S, not Canonical, not P4). An explorer cannot independently rebuild a `from_lab` judgment; that is `judge` (`physis judge`; unique-vacuum stays heuristic failed; JSON cannot mint `logical proved`; not Canonical, not P4). `physis hypothesize [theory]` probes chosen/fitted knobs for scientific-axis diffs and restores; measured knobs (generations, observed_dim) are frozen — they are not hypotheses about the encoding. It also applies IR package mutations (`Theory::structural_mutations`): `combinational-circuit` is a NAND netlist, and `add-feedback` and `add-contention` are not knobs; `klein-gordon` is a nearest-neighbour Laplacian with a quadratic potential, and `add-next-nearest` and `add-quartic` are not knobs; `wilson-u1`, `wilson-su2`, and `wilson-su3` are unimproved 1×1 Wilson stencils, and `add-rectangle` and `add-higgs` are not knobs; `ohm-circuit` is a lumped Kirchhoff netlist, and `add-tline` and `add-flux` are not knobs; `bell-test` is a two-qubit singlet ket, and `add-product` and `add-pr-box` are not knobs; `newtonian-gravity` is an inverse-square Binet rhs, and `add-schwarzschild` and `add-yukawa` are not knobs; `linear-medium` is an isotropic-linear constitutive law, and `add-tellegen` and `add-chiral` are not knobs; `maxwell-vacuum` is source-free homogeneous Faraday, and `add-monopole` and `add-proca` are not knobs; `ideal-gas` is Maxwell-Boltzmann statistics, and `add-bose` and `add-fermi` are not knobs; `landauer-engine` is a kT ln2 Landauer bound, and `add-kt` and `add-demon` are not knobs; `dirac-fermion` is a naive 1D Dirac operator, and `add-wilson` and `add-next-nearest` are not knobs; `general-relativity` is an Einstein-Hilbert action, and `add-r-squared` and `add-brans-dicke` are not knobs; `special-relativity` is a Lorentz boost plus the catalog interval, composition, and mass-shell trees, and `add-binomial-gamma` and `add-minus-uv` are not knobs; `planck` is a Planck-Bose occupation, and `add-wien` and `add-zero-point` are not knobs; `de-rham` is a discrete coboundary identity, and `add-sign-flip` and `add-down-laplacian` are not knobs; `turing-machine` is an unrelativized Turing machine, and `add-oracle` is not a knob; `olbers-static` is inverse-square Euclidean shells, and `add-tired-light` is not a knob; `su5-gut` is a complete 5bar + 10, and `add-missing-10` is not a knob; `debye-solid` is a 3D ω² continuum, and `add-2d` is not a knob; `standard-model` is a complete one-generation Weyl content, and `add-missing-eR` is not a knob; `observer-geometry` is Spin(10) on a 10-fibre, and `add-missing-spin10` is not a knob; `dulong-petit` is harmonic U = 3 N k T, and `add-quartic` is not a knob; `heterotic-e8e8` is complete E8 x E8, and `add-missing-e8` is not a knob; `heterotic-so32` is complete SO(32), and `add-so16` is not a knob; `type-i` is Chan-Paton SO(32), and `add-chan-paton-16` is not a knob. Mutants are not installed. `physis evidence <claim>` groups live evaluations by statement hash: a shared slug is not one FormalClaim; confidence is a derived TrustProfile, not a numeric score. It inserts a content-addressed Evidence graph (Statement and Evaluation nodes) and journals `JournalEvent::Evidence`. `--journal` restore rebuilds that DAG from live evaluations; a recorded `graph_hash` is not deserialized as the graph and is not Canonical or P4. `physis replay` still certifies only `set-knob`. `--budget prove=N,review=N,set=N` is a research cap, not a proof. `physis reproduce` remints a stored receipt in-process and is **not** P4; it requires P3F. `physis gaps` rebuilds the knowledge-gap graph from live verdicts and declared lemma edges. A failing evaluation is not a missing theorem. Overlap without containment on an empirical receipt is insufficient-precision, not compatible. A lattice too coarse to certify a numerical order (`field.second-order-accurate`) is also insufficient-precision, not a failed theorem. coNP-complete search is computationally-intractable, not logically undecidable. An empirical prediction with no registered dataset is missing-dataset. Super-K `p→e+π0` is a Dataset (Takenaka et al., Phys. Rev. D 102, 112011); `gut.proton-lifetime-sk` is the dim-6 `M_GUT^4` comparison (excluded for minimal SU(5), compatible for MSSM dim-6), not P3N and not a dimension-5 operator. PDG `sin²θ_W(M_Z)` is a Gaussian; `gut.weinberg-angle-mz-interval` is `statistical computed` from an exact NLL of the algebraic π-free GQW Ratio centre rounded to the PDG `10^{-5}` scale, still not P3N. The research loop rebuilds the versioned constants ledger after cite (not P3N); it will not raise P3S on an unproved identity; standalone `review` is still encoding-axis.
3. Turn knobs through `Lab::set_knob` / `physis set`. Illegal values must be rejected by domain checks.
4. Add tests that demonstrate a knob → verdict diff.
5. Add a new theory as a `Theory` impl with its own knobs and claims. Do not special-case it in the CLI.
6. Add a new scientific domain as layers + theories, following `specs/007-reuse-domains.md`.
7. Propose candidate theorems, counterexamples, and encodings. You may not mint `Verified<T>`. Call `physis_verifier::verify` with a trusted `Challenge`; you still cannot deserialize a `Verified` from JSON.

## What you must not do

- Do not treat `VerdictKind::Holds` as “true of nature”. Verdicts are internal to the encoding.
- Do not treat `DerivationAssurance::Executed` as a kernel proof. It means the evaluator ran.
- Do not invent a `theorem` tag or a `MachineProved` enum variant. Only `physis-verifier` can mint `Verified<T>`, and that mint is crate-private.
- Do not silently upgrade `Asserted` (conjecture/heuristic/open) to `Executed`, or `Unreviewed` to a stronger semantic tag, without encoding an actual check. `physis review` is allowed only because it *runs* provenance, a second encoding, and the red-team corpus. There is no `SemanticAssurance::Canonical` variant.
- Do not implement Geometric Unity, or claim to. `observer-geometry` is a scaffold.
- Do not declare string theory false because `predictivity.unique-vacuum` fails. That cell *is* the landscape objection, labelled heuristic. The four encodings name distinct regimes (flux/moduli landscape, observer-geometry program axiom, classical Einstein–Hilbert plus Λ, SM Higgs vacuum); they are not one FormalClaim and not Canonical.
- Do not add `unsafe`. The workspace is `#![forbid(unsafe_code)]`.
- Do not add FFI or non-Rust **physics engines**. Unverified external computation is never authoritative. External formal systems may produce proof *artifacts* only through isolated certificate-checking boundaries (`specs/020-proof-carrying.md`). `physis_verifier::verify` on `LeanSource` runs the Lean kernel and nanoda on a `lean4export`; missing tools or export-only bytes are `LeanPipelineNotWired` (no mint). `ExactCertificate` is a dual-expanded identity, not a kernel proof.
- Do not paper over type errors with `f64` bags. If two quantities should not add, they must not share a type.
- Do not invent particles, groups, or critical dimensions. Cite, or mark `OpenProblem`.
- Do not rewrite history in the journal. Append only.
- Do not treat generated prose as scientific authority.

## Protocol

Every interesting action is:

1. observe (`run`, `knobs`, `layers`, `why`)
2. set a knob
3. read the verdict diff (kind **and** derivation / empirical / judgment when those axes move)
4. record (the lab journals this)

If a change produces **no** scientific-axis diff and you expected one, either the claim is missing or the mapping is a bug. Add a test. `set klein-gordon spacing 100` must show `holds → undecidable` **and** `not-applicable → inconclusive` / `logical undetermined → numeric unresolved`, not a failed theorem.

## Assurance axes

Every claim answers four questions. They are different Rust types.

| Axis | Meaning |
|---|---|
| `ClaimClass` | mathematical / model-internal / phenomenological / empirical-prediction / measurement / conjecture / heuristic / open-problem |
| `DerivationAssurance` | asserted / executed / cross-checked / certified-numeric. **No MachineProved variant.** |
| `EmpiricalStatus` | not-applicable / untested / compatible / supported / tension / excluded / inconclusive |
| `SemanticAssurance` | unreviewed / source-anchored / independently-encoded / adversarially-reviewed |

A theory can `Executed`-derive a prediction that nature `Excluded`. That is a feature.

`physis why <claim>` prints assumptions, a typed `judgment` (evaluator
`holds` is `logical undetermined` until a dual-checked receipt exists;
`CertifiedNumeric` Holds is `numeric certified` with a display enclosure,
not a kernel proof; a coarse lattice is `numeric unresolved`; `Judgment`
has no `Deserialize`, so JSON cannot mint `logical proved`;
`LogicalJudgment` has no public `Proved` constructor;
`NumericJudgment` has no public `Certified` constructor;
`EmpiricalJudgment` has no public `Compatible` constructor;
`HeuristicJudgment` has no public `Suggestive` constructor;
`StatisticalJudgment` has no public `Computed` constructor;
`from_lab` projects `statistical computed` only from an exact Gaussian
NLL overlay on an empirical or measurement claim (PDG `sin²θ_W(M_Z)`);
Super-K interval-subset stays empirical;
there is no `SemanticAssurance::Canonical` variant; P3S is a review-store
tag, not an evaluator field; `Verdict` has no `Deserialize`, so JSON
cannot mint a `certified-numeric` overlay; `Claim` derivation / empirical /
semantic fields are private; `Verdict` derivation / empirical / semantic /
enclosure / NLL fields are private, so a public assignment cannot mint
`CertifiedNumeric` or a Gaussian NLL),
a derived `trust` profile (P3F only from a verifier receipt; P3S from
encoding review of the live `statement_hash`, not the slug; P4 is not assigned from an in-process remint), the
statement hash (which commits to quantifiers, units, constants, boundary
conditions, conventions, theory version, definitions, datasets, and
formal-library identity; the lab slug is stable; P3F is a receipt of
that hash, not of the slug; P3S is a review of that hash, not of the
slug), non-default identity
fields, and `kernel proof: none` until `physis prove` records a
receipt. A P3F result that is still `Unreviewed` prints a danger note.
When Lean 4.34 and `lean4export` are on PATH (`LEAN4EXPORT`), `prove` of a
catalog FormalClaim identity mints `FormalBackend::Lean4` (`lean-kernel` + `nanoda`).
The catalog obligation is that identity, not the slug: a matching id with
different commitments, the encoding-wide domain placeholder, or only
`encoding-is-the-model` cannot borrow ExactIdentity or Physlib. `FormalClaim`
is from_claim-only (private fields, no Deserialize; the hash is recomputed
from the live sentence). `Claim::statement_hash` is derived from the live
sentence (`Claim.statement`, id, class, layer, assumptions, domain, and
commitments are private; lemma edges stay public; no stored hash
field, no Deserialize). `Challenge`
is generate-only (private fields, no Deserialize). Otherwise it mints `ExactCertificate` (dual expanders). Neither is an enum
an agent can set. `physis review` overlays a justified semantic tag from a
trusted dossier bound to the live FormalClaim; journal restore re-runs
review of that identity rather than deserializing the tag, and only when
the journaled `statement_hash` is the live FormalClaim. A slug-only
review line is not P3S. A review of an
older identity that kept the slug is not P3S. `physis inspect trust|class|origin|gap|judgment <value>` inverts those
axes: knobs carry a `ParameterOrigin` so a fitted dilaton is not a derived
prediction, and `inspect origin fitted` lists the knobs that accommodate
rather than derive. `inspect judgment statistical-computed` lists the PDG
GQW NLL cell; `inspect judgment empirical-excluded` lists Super-K (not that
Gaussian); `inspect judgment logical-proved` is empty until a dual-checked
receipt exists. `inspect trust P3N` lists exact-`Ratio` cancellations
and the exact hypercharge solve (Standard Model chiral anomalies and
`sm.hypercharge-derivation`) and hydrogen neutrality from `Q = T₃ + Y`
(`empirical.charge-quantization`), plus GUT-scale `sin²θ_W = 3/8`
(`gut.weinberg-angle`). Those P3N cells and the GQW / PDG mixing-angle
siblings name a `DomainOfValidity`. Super-K `p→e+π0` names the dim-6 /
90% CL regime; GUT `Tr Q` stays encoding-wide. Super-K is not P3N. `thermo.high-t-classical` names `T/Θ ≥ 8`; `thermo.debye-t3` names the `Θ/20` probe; `thermo.rj-ir-limit` names `hν = 0.01 kT`; `gauge.exact-area-law-2d` names 2D. On `dulong-petit`, `thermo.dulong-petit` names harmonic `U = 3 N k T`; on `einstein-solid` and `debye-solid` it stays encoding-wide. On `heterotic-e8e8`, `consistency.anomaly-cancellation` names complete `E8 x E8`; on `heterotic-so32` it names complete `SO(32)`; on `type-i` it names complete `Chan-Paton SO(32)`; Type II copies stay encoding-wide. It does not list GUT `Tr Q` (`ΣY` is already
the gravitational anomaly), Georgi–Quinn–Weinberg running at `M_Z`, the
the sourced PDG input-interval GQW enclosure, the 3% heuristic hit, or a kernel proof. `inspect trust P2` lists Hodge Laplacian-versus-`b₁` agreement
(`dec.hodge-harmonic`), not Euler–Poincaré rank-cancellation, not Poincaré,
and not a kernel proof. That P2 cell names discrete combinatorial Hodge
on finite simplicial 1-cochains; Euler–Poincaré and Poincaré stay
encoding-wide. `exec` checks role, then trust, then budget:
`reproduce` and the loop's review step require P3F. Standalone encoding
review does not.

## First lab

The current flagship is `string-critique`. Before proposing that “we were fundamentally wrong about string theory,” you must:

1. Produce a knob path that flips an **executed model-internal** claim (not a heuristic) to `fails` for every viable string construction, **or**
2. Produce a unique-geometry construction whose empirical-contact claims are `executed` model-internal or phenomenological, not `conjecture`/`open-problem`.

Until then, report the matrix and the diffs. That *is* the work. A kernel proof of a catalog identity is `physis prove` when the Lean pipeline is wired, not an enum.

## Style

- Small crates, documented public items, `cargo fmt`, clippy `-D warnings`.
- Tests name the claim they protect (`turning_iib_dimension_flips_critical_claim`).
- Prefer enums over strings, except at the agent protocol boundary (`KnobValue`).
- When you add a knob, add it to specs if it is load-bearing.
