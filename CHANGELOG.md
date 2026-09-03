# Changelog

Every change to `physis` is atomic, committed directly to `main`, agentically
reviewed, and recorded here with its rationale and the verification that backs
it. This log is part of the contract: the process is meant to be as inspectable
as the physics.

Format loosely follows [Keep a Changelog](https://keepachangelog.com/).
The project keeps `unsafe`-free pure Rust and honest epistemic tags.

## [Unreleased]

### Numerics

- **Independent Interval enclose of one-loop inverse-alpha unification.**
  Third vertex of the one-loop triangle: `α_s` and `sin²θ_W` predict
  `α_em⁻¹(M_Z)` as a π-free Ratio (`2π` cancels).
  `gut.inverse-alpha-em-mz-interval` is an EmpiricalPrediction with sourced
  PDG 2024 / 2022 input-σ hulls versus the tight PDG `α_em⁻¹` hull, plus
  exact Gaussian NLL at the PDG `10^{-3}` scale. `physis enclose`
  independently parses the overlay; not CertifiedNumeric, not P3N (count
  stays 4). Minimal SU(5) is excluded; one-loop MSSM is still excluded
  (`α_em` is known far more precisely than `α_s`). The 3% heuristic cell
  can still hold on `gut.coupling-unification`. GQW and coupling-unification
  interval FormalClaim identity and certificate pins unchanged. GUT-scale
  3/8 certificate pin unchanged. Inverse-alpha interval FormalClaim identity
  `cdee77621ca682a565cb75b277f5c583fd6dc179dccd3c523dc3d63375149f8a`.
  Interval certificate
  `166b6853d0b9a21996e896fc7d13e7011309472f9201c21e61938ccf629b569b`.
  Verified: unit tests; inspect judgment statistical-computed count 3;
  inspect gap insufficient-precision after SUSY stays count 2; role gates.

- **Independent Interval enclose of one-loop coupling unification.**
  Complementary to GQW: `α_em⁻¹` and `sin²θ_W` predict `α_3(M_Z)` as a
  π-free Ratio (`2π` cancels). `gut.coupling-unification-interval` is an
  EmpiricalPrediction with sourced PDG 2024 / 2022 input-σ hulls versus
  the PDG `α_s` hull, plus exact Gaussian NLL at the PDG `10^{-4}` scale.
  `physis enclose` independently parses the overlay; not CertifiedNumeric,
  not P3N (count stays 4). Minimal SU(5) is excluded; one-loop MSSM is
  inconclusive. The 3% heuristic cell stays on `gut.coupling-unification`.
  GQW interval FormalClaim identity and certificate pins unchanged.
  GUT-scale 3/8 certificate pin unchanged. Coupling-unification
  interval FormalClaim identity
  `11b15a7f8fbfaab08c38b773cfaac930a13e0d77b021194a7c07676a083e8825`.
  Interval certificate
  `1ad86215fce82a37f806343448386d924c59c2abdc3f7c3a1b1cf77924fb997c`.
  Verified: unit tests; inspect judgment statistical-computed count 2;
  role gates.

- **Independent Interval enclose of the GQW input-interval overlay.**
  `physis enclose` independently parses live `Interval` endpoints on
  `gut.weinberg-angle-mz-interval` via `Interval::parse_display` of
  `[lo, hi]`. `Verdict::with_interval_enclosure` stores those display
  strings without assigning `CertifiedNumeric`, so the cell stays
  `statistical computed` and `inspect trust P3N` stays count 4. Super-K,
  unique-vacuum, and Poincaré still refuse. The overlay is not the
  certificate. Not a kernel proof, not Canonical, not P4. Loop encloses
  the overlay with the P3N Ratio cells. GUT-scale 3/8 certificate pin
  unchanged
  `0967e9f42ec9ff0fd8e29fecc5bb5a3ed9aba4974ac77b0e5217a4bb634ec202`.
  GQW interval certificate
  `abb134fa6d8b112c92c0dfbefb789a4446cbed54aaeb83528658bd65d2b1ace3`.
  Verified: parse_display rejects unreduced and reversed dumps; role
  gates; P3N count.

- **SciInterval is a closed hull of overflow-scale SciExact endpoints.**
  `physis-numeric` stores measured hulls whose Ratio scale overflows
  `i128` as canonical terminating-decimal endpoints. `parse_display` of
  `[lo, hi]` is the independent check of a dump string. This is not
  Interval of Ratio. Electron mass `m_e` is the first ledger user.
  Not a kernel proof, not Canonical, not P4.

- **Ratio order does not saturate on CODATA mass scale.**
  Cross-multiplying `a.num * b.den` for a `10^{38}` denominator
  overflows `i128`. Comparison now uses a 256-bit product instead of
  saturating `i128` mul, so a one-sigma hull at proton-mass scale is
  independently checkable. Small-ratio order (GQW `3/8`, PDG NLL) is
  unchanged. Interval arithmetic still saturates; this is comparison
  only. Not a kernel proof, not Canonical, not P4.

### Constants

- **IAU 2015 bolometric magnitude zero luminosity is an exact Ratio.**
  `physis-constants` versions `L_0` as `30128000000000000000000000000` W from
  IAU 2015 Resolution B2 equation 1 (arXiv:1510.06262). That is the
  conversion ruler for `M_Bol = 0`, not a measured luminosity, not
  `L_sun`, not `S_sun`, not apparent `f_0` (10 parsecs, π), and not P3N.
  `L0`, `Lbol`, and `Mbol` are not second names. `physis_model`
  `bolometric_zero_luminosity()` Qty locksteps via integer `to_f64`.
  Adding `L_0` to LEDGER changes the ledger bundle pin. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4.
  Verified: `L_0` hash
  b913459b9de403e5040bba4f1ab82c2619c782230d6cf21816ab70beb994e71b;
  node
  1b41364e1e14a766419e77b39f8b156b2a846d0e0cb3a498edcc3050c2e2a6b8;
  ledger node
  a7ed5256f5c6962961df3ff4f6b3f35f644655bc52fa18ff2bb4c10a3be1907b.
  `GM_jup` hash
  e02c32483ee4b17fdc379fbcab7ff357487937b3ebea7af48678b0bfea851d8c,
  `T_sun` hash
  80708833f8297957273286c4f202016c2a5f5bfd9899e9da1e4a9207e912d11c, and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **IAU 2015 nominal Jovian mass parameter is an exact Ratio.**
  `physis-constants` versions `GM_jup` as `126686530000000000` m³ s⁻² from
  Prša et al., Astron. J. 152, 41 table 1 (IAU 2015 Resolution B3). That
  is the conversion ruler `(GM)_J^N`, not a measured Jovian mass, not
  `GM_earth`, not `GM_sun`, not CODATA `G`, and not P3N. `GM_J`, `GMjup`,
  and `mu_jup` are not second names. `physis_model` `jovian_gm()` Qty
  locksteps via integer `to_f64`. Adding `GM_jup` to LEDGER changes the
  ledger bundle pin. Theories still evaluate with `f64` Qty. That is not
  a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `GM_jup` hash
  e02c32483ee4b17fdc379fbcab7ff357487937b3ebea7af48678b0bfea851d8c;
  node
  a0aaa1beb9fd20c2e6bf757b3922aec34aabed77e404196a4baa38ddc6e40756;
  ledger node
  fbb876d40ad81848cc51fcccd8bc8b48ea79098094fc9f3ce0ed9c4c66137b53.
  `GM_earth` hash
  434261b3d8c3d1dee7e4772e4f166c1762a0bfbaa56356056d6dc1cb145bef06,
  `T_sun` hash
  80708833f8297957273286c4f202016c2a5f5bfd9899e9da1e4a9207e912d11c, and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **IAU 2015 nominal terrestrial mass parameter is an exact Ratio.**
  `physis-constants` versions `GM_earth` as `398600400000000` m³ s⁻² from
  Prša et al., Astron. J. 152, 41 table 1 (IAU 2015 Resolution B3). That
  is the conversion ruler `(GM)_E^N`, not a measured terrestrial mass,
  not `GM_sun`, not CODATA `G`, and not P3N. `GM_E`, `GMearth`, and
  `mu_earth` are not second names. `physis_model` `terrestrial_gm()` Qty
  locksteps via integer `to_f64`. Adding `GM_earth` to LEDGER changes the
  ledger bundle pin. Theories still evaluate with `f64` Qty. That is not
  a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `GM_earth` hash
  434261b3d8c3d1dee7e4772e4f166c1762a0bfbaa56356056d6dc1cb145bef06;
  node
  76561254bec6cbfaadbb00ca0d858c7fece88dfe7fda29a73bc17b7009392500;
  ledger node
  e04d407b01cd165c4a5d9a89b97e557fdedc56a1875c040c8b32997db6017833.
  `R_jup_p` hash
  7a01b6a227db031d78c739a86e8c66b9ce17988a190d9969f61d14353ecf83ae,
  `T_sun` hash
  80708833f8297957273286c4f202016c2a5f5bfd9899e9da1e4a9207e912d11c, and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **IAU 2015 nominal Jovian polar radius is an exact Ratio.**
  `physis-constants` versions `R_jup_p` as `66854000` m from Prša et al.,
  Astron. J. 152, 41 table 1 (IAU 2015 Resolution B3). That is the polar
  conversion ruler `R_Jp^N`, not a measured Jovian radius, not
  equatorial `R_jup`, not `R_earth_p`, not `R_sun`, and not P3N. `R_pJ`,
  `R_Jp`, and `jovian_polar_radius` are not second names. `physis_model`
  `jovian_polar_radius()` Qty locksteps via integer `to_f64`.
  Adding `R_jup_p` to LEDGER changes the ledger bundle pin. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4.
  Verified: `R_jup_p` hash
  7a01b6a227db031d78c739a86e8c66b9ce17988a190d9969f61d14353ecf83ae;
  node
  598c897741b73575f2617eeb1ee918c072ce028b163788e70bfe09fc85148f53;
  ledger node
  5b040306eaf3d97d2e9f84ac160281bd936689183e4411d740fe44a0f4500719.
  `R_jup` hash
  c071bb5f05ba21b8aec1e0ee87062de6f213eb3e713f2fcd24b11776de7de44f,
  `T_sun` hash
  80708833f8297957273286c4f202016c2a5f5bfd9899e9da1e4a9207e912d11c, and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **IAU 2015 nominal Jovian equatorial radius is an exact Ratio.**
  `physis-constants` versions `R_jup` as `71492000` m from Prša et al.,
  Astron. J. 152, 41 table 1 (IAU 2015 Resolution B3). That is the
  equatorial conversion ruler `R_Je^N`, not a measured Jovian radius,
  not polar `R_pJ`, not `R_earth`, not `R_sun`, and not P3N. `Rjup`,
  `R_eJ`, and `jovian_radius` are not second names. `physis_model`
  `jovian_equatorial_radius()` Qty locksteps via integer `to_f64`.
  Adding `R_jup` to LEDGER changes the ledger bundle pin. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4.
  Verified: `R_jup` hash
  c071bb5f05ba21b8aec1e0ee87062de6f213eb3e713f2fcd24b11776de7de44f;
  node
  dfbf13f3d104457d6bc1baa1cb32225ce4cdc4c988c773944d48cbb4e88c69c1;
  ledger node
  cd0b1e44544a1c2a2439cff26d8cd7ab45f896e32ca4395da0ffc66ca5178c1d.
  `R_earth_p` hash
  dd77beb174e99c8772a1219c6a714d49800747bd3351f8abcdd472630da99cd9,
  `T_sun` hash
  80708833f8297957273286c4f202016c2a5f5bfd9899e9da1e4a9207e912d11c, and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **IAU 2015 nominal terrestrial polar radius is an exact Ratio.**
  `physis-constants` versions `R_earth_p` as `6356800` m from Prša et al.,
  Astron. J. 152, 41 table 1 (IAU 2015 Resolution B3). That is the polar
  conversion ruler `R_Ep^N`, not a measured geodetic radius, not
  equatorial `R_earth`, not `R_sun`, and not P3N. `R_pE`, `R_Ep`, and
  `polar_radius` are not second names. `physis_model`
  `terrestrial_polar_radius()` Qty locksteps via integer `to_f64`.
  Adding `R_earth_p` to LEDGER changes the ledger bundle pin. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4.
  Verified: `R_earth_p` hash
  dd77beb174e99c8772a1219c6a714d49800747bd3351f8abcdd472630da99cd9;
  node
  aa1c29a7ea61e06c4408abc2e7e9d8458d870955fcfa514424965ad7cec27126;
  ledger node
  4bb18a6799704cca1a062d3e58a8d014b8fabb3d89dfb40be5769d05ea31278d.
  `R_earth` hash
  c150e7ec9e5e3f915003f91334cafd06669fa6b36429cd9d4e016e4b7a47fab0,
  `T_sun` hash
  80708833f8297957273286c4f202016c2a5f5bfd9899e9da1e4a9207e912d11c, and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **IAU 2015 nominal terrestrial equatorial radius is an exact Ratio.**
  `physis-constants` versions `R_earth` as `6378100` m from Prša et al.,
  Astron. J. 152, 41 table 1 (IAU 2015 Resolution B3). That is the
  equatorial conversion ruler `R_Ee^N`, not a measured geodetic radius,
  not polar `R_pE`, not `R_sun`, and not P3N. `Rearth`, `R_eE`, `R_E`,
  and `earth_radius` are not second names. `physis_model`
  `terrestrial_equatorial_radius()` Qty locksteps via integer `to_f64`.
  Adding `R_earth` to LEDGER changes the ledger bundle pin. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4.
  Verified: `R_earth` hash
  c150e7ec9e5e3f915003f91334cafd06669fa6b36429cd9d4e016e4b7a47fab0;
  node
  20b574fdd9c19171a63c6600da72042144815dd7e17e3defed474b25207eb5f1;
  ledger node
  6188f32e6ad6bf261ec5a3a895b645700f912d3e4ef2f6691ed114e9b65d90cf.
  `T_sun` hash
  80708833f8297957273286c4f202016c2a5f5bfd9899e9da1e4a9207e912d11c,
  `S_sun` hash
  26a5268a7775ede81697bfc65775a12c8ff7b504a70c27b2c00558c9e9a685cd, and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **IAU 2015 nominal solar effective temperature is an exact Ratio.**
  `physis-constants` versions `T_sun` as `5772` K from Prša et al.,
  Astron. J. 152, 41 table 1 (IAU 2015 Resolution B3). That is a
  conversion ruler, not a measured photospheric temperature, not
  `S_sun`, and not P3N. `Tsun`, `T-sun`, `Teff`, `T_eff`, `T_eff_sun`,
  and `solar_temperature` are not second names. `physis_model`
  `solar_effective_temperature()` Qty locksteps via integer `to_f64`.
  Adding `T_sun` to LEDGER changes the ledger bundle pin. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4.
  Verified: `T_sun` hash
  80708833f8297957273286c4f202016c2a5f5bfd9899e9da1e4a9207e912d11c;
  node
  98f786f76f30a1257bd093af7014a20f7f8b42a68d24c7f692fed8a83b3a0c72;
  ledger node
  332ef45c93322e26251b069a15c0f8c3623f771c3a8cb05d674dd4a7f1db3888.
  `S_sun` hash
  26a5268a7775ede81697bfc65775a12c8ff7b504a70c27b2c00558c9e9a685cd,
  `sigma_h` hash
  b364a92ff1578713cbdfb75c740edfed04acb3d047adafc240433b39aa55aab9, and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **IAU 2015 nominal solar irradiance is an exact Ratio.**
  `physis-constants` versions `S_sun` as `1361` W m^{-2} from Prša et
  al., Astron. J. 152, 41 table 1 (IAU 2015 Resolution B3). That is a
  conversion ruler, not a measured total solar irradiance, not `L_sun`,
  and not P3N. `Ssun`, `S-sun`, `TSI`, `solar_constant`, `solar_irradiance`,
  and `S0` are not second names. `physis_model` `solar_irradiance()` Qty
  locksteps via integer `to_f64`. Adding `S_sun` to LEDGER changes the
  ledger bundle pin. Theories still evaluate with `f64` Qty. That is not
  a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `S_sun` hash
  26a5268a7775ede81697bfc65775a12c8ff7b504a70c27b2c00558c9e9a685cd;
  node
  9c6333c119483f19d558659562d5e191e7466f8b1390f8bb475b3e724ad14201;
  ledger node
  ba3c73f68c1fc7bbd4ab886904b57b7a527872b401bf91f251664b65f65d47d3.
  `sigma_h` hash
  b364a92ff1578713cbdfb75c740edfed04acb3d047adafc240433b39aa55aab9,
  `L_sun` hash
  444f85fba501ddec8fb08ba403c1b869cc78a2284df5466a56a617043807bbc4, and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 helion shielding shift is a one-sigma Interval.**
  `physis-constants` versions `sigma_h` as the CODATA 2018 one-sigma hull
  `5.996743(10)×10^{-5}` from the NIST 2018 complete listing (JPCRD 50, 033105
  table XXXI ATOMIC AND NUCLEAR). This is the recommended `σ_h` hull, not HT
  sibling `sigma_tp`, not HD sibling `sigma_dp`, and not proton magnetic
  shielding `sigma0p`. Not a FormalClaim reconstructing `1 − μ′_h/μ_h` from a
  live lookup. Table XXXI OCR has no `σ_h` row. Decade `10^{10}` on the printed
  5996743-digit is the 10× trap. 2018 last-digit is `5996743`; 2022 last-digit
  `7029` is excluded. The ledger name is `sigma_h`; `sigmah`, `sigma-h`,
  `sigma_helion`, `s_h`, `helion_shielding`, and `shielding_shift` are not
  second names. `physis_model` `helion_shielding_shift()` Qty locksteps to the
  CODATA centre inside the hull. Adding `sigma_h` to LEDGER changes the ledger
  bundle pin. Theories still evaluate with `f64` Qty. That is not a kernel
  proof, not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph
  id unchanged. P3N count stays 4.
  Verified:
  `sigma_h` hash
  b364a92ff1578713cbdfb75c740edfed04acb3d047adafc240433b39aa55aab9;
  node e08986033fa722a04269fded170d0288d7c33a9885f4679f603893e4f1750e25;
  ledger node
  1c0ae14b498588bc92242061a2293f123357b9af5b9fc518a40f1186271c342c.
  `sigma_tp` hash
  0d2f0fe3278bcf8a51cc468c792106f732e5d84703cd342cbd559978c038f90f and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 shielding difference of t and p in HT is a one-sigma Interval.**
  `physis-constants` versions `sigma_tp` as the CODATA 2018 one-sigma hull
  `2.4140(20)×10^{-8}` from JPCRD 50, 033105 table XXXI (ATOMIC AND NUCLEAR).
  This is the recommended `σ_tp` hull from the NIST 2018 complete listing and
  the JPCRD adjusted-constant / XIV.C row, not HD sibling `sigma_dp`, and
  not proton magnetic shielding `sigma0p`. Table XXI D43 prints the same
  digits as the recommended hull and is not a second name. Not a FormalClaim
  reconstructing that difference from a live lookup. Table XXXI OCR has no
  `σ_tp` row. Decade `10^{11}` on the printed 24140-digit is the 10× trap.
  2018 last-digit is `24140`; 2022 last-digit `39450` is excluded. The ledger
  name is `sigma_tp`; `sigmatp`, `sigma-tp`, `sigma_t_p`, `s_tp`,
  `HT_shielding`, and `D43` are not second names. `physis_model`
  `shielding_difference_t_p_in_ht()` Qty locksteps to the CODATA centre
  inside the hull. Adding `sigma_tp` to LEDGER changes the ledger bundle
  pin. Theories still evaluate with `f64` Qty. That is not a kernel
  proof, not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph
  id unchanged. P3N count stays 4.
  Verified:
  `sigma_tp` hash
  0d2f0fe3278bcf8a51cc468c792106f732e5d84703cd342cbd559978c038f90f;
  node bc532b1ad32b8920e859c14f2fc288e13382210d76593cd048efacf778fc3cf9;
  ledger node
  b558cdb0e728b2d414829dba3efd8eab7e2fe0897e89ac0e772ee300e3747be2.
  `sigma_dp` hash
  23b7863479f6cdb51f13efc947f52a2620dcb69c3363e46ab1fe848d293a3ff6 and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 shielding difference of d and p in HD is a one-sigma Interval.**
  `physis-constants` versions `sigma_dp` as the CODATA 2018 one-sigma hull
  `2.0200(20)×10^{-8}` from JPCRD 50, 033105 table XXXI (ATOMIC AND NUCLEAR).
  This is the recommended `σ_dp` hull from the NIST 2018 complete listing and
  the JPCRD adjusted-constant / XIV.C row, not proton magnetic shielding
  `sigma0p`, and not the HT sibling `σ_tp`. Table XXI D42 prints the same
  digits as the recommended hull and is not a second name. Not a FormalClaim
  reconstructing that difference from a live lookup. Table XXXI OCR has no
  `σ_dp` row. Decade `10^{11}` on the printed 20200-digit is the 10× trap.
  2018 last-digit is `20200`; 2022 last-digit `8770` is excluded. The ledger
  name is `sigma_dp`; `sigmadp`, `sigma-dp`, `sigma_d_p`, `s_dp`,
  `HD_shielding`, and `D42` are not second names. `physis_model`
  `shielding_difference_d_p_in_hd()` Qty locksteps to the CODATA centre
  inside the hull. Adding `sigma_dp` to LEDGER changes the ledger bundle
  pin. Theories still evaluate with `f64` Qty. That is not a kernel
  proof, not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph
  id unchanged. P3N count stays 4.
  Verified:
  `sigma_dp` hash
  23b7863479f6cdb51f13efc947f52a2620dcb69c3363e46ab1fe848d293a3ff6;
  node 60f0667a7e4cfbb470a8769dfdeee4c4037bb4a97f0f8319b03f9039b7580956;
  ledger node
  fc78b84cd4a5eeaf226450bcdd4069a03188880ad65d34fedf3ecd82bb879d4d.
  `mu_t_mup` hash
  0326c3b5a71ba51c0c9dc8178cd4decfd1c1475ce7bbb53ff427c8f40bef7ce2 and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 triton-proton magnetic-moment ratio is a one-sigma Interval.**
  `physis-constants` versions `mu_t_mup` as the CODATA 2018 one-sigma hull
  `1.0666399191(21)` from JPCRD 50, 033105 table XXXI (Triton). This is the
  recommended free `μ_t/μ_p` hull from the NIST 2018 complete listing and
  the JPCRD adjusted-constant row, not triton g-factor `gt`, not
  deuteron-proton `mu_d_mup`, and not bound Table XXI D41
  `μ_t(HT)/μ_p(HT)` (different digits). Not a FormalClaim reconstructing
  that quotient from a live lookup. Decade `10^{9}` on the printed 9191-digit
  is the 10× trap. 2018 last-digit is `9191`; 2022 last-digit `9189` is not
  the stored centre (the 2018 hull still covers it). The ledger name is
  `mu_t_mup`; `mut_mup`, `mu_t/mup`, `mu-t-mup`, `mu_t_mu_p`, `mu_tp`, and
  `D41` are not second names. `physis_model`
  `triton_proton_magnetic_moment_ratio()` Qty locksteps to the CODATA
  centre inside the hull. Adding `mu_t_mup` to LEDGER changes the ledger
  bundle pin. Theories still evaluate with `f64` Qty. That is not a kernel
  proof, not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph
  id unchanged. P3N count stays 4.
  Verified:
  `mu_t_mup` hash
  0326c3b5a71ba51c0c9dc8178cd4decfd1c1475ce7bbb53ff427c8f40bef7ce2;
  node a594dfc5dd4f6d68da2d74b993a5bd54f5bfc451de97ffdedeb534cc00970663;
  ledger node
  44ad3d3dcf486a0d6886a40aacba51147c0489406626a11b413242233df71e95.
  `au_chi` hash
  b5edc5651e0f2f662eafb0897c3354db76129ed9350444a27e9a76998ebc851f and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 atomic unit of magnetizability is a one-sigma SciInterval.**
  `physis-constants` versions `au_chi` as the CODATA 2018 one-sigma hull
  `7.8910366008(48)×10^{-29}` J T^{-2} from JPCRD 50, 033105 table XXXIV
  (atomic units). This is the recommended atomic unit of magnetizability
  listed as `e² a0² / m_e`, not magnetic flux density `au_B`, not magnetic
  dipole `au_mu`, and not a FormalClaim reconstructing that quotient from
  a live lookup. The printed formula does not cite ħ; the ledger still
  stores the printed hull. A Ratio denominator `10^{39}` overflows i128.
  Atomic unit of time still cites ħ and is not stored. Atomic unit of
  electric potential is a second name for `Eh_eV` and is not stored. The
  {220} lattice spacing is not stored. Decade `10^{38}` on the printed
  6008-digit is the 10× trap. 2018 last-digit is `6008`; 2022 last-digit
  `5794` is excluded. The ledger name is `au_chi`; `auchi`, `au-chi`,
  `au_xi`, `magnetizability`, `chi_au`, `e2a02_me`, and
  `au_magnetizability` are not second names. `physis_model`
  `atomic_unit_of_magnetizability()` Qty locksteps to the CODATA centre
  inside the hull. Adding `au_chi` to LEDGER changes the ledger bundle
  pin. Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4.
  Verified:
  `au_chi` hash
  b5edc5651e0f2f662eafb0897c3354db76129ed9350444a27e9a76998ebc851f;
  node 6868deeef8f6aaa269fa9111656c9b977454c670786dcf39e604ca0c6e3df974;
  ledger node
  0ffacc6eef9e3626c9aa66b7f7a246e9063c3f424b4d4e729581f24b5f6d66b0.
  `au_B` hash
  74c93d71b77f529ed79af1686c451dbc4a36f36ff80f9e94f39976bf97e9afc3 and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 atomic unit of magnetic flux density is a one-sigma Interval.**
  `physis-constants` versions `au_B` as the CODATA 2018 one-sigma hull
  `2.35051756758(71)×10^{5}` T from JPCRD 50, 033105 table XXXIV
  (atomic units). This is the recommended atomic unit of mag. flux density
  listed as `ħ/(e a0²)`, not magnetic dipole `au_mu`, not Bohr magneton
  `muB`, and not a FormalClaim reconstructing that quotient from a live
  lookup. The printed formula cites ħ and is unused. Atomic unit of time
  still cites ħ and is not stored. Atomic unit of electric potential is a
  second name for `Eh_eV` and is not stored. The {220} lattice spacing is
  not stored. Decade `10^{5}` on the printed 56758-digit is the 10× trap.
  2018 last-digit is `56758`; 2022 last-digit `57077` is excluded. The
  ledger name is `au_B`; `auB`, `au-B`, `au_T`, `tesla_au`, `mag_flux`,
  `magnetic_flux_density`, `hbar_ea02`, and `B_au` are not second names.
  `physis_model` `atomic_unit_of_magnetic_flux_density()` Qty locksteps to
  the CODATA centre inside the hull. Adding `au_B` to LEDGER changes the
  ledger bundle pin. Theories still evaluate with `f64` Qty. That is not
  a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified:
  `au_B` hash
  74c93d71b77f529ed79af1686c451dbc4a36f36ff80f9e94f39976bf97e9afc3;
  node a4a2d3b8eb217eccc9467f6a48c04b6916a691e35eb30c6d40e70b3b60daa3a0;
  ledger node
  351779056aee4f1853b1c3699284134ea0aa751e203ded2ce45bd93b4825b6f3.
  `au_mu` hash
  6a48549025982a0761b17bf80c9ce18bc9dee346e3e9cd906a37a07c1a186e58 and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 atomic unit of magnetic dipole moment is a one-sigma Interval.**
  `physis-constants` versions `au_mu` as the CODATA 2018 one-sigma hull
  `1.85480201566(56)×10^{-23}` J T^{-1} from JPCRD 50, 033105 table XXXIV
  (atomic units). This is the recommended atomic unit of mag. dipole mom.
  listed as `2μ_B = ħe/m_e`, not Bohr magneton `muB`, not electron magnetic
  moment `mu_e`, and not a FormalClaim reconstructing twice Bohr magneton
  from a live lookup. The printed formula cites ħ and is unused. Atomic
  unit of time still cites ħ and is not stored. Atomic unit of electric
  potential is a second name for `Eh_eV` and is not stored. The {220}
  lattice spacing is not stored. Decade `10^{33}` on the printed 01566-digit
  is the 10× trap. 2018 last-digit is `01566`; 2022 last-digit `01315` is
  excluded. The ledger name is `au_mu`; `aumu`, `au-mu`, `au_mdm`,
  `mag_dipole`, `magnetic_dipole`, `2muB`, `two_muB`, and `ehbar_me` are
  not second names. `physis_model` `atomic_unit_of_magnetic_dipole_moment()`
  Qty locksteps to the CODATA centre inside the hull. Adding `au_mu` to
  LEDGER changes the ledger bundle pin. Theories still evaluate with `f64`
  Qty. That is not a kernel proof, not Canonical, not P4. Encode pins
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified:
  `au_mu` hash
  6a48549025982a0761b17bf80c9ce18bc9dee346e3e9cd906a37a07c1a186e58;
  node 9fa5ea601712ac03f972d0286b883572ec4cecdf2bf64f6143717b1b17f34696;
  ledger node
  bd5413ea3d984e34b509a18e99228aaa9d500aa9db0f559586ea5b474a003ba9.
  `au_hyp2` hash
  fa9ba2625e88e8128830141fdca61a3a0ba52376e323ff959002f897315feb2e and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 atomic unit of 2nd hyperpolarizability is a one-sigma SciInterval.**
  `physis-constants` versions `au_hyp2` as the CODATA 2018 one-sigma hull
  `6.2353799905(38)×10^{-65}` C^{4} m^{4} J^{-3} from JPCRD 50, 033105 table XXXIV
  (atomic units). This is the recommended atomic unit of 2nd hyperpolarizability
  defined as `e⁴ a0⁴ / E_h³`, not elementary charge `e`, not atomic unit of 1st
  hyperpolarizability `au_hyp`, and not a FormalClaim reconstructing that
  quotient from a live lookup. A Ratio denominator `10^{75}` overflows `i128`,
  so the ledger stores a SciInterval. Atomic unit of time still cites ħ and
  is not stored. Atomic unit of electric potential is a second name for
  `Eh_eV` and is not stored. The {220} lattice spacing is not stored. Decade
  `10^{74}` on the printed 9905-digit is the 10× trap. 2018 last-digit is
  `9905`; 2022 last-digit `9735` is excluded. The ledger name is `au_hyp2`;
  `auhypol2`, `au_gamma`, `gamma`, `second_hyperpolarizability`, `au_2nd_hyp`,
  `e4a04_Eh3`, and `hyperpolarizability2` are not second names.
  `physis_model` `atomic_unit_of_second_hyperpolarizability()` Qty locksteps
  to the CODATA centre inside the hull. Adding `au_hyp2` to LEDGER changes
  the ledger bundle pin. Theories still evaluate with `f64` Qty. That is not
  a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified:
  `au_hyp2` hash
  fa9ba2625e88e8128830141fdca61a3a0ba52376e323ff959002f897315feb2e;
  node 283772212168907d278d9a5a9bce9331843db9830e30e03f08235d13bb415103;
  ledger node
  32795e090f1531e5df73dce23244804824cb3ccac0c41707158b4305c7c300a2.
  `au_hyp` hash
  434651937f575d0bd441cbbf9277985302d35b5d637c1ed3be03f0f4d881cb04 and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 atomic unit of 1st hyperpolarizability is a one-sigma SciInterval.**
  `physis-constants` versions `au_hyp` as the CODATA 2018 one-sigma hull
  `3.2063613061(15)×10^{-53}` C^{3} m^{3} J^{-2} from JPCRD 50, 033105 table XXXIV
  (atomic units). This is the recommended atomic unit of 1st hyperpolarizability
  defined as `e³ a0³ / E_h²`, not elementary charge `e`, not atomic unit of
  electric polarizability `au_pol`, and not a FormalClaim reconstructing that
  quotient from a live lookup. A Ratio denominator `10^{63}` overflows `i128`,
  so the ledger stores a SciInterval. Atomic unit of time still cites ħ and
  is not stored. Atomic unit of electric potential is a second name for
  `Eh_eV` and is not stored. The {220} lattice spacing is not stored. Decade
  `10^{62}` on the printed 3061-digit is the 10× trap. 2018 last-digit is
  `3061`; 2022 last-digit `2996` is excluded. The ledger name is `au_hyp`;
  `auhypol`, `au_beta`, `beta`, `hyperpolarizability`,
  `au_hyperpolarizability`, `e3a03_Eh2`, and `au_1st_hyp` are not second
  names. `physis_model` `atomic_unit_of_first_hyperpolarizability()` Qty
  locksteps to the CODATA centre inside the hull. Adding `au_hyp` to LEDGER
  changes the ledger bundle pin. Theories still evaluate with `f64` Qty. That
  is not a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified:
  `au_hyp` hash
  434651937f575d0bd441cbbf9277985302d35b5d637c1ed3be03f0f4d881cb04;
  node f2e2cb6bff1a1717af0dcfa04b668988e2836d81dc6bcda7f1e8c09e7ec75561;
  ledger node
  35dd2c74a6d871ac8c56dc5af13f4ed6a5b932125ea29788af0f803eb6c6d83f.
  `au_pol` hash
  83f51c8d2e9ae545ab1298786b37bfa9b49ff6a2172b60299e872b9a7de4e9f7 and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 atomic unit of electric polarizability is a one-sigma SciInterval.**
  `physis-constants` versions `au_pol` as the CODATA 2018 one-sigma hull
  `1.64877727436(50)×10^{-41}` C^{2} m^{2} J^{-1} from JPCRD 50, 033105 table XXXIV
  (atomic units). This is the recommended atomic unit of electric polarizability
  defined as `e² a0² / E_h`, not elementary charge `e`, not atomic unit of
  electric quadrupole moment `ea02`, not permittivity `au_eps`, and not a
  FormalClaim reconstructing that quotient from a live lookup. A Ratio
  denominator `10^{52}` overflows `i128`, so the ledger stores a SciInterval.
  Atomic unit of time still cites ħ and is not stored. Atomic unit of electric
  potential is a second name for `Eh_eV` and is not stored. The {220} lattice
  spacing is not stored. Decade `10^{51}` on the printed 7436-digit is the 10×
  trap. 2018 last-digit is `7436`; 2022 last-digit `7212` is excluded. The
  ledger name is `au_pol`; `auepol`, `A_ep`, `au_alpha`, `alpha_e`,
  `polarizability`, `au_polarizability`, and `e2a02_Eh` are not second names.
  `physis_model` `atomic_unit_of_electric_polarizability()` Qty locksteps to
  the CODATA centre inside the hull. Adding `au_pol` to LEDGER changes the
  ledger bundle pin. Theories still evaluate with `f64` Qty. That is not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified:
  `au_pol` hash
  83f51c8d2e9ae545ab1298786b37bfa9b49ff6a2172b60299e872b9a7de4e9f7;
  node 597e7d6c752f32550960ded18b2c621cef405cddd1d8bb90bec7e2d40697ccb7;
  ledger node
  7586a5f129193368d6399771c71f3d25ac051ea4b658595a2d4bc2ffd93ab735.
  `ea02` hash
  eb5d43dc3a14917258bbbf6b0b2d98d3cc5c54b1ce734bd3287159b43d201bbc and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 atomic unit of electric quadrupole moment is a one-sigma SciInterval.**
  `physis-constants` versions `ea02` as the CODATA 2018 one-sigma hull
  `4.4865515246(14)×10^{-40}` C m^{2} from JPCRD 50, 033105 table XXXIV
  (atomic units). This is the recommended atomic unit of electric quadrupole
  moment defined as `e a0²`, not elementary charge `e`, not atomic unit of
  electric dipole moment `ea0`, and not a FormalClaim reconstructing that
  product from a live lookup. A Ratio denominator `10^{50}` overflows `i128`,
  so the ledger stores a SciInterval. Atomic unit of time still cites ħ and
  is not stored. Atomic unit of electric potential is a second name for
  `Eh_eV` and is not stored. The {220} lattice spacing is not stored. Decade
  `10^{49}` on the printed 5246-digit is the 10× trap. 2018 last-digit is
  `5246`; 2022 last-digit `5185` is excluded. The ledger name is `ea02`;
  `ea0_2`, `e_a0_2`, `au_Q`, `au_eq`, `ea0sq`, and `electric_quadrupole` are
  not second names. `physis_model` `atomic_unit_of_electric_quadrupole_moment()`
  Qty locksteps to the CODATA centre inside the hull. Adding `ea02` to LEDGER
  changes the ledger bundle pin. Theories still evaluate with `f64` Qty. That
  is not a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified:
  `ea02` hash
  eb5d43dc3a14917258bbbf6b0b2d98d3cc5c54b1ce734bd3287159b43d201bbc;
  node fb03c45ef722877d0484588e7f81625fc15d1fd21fdc5513b548c4f9ea4be38a;
  ledger node
  98958f2327584dd1039acd44fed2caaf0101e618230eaae96ae36190a6a417d2.
  `ea0` hash
  509271eab9ef8873e10798db919a211b1e5c10670f3da41c7fc9e420a10ae566 and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 atomic unit of electric dipole moment is a one-sigma SciInterval.**
  `physis-constants` versions `ea0` as the CODATA 2018 one-sigma hull
  `8.4783536255(13)×10^{-30}` C m from JPCRD 50, 033105 table XXXIV
  (atomic units). This is the recommended atomic unit of electric dipole
  moment defined as `e a0`, not elementary charge `e`, not atomic unit of
  electric field gradient `au_EFG`, and not a FormalClaim reconstructing
  that product from a live lookup. A Ratio denominator `10^{40}` overflows
  `i128`, so the ledger stores a SciInterval. Atomic unit of time still
  cites ħ and is not stored. Atomic unit of electric potential is a second
  name for `Eh_eV` and is not stored. The {220} lattice spacing is not
  stored. Decade `10^{39}` on the printed 6255-digit is the 10× trap.
  2018 last-digit is `6255`; 2022 last-digit `6198` is excluded. The
  ledger name is `ea0`; `ea_0`, `e_a0`, `au_d`, `au_dip`, `au_ea0`, and
  `electric_dipole` are not second names. `physis_model`
  `atomic_unit_of_electric_dipole_moment()` Qty locksteps to the CODATA
  centre inside the hull. Adding `ea0` to LEDGER changes the ledger
  bundle pin. Theories still evaluate with `f64` Qty. That is not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified:
  `ea0` hash
  509271eab9ef8873e10798db919a211b1e5c10670f3da41c7fc9e420a10ae566;
  node 3d447768f55bab6d0260cae026b0a7827c1b34401b80c155d643ca2edcd7dc57;
  ledger node
  d2d440f4637e1d15ae12be6b36f71a44ebb7ffd3befd7bb5d3436f46f897244d.
  `au_EFG` hash
  1bccca0e5554050ed3c407bf6ea59fb7285cee9d91f1bb06f8d68252bb379858 and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 atomic unit of electric field gradient is a one-sigma Interval.**
  `physis-constants` versions `au_EFG` as the CODATA 2018 one-sigma hull
  `9.7173624292(29)×10^{21}` V m^{-2} from JPCRD 50, 033105 table XXXIV
  (atomic units). This is the recommended atomic unit of electric field
  gradient defined as `E_h / (e a0²)`, not elementary charge `e`, not
  atomic unit of electric field `au_E`, and not a FormalClaim
  reconstructing that quotient from a live lookup. Atomic unit of time
  still cites ħ and is not stored. Atomic unit of electric potential is
  a second name for `Eh_eV` and is not stored. The {220} lattice spacing
  is not stored. Decade `10^{10}` on the printed 4292-digit is the 10×
  trap. 2018 last-digit is `4292`; 2022 last-digit `4424` is excluded.
  The ledger name is `au_EFG`; `auEFG`, `au-EFG`, `au_dE`, and
  `electric_field_gradient` are not second names. `physis_model`
  `atomic_unit_of_electric_field_gradient()` Qty locksteps to the CODATA
  centre inside the hull. Adding `au_EFG` to LEDGER changes the ledger
  bundle pin. Theories still evaluate with `f64` Qty. That is not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified:
  `au_EFG` hash
  1bccca0e5554050ed3c407bf6ea59fb7285cee9d91f1bb06f8d68252bb379858;
  node 092b7b2bb4a6214a1dab634376786fd765076d665b6079c28ea4a4d14bb94a18;
  ledger node
  6843d99be1aa4dbaef2953e6c34a3db14f503469966457a437f1a293040cd728.
  `au_E` hash
  bf01829e8a7462f14332d4429ba4e33ca0e624fdb465701f891181bc7725bb48 and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 atomic unit of electric field is a one-sigma Interval.**
  `physis-constants` versions `au_E` as the CODATA 2018 one-sigma hull
  `5.14220674763(78)×10^{11}` V m^{-1} from JPCRD 50, 033105 table XXXIV
  (atomic units). This is the recommended atomic unit of electric field
  defined as `E_h / (e a0)`, not elementary charge `e`, not atomic unit
  of charge density `au_rho`, and not a FormalClaim reconstructing that
  quotient from a live lookup. Atomic unit of time still cites ħ and is
  not stored. Atomic unit of electric potential is a second name for
  `Eh_eV` and is not stored. The {220} lattice spacing is not stored.
  Dividing the e11 centre by 10 is the 10× trap. 2018 last-digit is
  `763`; 2022 last-digit `112` is excluded. The ledger name is `au_E`;
  `auE`, `au-E`, `E_au`, and `electric_field` are not second names.
  `physis_model` `atomic_unit_of_electric_field()` Qty locksteps to the
  CODATA centre inside the hull. Adding `au_E` to LEDGER changes the
  ledger bundle pin. Theories still evaluate with `f64` Qty. That is
  not a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified:
  `au_E` hash
  bf01829e8a7462f14332d4429ba4e33ca0e624fdb465701f891181bc7725bb48;
  node b2201788613356ad96465078c7c0797ef43122a352b5bf53ccb4f3e0c7ef208e;
  ledger node
  705eff2408bc4e0f26a002942a77e0f3a2e944784b5afb6abb298a664691617f.
  `au_rho` hash
  438f5e555b9af97c484c28fcd7227ed3fe7797300b9b738d5d0e0a8bc4dade4c and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 atomic unit of charge density is a one-sigma Interval.**
  `physis-constants` versions `au_rho` as the CODATA 2018 one-sigma hull
  `1.08120238457(49)×10^{12}` C m^{-3} from JPCRD 50, 033105 table XXXIV
  (atomic units). This is the recommended atomic unit of charge density
  defined as `e / a0³`, not elementary charge `e`, not atomic unit of
  current `au_I`, and not a FormalClaim reconstructing that quotient
  from a live lookup. Atomic unit of time still cites ħ and is not
  stored. Atomic unit of electric potential is a second name for
  `Eh_eV` and is not stored. The {220} lattice spacing is not stored.
  Dropping the trailing zero of the e12 centre is the 10× trap. 2018
  last-digit is `457`; 2022 last-digit `677` is excluded. The ledger
  name is `au_rho`; `aurho`, `au-rho`, `au_n`, and `charge_density` are
  not second names. `physis_model` `atomic_unit_of_charge_density()` Qty
  locksteps to the CODATA centre inside the hull. Adding `au_rho` to
  LEDGER changes the ledger bundle pin. Theories still evaluate with
  `f64` Qty. That is not a kernel proof, not Canonical, not P4. Encode
  pins unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified:
  `au_rho` hash
  438f5e555b9af97c484c28fcd7227ed3fe7797300b9b738d5d0e0a8bc4dade4c;
  node c9a89a107638023a042ec932406cf5f08ab5351d453b497401a9c231dc29f2b0;
  ledger node
  f7ef3b5b2a03f147c65b05cf6028248cf5285807bac3c095cb208c1b66afe7e2.
  `au_I` hash
  3acb601feadcd30599636ce6217327ba99f8a27962026b7459beaf38afc75e5a and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 atomic unit of current is a one-sigma Interval.**
  `physis-constants` versions `au_I` as the CODATA 2018 one-sigma hull
  `6.623618237510(13)×10^{-3}` A from JPCRD 50, 033105 table XXXIV
  (atomic units). This is the recommended atomic unit of current
  defined as `e E_h / ħ`, not elementary charge `e`, not molybdenum
  x unit `xu_Mo`, and not a FormalClaim reconstructing that quotient
  from a live lookup. Atomic unit of time still cites ħ and is not
  stored. Atomic unit of electric potential is a second name for
  `Eh_eV` and is not stored. The {220} lattice spacing is not stored.
  The decade is `10^{15}`; `10^{14}` is the 10× trap. 2018 last-digit
  is `7510`; 2022 last-digit `75082` is contained. The ledger name is
  `au_I`; `auI`, `au-I`, `au_i`, and `atomic_current` are not second
  names. `physis_model` `atomic_unit_of_current()` Qty locksteps to the
  CODATA centre inside the hull. Adding `au_I` to LEDGER changes the
  ledger bundle pin. Theories still evaluate with `f64` Qty. That is
  not a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified:
  `au_I` hash
  3acb601feadcd30599636ce6217327ba99f8a27962026b7459beaf38afc75e5a;
  node 6e99ebfc71c14f562d668d9758d7820d9f9dff791205f38d621981d703e874e2;
  ledger node
  0db0c914448fea446a47a610beab7b31ba3e9682d7e205f824c5f31c5d2c4535.
  `xu_Mo` hash
  fd3f7d6bace240afd207aaf8b1dbbc018516ec6e5b9273435cf0351ef678d471 and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 molybdenum x unit is a one-sigma Interval.**
  `physis-constants` versions `xu_Mo` as the CODATA 2018 one-sigma hull
  `1.00209952(53)×10^{-13}` m from JPCRD 50, 033105 table XXXIII
  (x-ray-related quantities). This is the recommended molybdenum x unit
  defined as `λ(MoKα1)/707.831`, not Bohr radius `a0`, not copper x unit
  `xu_Cu`, and not a FormalClaim reconstructing that quotient from a
  live lookup. The {220} lattice spacing is a separately recommended
  hull and is not stored. The decade is `10^{21}`; `10^{20}` is the 10×
  trap. 2018 and 2022 print the same last-digit `52`; the 2018 hull
  contains that centre. The ledger name is `xu_Mo`; `xuMo`, `xu-Mo`,
  and `molybdenum_xu` are not second names. `physis_model`
  `molybdenum_x_unit()` Qty locksteps to the CODATA centre inside the
  hull. Adding `xu_Mo` to LEDGER changes the ledger bundle pin. Theories
  still evaluate with `f64` Qty. That is not a kernel proof, not
  Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4.
  Verified:
  `xu_Mo` hash
  fd3f7d6bace240afd207aaf8b1dbbc018516ec6e5b9273435cf0351ef678d471;
  node 1b4476162aae45b450ecfa50563e6db0328e26fb5c34ab0c4f35ad6381fa7909;
  ledger node
  3c6e799c17b98455803e2a83579ac6e4b4ba30ed3d363c9e73f693b0476be77f.
  `xu_Cu` hash
  053633d6b5b4910c0eddb81392c53e555a2decaf0ec16b42f326a013cb717d41 and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 copper x unit is a one-sigma Interval.**
  `physis-constants` versions `xu_Cu` as the CODATA 2018 one-sigma hull
  `1.00207697(28)×10^{-13}` m from JPCRD 50, 033105 table XXXIII
  (x-ray-related quantities). This is the recommended copper x unit
  defined as `λ(CuKα1)/1537.400`, not Bohr radius `a0`, not Angstrom
  star `Astar`, and not a FormalClaim reconstructing that quotient
  from a live lookup. The molybdenum x unit is a later table XXXIII
  row and is not stored. The {220} lattice spacing is a separately
  recommended hull and is not stored. The decade is `10^{21}`;
  `10^{20}` is the 10× trap. 2018 and 2022 print the same last-digit
  `97`; the 2018 hull contains that centre. The ledger name is `xu_Cu`;
  `xuCu`, `xu-Cu`, and `copper_xu` are not second names. `physis_model`
  `copper_x_unit()` Qty locksteps to the CODATA centre inside the hull.
  Adding `xu_Cu` to LEDGER changes the ledger bundle pin. Theories
  still evaluate with `f64` Qty. That is not a kernel proof, not
  Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4.
  Verified:
  `xu_Cu` hash
  053633d6b5b4910c0eddb81392c53e555a2decaf0ec16b42f326a013cb717d41;
  node 3c5dc132b9ee4d6775f2283245f4040764d3b12efe7cafdd47651164665e425c;
  ledger node
  bf994c822484de43bfd85a56973fa3c2d0db5a06f989ec2a339fb314af986ef8.
  `Astar` hash
  10ae8bb08a4d7b093cd5ba3735496af76ec987748473f532f10731286da3dfb7 and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 Angstrom star is a one-sigma Interval.**
  `physis-constants` versions `Astar` as the CODATA 2018 one-sigma hull
  `1.00001495(90)×10^{-10}` m from JPCRD 50, 033105 table XXXIII
  (x-ray-related quantities). This is the recommended Angstrom star
  defined as `λ(WKα1)/0.2090100`, not Bohr radius `a0`, not lattice
  parameter `a_Si`, and not a FormalClaim reconstructing that quotient
  from a live lookup. The copper and molybdenum x units are later table
  XXXIII rows and are not stored. The {220} lattice spacing is a
  separately recommended hull and is not stored. The decade is `10^{18}`;
  `10^{17}` is the 10× trap. 2018 and 2022 print the same last-digit
  `95`; the 2018 hull contains that centre. The ledger name is `Astar`;
  `Angstromstar`, `A-star`, and `A_star` are not second names.
  `physis_model` `angstrom_star()` Qty locksteps to the CODATA centre
  inside the hull. Adding `Astar` to LEDGER changes the ledger bundle
  pin. Theories still evaluate with `f64` Qty. That is not a kernel
  proof, not Canonical, not P4. Encode pins unchanged. Unique-vacuum
  graph id unchanged. P3N count stays 4.
  Verified:
  `Astar` hash
  10ae8bb08a4d7b093cd5ba3735496af76ec987748473f532f10731286da3dfb7;
  node 789ad47fe6f7845ec4649f92f321a4c3ba4a496bdaa01f96342a5c17ab2b9ae6;
  ledger node
  840383efca29c75ef36aa0080ae2fa5fe92ba404d02a4dfa4f13aeae03573dbe.
  `Vm_Si` hash
  36d2371a832cfaa4ea7cd680b7cabaada3b9eaa5d4c9750c0a7e38c94c080222 and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 molar volume of silicon is a one-sigma Interval.**
  `physis-constants` versions `Vm_Si` as the CODATA 2018 one-sigma hull
  `1.205883199(60)×10^{-5}` m³ mol⁻¹ from JPCRD 50, 033105 table XXXIII
  (x-ray-related quantities). This is the recommended molar volume of
  an ideal single crystal of naturally occurring Si in vacuum at
  22.5 °C, not ideal-gas molar volume `Vm`, not lattice parameter
  `a_Si`, and not a FormalClaim reconstructing `N_A a_Si³ / 8` from
  live lookups. The {220} lattice spacing is a separately recommended
  hull and is not stored. The decade is `10^{14}`; `10^{13}` is the 10×
  trap. 2018 and 2022 print the same last-digit `199`; the 2018 hull
  contains that centre. The ledger name is `Vm_Si`; `VmSi`, `Vm-Si`,
  and `molar_si` are not second names. `physis_model`
  `molar_volume_of_silicon()` Qty locksteps to the CODATA centre inside
  the hull. Adding `Vm_Si` to LEDGER changes the ledger bundle pin.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4.
  Verified:
  `Vm_Si` hash
  36d2371a832cfaa4ea7cd680b7cabaada3b9eaa5d4c9750c0a7e38c94c080222;
  node 552c36b3123815335b65f077e765cfc452ad67512f68806d114388c732024a1e;
  ledger node
  58c6c50474c6e4148300ba42fc847f3860016f99d5e91d8f89c29753467e69e3.
  `a_Si` hash
  8d9072d01e48779f7404ab918ba023a055f197cd2c8d9f19796431939f568344 and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 lattice parameter of silicon is a one-sigma Interval.**
  `physis-constants` versions `a_Si` as the CODATA 2018 one-sigma hull
  `5.431020511(89)×10^{-10}` m from JPCRD 50, 033105 table XXXIII
  (x-ray-related quantities). This is the recommended lattice parameter
  of an ideal single crystal of naturally occurring Si in vacuum at
  22.5 °C, not Bohr radius `a0`, not classical electron radius `re`,
  and not a FormalClaim reconstructing `d220 * √8` from a live lookup.
  The {220} lattice spacing is a separately recommended hull and is not
  stored. The decade is `10^{19}`; `10^{18}` is the 10× trap. 2018 and
  2022 print the same last-digit `511`; the 2018 hull contains that
  centre. The ledger name is `a_Si`; `aSi`, `a-Si`, and `lattice_si`
  are not second names. `physis_model` `lattice_parameter_of_silicon()`
  Qty locksteps to the CODATA centre inside the hull. Adding `a_Si` to
  LEDGER changes the ledger bundle pin. Theories still evaluate with
  `f64` Qty. That is not a kernel proof, not Canonical, not P4. Encode
  pins unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified:
  `a_Si` hash
  8d9072d01e48779f7404ab918ba023a055f197cd2c8d9f19796431939f568344;
  node 786af851dcfebdab86280502344a362caf43e618690b85f0f3c699828c5bec29;
  ledger node
  58290942461f9847b63d0fbc97b2ead26afc2f8a6b13da64164ceeeb6a5493e7.
  `Eh_kg` hash
  fc95a867392143c42d4006b7b085cf529610c633533ddbe2066b92390535509f and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 hartree-kilogram relationship is a one-sigma SciInterval.**
  `physis-constants` versions `Eh_kg` as the CODATA 2018 one-sigma hull
  `4.8508702095432(94)×10^{-35}` kg from JPCRD 50, 033105 table XXXV
  (energy conversion factors). This is the recommended inverse listing
  of the kilogram-hartree companion pair, not a Ratio reciprocal of
  `kg_Eh`, not joule Hartree `Eh` inverted as a Ratio, not kg atomic
  mass constant `m_u`, and not a FormalClaim reconstructing `Eh / c²`
  from live lookups. A Ratio denominator `10^{48}` overflows `i128`, so
  the ledger stores a SciInterval of terminating decimals. Hertz-kilogram
  is not stored: `h/c²` overflows `i128`. Joule-kilogram is not stored:
  it is the reciprocal of ledger `kg_J`. The decade is `10^{48}`;
  `10^{47}` is the 10× trap. This is not the CODATA 2022 last-digit
  `5419` as the stored centre; the 2018 hull still contains that 2022
  centre. The ledger name is `Eh_kg`; `Ehkg`, `Eh-kg`, and `hartree_kg`
  are not second names. `physis_model` `hartree_in_kilogram()` Qty
  locksteps to the CODATA centre inside the hull. Adding `Eh_kg` to
  LEDGER changes the ledger bundle pin. Theories still evaluate with
  `f64` Qty. That is not a kernel proof, not Canonical, not P4. Encode
  pins unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified:
  `Eh_kg` hash
  fc95a867392143c42d4006b7b085cf529610c633533ddbe2066b92390535509f;
  node 84aed9832f9cce41e32c84f2b27f7c7f7301cb7642e442b9b15f9faf0439211b;
  ledger node
  c0eaa64ba7373318c13bc303113d266a6bf7cdde1e8405f7a6f3b82557f33c1f.
  `u_K` hash
  eceb5956e7b5b435a32c4d8b9b4a8f97cd96a597334919a667d804f13e1a495b and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 atomic mass unit-kelvin relationship is a one-sigma Interval.**
  `physis-constants` versions `u_K` as the CODATA 2018 one-sigma hull
  `1.08095401916(33)×10^{13}` K from JPCRD 50, 033105 table XXXV
  (energy conversion factors). This is the recommended inverse listing
  of the kelvin-atomic mass unit companion pair, not a Ratio reciprocal
  of `K_u`, not hartree-kelvin `Eh_K`, not kelvin-hartree `K_Eh`, not
  joule-kelvin `J_K`, not Boltzmann `k`, and not a FormalClaim
  reconstructing `c² m_u / k` from live lookups. Inverse meter-atomic
  mass unit cannot be named `m_u` and is not stored under a second name.
  The decade is `10^{2}`; `10^{1}` is the 10× trap (`σ = 3.3` is not an
  integer). This is not the CODATA 2022 last-digit `2067` as the stored
  centre; the 2018 hull excludes that 2022 centre. The ledger name is
  `u_K`; `uK`, `u-K`, and `amu_K` are not second names. `physis_model`
  `atomic_mass_unit_in_kelvin()` Qty locksteps to the CODATA centre
  inside the hull. Adding `u_K` to LEDGER changes the ledger bundle pin.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified:
  `u_K` hash
  eceb5956e7b5b435a32c4d8b9b4a8f97cd96a597334919a667d804f13e1a495b;
  node 4d6058f8883ec3209ea999ba52721aab5843129d8aa32ca20769e345dc1acbfa;
  ledger node
  1ba7978999d148662a71b48380d1b81bd026e9960a7fcd5bf9791db14dbad2d1.
  `u_m` hash
  3380878aa06e31c17144288a338b9b4614e21389fa72a9d2c16e1444c6b07035 and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 atomic mass unit-inverse meter relationship is a one-sigma Interval.**
  `physis-constants` versions `u_m` as the CODATA 2018 one-sigma hull
  `7.5130066104(23)×10^{14}` m^{-1} from JPCRD 50, 033105 table XXXV
  (energy conversion factors). This is the recommended atomic mass
  unit-inverse meter listing, not a Ratio reciprocal of unstored inverse
  meter-atomic mass unit, not Rydberg constant `Rinf`, not hartree-inverse
  meter `Eh_m`, not SI-exact hertz-inverse meter `Hz_m`, not kg atomic
  mass constant `m_u`, not atomic mass unit-hertz `u_Hz`, not electron
  volt-atomic mass unit `eV_u`, and not a FormalClaim reconstructing
  `c² m_u / (h c)` from live lookups. Inverse meter-atomic mass unit
  cannot be named `m_u` and is not stored under a second name. Atomic
  mass unit-kelvin is a later table row and is not stored. The decade is
  `10^{4}`; `10^{3}` is the 10× trap (`σ = 2.3` is not an integer). This
  is not the CODATA 2022 last-digit `66209` as the stored centre; the 2018
  hull excludes that 2022 centre. The ledger name is `u_m`; `um`, `u-m`,
  and `amu_m` are not second names. `physis_model`
  `atomic_mass_unit_in_inverse_meter()` Qty locksteps to the CODATA centre
  inside the hull. Adding `u_m` to LEDGER changes the ledger bundle pin.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified:
  `u_m` hash
  3380878aa06e31c17144288a338b9b4614e21389fa72a9d2c16e1444c6b07035;
  node acc94a83eafd38f422f07aa2387373cdfc0b98c33890b5bdce317ff4fe7c8651;
  ledger node
  2c9fbbec04f72c3c5abdf25b4e55be4fc71b192584f2973a7b11241ae488ed0c.
  `eV_u` hash
  39dba467459e1b7d4e44cd12512342f420c7f5d8b79b0f975af03b6d361225c6 and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 electron volt-atomic mass unit relationship is a one-sigma Interval.**
  `physis-constants` versions `eV_u` as the CODATA 2018 one-sigma hull
  `1.07354410233(32)×10^{-9}` u from JPCRD 50, 033105 table XXXV
  (energy conversion factors). This is the recommended inverse listing
  of the atomic mass unit-electron volt / `m_u_c2_MeV` companion pair,
  not a Ratio reciprocal of `m_u_c2_MeV`, not electron volt-hartree
  `eV_Eh`, not kelvin-atomic mass unit `K_u`, not the exact electronvolt
  Ratio `eV`, and not a FormalClaim reconstructing `e / (c² m_u)` from
  live lookups. Atomic mass unit-electron volt is not stored under a
  second name. The decade is `10^{20}`; `10^{19}` is the 10× trap
  (`σ = 3.2` is not an integer). This is not the CODATA 2022 last-digit
  `10083` as the stored centre; the 2018 hull excludes that 2022 centre.
  The ledger name is `eV_u`; `eVu`, `eV-u`, and `electronvolt_u` are not
  second names. `physis_model`
  `electron_volt_in_atomic_mass_unit()` Qty locksteps to the CODATA centre
  inside the hull. Adding `eV_u` to LEDGER changes the ledger bundle pin.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified:
  `eV_u` hash
  39dba467459e1b7d4e44cd12512342f420c7f5d8b79b0f975af03b6d361225c6;
  node af9b87a44fa433ffb7b314c4c4aeb3f2736ad27f928d258da05e338fca431b91;
  ledger node
  b2c28933c8b0a5413719bb3baac65d201e899973ed75d4aa12fbcd935ef61043.
  `K_u` hash
  92a82ff43fedced786dd0efca6fa5cddc57a33ca3cb083217ac936fcd2b72c5e and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 kelvin-atomic mass unit relationship is a one-sigma Interval.**
  `physis-constants` versions `K_u` as the CODATA 2018 one-sigma hull
  `9.2510873014(28)×10^{-14}` u from JPCRD 50, 033105 table XXXV
  (energy conversion factors). This is the recommended kelvin-atomic
  mass unit listing, not a Ratio reciprocal of unstored atomic mass
  unit-kelvin, not kelvin-hartree `K_Eh`, not joule-kelvin `J_K`, not
  Boltzmann `k`, not joule-atomic mass unit `J_u`, and not a FormalClaim
  reconstructing `k / (c² m_u)` from live lookups. Atomic mass
  unit-kelvin is not stored under a second name. The decade is `10^{24}`;
  `10^{23}` is the 10× trap (`σ = 2.8` is not an integer). This is not
  the CODATA 2022 last-digit `2884` as the stored centre; the 2018 hull
  excludes that 2022 centre. The ledger name is `K_u`; `Ku`, `K-u`, and
  `kelvin_u` are not second names. `physis_model`
  `kelvin_in_atomic_mass_unit()` Qty locksteps to the CODATA centre
  inside the hull. Adding `K_u` to LEDGER changes the ledger bundle pin.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified:
  `K_u` hash
  92a82ff43fedced786dd0efca6fa5cddc57a33ca3cb083217ac936fcd2b72c5e;
  node 6f2e5d8122826c006e5d64183b9035261fa51e1e6508ff2887a9e5348c33039b;
  ledger node
  d95f324909c9282766524b628dacba6013ffd7097c9e4da797adeade9c5b99ec.
  `J_u` hash
  b2f6eb6e00f483e99217fb38006ce9d8339ad91f1d6d6d9e0e38c35b03af4d40 and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 joule-atomic mass unit relationship is a one-sigma Interval.**
  `physis-constants` versions `J_u` as the CODATA 2018 one-sigma hull
  `6.7005352565(20)×10^{9}` u from JPCRD 50, 033105 table XXXV
  (energy conversion factors). This is the recommended inverse listing
  of the atomic mass constant energy-equivalent pair, not a Ratio
  reciprocal of `m_u_c2`, not kilogram-atomic mass unit `kg_u`, not
  joule-hartree `J_Eh`, not joule-electron volt `J_eV`, and not a
  FormalClaim reconstructing `1 / m_u_c2` from live lookups. Atomic mass
  unit-joule is `m_u_c2` and is not stored under a second name. The
  decade is `10^{1}`; `10^{0}` is the 10× trap (the printed centre is
  not an integer there). This is not the CODATA 2022 last-digit `2471`
  as the stored centre; the 2018 hull excludes that 2022 centre. The
  ledger name is `J_u`; `Ju`, `J-u`, and `joule_u` are not second names.
  `physis_model` `joule_in_atomic_mass_unit()` Qty locksteps to the
  CODATA centre inside the hull. Adding `J_u` to LEDGER changes the
  ledger bundle pin. Theories still evaluate with `f64` Qty. That is not
  a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified:
  `J_u` hash
  b2f6eb6e00f483e99217fb38006ce9d8339ad91f1d6d6d9e0e38c35b03af4d40;
  node c52249bcac19ccd303fbe29b8355da38e3ad5a2aef6bc179978f5d8d83563887;
  ledger node
  58cfa2ae32d8225885499e72aac0b5b47c26795b23555c907d9b08c49aa4fbf2.
  `kg_u` hash
  8d1f4f91a93aa7b56e3581d371e08e8eb0377c6e892fb24799ab341297d88bb5 and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 kilogram-atomic mass unit relationship is a one-sigma Interval.**
  `physis-constants` versions `kg_u` as the CODATA 2018 one-sigma hull
  `6.0221407621(18)×10^{26}` u from JPCRD 50, 033105 table XXXV
  (energy conversion factors). This is the recommended inverse listing
  of the kg atomic mass constant pair, not a Ratio reciprocal of `m_u`,
  not kilogram-hartree `kg_Eh`, not Avogadro `N_A`, not molar mass
  constant `M_u`, and not a FormalClaim reconstructing `1 / m_u` from
  live lookups. Atomic mass unit-kilogram is `m_u` and is not stored
  under a second name. The integer decade is `10^{16}` on the `10^{26}`
  form; `10^{17}` is the 10× trap (`σ = 1.8` is not an integer). This is
  not the CODATA 2022 last-digit `7537` as the stored centre; the 2018
  hull excludes that 2022 centre. The ledger name is `kg_u`; `kg_amu`,
  `kg-u`, and `kilogram_u` are not second names. `physis_model`
  `kilogram_in_atomic_mass_unit()` Qty locksteps to the CODATA centre
  inside the hull. Adding `kg_u` to LEDGER changes the ledger bundle
  pin. Theories still evaluate with `f64` Qty. That is not a kernel
  proof, not Canonical, not P4. Encode pins unchanged. Unique-vacuum
  graph id unchanged. P3N count stays 4. Verified:
  `kg_u` hash
  8d1f4f91a93aa7b56e3581d371e08e8eb0377c6e892fb24799ab341297d88bb5;
  node 512d569359b032f207037b2cf2727128ced6dfa813c61a56ca0c59e4db139dff;
  ledger node
  2af6c60370d905a9235742460a2a19e5fecbdf489c515c6cdb7b7707816f277f.
  `kg_Eh` hash
  0abb167c6721131a4310043821b59f97a18ac4a594c041de9b3101303bf8250a and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 kilogram-hartree relationship is a one-sigma Interval.**
  `physis-constants` versions `kg_Eh` as the CODATA 2018 one-sigma hull
  `2.0614857887409(40)×10^{34}` E_h from JPCRD 50, 033105 table XXXV
  (energy conversion factors). This is the recommended kilogram-hartree
  listing, not a Ratio reciprocal of unstored hartree-kilogram, not
  atomic mass unit-hartree `u_Eh`, not joule-hartree `J_Eh`, not
  kilogram-joule `kg_J`, not kg atomic mass constant `m_u`, and not a
  FormalClaim reconstructing `c² / Eh` from live lookups. Hartree-kilogram
  is not stored: Ratio scale overflows `i128`. The integer decade is
  `10^{21}` on the `10^{34}` form; `10^{22}` is the 10× trap (`σ = 4.0`
  is not an integer). This is not the CODATA 2022 last-digit `7415` as
  the stored centre; the 2018 hull still contains that 2022 centre. The
  ledger name is `kg_Eh`; `kgEh`, `kg-Eh`, and `kilogram_Eh` are not
  second names. `physis_model` `kilogram_in_hartree()` Qty locksteps to
  the CODATA centre inside the hull. Adding `kg_Eh` to LEDGER changes
  the ledger bundle pin. Theories still evaluate with `f64` Qty. That is
  not a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified:
  `kg_Eh` hash
  0abb167c6721131a4310043821b59f97a18ac4a594c041de9b3101303bf8250a;
  node d25f1232a25f8dafd253a2cc588d36f8f31844981dd8ccf1e54f9a75c95d3dc9;
  ledger node
  83b0e06e4a2c34ac0bff3221163a4db4284ba1e0424ccb0a21d8a62a01b712d8.
  `u_Eh` hash
  15c1d5e49dca9b631334cf8c46bd90a65e8ff6658fb7a64ba43a5a37862c60a1 and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 atomic mass unit-hartree relationship is a one-sigma Interval.**
  `physis-constants` versions `u_Eh` as the CODATA 2018 one-sigma hull
  `3.4231776874(10)×10^{7}` E_h from JPCRD 50, 033105 table XXXV
  (energy conversion factors). This is the recommended inverse listing
  of the hartree-atomic mass unit pair, not a Ratio reciprocal of
  `Eh_u`, not joule-hartree `J_Eh`, not kg atomic mass constant `m_u`,
  not atomic mass unit-hertz `u_Hz`, and not a FormalClaim reconstructing
  `c² m_u / Eh` from live lookups. Hartree-kilogram is not stored: Ratio
  scale overflows `i128`. Kilogram-hartree is not stored. The decade is
  `10^{3}`; `10^{2}` is the 10× trap (the printed centre is not an
  integer there). This is not the CODATA 2022 last-digit `6922` as the
  stored centre; the 2018 hull excludes that 2022 centre. The ledger
  name is `u_Eh`; `uEh`, `u-Eh`, and `amu_Eh` are not second names.
  `physis_model` `atomic_mass_unit_in_hartree()` Qty locksteps to the
  CODATA centre inside the hull. Adding `u_Eh` to LEDGER changes the
  ledger bundle pin. Theories still evaluate with `f64` Qty. That is
  not a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified:
  `u_Eh` hash
  15c1d5e49dca9b631334cf8c46bd90a65e8ff6658fb7a64ba43a5a37862c60a1;
  node 965c12163e67c74d6c8665e15dbef38db6ab88fc42720a075786c7d8af3f6726;
  ledger node
  71970f25b40a651d9b754fe21a35df99ebe9b8b17073aea2ee462742ed3c217a.
  `J_Eh` hash
  f922a6b2268fef1d8e0a56c71cfcdd7daee0359c8a1a3ad7e728d05b8e424ddd and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 joule-hartree relationship is a one-sigma Interval.**
  `physis-constants` versions `J_Eh` as the CODATA 2018 one-sigma hull
  `2.2937122783963(45)×10^{17}` E_h from JPCRD 50, 033105 table XXXV
  (energy conversion factors). This is the recommended inverse listing
  of the hartree-joule pair, not a Ratio reciprocal of `Eh`, not
  electron volt-hartree `eV_Eh`, not hartree energy in eV `Eh_eV`, not
  joule-electron volt `J_eV`, and not a FormalClaim reconstructing
  `1 / Eh` from live lookups. Inverse atomic-mass-unit-hartree is not
  stored. Hartree-kilogram is not stored: Ratio scale overflows
  `i128`. The integer decade is `10^{4}` on the `10^{17}` form; `10^{3}`
  is the 10× trap (`σ = 4.5` is not an integer). This is not the
  CODATA 2022 last-digit `3969` as the stored centre; the 2018 hull
  still contains that 2022 centre. The ledger name is `J_Eh`; `JEh`,
  `J-Eh`, and `joule_Eh` are not second names. `physis_model`
  `joule_in_hartree()` Qty locksteps to the CODATA centre inside the
  hull. Adding `J_Eh` to LEDGER changes the ledger bundle pin.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified: `J_Eh` hash
  f922a6b2268fef1d8e0a56c71cfcdd7daee0359c8a1a3ad7e728d05b8e424ddd;
  node 897c2126a3ef852a26f86bd5d39162b2d1ea47bb5875232e6ec15909626458b2;
  ledger node
  e90f7ee323c5bf0d74c8ce3d44a2f6016bd9dedda2d1a4c247925edf006fb4b8.
  `eV_Eh` hash
  8c15bdef7dbec61c106d7df00c024ac4aad0ff46fb280a6e950f11248024a201 and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 electron volt-hartree relationship is a one-sigma Interval.**
  `physis-constants` versions `eV_Eh` as the CODATA 2018 one-sigma hull
  `3.6749322175655(71)×10^{-2}` E_h from JPCRD 50, 033105 table XXXV
  (energy conversion factors). This is the recommended inverse listing
  of the hartree-electron volt pair, not a Ratio reciprocal of `Eh_eV`,
  not hartree-atomic mass unit `Eh_u`, not joule `Eh`, not SI-exact
  electronvolt `eV`, not joule-electron volt `J_eV`, and not a
  FormalClaim reconstructing `e / Eh` from live lookups. Inverse
  atomic-mass-unit-hartree is not stored. Hartree-kilogram is not
  stored: Ratio scale overflows `i128`. The decade is `10^{15}`;
  `10^{14}` is the 10× trap (`σ = 7.1` is not an integer). This is not
  the CODATA 2022 last-digit `5665` as the stored centre; the 2018 hull
  still contains that 2022 centre. The ledger name is `eV_Eh`; `eVEh`,
  `eV-Eh`, and `electron_volt_Eh` are not second names. `physis_model`
  `electron_volt_in_hartree()` Qty locksteps to the CODATA centre inside
  the hull. Adding `eV_Eh` to LEDGER changes the ledger bundle pin.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified: `eV_Eh` hash
  8c15bdef7dbec61c106d7df00c024ac4aad0ff46fb280a6e950f11248024a201;
  node a9e5d2c994add3d47fe7ad539d3417cebf9aa69aee2b2b10154d04190c80172e;
  ledger node
  df5da730bbdd833a26ed840b7126d9970e114e3f5ec772dde5c1927254c51332.
  `Eh_u` hash
  a3c699716e28cb116c1c86af4014cea3c65677566d268105e86fb13db75a1047 and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 hartree-atomic mass unit relationship is a one-sigma Interval.**
  `physis-constants` versions `Eh_u` as the CODATA 2018 one-sigma hull
  `2.92126232205(88)×10^{-8}` u from JPCRD 50, 033105 table XXXV
  (energy conversion factors). This is the next unique measured
  conversion hull, not kelvin-hartree `K_Eh`, not kg atomic mass
  constant `m_u`, not hertz-atomic mass unit `Hz_u`, not atomic mass
  unit-hertz `u_Hz`, not joule `Eh`, not the eV companion `Eh_eV`, and
  not a FormalClaim reconstructing `Eh / (c² m_u)` from live lookups.
  Inverse atomic-mass-unit-hartree is not stored. Hartree-kilogram is
  not stored: Ratio scale overflows `i128`. The decade is `10^{19}`;
  `10^{18}` is the 10× trap (`σ = 8.8` is not an integer). This is not
  the CODATA 2022 last-digit `1797` as the stored centre; the 2018 hull
  excludes that 2022 centre. The ledger name is `Eh_u`; `Ehu`, `Eh-u`,
  `hartree_u`, and `u_Eh` are not second names. `physis_model`
  `hartree_in_atomic_mass_unit()` Qty locksteps to the CODATA centre
  inside the hull. Adding `Eh_u` to LEDGER changes the ledger bundle
  pin. Theories still evaluate with `f64` Qty. That is not a kernel
  proof, not Canonical, not P4. Encode pins unchanged. Unique-vacuum
  graph id unchanged. P3N count stays 4. Verified: `Eh_u` hash
  a3c699716e28cb116c1c86af4014cea3c65677566d268105e86fb13db75a1047;
  node 2c8618557602554a8d101ef34a276b06bcc981cce6ba57d6d93f01de95d3f8a1;
  ledger node
  b647dc2b754be01df5596df66e77a3040dc65e8b4f0f8771e920ca22e16d9c62.
  `K_Eh` hash
  9f4581b1e00277c9a3df0c954203e107ae847ba3ddaf197df6636f02d8418aa9 and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 kelvin-hartree relationship is a one-sigma Interval.**
  `physis-constants` versions `K_Eh` as the CODATA 2018 one-sigma hull
  `3.1668115634556(61)×10^{-6}` E_h from JPCRD 50, 033105 table XXXV
  (energy conversion factors). This is the recommended inverse listing
  of the hartree-kelvin pair, not a Ratio reciprocal of `Eh_K`, not
  hertz-hartree `Hz_Eh`, not inverse meter-hartree `m_Eh`, not Boltzmann
  `k`, not joule-kelvin `J_K`, not joule `Eh`, and not a FormalClaim
  reconstructing `k / Eh` from live lookups. Hertz-kilogram is not
  stored: `h/c²` overflows `i128`. The decade is `10^{19}`; `10^{18}` is
  the 10× trap (`σ = 6.1` is not an integer). This is not the CODATA
  2022 last-digit `4564` as the stored centre; the 2018 hull still
  contains that 2022 centre. The ledger name is `K_Eh`; `KEh`, `K-Eh`,
  and `kelvin_Eh` are not second names. `physis_model`
  `kelvin_in_hartree()` Qty locksteps to the CODATA centre inside the
  hull. Adding `K_Eh` to LEDGER changes the ledger bundle pin. Theories
  still evaluate with `f64` Qty. That is not a kernel proof, not
  Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified: `K_Eh` hash
  9f4581b1e00277c9a3df0c954203e107ae847ba3ddaf197df6636f02d8418aa9;
  node 0bce613d552818e07fadd6e4d8aa5c2bb7e948243e48871a38848810befaabd0;
  ledger node
  8907119e7618e985e396435105ddaf498baf41ac53ef9ac304dc810fbbc1ab95.
  `Eh_K` hash
  6f0a8bf07131fca3dec89383496f280b6c1b93086c049e3466e8653c3cc45430 and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 hartree-kelvin relationship is a one-sigma Interval.**
  `physis-constants` versions `Eh_K` as the CODATA 2018 one-sigma hull
  `3.1577502480407(61)×10^{5}` K from JPCRD 50, 033105 table XXXV
  (energy conversion factors). This is the next unique measured
  conversion hull, not joule-kelvin `J_K`, not electron volt-kelvin
  `eV_K`, not hertz-kelvin `Hz_K`, not Boltzmann `k`, not joule `Eh`,
  and not a FormalClaim reconstructing `Eh / k` from live lookups. The
  inverse kelvin-hartree listing is not stored. Hertz-kilogram is not
  stored: `h/c²` overflows `i128`. The decade is `10^{8}`; `10^{7}` is
  the 10× trap (`σ = 6.1` is not an integer). This is not the CODATA
  2022 last-digit `0398` as the stored centre; the 2018 hull still
  contains that 2022 centre. The ledger name is `Eh_K`; `EhK`, `Eh-K`,
  `hartree_K`, and `K_Eh` are not second names. `physis_model`
  `hartree_in_kelvin()` Qty locksteps to the CODATA centre inside the
  hull. Adding `Eh_K` to LEDGER changes the ledger bundle pin. Theories
  still evaluate with `f64` Qty. That is not a kernel proof, not
  Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified: `Eh_K` hash
  6f0a8bf07131fca3dec89383496f280b6c1b93086c049e3466e8653c3cc45430;
  node 2dae2b24d50cbbb0e7f5e5d3afac7b5ca4dd1eddb828146c9f245ec4052fa147;
  ledger node
  bceb29e8c82a37839e1edc56e6d5317733144cc743a555ef42907a1bb4f9b96a.
  `m_Eh` hash
  ec9127261ac8d38bd117a7b563580e40cd7d6e6a893da1d89862739aee98861c and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 inverse meter-hartree relationship is a one-sigma Interval.**
  `physis-constants` versions `m_Eh` as the CODATA 2018 one-sigma hull
  `4.5563352529120(88)×10^{-8}` E_h from JPCRD 50, 033105 table XXXV
  (energy conversion factors). This is the recommended inverse listing
  of the hartree-inverse meter pair, not a Ratio reciprocal of `Eh_m`,
  not hertz-hartree `Hz_Eh`, not Rydberg `Rinf`, not SI-exact inverse
  meter-joule `m_J`, not joule `Eh`, and not a FormalClaim reconstructing
  `1 / (2 Rinf)` or `h c / Eh` from live lookups. Hertz-kilogram is not
  stored: `h/c²` overflows `i128`. The decade is `10^{21}`; `10^{20}` is
  the 10× trap (`σ = 8.8` is not an integer). This is not the CODATA
  2022 last-digit `9132` as the stored centre; the 2018 hull still
  contains that 2022 centre. The ledger name is `m_Eh`; `mEh`, `m-Eh`,
  and `inv_m_hartree` are not second names. `physis_model`
  `inverse_meter_in_hartree()` Qty locksteps to the CODATA centre inside
  the hull. Adding `m_Eh` to LEDGER changes the ledger bundle pin.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified: `m_Eh` hash
  ec9127261ac8d38bd117a7b563580e40cd7d6e6a893da1d89862739aee98861c;
  node 93026de8e32f434e1e60293935443a6cd2f35a77c74adaaf1accd2de43bdf6f3;
  ledger node
  fd65295dfb2e03997165e023cdbc0231a79d8db8c53f3454fec2c71f7b15a141.
  `Eh_m` hash
  9696fa523650b61199c2590965df2a7d36be3f0e319f17e3457b126192fb9796 and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 hartree-inverse meter relationship is a one-sigma Interval.**
  `physis-constants` versions `Eh_m` as the CODATA 2018 one-sigma hull
  `2.1947463136320(43)×10^{7}` m⁻¹ from JPCRD 50, 033105 table XXXV
  (energy conversion factors). This is the next unique measured
  conversion hull, not Rydberg `Rinf`, not SI-exact `Hz_m`, not
  hartree-hertz `Eh_Hz`, and not a FormalClaim reconstructing `2 Rinf`
  or `Eh / (h c)` from live lookups. The inverse meter-hartree listing
  is not stored. Hertz-kilogram is not stored: `h/c²` overflows `i128`.
  The decade is `10^{6}`; `10^{5}` is the 10× trap. This is not the
  CODATA 2022 last-digit `6314` as the stored centre; the 2018 hull
  still contains that 2022 centre. The ledger name is `Eh_m`; `Ehm`,
  `Eh-m`, `hartree_m`, and `m_Eh` are not second names. `physis_model`
  `hartree_in_inverse_meter()` Qty locksteps to the CODATA centre inside
  the hull. Adding `Eh_m` to LEDGER changes the ledger bundle pin.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified: `Eh_m` hash
  9696fa523650b61199c2590965df2a7d36be3f0e319f17e3457b126192fb9796;
  node 04a36e4495af1759d633be2c96749e3dcbcc6583b6cad1107128eea4746a56c1;
  ledger node
  7594a2c1705c0fb83cbf634d1dfb89aee4d459a74f062bea90aefd615fb00768.
  `Eh_Hz` hash
  9d3d98fd812ac48876d0130c20977f62d5ba838f61c6604868ad6ff050e8358a and
  `Rinf` hash
  fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090
  unchanged.

- **CODATA 2018 hartree-hertz relationship is a one-sigma Interval.**
  `physis-constants` versions `Eh_Hz` as the CODATA 2018 one-sigma hull
  `6.579683920502(13)×10^{15}` Hz from JPCRD 50, 033105 table XXXV
  (energy conversion factors). This is the recommended inverse listing
  of the hertz-hartree pair, not a Ratio reciprocal of `Hz_Eh`, not
  joule `Eh`, not `Eh_eV`, not Rydberg frequency `cRinf`, and not a
  FormalClaim reconstructing `Eh / h` from live lookups. Hertz-kilogram
  is not stored: `h/c²` overflows `i128`. The integer decade is `10^{3}`
  on the `10^{15}` form; `10^{2}` is the 10× trap. This is not the
  CODATA 2022 last-digit `4999` as the stored centre; the 2018 hull
  still contains that 2022 centre. The ledger name is `Eh_Hz`; `EhHz`,
  `Eh-Hz`, and `hartree_Hz` are not second names. `physis_model`
  `hartree_in_hertz()` Qty locksteps to the CODATA centre inside the
  hull. Adding `Eh_Hz` to LEDGER changes the ledger bundle pin. Theories
  still evaluate with `f64` Qty. That is not a kernel proof, not
  Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified: `Eh_Hz` hash
  9d3d98fd812ac48876d0130c20977f62d5ba838f61c6604868ad6ff050e8358a;
  node 1e4278900e9f26858816ca32a4ba47cb573ac63172ecd5585a92c455ed1fce70;
  ledger node
  7b9231f8691b9584779f3bab5276e978f1055e92f2d42624f90fd71cccad4fef.
  `Hz_Eh` hash
  1b7fb5c9bc08aea3d58daae4cbb5bb4d59fec7d1ffddea3db40acea388b29473 and
  `u_Hz` hash
  5e8602ee280fe9f615ca619efa4c69b84bdfac7e4601772fc5b9a3d3c7be866a
  unchanged.

- **CODATA 2018 atomic mass unit-hertz relationship is a one-sigma Interval.**
  `physis-constants` versions `u_Hz` as the CODATA 2018 one-sigma hull
  `2.25234271871(68)×10^{23}` Hz from JPCRD 50, 033105 table XXXV
  (energy conversion factors). This is the recommended inverse listing
  of the hertz-atomic mass unit pair, not a Ratio reciprocal of `Hz_u`,
  not kg `m_u`, not Rydberg frequency `cRinf`, and not a FormalClaim
  reconstructing `c² m_u / h` from live lookups. Inverse hartree-hertz
  is not stored. Hertz-kilogram is not stored: `h/c²` overflows `i128`.
  The integer decade is `10^{12}` on the `10^{23}` form; `10^{11}` is
  the 10× trap. This is not the CODATA 2022 last-digit `2185` as the
  stored centre; the 2018 hull excludes that 2022 centre. The ledger
  name is `u_Hz`; `uHz`, `u-Hz`, and `amu_Hz` are not second names.
  `physis_model` `atomic_mass_unit_in_hertz()` Qty locksteps to the
  CODATA centre inside the hull. Adding `u_Hz` to LEDGER changes the
  ledger bundle pin. Theories still evaluate with `f64` Qty. That is
  not a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified: `u_Hz`
  hash 5e8602ee280fe9f615ca619efa4c69b84bdfac7e4601772fc5b9a3d3c7be866a;
  node 42e50e73ed21eb10bb761a7ab85ea119d621797520e8138e69d473243c8667b7;
  ledger node
  bc70d0d1fbdd9ca8aff8ef9f1271f2ca2aa8ab4e2eea367d7d7718970a48977a.
  `Hz_u` hash
  8b4e79b5cf4df4a885eea94ff417860225cc1be92f4fad10a3cae632262b80dd and
  `m_u` hash
  fcefc139b85d5be198ab911fed33049d37641b01dcd0b87e12630db6dfd467d3
  unchanged.

- **CODATA 2018 hertz-atomic mass unit relationship is a one-sigma Interval.**
  `physis-constants` versions `Hz_u` as the CODATA 2018 one-sigma hull
  `4.4398216652(13)×10^{-24}` u from JPCRD 50, 033105 table XXXV
  (energy conversion factors). This is the next unique measured
  conversion hull, not a second name for kg `m_u` or `m_u_c2`, not
  electron mass in u `m_e_u`, not SI-exact `Hz_m`, not hertz-hartree
  `Hz_Eh`, and not a FormalClaim reconstructing `h / (c² m_u)` from live
  lookups. The inverse atomic mass unit-hertz listing is not stored.
  Inverse hartree-hertz is not stored. Hertz-kilogram is not stored:
  `h/c²` overflows `i128`. The decade is `10^{34}`; `10^{33}` is the 10×
  trap. This is not the CODATA 2022 last-digit `6590` as the stored
  centre; the 2018 hull excludes that 2022 centre. The ledger name is
  `Hz_u`; `Hzu`, `Hz-u`, `Hz_amu`, `u_Hz`, and `amu_Hz` are not second
  names. `physis_model` `hertz_in_atomic_mass_unit()` Qty locksteps to
  the CODATA centre inside the hull. Adding `Hz_u` to LEDGER changes
  the ledger bundle pin. Theories still evaluate with `f64` Qty. That
  is not a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified: `Hz_u`
  hash 8b4e79b5cf4df4a885eea94ff417860225cc1be92f4fad10a3cae632262b80dd;
  node 54e0629cd6e073f6b5ad822d4120afa624384b2a228a0b46657c5a0e59c52f26;
  ledger node
  ead7255b0c1c876ad6f97334923aa7547b37623d2b2ac379daf33c8cc13950cd.
  `Hz_Eh` hash
  1b7fb5c9bc08aea3d58daae4cbb5bb4d59fec7d1ffddea3db40acea388b29473 and
  `m_u` hash
  fcefc139b85d5be198ab911fed33049d37641b01dcd0b87e12630db6dfd467d3
  unchanged.

- **CODATA 2018 hertz-hartree relationship is a one-sigma Interval.**
  `physis-constants` versions `Hz_Eh` as the CODATA 2018 one-sigma hull
  `1.5198298460570(29)×10^{-16}` E_h from JPCRD 50, 033105 table XXXV
  (energy conversion factors). Unique SI-exact XXXV names that still
  fit `i128` are exhausted; this is the next unique measured conversion
  hull, not a second name for joule `Eh` or `Eh_eV`, not SI-exact
  `Hz_m`, not Rydberg frequency `cRinf`, and not a FormalClaim
  reconstructing `h / Eh` from live lookups. The inverse hartree-hertz
  listing is not stored. Hertz-kilogram is not stored: `h/c²` overflows
  `i128`. The decade is `10^{29}`; `10^{28}` is the 10× trap. This is
  not the CODATA 2022 last-digit `0574` as the stored centre; the 2018
  hull still contains that 2022 centre. The ledger name is `Hz_Eh`;
  `HzEh`, `Hz-Eh`, `Eh_Hz`, and `hartree_Hz` are not second names.
  `physis_model` `hertz_in_hartree()` Qty locksteps to the CODATA centre
  inside the hull. Adding `Hz_Eh` to LEDGER changes the ledger bundle
  pin. Theories still evaluate with `f64` Qty. That is not a kernel
  proof, not Canonical, not P4. Encode pins unchanged. Unique-vacuum
  graph id unchanged. P3N count stays 4. Verified: `Hz_Eh` hash
  1b7fb5c9bc08aea3d58daae4cbb5bb4d59fec7d1ffddea3db40acea388b29473;
  node f6ef614b65ab4700b144c8e49be1f26e3706003f3972cee939983d67a5ceade7;
  ledger node
  719bbd61864ac76dedfe6694f4473a966c705b8b10cdde783ed02af0c8620e19.
  `Hz_m` hash
  e41772022e94f6c2f45b5a728b61f8258bfac9269c643b84e39d530798bf8421 and
  `Eh` hash
  c4606c77e55763a397f633ef0f3ace1328d3e1e8781428baf97554c97f4fba5a
  unchanged.

- **CODATA 2018 hertz-inverse meter relationship is an exact Ratio.**
  `physis-constants` versions `Hz_m` as the SI-exact Ratio `1/c` =
  `1/299792458` m⁻¹ from JPCRD 50, 033105 table XXXV (energy conversion
  factors). The table prints `3.335 640 951… × 10^{-9}`; the ledger
  stores the full quotient. The reduced denominator keeps factors 7, 73,
  and 293339 (the odd primes in SI `c`), so this is not a terminating
  SciExact. This is not SI `c`, not inverse meter-joule `m_J`, not
  Boltzmann in inverse meter per kelvin `k_m`, not second radiation
  `c2`, not kilogram-joule `kg_J`, not an SI defining constant, and not
  a FormalClaim reconstructing `1/c` from live lookups. Inverse
  meter-hertz is not stored: it is SI `c`. Hertz-kilogram is not stored:
  `h/c²` overflows `i128`. Electron volt-inverse meter is not stored: it
  is the reciprocal of ledger `m_eV`. The ledger name is `Hz_m`; `Hzm`,
  `Hz-m`, `m_Hz`, and `1/c` are not second names. `physis_model`
  `hertz_in_inverse_meter()` Qty locksteps to `Ratio::to_f64` of that
  Ratio. Adding `Hz_m` to LEDGER changes the ledger bundle pin. Theories
  still evaluate with `f64` Qty. That is not a kernel proof, not
  Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified: `Hz_m` hash e41772022e94f6c2f45b5a728b61f8258bfac9269c643b84e39d530798bf8421;
  node 846bab463c0a51c951864c1258a50764889007111b9f543013c1f060ab63428e; ledger node 1d9fd5e457ac1e55a84d4a7629b89950e28566db69525d1d130caef9114add7a. `Hz_K` hash
  d45e08d73394c3c40e187d824d9b9a36160ab82a41ab6ede4f6869d55d772e0c and
  `c` hash 691eb73ea444f6d10fb223b999a1b37c0b67da92d51e43ca8bd8a6561785a3c1
  unchanged.

- **CODATA 2018 hertz-kelvin relationship is an exact Ratio.**
  `physis-constants` versions `Hz_K` as the SI-exact Ratio `h/k` =
  `132521403/2761298000000000000` K from JPCRD 50, 033105 table XXXV
  (energy conversion factors). The table prints
  `4.799 243 073… × 10^{-11}`; the ledger stores the full quotient.
  The reduced numerator keeps factors 3, 7, and 6310543 (the odd primes
  remaining in SI `h` after cancelling 5) and the reduced denominator
  keeps 73 and 18913 (the odd primes in SI `k`'s numerator), so this is
  not a terminating SciExact. This is not Boltzmann in Hz/K `k_Hz`, not
  SI `k`, not Planck `h`, not electron volt-kelvin `eV_K`, not an SI
  defining constant, and not a FormalClaim reconstructing `h/k` from
  live lookups. Hertz-electron volt is not stored: it is ledger
  `h_eVHz`. Hertz-joule is not stored: it is SI `h`. Inverse
  meter-kelvin is not stored: it is ledger `c2`. Electron volt-inverse
  meter is not stored: it is the reciprocal of ledger `m_eV`. Electron
  volt-kilogram is not stored: `e/c²` overflows `i128`. The ledger name
  is `Hz_K`; `HzK`, `Hz-K`, and `1/k_Hz` are not second names.
  `physis_model` `hertz_in_kelvin()` Qty locksteps to `Ratio::to_f64`
  of that Ratio. Adding `Hz_K` to LEDGER changes the ledger bundle pin.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified: `Hz_K` hash d45e08d73394c3c40e187d824d9b9a36160ab82a41ab6ede4f6869d55d772e0c;
  node 67ac7baad7e320e2795aa3aebdcd590ca7c3d2ef678458c9b866fe4491593dc8; ledger node 23523f11a62f8c8c03e42a5c1e84aefbe5daa7872752ba1782eed98ee3c37187. `eV_K` hash
  a2763a30976f052db834b3260d5399ef0553afaf42a848f22a174c70f0fbdad0 and
  `k_Hz` hash 4e53cf9938c70b39d13f107dc2c90be1486148fd1ebb585505e2e3b8637582bc
  unchanged.

- **CODATA 2018 electron volt-kelvin relationship is an exact Ratio.**
  `physis-constants` versions `eV_K` as the SI-exact Ratio `e/k` =
  `16021766340/1380649` K from JPCRD 50, 033105 table XXXV (energy
  conversion factors). The table prints `1.160 451 812… × 10^{4}`; the
  ledger stores the full quotient. The reduced denominator keeps
  factors 73 and 18913 (the odd primes in SI `k`'s numerator), so this
  is not a terminating SciExact. This is not Boltzmann in eV/K `k_eV`,
  not SI `k`, not joule-kelvin `J_K`, not BIPM electronvolt, not an SI
  defining constant, and not a FormalClaim reconstructing `e/k` from
  live lookups. Electron volt-inverse meter is not stored: it is the
  reciprocal of ledger `m_eV` from the same table. Electron
  volt-kilogram is not stored: `e/c²` overflows `i128`. The ledger name
  is `eV_K`; `eVK`, `eV-K`, and `1/k_eV` are not second names.
  `physis_model` `electron_volt_in_kelvin()` Qty locksteps to
  `Ratio::to_f64` of that Ratio. Adding `eV_K` to LEDGER changes the
  ledger bundle pin. Theories still evaluate with `f64` Qty. That is
  not a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified:
  `eV_K` hash a2763a30976f052db834b3260d5399ef0553afaf42a848f22a174c70f0fbdad0; node e5f5954239201da5cb06da674987a5a1aae6ee343db21317a92a75be82b5424f; ledger node
  c61c26280d504c53d8f7e1f1824ca27fd4d8ed1c6cfe19b87cb19a567a990957. `eV_Hz` hash d323fa85e441848900b49787ac5c058ae78c2bb59d3f46627a5c5ba68ef339d5
  and `k_eV` hash 6af2dc4a70fb23c2c85ff1537e3b6c4c32068d11cbe0a9abca6d651f5cdceed6
  unchanged.

- **CODATA 2018 electron volt-hertz relationship is an exact Ratio.**
  `physis-constants` versions `eV_Hz` as the SI-exact Ratio `e/h` =
  `10681177560000000000000/44173801` Hz from JPCRD 50, 033105
  table XXXV (energy conversion factors). The table prints
  `2.417 989 242… × 10^{14}`; the ledger stores the full quotient.
  The reduced denominator keeps factors 7 and 6310543 (the odd primes
  remaining in SI `h` after cancelling 3 and 5), so this is not a
  terminating SciExact. This is not Planck in eV/Hz `h_eVHz`, not SI
  Planck `h`, not `k_Hz`, not Josephson `KJ`, not BIPM electronvolt,
  not an SI defining constant, and not a FormalClaim reconstructing
  `e/h` from live lookups. Joule-hertz `J_Hz` is not stored: `10^{41}`
  overflows `i128`. Electron volt-inverse meter is not stored: it is
  the reciprocal of ledger `m_eV` from the same table. Electron
  volt-kilogram is not stored: `e/c²` overflows `i128`. The ledger name
  is `eV_Hz`; `eVHz`, `eV-Hz`, and `1/h_eVHz` are not second names.
  `physis_model` `electron_volt_in_hertz()` Qty locksteps to
  `Ratio::to_f64` of that Ratio. Adding `eV_Hz` to LEDGER changes the
  ledger bundle pin. Theories still evaluate with `f64` Qty. That is
  not a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified:
  `eV_Hz` hash d323fa85e441848900b49787ac5c058ae78c2bb59d3f46627a5c5ba68ef339d5; node d98001e49653a95a3bb1543e45fbb993cdecdfc79d3303e36cd0738fec4b09c6; ledger node
  75d3b67f7061b36f19f554be28e1a5621d845574f583fa54bb4c7264adebe018. `J_K` hash 294b1e620f7a8cacd4c276a74a0f43eba7e28c5969d83a1983a9df046dbb5f26
  and `h_eVHz` hash bc3fb761f651c84f885a4749f6099f7eef62b31467e2df1ca778aede28ce2964
  unchanged.

- **CODATA 2018 joule-kelvin relationship is an exact Ratio.**
  `physis-constants` versions `J_K` as the SI-exact Ratio `1/k` =
  `100000000000000000000000000000/1380649` K from JPCRD 50, 033105
  table XXXV (energy conversion factors). The table prints
  `7.242 970 516… × 10^{22}`; the ledger stores the full reciprocal.
  The reduced denominator keeps factors 73 and 18913 (the odd primes
  in SI `k`'s numerator), so this is not a terminating SciExact. This
  is not SI Boltzmann `k`, not `k_eV`, not `k_Hz`, not `k_m`, not
  second radiation `c2`, not Josephson `KJ`, not an SI defining
  constant, and not a FormalClaim reconstructing `1/k` from live
  lookups. Joule-hertz `J_Hz` is not stored: after reducing `10^{42}/h`
  the numerator is `10^{41}`, which overflows `i128`. The ledger name
  is `J_K`; `JK`, `J-K`, and `1/k` are not second names.
  `physis_model` `joule_in_kelvin()` Qty locksteps to
  `Ratio::to_f64` of that Ratio. Adding `J_K` to LEDGER changes the
  ledger bundle pin. Theories still evaluate with `f64` Qty. That is
  not a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified:
  `J_K` hash 294b1e620f7a8cacd4c276a74a0f43eba7e28c5969d83a1983a9df046dbb5f26; node f830b9ca4ce8383d69a8818628c66a968b039f3f30aa7a22d591a70f629031a4; ledger node
  580ba644fdc63e19b04b64a842d1dfa1728bb689f4d21f7c4802f59d0e59b3ff. `J_eV` hash b775a4c8372acd2d0ba110b108d3971cca85e2fa26a340d9a771522eefdd23e6
  unchanged.

- **CODATA 2018 joule-electron volt relationship is an exact Ratio.**
  `physis-constants` versions `J_eV` as the SI-exact Ratio `1/e` =
  `5000000000000000000000000000/801088317` eV from JPCRD 50, 033105
  table XXXV (energy conversion factors). The table prints
  `6.241 509 074… × 10^{18}`; the ledger stores the full reciprocal.
  The reduced denominator keeps factors 3, 19, 389, and 12043 (the
  same odd primes as `h_eVHz`, `RK`, `k_eV`, and `m_eV`, because all
  divide by `e`), so this is not a terminating SciExact. This is not
  BIPM/SI electronvolt `eV`, not inverse meter-electron volt `m_eV`,
  not `h_eVHz`, not `k_eV`, not an SI defining constant, and not a
  FormalClaim reconstructing `1/e` from live lookups. Kilogram-electron
  volt `kg_eV` is not stored: `c²/e` overflows `i128`. The ledger name
  is `J_eV`; `JeV`, `J-eV`, and `1/eV` are not second names.
  `physis_model` `joule_in_electronvolt()` Qty locksteps to
  `Ratio::to_f64` of that Ratio. Adding `J_eV` to LEDGER changes the
  ledger bundle pin. Theories still evaluate with `f64` Qty. That is
  not a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified:
  `J_eV` hash b775a4c8372acd2d0ba110b108d3971cca85e2fa26a340d9a771522eefdd23e6; node e140741974a7632531ce17777058213576c61bca740b33f5ceb671c5d7924260; ledger node
  4e527b60a8c70100d110cd50b272cc414b037ef4c556bc206982b524374a8349. `m_eV` hash
  12c1ae591caa23af86b67134aea0b013f49ede015195013efc865ec6b1340dff,
  `eV` hash d5514de9cbef3f6990067899529d34f20b4349ca3b20ba18c9a5932c8c6b6c0f,
  and `h_eVHz` hash bc3fb761f651c84f885a4749f6099f7eef62b31467e2df1ca778aede28ce2964
  unchanged.

- **CODATA 2018 natural unit of time is a one-sigma Interval.**
  `physis-constants` versions `nu_t` as the CODATA 2018 hull
  `1.28808866819(39)×10^{-21}` s from JPCRD 50, 033105 table XXXIV
  (Natural units). This is the printed recommended time hull listed as
  hbar/(m_e c^2), not Planck time, not atomic-unit time, not `nu_p`,
  not an SI defining Ratio, not a terminating SciExact, and not a
  FormalClaim reconstructing that quotient from live lookups. Atomic
  unit of time still cites hbar and is not stored. The ledger name is
  `nu_t`; `nut`, `nu_T`, `t_nu`, and `hbar_mec2` are not second names.
  Decade `10^{32}` (`10^{31}` is the 10x trap). This is not the CODATA
  2022 last-digit `66644` as the stored centre; the 2018 hull does not
  contain that 2022 centre. `physis_model` `natural_unit_of_time()` Qty
  locksteps to `Ratio::to_f64` of the recommended centre inside the
  hull. Adding `nu_t` to LEDGER changes the ledger bundle pin. Theories
  still evaluate with `f64` Qty. That is not a kernel proof, not
  Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified: `nu_t` hash eb796d28a29182f7211327fb07280d4b126e1ea949e19f0fb66575476dad80a5;
  node 2b50e30ecc5e7a5f893e51c261d0be10a8ffb8ec0976622f55c40787d527359b; ledger node e7e9283d3ae495029cb8ef17aaac87d726148e463b08f27bfe68be4499de569d. `au_p` hash
  f12708bfd5d8f16fd214f8c636edb390aa238130bc694a67e131d02cf08953b0
  unchanged.

- **CODATA 2018 atomic unit of momentum is a one-sigma Interval.**
  `physis-constants` versions `au_p` as the CODATA 2018 hull
  `1.99285191410(30)×10^{-24}` kg m s^{-1} from JPCRD 50, 033105 table XXXIV
  (Atomic units). This is the printed recommended momentum hull listed
  as hbar/a0, not natural-unit momentum `nu_p`, not kg `m_e`, not
  astronomical `au`, not `au_eps`, not an SI defining Ratio, not a
  terminating SciExact, and not a FormalClaim reconstructing that
  quotient from live lookups. Atomic unit of time still cites hbar and
  is not stored. The ledger name is `au_p`; `aup`, `au_P`, `p_au`, and
  `hbar_a0` are not second names. Decade `10^{35}` (`10^{34}` is the 10x
  trap). This is not the CODATA 2022 last-digit `91545` as the stored
  centre; the 2018 hull does not contain that 2022 centre. `physis_model`
  `atomic_unit_of_momentum()` Qty locksteps to `Ratio::to_f64` of the
  recommended centre inside the hull. Adding `au_p` to LEDGER changes
  the ledger bundle pin. Theories still evaluate with `f64` Qty. That
  is not a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified: `au_p`
  hash f12708bfd5d8f16fd214f8c636edb390aa238130bc694a67e131d02cf08953b0; node 7fbd9d2b00b5cc4e224ca5519bde5a590cb7aedc0fc0b1020c13d79140adb7fa; ledger
  node afbb7cac69bfa29ffd08f25ebe891f86d0e821343e7ef6b5a73306d487cc9b9f. `au_eps` hash
  82d6ab46e3ea0d80d80423bea9f6fa44a2e4e4004660c4b39b7a326deb95cd06
  unchanged.

- **CODATA 2018 atomic unit of permittivity is a one-sigma Interval.**
  `physis-constants` versions `au_eps` as the CODATA 2018 hull
  `1.11265005545(17)×10^{-10}` F m^{-1} from JPCRD 50, 033105 table XXXIV
  (Atomic units). This is the printed recommended permittivity hull
  listed as e^2/(a0 Eh), not vacuum `epsilon0`, not `Y0`, not
  astronomical `au`, not `au_v`, not an SI defining Ratio, not a
  terminating SciExact, and not a FormalClaim reconstructing that
  quotient or 4 pi epsilon0 from live lookups. Atomic unit of time
  still cites hbar and is not stored. The ledger name is `au_eps`;
  `aueps`, `au_e0`, `eps_au`, and `4pi_eps` are not second names.
  Decade `10^{21}` (`10^{20}` is the 10x trap). This is not the CODATA
  2022 last-digit `05620` as the stored centre; the 2018 hull does not
  contain that 2022 centre. `physis_model` `atomic_unit_of_permittivity()`
  Qty locksteps to `Ratio::to_f64` of the recommended centre inside the
  hull. Adding `au_eps` to LEDGER changes the ledger bundle pin. Theories
  still evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4. Verified: `au_eps` hash 82d6ab46e3ea0d80d80423bea9f6fa44a2e4e4004660c4b39b7a326deb95cd06; node 2a20f893702481b4fe30277d0cf5264392f1d4a10e4c2c27f314c4f023e72a0c; ledger
  node 86ab86113fd53d92b3b7d4871e916cbbfedf7a02401cffb0f6e449b5011a8ef8. `au_v` hash
  5b8ea3a788076f8159c305e8a97f324a80989c29b6c8fee509a3e15714417ba9
  unchanged.

- **CODATA 2018 atomic unit of velocity is a one-sigma Interval.**
  `physis-constants` versions `au_v` as the CODATA 2018 hull
  `2.18769126364(33)×10^{6}` m s^{-1} from JPCRD 50, 033105 table XXXIV
  (Atomic units). This is the printed recommended velocity hull, not SI
  `c`, not `alpha`, not astronomical `au`, not `au_F`, not an SI defining
  Ratio, not a terminating SciExact, and not a FormalClaim
  reconstructing `alpha c` from live lookups. Atomic unit of time still
  cites hbar and is not stored. The ledger name is `au_v`; `auv`,
  `au_V`, `alpha_c`, and `v_au` are not second names. Decade `10^{5}`
  (`10^{4}` is the 10x trap). This is not the CODATA 2022 last-digit
  `26216` as the stored centre; the 2018 hull does not contain that
  2022 centre. `physis_model` `atomic_unit_of_velocity()` Qty locksteps
  to `Ratio::to_f64` of the recommended centre inside the hull. Adding
  `au_v` to LEDGER changes the ledger bundle pin. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4. Verified: `au_v` hash 5b8ea3a788076f8159c305e8a97f324a80989c29b6c8fee509a3e15714417ba9; node 9593f075fe301bc700285a704f9f5645120b1a9716362313d1bb2c7a2eb743da; ledger
  node 347d6c99025a921417993a5d7ff53c0fab09bbb918a773b967d6b7a78162a2e0. `au_F` hash
  c5d5a76de08e86fe094f04e0619baf65676bf1710b24d59a1f6097d180966bc5
  unchanged.

- **CODATA 2018 atomic unit of force is a one-sigma Interval.**
  `physis-constants` versions `au_F` as the CODATA 2018 hull
  `8.2387234983(12)×10^{-8}` N from JPCRD 50, 033105 table XXXIV
  (Atomic units). This is the printed recommended force hull, not `Eh`,
  not `a0`, not astronomical `au`, not Faraday `NAe`, not natural-unit
  momentum, not an SI defining Ratio, not a terminating SciExact, and
  not a FormalClaim reconstructing `Eh / a0` from live lookups. Atomic
  unit of time still cites hbar and is not stored. Atomic unit of
  electric potential is a second name for `Eh_eV` and is not stored.
  The ledger name is `au_F`; `auf`, `auF`, `au_f`, `F_h`, and `Eh_a0`
  are not second names. Decade `10^{18}` (`10^{17}` is the 10x trap).
  This is not the CODATA 2022 last-digit `5038` as the stored centre;
  the 2018 hull does not contain that 2022 centre. `physis_model`
  `atomic_unit_of_force()` Qty locksteps to `Ratio::to_f64` of the
  recommended centre inside the hull. Adding `au_F` to LEDGER changes
  the ledger bundle pin. Theories still evaluate with `f64` Qty. That
  is not a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified:
  `au_F` hash c5d5a76de08e86fe094f04e0619baf65676bf1710b24d59a1f6097d180966bc5; node 955545825f8e26d3e1ab94353d171f043cd712a5f4d1be9c3493dda6fb9be701; ledger
  node c7d0daa297b67cd030730e4d72eb3bd3d64b036f2033a4db63d4b53c8093b841. `nu_p` hash
  e9b6fb4c3612f5c594b48eaa0227c83a9179843e431c0b373c5ef0c25d151daa
  unchanged.

- **CODATA 2018 natural unit of momentum is a one-sigma Interval.**
  `physis-constants` versions `nu_p` as the CODATA 2018 hull
  `2.73092453075(82)×10^{-22}` kg m s^{-1} from JPCRD 50, 033105
  table XXXIV (Natural units). This is the printed recommended
  momentum hull, not kg `m_e`, not joule `m_e_c2`, not MeV
  `m_e_c2_MeV`, not the MeV/c companion (a second name for that MeV
  row), not an SI defining Ratio, not a terminating SciExact, and not a
  FormalClaim reconstructing `m_e c` from live lookups. Natural unit of
  time still cites hbar and is not stored. The ledger name is `nu_p`;
  `nup`, `p_e`, and `pe` are not second names. Decade `10^{33}`
  (`10^{32}` is the 10x trap). This is not the CODATA 2022 last-digit
  `53446` as the stored centre; the 2018 hull does not contain that
  2022 centre. `physis_model` `natural_unit_of_momentum()` Qty locksteps
  to `Ratio::to_f64` of the recommended centre inside the hull. Adding
  `nu_p` to LEDGER changes the ledger bundle pin. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4. Verified: `nu_p` hash e9b6fb4c3612f5c594b48eaa0227c83a9179843e431c0b373c5ef0c25d151daa; node 3b73f5c2000281e562fdf12b64d9bf7b36515105b92a3fee0e31906d0b7cb043; ledger
  node 2a2c131d26579ddeaae34502eb111a6c820652a7d6abb3bdc4ec193a4e955e94. `m_e` hash
  961f48a48d6ca0a563b2ae710ebc3a908369fcfd0bd9d7c59ba2b663a6c275bd
  unchanged.

- **CODATA 2018 electron mass is a one-sigma SciInterval.**
  `physis-constants` versions `m_e` as the CODATA 2018 hull
  `9.1093837015(28)×10^{-31}` kg from JPCRD 50, 033105 table XXXI
  (Electron, e-). A Ratio denominator `10^{41}` overflows `i128`, so
  this is a `SciInterval` of terminating decimals, not an Interval of
  Ratio, not `m_e_u`, not joule `m_e_c2`, not MeV `m_e_c2_MeV`, not
  `M_e`, not `m_p`, not `m_u`, not atomic unit of mass under a second
  name, not an SI defining Ratio, not a terminating SciExact, and not a
  FormalClaim reconstructing `m_e_u * m_u` from live lookups. The
  ledger name is `m_e`. Quantum of circulation still cites pi hbar /
  m_e and is not stored. Decade `10^{41}` (`10^{40}` is the 10x trap).
  This is not the CODATA 2022 last-digit `7139` as the stored centre;
  the 2018 hull does not contain that 2022 centre.
  `physis_model` `electron_mass()` Qty locksteps to `SciExact::to_f64`
  of the recommended centre inside the hull. Adding `m_e` to LEDGER
  changes the ledger bundle pin. Theories still evaluate with `f64`
  Qty. That is not a kernel proof, not Canonical, not P4. Encode pins
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `m_e` hash 961f48a48d6ca0a563b2ae710ebc3a908369fcfd0bd9d7c59ba2b663a6c275bd; node 25b4aa84438ef207187bb7361b283608c551cf3a992d1bbf9aead9228af322aa; ledger
  node a6fb51023657427125615284015db8adf97725032138669a1dce85c88c668a14. `m_e_u` hash
  f0f8df112f644065bd01e2b903312927e5ef5d21d7792484aca0e8e7d29934d5,
  `m_e_c2` hash 935f7db8457024efb853abe9ee42c24e5efec5c4a831a94a308eb46efa6db0d5,
  and `m_e_c2_MeV` hash c076e0b56ff109b6f16661d0e7874b9f352cf53c4f862c38203c082a17d87f6b
  unchanged.

- **CODATA 2018 inverse meter-electron volt relationship is an exact Ratio.**
  `physis-constants` versions `m_eV` as the SI-exact Ratio `h c / e` =
  `6621486190496429/5340588780000000000000` eV from JPCRD 50, 033105
  table XXXV (energy conversion factors). The table prints
  `1.239 841 984… × 10^{-6}`; the ledger stores the full product. The
  reduced denominator keeps factors 2, 3, 5, 19, 389, and 12043 (the
  same odd primes as `h_eVHz`, `RK`, and `k_eV`, because all divide by
  `e`), so this is not a terminating SciExact. This is not SI
  joule-second `h`, not metre-per-second `c`, not electronvolt `eV`, not
  `h_eVHz`, not inverse meter-joule `m_J`, not an SI defining constant,
  and not a FormalClaim reconstructing `h * c / e` from live lookups.
  Inverse meter-hertz is `c` and is not stored as a second name. The
  ledger name is `m_eV`; `m` and `meV` are not second names.
  `physis_model` `inverse_meter_in_electronvolt()` Qty locksteps to
  `Ratio::to_f64` of that Ratio. Adding `m_eV` to LEDGER changes the
  ledger bundle pin. Theories still evaluate with `f64` Qty. That is
  not a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified:
  `m_eV` hash 12c1ae591caa23af86b67134aea0b013f49ede015195013efc865ec6b1340dff; node 57c79435bf8516f4b2e547b8b221b5dd003f18f701b6d01c44268682fa010d2d; ledger node
  00792ae6cad24021b4db3dd0505bd4b3d6c802a5ec9eda7457396a60199d805c. `m_J` hash
  1b1ae8c0a216320aad8dd8ac91944989de7278e7d89575e0824bcf77e764deeb,
  `h_eVHz` hash bc3fb761f651c84f885a4749f6099f7eef62b31467e2df1ca778aede28ce2964,
  and `eV` hash d5514de9cbef3f6990067899529d34f20b4349ca3b20ba18c9a5932c8c6b6c0f
  unchanged.

- **CODATA 2018 inverse meter-joule relationship is SciExact.**
  `physis-constants` versions `m_J` as the SI-exact SciExact `h c` =
  `19864458571489287e-41` J from JPCRD 50, 033105 table XXXV (energy
  conversion factors). The table prints `1.986 445 857… × 10^{-25}`;
  the ledger stores the full terminating decimal. After stripping one
  trailing ten the denominator is `10^{41}`, which overflows `i128`, so
  this is not a Ratio (same reason Planck `h` is SciExact). This is not
  SI joule-second `h`, not metre-per-second `c`, not kilogram-joule
  `kg_J`, not first-radiation `c1L`, not second radiation `c2`, not an
  SI defining constant, and not a FormalClaim reconstructing `h * c`
  from live lookups. The ledger name is `m_J`; `m` and `hc` are not
  second names. `physis_model` `inverse_meter_in_joule()` Qty locksteps
  to `SciExact::to_f64` of that decimal. Adding `m_J` to LEDGER changes
  the ledger bundle pin. Theories still evaluate with `f64` Qty. That
  is not a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified:
  `m_J` hash 1b1ae8c0a216320aad8dd8ac91944989de7278e7d89575e0824bcf77e764deeb; node 8a296c0621b84a7c7bbc3ad279afa1cb8c353b843806863981fed43ad1e1ffc3; ledger node
  1871021e6340088265e894dd8309480d891a33923cd9faad8238efa8cee23f8d. `h` hash
  50a96a8715769547a90cba69b0775d8892d79f2fa32465ad13a6d73b2d111eef,
  `c` hash 691eb73ea444f6d10fb223b999a1b37c0b67da92d51e43ca8bd8a6561785a3c1,
  `c1L` hash bb3b42d41a8d8ebc3191a2aa98d974733538eaba1098eb89a1574d228479249c,
  and `kg_J` hash a54eee6c8f3046f2c68745c29a8040b9b486fd013f72511e1d7372366a34bc7f
  unchanged.

- **CODATA 2018 kilogram-joule relationship is an exact Ratio.**
  `physis-constants` versions `kg_J` as the SI-exact integer Ratio
  `c*c` = `89875517873681764` J from JPCRD 50, 033105 table XXXV
  (energy conversion factors). The table prints `8.987 551 787… × 10^{16}`;
  the ledger stores the full integer. This is a terminating integer
  Ratio, stored as Ratio like `c`, not SciExact. This is not SI
  metre-per-second `c`, not second radiation `c2`, not `eV`, not `h`,
  not `h_eVHz`, not an SI defining constant, and not a FormalClaim
  reconstructing `c * c` from live lookups. The ledger name is `kg_J`;
  `kg` is not a second name. IEEE `f64` cannot hold all 57 bits of the
  integer; `physis_model` `kilogram_in_joule()` Qty locksteps to
  `Ratio::to_f64` of the integer. Adding `kg_J` to LEDGER changes the
  ledger bundle pin. Theories still evaluate with `f64` Qty. That is
  not a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified:
  `kg_J` hash a54eee6c8f3046f2c68745c29a8040b9b486fd013f72511e1d7372366a34bc7f; node c926a11668840ba7f9e461d93be0fed6b3cebea1ee13171ca1675e174bcf58ca; ledger node
  853e6788c749251ca35bbcb4de6a8e97838af050a8c26279a586dd127443243f. `c` hash
  691eb73ea444f6d10fb223b999a1b37c0b67da92d51e43ca8bd8a6561785a3c1,
  `c2` hash 9b6ced8d9873adf9b03f13f024d13b8c2ebc18e15e9f3d57fadf0eff0ed61cbc,
  `h_eVHz` hash bc3fb761f651c84f885a4749f6099f7eef62b31467e2df1ca778aede28ce2964,
  and `h` hash 50a96a8715769547a90cba69b0775d8892d79f2fa32465ad13a6d73b2d111eef
  unchanged.

- **CODATA 2018 Planck constant in eV/Hz is an exact Ratio.**
  `physis-constants` versions `h_eVHz` as the SI-exact Ratio `h/e` =
  `44173801/10681177560000000000000` eV Hz^{-1} from JPCRD 50, 033105
  table XXXI (UNIVERSAL). The table prints `4.135 667 696… × 10^{-15}`;
  the ledger stores the full product. The reduced denominator keeps
  factors 2, 3, 5, 19, 389, and 12043 (the same odd primes as `RK` and
  `k_eV`, because all divide by `e`), so this is not a terminating
  SciExact. This is not SI joule-second `h`, not elementary charge `e`,
  not `eV`, not `k_eV`, not `KJ`, not `RK`, not an SI defining constant,
  and not a FormalClaim reconstructing `h / e` or `2/KJ` from live
  lookups. The ledger name is `h_eVHz`. Reduced Planck in eV s is not
  stored. CODATA 2022 prints the same SI-exact ellipsis; there is no
  last-digit trap. `physis_model` `planck_in_ev_per_hz()` Qty locksteps
  to `Ratio::to_f64` of the reduced fraction. Adding `h_eVHz` to LEDGER
  changes the ledger bundle pin. Theories still evaluate with `f64` Qty.
  That is not a kernel proof, not Canonical, not P4. Encode pins
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `h_eVHz` hash bc3fb761f651c84f885a4749f6099f7eef62b31467e2df1ca778aede28ce2964; node a0f9b64c98abf1c57a0c07310fa6daea9d1613255bd4b2048eaf2f845b58e59d;
  ledger node f2168303d72f0eaa78d9cb766dbecc16ed607697a65acf9d3ef87d2ed7f232d4. `h` hash
  50a96a8715769547a90cba69b0775d8892d79f2fa32465ad13a6d73b2d111eef,
  `e` hash 412cb379a6bf6cca245ba89fc43539399942e644fa08000cd30bd1d9b25372a5,
  `k_m` hash 533849bdd6300f5e1e48545708d539f94017879558130c41b62dbe7f7742b501,
  `KJ` hash eb31c5b04ef0823e6e80a2921172c06fa6ef692e5a7700cb25d183b00a0090d2,
  and `RK` hash 2faf6f39986b543d3370bdd5764f0d075fa94a709d3fadd235ed82026fed2d46
  unchanged.

- **CODATA 2018 Boltzmann constant in inverse meter per kelvin is an exact Ratio.**
  `physis-constants` versions `k_m` as the SI-exact Ratio `k/(h c)` =
  `18913000000000000/272115870842319` m^{-1} K^{-1} from JPCRD 50, 033105
  table XXXI (PHYSICOCHEMICAL). The table prints `69.503 480 04…`; the
  ledger stores the full product. The reduced denominator keeps factors
  3, 7, 293339, and 6310543 (`k_Hz` keeps 3, 7, and 6310543; the extra
  293339 remains after cancelling factors in `h c`), so this is not a
  terminating SciExact. This is not SI joule-per-kelvin `k`, not Planck
  `h`, not speed of light `c`, not `k_Hz`, not `k_eV`, not second
  radiation `c2`, not an SI defining constant, and not a FormalClaim
  reconstructing `k / (h c)` or `1/c2` from live lookups. The ledger
  name is `k_m`; `k/hc` is a JPCRD alias and is not a second name.
  CODATA 2022 prints the same SI-exact ellipsis; there is no last-digit
  trap. `physis_model` `boltzmann_in_inverse_meter_per_kelvin()` Qty
  locksteps to `Ratio::to_f64` of the reduced fraction. Adding `k_m` to
  LEDGER changes the ledger bundle pin. Theories still evaluate with
  `f64` Qty. That is not a kernel proof, not Canonical, not P4. Encode
  pins unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `k_m` hash 533849bdd6300f5e1e48545708d539f94017879558130c41b62dbe7f7742b501; node 0af89015e9d3fa2b164a62f44ac054af48855f03338c51a424abbf9a126e0e93;
  ledger node 331c3a6e3acf0505ae4c9b71c7e0cb16747f32f48998845c8122274c7193bfc6. `k_Hz` hash
  4e53cf9938c70b39d13f107dc2c90be1486148fd1ebb585505e2e3b8637582bc,
  `c2` hash 9b6ced8d9873adf9b03f13f024d13b8c2ebc18e15e9f3d57fadf0eff0ed61cbc,
  `k` hash 0d6156b1dea5afb156a9bbdcde78709fcfbac53df129a27698ea3fd76e812061,
  `h` hash 50a96a8715769547a90cba69b0775d8892d79f2fa32465ad13a6d73b2d111eef,
  `c` hash 691eb73ea444f6d10fb223b999a1b37c0b67da92d51e43ca8bd8a6561785a3c1,
  and `NAe` hash dbc99e6a827156d94029a58f2134e4f2833c556723a089cc2a9e462f3fa76ba4
  unchanged.

- **CODATA 2018 Boltzmann constant in Hz/K is an exact Ratio.**
  `physis-constants` versions `k_Hz` as the SI-exact Ratio `k/h` =
  `2761298000000000000/132521403` Hz K^{-1} from JPCRD 50, 033105 table
  XXXI (PHYSICOCHEMICAL). The table prints `2.083 661 912… × 10^{10}`;
  the ledger stores the full product. The reduced denominator keeps
  factors 3, 7, and 6310543 (KJ keeps 7 and 6310543; the extra 3 remains
  after cancelling the factor 5 in `h`), so this is not a terminating
  SciExact. This is not SI joule-per-kelvin `k`, not Planck `h`, not
  `k_eV`, not `KJ`, not an SI defining constant, and not a FormalClaim
  reconstructing `k / h` from live lookups. The ledger name is `k_Hz`;
  `k/h` is not a second name. `k/hc` is not stored. CODATA 2022 prints
  the same SI-exact ellipsis; there is no last-digit trap. `physis_model`
  `boltzmann_in_hz_per_kelvin()` Qty locksteps to `Ratio::to_f64` of the
  reduced fraction. Adding `k_Hz` to LEDGER changes the ledger bundle
  pin. Theories still evaluate with `f64` Qty. That is not a kernel
  proof, not Canonical, not P4. Encode pins unchanged. Unique-vacuum
  graph id unchanged. P3N count stays 4.
  Verified: `k_Hz` hash 4e53cf9938c70b39d13f107dc2c90be1486148fd1ebb585505e2e3b8637582bc; node f77db081575132d0b379eddeba0f9e4642c54221ffbdcfa29d6147bb30d917c4;
  ledger node 9e72884125faa3c4895c3803a8ed4564fd7fb3f06aaec7cd5b3ec3b28c9dd3d4. `k_eV` hash
  6af2dc4a70fb23c2c85ff1537e3b6c4c32068d11cbe0a9abca6d651f5cdceed6,
  `k` hash 0d6156b1dea5afb156a9bbdcde78709fcfbac53df129a27698ea3fd76e812061,
  `h` hash 50a96a8715769547a90cba69b0775d8892d79f2fa32465ad13a6d73b2d111eef,
  `KJ` hash eb31c5b04ef0823e6e80a2921172c06fa6ef692e5a7700cb25d183b00a0090d2,
  and `NAe` hash dbc99e6a827156d94029a58f2134e4f2833c556723a089cc2a9e462f3fa76ba4
  unchanged.

- **CODATA 2018 Boltzmann constant in eV/K is an exact Ratio.**
  `physis-constants` versions `k_eV` as the SI-exact Ratio `k/e` =
  `1380649/16021766340` eV K^{-1} from JPCRD 50, 033105 table XXXI
  (PHYSICOCHEMICAL). The table prints `8.617 333 262… × 10^{-5}`; the
  ledger stores the full product. The reduced denominator keeps factors
  3, 19, 389, and 12043 (the same primes as `RK`, because both divide
  by `e`), so this is not a terminating SciExact. This is not SI
  joule-per-kelvin `k`, not `eV`, not `NAk`, not `NAe`, not `RK`, not
  an SI defining constant, and not a FormalClaim reconstructing `k / e`
  from live lookups. The ledger name is `k_eV`. Boltzmann in Hz/K and
  `k/hc` are not stored. CODATA 2022 prints the same SI-exact ellipsis;
  there is no last-digit trap. `physis_model`
  `boltzmann_in_ev_per_kelvin()` Qty locksteps to `Ratio::to_f64` of the
  reduced fraction. Adding `k_eV` to LEDGER changes the ledger bundle
  pin. Theories still evaluate with `f64` Qty. That is not a kernel
  proof, not Canonical, not P4. Encode pins unchanged. Unique-vacuum
  graph id unchanged. P3N count stays 4.
  Verified: `k_eV` hash 6af2dc4a70fb23c2c85ff1537e3b6c4c32068d11cbe0a9abca6d651f5cdceed6; node 5c6b4f0b437c59c5d406f7bccad046a88f962664f7bceb604cadc09f91d3dcea;
  ledger node c33f05b6a3385cb765aec7cf22e637a6b43e72d2cc9923ee11c0bad2032f24f0. `NAk` hash
  28c95a46c67bec666b887658cc44664000bf821eac09b9023cf401b89231efc3,
  `k` hash 0d6156b1dea5afb156a9bbdcde78709fcfbac53df129a27698ea3fd76e812061,
  `eV` hash d5514de9cbef3f6990067899529d34f20b4349ca3b20ba18c9a5932c8c6b6c0f,
  `NAe` hash dbc99e6a827156d94029a58f2134e4f2833c556723a089cc2a9e462f3fa76ba4,
  `c2` hash 9b6ced8d9873adf9b03f13f024d13b8c2ebc18e15e9f3d57fadf0eff0ed61cbc,
  and `RK` hash 2faf6f39986b543d3370bdd5764f0d075fa94a709d3fadd235ed82026fed2d46
  unchanged.

- **CODATA 2018 electron mass energy equivalent in MeV is a one-sigma Interval.**
  `physis-constants` versions `m_e_c2_MeV` as the CODATA 2018 hull
  `0.51099895000(15)` MeV from JPCRD 50, 033105 table XXXI
  (Electron, e-). This is the recommended printed companion, not joule
  `m_e_c2`, not kg `m_e`, not `m_e_u`, not `m_mu_c2_MeV`, not joule
  `Eh`, not SI-exact `eV`, not an SI defining Ratio, not a terminating
  SciExact, and not a FormalClaim reconstructing `m_e c^2 / e` from
  live lookups. The ledger name is `m_e_c2_MeV`. Quantum of circulation
  still cites pi hbar / m_e and is not stored. Decade `10^{11}`
  (`10^{10}` is the 10x trap). This is not the CODATA 2022 last-digit
  `95069` as the stored centre; the 2018 hull does not contain that
  2022 centre. `physis_model`
  `electron_mass_energy_equivalent_in_mev()` Qty locksteps to the
  recommended centre inside the hull. Adding `m_e_c2_MeV` to LEDGER
  changes the ledger bundle pin. Theories still evaluate with `f64`
  Qty. That is not a kernel proof, not Canonical, not P4. Encode pins
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `m_e_c2_MeV` hash c076e0b56ff109b6f16661d0e7874b9f352cf53c4f862c38203c082a17d87f6b; node 9ac6f08d59173db167d93f15a3d4265d26f966a92584a7053dd0dd8d62f3d33a;
  ledger node 666036c90f15b57467efbdcc25fd5f80012f4939c3fda679236e59bf7af38125. `m_e_c2` hash
  935f7db8457024efb853abe9ee42c24e5efec5c4a831a94a308eb46efa6db0d5
  and `m_mu_c2_MeV` hash 292b0524e0f1a160403fe1a2a4998cd4c2690f5d3b344a5f8ba31e9248be0416
  unchanged.

- **CODATA 2018 electron mass energy equivalent is a one-sigma Interval.**
  `physis-constants` versions `m_e_c2` as the CODATA 2018 hull
  `8.1871057769(25)×10^{-14}` J from JPCRD 50, 033105 table XXXI
  (Electron, e-). This is the recommended printed companion, not kg
  `m_e` (`10^{42}` overflows `i128`), not `m_e_u`, not the MeV
  conversion, not `m_mu_c2`, not joule `hcRinf`, not joule `Eh`, not
  SI-exact `eV`, not an SI defining Ratio, not a terminating SciExact,
  and not a FormalClaim reconstructing `m_e c^2` from live lookups.
  The ledger name is `m_e_c2`. Quantum of circulation still cites pi
  hbar / m_e and is not stored. Decade `10^{24}` (`10^{23}` is the 10x
  trap). This is not the CODATA 2022 last-digit `7880` as the stored
  centre; the 2018 hull does not contain that 2022 centre.
  `physis_model` `electron_mass_energy_equivalent()` Qty locksteps to
  the recommended centre inside the hull. Adding `m_e_c2` to LEDGER
  changes the ledger bundle pin. Theories still evaluate with `f64`
  Qty. That is not a kernel proof, not Canonical, not P4. Encode pins
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `m_e_c2` hash 935f7db8457024efb853abe9ee42c24e5efec5c4a831a94a308eb46efa6db0d5; node c78389974fdf97ffc418c12da149e820bc718c7ae103546574696f8d78a13c11; ledger
  node 11817bf9baaf470c2ea58aa2df951c549a31937cbd4695d4dfa6f985c591a1bf. `m_e_u` hash
  f0f8df112f644065bd01e2b903312927e5ef5d21d7792484aca0e8e7d29934d5
  and `m_mu_c2` hash d83a5072b8cb4fe869a2aa076aff9c4cd0d8f9f613a41eef52117124acde5854
  unchanged.

- **CODATA 2018 electron mass in u is a one-sigma Interval.**
  `physis-constants` versions `m_e_u` as the CODATA 2018 hull
  `5.48579909065(16)×10^{-4}` u from JPCRD 50, 033105 table XXXI
  (Electron, e-). This is the recommended printed companion, not kg
  `m_e` (`10^{42}` overflows `i128`), not `M_e`, not relative atomic
  mass under a second name, not `m_mu_u`, not `m_p_u`, not `m_u`, not
  an SI defining Ratio, not a terminating SciExact, and not a
  FormalClaim reconstructing `m_e / m_u` from live lookups. The ledger
  name is `m_e_u`. Quantum of circulation still cites pi hbar / m_e and
  is not stored. Decade `10^{15}` (`10^{14}` is the 10x trap). This is
  not the CODATA 2022 last-digit `0441` as the stored centre; the 2018
  hull does not contain that 2022 centre. `physis_model`
  `electron_mass_in_u()` Qty locksteps to the recommended centre inside
  the hull. Adding `m_e_u` to LEDGER changes the ledger bundle pin.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified: `m_e_u` hash f0f8df112f644065bd01e2b903312927e5ef5d21d7792484aca0e8e7d29934d5;
  node 682e118704f0d556c0c83465b59e59e27c0d59730ddaf82c430d3d5162066741; ledger node c8dfaf551acce175bd50f04cdae47e0b2248259e1b214e25d83d7d44ebdcfe96. `M_e` hash
  0a8b3285a4969854567b59db2ebf9449268df86ffdbb461e3b9c1db0955eb804
  and `hcRinf_eV` hash 5af1daec68e85898cf41891c1a8336b720457d3bc73c4384bafbb07b9b7050e6
  unchanged.

- **CODATA 2018 Rydberg energy equivalent in eV is a one-sigma Interval.**
  `physis-constants` versions `hcRinf_eV` as the CODATA 2018 hull
  `13.605693122994(26)` eV from JPCRD 50, 033105 table XXXI
  (ATOMIC AND NUCLEAR). This is the recommended printed companion, not
  joule `hcRinf`, not `Eh_eV`, not SI-exact `eV`, not an SI defining
  Ratio, not a terminating SciExact, not `hbar`, and not a FormalClaim
  reconstructing `hcRinf / e` from live lookups. The ledger name is
  `hcRinf_eV`. Quantum of circulation still cites pi hbar / m_e and is
  not stored. Decade `10^{12}` (`10^{11}` is the 10x trap). This is not
  the CODATA 2022 last-digit `2990` as the stored centre; the 2018 hull
  still contains that 2022 centre. Electron mass is still not stored
  (`10^{42}` overflows `i128`). `physis_model`
  `rydberg_energy_equivalent_in_ev()` Qty locksteps to the recommended
  centre inside the hull. Adding `hcRinf_eV` to LEDGER changes the
  ledger bundle pin. Theories still evaluate with `f64` Qty. That is
  not a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified:
  `hcRinf_eV` hash 5af1daec68e85898cf41891c1a8336b720457d3bc73c4384bafbb07b9b7050e6; node f5a202be0fdb836a2239c19a4da5300c1a153d4436db4d9151e4764eddf51168; ledger
  node 0751a3f5088d2d573c072633a58da1459f5c34e1df28ddc8b4abb5883fa7093c. `hcRinf` hash
  0d0308e874e54cb3d02570c972232b0d26c2d1d64b493880a1bb7ce4ff7827b2
  and `Eh_eV` hash 6be9d50e9eae8a9a943d69b81db60616a84e98bd294f2d85300ce39f9f4a6262
  unchanged.

- **CODATA 2018 Hartree energy in eV is a one-sigma Interval.**
  `physis-constants` versions `Eh_eV` as the CODATA 2018 hull
  `27.211386245988(53)` eV from JPCRD 50, 033105 table XXXI
  (ATOMIC AND NUCLEAR). This is the recommended printed companion, not
  joule `Eh`, not SI-exact `eV`, not Rydberg energy equivalent in eV,
  not the atomic unit of electric potential, not an SI defining Ratio,
  not a terminating SciExact, not `hbar`, and not a FormalClaim
  reconstructing `Eh / e` from live lookups. The ledger name is `Eh_eV`.
  Quantum of circulation still cites pi hbar / m_e and is not stored.
  Decade `10^{12}` (`10^{11}` is the 10x trap). This is not the CODATA
  2022 last-digit `5981` as the stored centre; the 2018 hull still
  contains that 2022 centre. Electron mass is still not stored (`10^{42}`
  overflows `i128`). `physis_model` `hartree_energy_in_ev()` Qty
  locksteps to the recommended centre inside the hull. Adding `Eh_eV`
  to LEDGER changes the ledger bundle pin. Theories still evaluate with
  `f64` Qty. That is not a kernel proof, not Canonical, not P4. Encode
  pins unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `Eh_eV` hash 6be9d50e9eae8a9a943d69b81db60616a84e98bd294f2d85300ce39f9f4a6262; node 6c1374381f3d7b77e32895687f2effdb0fc6f9e689f1ef2c93275fe6f45ef949; ledger
  node 3303e64de1a13819cb8ee7591fd827e0b5694425b2041a46d142d053ca4ba817. `Eh` hash
  c4606c77e55763a397f633ef0f3ace1328d3e1e8781428baf97554c97f4fba5a
  and prior gyromagnetic hashes unchanged.

- **CODATA 2018 shielded helion gyromagnetic ratio in MHz/T is a one-sigma Interval.**
  `physis-constants` versions `gamma0h_MHz` as the CODATA 2018 hull
  `32.43409942(38)` MHz T⁻¹ from JPCRD 50, 033105 table XXXI
  (Helion, h). This is the recommended printed companion, not s⁻¹ T⁻¹
  `gamma0h`, not `gamma0p_MHz`, not `gamma_e_MHz`, not `gamma_n_MHz`, not
  `muN_MHz`, not glossary `g0p`, not an SI defining Ratio, not a
  terminating SciExact, not `hbar`, and not a FormalClaim reconstructing
  `gamma0h / 2π` from live lookups. NIST lists MHz T⁻¹, not Hz T⁻¹.
  The ledger name is `gamma0h_MHz`. Glossary `g0p` is still skipped.
  Decade `10^{8}` (`10^{7}` is the 10× trap). This is not the CODATA
  2022 last-digit `033`; the 2018 hull excludes that 2022 centre.
  Table XXXI recommended printed gyromagnetic hulls are now stored.
  Electron mass is still not stored (`10^{42}` overflows `i128`).
  `physis_model` `shielded_helion_gyromagnetic_ratio_in_mhz_per_tesla()`
  Qty locksteps to the recommended centre inside the hull. Adding
  `gamma0h_MHz` to LEDGER changes the ledger bundle pin. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4. Verified: `gamma0h_MHz` hash 222550d6fcbe1f85109b0d4fb4e6e9d4529a471976a1f24587a3cc29fac5f6ac; node
  ea16a9b399f93c1d2c29197dba77ad96c1b201cbc041dca6995984f7a0986ec1; ledger node 1bf1bde6a7e28c328be227bdaf31a9fcf64c86accf132e993e683d0636eab6ec. `gamma0h`
  hash d0d76042a7c3a216840e099b7c709a90930cc9582a3e194e091bf38703f2840a
  and prior gyromagnetic hashes unchanged.

- **CODATA 2018 shielded helion gyromagnetic ratio is a one-sigma Interval.**
  `physis-constants` versions `gamma0h` as the CODATA 2018 hull
  `2.037894569(24)×10^{8}` s⁻¹ T⁻¹ from JPCRD 50, 033105 table XXXI
  (Helion, h). This is the recommended printed hull for the helion in a
  spherical gas sample at 25 °C, not `gamma0p`, not `gamma_e`, not
  `mu0h`, not `gh`, not an SI defining Ratio, not a terminating
  SciExact, not `hbar`, and not a FormalClaim reconstructing
  `2 |μ′h| / ℏ` from live lookups. The printed formula cites ħ; the
  reconstruction is unused. The ledger name is `gamma0h`. Shielded
  helion gyromagnetic ratio in MHz/T is a later row. Decade `10^{1}`
  (`10^{0}` is the 10× trap). This is not the CODATA 2022 last-digit
  `6078`; the 2018 hull excludes that 2022 centre. Electron mass is
  still not stored (`10^{42}` overflows `i128`). `physis_model`
  `shielded_helion_gyromagnetic_ratio()` Qty locksteps to the
  recommended centre inside the hull. Adding `gamma0h` to LEDGER
  changes the ledger bundle pin. Theories still evaluate with `f64` Qty.
  That is not a kernel proof, not Canonical, not P4. Encode pins
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `gamma0h` hash d0d76042a7c3a216840e099b7c709a90930cc9582a3e194e091bf38703f2840a; node f4bd91931b09ce44201f11f82d8212d6789ef070780322823cf3e813d8bfecd2;
  ledger node 12bca5650f162364de9c3b4da1b218fdd87eba003395b6dbb139a571e1b50e4b. `gamma_e_MHz` hash
  4467f4343ca683946219dde053497c42e8da08518663051808ffe7529630eda0
  and prior gyromagnetic hashes unchanged.

- **CODATA 2018 electron gyromagnetic ratio in MHz/T is a one-sigma Interval.**
  `physis-constants` versions `gamma_e_MHz` as the CODATA 2018 hull
  `28024.9514242(85)` MHz T⁻¹ from JPCRD 50, 033105 table XXXI
  (Electron, e-). This is the recommended printed companion, not s⁻¹ T⁻¹
  `gamma_e`, not `gamma_p_MHz`, not `gamma_n_MHz`, not `gamma0p_MHz`, not
  `muN_MHz`, not `ge`, not an SI defining Ratio, not a terminating
  SciExact, not `hbar`, and not a FormalClaim reconstructing
  `gamma_e / 2π` from live lookups. NIST lists MHz T⁻¹, not Hz T⁻¹.
  The ledger name is `gamma_e_MHz`. Helion gyromagnetic `gamma0h` still
  cites ħ and is not stored. Decade `10^{7}` (`10^{6}` is the 10× trap).
  This is not the CODATA 2022 last-digit `3861`; the 2018 hull excludes
  that 2022 centre. Electron mass is still not stored (`10^{42}`
  overflows `i128`). `physis_model`
  `electron_gyromagnetic_ratio_in_mhz_per_tesla()` Qty locksteps to the
  recommended centre inside the hull. Adding `gamma_e_MHz` to LEDGER
  changes the ledger bundle pin. Theories still evaluate with `f64` Qty.
  That is not a kernel proof, not Canonical, not P4. Encode pins
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `gamma_e_MHz` hash 4467f4343ca683946219dde053497c42e8da08518663051808ffe7529630eda0; node
  a2a19a2dbfc04e754e457c06a5f84f67a7dc185a5a1dc63634ef97f75700f243; ledger node ad4c9f69bf42589bdb5ca036d0f0c3e183116c08591a2326d64d70e95729d60a. `gamma_e`
  hash ebc106324ea058d91d5790627b8c24a19ea6a9865e2fe273b07239771454e7c9
  and prior gyromagnetic hashes unchanged.

- **CODATA 2018 electron gyromagnetic ratio is a one-sigma Interval.**
  `physis-constants` versions `gamma_e` as the CODATA 2018 hull
  `1.76085963023(53)×10^{11}` s⁻¹ T⁻¹ from JPCRD 50, 033105 table XXXI
  (Electron, e-). This is the recommended printed hull, not g-factor
  `ge`, not `mu_e`, not `gamma_p`, not `gamma_n`, not `e_mp`, not an SI
  defining Ratio, not a terminating SciExact, not `hbar`, and not a
  FormalClaim reconstructing `2 |μe| / ℏ` from live lookups. The printed
  formula cites ħ; the reconstruction is unused. The ledger name is
  `gamma_e`. Electron gyromagnetic ratio in MHz/T is a later row. Helion
  gyromagnetic `gamma0h` still cites ħ and is not stored. Decade `10^{0}`
  (`10^{1}` is the 10× trap). This is not the CODATA 2022 last-digit
  `62784`; the 2018 hull excludes that 2022 centre. Electron mass is
  still not stored (`10^{42}` overflows `i128`). `physis_model`
  `electron_gyromagnetic_ratio()` Qty locksteps to the recommended
  centre inside the hull. Adding `gamma_e` to LEDGER changes the ledger
  bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`,
  `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`,
  `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`,
  `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`,
  `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, `KJ`, `RK`, `muB`, `muB_eV`, `muB_Hz`, `muB_m`, `muB_K`,
  `muN`, `muN_eV`, `muN_m`, `muN_K`, `muN_MHz`, `gamma_p`,
  `gamma_p_MHz`, `gamma0p`, `gamma0p_MHz`, `gamma_n`, and
  `gamma_n_MHz` hashes are unchanged. Theories still evaluate with
  `f64` Qty. That is not a kernel proof, not Canonical, not P4. Encode
  pins unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `gamma_e` hash ebc106324ea058d91d5790627b8c24a19ea6a9865e2fe273b07239771454e7c9; node 8d3e3aeab146c0e8e7ef31e521dad0b8fba6385f1c496900f93d17d45071c788;
  ledger node 4f2d73b8d7b40adf70a91256985c10e81bb813888e01c3c7103310d7d6650ab6. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`,
  `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, `KJ`, `RK`, `muB`, `muB_eV`, `muB_Hz`, `muB_m`, `muB_K`,
  `muN`, `muN_eV`, `muN_m`, `muN_K`, `muN_MHz`, `gamma_p`,
  `gamma_p_MHz`, `gamma0p`, `gamma0p_MHz`, `gamma_n`, and
  `gamma_n_MHz` hashes and nodes unchanged.

- **CODATA 2018 neutron gyromagnetic ratio in MHz/T is a one-sigma Interval.**
  `physis-constants` versions `gamma_n_MHz` as the CODATA 2018 hull
  `29.1646931(69)` MHz T⁻¹ from JPCRD 50, 033105 table XXXI
  (Neutron, n). This is the recommended printed companion, not s⁻¹ T⁻¹
  `gamma_n`, not `gamma_p_MHz`, not `gamma0p_MHz`, not `muN_MHz`, not
  `muB_Hz`, not `gn`, not an SI defining Ratio, not a terminating
  SciExact, not `hbar`, and not a FormalClaim reconstructing
  `gamma_n / 2π` from live lookups. NIST lists MHz T⁻¹, not Hz T⁻¹.
  The ledger name is `gamma_n_MHz`. Helion gyromagnetic `gamma0h`
  still cites ħ and is not stored. Decade `10^{7}` (`10^{6}` is the
  10× trap). This is not the CODATA 2022 last-digit `6935` as the
  stored centre; the 2018 hull still contains that 2022 centre.
  Electron mass is still not stored (`10^{42}` overflows `i128`).
  `physis_model` `neutron_gyromagnetic_ratio_in_mhz_per_tesla()` Qty
  locksteps to the recommended centre inside the hull. Adding
  `gamma_n_MHz` to LEDGER changes the ledger bundle pin. The `G`,
  `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`,
  `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`,
  `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, `KJ`, `RK`, `muB`, `muB_eV`, `muB_Hz`, `muB_m`, `muB_K`,
  `muN`, `muN_eV`, `muN_m`, `muN_K`, `muN_MHz`, `gamma_p`,
  `gamma_p_MHz`, `gamma0p`, `gamma0p_MHz`, and `gamma_n` hashes are
  unchanged. Theories still evaluate with `f64` Qty. That is not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified:
  `gamma_n_MHz` hash afb240ddc4bf6fa45a44eb869268021801762dd70fbaa21d23e5fbc3669049a7; node 2ab81c06d7033e1ee4d83bc510c77355be587dae3c270af8941ba12a568d8608;
  ledger node 780105b74cec1ba83b5985e416356e56e275a11175b9a2f0696ab6615e57aabf. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`,
  `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, `KJ`, `RK`, `muB`, `muB_eV`, `muB_Hz`, `muB_m`, `muB_K`,
  `muN`, `muN_eV`, `muN_m`, `muN_K`, `muN_MHz`, `gamma_p`,
  `gamma_p_MHz`, `gamma0p`, `gamma0p_MHz`, and `gamma_n` hashes and
  nodes unchanged.

- **CODATA 2018 neutron gyromagnetic ratio is a one-sigma Interval.**
  `physis-constants` versions `gamma_n` as the CODATA 2018 hull
  `1.83247171(43)×10^{8}` s⁻¹ T⁻¹ from JPCRD 50, 033105 table XXXI
  (Neutron, n). This is the recommended printed hull, not dimensionless
  `gn`, not s⁻¹ T⁻¹ `gamma_p`, not `gamma0p`, not `gamma0p_MHz`, not
  `mu_n`, not an SI defining Ratio, not a terminating SciExact, not
  `hbar`, and not a FormalClaim reconstructing `2 μn / ℏ` from live
  lookups. The printed formula cites ħ; the reconstruction is unused.
  The ledger name is `gamma_n`. Neutron gyromagnetic ratio in MHz/T is
  a later row and is not stored. Helion gyromagnetic `gamma0h` still
  cites ħ and is not stored. Decade `10^{0}` (`10^{1}` is the 10× trap).
  This is not the CODATA 2022 last-digit `74` as the stored centre; the
  2018 hull still contains that 2022 centre. Electron mass is still not
  stored (`10^{42}` overflows `i128`). `physis_model`
  `neutron_gyromagnetic_ratio()` Qty locksteps to the recommended
  centre inside the hull. Adding `gamma_n` to LEDGER changes the ledger
  bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`,
  `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`,
  `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`,
  `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`,
  `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, `KJ`, `RK`, `muB`, `muB_eV`, `muB_Hz`, `muB_m`, `muB_K`,
  `muN`, `muN_eV`, `muN_m`, `muN_K`, `muN_MHz`, `gamma_p`,
  `gamma_p_MHz`, `gamma0p`, and `gamma0p_MHz` hashes are unchanged.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified: `gamma_n` hash cc9cc1b05368ff576b267d4475acaf2b2fb98fda286dc4a8e1f53556e4841914;
  node 26593240a293a2a081e5f473eb3f2befbbc559fade13a6761e8b9022c8fae84f; ledger node a7e09f674ac8f08fe05e79fdbab5bdc9578300d026083aeebc099df12af04ed2. `G`,
  `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`,
  `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`,
  `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`,
  `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`, `rp`,
  `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`,
  `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`,
  `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`,
  `gd`, `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`,
  `m_t_c2`, `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`,
  `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`, `m_h_u`, `m_h_c2`,
  `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`, `mu_h_muB`,
  `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`, `mu0h_mup`,
  `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, `KJ`, `RK`, `muB`, `muB_eV`, `muB_Hz`, `muB_m`, `muB_K`,
  `muN`, `muN_eV`, `muN_m`, `muN_K`, `muN_MHz`, `gamma_p`,
  `gamma_p_MHz`, `gamma0p`, and `gamma0p_MHz` hashes and nodes unchanged.

- **CODATA 2018 shielded proton gyromagnetic ratio in MHz/T is a one-sigma Interval.**
  `physis-constants` versions `gamma0p_MHz` as the CODATA 2018 hull
  `42.57638474(46)` MHz T⁻¹ from JPCRD 50, 033105 table XXXI
  (Proton, p) for the proton in spherical H2O at 25 °C. This is the
  recommended printed companion, not s⁻¹ T⁻¹ `gamma0p`, not free
  `gamma_p_MHz`, not `muN_MHz`, not `muB_Hz`, not glossary `g0p`, not
  an SI defining Ratio, not a terminating SciExact, not `hbar`, and
  not a FormalClaim reconstructing `gamma0p / 2π` from live lookups.
  NIST lists MHz T⁻¹, not Hz T⁻¹. The ledger name is `gamma0p_MHz`.
  Neutron gyromagnetic ratio is a later row and is not stored. Helion
  gyromagnetic `gamma0h` still cites ħ and is not stored. Decade
  `10^{8}` (`10^{7}` is the 10× trap). This is not the CODATA 2022
  last-digit `543`. Electron mass is still not stored (`10^{42}`
  overflows `i128`). `physis_model`
  `shielded_proton_gyromagnetic_ratio_in_mhz_per_tesla()` Qty locksteps
  to the recommended centre inside the hull. Adding `gamma0p_MHz` to
  LEDGER changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`,
  `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, `KJ`, `RK`, `muB`, `muB_eV`, `muB_Hz`, `muB_m`, `muB_K`,
  `muN`, `muN_eV`, `muN_m`, `muN_K`, `muN_MHz`, `gamma_p`,
  `gamma_p_MHz`, and `gamma0p` hashes are unchanged. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged.
  P3N count stays 4. Verified: `gamma0p_MHz` hash 0a531c484802446cb1ed9633e0d097ccdf86fdae629fc24a8417da5ffc0f1c38;
  node 793d27d5acfe59c2cd5c585713e1534f9bbcae14f04a025dc62b152798b639b1; ledger node 128c3050f82f123d9f23cbc57c85cf8d59bdf56b72eb123c20396e36afaad94a. `G`,
  `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`,
  `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`,
  `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`,
  `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`, `rp`,
  `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`,
  `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`,
  `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`,
  `gd`, `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`,
  `m_t_c2`, `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`,
  `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`, `m_h_u`, `m_h_c2`,
  `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`, `mu_h_muB`,
  `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`, `mu0h_mup`,
  `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, `KJ`, `RK`, `muB`, `muB_eV`, `muB_Hz`, `muB_m`, `muB_K`,
  `muN`, `muN_eV`, `muN_m`, `muN_K`, `muN_MHz`, `gamma_p`,
  `gamma_p_MHz`, and `gamma0p` hashes and nodes unchanged.

- **CODATA 2018 shielded proton gyromagnetic ratio is a one-sigma Interval.**
  `physis-constants` versions `gamma0p` as the CODATA 2018 hull
  `2.675153151(29)×10^8` s⁻¹ T⁻¹ from JPCRD 50, 033105 table XXXI
  (Proton, p) for the proton in spherical H2O at 25 °C. This is the
  recommended printed hull, not free `gamma_p`, not MHz T⁻¹
  `gamma_p_MHz`, not `gp`, not `mu0p`, not glossary `g0p`, not an SI
  defining Ratio, not a terminating SciExact, not `hbar`, and not a
  FormalClaim reconstructing `2 μ′p/ℏ` from live lookups. The printed
  formula cites ħ; the reconstruction is unused. The ledger name is
  `gamma0p`. Shielded proton gyromagnetic ratio in MHz/T is a later row
  and is not stored. Helion gyromagnetic `gamma0h` still cites ħ and is
  not stored. Decade `10^{1}` (`10^{0}` is the 10× trap). This is not
  the CODATA 2022 last-digit `194`. Electron mass is still not stored
  (`10^{42}` overflows `i128`). `physis_model`
  `shielded_proton_gyromagnetic_ratio()` Qty locksteps to the
  recommended centre inside the hull. Adding `gamma0p` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, `KJ`, `RK`, `muB`, `muB_eV`, `muB_Hz`, `muB_m`, `muB_K`,
  `muN`, `muN_eV`, `muN_m`, `muN_K`, `muN_MHz`, `gamma_p`, and `gamma_p_MHz` hashes are unchanged. Theories still evaluate with
  `f64` Qty. That is not a kernel proof, not Canonical, not P4. Encode
  pins unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `gamma0p` hash ad17e3a83ff5bcbb8f9cb82c9065a16a82caf68df4f48c901c86b4f62a633cbc; node b5ab615ca36f3799a21c3574ba8d45c12142ec93172797980396ffa02d2f1963; ledger node
  dbb12d9624f3b0195db33ffadde179c14a9d2dc89ffcf815e8865bb95d73da9d. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`,
  `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`,
  `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`,
  `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`,
  `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, `KJ`, `RK`, `muB`, `muB_eV`, `muB_Hz`, `muB_m`, `muB_K`,
  `muN`, `muN_eV`, `muN_m`, `muN_K`, `muN_MHz`, `gamma_p`, and `gamma_p_MHz` hashes and nodes unchanged.

- **CODATA 2018 proton gyromagnetic ratio in MHz/T is a one-sigma Interval.**
  `physis-constants` versions `gamma_p_MHz` as the CODATA 2018 hull
  `42.577478518(18)` MHz T⁻¹ from JPCRD 50, 033105 table XXXI
  (Proton, p). This is the recommended printed companion, not
  s⁻¹ T⁻¹ `gamma_p`, not `gp`, not MHz T⁻¹ `muN_MHz`, not Hz/T
  `muB_Hz`, not an SI defining Ratio, not a terminating SciExact, not
  `hbar`, and not a FormalClaim reconstructing `gamma_p / 2π` from live
  lookups. NIST lists MHz T⁻¹, not Hz T⁻¹. The ledger name is
  `gamma_p_MHz`. Shielded proton gyromagnetic ratio is a later row and
  is not stored. Helion gyromagnetic `gamma0h` still cites ħ and is not
  stored. Decade `10^{9}` (`10^{8}` is the 10× trap). This is not the
  CODATA 2022 last-digit `461`. Electron mass is still not stored
  (`10^{42}` overflows `i128`). `physis_model`
  `proton_gyromagnetic_ratio_in_mhz_per_tesla()` Qty locksteps to the
  recommended centre inside the hull. Adding `gamma_p_MHz` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, `KJ`, `RK`, `muB`, `muB_eV`, `muB_Hz`, `muB_m`, `muB_K`,
  `muN`, `muN_eV`, `muN_m`, `muN_K`, `muN_MHz`, and `gamma_p` hashes are unchanged. Theories still evaluate with
  `f64` Qty. That is not a kernel proof, not Canonical, not P4. Encode
  pins unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `gamma_p_MHz` hash 0e7b084d03777a4e9e875e48a87702d1d4284d30cb7ea50ba424f103d4660f73; node 89744598268cf707cc8c75c45c9eecd6e9380f4caa2521b9a94633818fc6ebf9; ledger node
  e2ad435e2d75302c87aed1816356b552c2d148c55337e94cd95070c0344abbf8. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`,
  `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`,
  `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`,
  `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`,
  `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, `KJ`, `RK`, `muB`, `muB_eV`, `muB_Hz`, `muB_m`, `muB_K`,
  `muN`, `muN_eV`, `muN_m`, `muN_K`, `muN_MHz`, and `gamma_p` hashes and nodes unchanged.

- **CODATA 2018 proton gyromagnetic ratio is a one-sigma Interval.**
  `physis-constants` versions `gamma_p` as the CODATA 2018 hull
  `2.6752218744(11)×10^8` s⁻¹ T⁻¹ from JPCRD 50, 033105 table XXXI
  (Proton, p). This is the recommended printed hull, not dimensionless
  `gp`, not J T⁻¹ `mu_p`, not `muN`, not MHz T⁻¹ `muN_MHz`, not Hz/T
  `muB_Hz`, not `e_mp`, not an SI defining Ratio, not a terminating
  SciExact, not `hbar`, and not a FormalClaim reconstructing `2 μp/ℏ`
  from live lookups. The printed formula cites ħ; the reconstruction is
  unused (same unused-formula exception as `muB` / `muN`). The ledger
  name is `gamma_p`. Proton gyromagnetic ratio in MHz/T is a later row
  and is not stored. Helion gyromagnetic `gamma0h` still cites ħ and is
  not stored. Decade `10^{2}` (`10^{1}` is the 10× trap). This is not
  the CODATA 2022 last-digit `8708`. Leftover pointers that said
  gyromagnetic ratios cite ħ now point at `gamma_p`. Electron mass is
  still not stored (`10^{42}` overflows `i128`). `physis_model`
  `proton_gyromagnetic_ratio()` Qty locksteps to the recommended centre
  inside the hull. Adding `gamma_p` to LEDGER changes the ledger bundle
  pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`,
  `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`,
  `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`,
  `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`,
  `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`,
  `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, `KJ`, `RK`, `muB`, `muB_eV`, `muB_Hz`, `muB_m`, `muB_K`,
  `muN`, `muN_eV`, `muN_m`, `muN_K`, and `muN_MHz` hashes are unchanged. Theories still evaluate with
  `f64` Qty. That is not a kernel proof, not Canonical, not P4. Encode
  pins unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `gamma_p` hash 116f462ed588536a31ea6e33be0ef8c1e26a5adb78b807dc93a8d5df0204457e; node d9f5089e6e55a37dea55e5d8418cf58eba84135d3a03bc0cd1ada40b07db50f3; ledger node
  7a46985daf6f2be4088255294c7df0ed9ca945b2b46190619d82317ea7f5debd. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`,
  `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`,
  `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`,
  `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`,
  `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, `KJ`, `RK`, `muB`, `muB_eV`, `muB_Hz`, `muB_m`, `muB_K`,
  `muN`, `muN_eV`, `muN_m`, `muN_K`, and `muN_MHz` hashes and nodes unchanged.

- **CODATA 2018 nuclear magneton in MHz/T is a one-sigma Interval.**
  `physis-constants` versions `muN_MHz` as the CODATA 2018 hull
  `7.6225932291(23)` MHz T⁻¹ from JPCRD 50, 033105 table XXXI
  (ELECTROMAGNETIC). This is the recommended printed companion, not
  J T⁻¹ `muN`, not K/T `muN_K`, not Hz/T `muB_Hz`, not an SI defining
  Ratio, not a terminating SciExact, not Planck `h`, not `hbar`, and
  not a FormalClaim reconstructing `muN/h` from live lookups. NIST lists
  MHz T⁻¹, not Hz T⁻¹. The ledger name is `muN_MHz`. Proton gyromagnetic
  ratio is a later row and is not stored. Decade `10^{10}` (`10^{9}` is
  the 10× trap). This is not the CODATA 2022 last-digit `2188`. Electron
  mass is still not stored (`10^{42}` overflows `i128`). `physis_model`
  `nuclear_magneton_in_mhz_per_tesla()` Qty locksteps to the recommended
  centre inside the hull. Adding `muN_MHz` to LEDGER changes the ledger
  bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`,
  `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`,
  `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`,
  `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`,
  `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`,
  `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, `KJ`, `RK`, `muB`, `muB_eV`, `muB_Hz`, `muB_m`, `muB_K`,
  `muN`, `muN_eV`, `muN_m`, and `muN_K` hashes are unchanged. Theories still evaluate with
  `f64` Qty. That is not a kernel proof, not Canonical, not P4. Encode
  pins unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `muN_MHz` hash bd63dae42eacb72e6c76f65457ce03a8abd107dbce9faa68ee04e0ed78c9adf1; node 54de34e2ffd258b5a8661cfa82244d91bad42238d9d554602c1eb8a269af583c; ledger node
  3b1c05aa095972dd7352ad58a3396b6e28ec6f2c56ac5b77bef03a31f893caea. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`,
  `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`,
  `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`,
  `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`,
  `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, `KJ`, `RK`, `muB`, `muB_eV`, `muB_Hz`, `muB_m`, `muB_K`,
  `muN`, `muN_eV`, `muN_m`, and `muN_K` hashes and nodes unchanged.

- **CODATA 2018 nuclear magneton in K/T is a one-sigma Interval.**
  `physis-constants` versions `muN_K` as the CODATA 2018 hull
  `3.6582677756(11)×10^{-4}` K T⁻¹ from JPCRD 50, 033105 table XXXI
  (ELECTROMAGNETIC). This is the recommended printed companion, not
  J T⁻¹ `muN`, not inverse-meter `muN_m`, not K/T `muB_K`, not an SI
  defining Ratio, not a terminating SciExact, not Boltzmann `k`, not
  `hbar`, and not a FormalClaim reconstructing `muN/k` from live
  lookups. The ledger name is `muN_K`. Nuclear magneton in MHz/T is a later
  ELECTROMAGNETIC row and is not stored. Decade `10^{14}` (`10^{13}` is
  the 10× trap). This is not the CODATA 2022 last-digit `7706`. Electron
  mass is still not stored (`10^{42}` overflows `i128`). `physis_model`
  `nuclear_magneton_in_kelvin_per_tesla()` Qty locksteps to the
  recommended centre inside the hull. Adding `muN_K` to LEDGER changes
  the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`,
  `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`,
  `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`,
  `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`,
  `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, `KJ`, `RK`, `muB`, `muB_eV`, `muB_Hz`, `muB_m`, `muB_K`,
  `muN`, `muN_eV`, and `muN_m` hashes are unchanged. Theories still evaluate with
  `f64` Qty. That is not a kernel proof, not Canonical, not P4. Encode
  pins unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `muN_K` hash deb575e22e92050ab049888ba327bad752ec6d478cb3384b64ad7b3d2e6592bd; node e1076582e911fb1259ed2db8919b357e754aee37032996e48ae58a8759dadd68; ledger node
  b9164f27818ac68c0e4d8b0801fde0a7f85260dfb331591387f41b8d3e182f54. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`,
  `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`,
  `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`,
  `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`,
  `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, `KJ`, `RK`, `muB`, `muB_eV`, `muB_Hz`, `muB_m`, `muB_K`,
  `muN`, `muN_eV`, and `muN_m` hashes and nodes unchanged.

- **CODATA 2018 nuclear magneton in inverse meter per tesla is a one-sigma Interval.**
  `physis-constants` versions `muN_m` as the CODATA 2018 hull
  `2.54262341353(78)×10^{-2}` m⁻¹ T⁻¹ from JPCRD 50, 033105 table XXXI
  (ELECTROMAGNETIC). This is the recommended printed companion, not
  J T⁻¹ `muN`, not eV T⁻¹ `muN_eV`, not inverse-meter `muB_m`, not an SI
  defining Ratio, not a terminating SciExact, not Planck `h`, not
  `hbar`, and not a FormalClaim reconstructing `muN/hc` from live
  lookups. The ledger name is `muN_m`. Nuclear magneton in K/T is a later
  ELECTROMAGNETIC row and is not stored. Decade `10^{13}` (`10^{12}` is
  the 10× trap). This is not the CODATA 2022 last-digit `41009`. Electron
  mass is still not stored (`10^{42}` overflows `i128`). `physis_model`
  `nuclear_magneton_in_inverse_meter_per_tesla()` Qty locksteps to the
  recommended centre inside the hull. Adding `muN_m` to LEDGER changes
  the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`,
  `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`,
  `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`,
  `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`,
  `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, `KJ`, `RK`, `muB`, `muB_eV`, `muB_Hz`, `muB_m`, `muB_K`,
  `muN`, and `muN_eV` hashes are unchanged. Theories still evaluate with
  `f64` Qty. That is not a kernel proof, not Canonical, not P4. Encode
  pins unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `muN_m` hash 4cbff17f649eaa3b46d7ec75aeeddc8a8becebcddc7c4538bca1e96097800e0f; node 1c927bad61aec31ce19c4de08f8331adeba44d15f07c9577d84556b2315f7b9a; ledger node
  2418b2f7044ee17821b52265df8e5d4ef6b7d876cd8a5bd5243923eeca01914f. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`,
  `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`,
  `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`,
  `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`,
  `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, `KJ`, `RK`, `muB`, `muB_eV`, `muB_Hz`, `muB_m`, `muB_K`,
  `muN`, and `muN_eV` hashes and nodes unchanged.

- **CODATA 2018 nuclear magneton in eV/T is a one-sigma Interval.**
  `physis-constants` versions `muN_eV` as the CODATA 2018 hull
  `3.15245125844(96)×10^{-8}` eV T⁻¹ from JPCRD 50, 033105 table XXXI
  (ELECTROMAGNETIC). This is the recommended printed companion, not
  J T⁻¹ `muN`, not Bohr magneton in eV/T `muB_eV`, not an SI defining
  Ratio, not a terminating SciExact, not electronvolt `eV`, not `hbar`,
  and not a FormalClaim reconstructing `muN/e` from live lookups. The
  ledger name is `muN_eV`. Nuclear magneton in inverse meter per tesla is
  a later ELECTROMAGNETIC row and is not stored. Decade `10^{19}`
  (`10^{18}` is the 10× trap). This is not the CODATA 2022 last-digit
  `25417`. Electron mass is still not stored (`10^{42}` overflows
  `i128`). `physis_model` `nuclear_magneton_in_ev_per_tesla()` Qty
  locksteps to the recommended centre inside the hull. Adding `muN_eV`
  to LEDGER changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`,
  `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`,
  `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`,
  `mu_t_muN`, `gt`, `m_h`, `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`,
  `mh_mp`, `M_h`, `mu_h`, `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`,
  `mu0h_muB`, `mu0h_muN`, `mu0h_mup`, `mu0h_mu0p`, `m_alpha`,
  `m_alpha_u`, `m_alpha_c2`, `m_alpha_c2_MeV`, `malpha_me`,
  `malpha_mp`, `M_alpha`, `m_u`, `m_u_c2`, `m_u_c2_MeV`, `M_u`,
  `M_12C`, `NAh`, `NAk`, `NAe`, `p0`, `atm`, `Vm`, `n0`, `Vm_atm`,
  `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`, `c2`, `KJ`, `RK`, `muB`,
  `muB_eV`, `muB_Hz`, `muB_m`, `muB_K`, and `muN` hashes are unchanged.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified: `muN_eV` hash 896e79954d3048b9eb9f1f8d0be11c351690c3bd2db4d3be8f1853395bc51291;
  node 3499319aa12a29a8123e77d0a0fcbd6cc4a0ee6da708788ef7450ad0453892d0; ledger node 676bc6902c1f6789809a84e37a0aac4349a1959b110c132542afa95d7fd8e050. `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`,
  `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`,
  `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`,
  `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`,
  `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`,
  `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, `KJ`, `RK`, `muB`, `muB_eV`, `muB_Hz`, `muB_m`, `muB_K`, and
  `muN` hashes and nodes unchanged.

- **CODATA 2018 nuclear magneton is a one-sigma Interval.**
  `physis-constants` versions `muN` as the CODATA 2018 hull
  `5.0507837461(15)×10^{-27}` J T⁻¹ from JPCRD 50, 033105 table XXXI
  (ELECTROMAGNETIC). This is the recommended printed hull, not Bohr
  magneton `muB`, not neutron magnetic moment `mu_n`, not electron
  magnetic moment `mu_e`, not an SI defining Ratio, not a terminating
  SciExact, not `hbar`, and not a FormalClaim reconstructing `eℏ/2mp`
  from live lookups. The printed formula cites ħ and is unused. The
  ledger name is `muN`. Nuclear magneton in eV/T is a later
  ELECTROMAGNETIC row and is not stored. Decade `10^{37}` (`10^{36}` is
  the 10× trap). This is not the CODATA 2022 last-digit `7393`. Electron
  mass is still not stored (`10^{42}` overflows `i128`). `physis_model`
  `nuclear_magneton()` Qty locksteps to the recommended centre inside the
  hull. Adding `muN` to LEDGER changes the ledger bundle pin. The `G`,
  `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`,
  `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`,
  `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`,
  `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`,
  `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`,
  `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, `KJ`, `RK`, `muB`, `muB_eV`, `muB_Hz`, `muB_m`, and `muB_K`
  hashes are unchanged. Theories still evaluate with `f64` Qty. That is
  not a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified:
  `muN` hash c3185d2ffda0a76ed87ea106513eb25592aec9bbec35f8647a81d367f70f3835; node 5b78c0f11cbde89d9e2b0fb274c93113715007644fab16975a1c87fea4c79efb; ledger node
  d8124a7069227bfa72557716ca04944baeefe364c1c66dc5811427a1ce8b8ea5. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`,
  `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`,
  `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`,
  `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`,
  `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, `KJ`, `RK`, `muB`, `muB_eV`, `muB_Hz`, `muB_m`, and `muB_K`
  hashes and nodes unchanged.

- **CODATA 2018 Bohr magneton in K/T is a one-sigma Interval.**
  `physis-constants` versions `muB_K` as the CODATA 2018 hull
  `0.67171381563(20)` K T⁻¹ from JPCRD 50, 033105 table XXXI
  (ELECTROMAGNETIC). This is the recommended printed companion, not J T⁻¹
  `muB`, not inverse-meter `muB_m`, not an SI defining Ratio, not a
  terminating SciExact, not Boltzmann `k`, not nuclear magneton `muN`,
  not `hbar`, and not a FormalClaim reconstructing `muB/k` from live
  lookups. The ledger name is `muB_K`. Nuclear magneton is a later
  ELECTROMAGNETIC row and is not stored (printed formula cites ħ). Decade
  `10^{11}` (`10^{10}` is the 10× trap). This is not the CODATA 2022
  last-digit `81472`. Electron mass is still not stored (`10^{42}`
  overflows `i128`). `physis_model` `bohr_magneton_in_kelvin_per_tesla()`
  Qty locksteps to the recommended centre inside the hull. Adding
  `muB_K` to LEDGER changes the ledger bundle pin. The `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`,
  `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`,
  `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`,
  `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`,
  `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`,
  `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, `KJ`, `RK`, `muB`, `muB_eV`, `muB_Hz`, and `muB_m` hashes are
  unchanged. Theories still evaluate with `f64` Qty. That is not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified:
  `muB_K` hash fd9a45856a999a4b1af21966c2cb3e3cee7f27aa6e8960a3168868dea04aa451; node 0dc9efe8516984f8cd702a37559d258d6073f1e6f2c04544c705b7a1b2a0d083; ledger node
  2ab1a604005a065c90cdef3efaf27e524fad12bbefedae818a4c317a746cf3af. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`,
  `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`,
  `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`,
  `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`,
  `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, `KJ`, `RK`, `muB`, `muB_eV`, `muB_Hz`, and `muB_m` hashes and
  nodes unchanged.

- **CODATA 2018 Bohr magneton in inverse meter per tesla is a one-sigma Interval.**
  `physis-constants` versions `muB_m` as the CODATA 2018 hull
  `46.686447783(14)` m⁻¹ T⁻¹ from JPCRD 50, 033105 table XXXI
  (ELECTROMAGNETIC). This is the recommended printed companion, not J T⁻¹
  `muB`, not eV T⁻¹ `muB_eV`, not Hz T⁻¹ `muB_Hz`, not an SI defining
  Ratio, not a terminating SciExact, not Planck `h`, not nuclear magneton
  `muN`, not `hbar`, and not a FormalClaim reconstructing `muB/hc` from
  live lookups. The ledger name is `muB_m`. Bohr magneton in K/T is a
  later ELECTROMAGNETIC row and is not stored. Decade `10^{9}` (`10^{8}`
  is the 10× trap). This is not the CODATA 2022 last-digit `719`. Electron
  mass is still not stored (`10^{42}` overflows `i128`). `physis_model`
  `bohr_magneton_in_inverse_meter_per_tesla()` Qty locksteps to the
  recommended centre inside the hull. Adding `muB_m` to LEDGER changes
  the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`,
  `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`,
  `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`,
  `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`,
  `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, `KJ`, `RK`, `muB`, `muB_eV`, and `muB_Hz` hashes are unchanged.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified: `muB_m` hash 5870c955c8af2612ebe76eee32448cabd843ef430611ac3cd01623ae9dfd7bb3;
  node a5c6bc7a371abe6a50398009955f3ff1790cb5323027ddb9f51bc90920664e04; ledger node 47b796e441106474349a9fb9ed6aa871db876638f18ee5f3953c1bbbe3e09219. `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`,
  `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`,
  `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`,
  `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`,
  `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`,
  `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`,
  `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, `KJ`, `RK`, `muB`, `muB_eV`, and `muB_Hz` hashes and nodes
  unchanged.

- **CODATA 2018 Bohr magneton in Hz/T is a one-sigma Interval.**
  `physis-constants` versions `muB_Hz` as the CODATA 2018 hull
  `1.39962449361(42)×10^{10}` Hz T⁻¹ from JPCRD 50, 033105 table XXXI
  (ELECTROMAGNETIC). This is the recommended printed companion, not J T⁻¹
  `muB`, not eV T⁻¹ `muB_eV`, not an SI defining Ratio, not a terminating
  SciExact, not Planck `h`, not nuclear magneton `muN`, not `hbar`, and
  not a FormalClaim reconstructing `muB/h` from live lookups. The ledger
  name is `muB_Hz`. Bohr magneton in inverse meter per tesla is a later
  ELECTROMAGNETIC row and is not stored. Decade `10^{1}` (`10^{0}` is the
  10× trap). This is not the CODATA 2022 last-digit `49171`. Electron
  mass is still not stored (`10^{42}` overflows `i128`). `physis_model`
  `bohr_magneton_in_hz_per_tesla()` Qty locksteps to the recommended
  centre inside the hull. Adding `muB_Hz` to LEDGER changes the ledger
  bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`,
  `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`,
  `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`,
  `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`,
  `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, `KJ`, `RK`, `muB`, and `muB_eV` hashes are unchanged. Theories
  still evaluate with `f64` Qty. That is not a kernel proof, not
  Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified: `muB_Hz` hash b51f2cbb2761484c081909689471b593a47521359e481cbbe1c6c0083d6cb749;
  node 3cf64fd1f060d0379c3d21d17b5ba9bffa0c40ad1817493a084d13cde1cba088; ledger node d2b10504529dba01d54915fb4bbcb00bffa2ff8d3f22201773531c0234c0e72d. `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`,
  `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`,
  `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`,
  `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`,
  `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`,
  `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`,
  `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, `KJ`, `RK`, `muB`, and `muB_eV` hashes and nodes unchanged.

- **CODATA 2018 Bohr magneton in eV/T is a one-sigma Interval.**
  `physis-constants` versions `muB_eV` as the CODATA 2018 hull
  `5.7883818060(17)×10^{-5}` eV T⁻¹ from JPCRD 50, 033105 table XXXI
  (ELECTROMAGNETIC). This is the recommended printed companion, not J T⁻¹
  `muB`, not an SI defining Ratio, not a terminating SciExact, not
  electronvolt `eV`, not elementary charge `e`, not electron magnetic
  moment `mu_e`, not nuclear magneton `muN`, not `hbar`, and not a
  FormalClaim reconstructing `muB/e` from live lookups. The ledger name
  is `muB_eV`. Bohr magneton in Hz/T is a later ELECTROMAGNETIC row and
  is not stored. Decade `10^{15}` (`10^{14}` is the 10× trap). This is
  not the CODATA 2022 last-digit `7982`. Electron mass is still not
  stored (`10^{42}` overflows `i128`). `physis_model`
  `bohr_magneton_in_ev_per_tesla()` Qty locksteps to the recommended
  centre inside the hull. Adding `muB_eV` to LEDGER changes the ledger
  bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`,
  `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`,
  `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`,
  `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`,
  `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, `KJ`, `RK`, and `muB` hashes are unchanged. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4. Verified: `muB_eV` hash 2eba7e5ba5747c0aabedad751c362d39314d5d212378279e872a2a0c48cdf15b; node
  8df4a0984aa398db3d36b391d61ec73784b0a7c72257906a42b9e7425efc6d49; ledger node 09f9ae948d03ec7ea419f806380d94470bba42d070cc44a42ece17891a71bfdc. `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`,
  `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`,
  `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`,
  `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`,
  `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`,
  `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`,
  `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, `KJ`, `RK`, and `muB` hashes and nodes unchanged.

- **CODATA 2018 Bohr magneton is a one-sigma Interval.**
  `physis-constants` versions `muB` as the CODATA 2018 hull
  `9.2740100783(28)×10^{-24}` J T⁻¹ from JPCRD 50, 033105 table XXXI
  (ELECTROMAGNETIC). The printed formula `eℏ/2me` cites ħ and is unused;
  the ledger stores the recommended one-sigma hull. This is not an SI
  defining Ratio, not a terminating SciExact, not electron magnetic
  moment `mu_e`, not nuclear magneton `muN` (printed `eℏ/2mp`; ħ; not
  stored), not magnetic flux quantum `Phi0`, not conductance quantum
  `G0`, not Josephson `KJ`, not von Klitzing `RK`, not Planck `h`, not
  elementary charge `e`, not electron mass, not `hbar`, and not a
  FormalClaim reconstructing `eℏ/2me` from live lookups. The JPCRD
  symbol `μB` is the ledger name `muB`. Bohr magneton in eV/T is a later
  ELECTROMAGNETIC row and is not stored. Decade `10^{34}` (`10^{33}` is
  the 10× trap). This is not the CODATA 2022 last-digit `0657`. Electron
  mass is still not stored (`10^{42}` overflows `i128`). `physis_model`
  `bohr_magneton()` Qty locksteps to the recommended centre inside the
  hull. Adding `muB` to LEDGER changes the ledger bundle pin. The `G`,
  `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`,
  `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`,
  `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`,
  `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`,
  `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`,
  `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`,
  `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, `KJ`, and `RK` hashes are unchanged. Theories still evaluate
  with `f64` Qty. That is not a kernel proof, not Canonical, not P4.
  Encode pins unchanged. Unique-vacuum graph id unchanged. P3N count
  stays 4. Verified: `muB` hash 05bdf64c433e9c8bdf8db2dd7991db310baf2ba41fb0e3cff31d1cd98ef0f9df; node 25548862db56005d5ed865aee7f0ee08a7cd24c478ec0aec2e78c775ca9060ef;
  ledger node 27bcc0d5a9575f283208d389705ce76c35a0c0e03ce0540c1b80d0b08c352f2c. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`,
  `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, `KJ`, and `RK` hashes and nodes unchanged.

- **CODATA 2018 von Klitzing constant is an exact Ratio.**
  `physis-constants` versions `RK` as the SI-exact Ratio `h / e²` =
  `5521725125000000000000/213914163877964163` ohm from JPCRD 50, 033105
  table XXXI (ELECTROMAGNETIC). The table prints `25 812.807 45…`; the
  ledger stores the exact product. The reduced denominator keeps
  factors 3, 19, 389, and 12043, so this is not a terminating SciExact.
  This is not Planck `h`, not elementary charge `e`, not Josephson
  `KJ`, not vacuum impedance `Z0`, not magnetic flux quantum `Phi0`
  (printed `2πℏ/(2e)`; π and ħ; not stored), not conductance quantum
  `G0` (printed `2e²/2πℏ`; π and ħ; not stored), not conventional 1990
  `RK-90`, not an SI defining constant, and not a FormalClaim
  reconstructing `h / e²` from live lookups. JPCRD also writes
  `2πℏ/e²`; that printed formula cites π and ħ and is not the stored
  product. The JPCRD symbol `RK` is the ledger name. Bohr magneton
  `muB` is a later ELECTROMAGNETIC row (ħ; not stored). Electron mass
  is still not stored (`10^{42}` overflows `i128`). CODATA 2022 prints
  the same SI-exact ellipsis; there is no last-digit trap.
  `physis_model` `von_klitzing_constant()` Qty locksteps to
  `Ratio::to_f64` of the reduced fraction. Adding `RK` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, and `KJ` hashes are unchanged. Theories still evaluate with
  `f64` Qty. That is not a kernel proof, not Canonical, not P4. Encode
  pins unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `RK` hash 2faf6f39986b543d3370bdd5764f0d075fa94a709d3fadd235ed82026fed2d46; node 9d6fa90a4590f96e4ab850e99847ab06da3d60e90a61218f2c5642a67d174f19;
  ledger node 79dc4a78dfa2007cb5d1b1600b0508b4b14aa02d3f2f3a5c41436cd2c00f0cca. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  `c2`, and `KJ` hashes and nodes unchanged.

- **CODATA 2018 Josephson constant is an exact Ratio.**
  `physis-constants` versions `KJ` as the SI-exact Ratio `2 e / h` =
  `21362355120000000000000/44173801` Hz V^{-1} from JPCRD 50, 033105
  table XXXI (ELECTROMAGNETIC). The table prints
  `483 597.848 4… × 10^9`; the ledger stores the exact product. The
  reduced denominator keeps factors 7 and 6310543, so this is not a
  terminating SciExact. This is not elementary charge `e`, not Planck
  `h`, not magnetic flux quantum `Phi0` (printed `2πℏ/(2e)`; π and ħ;
  not stored), not conductance quantum `G0` (printed `2e²/2πℏ`; π and
  ħ; not stored), not conventional 1990 `KJ-90`, not `c2`, not an SI
  defining constant, and not a FormalClaim reconstructing `2 e / h`
  from live lookups. The JPCRD symbol `KJ` is the ledger name. Von
  Klitzing `RK` is a later ELECTROMAGNETIC row. Electron mass is still
  not stored (`10^{42}` overflows `i128`). CODATA 2022 prints the same
  SI-exact ellipsis; there is no last-digit trap. `physis_model`
  `josephson_constant()` Qty locksteps to `Ratio::to_f64` of the
  reduced fraction, not the unreduced `as f64` (one ulp lower) and not
  Python true-division. Adding `KJ` to LEDGER changes the ledger bundle
  pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`,
  `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`,
  `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`,
  `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`,
  `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  and `c2` hashes are unchanged. Theories still evaluate with `f64`
  Qty. That is not a kernel proof, not Canonical, not P4. Encode pins
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `KJ` hash eb31c5b04ef0823e6e80a2921172c06fa6ef692e5a7700cb25d183b00a0090d2; node 911ac483d6b64b64f2d44cd808b2242e4445fd5e2eb6b13c0c714cd0aae6527c;
  ledger node ba05f02b609493c4aa8eebd9069ca47259988504d21cbd291f098267b16d950c. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, `c1L`,
  and `c2` hashes and nodes unchanged.

- **CODATA 2018 second radiation constant is an exact Ratio.**
  `physis-constants` versions `c2` as the SI-exact Ratio `h c / k` =
  `272115870842319/18913000000000000` m K from JPCRD 50, 033105 table
  XXXI (PHYSICOCHEMICAL). The table prints `1.438 776 877… × 10^{-2}`;
  the ledger stores the exact product. The reduced denominator keeps
  factor 18913, so this is not a terminating SciExact. This is not
  Planck `h`, not Boltzmann `k`, not `c1L`, not first radiation
  constant `c1` = `2πhc²` (π; not stored), not Stefan-Boltzmann `σ`
  (π; not stored), not Wien `b` / `b0` (transcendental; later rows),
  not an SI defining constant, and not a FormalClaim reconstructing
  `h c / k` from live lookups. The JPCRD symbol `c2` is the ledger
  name. Electron mass is still not stored (`10^{42}` overflows
  `i128`). CODATA 2022 prints the same SI-exact ellipsis; there is no
  last-digit trap. `physis_model` `second_radiation_constant()` Qty
  locksteps to `Ratio::to_f64` of the reduced fraction. Adding `c2` to
  LEDGER changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`,
  `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, and
  `c1L` hashes are unchanged. Theories still evaluate with `f64` Qty.
  That is not a kernel proof, not Canonical, not P4. Encode pins
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `c2` hash 9b6ced8d9873adf9b03f13f024d13b8c2ebc18e15e9f3d57fadf0eff0ed61cbc; node 8c098b806e87a28bd2d42c5d7f5662dd61b3b81f9c6d926ecc1cd2c7665ccc87;
  ledger node b1a0822a74113454ec1cabca659e2574f435a09095fbd25e80e87100893b4570. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, `S0_R_atm`, and
  `c1L` hashes and nodes unchanged.

- **CODATA 2018 first radiation constant for spectral radiance is SI-exact SciExact.**
  `physis-constants` versions `c1L` as the SI-exact terminating decimal
  `2 h c²` = `11910429723971884140794892e-41` W m² sr⁻¹ from JPCRD 50,
  033105 table XXXI (PHYSICOCHEMICAL). The table prints
  `1.191 042 972… × 10^{-16}`; the ledger stores the full product. That
  product does not fit Ratio (`10^{41}` overflows `i128`, same reason
  Planck `h` is SciExact). This is not Planck `h`, not Stefan-Boltzmann
  `σ` (that formula cites π and is not stored), not first radiation
  constant `c1` = `2πhc²` (π; a later row), not second radiation
  constant `c2`, not an SI defining constant, and not a FormalClaim
  reconstructing `2 h c²` from live lookups. The JPCRD symbol `c1L` is
  the ledger name. Electron mass is still not stored (`10^{42}`
  overflows `i128`). CODATA 2022 prints the same SI-exact ellipsis;
  there is no last-digit trap. `physis_model`
  `first_radiation_constant_spectral_radiance()` Qty locksteps to
  `SciExact::to_f64` of that decimal. Adding `c1L` to LEDGER changes the
  ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`,
  `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, and `S0_R_atm` hashes
  are unchanged. Theories still evaluate with `f64` Qty. That is not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `c1L` hash bb3b42d41a8d8ebc3191a2aa98d974733538eaba1098eb89a1574d228479249c; node 3c68d593569cb32f01daa8afb184f8565dfad4311457c390e6a3d49f665357b3;
  ledger node 9215c6352dd1c33fb79ccd1c9227adf1df97e72c402c3bc3e40e7bacbcbd6d29. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, `S0_R`, and `S0_R_atm` hashes
  and nodes unchanged.

- **CODATA 2018 101.325 kPa Sackur-Tetrode constant is a one-sigma Interval.**
  `physis-constants` versions `S0_R_atm` as the CODATA 2018 hull
  `-1.16487052358(45)` (dimensionless) from JPCRD 50, 033105 table XXXI
  (PHYSICOCHEMICAL) at T1 = 1 K and p0 = 101.325 kPa. JPCRD prints the
  same symbol `S0/R` as the 100 kPa row; `S0_R_atm` is the ledger name.
  `S0` and `S0/R` are not second names. The JPCRD formula cites ħ and
  π, so this is not an exact Ratio and not a FormalClaim reconstructing
  that formula from live lookups. This is not 100 kPa `S0_R`, not
  101.325 kPa Loschmidt `n0_atm`, not 100 kPa `n0`, not molar gas
  `NAk`, not an SI defining Ratio, and not P3N. Stefan-Boltzmann is a
  later table row and is not stored. Electron mass is still not stored
  (`10^{42}` overflows `i128`). Decade `10^{11}` (`10^{10}` is the 10×
  trap). This is not the CODATA 2022 last-digit `49`. `physis_model`
  `sackur_tetrode_constant_atm()` Qty locksteps to the recommended
  centre inside the hull. Adding `S0_R_atm` to LEDGER changes the
  ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`,
  `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, and `S0_R` hashes are
  unchanged. Theories still evaluate with `f64` Qty. That is not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `S0_R_atm` hash 80cbdc3db3e995895b8c311f14beea81756bf55eee0004c45e97efc43d54af2f; node c3fb74638f4c181d336240b94bd416d3976a63c2720af0497f3a1347f6fab560;
  ledger node b03e691473aac9cf7759b1a17b4d06307405c359996ffac608cf004642eeee34. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, `n0_atm`, and `S0_R` hashes and nodes
  unchanged.

- **CODATA 2018 Sackur-Tetrode constant is a one-sigma Interval.**
  `physis-constants` versions `S0_R` as the CODATA 2018 hull
  `-1.15170753706(45)` (dimensionless) from JPCRD 50, 033105 table XXXI
  (PHYSICOCHEMICAL) at T1 = 1 K and p0 = 100 kPa. JPCRD prints `S0/R`;
  `S0_R` is the ledger name. `S0` and `S0/R` are not second names. The
  JPCRD formula cites ħ and π, so this is not an exact Ratio and not a
  FormalClaim reconstructing that formula from live lookups. This is
  not 101.325 kPa Loschmidt `n0_atm`, not 100 kPa `n0`, not molar gas
  `NAk`, not the 101.325 kPa Sackur-Tetrode companion, not an SI
  defining Ratio, and not P3N. The 101.325 kPa companion is a later
  table row and is not stored. Stefan-Boltzmann is a later table row
  and is not stored. Electron mass is still not stored (`10^{42}`
  overflows `i128`). Decade `10^{11}` (`10^{10}` is the 10× trap).
  This is not the CODATA 2022 last-digit `96`. `physis_model`
  `sackur_tetrode_constant()` Qty locksteps to the recommended centre
  inside the hull. Adding `S0_R` to LEDGER changes the ledger bundle
  pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`,
  `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`,
  `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`,
  `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`,
  `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, and `n0_atm` hashes are unchanged.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4.
  Verified: `S0_R` hash 37bd72c139f411ab023e7a400c4d8b90ef044d0334ab01e29ca31d6bdca08a8e; node 5056a0679ae99314961bcadfe60712a265c4e1b687bf7f28b4cab22f30aa8c56;
  ledger node d5e6aaa6d301c10f79c74350d6a07f7fbc99f8b7c617555ed77131d77e29d69f. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, `Vm_atm`, and `n0_atm` hashes and nodes unchanged.

- **CODATA 2018 101.325 kPa Loschmidt constant is an exact Ratio.**
  `physis-constants` versions `n0_atm` as the exact SI 2019 Ratio
  `atm / (k T)` at `T = 273.15 K` and `p = 101.325 kPa`
  (`67550000000000000000000000000000000/2514161829` m⁻³) from JPCRD
  50, 033105 table XXXI (PHYSICOCHEMICAL). Equal to `N_A / Vm_atm`.
  JPCRD prints the same symbol `n0` as the 100 kPa row; `n0_atm` is
  the ledger name. The table prints an ellipsis; the ledger stores the
  exact Ratio, not the truncated display digits. The denominator is
  not a pure power of ten, so this is not a terminating SciExact. This
  is not 100 kPa `n0`, not molar volume `Vm_atm`, not Boltzmann `k`,
  not Avogadro `N_A`, not an SI defining constant, not a FormalClaim
  that reconstructs `N_A / Vm_atm` from live lookups, and not P3N.
  The Sackur-Tetrode constant is a later table row and is not stored.
  Electron mass is still not stored (`10^{42}` overflows `i128`). The
  product fits Ratio (`10^{31}`). CODATA 2022 prints the same SI-exact
  ellipsis; there is no last-digit trap. `physis_model`
  `loschmidt_constant_atm()` Qty locksteps to the IEEE rounding of
  that exact Ratio (`Ratio::to_f64`). Adding `n0_atm` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, and `Vm_atm` hashes are unchanged. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4.
  Verified: `n0_atm` hash 040ac164b64d31d949ee7f2b59af9ed649dcd8f6fb69f09546b317b8a9beb14b; node a57c2a552d28588ebe8cfabbf47a74327805c9e6352f78c46b78be923ee7331e;
  ledger node 1329a174d0e22890ad2a3f04e5434f7770419e113f28fa0ae4d12830060e2bb8. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, `n0`, and `Vm_atm` hashes and nodes unchanged.

- **CODATA 2018 101.325 kPa molar volume of ideal gas is an exact Ratio.**
  `physis-constants` versions `Vm_atm` as the exact SI 2019 Ratio
  `R T / atm` at `T = 273.15 K` and `p = 101.325 kPa`
  (`378515910691426251/16887500000000000000` m³ mol⁻¹) from JPCRD 50,
  033105 table XXXI (PHYSICOCHEMICAL). JPCRD prints the same symbol
  `Vm` as the 100 kPa row; `Vm_atm` is the ledger name. The table
  prints an ellipsis; the ledger stores the exact Ratio, not the
  truncated display digits. The denominator keeps factors 7 and 193
  from 101325, so this is not a terminating SciExact. This is not
  100 kPa `Vm`, not Loschmidt `n0`, not standard atmosphere `atm`, not
  an SI defining constant, not a FormalClaim that reconstructs
  `NAk × T / atm` from live lookups, and not P3N. The 101.325 kPa
  Loschmidt constant is a later table row and is not stored. Electron
  mass is still not stored (`10^{42}` overflows `i128`). The product
  fits Ratio (`10^{16}`). CODATA 2022 prints the same SI-exact
  ellipsis; there is no last-digit trap. `physis_model`
  `molar_volume_ideal_gas_atm()` Qty locksteps to the IEEE rounding of
  that exact Ratio (`Ratio::to_f64`). Adding `Vm_atm` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, and `n0` hashes are unchanged. Theories still evaluate
  with `f64` Qty. That is not a kernel proof, not Canonical, not P4.
  Encode pins unchanged. Unique-vacuum graph id unchanged. P3N count
  stays 4.
  Verified: `Vm_atm` hash ee25d6479dd4102060b836649dc7a84cddec0dab3838c1f79d33b9e19ff11e92; node cf842ed327f1aa6b028b66c6dc6c392ab75f035a01e6dc704ab14702712db094;
  ledger node 1d0bbc892b5b342a1ce85d1c689365c2c894d9a871070c8820aea07ac44d4868. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, `Vm`, and `n0` hashes and nodes unchanged.

- **CODATA 2018 Loschmidt constant is an exact Ratio.**
  `physis-constants` versions `n0` as the exact SI 2019 Ratio
  `p0 / (k T)` at `T = 273.15 K` and `p = 100 kPa`
  (`200000000000000000000000000000000000/7542485487` m⁻³) from JPCRD
  50, 033105 table XXXI (PHYSICOCHEMICAL). Equal to `N_A / Vm`. The
  table prints an ellipsis; the ledger stores the exact Ratio, not the
  truncated display digits. The denominator is not a pure power of ten,
  so this is not a terminating SciExact. This is not molar volume `Vm`,
  not Boltzmann `k`, not Avogadro `N_A`, not standard-state pressure
  `p0`, not standard atmosphere `atm`, not the 101.325 kPa companion,
  not an SI defining constant, not a FormalClaim that reconstructs
  `N_A / Vm` from live lookups, and not P3N. The 101.325 kPa molar
  volume is a later table row and is not stored. Electron mass is still
  not stored (`10^{42}` overflows `i128`). The numerator fits Ratio
  (`10^{36}`). CODATA 2022 prints the same SI-exact ellipsis; there is
  no last-digit trap. `physis_model` `loschmidt_constant()` Qty
  locksteps to the IEEE rounding of that exact Ratio (`Ratio::to_f64`).
  Adding `n0` to LEDGER changes the ledger bundle pin. The `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`,
  `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`,
  `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`,
  `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`,
  `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`,
  `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`,
  `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, and `Vm` hashes are unchanged. Theories still evaluate with
  `f64` Qty. That is not a kernel proof, not Canonical, not P4. Encode
  pins unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `n0` hash 886c42750e98f22584361f3cba1c202a4b75fbcdb5485e4fa06df7645129e3f3; node 9561ce70ecd5a964b572bf460bed2c6f035b23a4a95815923788dbc075d95eea;
  ledger node 0be3e99e0bb6f8d2dabe302be9e76217da675e9f4c19f2934d39c6d0a6f2cc16. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`,
  `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  `atm`, and `Vm` hashes and nodes unchanged.

- **CODATA 2018 molar volume of ideal gas is an exact Ratio.**
  `physis-constants` versions `Vm` as the exact SI 2019 product
  `R T / p0` at `T = 273.15 K` and `p = 100 kPa`
  `0.022710954641485575` m³ mol⁻¹ from JPCRD 50, 033105 table XXXI
  (PHYSICOCHEMICAL). The table prints an ellipsis; the ledger stores
  the full terminating decimal, not the truncated display digits.
  This is not molar gas `NAk`, not standard-state pressure `p0`, not
  standard atmosphere `atm`, not the 101.325 kPa companion molar
  volume, not Loschmidt `n0`, not Faraday `NAe`, not an SI defining
  constant, not a FormalClaim that reconstructs `NAk × T / p0` from
  live lookups, and not P3N. Loschmidt constant is a later table row
  and is not stored. Electron mass is still not stored (`10^{42}`
  overflows `i128`). The product fits Ratio (`10^{21}`). CODATA 2022
  prints the same SI-exact ellipsis; there is no last-digit trap.
  `physis_model` `molar_volume_ideal_gas()` Qty locksteps to the IEEE
  rounding of that SI decimal (`SciExact::to_f64`). Adding `Vm` to
  LEDGER changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`,
  `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`, `mt_mp`,
  `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`, `m_h_u`,
  `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`, `mu_h_muB`,
  `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`, `mu0h_mup`,
  `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`, `m_alpha_c2_MeV`,
  `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`, `m_u_c2`, `m_u_c2_MeV`,
  `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`, and `atm` hashes are
  unchanged. Theories still evaluate with `f64` Qty. That is not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `Vm` hash 0b56b98a81e8961ca9be8efeb6775ea197dbf0f96a913cae13944d40e71479a5; node e5115026033b6250c155eb66657eeec10fa5f72ddf3ae38e85ade223235f5f6b;
  ledger node 8e148d227818b9a85f4084313e6ad89bf63e1598861076d8b9f6ea5cfa325f25. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`,
  `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, `p0`,
  and `atm` hashes and nodes unchanged.

- **CODATA 2018 standard atmosphere is an exact Ratio.**
  `physis-constants` versions `atm` as the exact PHYSICOCHEMICAL
  conventional pressure `101325` Pa from JPCRD 50, 033105 table XXXI.
  JPCRD prints no symbol; `atm` is the ledger name. This is not
  standard-state pressure `p0` (100000 Pa), not Faraday `NAe`, not
  Newtonian `G`, not an SI defining constant, not `Torr` or `mmHg` as
  a second name, not a FormalClaim, and not P3N. The molar volume of
  ideal gas is a later table row and is not stored. Electron mass is
  still not stored (`10^{42}` overflows `i128`). CODATA 2022 prints
  the same exact 101 325 Pa; there is no last-digit trap.
  `physis_model` `standard_atmosphere()` Qty locksteps to the integer
  `to_f64` of that pascal count. Adding `atm` to LEDGER changes the
  ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`,
  `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`, `mt_mp`,
  `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`, `m_h_u`,
  `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`, `mu_h_muB`,
  `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`, `mu0h_mup`,
  `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`, `m_alpha_c2_MeV`,
  `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`, `m_u_c2`, `m_u_c2_MeV`,
  `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, and `p0` hashes are unchanged.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4.
  Verified: `atm` hash 0bb71f6a38e105217751cdd7fb11c3cff6eefd0cd8aab0ae2ae366d28119ba2d; node 457453b8c8f008730f56e5cb11b521456907ef056c284351bf23c75da42258f6;
  ledger node deda8ff663f99b494886db6b438b5211c0a06f4216df08d29e97e42e1b90fca0. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`,
  `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, `NAe`, and
  `p0` hashes and nodes unchanged.

- **CODATA 2018 standard-state pressure is an exact Ratio.**
  `physis-constants` versions `p0` as the exact PHYSICOCHEMICAL
  conventional pressure `100000` Pa from JPCRD 50, 033105 table XXXI.
  JPCRD prints no symbol; `p0` is the ledger name (ASCII for p°).
  This is not Faraday `NAe`, not the standard atmosphere `101325` Pa,
  not Newtonian `G`, not an SI defining constant, not `bar` as a
  second name, not trust-tier P0, not a FormalClaim, and not P3N.
  Standard atmosphere is a later table row and is not stored.
  Electron mass is still not stored (`10^{42}` overflows `i128`).
  CODATA 2022 prints the same exact 100 000 Pa; there is no last-digit
  trap. `physis_model` `standard_state_pressure()` Qty locksteps to
  the integer `to_f64` of that pascal count. Adding `p0` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`, `mt_mp`,
  `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`, `m_h_u`,
  `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`, `mu_h_muB`,
  `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`, `mu0h_mup`,
  `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`, `m_alpha_c2_MeV`,
  `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`, `m_u_c2`, `m_u_c2_MeV`,
  `M_u`, `M_12C`, `NAh`, `NAk`, and `NAe` hashes are unchanged.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4.
  Verified: `p0` hash 21e32b495cec5be6d2655b0f3fc2e6d27541b76c8582ad17562002a30b5a1217; node 85d191df5c952fa73292a6084ae6813f7f99785b624682af2fa135aab85e45f4;
  ledger node e5947dbe916ff12ab74ea9948909f21852fe45a1f82fe5e401ad4118b48f7cb6. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`,
  `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, `NAk`, and `NAe`
  hashes and nodes unchanged.

- **CODATA 2018 Faraday constant is an exact Ratio.**
  `physis-constants` versions `NAe` as the exact SI 2019 product
  `N_A × e` `96485.3321233100184` C mol⁻¹ from JPCRD 50, 033105 table
  XXXI (PHYSICOCHEMICAL). The table prints an ellipsis; the ledger
  stores the full terminating decimal, not the truncated display
  digits. This is not elementary charge `e`, not Avogadro `N_A`, not
  molar gas `NAk`, not molar Planck `NAh`, not Hartree, not Maxwell
  Faraday `dF=0`, not an SI defining constant, not a FormalClaim that
  reconstructs `N_A × e` from live lookups, not the Thomson cross
  section, and not P3N. JPCRD also writes `F`; that is not a second
  ledger name. Standard-state pressure is a later table row and is not
  stored. Electron mass is still not stored (`10^{42}` overflows
  `i128`). The product fits Ratio (`10^{13}`). CODATA 2022 prints the
  same SI-exact ellipsis; there is no last-digit trap. `physis_model`
  `faraday_constant()` Qty locksteps to the IEEE rounding of that SI
  decimal (`SciExact::to_f64`). Adding `NAe` to LEDGER changes the
  ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`,
  `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`, `mt_mp`,
  `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`, `m_h_u`,
  `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`, `mu_h_muB`,
  `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`, `mu0h_mup`,
  `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`, `m_alpha_c2_MeV`,
  `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`, `m_u_c2`, `m_u_c2_MeV`,
  `M_u`, `M_12C`, `NAh`, and `NAk` hashes are unchanged. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4.
  Verified: `NAe` hash dbc99e6a827156d94029a58f2134e4f2833c556723a089cc2a9e462f3fa76ba4; node 4146991435f7e1aeca08cafaa6c26cd8f8947a49f7c1ca32df3eac4d36bb060d;
  ledger node 2b8808478cff2036f511122b68700fa51eabb7c5c5956eb948d4147c25a6018b. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`,
  `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `NAh`, and `NAk` hashes and
  nodes unchanged.

- **CODATA 2018 molar gas constant is an exact Ratio.**
  `physis-constants` versions `NAk` as the exact SI 2019 product
  `N_A × k` `8.31446261815324` J mol⁻¹ K⁻¹ from JPCRD 50, 033105 table
  XXXI (PHYSICOCHEMICAL). The table prints an ellipsis; the ledger
  stores the full terminating decimal, not the truncated display
  digits. This is not Boltzmann `k`, not Avogadro `N_A`, not molar
  Planck `NAh`, not Hartree, not an SI defining constant, not a
  FormalClaim that reconstructs `N_A × k` from live lookups, not the
  Thomson cross section, and not P3N. JPCRD also writes `R`; that is
  not a second ledger name. Faraday constant is a later table row and
  is not stored. Electron mass is still not stored (`10^{42}` overflows
  `i128`). The product fits Ratio (`10^{14}`). CODATA 2022 prints the
  same SI-exact ellipsis; there is no last-digit trap. `physis_model`
  `molar_gas_constant()` Qty locksteps to the IEEE rounding of that SI
  decimal (`SciExact::to_f64`). Adding `NAk` to LEDGER changes the
  ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`,
  `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`, `mt_mp`,
  `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`, `m_h_u`,
  `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`, `mu_h_muB`,
  `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`, `mu0h_mup`,
  `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`, `m_alpha_c2_MeV`,
  `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`, `m_u_c2`, `m_u_c2_MeV`,
  `M_u`, `M_12C`, and `NAh` hashes are unchanged. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4.
  Verified: `NAk` hash 28c95a46c67bec666b887658cc44664000bf821eac09b9023cf401b89231efc3; node ebdfd5d64f47a7b38faa4e4907ac5de394242e039670478b1b8558ffe1a8a853;
  ledger node 6a8a0a101960508fc58bf426868dea9421db52c10b5c919b0f0f3dc8a14b8790. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`,
  `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, and `NAh` hashes and nodes
  unchanged.

- **CODATA 2018 molar Planck constant is an exact Ratio.**
  `physis-constants` versions `NAh` as the exact SI 2019 product
  `N_A × h` `3.99031271289343140×10^{-10}` J Hz⁻¹ mol⁻¹ from JPCRD 50,
  033105 table XXXI (PHYSICOCHEMICAL). The table prints an ellipsis;
  the ledger stores the full terminating decimal, not the truncated
  display digits. This is not Planck `h`, not Avogadro `N_A`, not `ħ`,
  not Hartree, not `M_u`, not `M_12C`, not an SI defining constant, not
  a FormalClaim that reconstructs `N_A × h` from live lookups, not the
  Thomson cross section, and not P3N. Molar gas constant is a later
  table row and is not stored. Electron mass is still not stored
  (`10^{42}` overflows `i128`). The product fits Ratio (`10^{27}`);
  Planck `h` does not (`10^{42}`). CODATA 2022 prints the same SI-exact
  ellipsis; there is no last-digit trap. `physis_model`
  `molar_planck_constant()` Qty locksteps to the IEEE rounding of that
  SI decimal (`SciExact::to_f64`, not reduced `Ratio::to_f64`). Adding
  `NAh` to LEDGER changes the ledger bundle pin. The `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`,
  `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`,
  `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`,
  `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`,
  `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`,
  `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`,
  `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`, `mt_mp`,
  `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`, `m_h_u`,
  `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`, `mu_h_muB`,
  `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`, `mu0h_mup`,
  `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`, `m_alpha_c2_MeV`,
  `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`, `m_u_c2`, `m_u_c2_MeV`,
  `M_u`, and `M_12C` hashes are unchanged. Theories still evaluate with
  `f64` Qty. That is not a kernel proof, not Canonical, not P4. Encode
  pins unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `NAh` hash 9290f6b333a3a26c429b761769bd641d7a642b68d6e34cbea852119c170d6228; node 2bcc73daab70d1b719c651342dbafc5473a4ee650ce29aacf4e3109bcfa6ebd1;
  ledger node 77ca32b2f5b89e3a7b234b3ae3182b97e0ca77dd73716ed5920b306a6ee0efba. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`,
  `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, `M_u`, and `M_12C` hashes and nodes unchanged.

- **CODATA 2018 molar mass of carbon-12 is a one-sigma Interval.**
  `physis-constants` versions `M_12C` as the CODATA 2018 hull
  `11.9999999958(36)×10^{-3}` kg mol⁻¹ from JPCRD 50, 033105 table XXXI
  (PHYSICOCHEMICAL). This is not the molar mass constant `M_u`, not
  alpha-particle `M_alpha`, not helion `M_h`, not triton `M_t`, not
  deuteron `M_d`, not neutron `M_n`, not proton `M_p`, not electron
  `M_e`, not muon `M_mu`, not the kg hull `m_u`, not Avogadro `N_A`,
  not a certificate that this equals `12 × M_u`, not an SI defining
  Ratio, not the Thomson cross section, and not P3N. Molar Planck
  constant is a later table row and is not stored. Electron mass is
  still not stored (`10^{42}` overflows `i128`). Decade `10^{13}`
  (`10^{12}` is the 10× trap). This is not the CODATA 2022 last-digit
  `126`. `physis_model` `carbon_12_molar_mass()` Qty locksteps to the
  recommended centre inside the hull. Adding `M_12C` to LEDGER changes
  the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`,
  `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, and `M_u` hashes are unchanged. Theories
  still evaluate with `f64` Qty. That is not a kernel proof, not
  Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4.
  Verified: `M_12C` hash bec80fb1c51bead2000a5ba56e2fd680fd79d5538dd3e0e0cd9ceca1fb983d43; node 2ad548958aa8e044a9f42f941c3a105621d41ed86c462eb8408fc30b76fdad74;
  ledger node 25d42dc4c51680cdc68bed4f27e1aa00b36e697f27b7aba9fd11dadbac782eff. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, `m_u_c2_MeV`, and `M_u` hashes and nodes unchanged.

- **CODATA 2018 molar mass constant is a one-sigma Interval.**
  `physis-constants` versions `M_u` as the CODATA 2018 hull
  `0.99999999965(30)×10^{-3}` kg mol⁻¹ from JPCRD 50, 033105 table XXXI
  (PHYSICOCHEMICAL). This is not alpha-particle `M_alpha`, not helion
  `M_h`, not triton `M_t`, not deuteron `M_d`, not neutron `M_n`, not
  proton `M_p`, not electron `M_e`, not muon `M_mu`, not the kg hull
  `m_u`, not Avogadro `N_A`, not vacuum permeability `mu0`, not a
  certificate that this equals `N_A × m_u`, not an SI defining Ratio,
  not the Thomson cross section, and not P3N. Molar mass of carbon-12
  is a later table row and is not stored. Electron mass is still not
  stored (`10^{42}` overflows `i128`). Decade `10^{14}` (`10^{13}` is
  the 10× trap). This is not the CODATA 2022 last-digit `105`.
  `physis_model` `molar_mass_constant()` Qty locksteps to the
  recommended centre inside the hull. Adding `M_u` to LEDGER changes
  the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`,
  `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, and `m_u_c2_MeV` hashes are unchanged. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged.
  P3N count stays 4.
  Verified: `M_u` hash db927829cb6a1d796a00ab6509b3a9faf0e2f09ed7bb3dc5aca6154abb9e388e; node 20a2cd8642b0035f68a9c0487b930415e5472c5bbd393b2f97049772e6bea642;
  ledger node eef8e00dc7d3cd57c5512d876fb7331a37c288fa13dd5fd46105e6cedd3d3eae. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`,
  `m_u_c2`, and `m_u_c2_MeV` hashes and nodes unchanged.

- **CODATA 2018 atomic mass constant energy equivalent in MeV is a one-sigma Interval.**
  `physis-constants` versions `m_u_c2_MeV` as the CODATA 2018 hull
  `931.49410242(28)` MeV from JPCRD 50, 033105 table XXXI
  (PHYSICOCHEMICAL). This is not the joule hull `m_u_c2`, not proton
  `m_p_c2_MeV`, not neutron `m_n_c2_MeV`, not deuteron `m_d_c2_MeV`, not
  triton `m_t_c2_MeV`, not helion `m_h_c2_MeV`, not alpha-particle
  `m_alpha_c2_MeV`, not muon `m_mu_c2_MeV`, not Hartree, not the exact
  electronvolt Ratio, not a certificate that this converts the joule
  hull, not an SI defining Ratio, not the Thomson cross section, and
  not P3N. Molar mass constant is a later table row and is not stored.
  Electron mass is still not stored (`10^{42}` overflows `i128`). Decade
  `10^{8}` (`10^{7}` is the 10× trap). This is not the CODATA 2022
  last-digit `372`. `physis_model`
  `atomic_mass_constant_energy_equivalent_in_mev()` Qty locksteps to the
  recommended centre inside the hull. Adding `m_u_c2_MeV` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`,
  `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`, and
  `m_u_c2` hashes are unchanged. Theories still evaluate with `f64`
  Qty. That is not a kernel proof, not Canonical, not P4. Encode pins
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `m_u_c2_MeV` hash cbd8cdc0c0c358c2a7204f343ab90d33e1c342820eb596fdb67b330d872c9d3e; node 1e05445f79b116401e18e22766ad853e02804628ad5a00e7fd61c000fe93057f;
  ledger node 02cc15229e173a6347de42b4bcc357166b06e7c2696da2d630baf57907c9ff76. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`, and
  `m_u_c2` hashes and nodes unchanged.

- **CODATA 2018 atomic mass constant energy equivalent is a one-sigma Interval.**
  `physis-constants` versions `m_u_c2` as the CODATA 2018 hull
  `1.49241808560(45)×10^{-10}` J from JPCRD 50, 033105 table XXXI
  (PHYSICOCHEMICAL). This is not the kg hull `m_u`, not proton
  `m_p_c2`, not neutron `m_n_c2`, not deuteron `m_d_c2`, not triton
  `m_t_c2`, not helion `m_h_c2`, not alpha-particle `m_alpha_c2`, not
  muon `m_mu_c2`, not Hartree `Eh`, not the exact electronvolt Ratio,
  not a reconstructed `m_u c²` certificate, not an SI defining Ratio,
  not the Thomson cross section, and not P3N. The MeV conversion is a
  later table row and is not stored. Electron mass is still not stored
  (`10^{42}` overflows `i128`). Decade `10^{21}` matching the printed
  11-decimal × `10^{-10}` form (`10^{20}` is the 10× trap: `σ` would
  not be an integer). This is not the CODATA 2022 last-digit `768`.
  `physis_model` `atomic_mass_constant_energy_equivalent()` Qty
  locksteps to the recommended centre inside the hull. Adding `m_u_c2`
  to LEDGER changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`,
  `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, and `m_u`
  hashes are unchanged. Theories still evaluate with `f64` Qty. That
  is not a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `m_u_c2` hash e3e57faf62ef6c5ef37e31fd959eedc747be1444953bb83f39c73590b0454750; node d6c812670024baec7c1ae41bbb4961d859585d2e1b4c0ee4596ed01d8eaaa684;
  ledger node 4532705744b35fde0be57c1f2b60aee321d96e47af2602324bee0f2a147f888e. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, and `m_u`
  hashes and nodes unchanged.

- **CODATA 2018 atomic mass constant is a one-sigma Interval.**
  `physis-constants` versions `m_u` as the CODATA 2018 hull
  `1.66053906660(50)×10^{-27}` kg from JPCRD 50, 033105 table XXXI
  (PHYSICOCHEMICAL). This is not proton `m_p`, not neutron `m_n`, not
  deuteron `m_d`, not triton `m_t`, not helion `m_h`, not alpha-particle
  `m_alpha`, not muon `m_mu`, not alpha-particle molar mass `M_alpha`,
  not Avogadro `N_A`, not vacuum permeability `mu0`, not the JPCRD
  symbol `mu` as a ledger name, not unified atomic mass unit `u` under a
  second name, not an SI defining Ratio, not the Thomson cross section,
  and not P3N. Energy-equivalent rows are later table rows and are not
  stored. Electron mass is still not stored (`10^{42}` overflows
  `i128`). Decade `10^{38}` matching the printed 11-decimal × `10^{-27}`
  form (`10^{36}` is the 10× trap; `10^{37}` reduces the same hull
  because the last printed digit is 0). This is not the CODATA 2022
  last-digit `892`. `physis_model` `atomic_mass_constant()` Qty locksteps
  to the recommended centre inside the hull. Adding `m_u` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`,
  `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, and `M_alpha` hashes are
  unchanged. Theories still evaluate with `f64` Qty. That is not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `m_u` hash fcefc139b85d5be198ab911fed33049d37641b01dcd0b87e12630db6dfd467d3; node 2771039a06696ad5cd983ff13f28411b8d634e1d890869bb4cf6a6d8e9536b16;
  ledger node 68e9be2f659189651fb64130cc624d100d4b3fa07a43eb783268764679e09ca3. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, and `M_alpha` hashes and
  nodes unchanged.

- **CODATA 2018 alpha particle molar mass is a one-sigma Interval.**
  `physis-constants` versions `M_alpha` as the CODATA 2018 hull
  `4.0015061777(12)×10^{-3}` kg mol⁻¹ from JPCRD 50, 033105 table XXXI
  (Alpha particle, a). This is not helion `M_h`, not triton `M_t`, not
  deuteron `M_d`, not neutron `M_n`, not proton `M_p`, not electron
  `M_e`, not muon `M_mu`, not the kg hull `m_alpha`, not the u-row
  `m_alpha_u`, not a certificate that this equals `N_A × m_alpha`, not
  an SI defining Ratio, not the Thomson cross section, and not P3N.
  Relative atomic mass is the same digits as the u-row and is not stored
  under a second name. PHYSICOCHEMICAL rows are later table rows and are
  not stored. Electron mass is still not stored (`10^{42}` overflows
  `i128`). Decade `10^{13}` (`10^{12}` is the 10× trap). This is not the
  CODATA 2022 last-digit `1833`. `physis_model` `alpha_particle_molar_mass()`
  Qty locksteps to the recommended centre inside the hull. Adding
  `M_alpha` to LEDGER changes the ledger bundle pin. The `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`,
  `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`,
  `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`,
  `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`,
  `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`,
  `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`,
  `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, and `malpha_mp` hashes are unchanged. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4.
  Verified: `M_alpha` hash 247f1e1ac48e536f49b0e9ea42f1233960dc446e599c1177469c80d2b50fddf3; node d44a8e073d6b4105db288a959ad5cb0adf3a174239e2753a68e5c7f7981a83ac;
  ledger node eda64b62c6716bdb4722547fea1200a960b3cb8a85f6049517a5c78ff6f548bb. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, `malpha_me`, and `malpha_mp` hashes and nodes unchanged.

- **CODATA 2018 alpha particle-proton mass ratio is a one-sigma Interval.**
  `physis-constants` versions `malpha_mp` as the CODATA 2018 hull
  `3.97259969009(22)` from JPCRD 50, 033105 table XXXI (Alpha particle,
  a). This is not helion `mh_mp`, not triton `mt_mp`, not deuteron
  `md_mp`, not neutron `mn_mp`, not proton-neutron `mp_mn`, not muon
  `mmu_mp`, not alpha-electron `malpha_me`, not kg hull `m_alpha`, not
  proton mass `m_p`, not a reconstructed sibling-mass certificate, not
  an SI defining Ratio, not the Thomson cross section, and not P3N.
  Molar mass is a later table row and is not stored. Electron mass is
  still not stored (`10^{42}` overflows `i128`). Decade `10^{11}`
  (`10^{10}` is the 10× trap). This is not the CODATA 2022 last-digit
  `252`. `physis_model` `alpha_particle_proton_mass_ratio()` Qty
  locksteps to the recommended centre inside the hull. Adding
  `malpha_mp` to LEDGER changes the ledger bundle pin. The `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`,
  `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`,
  `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`,
  `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`,
  `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`,
  `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`,
  `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, and `malpha_me` hashes are unchanged. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4.
  Verified: `malpha_mp` hash 0c31195c0e868eb3e6b4a54c10ed662a1075d58cad2e565ad5ec5f389f7c567d; node 172c865937b6a232151daee8e336fcef43c518721c21e67abbdc7bf95b044801;
  ledger node b33c0f610446cf18bf33c48ea9169a8d3ba586181f0fb173647b258b4b66d288. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`,
  `m_alpha_c2_MeV`, and `malpha_me` hashes and nodes unchanged.

- **CODATA 2018 alpha particle-electron mass ratio is a one-sigma Interval.**
  `physis-constants` versions `malpha_me` as the CODATA 2018 hull
  `7294.29954142(24)` from JPCRD 50, 033105 table XXXI (Alpha particle,
  a). This is not electron-alpha `me_malpha`, not helion `mh_me`, not
  triton `mt_me`, not deuteron `md_me`, not neutron `mn_me`, not proton
  `mp_me`, not muon `mmu_me`, not kg hull `m_alpha`, not MeV hull
  `m_alpha_c2_MeV`, not a certificate that the stored centres invert, not
  an SI defining Ratio, not the Thomson cross section, and not P3N. The
  alpha-proton mass ratio is a later table row and is not stored.
  Electron mass is still not stored (`10^{42}` overflows `i128`). Decade
  `10^{8}` (`10^{7}` is the 10× trap). This is not the CODATA 2022
  last-digit `71`. `physis_model` `alpha_particle_electron_mass_ratio()`
  Qty locksteps to the recommended centre inside the hull. Adding
  `malpha_me` to LEDGER changes the ledger bundle pin. The `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`,
  `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`,
  `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`,
  `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`,
  `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`,
  `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`,
  `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`, and
  `m_alpha_c2_MeV` hashes are unchanged. Theories still evaluate with
  `f64` Qty. That is not a kernel proof, not Canonical, not P4. Encode
  pins unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `malpha_me` hash 72ecb3fbc0d48ab53a5d6f3a08cbe581b7909dc03ece21eb07cb35887e7c68c1; node e78c04dd2b05c41c6d7bcd91830d202d14c3dfe104743e86305445a5d0b52937;
  ledger node b27b9b14315a1979bd15e1af94334d502858c365be90634195d99711c3783fc2. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`, and
  `m_alpha_c2_MeV` hashes and nodes unchanged.

- **CODATA 2018 alpha particle mass energy equivalent in MeV is a one-sigma Interval.**
  `physis-constants` versions `m_alpha_c2_MeV` as the CODATA 2018 hull
  `3727.3794066(11)` MeV from JPCRD 50, 033105 table XXXI (Alpha
  particle, a). This is not joule hull `m_alpha_c2`, not helion
  `m_h_c2_MeV`, not triton `m_t_c2_MeV`, not deuteron `m_d_c2_MeV`, not
  neutron `m_n_c2_MeV`, not proton `m_p_c2_MeV`, not muon `m_mu_c2_MeV`,
  not Hartree `Eh`, not the exact electronvolt Ratio, not a certificate
  of a reconstruction from sibling masses, not an SI defining Ratio, not
  the Thomson cross section, and not P3N. The alpha-electron mass ratio
  is a later table row and is not stored. Electron mass is still not
  stored (`10^{42}` overflows `i128`). Decade `10^{7}` (`10^{6}` is the
  10× trap). This is not the CODATA 2022 last-digit `4118`.
  `physis_model` `alpha_particle_mass_energy_equivalent_in_mev()` Qty
  locksteps to the recommended centre inside the hull. Adding
  `m_alpha_c2_MeV` to LEDGER changes the ledger bundle pin. The `G`,
  `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`,
  `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`,
  `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`,
  `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`,
  `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`,
  `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`,
  `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, and `m_alpha_c2`
  hashes are unchanged. Theories still evaluate with `f64` Qty. That is
  not a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `m_alpha_c2_MeV` hash 5a924d0532dc37f1e318a7dd27ad318160dd7a606d6ddc75fac704b5277a4a42; node cfc774331f8e78a1784d3585d0831969df0d21dc26cd5be819922d0706b33d3b;
  ledger node 5827f99c4ba7bde84f38f56e09dcbe10545317b3f9fde432bf691870bb2a4523. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, and `m_alpha_c2`
  hashes and nodes unchanged.

- **CODATA 2018 alpha particle mass energy equivalent is a one-sigma Interval.**
  `physis-constants` versions `m_alpha_c2` as the CODATA 2018 hull
  `5.9719201914(18)×10^{-10}` J from JPCRD 50, 033105 table XXXI
  (Alpha particle, a). This is not kg hull `m_alpha`, not u-row
  `m_alpha_u`, not helion `m_h_c2`, not triton `m_t_c2`, not deuteron
  `m_d_c2`, not neutron `m_n_c2`, not proton `m_p_c2`, not muon
  `m_mu_c2`, not Rydberg `hcRinf`, not Hartree `Eh`, not the exact
  electronvolt Ratio, not a certificate of a reconstruction from sibling
  masses, not an SI defining Ratio, not the Thomson cross section, and
  not P3N. The MeV conversion is a later table row and is not stored.
  Mass-ratio and molar-mass rows are later table rows and are not
  stored. Electron mass is still not stored (`10^{42}` overflows
  `i128`). Decade `10^{20}` (`10^{19}` is the 10× trap). This is not the
  CODATA 2022 last-digit `1997`. `physis_model`
  `alpha_particle_mass_energy_equivalent()` Qty locksteps to the
  recommended centre inside the hull. Adding `m_alpha_c2` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`,
  `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, and `m_alpha_u` hashes are
  unchanged. Theories still evaluate with `f64` Qty. That is not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `m_alpha_c2` hash f3850fe7ecb1fd455ca3422b528eda05a5caf07f149b85c1270102b987fc723e; node 64a4c00516144551ce261049edb83f3f09431ed32b0da56416dd7f02c51d32ac;
  ledger node 4fc4fa68bbefb2bfbb79dc00a927f2888a14f894485deea0445c47661a502201. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`,
  `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`,
  `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`,
  `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`,
  `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`,
  `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`,
  `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`,
  `mu_t_muN`, `gt`, `m_h`, `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`,
  `mh_mp`, `M_h`, `mu_h`, `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`,
  `mu0h_muB`, `mu0h_muN`, `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, and
  `m_alpha_u` hashes and nodes unchanged.

- **CODATA 2018 alpha particle mass in u is a one-sigma Interval.**
  `physis-constants` versions `m_alpha_u` as the CODATA 2018 hull
  `4.001506179127(63)` u from JPCRD 50, 033105 table XXXI
  (Alpha particle, a). This is not kg hull `m_alpha`, not helion `m_h_u`,
  not triton `m_t_u`, not deuteron `m_d_u`, not neutron `m_n_u`, not
  proton `m_p_u`, not muon `m_mu_u`, not electron-alpha `me_malpha`, not
  electron molar mass `M_e`, not relative atomic mass under a different
  name, not a certificate of a reconstruction from sibling masses, not
  an SI defining Ratio, not the Thomson cross section, and not P3N.
  Energy-equivalent, MeV, mass-ratio, and molar-mass rows are later
  table rows and are not stored. Electron mass is still not stored
  (`10^{42}` overflows `i128`). Decade `10^{12}` (`10^{11}` is the 10×
  trap). This is not the CODATA 2022 last-digit `129`. `physis_model`
  `alpha_particle_mass_in_u()` Qty locksteps to the recommended centre
  inside the hull. Adding `m_alpha_u` to LEDGER changes the ledger
  bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`,
  `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`,
  `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`,
  `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`,
  `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`,
  `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`,
  `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`,
  `mu_t_muN`, `gt`, `m_h`, `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`,
  `mh_mp`, `M_h`, `mu_h`, `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`,
  `mu0h_muB`, `mu0h_muN`, `mu0h_mup`, `mu0h_mu0p`, and `m_alpha` hashes
  are unchanged. Theories still evaluate with `f64` Qty. That is not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `m_alpha_u` hash 69b88fb739109ee090cb94bf699ce2a8b76c941e8b279c5b9f28b6273fa67935; node 2f8cdcc85902aaf6946137138a2db30503ea12910fbedb15772898d906a7f5df;
  ledger node 057c3c1bab90a3602d4f7f0a76639800493cd347af3d5aa29f33547be3bd1be4. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`,
  `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`,
  `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`,
  `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`,
  `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`,
  `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`,
  `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`,
  `mu_t_muN`, `gt`, `m_h`, `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`,
  `mh_mp`, `M_h`, `mu_h`, `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`,
  `mu0h_muB`, `mu0h_muN`, `mu0h_mup`, `mu0h_mu0p`, and `m_alpha` hashes
  and nodes unchanged.

- **CODATA 2018 alpha particle mass is a one-sigma Interval.**
  `physis-constants` versions `m_alpha` as the CODATA 2018 signed hull
  `6.6446573357(20)×10^{-27}` kg from JPCRD 50, 033105 table XXXI
  (Alpha particle, a). This is not helion mass `m_h`, not triton `m_t`,
  not deuteron `m_d`, not neutron `m_n`, not proton `m_p`, not muon
  `m_mu`, not electron-alpha mass ratio `me_malpha`, not a reconstructed
  sibling-mass certificate, not an SI defining Ratio, not the Thomson
  cross section, and not P3N. The u-row, energy equivalent, mass ratios,
  and molar mass are later table rows and are not stored. Relative
  atomic mass is not stored under a second name. Gyromagnetic ratios
  cite ħ and are not stored. Electron mass is still not stored (`10^{42}`
  overflows `i128`). Decade `10^{37}` (`10^{36}` is the 10× trap). This
  is not the CODATA 2022 last-digit `3450`. `physis_model`
  `alpha_particle_mass()` Qty locksteps to the recommended centre inside
  the hull. Adding `m_alpha` to LEDGER changes the ledger bundle pin.
  The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`,
  `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`,
  `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`,
  `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`,
  `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`,
  `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`,
  `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`,
  `mu_t_muN`, `gt`, `m_h`, `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`,
  `mh_mp`, `M_h`, `mu_h`, `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`,
  `mu0h_muB`, `mu0h_muN`, `mu0h_mup`, and `mu0h_mu0p` hashes are
  unchanged. Theories still evaluate with `f64` Qty. That is not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `m_alpha` hash 8f3ec14a8381c0b83aba64d6f42a44dcd12b59e65bcf4ff11ab7edb36b4296c4; node 162b3c933eb5fc4e331a1d3b9a597f0025d94678c7c0a3ea6916c9a8249d554f;
  ledger node a14cf43ad3620d5423a72bdfa0c6d2b10f53d300639b9492d47d039b32fa3f69. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`,
  `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`,
  `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`,
  `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`,
  `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`,
  `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`,
  `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`,
  `mu_t_muN`, `gt`, `m_h`, `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`,
  `mh_mp`, `M_h`, `mu_h`, `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`,
  `mu0h_muB`, `mu0h_muN`, `mu0h_mup`, and `mu0h_mu0p` hashes and nodes
  unchanged.

- **CODATA 2018 shielded helion to shielded proton magnetic-moment ratio is a one-sigma Interval.**
  `physis-constants` versions `mu0h_mu0p` as the CODATA 2018 signed hull
  `−0.7617861313(33)` from JPCRD 50, 033105 table XXXI (Helion, h; gas,
  H2O, spheres, 25 °C). This is not free-proton ratio `mu0h_mup`, not
  shielded helion magnetic moment `mu0h`, not shielded proton `mu0p`,
  not neutron to shielded-proton `mu_n_mu0p`, not electron to
  shielded-proton `mu_e_mu0p`, not electron to shielded-helion
  `mu_e_mu0h`, not helion-proton mass ratio `mh_mp`, not a reconstructed
  `μ′_h/μ′_p` certificate, not an SI defining Ratio, not the Thomson
  cross section, and not P3N. Gyromagnetic ratios cite ħ and are not
  stored. Electron mass is still not stored (`10^{42}` overflows
  `i128`). Decade `10^{10}` (`10^{9}` is the 10× trap). This is not the
  CODATA 2022 last-digit `1334`. `physis_model`
  `shielded_helion_to_shielded_proton_magnetic_moment_ratio()` Qty
  locksteps to the recommended centre inside the hull. Adding
  `mu0h_mu0p` to LEDGER changes the ledger bundle pin. The `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`,
  `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`,
  `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`,
  `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`,
  `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`,
  `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`,
  `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`, and
  `mu0h_mup` hashes are unchanged. Theories still evaluate with `f64`
  Qty. That is not a kernel proof, not Canonical, not P4. Encode pins
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `mu0h_mu0p` hash 18c53bbc40a6c0c95dc9af5019cf6a9482b3ae1918cd6381c949250e95bebd90; node b92f8c9af39f5e7287562555b7531498bab3028db3f4bf11816d29ec3b323ef6;
  ledger node 78f731a52e6bfdb8949fcca914de9fb3ee5ce1bc95cf090c525bb07a2846bc35. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`,
  `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`,
  `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`,
  `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`,
  `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`,
  `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`,
  `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`,
  `mu_t_muN`, `gt`, `m_h`, `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`,
  `mh_mp`, `M_h`, `mu_h`, `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`,
  `mu0h_muB`, `mu0h_muN`, and `mu0h_mup` hashes and nodes unchanged.

- **CODATA 2018 shielded helion to proton magnetic-moment ratio is a one-sigma Interval.**
  `physis-constants` versions `mu0h_mup` as the CODATA 2018 signed hull
  `−0.7617665618(89)` from JPCRD 50, 033105 table XXXI (Helion, h; gas,
  sphere, 25 °C). This is not shielded helion magnetic moment `mu0h`,
  not nuclear-magneton ratio `mu0h_muN`, not neutron-proton `mu_n_mup`,
  not electron-proton `mu_e_mup`, not deuteron-proton `mu_d_mup`, not
  electron to shielded-helion `mu_e_mu0h`, not helion-proton mass ratio
  `mh_mp`, not a reconstructed `μ′_h/μ_p` certificate, not an SI
  defining Ratio, not the Thomson cross section, and not P3N. The
  shielded helion to shielded proton ratio is a later table row and is
  not stored. Gyromagnetic ratios cite ħ and are not stored. Electron
  mass is still not stored (`10^{42}` overflows `i128`). Decade `10^{10}`
  (`10^{9}` is the 10× trap). This is not the CODATA 2022 last-digit
  `57721`. `physis_model` `shielded_helion_to_proton_magnetic_moment_ratio()`
  Qty locksteps to the recommended centre inside the hull. Adding
  `mu0h_mup` to LEDGER changes the ledger bundle pin. The `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`,
  `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`,
  `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`,
  `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`,
  `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`,
  `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`,
  `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, and `mu0h_muN`
  hashes are unchanged. Theories still evaluate with `f64` Qty. That is
  not a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `mu0h_mup` hash 2a8e0963d82d303bc3cc41ef13d266a3e6aa732e724579beb1361a7cf849c80f; node 12d5e03dfa5101a9f92901c642bd9870cbe5340890fab192a12a0e18a36d9d97;
  ledger node e18157e2e0e5de2157e52a1e8c1996acf606311e98a00956afa20cffb844b076. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`,
  `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`,
  `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`,
  `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`,
  `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`,
  `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`,
  `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`,
  `mu_t_muN`, `gt`, `m_h`, `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`,
  `mh_mp`, `M_h`, `mu_h`, `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`,
  `mu0h_muB`, and `mu0h_muN` hashes and nodes unchanged.

- **CODATA 2018 shielded helion nuclear-magneton ratio is a one-sigma Interval.**
  `physis-constants` versions `mu0h_muN` as the CODATA 2018 signed hull
  `−2.127497719(25)` from JPCRD 50, 033105 table XXXI (Helion, h; gas,
  sphere, 25 °C). This is not free helion nuclear-magneton ratio
  `mu_h_muN`, not shielded Bohr-magneton ratio `mu0h_muB`, not shielded
  helion magnetic moment `mu0h`, not shielded proton `mu0p_muN`, not
  triton `mu_t_muN`, not helion g-factor `gh`, not a reconstructed
  `μ′_h/μ_N` or `gh/2` certificate, not an SI defining Ratio, not the
  Thomson cross section, and not P3N. JPCRD prints different digits from
  `mu_h_muN` because this is the shielded row. Shielded helion to proton
  ratio rows are later table rows and are not stored. Gyromagnetic
  ratios cite ħ and are not stored. Electron mass is still not stored
  (`10^{42}` overflows `i128`). Decade `10^{9}` (`10^{8}` is the 10×
  trap). This is not the CODATA 2022 last-digit `7624`. `physis_model`
  `shielded_helion_magnetic_moment_to_nuclear_magneton()` Qty locksteps
  to the recommended centre inside the hull. Adding `mu0h_muN` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`,
  `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`,
  `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`,
  `mu_t_muN`, `gt`, `m_h`, `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`,
  `mh_mp`, `M_h`, `mu_h`, `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, and
  `mu0h_muB` hashes are unchanged. Theories still evaluate with `f64`
  Qty. That is not a kernel proof, not Canonical, not P4. Encode pins
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `mu0h_muN` hash ee1cdad515f45e0be5c09c52afd3fbe0f4e5aab9b26d25218e5d09de60f2b4b5; node b9f0dcbb3f7f4a7d7a965e00f7d25cf6f8229b62aaa1ac187f07c85387046b47;
  ledger node 31eebe0b09c0bbb430da01b1c70314d03aa181bdc46928145baf24cfcd95e201. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`,
  `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`,
  `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`,
  `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`,
  `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`,
  `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`,
  `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`,
  `mu_t_muN`, `gt`, `m_h`, `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`,
  `mh_mp`, `M_h`, `mu_h`, `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, and
  `mu0h_muB` hashes and nodes unchanged.

- **CODATA 2018 shielded helion Bohr-magneton ratio is a one-sigma Interval.**
  `physis-constants` versions `mu0h_muB` as the CODATA 2018 signed hull
  `−1.158671471(14)×10^{-3}` from JPCRD 50, 033105 table XXXI
  (Helion, h; gas, sphere, 25 °C). This is not free helion Bohr-magneton
  ratio `mu_h_muB`, not shielded helion magnetic moment `mu0h`, not
  shielded proton Bohr-magneton ratio `mu0p_muB`, not triton
  `mu_t_muB`, not electron `mu_e_muB`, not vacuum permeability `mu0`,
  not helion g-factor `gh`, not a reconstructed `μ′_h/μ_B` certificate,
  not an SI defining Ratio, not the Thomson cross section, and not P3N.
  Nuclear-magneton ratio rows are later table rows and are not stored.
  Gyromagnetic ratios cite ħ and are not stored. Electron mass is still
  not stored (`10^{42}` overflows `i128`). Decade `10^{12}` (`10^{11}`
  is the 10× trap). This is not the CODATA 2022 last-digit `49457`.
  `physis_model` `shielded_helion_magnetic_moment_to_bohr_magneton()` Qty
  locksteps to the recommended centre inside the hull. Adding
  `mu0h_muB` to LEDGER changes the ledger bundle pin. The `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`,
  `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`,
  `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`,
  `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`,
  `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`,
  `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`,
  `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`,
  `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`,
  `mu_h_muB`, `mu_h_muN`, `gh`, and `mu0h` hashes are unchanged.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4.
  Verified: `mu0h_muB` hash 9265606a87f0abee2603a308bba4322bd8fc1ac40513d8e8bb441cbeaa3e7c91; node 418169685f940c3a11d1df4c4a98825558ff51c7aa59ff97872428511b1db9e2;
  ledger node 2504e41ac434e21c2de57d5f6298d4d7d9cf53d505395ff585903851917ca79c. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`,
  `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`,
  `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`,
  `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`,
  `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`,
  `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`,
  `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`,
  `mu_t_muN`, `gt`, `m_h`, `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`,
  `mh_mp`, `M_h`, `mu_h`, `mu_h_muB`, `mu_h_muN`, `gh`, and `mu0h` hashes and
  nodes unchanged.

- **CODATA 2018 shielded helion magnetic moment is a one-sigma Interval.**
  `physis-constants` versions `mu0h` as the CODATA 2018 signed hull
  `−1.074553090(13)×10^{-26}` J T⁻¹ from JPCRD 50, 033105 table XXXI
  (Helion, h; gas, sphere, 25 °C). This is not free helion magnetic
  moment `mu_h`, not shielded proton `mu0p`, not electron to
  shielded-helion `mu_e_mu0h`, not vacuum permeability `mu0`, not
  helion g-factor `gh`, not an SI defining Ratio, not the Thomson
  cross section, and not P3N. Bohr-magneton ratio rows are later table
  rows and are not stored. Gyromagnetic ratios cite ħ and are not
  stored. Electron mass is still not stored (`10^{42}` overflows
  `i128`). Decade `10^{35}` (`10^{34}` is the 10× trap). This is not
  the CODATA 2022 last-digit `11035`. `physis_model`
  `shielded_helion_magnetic_moment()` Qty locksteps to the recommended
  centre inside the hull. Adding `mu0h` to LEDGER changes the ledger
  bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`,
  `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`,
  `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`,
  `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`,
  `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`,
  `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`,
  `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`,
  `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`,
  `mu_t_muN`, `gt`, `m_h`, `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`,
  `mh_mp`, `M_h`, `mu_h`, `mu_h_muB`, `mu_h_muN`, and `gh` hashes are
  unchanged. Theories still evaluate with `f64` Qty. That is not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `mu0h` hash f207205c27290f0b85017413fd3cd47593d77a3ad71a7d4337c96d0bff8ff559; node 1292784df9b775d6115c93d72a320c32327fccef404f1e628513353025312cc3;
  ledger node cbf86166c14e0c66783a33eb27357b5bef7a0944b68c461201275b1b83edf107. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`,
  `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`,
  `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`,
  `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`,
  `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`,
  `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`,
  `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`,
  `mu_t_muN`, `gt`, `m_h`, `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`,
  `mh_mp`, `M_h`, `mu_h`, `mu_h_muB`, `mu_h_muN`, and `gh` hashes and
  nodes unchanged.

- **CODATA 2018 helion g-factor is a one-sigma Interval.**
  `physis-constants` versions `gh` as the CODATA 2018 signed hull
  `−4.255250615(50)` from JPCRD 50, 033105 table XXXI (Helion, h). This
  is not helion nuclear-magneton ratio `mu_h_muN`, not triton `gt`, not
  deuteron `gd`, not electron `ge`, not muon `gmu`, not proton `gp`, not
  neutron `gn`, not a certificate that the stored centres reconstruct
  `2 μ_h/μ_N`, not an SI defining Ratio, not the Thomson cross section,
  and not P3N. JPCRD prints different digits from `mu_h_muN` because
  `I = 1/2` (`g = 2μ/μN`); each row has its own Claim identity.
  Shielded-helion rows are later table rows and are not stored. Electron
  mass is still not stored (`10^{42}` overflows `i128`). Decade `10^{9}`
  (`10^{8}` is the 10× trap). This is not the CODATA 2022 last-digit
  `6995`. `physis_model` `helion_g_factor()` Qty locksteps to the
  recommended centre inside the hull. Adding `gh` to LEDGER changes the
  ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`,
  `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`,
  `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`,
  `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`,
  `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`,
  `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`,
  `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`,
  `mu_t_muN`, `gt`, `m_h`, `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`,
  `mh_mp`, `M_h`, `mu_h`, `mu_h_muB`, and `mu_h_muN` hashes are
  unchanged. Theories still evaluate with `f64` Qty. That is not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `gh` hash 89764c03ec4774afa24862fca730205f559747ba9256e7378c64629720d31c4f; node 89d787cea08d18f680faa8fa0e4e2bc9e25ed8d4c4f649ba7bfe47ed4096e90f;
  ledger node 8e7397c6d237ba060d412fb1922dcb1f2a0439a99a6c991f60f5d7d060f675c8. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`,
  `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`,
  `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`,
  `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`,
  `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`,
  `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`,
  `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`,
  `mu_t_muN`, `gt`, `m_h`, `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`,
  `mh_mp`, `M_h`, `mu_h`, `mu_h_muB`, and `mu_h_muN` hashes and nodes
  unchanged.

- **CODATA 2018 helion nuclear-magneton ratio is a one-sigma Interval.**
  `physis-constants` versions `mu_h_muN` as the CODATA 2018 signed hull
  `−2.127625307(25)` from JPCRD 50, 033105 table XXXI (Helion, h). This
  is not helion magnetic moment `mu_h`, not Bohr-magneton ratio
  `mu_h_muB`, not triton `mu_t_muN`, not deuteron `mu_d_muN`, not proton
  `mu_p_muN`, not neutron `mu_n_muN`, not electron `mu_e_muN`, not muon
  `mu_mu_muN`, not a certificate that this equals a reconstructed
  `μ_h/μ_N`, not a certificate that this equals the g-factor `gh`, not
  an SI defining Ratio, not the Thomson cross section, and not P3N.
  JPCRD prints different digits from `mu_h_muN` because `I = 1/2`
  (`g = 2μ/μN`); each row has its own Claim identity. G-factor rows are
  later table rows and are not stored. Electron mass is still not stored
  (`10^{42}` overflows `i128`). Decade `10^{9}` (`10^{8}` is the 10×
  trap). This is not the CODATA 2022 last-digit `3498`. `physis_model`
  `helion_magnetic_moment_to_nuclear_magneton()` Qty locksteps to the
  recommended centre inside the hull. Adding `mu_h_muN` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`,
  `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`,
  `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`,
  `mu_t_muN`, `gt`, `m_h`, `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`,
  `mh_mp`, `M_h`, `mu_h`, and `mu_h_muB` hashes are unchanged. Theories
  still evaluate with `f64` Qty. That is not a kernel proof, not
  Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4.
  Verified: `mu_h_muN` hash b8daa265220c3de6776c9b0703e094ea1e56ce2e7d8f5e4c330f3bb38b513e3e; node d5ec8eaa8842a4d6a63449e9c864686d2fe786bb47186ec5be6d6fe86c398e2e;
  ledger node c1482f81f0d6c9234c1213baf7d93cfabd2039205c1ffdd6c095f99f4f2ef38f. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`,
  `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`,
  `mu_t_muN`, `gt`, `m_h`, `m_h_u`, `m_h_c2`, `m_h_c2_MeV`,
  `mh_me`, `mh_mp`, `M_h`, `mu_h`, and `mu_h_muB` hashes and nodes
  unchanged.

- **CODATA 2018 helion Bohr-magneton ratio is a one-sigma Interval.**
  `physis-constants` versions `mu_h_muB` as the CODATA 2018 signed hull
  `−1.158740958(14)×10^{-3}` from JPCRD 50, 033105 table XXXI
  (Helion, h). This is not helion magnetic moment `mu_h`, not triton
  `mu_t_muB`, not deuteron `mu_d_muB`, not proton `mu_p_muB`, not
  neutron `mu_n_muB`, not electron `mu_e_muB`, not muon `mu_mu_muB`,
  not vacuum permeability `mu0`, not molar mass `M_h`, not a
  certificate that this equals a reconstructed `μ_h/μ_B`, not an SI
  defining Ratio, not the Thomson cross section, and not P3N.
  Nuclear-magneton ratio rows are later table rows and are not stored.
  Electron mass is still not stored (`10^{42}` overflows `i128`).
  Decade `10^{12}` (`10^{11}` is the 10× trap). This is not the CODATA
  2022 last-digit `98083`. `physis_model`
  `helion_magnetic_moment_to_bohr_magneton()` Qty locksteps to the
  recommended centre inside the hull. Adding `mu_h_muB` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`,
  `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`,
  `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`,
  `mu_t_muN`, `gt`, `m_h`, `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`,
  `mh_mp`, `M_h`, and `mu_h` hashes are unchanged. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4.
  Verified: `mu_h_muB` hash bffb76fa55e4a2baad9d666cebb1aa1df1bc5c500d58e081ee25fde1ba67fac5; node 3bb0b020d185e2a5a5fa6fd0448fca4a0c41a3cafa42e2e347e4f86d0dfacb32;
  ledger node c054ff7ae42876cb336af518fb42c58f5f449e48ed61d3c881a1835eaf29d36c. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`,
  `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`,
  `mu_t_muN`, `gt`, `m_h`, `m_h_u`, `m_h_c2`, `m_h_c2_MeV`,
  `mh_me`, `mh_mp`, `M_h`, and `mu_h` hashes and nodes unchanged.

- **CODATA 2018 helion magnetic moment is a one-sigma Interval.**
  `physis-constants` versions `mu_h` as the CODATA 2018 signed hull
  `−1.074617532(13)×10^{-26}` J T⁻¹ from JPCRD 50, 033105 table XXXI
  (Helion, h). This is not triton `mu_t`, not deuteron `mu_d`, not
  neutron `mu_n`, not proton `mu_p`, not electron `mu_e`, not muon
  `mu_mu`, not vacuum permeability `mu0`, not molar mass `M_h`, not a
  certificate that this equals `g_h μ_N / 2`, not an SI defining Ratio,
  not the Thomson cross section, and not P3N. Bohr-magneton ratio rows
  are later table rows and are not stored. Electron mass is still not
  stored (`10^{42}` overflows `i128`). Decade `10^{35}` (`10^{34}` is
  the 10× trap). This is not the CODATA 2022 last-digit `55198`.
  `physis_model` `helion_magnetic_moment()` Qty locksteps to the
  recommended centre inside the hull. Adding `mu_h` to LEDGER changes
  the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`,
  `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`,
  `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`,
  `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`,
  `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`,
  `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`,
  `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`,
  `mu_t_muN`, `gt`, `m_h`, `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`,
  `mh_mp`, and `M_h` hashes are unchanged. Theories still evaluate with
  `f64` Qty. That is not a kernel proof, not Canonical, not P4. Encode
  pins unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `mu_h` hash c2cc411107459d5d4bbf914e1dc3eab7b9abb63d9a5189b497c5390169fe577f; node d915995be231b6c53b6d71929708c0b6316d07b165cc70da16e043b6acc04ff5;
  ledger node 1d663920324a31296a8f310ef9f31cd9c35c3073256e4d6f0f9067627b790f0a. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`,
  `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`,
  `mu_t_muN`, `gt`, `m_h`, `m_h_u`, `m_h_c2`, `m_h_c2_MeV`,
  `mh_me`, `mh_mp`, and `M_h` hashes and nodes unchanged.

- **CODATA 2018 helion molar mass is a one-sigma Interval.**
  `physis-constants` versions `M_h` as the CODATA 2018 hull
  `3.01493224613(91)×10^{-3}` kg mol⁻¹ from JPCRD 50, 033105 table XXXI
  (Helion, h). This is not triton `M_t`, not deuteron `M_d`, not neutron
  `M_n`, not proton `M_p`, not electron `M_e`, not muon `M_mu`, not the
  kg hull `m_h`, not the u-row `m_h_u`, not a certificate that this
  equals `N_A × m_h`, not an SI defining Ratio, not the Thomson cross
  section, and not P3N. Moment rows are later table rows and are not
  stored. Electron mass is still not stored (`10^{42}` overflows
  `i128`). Decade `10^{14}` (`10^{13}` is the 10× trap). This is not the
  CODATA 2022 last-digit `25010`. `physis_model` `helion_molar_mass()`
  Qty locksteps to the recommended centre inside the hull. Adding `M_h`
  to LEDGER changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`,
  `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`,
  `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`,
  `mu_t_muN`, `gt`, `m_h`, `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`,
  and `mh_mp` hashes are unchanged. Theories still evaluate with `f64`
  Qty. That is not a kernel proof, not Canonical, not P4. Encode pins
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `M_h` hash a03524238032cd0ba44f2ed25ec8d2a62a5259d7b5353a00df939716d6efb7bf; node 846bd5680670d73586466f859106f679a20e316bc8f379fd20a0f0198de56c89;
  ledger node 956375bd2e397d5a3d1f0242accd9c2d167cb676d1c16f90e6699f611f17a897. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`,
  `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`,
  `mu_t_muN`, `gt`, `m_h`, `m_h_u`, `m_h_c2`, `m_h_c2_MeV`,
  `mh_me`, and `mh_mp` hashes and nodes unchanged.

- **CODATA 2018 helion-proton mass ratio is a one-sigma Interval.**
  `physis-constants` versions `mh_mp` as the CODATA 2018 hull
  `2.99315267167(13)` from JPCRD 50, 033105 table XXXI (Helion, h).
  This is not triton `mt_mp`, not deuteron `md_mp`, not neutron `mn_mp`,
  not proton-neutron `mp_mn`, not helion-electron `mh_me`, not a
  certificate that the stored centres reconstruct `m_h/m_p`, not an SI
  defining Ratio, not the Thomson cross section, and not P3N. The molar
  mass is a later table row and is not stored. Moment rows are later
  table rows and are not stored. Electron mass is still not stored
  (`10^{42}` overflows `i128`). Decade `10^{11}` (`10^{10}` is the 10×
  trap). This is not the CODATA 2022 last-digit `671552`. `physis_model`
  `helion_proton_mass_ratio()` Qty locksteps to the recommended centre
  inside the hull. Adding `mh_mp` to LEDGER changes the ledger bundle
  pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`,
  `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`,
  `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`,
  `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`,
  `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`,
  `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`,
  `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`,
  `mu_t_muN`, `gt`, `m_h`, `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, and `mh_me`
  hashes are unchanged. Theories still evaluate with `f64` Qty. That is
  not a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `mh_mp` hash 46ae4686293f59d147df432b57e622e0883c8611813e0684326e4f2fd00d5c6d; node cd44c6dd61faf6bab9fc807f39873493ef1f0608501f15958cf18df3900b8e0e;
  ledger node 23f20840109dd23b7ddd6b8eda4afc3a012c5dbe29ce70a64b89d7c6b28b391b. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`,
  `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`,
  `mu_t_muN`, `gt`, `m_h`, `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, and
  `mh_me` hashes and nodes unchanged.

- **CODATA 2018 helion-electron mass ratio is a one-sigma Interval.**
  `physis-constants` versions `mh_me` as the CODATA 2018 hull
  `5495.88528007(24)` from JPCRD 50, 033105 table XXXI (Helion, h).
  This is not electron-helion `me_mh`, not a certificate that the stored
  centres invert, not triton `mt_me`, not deuteron `md_me`, not neutron
  `mn_me`, not proton `mp_me`, not muon `mmu_me`, not MeV hull
  `m_h_c2_MeV`, not an SI defining Ratio, not the Thomson cross section,
  and not P3N. The helion-proton mass ratio is a later table row and is
  not stored. Molar-mass and moment rows are later table rows and are
  not stored. Electron mass is still not stored (`10^{42}` overflows
  `i128`). Decade `10^{8}` (`10^{7}` is the 10× trap). This is not the
  CODATA 2022 last-digit `27984`. `physis_model`
  `helion_electron_mass_ratio()` Qty locksteps to the recommended centre
  inside the hull. Adding `mh_me` to LEDGER changes the ledger bundle
  pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`,
  `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`,
  `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`,
  `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`,
  `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`,
  `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`,
  `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`,
  `mu_t_muN`, `gt`, `m_h`, `m_h_u`, `m_h_c2`, and `m_h_c2_MeV` hashes
  are unchanged. Theories still evaluate with `f64` Qty. That is not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `mh_me` hash 2456ff97efcbfce853572c4f0bedf455455afedf9cec1628db183f23c2fe5368; node ae4ffe1f19def746094bea82f05535eb48236670eb61d5eeaed0c821c42ce1eb;
  ledger node 69e34010c2bd3c334738371f4dec055e9108ce2376d3c7bf11e99b39638ae9e8. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`,
  `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`,
  `mu_t_muN`, `gt`, `m_h`, `m_h_u`, `m_h_c2`, and `m_h_c2_MeV`
  hashes and nodes unchanged.

- **CODATA 2018 helion mass energy equivalent in MeV is a one-sigma Interval.**
  `physis-constants` versions `m_h_c2_MeV` as the CODATA 2018 hull
  `2808.39160743(85)` MeV from JPCRD 50, 033105 table XXXI
  (Helion, h). This is not joule hull `m_h_c2`, not triton
  `m_t_c2_MeV`, not deuteron `m_d_c2_MeV`, not neutron `m_n_c2_MeV`,
  not proton `m_p_c2_MeV`, not muon `m_mu_c2_MeV`, not Hartree `Eh`,
  not the exact electronvolt Ratio, not a certificate of a reconstruction
  from sibling masses, not an SI defining Ratio, not the Thomson cross
  section, and not P3N. The helion-electron mass ratio is a later table
  row and is not stored. Mass-ratio, molar-mass, and moment rows are
  later table rows and are not stored. Electron mass is still not stored
  (`10^{42}` overflows `i128`). Decade `10^{8}` (`10^{7}` is the 10×
  trap). This is not the CODATA 2022 last-digit `61112`. `physis_model`
  `helion_mass_energy_equivalent_in_mev()` Qty locksteps to the
  recommended centre inside the hull. Adding `m_h_c2_MeV` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`,
  `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`,
  `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`,
  `mu_t_muN`, `gt`, `m_h`, `m_h_u`, and `m_h_c2` hashes are unchanged.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4.
  Verified: `m_h_c2_MeV` hash 578eec087078da75a87dc84591b521cdf65581fda497b249d2e3e8e2bb6a4e8a; node 7b477dc07fbc9093483a14d474da85361c3f1b41df0815caf12c3c3c4c6750f8;
  ledger node 13b68e153a2f60609d5f884f0a180c93977791a51bb8fadd50cf66675c95ea01. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`,
  `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`,
  `mu_t_muN`, `gt`, `m_h`, `m_h_u`, and `m_h_c2` hashes and nodes unchanged.

- **CODATA 2018 helion mass energy equivalent is a one-sigma Interval.**
  `physis-constants` versions `m_h_c2` as the CODATA 2018 hull
  `4.4995394125(14)×10^{-10}` J from JPCRD 50, 033105 table XXXI
  (Helion, h). This is not kg hull `m_h`, not u-row `m_h_u`, not triton
  `m_t_c2`, not deuteron `m_d_c2`, not neutron `m_n_c2`, not proton
  `m_p_c2`, not muon `m_mu_c2`, not Rydberg `hcRinf`, not Hartree `Eh`,
  not the exact electronvolt Ratio, not a certificate of a reconstruction
  from sibling masses, not an SI defining Ratio, not the Thomson cross
  section, and not P3N. The MeV conversion is a later table row and is
  not stored. Mass-ratio, molar-mass, and moment rows are later table
  rows and are not stored. Electron mass is still not stored (`10^{42}`
  overflows `i128`). Decade `10^{20}` (`10^{19}` is the 10× trap). This
  is not the CODATA 2022 last-digit `4185`. `physis_model`
  `helion_mass_energy_equivalent()` Qty locksteps to the recommended
  centre inside the hull. Adding `m_h_c2` to LEDGER changes the ledger
  bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`,
  `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`,
  `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`,
  `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`,
  `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`,
  `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`,
  `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`,
  `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`,
  `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`,
  `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`,
  `mu_t_muN`, `gt`, `m_h`, and `m_h_u` hashes are unchanged. Theories
  still evaluate with `f64` Qty. That is not a kernel proof, not
  Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4.
  Verified: `m_h_c2` hash 33cd4c3d375911c13d541bf56f9664717286c73fa2ef1ebe83e2728a196a8a46; node ca342c9a437d6127ac6cc8fa5219e366912504aadbd7d9c13ebea57feda513a2;
  ledger node 2bf94f7da39e0ab80042cf7d908f33667805098c0b957c767e689594a25437c7. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`,
  `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`,
  `mu_t_muN`, `gt`, `m_h`, and `m_h_u` hashes and nodes unchanged.

- **CODATA 2018 helion mass in u is a one-sigma Interval.**
  `physis-constants` versions `m_h_u` as the CODATA 2018 hull
  `3.014932247175(97)` u from JPCRD 50, 033105 table XXXI (Helion, h).
  This is not kg hull `m_h`, not triton `m_t_u`, not deuteron `m_d_u`,
  not neutron `m_n_u`, not proton `m_p_u`, not muon `m_mu_u`, not
  electron-helion `me_mh`, not electron molar mass `M_e`, not relative
  atomic mass under a different name, not a certificate of a
  reconstruction from sibling masses, not an SI defining Ratio, not the
  Thomson cross section, and not P3N. Energy-equivalent, MeV, mass-ratio,
  molar-mass, and moment rows are later table rows and are not stored.
  Electron mass is still not stored (`10^{42}` overflows `i128`). Decade
  `10^{12}` (`10^{11}` is the 10× trap). This is not the CODATA 2022
  last-digit `932`. `physis_model` `helion_mass_in_u()` Qty locksteps to
  the recommended centre inside the hull. Adding `m_h_u` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, and `m_h`
  hashes are unchanged. Theories still evaluate with `f64` Qty. That is
  not a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `m_h_u` hash 652b05943359b759c6dd9125ca3fb791ebfe9272583a413f3d752ca742f5da51; node 78d82dde163ee7a883b7e0748ee24f0af600a8d8feb762eaec4dccbce750e07a;
  ledger node 3dda2c4cf9c3b26ea25dd96b3ff60e61257aaca19ed19d61a318acc307cfb26e. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`,
  `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`,
  `mu_t_muN`, `gt`, and `m_h` hashes and nodes unchanged.

- **CODATA 2018 helion mass is a one-sigma Interval.**
  `physis-constants` versions `m_h` as the CODATA 2018 hull
  `5.0064127796(15)×10^{-27}` kg from JPCRD 50, 033105 table XXXI
  (Helion, h). This is not triton `m_t`, not deuteron `m_d`, not
  neutron `m_n`, not proton `m_p`, not muon `m_mu`, not electron-helion
  `me_mh`, not a certificate of a reconstruction from sibling masses,
  not an SI defining Ratio, not the Thomson cross section, and not P3N.
  The u-row is a later table row and is not stored. Energy-equivalent,
  mass-ratio, molar-mass, and moment rows are later table rows and are
  not stored. Electron mass is still not stored (`10^{42}` overflows
  `i128`). Decade `10^{37}` (`10^{36}` is the 10× trap). This is not
  the CODATA 2022 last-digit `7862`. `physis_model` `helion_mass()` Qty
  locksteps to the recommended centre inside the hull. Adding `m_h` to
  LEDGER changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`,
  `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`,
  `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, and `gt` hashes are
  unchanged. Theories still evaluate with `f64` Qty. That is not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `m_h` hash 1a53756d23bdbbc188edb9cc55d1f3a9e5cc952d386bf885530507a4cded2492; node dd7fb3d8d0f506e48199cd8b575f47d9668d7968a53a4d82a9d40c0f62e4cb9a;
  ledger node 64e966c68754735c57c3f0a5888162bc2abdbca3df5397b84023ecfaf46b194e. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`,
  `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`,
  `mu_t_muN`, and `gt` hashes and nodes unchanged.

- **CODATA 2018 triton g-factor is a one-sigma Interval.**
  `physis-constants` versions `gt` as the CODATA 2018 hull
  `5.957924931(12)` from JPCRD 50, 033105 table XXXI (Triton, t). This
  is not triton nuclear-magneton ratio `mu_t_muN`, not deuteron `gd`,
  not electron `ge`, not muon `gmu`, not proton `gp`, not neutron `gn`,
  not a certificate that the stored centres reconstruct `2 μ_t/μ_N`,
  not an SI defining Ratio, not the Thomson cross section, and not P3N.
  JPCRD prints different digits from `mu_t_muN` because `I = 1/2`
  (`g = 2μ/μN`); each row has its own Claim identity. Helion rows are
  later table rows and are not stored. Electron mass is still not
  stored (`10^{42}` overflows `i128`). Decade `10^{9}` (`10^{8}` is the
  10× trap). This is not the CODATA 2022 last-digit `930`.
  `physis_model` `triton_g_factor()` Qty locksteps to the recommended
  centre inside the hull. Adding `gt` to LEDGER changes the ledger
  bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`,
  `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`,
  `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`,
  `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`,
  `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`,
  `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`,
  `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`,
  `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`,
  `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`,
  `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, and
  `mu_t_muN` hashes are unchanged. Theories still evaluate with `f64`
  Qty. That is not a kernel proof, not Canonical, not P4. Encode pins
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `gt` hash 2f77c0ee28920e4866250d271d9288df2b68a2f82655588827e4678e5c0825ac; node 78102b0ca8a56bb2e5a99294daa66bce2d2899a0527102b56ffc0add1b2348f8;
  ledger node e36fff7e272cc2f04b97dcb839b7c312c8a6b20d8264ec0db593090057e29efa. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`,
  `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, and
  `mu_t_muN` hashes and nodes unchanged.

- **CODATA 2018 triton magnetic moment to nuclear magneton ratio is a one-sigma Interval.**
  `physis-constants` versions `mu_t_muN` as the CODATA 2018 hull
  `2.9789624656(59)` from JPCRD 50, 033105 table XXXI
  (Triton, t). This is not triton magnetic moment `mu_t`, not triton
  Bohr-magneton ratio `mu_t_muB`, not deuteron `mu_d_muN`, not proton
  `mu_p_muN`, not neutron `mu_n_muN`, not electron `mu_e_muN`, not muon
  `mu_mu_muN`, not a certificate that the stored centres reconstruct
  `μ_t/μ_N`, not a certificate that this equals the g-factor `gt`, not
  an SI defining Ratio, not the Thomson cross section, and not P3N. The
  g-factor is a later table row and is not stored. Electron mass is
  still not stored (`10^{42}` overflows `i128`). Decade `10^{10}`
  (`10^{9}` is the 10× trap). This is not the CODATA 2022 last-digit
  `4650`. `physis_model` `triton_magnetic_moment_to_nuclear_magneton()`
  Qty locksteps to the recommended centre inside the hull. Adding
  `mu_t_muN` to LEDGER changes the ledger bundle pin. The `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`,
  `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`,
  `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`,
  `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`,
  `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`,
  `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`,
  `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`, `mt_mp`,
  `M_t`, `mu_t`, and `mu_t_muB` hashes are unchanged. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4.
  Verified: `mu_t_muN` hash 88f8acc9ba93fb7694b33fa92505cf9ade50157256c713a95b5802113dfe5c65; node 499abd51eb8397c380382aaa5cdf4e713c71f2feb1ef36b0c8d3faa31a268266;
  ledger node 20f68d6788366c99414086af9d33aeffb3ab9960b0fdfd6d84c6d34e2b9ac891. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`,
  `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, and `mu_t_muB`
  hashes and nodes unchanged.

- **CODATA 2018 triton magnetic moment to Bohr magneton ratio is a one-sigma Interval.**
  `physis-constants` versions `mu_t_muB` as the CODATA 2018 hull
  `1.6223936651(32)×10^{-3}` from JPCRD 50, 033105 table XXXI
  (Triton, t). This is not triton magnetic moment `mu_t`, not deuteron
  `mu_d_muB`, not proton `mu_p_muB`, not neutron `mu_n_muB`, not electron
  `mu_e_muB`, not muon `mu_mu_muB`, not a certificate that the stored
  centres reconstruct `μ_t/μ_B`, not an SI defining Ratio, not the
  Thomson cross section, and not P3N. The nuclear-magneton ratio is a
  later table row and is not stored. The g-factor is a later table row
  and is not stored. Electron mass is still not stored (`10^{42}`
  overflows `i128`). Decade `10^{13}` (`10^{12}` is the 10× trap). This
  is not the CODATA 2022 last-digit `6648`. `physis_model`
  `triton_magnetic_moment_to_bohr_magneton()` Qty locksteps to the
  recommended centre inside the hull. Adding `mu_t_muB` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`,
  `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`,
  `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, and `mu_t` hashes are unchanged.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4.
  Verified: `mu_t_muB` hash 17fb6a9ef0fb6278581531a4a291899413f104dd27292cf4710f1903d1bdaefb; node 0165f8433e9823ce96f4b3758754f555006d0a3d074d2311d5b2424cb0101ec5;
  ledger node 5b8f42ce4f8bf3aa3f767c282afc34107bde0e3ecca4dc9253c84b571e49c682. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`,
  `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, and `mu_t` hashes and nodes
  unchanged.

- **CODATA 2018 triton magnetic moment is a one-sigma Interval.**
  `physis-constants` versions `mu_t` as the CODATA 2018 hull
  `1.5046095202(30)×10^{-26}` J T⁻¹ from JPCRD 50, 033105 table
  XXXI (Triton, t). This is not deuteron `mu_d`, not proton `mu_p`, not
  neutron `mu_n`, not electron `mu_e`, not muon `mu_mu`, not vacuum
  permeability `mu0`, not molar mass `M_t`, not a certificate that this
  equals `g_t μ_N / 2`, not an SI defining Ratio, not the Thomson cross
  section, and not P3N. Bohr-magneton, nuclear-magneton, and g-factor
  rows are later table rows and are not stored. Electron mass is still
  not stored (`10^{42}` overflows `i128`). Decade `10^{36}`
  (`10^{35}` is the 10× trap). This is not the CODATA 2022 last-digit
  `5178`. `physis_model` `triton_magnetic_moment()` Qty locksteps to the
  recommended centre inside the hull. Adding `mu_t` to LEDGER changes
  the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`,
  `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`,
  `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`,
  `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`,
  `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`,
  `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`,
  `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, and `M_t` hashes are unchanged. Theories
  still evaluate with `f64` Qty. That is not a kernel proof, not
  Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4.
  Verified: `mu_t` hash f07f9b5d76ca552fd5126f5d9d3b2c21a68b0aec0da98c40394f95290766aa62; node cf24b32cc388f7e447b4fd094f607dcb10e8982bd2251a0538b5a65d9fdf039c;
  ledger node 9a6a0b6a17ba7ba2b82d0e080989bbfd5f5e8a9cb6b132f8b00dfc7f0edc78e1. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`,
  `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, `mt_mp`, and `M_t` hashes and nodes unchanged.

- **CODATA 2018 triton molar mass is a one-sigma Interval.**
  `physis-constants` versions `M_t` as the CODATA 2018 hull
  `3.01550071517(92)×10^{-3}` kg mol⁻¹ from JPCRD 50, 033105 table
  XXXI (Triton, t). This is not neutron `M_n`, not proton `M_p`, not
  electron `M_e`, not muon `M_mu`, not the kg hull `m_t`, not the u-row
  `m_t_u`, not a certificate that this equals `N_A × m_t`, not an SI
  defining Ratio, not the Thomson cross section, and not P3N. Magnetic-
  moment rows are later table rows and are not stored. Electron mass is
  still not stored (`10^{42}` overflows `i128`). Decade `10^{14}`
  (`10^{13}` is the 10× trap). This is not the CODATA 2022 last-digit
  `71913`. `physis_model` `triton_molar_mass()` Qty locksteps to the
  recommended centre inside the hull. Adding `M_t` to LEDGER changes
  the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`,
  `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`,
  `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`,
  `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`,
  `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`,
  `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`,
  `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, and `mt_mp` hashes are unchanged. Theories
  still evaluate with `f64` Qty. That is not a kernel proof, not
  Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4.
  Verified: `M_t` hash c6c24c87f2920c72a840157ddeaa978adb4013e40c08e62168d27c940c0ff25b; node 98820fbeb58e46a4d5e31bc12f19bec979553c95500ffd5303811a5c7dd1edb0;
  ledger node 6ed4ff85133067c8c0d96789f704cef124edeaf4072b43d3c0ba7808a4f26b64. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`,
  `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, `mt_me`, and `mt_mp` hashes and nodes unchanged.

- **CODATA 2018 triton-proton mass ratio is a one-sigma Interval.**
  `physis-constants` versions `mt_mp` as the CODATA 2018 hull
  `2.99371703414(15)` from JPCRD 50, 033105 table XXXI (Triton, t).
  This is not deuteron `md_mp`, not neutron `mn_mp`, not proton-neutron
  `mp_mn`, not a certificate that the stored centres reconstruct
  `m_t/m_p`, not `mt_me`, not triton mass, not proton mass, not an SI
  defining Ratio, not the Thomson cross section, and not P3N. The
  molar mass is a later table row and is not stored. Electron mass is
  still not stored (`10^{42}` overflows `i128`). Decade `10^{11}`
  (`10^{10}` is the 10× trap). This is not the CODATA 2022 last-digit
  `03403`. `physis_model` `triton_proton_mass_ratio()` Qty locksteps to
  the recommended centre inside the hull. Adding `mt_mp` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`,
  `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`,
  `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, and `mt_me` hashes are unchanged. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4.
  Verified: `mt_mp` hash de2806f05b0502127a1c6e32a40eb5fde57e2dbbe06c02fbbd78e92899519595; node 41ffc89be1c1822959ce00cee4dc8def69aa243b60dc61a9191747d600a7548f;
  ledger node 8b16d4db57fcdabc3ed26200f74a9f1314cf882f5f68c69067e8e79198e815a3. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`,
  `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  `m_t_c2_MeV`, and `mt_me` hashes and nodes unchanged.

- **CODATA 2018 triton-electron mass ratio is a one-sigma Interval.**
  `physis-constants` versions `mt_me` as the CODATA 2018 hull
  `5496.92153573(27)` from JPCRD 50, 033105 table XXXI (Triton, t).
  This is not the electron-triton ratio `me_mt`, not a certificate that
  the stored centres invert, not deuteron `md_me`, not neutron `mn_me`,
  not proton `mp_me`, not muon `mmu_me`, not triton mass, not MeV energy
  equivalent, not an SI defining Ratio, not the Thomson cross section,
  and not P3N. The triton-proton mass ratio is a later table row and is
  not stored. Electron mass is still not stored (`10^{42}` overflows
  `i128`). Decade `10^{8}` (`10^{7}` is the 10× trap). This is not the
  CODATA 2022 last-digit `53551`. `physis_model`
  `triton_electron_mass_ratio()` Qty locksteps to the recommended centre
  inside the hull. Adding `mt_me` to LEDGER changes the ledger bundle
  pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`,
  `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`,
  `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`,
  `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`,
  `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`,
  `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`,
  `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, and
  `m_t_c2_MeV` hashes are unchanged. Theories still evaluate with `f64`
  Qty. That is not a kernel proof, not Canonical, not P4. Encode pins
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `mt_me` hash 93547acb180d081377858fe38ebb143d9f8c1309a209cb635157b8d2627aed58; node e204ed8ecdc5afb1b11914f6ba7980f0931aaabbf7ff74d0b8d2fca75025bd3a;
  ledger node 8c732d26dbac256e5c75053f4ff35438e4c969fae5351ada36b62359f8dd87e0. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`,
  `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`,
  and `m_t_c2_MeV` hashes and nodes unchanged.

- **CODATA 2018 triton mass energy equivalent in MeV is a one-sigma Interval.**
  `physis-constants` versions `m_t_c2_MeV` as the CODATA 2018 hull
  `2808.92113298(85)` MeV from JPCRD 50, 033105 table XXXI (Triton, t).
  This is not the joule hull `m_t_c2`, not deuteron `m_d_c2_MeV`, not
  neutron `m_n_c2_MeV`, not proton `m_p_c2_MeV`, not muon `m_mu_c2_MeV`,
  not Hartree, not the exact electronvolt Ratio, not a certificate of a
  reconstruction from sibling masses, not an SI defining Ratio, not the
  Thomson cross section, and not P3N. The triton-electron mass ratio is
  a later table row and is not stored. Electron mass is still not stored
  (`10^{42}` overflows `i128`). Decade `10^{8}` (`10^{7}` is the 10×
  trap). This is not the CODATA 2022 last-digit `13668`. `physis_model`
  `triton_mass_energy_equivalent_in_mev()` Qty locksteps to the
  recommended centre inside the hull. Adding `m_t_c2_MeV` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`,
  `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`,
  `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, and `m_t_c2`
  hashes are unchanged. Theories still evaluate with `f64` Qty. That is
  not a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `m_t_c2_MeV` hash 818597e9129aeea9aa601ed37421b0cdba5f257cddc2437fd22d635d8a205136; node 1332df022f8b2d564b2ec9f8760debb91a5260fe0c1691c4919ccffc9c410c99;
  ledger node 3200115db051a7cd35011f3a2cf6a4f829cfc4c525b0f2c88dd8a11c2d0875cf. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`,
  `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, and
  `m_t_c2` hashes and nodes unchanged.

- **CODATA 2018 triton mass energy equivalent is a one-sigma Interval.**
  `physis-constants` versions `m_t_c2` as the CODATA 2018 hull
  `4.5003878060(14)×10^{-10}` J from JPCRD 50, 033105 table XXXI
  (Triton, t). This is not the kg hull `m_t`, not the u-row `m_t_u`,
  not deuteron `m_d_c2`, not neutron `m_n_c2`, not proton `m_p_c2`,
  not muon `m_mu_c2`, not Rydberg energy equivalent, not Hartree, not
  the exact electronvolt Ratio, not a certificate of a reconstruction
  from sibling masses, not an SI defining Ratio, not the Thomson cross
  section, and not P3N. The MeV conversion is a later table row and is
  not stored. Electron mass is still not stored (`10^{42}` overflows
  `i128`). Decade `10^{20}` (`10^{19}` is the 10× trap). This is not
  the CODATA 2022 last-digit `8119`. `physis_model`
  `triton_mass_energy_equivalent()` Qty locksteps to the recommended
  centre inside the hull. Adding `m_t_c2` to LEDGER changes the ledger
  bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`,
  `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`,
  `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`,
  `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`,
  `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`,
  `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`,
  `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`,
  `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, and `m_t_u` hashes are
  unchanged. Theories still evaluate with `f64` Qty. That is not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `m_t_c2` hash 2e658432e2c5b63fc780b7af02de9f3d2d94c304d16e120708f73bb18ccd2b3c; node 1177d21850e21b80006faa5b86d7225fe8d070130ef30777ad3299f226cdf521;
  ledger node e2488640dc1e3319e59b36ddd54ab19c5b8c27db73a97feb03956e686bf8e92f. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`,
  `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, and `m_t_u` hashes
  and nodes unchanged.

- **CODATA 2018 triton mass in u is a one-sigma Interval.**
  `physis-constants` versions `m_t_u` as the CODATA 2018 hull
  `3.01550071621(12)` u from JPCRD 50, 033105 table XXXI (Triton, t).
  This is not the kg hull `m_t`, not deuteron mass in u `m_d_u`, not
  neutron `m_n_u`, not proton `m_p_u`, not muon `m_mu_u`, not electron
  molar mass, not relative atomic mass under a different name, not a
  certificate of a reconstruction from sibling masses or mass ratios,
  not an SI defining Ratio, not the Thomson cross section, and not P3N.
  Energy equivalent, MeV conversion, mass ratios, molar mass, and
  magnetic-moment rows are later table rows and are not stored. Electron
  mass is still not stored (`10^{42}` overflows `i128`). Decade `10^{11}`
  (`10^{10}` is the 10× trap). This is not the CODATA 2022 last-digit
  `597`. `physis_model` `triton_mass_in_u()` Qty locksteps to the
  recommended centre inside the hull. Adding `m_t_u` to LEDGER changes
  the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`,
  `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`,
  `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`,
  `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`,
  `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`,
  `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`,
  `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, and `m_t` hashes are unchanged.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4.
  Verified: `m_t_u` hash 60799f1d95e3ac37f9505743bac8b6a7437b1707963516078da90c73d276e43a; node 40b6aff75bbd01dd42f219fff535af9cd774cf38082fa837bcec84ede61ae9a4;
  ledger node 1b6128902115d8b8a2050df591fb853fe3723c16535e4b17d90e0c7035119abd. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`,
  `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, and `m_t` hashes and nodes
  unchanged.

- **CODATA 2018 triton mass is a one-sigma Interval.**
  `physis-constants` versions `m_t` as the CODATA 2018 hull
  `5.0073567446(15)×10^{-27}` kg from JPCRD 50, 033105 table XXXI
  (Triton, t). This is not deuteron mass `m_d`, not neutron mass `m_n`,
  not proton mass `m_p`, not muon mass `m_mu`, not electron-triton mass
  ratio `me_mt`, not a certificate of a reconstruction from sibling
  masses or mass ratios, not an SI defining Ratio, not the Thomson
  cross section, and not P3N. The u-row, energy equivalent, MeV
  conversion, mass ratios, molar mass, and magnetic-moment rows are
  later table rows and are not stored. Electron mass is still not stored
  (`10^{42}` overflows `i128`). Decade `10^{37}` (`10^{36}` is the 10×
  trap). This is not the CODATA 2022 last-digit `7512`. `physis_model`
  `triton_mass()` Qty locksteps to the recommended centre inside the
  hull. Adding `m_t` to LEDGER changes the ledger bundle pin. The `G`,
  `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`,
  `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`,
  `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`,
  `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`,
  `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`,
  `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`,
  `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`,
  and `mu_d_mun` hashes are unchanged. Theories still evaluate with
  `f64` Qty. That is not a kernel proof, not Canonical, not P4. Encode
  pins unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `m_t` hash 8f7874deeb241abcd6ab910a824ee8badc1c135ca735604a9e4116b86d6255bc; node 1f3c547a59756969902848bb496c7d19fddbdb577efca55e9e22074dd3f9ccf7;
  ledger node 86a5f95ac9389a8ee23495d9a6a828b9ebad31304ed1aaf44c16fedbacbc8787. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`,
  `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, `mu_d_mup`, and `mu_d_mun` hashes and nodes unchanged.

- **CODATA 2018 deuteron-neutron magnetic-moment ratio is a one-sigma Interval.**
  `physis-constants` versions `mu_d_mun` as the CODATA 2018 hull
  `−0.44820653(11)` from JPCRD 50, 033105 table XXXI (Deuteron, d).
  This is not electron-neutron magnetic-moment ratio `mu_e_mun`, not
  proton-neutron `mu_p_mun`, not deuteron nuclear-magneton ratio
  `mu_d_muN`, not deuteron-proton `mu_d_mup`, not a certificate that this
  equals a reconstructed `μ_d/μ_n` from sibling moments, not an SI
  defining Ratio, not the Thomson cross section, and not P3N. The live
  name `mu_d_mun` is not a case-variant of `mu_d_muN`. Triton rows are
  later table rows and are not stored. Electron mass is still not stored
  (`10^{42}` overflows `i128`). Decade `10^{8}` (`10^{7}` is the 10×
  trap). This is not the CODATA 2022 last-digit `2`. `physis_model`
  `deuteron_neutron_magnetic_moment_ratio()` Qty locksteps to the
  recommended signed centre inside the hull. Adding `mu_d_mun` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`,
  `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, and `mu_d_mup` hashes are unchanged. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical, not
  P4. Encode pins unchanged. Unique-vacuum graph id unchanged. P3N count
  stays 4.
  Verified: `mu_d_mun` hash 4bd463c23a25971e81f82ac0351265971ebedc2e0a3f37ca76935aaf486b4da3; node 1b59d33d5a68a24febbefa7ab88b5800bee2a0d0994ec769f9d147f6d1f6a184;
  ledger node ab6c4f538dda86f0ad9e0fced983371ff40ef2554c3b8cab9e88327024c22f65. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`,
  `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  `mu_d_mue`, and `mu_d_mup` hashes and nodes unchanged.

- **CODATA 2018 deuteron-proton magnetic-moment ratio is a one-sigma Interval.**
  `physis-constants` versions `mu_d_mup` as the CODATA 2018 hull
  `0.30701220939(79)` from JPCRD 50, 033105 table XXXI (Deuteron, d).
  This is not neutron-proton magnetic-moment ratio `mu_n_mup`, not
  electron-proton `mu_e_mup`, not deuteron-proton mass ratio `md_mp`,
  not deuteron-electron `mu_d_mue`, not a certificate that this equals a
  reconstructed `μ_d/μ_p` from sibling moments, not an SI defining
  Ratio, not the Thomson cross section, and not P3N. Neutron
  moment-ratio is a later table row and is not stored. Electron mass is
  still not stored (`10^{42}` overflows `i128`). Decade `10^{11}`
  (`10^{10}` is the 10× trap). This is not the CODATA 2022 last-digit
  `0`. `physis_model` `deuteron_proton_magnetic_moment_ratio()` Qty
  locksteps to the recommended centre inside the hull. Adding `mu_d_mup`
  to LEDGER changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`,
  `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`,
  `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`,
  and `mu_d_mue` hashes are unchanged. Theories still evaluate with
  `f64` Qty. That is not a kernel proof, not Canonical, not P4. Encode
  pins unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `mu_d_mup` hash 714492efa58a8f73eec0856f5a3587db0ba4eeddaf47fef8867ce1f1e5e52120; node 5d500b4b42060fbd1225eb945e6d95322456a99148415d18d11e04039c43be7a;
  ledger node fa2f9f86a3dc30ea51062fce0ca61b9287fe37f40f7ab69b18ff9cd5ba412e19. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`,
  `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, and
  `mu_d_mue` hashes and nodes unchanged.

- **CODATA 2018 deuteron-electron magnetic-moment ratio is a one-sigma Interval.**
  `physis-constants` versions `mu_d_mue` as the CODATA 2018 hull
  `−4.664345551(12)×10^{-4}` from JPCRD 50, 033105 table XXXI (Deuteron, d).
  This is not electron-deuteron magnetic-moment ratio `mu_e_mud`, not
  neutron-electron `mu_n_mue`, not Bohr-magneton ratio `mu_d_muB`, not a
  certificate that this equals the inverse of `mu_e_mud`, not an SI
  defining Ratio, not the Thomson cross section, and not P3N. Proton and
  neutron moment-ratio rows are later table rows and are not stored.
  Electron mass is still not stored (`10^{42}` overflows `i128`). Decade
  `10^{13}` (`10^{12}` is the 10× trap). This is not the CODATA 2022
  last-digit `0`. `physis_model`
  `deuteron_electron_magnetic_moment_ratio()` Qty locksteps to the
  recommended signed centre inside the hull. Adding `mu_d_mue` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`,
  `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, and
  `gd` hashes are unchanged. Theories still evaluate with `f64` Qty.
  That is not a kernel proof, not Canonical, not P4. Encode pins
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `mu_d_mue` hash c14a0b1e4aa0447ff0fe66724f0fe3058a2feec3dc134d41351dd57726d1bdb4; node b7e3f8d483fffdd0e0214b0066e04342242e5d6b9e15c6833c7946aaf8153ca8;
  ledger node 45060d873e6137d2b07ab12e07dd75f7b987b0addd33f0ccde6cd15eac3393e6. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`,
  `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, and `gd`
  hashes and nodes unchanged.

- **CODATA 2018 deuteron g-factor is a one-sigma Interval.**
  `physis-constants` versions `gd` as the CODATA 2018 hull
  `0.8574382338(22)` from JPCRD 50, 033105 table XXXI (Deuteron, d).
  This is not electron g-factor `ge`, not muon g-factor `gmu`, not proton
  g-factor `gp`, not neutron g-factor `gn`, not nuclear-magneton ratio
  `mu_d_muN`, not a certificate that this equals `μ_d/μ_N`, not an SI
  defining Ratio, not the Thomson cross section, and not P3N. JPCRD
  prints the same recommended digits as `mu_d_muN` because `I = 1`; each
  row has its own Claim identity. Moment-ratio rows are later table rows
  and are not stored. Electron mass is still not stored (`10^{42}`
  overflows `i128`). Decade `10^{10}` (`10^{9}` is the 10× trap). This
  is not the CODATA 2022 last-digit `5`. `physis_model`
  `deuteron_g_factor()` Qty locksteps to the recommended centre inside
  the hull. Adding `gd` to LEDGER changes the ledger bundle pin. The
  `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`,
  `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`,
  `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`,
  `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`,
  `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`,
  `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`,
  `mu_d`, `mu_d_muB`, and `mu_d_muN` hashes are unchanged. Theories
  still evaluate with `f64` Qty. That is not a kernel proof, not
  Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4.
  Verified: `gd` hash 2abb89db8e1e22310c2a7f61a8afee935dfe5e73a9034a17537dd58927215d84; node db4f985bb0b58a1e8c23a9d093544983d9b1a9b7d7e29f04b25bb16be0c75f97;
  ledger node b2a9b7fdf3e48c6337a5838815d661c6b2b968f0b29b6dfd171e2bd283a5b9a1. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`,
  `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, and `mu_d_muN` hashes
  and nodes unchanged.

- **CODATA 2018 deuteron magnetic moment to nuclear magneton ratio is a one-sigma Interval.**
  `physis-constants` versions `mu_d_muN` as the CODATA 2018 hull
  `0.8574382338(22)` from JPCRD 50, 033105 table XXXI (Deuteron, d).
  This is not deuteron magnetic moment `mu_d`, not Bohr-magneton ratio
  `mu_d_muB`, not proton nuclear-magneton ratio `mu_p_muN`, not neutron
  `mu_n_muN`, not electron `mu_e_muN`, not muon `mu_mu_muN`, not a
  certificate that this equals the g-factor `gd`, not an SI defining
  Ratio, not the Thomson cross section, and not P3N. JPCRD prints the
  same recommended digits for `gd` on the next row; that row is not
  stored. Electron mass is still not stored (`10^{42}` overflows
  `i128`). Decade `10^{10}` (`10^{9}` is the 10× trap). This is not the
  CODATA 2022 last-digit `5`. `physis_model`
  `deuteron_magnetic_moment_to_nuclear_magneton()` Qty locksteps to the
  recommended centre inside the hull. Adding `mu_d_muN` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`,
  `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, and `mu_d_muB` hashes are
  unchanged. Theories still evaluate with `f64` Qty. That is not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `mu_d_muN` hash 823bcbc76a3eb38e75a8c40931b5eff4b84ad60d667b3bf87e09ed5700ddcf96; node 20dd35057706c81f9f847cfe546030855559e0d520dee37b415d14357eab1d0e;
  ledger node a233353360617ab87cfb5df3cc943c260d00d523acd3ff88099157b30894fd01. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`,
  `md_mp`, `M_d`, `rd`, `mu_d`, and `mu_d_muB` hashes and nodes
  unchanged.

- **CODATA 2018 deuteron magnetic moment to Bohr magneton ratio is a one-sigma Interval.**
  `physis-constants` versions `mu_d_muB` as the CODATA 2018 hull
  `4.669754570(12)×10^{-4}` from JPCRD 50, 033105 table XXXI
  (Deuteron, d). This is not deuteron magnetic moment `mu_d`, not proton
  Bohr-magneton ratio `mu_p_muB`, not neutron `mu_n_muB`, not electron
  `mu_e_muB`, not muon `mu_mu_muB`, not a certificate that this equals
  `μ_d/μ_B` from sibling moments, not an SI defining Ratio, not the
  Thomson cross section, and not P3N. The nuclear-magneton ratio is a
  later table row and is not stored. Electron mass is still not stored
  (`10^{42}` overflows `i128`). Decade `10^{13}` (`10^{12}` is the 10×
  trap; `σ = 1.2` is not an integer). This is not the CODATA 2022
  last-digit `8`. `physis_model`
  `deuteron_magnetic_moment_to_bohr_magneton()` Qty locksteps to the
  recommended centre inside the hull. Adding `mu_d_muB` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`,
  `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, `rd`, and `mu_d` hashes are unchanged.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4.
  Verified: `mu_d_muB` hash 309b4366be950e8638826c987a8f2bfde2d5773bba5896727fb2f4ae859208e1; node 1cb11f5a26cf543d181459b2c769e9a610d5573825e81d6bfcebd41085521ae0;
  ledger node 08ef56e13ccde6c98cc303751e84b8a1f2475c12d5d63034db6287c2aa4edab0. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`,
  `md_mp`, `M_d`, `rd`, and `mu_d` hashes and nodes unchanged.

- **CODATA 2018 deuteron magnetic moment is a one-sigma Interval.**
  `physis-constants` versions `mu_d` as the CODATA 2018 hull
  `4.330735094(11)×10^{-27}` J T⁻¹ from JPCRD 50, 033105 table XXXI
  (Deuteron, d). This is not proton magnetic moment `mu_p`, not neutron
  `mu_n`, not electron `mu_e`, not muon `mu_mu`, not vacuum permeability
  `mu0`, not the electron-deuteron moment ratio `mu_e_mud`, not rms
  charge radius `rd`, not a certificate that this equals `g_d μ_N / 2`,
  not an SI defining Ratio, not the Thomson cross section, and not P3N.
  The Bohr-magneton ratio is a later table row and is not stored.
  Electron mass is still not stored (`10^{42}` overflows `i128`).
  Decade `10^{36}` (`10^{35}` is the 10× trap). This is not the CODATA
  2022 last-digit `7`. `physis_model` `deuteron_magnetic_moment()` Qty
  locksteps to the recommended centre inside the hull. Adding `mu_d` to
  LEDGER changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`,
  `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`,
  `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, `M_d`, and `rd` hashes are unchanged. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4.
  Verified: `mu_d` hash c13c3bf94350f4bfc43806e523fa562c2309d7b6cf3bd2a0d0cca1017e5d3cbf; node 27fa5e1fad0d9768c160ff694e634905908467dc52156c39fb642d7406045835;
  ledger node d620caa21eb0105cf4f76b454c0b2618a7c38875d402c4712f07fcb1267cf4a8. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`,
  `md_mp`, `M_d`, and `rd` hashes and nodes unchanged.

- **CODATA 2018 deuteron rms charge radius is a one-sigma Interval.**
  `physis-constants` versions `rd` as the CODATA 2018 hull
  `2.12799(74)×10^{-15}` m from JPCRD 50, 033105 table XXXI (Deuteron, d).
  This is not proton rms `rp`, not classical electron radius `re`, not
  molar mass `M_d`, not the kg hull `m_d`, not a certificate of a
  deuteron-proton radius difference, not the Table XIX C10 adjusted
  constant `2.111(19) fm`, not an SI defining Ratio, not the Thomson
  cross section, and not P3N. Magnetic-moment rows are later table rows
  and are not stored. Electron mass is still not stored (`10^{42}`
  overflows `i128`). Decade `10^{20}` (`10^{19}` is the 10× trap). This
  is not the CODATA 2022 last-digit `78`. `physis_model`
  `deuteron_rms_charge_radius()` Qty locksteps to the recommended
  centre inside the hull. Adding `rd` to LEDGER changes the ledger
  bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`,
  `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`,
  `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`,
  `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`,
  `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`,
  `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`,
  `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, `md_mp`, and `M_d` hashes are unchanged. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4.
  Verified: `rd` hash 972906db24ea49fb88e62cf390508d5970373c0b9252f4f0ef50c68818c8ecdf; node 0203790131852cb3c7f69d05f48d29347c4955cfbbd096d3906aab205127ede5;
  ledger node 0a928907c69a48ae12b140ebb4d769fb3be443537977130a97b9507bf1616b25. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`,
  `md_mp`, and `M_d` hashes and nodes unchanged.

- **CODATA 2018 deuteron molar mass is a one-sigma Interval.**
  `physis-constants` versions `M_d` as the CODATA 2018 hull
  `2.01355321205(61)×10^{-3}` kg mol⁻¹ from JPCRD 50, 033105 table XXXI
  (Deuteron, d). This is not neutron molar mass `M_n`, not proton
  `M_p`, not electron `M_e`, not muon `M_mu`, not the kg hull `m_d`,
  not the u-row `m_d_u`, not a certificate that this equals `N_A × m_d`,
  not an SI defining Ratio, not the Thomson cross section, and not P3N.
  The rms charge radius is a later table row and is not stored.
  Electron mass is still not stored (`10^{42}` overflows `i128`).
  Decade `10^{14}` (`10^{13}` is the 10× trap). This is not the CODATA
  2022 last-digit `466`. `physis_model` `deuteron_molar_mass()` Qty
  locksteps to the recommended centre inside the hull. Adding `M_d` to
  LEDGER changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`,
  `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`,
  `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`,
  `md_me`, and `md_mp` hashes are unchanged. Theories still evaluate
  with `f64` Qty. That is not a kernel proof, not Canonical, not P4.
  Encode pins unchanged. Unique-vacuum graph id unchanged. P3N count
  stays 4.
  Verified: `M_d` hash cd2742c648825c389159209b4b9ab8105b81bbb4696f9cbd103883f371d3b50a; node 29960ff017c63127f01463751240e48b884a25734f16f442f05e4b42806d3a70;
  ledger node 292e765294c448f34d7623a82852f465f12b62b6c5718052fc3d441d56cc233e. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`,
  and `md_mp` hashes and nodes unchanged.

- **CODATA 2018 deuteron-proton mass ratio is a one-sigma Interval.**
  `physis-constants` versions `md_mp` as the CODATA 2018 hull
  `1.99900750139(11)` from JPCRD 50, 033105 table XXXI (Deuteron, d).
  This is not the deuteron-electron ratio `md_me`, not neutron-proton
  `mn_mp`, not proton-neutron `mp_mn`, not deuteron mass, not proton
  mass, not a certificate that the stored centres reconstruct `m_d/m_p`,
  not an SI defining Ratio, not the Thomson cross section, and not P3N.
  The molar mass and rms charge radius are later table rows and are not
  stored. Electron mass is still not stored (`10^{42}` overflows `i128`).
  Decade `10^{11}` (`10^{10}` is the 10× trap). This is not the CODATA
  2022 last-digit `2699`. `physis_model` `deuteron_proton_mass_ratio()`
  Qty locksteps to the recommended centre inside the hull. Adding
  `md_mp` to LEDGER changes the ledger bundle pin. The `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`,
  `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`,
  `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`,
  `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`,
  `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`,
  `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, and `md_me` hashes are
  unchanged. Theories still evaluate with `f64` Qty. That is not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `md_mp` hash a1c84e01de3c4fb4e5eb0c9de98edd0d13aefc51c32e1c95c2d4203d6144a919; node 76438ea152e9bce3c9733ba0635291b65794e143615119234ed8777508b0f61d;
  ledger node 43d1972dec8b90b34788a970d76f6f0a6d6406075cb149e952b81f65195b5a02. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, and
  `md_me` hashes and nodes unchanged.

- **CODATA 2018 deuteron-electron mass ratio is a one-sigma Interval.**
  `physis-constants` versions `md_me` as the CODATA 2018 hull
  `3670.48296788(13)` from JPCRD 50, 033105 table XXXI (Deuteron, d).
  This is not the electron-deuteron ratio `me_md`, not neutron-electron
  `mn_me`, not proton-electron `mp_me`, not muon-electron `mmu_me`, not
  deuteron mass, not MeV energy equivalent, not a certificate that the
  stored centres invert, not an SI defining Ratio, not the Thomson cross
  section, and not P3N. The deuteron-proton mass ratio, molar mass, and
  rms charge radius are later table rows and are not stored. Electron
  mass is still not stored (`10^{42}` overflows `i128`). Decade `10^{8}`
  (`10^{7}` is the 10× trap). This is not the CODATA 2022 last-digit
  `655`. `physis_model` `deuteron_electron_mass_ratio()` Qty locksteps to
  the recommended centre inside the hull. Adding `md_me` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`,
  `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`,
  `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, and `m_d_c2_MeV`
  hashes are unchanged. Theories still evaluate with `f64` Qty. That is
  not a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `md_me` hash 9c0f207cbd339c3665d3cc96b38c98fddbf31894f385c84fa5197f61a212236d; node 861d04500921d545595f7454950d748d589778b8e15b0633af2e6b2b89af6641;
  ledger node 06df96f84940b53381d00b378b2e3f409e2365a6c3e7eed718b0dbff0f14de9e. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, and `m_d_c2_MeV` hashes
  and nodes unchanged.

- **CODATA 2018 deuteron mass energy equivalent in MeV is a one-sigma Interval.**
  `physis-constants` versions `m_d_c2_MeV` as the CODATA 2018 hull
  `1875.61294257(57)` MeV from JPCRD 50, 033105 table XXXI (Deuteron, d).
  This is not the joule hull `m_d_c2`, not neutron MeV `m_n_c2_MeV`, not
  proton MeV `m_p_c2_MeV`, not muon MeV `m_mu_c2_MeV`, not Hartree, not
  the exact electronvolt Ratio, not a certificate of a reconstruction
  from sibling masses, not an SI defining Ratio, not the Thomson cross
  section, and not P3N. The deuteron-electron mass ratio, molar mass, and
  rms charge radius are later table rows and are not stored. Electron
  mass is still not stored (`10^{42}` overflows `i128`). Decade `10^{8}`
  (`10^{7}` is the 10× trap). This is not the CODATA 2022 last-digit
  `94500`. `physis_model` `deuteron_mass_energy_equivalent_in_mev()` Qty
  locksteps to the recommended centre inside the hull. Adding
  `m_d_c2_MeV` to LEDGER changes the ledger bundle pin. The `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`,
  `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`,
  `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`,
  `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, and `m_d_c2` hashes are unchanged.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4.
  Verified: `m_d_c2_MeV` hash 798ca780f4396ff95cc3582e796b9830baa21f69cf653160827d0d0f80502a0d; node 1fd451a0f9e39fa6f32963caf5375259d716af30de6dc462c39eadfac63a644a;
  ledger node 067e230a88ce948f4902aff11ee7d29153d0ea0a26d0b0366176a0e856a0b61c. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, `m_d_u`, and `m_d_c2` hashes and nodes unchanged.

- **CODATA 2018 deuteron mass energy equivalent is a one-sigma Interval.**
  `physis-constants` versions `m_d_c2` as the CODATA 2018 hull
  `3.00506323102(91)×10^{-10}` J from JPCRD 50, 033105 table XXXI
  (Deuteron, d). This is not the kg hull `m_d`, not the u-row `m_d_u`,
  not neutron joule `m_n_c2`, not proton joule `m_p_c2`, not muon joule
  `m_mu_c2`, not Rydberg energy equivalent, not Hartree, not the exact
  electronvolt Ratio, not a certificate of a reconstruction from sibling
  masses, not an SI defining Ratio, not the Thomson cross section, and
  not P3N. The MeV conversion, molar mass, and rms charge radius are
  later table rows and are not stored. Electron mass is still not stored
  (`10^{42}` overflows `i128`). Decade `10^{21}` (`10^{20}` is the 10×
  trap). This is not the CODATA 2022 last-digit `23491`. `physis_model`
  `deuteron_mass_energy_equivalent()` Qty locksteps to the recommended
  centre inside the hull. Adding `m_d_c2` to LEDGER changes the ledger
  bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`,
  `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`,
  `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`,
  `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`,
  `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`,
  `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`,
  `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`,
  `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`,
  `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`,
  `mu_n_mup`, `mu_n_mu0p`, `m_d`, and `m_d_u` hashes are unchanged.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4.
  Verified: `m_d_c2` hash dfe04d95b7f5a00ad95da03903d7218c1ba7e38d88d89a3e1644018a94e65868; node 98f021436a9b05709710fb44e4d4302f2559a000c80632c589d0ce35a60a310f;
  ledger node 1cbaf68c8ae0c2aad242d3e4beb088bb096164be51c7fbceb459f967f0337df6. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, `m_d`, and `m_d_u` hashes and nodes unchanged.

- **CODATA 2018 deuteron mass in u is a one-sigma Interval.**
  `physis-constants` versions `m_d_u` as the CODATA 2018 hull
  `2.013553212745(40)` u from JPCRD 50, 033105 table XXXI (Deuteron, d).
  This is not the kg hull `m_d`, not neutron mass in u `m_n_u`, not proton
  mass in u `m_p_u`, not muon mass in u `m_mu_u`, not relative atomic mass
  under a different name, not a certificate of a reconstruction from
  sibling masses, not an SI defining Ratio, not the Thomson cross section,
  and not P3N. The energy equivalent, molar mass, and rms charge radius
  are later table rows and are not stored. Electron mass is still not
  stored (`10^{42}` overflows `i128`). Decade `10^{12}` (`10^{11}` is the
  10× trap). This is not the CODATA 2022 last-digit `544`. `physis_model`
  `deuteron_mass_in_u()` Qty locksteps to the recommended centre inside
  the hull. Adding `m_d_u` to LEDGER changes the ledger bundle pin. The
  `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`,
  `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`,
  `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`,
  `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, and `m_d` hashes are unchanged. Theories still evaluate
  with `f64` Qty. That is not a kernel proof, not Canonical, not P4.
  Encode pins unchanged. Unique-vacuum graph id unchanged. P3N count
  stays 4.
  Verified: `m_d_u` hash 7a2afd4043689b99d9f043af14347050cf8d4f6b774886642c256c6ab0f2abbe; node d5f940659bb086ee0fe58faea4e01c168cf873d12a762de08d154892ce348373;
  ledger node 95e2c85293565a0db78056c6660cb46d28ed5095f5256230f9c4c287eb549403. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  `mu_n_mu0p`, and `m_d` hashes and nodes unchanged.

- **CODATA 2018 deuteron mass is a one-sigma Interval.**
  `physis-constants` versions `m_d` as the CODATA 2018 hull
  `3.3435837724(10)×10^{-27}` kg from JPCRD 50, 033105 table XXXI
  (Deuteron, d). This is not neutron mass `m_n`, not proton mass `m_p`,
  not muon mass `m_mu`, not electron-deuteron mass ratio `me_md`, not a
  certificate of a reconstruction from sibling masses, not an SI defining
  Ratio, not the Thomson cross section, and not P3N. The u-row, energy
  equivalent, molar mass, and rms charge radius are later table rows and
  are not stored. Electron mass is still not stored (`10^{42}` overflows
  `i128`). Decade `10^{37}` (`10^{36}` is the 10× trap). `10^{39}`
  overflows `i128`. This is not the CODATA 2022 last-digit `7768`.
  `physis_model` `deuteron_mass()` Qty locksteps to the recommended
  centre inside the hull. Adding `m_d` to LEDGER changes the ledger
  bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`,
  `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`,
  `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`,
  `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`,
  `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, and `mu_n_mu0p` hashes
  are unchanged. Theories still evaluate with `f64` Qty. That is not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `m_d` hash 0710831944a3d44d75fd1e63f10fd9f06edc9b0d93028dd57749f807b1a37432; node f4182bcd2231bc769f766746198bf60aa496cf0ae1b9ca7726cfc78c775eeaab;
  ledger node fe64c2663bdc031e3897dd444af8c67196cbdd0ef918aad4235f9733dc430dfe. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`,
  and `mu_n_mu0p` hashes and nodes unchanged.

- **CODATA 2018 neutron to shielded-proton magnetic-moment ratio is a one-sigma Interval.**
  `physis-constants` versions `mu_n_mu0p` as the CODATA 2018 hull
  `−0.68499694(16)` from JPCRD 50, 033105 table XXXI (Neutron, n).
  This is not free neutron-proton magnetic-moment ratio `mu_n_mup`, not
  electron to shielded-proton magnetic-moment ratio `mu_e_mu0p`, not
  shielded proton magnetic moment `mu0p`, not a certificate that this
  equals a reconstructed μ_n/μ′_p from sibling moments, not an SI
  defining Ratio, not the Thomson cross section, and not P3N.
  Gyromagnetic ratios cite ħ and are not stored. Neutron-tau is a PDG
  reprint (footnote e) and is not stored. Electron mass is still not
  stored (`10^{42}` overflows `i128`). Decade `10^{8}` (`10^{7}` is the
  10× trap). `physis_model`
  `neutron_to_shielded_proton_magnetic_moment_ratio()` Qty locksteps to
  the recommended signed centre inside the hull. Adding `mu_n_mu0p` to
  LEDGER changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`,
  `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, `gn`, `mu_n_mue`, and `mu_n_mup` hashes are unchanged.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4.
  Verified: `mu_n_mu0p` hash c691ce436f1d907625b45c49330e4f5481fab606584c677a69291b1897580550; node eb647725813c11763a6c554d40c2e6cb7c085fe4383c225ebe809024b9904620;
  ledger node ecffb7a9cc32500b1531c5c829e85e50bab3b744708984ca828356481d7baea0. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, and
  `mu_n_mup` hashes and nodes unchanged.

- **CODATA 2018 neutron-proton magnetic-moment ratio is a one-sigma Interval.**
  `physis-constants` versions `mu_n_mup` as the CODATA 2018 hull
  `−0.68497934(16)` from JPCRD 50, 033105 table XXXI (Neutron, n).
  This is not proton-neutron magnetic-moment ratio `mu_p_mun`, not
  neutron-electron magnetic-moment ratio `mu_n_mue`, not electron-proton
  magnetic-moment ratio `mu_e_mup`, not neutron-proton mass ratio
  `mn_mp`, not a certificate that this equals the inverse of
  `mu_p_mun`, not an SI defining Ratio, not the Thomson cross section,
  and not P3N. The shielded-proton moment-ratio is a later table row
  and is not stored. Neutron-tau is a PDG reprint (footnote e) and is
  not stored. Gyromagnetic ratios cite ħ and are not stored. Electron
  mass is still not stored (`10^{42}` overflows `i128`). Decade `10^{8}`
  (`10^{7}` is the 10× trap). This is not the CODATA 2022 last-digit
  `35`. `physis_model` `neutron_proton_magnetic_moment_ratio()` Qty
  locksteps to the recommended signed centre inside the hull. Adding
  `mu_n_mup` to LEDGER changes the ledger bundle pin. The `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`,
  `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`,
  `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`,
  `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, and `mu_n_mue` hashes are
  unchanged. Theories still evaluate with `f64` Qty. That is not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `mu_n_mup` hash 75b137c205d9297127a1955dc686acc1c0206fb92d7a7e4751a63addbef18942; node 677f9e0d701a23bb4a86c03666f3e9af0850affa168ef263f4170843cb91f2ad;
  ledger node f50c1106993f50db92595eeeded698f9d4c3844e1d5babcf63969b92c9e3f2de. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, and `mu_n_mue` hashes
  and nodes unchanged.

- **CODATA 2018 neutron-electron magnetic-moment ratio is a one-sigma Interval.**
  `physis-constants` versions `mu_n_mue` as the CODATA 2018 hull
  `1.04066882(25)×10^{-3}` from JPCRD 50, 033105 table XXXI (Neutron, n).
  This is not electron-neutron magnetic-moment ratio `mu_e_mun`, not
  neutron g-factor `gn`, not electron-proton magnetic-moment ratio
  `mu_e_mup`, not neutron-electron mass ratio `mn_me`, not a certificate
  that this equals the inverse of `mu_e_mun`, not an SI defining Ratio,
  not the Thomson cross section, and not P3N. Neutron-proton and
  shielded-proton moment-ratio rows are later table rows and are not
  stored. Neutron-tau is a PDG reprint (footnote e) and is not stored.
  Gyromagnetic ratios cite ħ and are not stored. Electron mass is still
  not stored (`10^{42}` overflows `i128`). Decade `10^{11}` (`10^{10}`
  is the 10× trap). This is not the CODATA 2022 last-digit `84`.
  `physis_model` `neutron_electron_magnetic_moment_ratio()` Qty locksteps
  to the recommended centre inside the hull. Adding `mu_n_mue` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`,
  `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`,
  `mu_n_muN`, and `gn` hashes are unchanged. Theories still evaluate
  with `f64` Qty. That is not a kernel proof, not Canonical, not P4.
  Encode pins unchanged. Unique-vacuum graph id unchanged. P3N count
  stays 4.
  Verified: `mu_n_mue` hash 6373ed28fe84cda2c7b3eb096b4db0662e477f295ff05247eafedc9aa61707bc; node 2116e88a6a333c70afbbac409f03add69be63c505641359e0311ddb16f98bff0;
  ledger node 57c6609ecb56844a1727e17d0936d50ccbab7a81887bcf6f6dc56ecbae5c9fa8. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, `mu_n_muN`, and `gn` hashes and nodes
  unchanged.

- **CODATA 2018 neutron g-factor is a one-sigma Interval.**
  `physis-constants` versions `gn` as the CODATA 2018 hull
  `−3.82608545(90)` from JPCRD 50, 033105 table XXXI (Neutron, n). This
  is not electron g-factor `ge`, not muon g-factor `gmu`, not proton
  g-factor `gp`, not nuclear-magneton ratio `mu_n_muN`, not a
  certificate that this equals `2 μ_n/μ_N`, not an SI defining Ratio,
  not the Thomson cross section, and not P3N. Moment-ratio rows are
  later table rows and are not stored. Neutron-tau is a PDG reprint
  (footnote e) and is not stored. Gyromagnetic ratios cite ħ and are
  not stored. Electron mass is still not stored (`10^{42}` overflows
  `i128`). Decade `10^{8}` (`10^{7}` is the 10× trap). This is not the
  CODATA 2022 last-digit `52`. `physis_model` `neutron_g_factor()` Qty
  locksteps to the recommended signed centre inside the hull. Adding
  `gn` to LEDGER changes the ledger bundle pin. The `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`,
  `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`,
  `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`,
  `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`,
  `mu_n_muB`, and `mu_n_muN` hashes are unchanged. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4.
  Verified: `gn` hash 745d414efe1b217af8239787ffd1bb6d0d820fd8753d8aeaedac10c87658aac9; node 5e266c7126aeed691a87f41410dd736298097a6a81760313b2d47517c7a3ebad;
  ledger node 4a97cf8518c4af08293960e424ae1d97abfc1c67c57c06b8b07b7f5af0433024. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, `mu_n_muB`, and `mu_n_muN` hashes and nodes unchanged.

- **CODATA 2018 neutron nuclear-magneton ratio is a one-sigma Interval.**
  `physis-constants` versions `mu_n_muN` as the CODATA 2018 hull
  `−1.91304273(45)` from JPCRD 50, 033105 table XXXI (Neutron, n). This
  is not neutron magnetic moment `mu_n`, not Bohr-magneton ratio
  `mu_n_muB`, not proton nuclear-magneton ratio `mu_p_muN`, not electron
  nuclear-magneton ratio `mu_e_muN`, not muon nuclear-magneton ratio
  `mu_mu_muN`, not a certificate that this equals `2 μ_n/μ_N` (the
  g-factor), not an SI defining Ratio, not the Thomson cross section,
  and not P3N. G-factor and moment-ratio rows are later table rows and
  are not stored. Neutron-tau is a PDG reprint (footnote e) and is not
  stored. Gyromagnetic ratios cite ħ and are not stored. Electron mass
  is still not stored (`10^{42}` overflows `i128`). Decade `10^{8}`
  (`10^{7}` is the 10× trap). `physis_model`
  `neutron_magnetic_moment_to_nuclear_magneton()` Qty locksteps to the
  recommended signed centre inside the hull. Adding `mu_n_muN` to
  LEDGER changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`,
  `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, and `mu_n_muB`
  hashes are unchanged. Theories still evaluate with `f64` Qty. That is
  not a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `mu_n_muN` hash 7f589089b63d3ad32bd59d8c16b542704a4dab7185cae50e8ed4179011ebb7f2; node 32dc6a7e69b86f074f85c31946033fa8267e4d48df5b511a2bd371f813c383ed;
  ledger node 8d98bea3730746dbf6c48fccd0f7b7e8817c440f3af8aba18d9540a2d1bca5ea. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  `mu_n`, and `mu_n_muB` hashes and nodes unchanged.

- **CODATA 2018 neutron Bohr-magneton ratio is a one-sigma Interval.**
  `physis-constants` versions `mu_n_muB` as the CODATA 2018 hull
  `−1.04187563(25)×10^{-3}` from JPCRD 50, 033105 table XXXI
  (Neutron, n). This is not neutron magnetic moment `mu_n`, not proton
  Bohr-magneton ratio `mu_p_muB`, not electron Bohr-magneton ratio
  `mu_e_muB`, not muon Bohr-magneton ratio `mu_mu_muB`, not a
  certificate that this equals a reconstructed `μ_n/μ_B`, not an SI
  defining Ratio, not the Thomson cross section, and not P3N. Nuclear,
  g-factor, and moment-ratio rows are later table rows and are not
  stored. Neutron-tau is a PDG reprint (footnote e) and is not stored.
  Gyromagnetic ratios cite ħ and are not stored. Electron mass is still
  not stored (`10^{42}` overflows `i128`). Decade `10^{11}` (`10^{10}`
  is the 10× trap). `physis_model`
  `neutron_magnetic_moment_to_bohr_magneton()` Qty locksteps to the
  recommended signed centre inside the hull. Adding `mu_n_muB` to
  LEDGER changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`,
  `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, and `mu_n` hashes are
  unchanged. Theories still evaluate with `f64` Qty. That is not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `mu_n_muB` hash 1e8fe1ba579ca229293ab7fd77a791d19df2f97296117a66bdf20339b4c0f45d; node c4f9861a3802f769b5127e1c0e6308c19920c5baacb3eae95802b855984c9d1e;
  ledger node c6b27eb1fd49a0c7d282d13c7e4301ea89a6a14df5d779fa05b7ff23af218899. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`,
  and `mu_n` hashes and nodes unchanged.

- **CODATA 2018 neutron magnetic moment is a one-sigma Interval.**
  `physis-constants` versions `mu_n` as the CODATA 2018 hull
  `−9.6623651(23)×10^{-27}` J T⁻¹ from JPCRD 50, 033105 table XXXI
  (Neutron, n). This is not proton magnetic moment `mu_p`, not electron
  magnetic moment `mu_e`, not muon magnetic moment `mu_mu`, not vacuum
  permeability `mu0`, not Compton wavelength `lambda_C_n`, not a
  certificate that this equals `g_n μ_N / 2`, not an SI defining
  Ratio, not the Thomson cross section, and not P3N. Bohr, nuclear,
  g-factor, and moment-ratio rows are later table rows and are not
  stored. Neutron-tau is a PDG reprint (footnote e) and is not stored.
  Gyromagnetic ratios cite ħ and are not stored. Electron mass is still
  not stored (`10^{42}` overflows `i128`). Decade `10^{34}` (`10^{33}`
  is the 10× trap). This is not the CODATA 2022 last-digit `3`.
  `physis_model` `neutron_magnetic_moment()` Qty locksteps to the
  recommended signed centre inside the hull. Adding `mu_n` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`,
  `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, `M_n`, and `lambda_C_n` hashes are unchanged.
  Theories still evaluate with `f64` Qty. That is not a kernel
  proof, not Canonical, not P4. Encode pins unchanged. Unique-vacuum
  graph id unchanged. P3N count stays 4.
  Verified: `mu_n` hash c9a6a49c3c793cee8a4e3f31b1245f16c05c8b90c6e5fb1752fff1f2337b5f2c; node f1a990d6d1f6c6abdae0834a0153935da5673e4979a29d6988e4ba563c90aa1d;
  ledger node 8082ff4f4c758892b085581f40c49ea8ceff517ea6ec0c5ea17773f8023e1078. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, and `lambda_C_n`
  hashes and nodes unchanged.

- **CODATA 2018 neutron Compton wavelength is a one-sigma Interval.**
  `physis-constants` versions `lambda_C_n` as the CODATA 2018 hull
  `1.31959090581(75)×10^{-15}` m from JPCRD 50, 033105 table XXXI
  (Neutron, n). This is not electron Compton `lambda_C`, not proton
  Compton `lambda_C_p`, not muon Compton `lambda_C_mu`, not a
  certificate that this equals `2π` times a reduced Compton
  wavelength, not molar mass `M_n`, not an SI defining Ratio, not the
  Thomson cross section, and not P3N. Reduced neutron Compton cites ħ
  and is not stored. Neutron-tau is a PDG reprint (footnote e) and is
  not stored. Gyromagnetic ratios cite ħ and are not stored. Electron
  mass is still not stored (`10^{42}` overflows `i128`). Decade
  `10^{26}` (`10^{25}` is the 10× trap). `physis_model`
  `neutron_compton_wavelength()` Qty locksteps to the recommended
  centre as the CODATA decimal (Ratio::to_f64 on the 10^{26} centre is
  one ulp below that decimal). Adding `lambda_C_n` to LEDGER changes
  the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`,
  `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`,
  `mn_minus_mp_c2_MeV`, and `M_n` hashes are unchanged.
  Theories still evaluate with `f64` Qty. That is not a kernel
  proof, not Canonical, not P4. Encode pins unchanged. Unique-vacuum
  graph id unchanged. P3N count stays 4.
  Verified: `lambda_C_n` hash 415d4c31f9aafa35bb84bf48212aa1669b889e9e3e147dec957a1d64a551a449; node 7e23a68e4206617cddff21229e106f6cb57a4a7f1eb41316ee4e2c251a2ac85a;
  ledger node 8fdd7f3c875365130642d6899b59729d211dc1e5b35fdff7c573bd759fe75a3b. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, and `M_n` hashes and
  nodes unchanged.

- **CODATA 2018 neutron molar mass is a one-sigma Interval.**
  `physis-constants` versions `M_n` as the CODATA 2018 hull
  `1.00866491560(57)×10^{-3}` kg mol⁻¹ from JPCRD 50, 033105 table XXXI
  (Neutron, n). This is not proton molar mass `M_p`, not electron
  molar mass `M_e`, not muon molar mass `M_mu`, not the kg hull `m_n`,
  not the u-row `m_n_u`, not a certificate that this equals `N_A` times
  the neutron-mass hull, not an SI defining Ratio, not the Thomson
  cross section, and not P3N. Neutron-tau is a PDG reprint (footnote
  e) and is not stored. Reduced Compton and gyromagnetic ratios cite ħ
  and are not stored. Electron mass is still not stored (`10^{42}`
  overflows `i128`). Decade `10^{14}` (`10^{13}` is the 10× trap).
  `physis_model` `neutron_molar_mass()` Qty locksteps to the recommended
  centre inside the hull. Adding `M_n` to LEDGER changes the ledger
  bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`,
  `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`,
  `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`,
  `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`,
  `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`,
  `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`,
  `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`,
  `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`,
  `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`, and
  `mn_minus_mp_c2_MeV` hashes are unchanged.
  Theories still evaluate with `f64` Qty. That is not a kernel
  proof, not Canonical, not P4. Encode pins unchanged. Unique-vacuum
  graph id unchanged. P3N count stays 4.
  Verified: `M_n` hash 503014b9a1cfa5be5f983c7cd8f477ec6fa601225d084f3acd22ab41b88151d5; node 8351e225509b669ee96bf0e8e63baff12e020575065270ef2d54cc77877ed1c0;
  ledger node ca7a363797c43459d9d47f63d145dd5079825df3eb18cefae0e940985c8052bf. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`,
  `mn_minus_mp_c2`, and `mn_minus_mp_c2_MeV` hashes and nodes
  unchanged.

  `physis-constants` versions `mn_minus_mp_c2_MeV` as the CODATA 2018 hull
  `1.29333236(46)` MeV from JPCRD 50, 033105 table XXXI
  (Neutron, n). This is not the joule hull `mn_minus_mp_c2`, not
  neutron MeV `m_n_c2_MeV`, not proton MeV `m_p_c2_MeV`, not a
  certificate of a reconstruction from sibling masses, `c`, or the
  exact electronvolt Ratio, not an SI defining Ratio, not the Thomson
  cross section, and not P3N. Neutron-tau is a PDG reprint (footnote
  e) and is not stored. Molar mass is a later table row and is not
  stored. Reduced Compton and gyromagnetic ratios cite ħ and are not
  stored. Electron mass is still not stored (`10^{42}` overflows
  `i128`). Decade `10^{8}` (`10^{7}` is the 10× trap).
  `physis_model` `neutron_proton_mass_difference_energy_equivalent_in_mev()`
  Qty locksteps to the recommended centre inside the hull. Adding
  `mn_minus_mp_c2_MeV` to LEDGER changes the ledger bundle pin. The `G`,
  `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`,
  `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`,
  `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`,
  `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`,
  `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`, and `mn_minus_mp_c2`
  hashes are unchanged.
  Theories still evaluate with `f64` Qty. That is not a kernel
  proof, not Canonical, not P4. Encode pins unchanged. Unique-vacuum
  graph id unchanged. P3N count stays 4.
  Verified: `mn_minus_mp_c2_MeV` hash 1049facdec3000d011eb1c003e5a3c4ee952917b78aaed18b503398a2627515a; node 3755ecb6520d54be37dbc22d659304b2504065b84d4a3b41422db7efe9a2ec6d;
  ledger node 2d7c7a5a044574cb6b1bdaa95be0cc9fa88f709e8f6ff0c612b28ff076fec4f6. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`, and
  `mn_minus_mp_c2` hashes and nodes unchanged.

  `physis-constants` versions `mn_minus_mp_c2` as the CODATA 2018 hull
  `2.07214689(74)×10^{-13}` J from JPCRD 50, 033105 table XXXI
  (Neutron, n). This is not the kg hull `mn_minus_mp`, not the u-row
  `mn_minus_mp_u`, not neutron joule `m_n_c2`, not proton joule
  `m_p_c2`, not a certificate of a reconstruction from sibling masses
  or `c`, not an SI defining Ratio, not the Thomson cross section,
  and not P3N. Neutron-tau is a PDG reprint (footnote e) and is not
  stored. The MeV row and molar mass are later table rows and are not
  stored. Reduced Compton and gyromagnetic ratios cite ħ and are not
  stored. Electron mass is still not stored (`10^{42}` overflows
  `i128`). Decade `10^{21}` (`10^{20}` is the 10× trap).
  `physis_model` `neutron_proton_mass_difference_energy_equivalent()`
  Qty locksteps to the recommended centre inside the hull. Adding
  `mn_minus_mp_c2` to LEDGER changes the ledger bundle pin. The `G`,
  `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`,
  `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`,
  `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`,
  `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`,
  `mn_mp`, `mn_minus_mp`, and `mn_minus_mp_u` hashes are unchanged.
  Theories still evaluate with `f64` Qty. That is not a kernel
  proof, not Canonical, not P4. Encode pins unchanged. Unique-vacuum
  graph id unchanged. P3N count stays 4.
  Verified: `mn_minus_mp_c2` hash fa3bd81d58322a2d13a2e2c98b628fef9fb69bcdcf7c3335cd8e26d6b2fb2c45; node d80066ce06701b9bfe83a44cfb9aa8a34bacc23e1d1a3860666ec0b317be0e08;
  ledger node 14bebc8eeb638f068e4a929363c0a053e5451b6dd2afdc698e408e0518cfe07d. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, `mn_minus_mp`, and `mn_minus_mp_u` hashes
  and nodes unchanged.

- **CODATA 2018 neutron-proton mass difference in u is a one-sigma Interval.**
  `physis-constants` versions `mn_minus_mp_u` as the CODATA 2018 hull
  `1.38844933(49)×10^{-3}` u from JPCRD 50, 033105 table XXXI
  (Neutron, n). This is not the kg hull `mn_minus_mp`, not neutron
  mass in u `m_n_u`, not proton mass in u `m_p_u`, not a certificate
  of a reconstruction from the kg hull, not an SI defining Ratio,
  not the Thomson cross section, and not P3N. Neutron-tau is a PDG
  reprint (footnote e) and is not stored. The joule and MeV energy
  equivalents and molar mass are later table rows and are not stored.
  Reduced Compton and gyromagnetic ratios cite ħ and are not stored.
  Electron mass is still not stored (`10^{42}` overflows `i128`).
  Decade `10^{11}` (`10^{10}` is the 10× trap). `physis_model`
  `neutron_proton_mass_difference_in_u()` Qty locksteps to the
  recommended centre inside the hull. Adding `mn_minus_mp_u` to
  LEDGER changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`,
  `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`,
  `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`,
  `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`,
  `mn_mp`, and `mn_minus_mp` hashes are unchanged. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged.
  P3N count stays 4.
  Verified: `mn_minus_mp_u` hash 3e84ef4f23bdc6d6c4b0f17c5ee3535eb8352184eba5836d253750fabdcbf122; node 393944c68f266ffe577f831ae5c03ecbccfc9f2ee336efd281ca912b89fbdb3c;
  ledger node be9e5f1f2ebfa8044d59332afc686773b98c2440ae66ecb550d5132d9c7c5b28. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, `mn_mp`, and `mn_minus_mp` hashes and nodes unchanged.

- **CODATA 2018 neutron-proton mass difference is a one-sigma Interval.**
  `physis-constants` versions `mn_minus_mp` as the CODATA 2018 hull
  `2.30557435(82)×10^{-30}` kg from JPCRD 50, 033105 table XXXI
  (Neutron, n). This is not neutron mass `m_n`, not proton mass `m_p`,
  not the mass ratio `mn_mp`, not a certificate that stored centres
  subtract, not an SI defining Ratio, not the Thomson cross section,
  and not P3N. Neutron-tau is a PDG reprint (footnote e) and is not
  stored. The u-row, joule and MeV energy equivalents, and molar mass
  are later table rows and are not stored. Reduced Compton and
  gyromagnetic ratios cite ħ and are not stored. Electron mass is
  still not stored (`10^{42}` overflows `i128`). Decade `10^{38}`
  (`10^{37}` is the 10× trap; `10^{39}` overflows `i128`).
  `physis_model` `neutron_proton_mass_difference()` Qty locksteps to
  the recommended centre inside the hull. Adding `mn_minus_mp` to
  LEDGER changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`,
  `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`,
  `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`,
  `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, and
  `mn_mp` hashes are unchanged. Theories still evaluate with `f64`
  Qty. That is not a kernel proof, not Canonical, not P4. Encode pins
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `mn_minus_mp` hash 16a765afe5d22205b54dcb17568b53b031f246b75748ece1624d4e553dbca66a; node c02b5d5688fa618f7e226612267689b8a0f1428e6b1f7ead92f7b5a5959789da;
  ledger node 0e878e7e823f885cb55e12ed65e368f4bb41a378e960d9a13c8d0f3e2a4a93d9. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  `mn_mmu`, and `mn_mp` hashes and nodes unchanged.

- **CODATA 2018 neutron-proton mass ratio is a one-sigma Interval.**
  `physis-constants` versions `mn_mp` as the CODATA 2018 hull
  `1.00137841931(49)` from JPCRD 50, 033105 table XXXI (Neutron, n).
  This is not proton-neutron `mp_mn`, not electron-neutron `me_mn`,
  not muon-neutron `mmu_mn`, not neutron-muon `mn_mmu`, not a
  certificate that stored centres invert, not an SI defining Ratio,
  not the Thomson cross section, and not P3N. Neutron-tau is a PDG
  reprint (footnote e) and is not stored. Neutron-proton mass
  difference is a later table row and is not stored. Reduced Compton
  and gyromagnetic ratios cite ħ and are not stored. Electron mass
  is still not stored (`10^{42}` overflows `i128`). Decade `10^{11}`
  (`10^{10}` is the 10× trap). `physis_model`
  `neutron_proton_mass_ratio()` Qty locksteps to the recommended
  centre inside the hull. Adding `mn_mp` to LEDGER changes the
  ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`,
  `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`,
  `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`,
  `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, and `mn_mmu`
  hashes are unchanged. Theories still evaluate with `f64` Qty.
  That is not a kernel proof, not Canonical, not P4. Encode pins
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `mn_mp` hash 4a1cc9e1870d573594b4de7b64f6ec1667f13da26f6cb8b72ae351eed5c89eb4; node 945a9d7cd8a8baf4c852e73d2b86cb4cca450eeee92775737e6b50877abce21d;
  ledger node ca68c11d44b28415daecdc32dfbf8a9c83259b0d996bc26ac4c856497f14e556. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`,
  `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`,
  and `mn_mmu` hashes and nodes unchanged.

- **CODATA 2018 neutron-muon mass ratio is a one-sigma Interval.**
  `physis-constants` versions `mn_mmu` as the CODATA 2018 hull
  `8.89248406(20)` from JPCRD 50, 033105 table XXXI (Neutron, n).
  This is not muon-neutron `mmu_mn`, not proton-muon `mp_mmu`, not
  neutron-electron `mn_me`, not a certificate that stored centres
  invert, not an SI defining Ratio, not the Thomson cross section,
  and not P3N. Neutron-tau is a PDG reprint (footnote e) and is not
  stored. Neutron-proton is a later table row and is not stored.
  Reduced Compton and gyromagnetic ratios cite ħ and are not stored.
  Electron mass is still not stored (`10^{42}` overflows `i128`).
  Decade `10^{8}` (`10^{7}` is the 10× trap). `physis_model`
  `neutron_muon_mass_ratio()` Qty locksteps to the recommended
  centre inside the hull. Adding `mn_mmu` to LEDGER changes the
  ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`,
  `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`,
  `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`,
  `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, and `mn_me` hashes are
  unchanged. Theories still evaluate with `f64` Qty. That is not
  a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified:
  `mn_mmu` hash ab9106af98fa15acd9352bb71a5874aa8f151d4530172a7f704bbb8cc96e52b4; node cf2fb2b1ded7aec6b2512de342860fb4bc3a9f7be2f2d8ce37fd0ef180047f12; ledger node
  1117d0485eb9d210d2438b42045ecf2375cddc41040a10b7557e523dd3105736. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`,
  `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`,
  `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`,
  `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, and `mn_me` hashes
  and nodes unchanged.

- **CODATA 2018 neutron-electron mass ratio is a one-sigma Interval.**
  `physis-constants` versions `mn_me` as the CODATA 2018 hull
  `1838.68366173(89)` from JPCRD 50, 033105 table XXXI
  (Neutron, n). This is not electron-neutron `me_mn`, not
  proton-electron `mp_me`, not muon-electron `mmu_me`, not neutron
  mass energy equivalent in MeV `m_n_c2_MeV`, not a certificate that
  stored centres invert, not an SI defining Ratio, not the Thomson
  cross section, and not P3N. Neutron-muon is a later table row and
  is not stored. Neutron-tau is a PDG reprint (footnote e) and is
  not stored. Reduced Compton and gyromagnetic ratios cite ħ and
  are not stored. Electron mass is still not stored (`10^{42}`
  overflows `i128`). Decade `10^{8}` (`10^{7}` is the 10× trap).
  `physis_model` `neutron_electron_mass_ratio()` Qty locksteps to
  the recommended centre inside the hull. Adding `mn_me` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`,
  `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`,
  `m_n`, `m_n_u`, `m_n_c2`, and `m_n_c2_MeV` hashes are unchanged.
  Theories still evaluate with `f64` Qty. That is not a kernel
  proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified:
  `mn_me` hash 024275bcd4128d6844efaf9740b16e69a8cfbb349d257465ad96a3cbada32b00; node 785b3da762cf12d86766853b74f872bc4c328111748be893a1ca4b39fef77fbb; ledger node
  7672ee9aadb84d02d5021d62399c4457f447b3b337d369e38e92845ed4639b20. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`,
  `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`,
  `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`,
  `m_n`, `m_n_u`, `m_n_c2`, and `m_n_c2_MeV` hashes and nodes
  unchanged.

- **CODATA 2018 neutron mass energy equivalent in MeV is a one-sigma Interval.**
  `physis-constants` versions `m_n_c2_MeV` as the CODATA 2018 hull
  `939.56542052(54)` MeV from JPCRD 50, 033105 table XXXI
  (Neutron, n). This is not neutron mass energy equivalent `m_n_c2`,
  not proton MeV `m_p_c2_MeV`, not muon MeV `m_mu_c2_MeV`, not
  Hartree `Eh`, not the exact electronvolt, not a certificate of a
  reconstruction from sibling masses, not an SI defining Ratio, not
  the Thomson cross section, and not P3N. Molar mass is a later table
  row and is not stored. Neutron-tau is a PDG reprint (footnote e)
  and is not stored. Reduced Compton and gyromagnetic ratios cite ħ
  and are not stored. Electron mass is still not stored (`10^{42}`
  overflows `i128`). Decade `10^{8}` (`10^{7}` is the 10× trap).
  `physis_model` `neutron_mass_energy_equivalent_in_mev()` Qty
  locksteps to the recommended centre inside the hull. Adding
  `m_n_c2_MeV` to LEDGER changes the ledger bundle pin. The `G`,
  `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`,
  `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`,
  `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`,
  `m_n`, `m_n_u`, and `m_n_c2` hashes are unchanged. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged.
  P3N count stays 4. Verified: `m_n_c2_MeV` hash 7f7aff06d346ee861dfaf56598a565600b09c0171deb1f46617ccc7a08aefef8;
  node 8e291e13da25fdb5817f4ce46a62918d72e82a1a023efbac440e986768ccc034; ledger node 4544d4bda745662274dc449dd54298c7dc9db3c31f9e574d70424bdb4436fe98. `G`,
  `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`,
  `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`,
  `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`,
  `m_n`, `m_n_u`, and `m_n_c2` hashes and nodes unchanged.

- **CODATA 2018 neutron mass energy equivalent is a one-sigma Interval.**
  `physis-constants` versions `m_n_c2` as the CODATA 2018 hull
  `1.50534976287(86)×10^{-10}` J from JPCRD 50, 033105 table XXXI
  (Neutron, n). This is not neutron mass `m_n`, not neutron mass in u
  `m_n_u`, not proton mass energy equivalent `m_p_c2`, not muon mass
  energy equivalent `m_mu_c2`, not Rydberg energy equivalent `hcRinf`,
  not Hartree `Eh`, not the exact electronvolt, not a certificate of a
  reconstruction from sibling masses, not an SI defining Ratio, not the
  Thomson cross section, and not P3N. The MeV conversion and molar mass
  are later table rows and are not stored. Neutron-tau is a PDG reprint
  (footnote e) and is not stored. Reduced Compton and gyromagnetic
  ratios cite ħ and are not stored. Electron mass is still not stored
  (`10^{42}` overflows `i128`). Decade `10^{21}` (`10^{20}` is the
  10× trap). `physis_model` `neutron_mass_energy_equivalent()` Qty
  locksteps to the recommended centre inside the hull. Adding `m_n_c2`
  to LEDGER changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`,
  `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`,
  `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`,
  `m_n`, and `m_n_u` hashes are unchanged. Theories still evaluate
  with `f64` Qty. That is not a kernel proof, not Canonical, not P4.
  Encode pins unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4. Verified: `m_n_c2` hash 6e677c893f10770fdedc46ec68d0c6de2321e9d5fd03819ca0bc0054308ebff4; node
  d11ab471f5ca712d524882673709646b0e6480bd019d01af2813441bb1389825; ledger node 9be08b8c6acc0d221d110c2516fe19e8413c5ca05d91119bc66b58c8c5f7e441. `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`,
  `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`,
  `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`,
  `m_n`, and `m_n_u` hashes and nodes unchanged.

- **CODATA 2018 neutron mass in u is a one-sigma Interval.**
  `physis-constants` versions `m_n_u` as the CODATA 2018 hull
  `1.00866491595(49)` u from JPCRD 50, 033105 table XXXI
  (Neutron, n). This is not neutron mass `m_n`, not proton mass
  in u `m_p_u`, not muon mass in u `m_mu_u`, not electron molar
  mass `M_e`, not proton-neutron mass ratio `mp_mn`, not a
  certificate of a reconstruction from sibling masses, not an SI
  defining Ratio, not the Thomson cross section, and not P3N. The
  joule and MeV energy equivalents and molar mass are later table
  rows and are not stored. Neutron-tau is a PDG reprint (footnote e)
  and is not stored. Reduced Compton and gyromagnetic ratios cite
  ħ and are not stored. Electron mass is still not stored
  (`10^{42}` overflows `i128`). Decade `10^{11}` (`10^{10}` is the
  10× trap). `physis_model` `neutron_mass_in_u()` Qty locksteps to
  the recommended centre inside the hull. Adding `m_n_u` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`,
  `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`,
  and `m_n` hashes are unchanged. Theories still evaluate with
  `f64` Qty. That is not a kernel proof, not Canonical, not P4.
  Encode pins unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4. Verified: `m_n_u` hash 7b6d3f11b99b03358a438ae921f035e5e3b581b543c06680df7420e64dfa7241; node
  97d71ba92203fee5acad8633da5489a51a8339cf70a47379866690bce701e6e9; ledger node a73b32864f875b170e26f35cafc1b25acc24eaaf5c6ee1c24fae0e1acc25201b. `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`,
  `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`,
  `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`,
  and `m_n` hashes and nodes unchanged.

- **CODATA 2018 neutron mass is a one-sigma Interval.**
  `physis-constants` versions `m_n` as the CODATA 2018 hull
  `1.67492749804(95)×10^{-27}` kg from JPCRD 50, 033105 table XXXI
  (Neutron, n). This is not proton mass `m_p`, not muon mass `m_mu`,
  not electron-neutron mass ratio `me_mn`, not muon-neutron mass
  ratio `mmu_mn`, not proton-neutron mass ratio `mp_mn`, not proton
  magnetic shielding correction `sigma0p`, not a certificate of a
  reconstruction from sibling masses, not an SI defining Ratio, not
  the Thomson cross section, and not P3N. The u-row, energy
  equivalents, and molar mass are later table rows and are not stored.
  Neutron-tau is a PDG reprint (footnote e) and is not stored. Reduced
  Compton and gyromagnetic ratios cite ħ and are not stored. Electron
  mass is still not stored (`10^{42}` overflows `i128`). Decade
  `10^{38}` (`10^{37}` is the 10× trap; `10^{39}` overflows `i128`).
  `physis_model` `neutron_mass()` Qty locksteps to the recommended
  centre inside the hull. Adding `m_n` to LEDGER changes the ledger
  bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`,
  `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`,
  `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`,
  `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`,
  `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`,
  `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`,
  `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`,
  `mu0p_muN`, and `sigma0p` hashes are unchanged. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged.
  P3N count stays 4. Verified: `m_n` hash a03f5c1c27081122d30ec7656df798ad4ad99413bb03790a40b3da58b76d1a0f; node
  226446ffad910bf91f0ec6f6a4ea8861e515ea82b319cbf2e077995edc051147; ledger node 3253ba98c1ab6f84adfd5006905a513280797fc274e30dc8e14ed2dc1f6f192e. `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`,
  `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`,
  `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, and `sigma0p`
  hashes and nodes unchanged.

- **CODATA 2018 proton magnetic shielding correction is a one-sigma Interval.**
  `physis-constants` versions `sigma0p` as the CODATA 2018 hull
  `2.5689(11)×10^{-5}` from JPCRD 50, 033105 table XXXI
  (Proton, p; `1 − μ′_p/μ_p` in spherical H2O, 25 °C). This is not
  shielded proton magnetic moment `mu0p`, not free proton magnetic
  moment `mu_p`, not vacuum permeability `mu0`, not a certificate of
  the reconstruction `1 − μ′_p/μ_p`, not an SI defining Ratio, not the
  Thomson cross section, and not P3N. Gyromagnetic ratios cite ħ and
  are not stored. Shielded g-factor `g0p` is a glossary identity, not
  a table XXXI recommended hull, and is not stored. The proton-tau
  ratio is a PDG reprint (footnote e) and is not stored. Electron
  mass is still not stored (`10^{42}` overflows `i128`).
  `physis_model`
  `proton_magnetic_shielding_correction()` Qty locksteps to the
  recommended centre inside the hull. Adding `sigma0p` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`,
  `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, and `mu0p_muN` hashes are
  unchanged. Theories still evaluate with `f64` Qty. That is not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified:
  `sigma0p` hash 2bd71c8a0c870e0f41ca7ec9ceada123c78583e8af24c97e5ad1918069bf1bd1; node 5750e14f1d96e1d0a29d15f26fab1bc146ff8fbcc345daaea980d499198050b3; ledger
  node 9d2f1cfac1c59106c9987fd1d06f37b1f3f8fc68957909ca7d5c7be2fa62f913. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`,
  `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`,
  `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, and `mu0p_muN` hashes and
  nodes unchanged.

- **CODATA 2018 shielded proton nuclear-magneton ratio is a one-sigma Interval.**
  `physis-constants` versions `mu0p_muN` as the CODATA 2018 hull
  `2.792775599(30)` from JPCRD 50, 033105 table XXXI
  (Proton, p; spherical H2O, 25 °C). This is not free proton
  nuclear-magneton ratio `mu_p_muN`, not electron nuclear-magneton
  ratio `mu_e_muN`, not muon nuclear-magneton ratio `mu_mu_muN`,
  not shielded proton Bohr-magneton ratio `mu0p_muB`, not shielded
  proton magnetic moment `mu0p`, not proton g-factor `gp`, not a
  certificate that this equals `gp/2` or a reconstructed `μ′p/μN`,
  not vacuum permeability `mu0`, not an SI defining Ratio, not the
  Thomson cross section, and not P3N. The shielding correction and
  shielded g-factor are later table rows and are not stored.
  Gyromagnetic ratios cite ħ and are not stored. The proton-tau
  ratio is a PDG reprint (footnote e) and is not stored. Electron
  mass is still not stored (`10^{42}` overflows `i128`).
  `physis_model`
  `shielded_proton_magnetic_moment_to_nuclear_magneton()` Qty
  locksteps to the recommended centre inside the hull. Adding
  `mu0p_muN` to LEDGER changes the ledger bundle pin. The `G`,
  `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`,
  `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`,
  `gp`, `mu_p_mun`, `mu0p`, and `mu0p_muB` hashes are unchanged.
  Theories still evaluate with `f64` Qty. That is not a kernel
  proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified:
  `mu0p_muN` hash 9f7913d246532a470d0c2dfe8ccfc7613aaafedaf22f42f0e5eb72d46fdfb9ed; node 842df9c30e0e80c22044f56801b4c741110707db286770da4a3f2d3ff25bf24a; ledger
  node 6fd8d269aea910a1b8a68e6e5aa0d7bd0612149c0d1563cbc302e8069c237a84. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`,
  `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`,
  `gp`, `mu_p_mun`, `mu0p`, and `mu0p_muB` hashes and nodes
  unchanged.

- **CODATA 2018 shielded proton Bohr-magneton ratio is a one-sigma Interval.**
  `physis-constants` versions `mu0p_muB` as the CODATA 2018 hull
  `1.520993128(17)×10^{-3}` from JPCRD 50, 033105 table XXXI
  (Proton, p; spherical H2O, 25 °C). This is not free proton
  Bohr-magneton ratio `mu_p_muB`, not electron Bohr-magneton ratio
  `mu_e_muB`, not muon Bohr-magneton ratio `mu_mu_muB`, not shielded
  proton magnetic moment `mu0p`, not the electron to shielded-proton
  moment ratio `mu_e_mu0p`, not vacuum permeability `mu0`, not an SI
  defining Ratio, not the Thomson cross section, and not P3N. The
  shielded nuclear-magneton ratio is a later table row and is not
  stored. The shielding correction and shielded g-factor are later
  table rows and are not stored. Gyromagnetic ratios cite ħ and are
  not stored. The proton-tau ratio is a PDG reprint (footnote e) and
  is not stored. Electron mass is still not stored (`10^{42}` overflows
  `i128`). `physis_model`
  `shielded_proton_magnetic_moment_to_bohr_magneton()` Qty locksteps to
  the recommended centre inside the hull. Adding `mu0p_muB` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`,
  `gp`, `mu_p_mun`, and `mu0p` hashes are unchanged. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not
  Canonical, not P4. Encode pins unchanged. Unique-vacuum graph
  id unchanged. P3N count stays 4. Verified: `mu0p_muB` hash
  6ff346294d2cdeefe14d9611bc3db150e51438fb3a864406768858faee733791; node 6204531acecb1bcba0d4e881586e346d94f771bf66c19c8c43148c5de99cbbb6; ledger node
  cbe031670c297b0d1cc8279822eac34911a544e2e9d8541baef707d09b90304f. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`,
  `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`,
  `gp`, `mu_p_mun`, and `mu0p` hashes and nodes unchanged.

- **CODATA 2018 shielded proton magnetic moment is a one-sigma Interval.**
  `physis-constants` versions `mu0p` as the CODATA 2018 hull
  `1.410570560(15)×10^{-26}` J T⁻¹ from JPCRD 50, 033105 table XXXI
  (Proton, p; spherical H2O, 25 °C). This is not free proton
  magnetic moment `mu_p`, not electron magnetic moment `mu_e`, not
  muon magnetic moment `mu_mu`, not the electron to shielded-proton
  moment ratio `mu_e_mu0p`, not vacuum permeability `mu0`, not an SI
  defining Ratio, not the Thomson cross section, and not P3N. The
  shielded Bohr-magneton and nuclear-magneton ratios are later table
  rows and are not stored. The shielding correction and shielded
  g-factor are later table rows and are not stored. Gyromagnetic
  ratios cite ħ and are not stored. The proton-tau ratio is a PDG
  reprint (footnote e) and is not stored. Electron mass is still not
  stored (`10^{42}` overflows `i128`). `physis_model`
  `shielded_proton_magnetic_moment()` Qty locksteps to the
  recommended centre inside the hull. Adding `mu0p` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`,
  `gp`, and `mu_p_mun` hashes are unchanged. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not
  Canonical, not P4. Encode pins unchanged. Unique-vacuum graph
  id unchanged. P3N count stays 4. Verified: `mu0p` hash
  e515a5f9475cddcdcb01f653117e63e25dbe9e3d63d7024a3b522ffe362679cb; node 06e2160c2ed44fd38fe9766274f2de71e2ec60d4e4c705763ba3446f07164739; ledger node
  04ae117b19ad76deb39574e4d2d61169453f4493dfee1b9e8ea245851224e4ae. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`,
  `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`,
  `gp`, and `mu_p_mun` hashes and nodes unchanged.

- **CODATA 2018 proton-neutron magnetic-moment ratio is a one-sigma Interval.**
  `physis-constants` versions `mu_p_mun` as the CODATA 2018 hull
  `−1.45989805(34)` from JPCRD 50, 033105 table XXXI
  (Proton, p). This is not electron-neutron magnetic-moment ratio
  `mu_e_mun`, not proton-neutron mass ratio `mp_mn`, not proton
  g-factor `gp`, not proton magnetic moment `mu_p`, not a
  certificate that this equals a reconstructed `μp/μn` from
  sibling moments, not vacuum permeability `mu0`, not an SI
  defining Ratio, not the Thomson cross section, and not P3N.
  The neutron-proton magnetic-moment ratio is a later Neutron-
  section row and is not stored. The shielded proton moment is a
  later table row and is not stored. Gyromagnetic ratios cite ħ
  and are not stored. The proton-tau ratio is a PDG reprint
  (footnote e) and is not stored. Electron mass is still not
  stored (`10^{42}` overflows `i128`). `physis_model`
  `proton_neutron_magnetic_moment_ratio()` Qty locksteps to the
  recommended centre inside the hull. Adding `mu_p_mun` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, and
  `gp` hashes are unchanged. Theories still evaluate with `f64`
  Qty. That is not a kernel proof, not Canonical, not P4. Encode
  pins unchanged. Unique-vacuum graph id unchanged. P3N count
  stays 4. Verified: `mu_p_mun` hash 962ad1f4f5a18ff54eb9bb0acff22f5fa8b96537291568c387b82f8bd41abd98; node
  865524beff711d79dbcb0c655d6494882c2a63f8988d1476a15c03d34daa29ce; ledger node 923f11376ba6c2bd5a5516e491653cde95f5d8589e365b713aed1fcf8ec4a3fc. `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`,
  `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`,
  `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`,
  `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`,
  `mu_p_muN`, and `gp` hashes and nodes unchanged.

- **CODATA 2018 proton g-factor is a one-sigma Interval.**
  `physis-constants` versions `gp` as the CODATA 2018 hull
  `5.5856946893(16)` from JPCRD 50, 033105 table XXXI
  (Proton, p). This is not electron g-factor `ge`, not muon
  g-factor `gmu`, not proton nuclear-magneton ratio `mu_p_muN`,
  not a certificate that this equals `2 μp/μN`, not vacuum
  permeability `mu0`, not an SI defining Ratio, not the Thomson
  cross section, and not P3N. The shielded proton g-factor is a
  later table row and is not stored. Gyromagnetic ratios cite ħ
  and are not stored. The proton-tau ratio is a PDG reprint
  (footnote e) and is not stored. Electron mass is still not
  stored (`10^{42}` overflows `i128`). `physis_model`
  `proton_g_factor()` Qty locksteps to the recommended centre
  inside the hull. Adding `gp` to LEDGER changes the ledger
  bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`,
  `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, and `mu_p_muN`
  hashes are unchanged. Theories still evaluate with `f64` Qty.
  That is not a kernel proof, not Canonical, not P4. Encode pins
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `gp` hash 9a1a482bd1adcc3258834dd9275ce119d29903b398307609f17788e5f4a6874d; node ae15d6d01a97e5ff6a9e8fdab3c5e3e3e262e41aab803a85ee3a8f8f7e471169; ledger node
  73f7d21dc88f326f9ecc0c40d4eb17b74e2426a684df07065f1bc73b721d5082. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`,
  `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, and `mu_p_muN`
  hashes and nodes unchanged.

- **CODATA 2018 proton magnetic moment to nuclear magneton ratio is a one-sigma Interval.**
  `physis-constants` versions `mu_p_muN` as the CODATA 2018 hull
  `2.79284734463(82)` from JPCRD 50, 033105 table XXXI
  (Proton, p). This is not electron nuclear-magneton ratio
  `mu_e_muN`, not muon nuclear-magneton ratio `mu_mu_muN`, not
  proton Bohr-magneton ratio `mu_p_muB`, not proton magnetic
  moment `mu_p`, not vacuum permeability `mu0`, not an SI
  defining Ratio, not the Thomson cross section, and not P3N.
  The shielded proton ratio is a later table row and is not
  stored. Gyromagnetic ratios cite ħ and are not stored. The
  proton g-factor is a later table row and is not stored here.
  The proton-tau ratio is a PDG reprint (footnote e) and is not
  stored. Electron mass is still not stored (`10^{42}` overflows
  `i128`). `physis_model`
  `proton_magnetic_moment_to_nuclear_magneton()` Qty locksteps to
  the recommended centre inside the hull. Adding `mu_p_muN` to
  LEDGER changes the ledger bundle pin. The `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`,
  `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`,
  `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`,
  `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, and `mu_p_muB`
  hashes are unchanged. Theories still evaluate with `f64` Qty.
  That is not a kernel proof, not Canonical, not P4. Encode pins
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `mu_p_muN` hash 3d1337d5d8845bcc477feee0bef86a648b907c20087669165db91902a1f14fd8; node
  4a808f05f3bd65df18072a9c679ee49155dd2f621824f039a0b83990ae14cc65; ledger node f89cd95a44c0af11d7cddd172917764aa0044185c104be22e144c1497643627d. `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`,
  `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`,
  `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`,
  `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`,
  `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, and `mu_p_muB`
  hashes and nodes unchanged.

- **CODATA 2018 proton magnetic moment to Bohr magneton ratio is a one-sigma Interval.**
  `physis-constants` versions `mu_p_muB` as the CODATA 2018 hull
  `1.52103220230(46)×10^{-3}` from JPCRD 50, 033105 table XXXI
  (Proton, p). This is not electron Bohr-magneton ratio `mu_e_muB`,
  not muon Bohr-magneton ratio `mu_mu_muB`, not proton magnetic
  moment `mu_p`, not vacuum permeability `mu0`, not an SI defining
  Ratio, not the Thomson cross section, and not P3N. The shielded
  proton ratio is a later table row and is not stored. Gyromagnetic
  ratios cite ħ and are not stored. The proton-tau ratio is a PDG
  reprint (footnote e) and is not stored. Electron mass is still not
  stored (`10^{42}` overflows `i128`). `physis_model`
  `proton_magnetic_moment_to_bohr_magneton()` Qty locksteps to the
  recommended centre inside the hull. Adding `mu_p_muB` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`, `rp`, and `mu_p`
  hashes are unchanged. Theories still evaluate with `f64` Qty. That
  is not a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified:
  `mu_p_muB` hash cadc896f8c2b6f960aa051bb05a70efb8b2e58bc36b7230787148191227cff3a; node c5a6302bb2a038bac321db0f3ab362024dfed4b37cf3bc42c1a2850ec255af2a; ledger node
  f5341a8fe145b02b6223ddb264339272fd3f9ee7f83782e90ca39ba7b6ec560c. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`,
  `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`, `rp`, and `mu_p`
  hashes and nodes unchanged.

- **CODATA 2018 proton magnetic moment is a one-sigma Interval.**
  `physis-constants` versions `mu_p` as the CODATA 2018 hull
  `1.41060679736(60)×10^{-26}` J T⁻¹ from JPCRD 50, 033105 table XXXI
  (Proton, p). This is not electron magnetic moment `mu_e`, not muon
  magnetic moment `mu_mu`, not the electron-proton moment ratio
  `mu_e_mup`, not vacuum permeability `mu0`, not an SI defining
  Ratio, not the Thomson cross section, and not P3N. The shielded
  proton moment is a later table row and is not stored. Gyromagnetic
  ratios cite ħ and are not stored. The proton-tau ratio is a PDG
  reprint (footnote e) and is not stored. Electron mass is still not
  stored (`10^{42}` overflows `i128`). `physis_model`
  `proton_magnetic_moment()` Qty locksteps to the recommended centre
  inside the hull. Adding `mu_p` to LEDGER changes the ledger bundle
  pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`,
  `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`,
  `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`,
  `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`,
  `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`,
  `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`, and `rp` hashes
  are unchanged. Theories still evaluate with `f64` Qty. That is
  not a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified:
  `mu_p` hash bf987c5fccc4ef40691f126024092320bc335b7942323fa90675a28a250e304c; node 05785ed691b20e86ee5246df2dad1d6a0f90fe6bb85bd35f06c8561a6194622e; ledger node
  443ed80dd599e08c877a0efaa190899b4da49e7119d501b9e0216ecf9a6b9028. `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`,
  `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`, and `rp` hashes
  and nodes unchanged.

- **CODATA 2018 proton rms charge radius is a one-sigma Interval.**
  `physis-constants` versions `rp` as the CODATA 2018 hull
  `8.414(19)×10^{-16}` m from JPCRD 50, 033105 table XXXI
  (Proton, p). This is not classical electron radius `re`, not
  electron Compton, not a certificate of the deuteron rms radius,
  not an SI defining Ratio, not the Thomson cross section, and not
  P3N. The reduced proton Compton wavelength remains unstored
  (`ħ`). The proton-tau ratio is a PDG reprint (footnote e) and is
  not stored. Electron mass is still not stored (`10^{42}`
  overflows `i128`). `physis_model` `proton_rms_charge_radius()` Qty
  locksteps to the recommended centre inside the hull. Adding `rp`
  to LEDGER changes the ledger bundle pin. The `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`,
  `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`,
  `M_p`, and `lambda_C_p` hashes are unchanged. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not
  Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified: `rp` hash 722e45d219142c882b4475408333d68c48c5952af08680c0a16bed5af1bfc944;
  node 1cae30455d7ad0da2a7cee5aea88b53749b93936a1d209c3e4bb514f1d9c3058; ledger node 26a30d95e3b9db4a83335f23999d2da6f24f20a49099362283a02174c3774494. `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, and `lambda_C_p` hashes and
  nodes unchanged.

- **CODATA 2018 proton Compton wavelength is a one-sigma Interval.**
  `physis-constants` versions `lambda_C_p` as the CODATA 2018 hull
  `1.32140985539(40)×10^{-15}` m from JPCRD 50, 033105 table XXXI
  (Proton, p). This is not electron Compton `lambda_C`, not muon
  Compton `lambda_C_mu`, not a certificate that this equals
  `h/(m_p c)` or `2π` times a reduced proton Compton wavelength,
  not an SI defining Ratio, not the Thomson cross section, and not
  P3N. The reduced proton Compton wavelength remains unstored
  (`ħ`). The proton-tau ratio is a PDG reprint (footnote e) and is
  not stored. Electron mass is still not stored (`10^{42}`
  overflows `i128`). `physis_model` `proton_compton_wavelength()` Qty
  locksteps to the recommended centre inside the hull. Adding
  `lambda_C_p` to LEDGER changes the ledger bundle pin. The `G`,
  `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`,
  `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, and
  `M_p` hashes are unchanged. Theories still evaluate with `f64`
  Qty. That is not a kernel proof, not Canonical, not P4. Encode
  pins unchanged. Unique-vacuum graph id unchanged. P3N count
  stays 4. Verified: `lambda_C_p` hash 439c5267d7664a8bd5c359b40f5a2291b0b364e0a915c57378abd6787b9d5a08; node
  7c05dfef72a29e8b2ede826332febcd0331436b3be2f6b97f83f2489d1ee6840; ledger node c31bcbe5fdf004c47560efe2c94d72980d51a2cceb1100fe7296ee2a00ed92f6. `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, `mp_mn`, `e_mp`, and `M_p` hashes and nodes unchanged.

- **CODATA 2018 proton molar mass is a one-sigma Interval.**
  `physis-constants` versions `M_p` as the CODATA 2018 hull
  `1.00727646627(31)×10^{-3}` kg mol⁻¹ from JPCRD 50, 033105 table XXXI
  (Proton, p). This is not electron molar mass `M_e`, not muon molar
  mass `M_mu`, not the mass-in-u row `m_p_u`, not a certificate that
  this equals `N_A × m_p`, not an SI defining Ratio, not the Thomson
  cross section, and not P3N. The proton-tau ratio is a PDG reprint
  (footnote e) and is not stored. Reduced proton Compton remains
  unstored (`ħ`). Electron mass is still not stored (`10^{42}`
  overflows `i128`). `physis_model` `proton_molar_mass()` Qty
  locksteps to the recommended centre inside the hull. Adding `M_p`
  to LEDGER changes the ledger bundle pin. The `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, `mp_mn`, and `e_mp` hashes are unchanged. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged.
  P3N count stays 4. Verified: `M_p` hash `6ca2722d15970d11783522598ee8879e560019865477f1735041e1c9c8180149`; node
  `aa03e8a73d0ec0d8805ac00559d7c6db336f50fc102dc2d7f295adadfb291162`; ledger node `d3c8cd6564da09c410dcacbf53b72888c28c5d14e29e3fc401ea9a4d657a3c74`. `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, `mp_mn`, and `e_mp` hashes and nodes unchanged.

- **CODATA 2018 proton charge-to-mass quotient is a one-sigma Interval.**
  `physis-constants` versions `e_mp` as the CODATA 2018 hull
  `9.5788331560(29)×10^7` C kg⁻¹ from JPCRD 50, 033105 table XXXI
  (Proton, p). This is not the electron quotient `e_me`, not a
  certificate that this equals `e/m_p` from the SI-exact charge and
  the proton-mass hull, not an SI defining Ratio, not the Thomson
  cross section, and not P3N. The proton-tau ratio is a PDG reprint
  (footnote e) and is not stored. Reduced proton Compton remains
  unstored (`ħ`). Electron mass is still not stored (`10^{42}`
  overflows `i128`). `physis_model` `proton_charge_to_mass()` Qty
  locksteps to the recommended centre inside the hull. Adding `e_mp`
  to LEDGER changes the ledger bundle pin. The `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, and `mp_mn` hashes are unchanged. Theories still evaluate
  with `f64` Qty. That is not a kernel proof, not Canonical, not P4.
  Encode pins unchanged. Unique-vacuum graph id unchanged. P3N count
  stays 4. Verified: `e_mp` hash `e0404fef33540ecdef40b36cb1e14112b9e91713cccab65bb6a920fa44106611`; node
  `297019b19da4904272c51a8e04b72e02d384fc15dd567e0d888d9edbc3ac5097`; ledger node `3aefb12abc2eab15f87d8126f17e1bd25f343713b5e7d4337e75ab1dd907f9b9`. `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`,
  `mp_mmu`, and `mp_mn` hashes and nodes unchanged.

- **CODATA 2018 proton-neutron mass ratio is a one-sigma Interval.**
  `physis-constants` versions `mp_mn` as the CODATA 2018 hull
  `0.99862347812(49)` from JPCRD 50, 033105 table XXXI (Proton, p).
  This is not the muon-neutron ratio `mmu_mn`, not electron-neutron
  `me_mn`, not a certificate that the stored centres divide, not an SI
  defining Ratio, not the Thomson cross section, and not P3N. The
  proton-tau ratio is a PDG reprint (footnote e) and is not stored.
  Reduced proton Compton remains unstored (`ħ`). Electron mass is still
  not stored (`10^{42}` overflows `i128`). `physis_model`
  `proton_neutron_mass_ratio()` Qty locksteps to the recommended
  centre inside the hull. Adding `mp_mn` to LEDGER changes the ledger
  bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`,
  `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`,
  `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`,
  `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`,
  `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`,
  `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`,
  `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, `m_p_c2_MeV`, `mp_me`, and `mp_mmu` hashes are unchanged.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified: `mp_mn` hash `fd6d15f0f9cd99a1889486d78f316d0e3299c3b9ff8db13d6429bcb47cccb465`; node
  `569fded02422616274c374100f8e423cedece58684b6bd0521c559301a74577e`; ledger node `f8d4769794c674fcffd490fdc3b04d4ff852925e9967c5413973e82bf8359834`. `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, and
  `mp_mmu` hashes and nodes unchanged.

- **CODATA 2018 proton-muon mass ratio is a one-sigma Interval.**
  `physis-constants` versions `mp_mmu` as the CODATA 2018 hull
  `8.88024337(20)` from JPCRD 50, 033105 table XXXI (Proton, p).
  This is not the muon-proton ratio `mmu_mp`, not a certificate that
  the stored centres invert, not proton-electron `mp_me`, not an SI
  defining Ratio, not the Thomson cross section, and not P3N. The
  proton-tau ratio is a PDG reprint (footnote e) and is not stored.
  Reduced proton Compton remains unstored (`ħ`). Electron mass is still
  not stored (`10^{42}` overflows `i128`). `physis_model`
  `proton_muon_mass_ratio()` Qty locksteps to the recommended
  centre inside the hull. Adding `mp_mmu` to LEDGER changes the ledger
  bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`,
  `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`,
  `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`,
  `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`,
  `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`,
  `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`,
  `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, `m_p_c2_MeV`, and `mp_me` hashes are unchanged. Theories
  still evaluate with `f64` Qty. That is not a kernel proof, not
  Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified: `mp_mmu` hash `f94b9bc09239d4e20d3f7e4b9f07f39cc5f820c334fe269635d2f80c0dd88bc3`; node
  `25ce5aab6c87db67ab59a27319ed6dc0e5958ade65afad70a977c9fa43049634`; ledger node `c1f098de34b6262323f9febd878f3cf5c5ff1db3c2db9722e51583873a2ae774`. `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, and `mp_me`
  hashes and nodes unchanged.

- **CODATA 2018 proton-electron mass ratio is a one-sigma Interval.**
  `physis-constants` versions `mp_me` as the CODATA 2018 hull
  `1836.15267343(11)` from JPCRD 50, 033105 table XXXI (Proton, p).
  This is not the electron-proton ratio `me_mp`, not a certificate that
  the stored centres invert, not muon-electron `mmu_me`, not an SI
  defining Ratio, not the Thomson cross section, and not P3N. The
  proton-tau ratio is a PDG reprint (footnote e) and is not stored.
  Reduced proton Compton remains unstored (`ħ`). Electron mass is still
  not stored (`10^{42}` overflows `i128`). `physis_model`
  `proton_electron_mass_ratio()` Qty locksteps to the recommended
  centre inside the hull. Adding `mp_me` to LEDGER changes the ledger
  bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`,
  `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`,
  `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`,
  `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`,
  `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`,
  `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`,
  `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`,
  `m_p_c2`, and `m_p_c2_MeV` hashes are unchanged. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4. Verified: `mp_me` hash `8b8047d3ebeca5e157da8e85248892e5731d87114960e63335d74a49631f27b6`; node
  `b26d54036d2d6b10bf50cda5ef46746fba7a7a0edeb71569428e895ee0a81dc0`; ledger node `6965348c0aadf4196452389d29af9ebd19f8db028e1355903dc48319b9376703`. `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, and `m_p_c2_MeV` hashes and
  nodes unchanged.

- **CODATA 2018 proton mass energy equivalent in MeV is a one-sigma
  Interval.**
  `physis-constants` versions `m_p_c2_MeV` as the CODATA 2018 hull
  `938.27208816(29)` MeV from JPCRD 50, 033105 table XXXI (Proton, p).
  This is not the joule hull `m_p_c2`, not muon MeV `m_mu_c2_MeV`, not
  the exact electronvolt Ratio, not Hartree `Eh`, not Rydberg energy
  equivalent `hcRinf`, not an SI defining Ratio, not the Thomson cross
  section, and not P3N. Reduced proton Compton remains unstored (`ħ`).
  Electron mass is still not stored (`10^{42}` overflows `i128`).
  `physis_model` `proton_mass_energy_equivalent_in_mev()` Qty locksteps
  to the recommended centre inside the hull. Adding `m_p_c2_MeV` to
  LEDGER changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`,
  `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`,
  `m_p_u`, and `m_p_c2` hashes are unchanged. Theories still evaluate
  with `f64` Qty. That is not a kernel proof, not Canonical, not P4.
  Encode pins unchanged. Unique-vacuum graph id unchanged. P3N count
  stays 4. Verified: `m_p_c2_MeV` hash `fe91682af8608f3a6117790109cc0cbb09c709fb7cc1a778d6c6be39efea1c5e`; node
  `c44790b16220afd067ff78b6174045801f94d8412541b829412bccf223891629`; ledger node `1495a6849044b17c1a4eb72cd5b73b53ca66891a3fae30e26226c219c1bc6791`. `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, `m_p_u`, and `m_p_c2` hashes and nodes unchanged.

- **CODATA 2018 proton mass energy equivalent is a one-sigma Interval.**
  `physis-constants` versions `m_p_c2` as the CODATA 2018 hull
  `1.50327761598(46)×10^{-10}` J from JPCRD 50, 033105 table XXXI
  (Proton, p). This is not the kg hull `m_p`, not the u-row `m_p_u`,
  not muon mass energy equivalent `m_mu_c2`, not Rydberg energy
  equivalent `hcRinf`, not Hartree `Eh`, not the exact electronvolt
  Ratio, not an SI defining Ratio, not the Thomson cross section, and
  not P3N. Reduced proton Compton remains unstored (`ħ`). Electron mass
  is still not stored (`10^{42}` overflows `i128`). `physis_model`
  `proton_mass_energy_equivalent()` Qty locksteps to the recommended
  centre inside the hull. Adding `m_p_c2` to LEDGER changes the ledger
  bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`,
  `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`,
  `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`,
  `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`,
  `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`,
  `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`,
  `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`,
  `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, and
  `m_p_u` hashes are unchanged. Theories still evaluate with `f64`
  Qty. That is not a kernel proof, not Canonical, not P4. Encode pins
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `m_p_c2` hash `9bdab3205a64c45d2c413626db03cdf6452e021df65c420170cd7ffb163990f3`; node
  `e242f8c1e541d797801325311916546d420c7a71b42303c94a0a7d060b8adde6`; ledger node `866cf73006cd75e91e3539b31b45164d72de58ed303d1fbbeff0b17cf9370e17`. `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, `m_p`, and `m_p_u` hashes and nodes unchanged.

- **CODATA 2018 proton mass in u is a one-sigma Interval.**
  `physis-constants` versions `m_p_u` as the CODATA 2018 hull
  `1.007276466621(53)` u from JPCRD 50, 033105 table XXXI (Proton, p).
  This is not the kg hull `m_p`, not muon mass in u `m_mu_u`, not
  electron molar mass `M_e`, not relative atomic mass under a different
  name, not an SI defining Ratio, not the Thomson cross section, and
  not P3N. Reduced proton Compton remains unstored (`ħ`). Electron mass
  is still not stored (`10^{42}` overflows `i128`). `physis_model`
  `proton_mass_in_u()` Qty locksteps to the recommended centre inside
  the hull. Adding `m_p_u` to LEDGER changes the ledger bundle pin. The
  `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`,
  `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`,
  `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`,
  `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`,
  `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, and `m_p` hashes are
  unchanged. Theories still evaluate with `f64` Qty. That is not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `m_p_u` hash `244a086710c746078b5de6d5f2c5f896dd01a8469448035eadbc63c49fff6435`; node
  `b7d7c12458d2829801236ee857190822650a55625bc19c759e683874869f9b6c`; ledger node `467a7580a876f18145913a873c4b355e53552a694c7469434e9f920f50a82d7f`. `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`,
  `mu_mu_mup`, and `m_p` hashes and nodes unchanged.

- **CODATA 2018 muon-proton magnetic-moment ratio is a one-sigma
  Interval.**
  `physis-constants` versions `mu_mu_mup` as the CODATA 2018 hull
  `−3.183345142(71)` from JPCRD 50, 033105 table XXXI (Muon, mu-).
  This is not electron-proton magnetic-moment ratio `mu_e_mup`, not
  muon-proton mass ratio `mmu_mp`, not muon g-factor `gmu`, not muon
  anomaly `amu`, not muon magnetic moment `mu_mu`, not an SI defining
  Ratio, not the Thomson cross section, and not P3N. Reduced muon
  Compton remains unstored (`ħ`). Electron mass is still not stored
  (`10^{42}` overflows `i128`). `physis_model`
  `muon_proton_magnetic_moment_ratio()` Qty locksteps to the
  recommended signed centre inside the hull. Adding `mu_mu_mup` to
  LEDGER changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`,
  `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, and `m_p` hashes
  are unchanged. Theories still evaluate with `f64` Qty. That is not
  a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `mu_mu_mup` hash `495228cd39ae86738938efec2d2639f9a0c198fdc4cb75ed4968321672b7df84`; node
  `6468f3033806b2fdcf78e2fbc7dfc7921b14f98b7a61b88127e4302c4d7fabff`; ledger node `e903796f74e02d5737f7ddb390e96a8d4d531e48b1c27e0818bdbde65cbe1931`. `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, and
  `m_p` hashes and nodes unchanged.

- **CODATA 2018 muon g-factor is a one-sigma Interval.**
  `physis-constants` versions `gmu` as the CODATA 2018 hull
  `−2.0023318418(13)` from JPCRD 50, 033105 table XXXI
  (Muon, mu-). This is not electron g-factor `ge`, not muon
  anomaly `amu`, not muon Bohr-magneton ratio `mu_mu_muB`, not
  muon magnetic moment `mu_mu`, not muon nuclear-magneton ratio
  `mu_mu_muN`, not an SI defining Ratio, not the Thomson cross
  section, and not P3N. Reduced muon Compton remains unstored (`ħ`).
  Electron mass is still not stored (`10^{42}` overflows `i128`).
  `physis_model` `muon_g_factor()` Qty locksteps to the recommended
  signed centre inside the hull. Adding `gmu` to LEDGER changes the
  ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`,
  `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, and `m_p` hashes are
  unchanged. Theories still evaluate with `f64` Qty. That is not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `gmu` hash `0a3447871c2dc78e3fa0c69d9134b7ee2852cbf55a0c570452a9118d9c747ded`; node
  `a810ed959ec82be1763f525714739871f182303850a02507d163f4e73fcc5996`; ledger node `4151f860aa94985ff9ee005181906e1ea8f8685f7c6e1440f153dc3d6d7e3d30`. `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, and `m_p`
  hashes and nodes unchanged.

- **CODATA 2018 muon magnetic-moment anomaly is a one-sigma Interval.**
  `physis-constants` versions `amu` as the CODATA 2018 hull
  `1.16592089(63)×10^{-3}` from JPCRD 50, 033105 table XXXI
  (Muon, mu-). This is not electron anomaly `ae`, not the muon
  g-factor, not muon Bohr-magneton ratio `mu_mu_muB`, not muon
  magnetic moment `mu_mu`, not muon nuclear-magneton ratio
  `mu_mu_muN`, not an SI defining Ratio, not the Thomson cross
  section, and not P3N. Reduced muon Compton remains unstored (`ħ`).
  Electron mass is still not stored (`10^{42}` overflows `i128`).
  `physis_model` `muon_magnetic_moment_anomaly()` Qty locksteps to
  the recommended centre inside the hull. Adding `amu` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, and `m_p` hashes are unchanged.
  Theories still evaluate with `f64` Qty. That is not a kernel
  proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `amu` hash `972c93982e6cd84f054db85605b9e7d106d124bd52bac104bb447d788cdc64c4`; node
  `8eeb79c20e0bd0766d4afb682a885a006a867378e21dced5d1b485c9a52b06ad`; ledger node `044a027898acd4fbe72cfb6f012d248e24f95be834da6c9f5598cabc268a52c1`. `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, and `m_p` hashes
  and nodes unchanged.

- **CODATA 2018 muon magnetic moment to nuclear magneton ratio is a
  one-sigma Interval.**
  `physis-constants` versions `mu_mu_muN` as the CODATA 2018 hull
  `−8.89059703(20)` from JPCRD 50, 033105 table XXXI (Muon, mu-).
  This is not electron nuclear-magneton ratio `mu_e_muN`, not muon
  Bohr-magneton ratio `mu_mu_muB`, not muon magnetic moment `mu_mu`,
  not the muon g-factor, not the muon anomaly, not an SI defining
  Ratio, not the Thomson cross section, and not P3N. Reduced muon
  Compton remains unstored (`ħ`). Electron mass is still not stored
  (`10^{42}` overflows `i128`). `physis_model`
  `muon_magnetic_moment_to_nuclear_magneton()` Qty locksteps to the
  recommended centre inside the hull. Adding `mu_mu_muN` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`,
  `mu_mu`, `mu_mu_muB`, and `m_p` hashes are unchanged. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged.
  P3N count stays 4.
  Verified: `mu_mu_muN` hash `52a97de9669b20480e5729de915cef56ba841da392185b3e5893b10c496ed16b`; node
  `a2ee12b91517cfe4a6ab8f310e7361b9adb4877f0f69643d9e7f31d983777854`; ledger node `7cd9697fcc08c7aae11f1a13cd1aea3faae3a2ee1bdb1d3e50050723f4f27641`. `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, and `m_p` hashes and nodes
  unchanged.

- **CODATA 2018 muon magnetic moment to Bohr magneton ratio is a
  one-sigma Interval.**
  `physis-constants` versions `mu_mu_muB` as the CODATA 2018 hull
  `−4.84197047(11)×10^{-3}` from JPCRD 50, 033105 table XXXI
  (Muon, mu-). This is not electron Bohr-magneton ratio `mu_e_muB`,
  not muon magnetic moment `mu_mu`, not the muon g-factor, not the
  muon anomaly, not an SI defining Ratio, not the Thomson cross
  section, and not P3N. Reduced muon Compton remains unstored (`ħ`).
  Electron mass is still not stored (`10^{42}` overflows `i128`).
  `physis_model` `muon_magnetic_moment_to_bohr_magneton()` Qty
  locksteps to the recommended centre inside the hull. Adding
  `mu_mu_muB` to LEDGER changes the ledger bundle pin. The `G`,
  `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`,
  `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`,
  `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`,
  `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`,
  `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, and `m_p` hashes are
  unchanged. Theories still evaluate with `f64` Qty. That is not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `mu_mu_muB` hash `5fa244938a528feff7867ea9ae972d76da59930a932f2a5ac9fe6ef52762c591`; node
  `598222a0188296e5619983efb3343cb0841a482f073854780655991ddbbc57e7`; ledger node `1918f0ff3384b460090f4606a61df6a3feaa5d6109f1ce4052a86b2b683bda90`. `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, `mu_mu`, and `m_p` hashes and nodes unchanged.

- **CODATA 2018 muon magnetic moment is a one-sigma Interval.**
  `physis-constants` versions `mu_mu` as the CODATA 2018 hull
  `−4.49044830(10)×10^{-26}` J T⁻¹ from JPCRD 50, 033105 table XXXI
  (Muon, mu-). This is not electron magnetic moment `mu_e`, not the
  electron-muon magnetic-moment ratio `mu_e_mmu`, not vacuum
  permeability `mu0`, not an SI defining Ratio, not the Thomson cross
  section, and not P3N. Reduced muon Compton remains unstored (`ħ`).
  Electron mass is still not stored (`10^{42}` overflows `i128`).
  `physis_model` `muon_magnetic_moment()` Qty locksteps to the
  recommended centre inside the hull. Adding `mu_mu` to LEDGER changes
  the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`,
  `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`,
  `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`,
  `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, and `m_p` hashes are
  unchanged. Theories still evaluate with `f64` Qty. That is not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `mu_mu` hash `3344549ca18b2db388cfff366cc63079f3d3b0b094cac6de12e318fe8531c3e0`; node
  `dbdd3710096e4ae7679dc5238012cec06c3ecc8242e3727978c5c15023b4abb8`; ledger node `4d4415b6f176ea2d6cb08dc62a38ab9f6dea218fc0c61fb5f61fcdba60482423`. `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`,
  `lambda_C_mu`, and `m_p` hashes and nodes unchanged.

- **CODATA 2018 muon Compton wavelength is a one-sigma Interval.**
  `physis-constants` versions `lambda_C_mu` as the CODATA 2018 hull
  `1.173444110(26)×10^{-14}` m from JPCRD 50, 033105 table XXXI
  (Muon, mu-). This is not electron Compton `lambda_C`, not a
  certificate of `2π` times a reduced muon Compton wavelength, not
  muon molar mass, not an SI defining Ratio, not the Thomson cross
  section, and not P3N. The reduced muon Compton wavelength is
  `ħ/m_μc` and is not stored. Electron mass is still not stored
  (`10^{42}` overflows `i128`). `physis_model`
  `muon_compton_wavelength()` Qty locksteps to the recommended centre
  inside the hull. Adding `lambda_C_mu` to LEDGER changes the ledger
  bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`,
  `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`,
  `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`,
  `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, `M_mu`, and `m_p` hashes are unchanged. Theories
  still evaluate with `f64` Qty. That is not a kernel proof, not
  Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4.
  Verified: `lambda_C_mu` hash `6fb48517f2b436bf1ede156c0dd4505692db4e7afe3e5d6f7ed2bfbfdc4198d9`; node
  `7927f45a9a1a6944fde9bc270c82a4a5014224d39fbfbf9802d3b5505556d4ec`; ledger node `167053ce3db9e915879a27387a0aba1c55d463ce3a5cad94db7651f4860cf1e6`. `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, and
  `m_p` hashes and nodes unchanged.

- **CODATA 2018 muon molar mass is a one-sigma Interval.**
  `physis-constants` versions `M_mu` as the CODATA 2018 hull
  `1.134289259(25)×10^{-4}` kg mol⁻¹ from JPCRD 50, 033105 table XXXI
  (Muon, mu-). This is not the muon mass in u `m_mu_u` (same mantissa,
  different unit and scale), not `N_A × m_mu` as a derived product, not
  electron molar mass `M_e`, not an SI defining Ratio, not the Thomson
  cross section, and not P3N. The muon-tau ratio is a PDG reprint and is
  not stored. Electron mass is still not stored (`10^{42}` overflows
  `i128`). `physis_model` `muon_molar_mass()` Qty locksteps to the
  recommended centre inside the hull. Adding `M_mu` to LEDGER changes
  the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`,
  `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`,
  `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`,
  `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`,
  `mmu_mp`, `mmu_mn`, and `m_p` hashes are unchanged. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4.
  Verified: `M_mu` hash `b53efc5e339708317e98c92c02ae506bf5b90c6d847e586d716d1631d902c81a`; node
  `4fb4263b7231f6a7d4fe1a73eba8b05adeaf94a12ccd7caccc4eb2ff4d0c6a07`; ledger node `4713fbb3f6a23f371864260e81c9183a7eb3fb6ea9a2fe60e188d23b31436d39`. `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, and `m_p`
  hashes and nodes unchanged.

- **CODATA 2018 muon-neutron mass ratio is a one-sigma Interval.**
  `physis-constants` versions `mmu_mn` as the CODATA 2018 hull
  `0.1124545170(25)` from JPCRD 50, 033105 table XXXI (Muon, mu-).
  This is not the electron-neutron ratio `me_mn`, not the muon-proton
  ratio `mmu_mp`, not a certificate that the stored centres divide, not
  the proton mass, not an SI defining Ratio, not the Thomson cross
  section, and not P3N. The muon-tau ratio is a PDG reprint of
  `m_tau c^2` (JPCRD table XXXI footnote e) and is not stored.
  Electron mass is still not stored (`10^{42}` overflows `i128`).
  `physis_model` `muon_neutron_mass_ratio()` Qty locksteps to the
  recommended centre inside the hull. Adding `mmu_mn` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, and `m_p` hashes are unchanged.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4.
  Verified: `mmu_mn` hash `f8a9dfb53e84c4a592143e9d17e9e04884b69cc9b2b378dc2a7c099c4d442835`; node
  `4f08dabdf90b0a433f93c5cd653e9d3c5f2f7b02145130049391199780b02e63`; ledger node `76b638a021a7aa33419333341bade653684a82695b1bddf3878d49fb42021ac0`. `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, and `m_p` hashes and
  nodes unchanged.

- **CODATA 2018 muon-proton mass ratio is a one-sigma Interval.**
  `physis-constants` versions `mmu_mp` as the CODATA 2018 hull
  `0.1126095264(25)` from JPCRD 50, 033105 table XXXI (Muon, mu-).
  This is not the electron-proton ratio `me_mp`, not the muon-electron
  ratio `mmu_me`, not a certificate that the stored centres divide, not
  the proton mass, not an SI defining Ratio, not the Thomson cross
  section, and not P3N. The muon-tau ratio is a PDG reprint of
  `m_tau c^2` (JPCRD table XXXI footnote e) and is not stored.
  Electron mass is still not stored (`10^{42}` overflows `i128`).
  `physis_model` `muon_proton_mass_ratio()` Qty locksteps to the
  recommended centre inside the hull. Adding `mmu_mp` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  `m_mu_c2_MeV`, `mmu_me`, and `m_p` hashes are unchanged. Theories
  still evaluate with `f64` Qty. That is not a kernel proof, not
  Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4.
  Verified: `mmu_mp` hash `1527aa21236682ad99206cf1ef6b6267d7432a5a1975bcc2315af9a510e147d2`; node
  `71390580bc3deae9e30e53f10b832a03e64680879ce3f0f076695e3f41024915`; ledger node `33962066edede73526b23ddaab8cb39498878522b2e2e4200de23c226e4c6d29`. `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, and `m_p` hashes and nodes
  unchanged.

- **CODATA 2018 muon-electron mass ratio is a one-sigma Interval.**
  `physis-constants` versions `mmu_me` as the CODATA 2018 hull
  `206.7682830(46)` from JPCRD 50, 033105 table XXXI (Muon, mu-).
  This is not the electron-muon ratio `me_mmu`, not a certificate that
  the stored centres invert, not the electron-muon magnetic-moment
  ratio `mu_e_mmu`, not the muon MeV hull, not an SI defining Ratio,
  not the Thomson cross section, and not P3N. Electron mass is still
  not stored (`10^{42}` overflows `i128`). `physis_model`
  `muon_electron_mass_ratio()` Qty locksteps to the recommended centre
  inside the hull. Adding `mmu_me` to LEDGER changes the ledger bundle
  pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`,
  `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`,
  `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`,
  `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, and `m_p`
  hashes are unchanged. Theories still evaluate with `f64` Qty. That is
  not a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `mmu_me` hash `0ac70815382ab74fd46513f298dd351685fcc54ab0e64b6fc00b64b4fccc426f`; node
  `6c0240f5d6812b9ed27e687e73892c43959dc1cac8c2697cd116f4c116f06c74`; ledger node `7ff71406d4f4487cb7e91b22cd6d2072212109057096b2784cc1a880998e853c`. `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, `m_mu_c2_MeV`, and `m_p` hashes and nodes unchanged.

- **CODATA 2018 muon mass energy equivalent in MeV is a one-sigma Interval.**
  `physis-constants` versions `m_mu_c2_MeV` as the CODATA 2018 hull
  `105.6583755(23)` MeV from JPCRD 50, 033105 table XXXI (Muon, mu-).
  This is not the joule hull `m_mu_c2`, not the exact electronvolt
  `eV`, not Hartree `Eh`, not Rydberg energy equivalent `hcRinf`, not
  an SI defining Ratio, not the Thomson cross section, and not P3N.
  Electron mass is still not stored (`10^{42}` overflows `i128`).
  `physis_model` `muon_mass_energy_equivalent_in_mev()` Qty locksteps
  to the recommended centre inside the hull. Adding `m_mu_c2_MeV` to
  LEDGER changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`,
  `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`,
  and `m_p` hashes are unchanged. Theories still evaluate with `f64`
  Qty. That is not a kernel proof, not Canonical, not P4. Encode pins
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `m_mu_c2_MeV` hash `292b0524e0f1a160403fe1a2a4998cd4c2690f5d3b344a5f8ba31e9248be0416`; node
  `b0d03e5dcc8f9174cfebf4d35d2ad0ab0836c6cde6d615cbdc21dd4e720d5dd4`; ledger node `1cf386b90e98059144cd17048cc9598ec27c2d9e01929cacaf37f9a8c041c5c6`. `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  `m_mu_c2`, and `m_p` hashes and nodes unchanged.

- **CODATA 2018 muon mass energy equivalent is a one-sigma Interval.**
  `physis-constants` versions `m_mu_c2` as the CODATA 2018 hull
  `1.692833804(38)×10^{-11}` J from JPCRD 50, 033105 table XXXI
  (Muon, mu-). This is not the kg hull `m_mu`, not the u-row `m_mu_u`,
  not the MeV conversion, not Rydberg energy equivalent `hcRinf`, not
  Hartree `Eh`, not an SI defining Ratio, not the Thomson cross
  section, and not P3N. Electron mass is still not stored (`10^{42}`
  overflows `i128`). `physis_model` `muon_mass_energy_equivalent()` Qty
  locksteps to the recommended centre inside the hull. Adding
  `m_mu_c2` to LEDGER changes the ledger bundle pin. The `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`,
  and `m_p` hashes are unchanged. Theories still evaluate with `f64`
  Qty. That is not a kernel proof, not Canonical, not P4. Encode pins
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `m_mu_c2` hash `d83a5072b8cb4fe869a2aa076aff9c4cd0d8f9f613a41eef52117124acde5854`; node `a451ddc9cfd85f74fc32ddaa156c25b2d60003cac9c3a2c7c60b17d3c2a2544a`;
  ledger node `0743d43662f2ecf9543e6a8e2375b730d04dfa0e832c2b2c9741fcff9f7051c6`. `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, and `m_p`
  hashes and nodes unchanged.

- **CODATA 2018 muon mass in u is a one-sigma Interval.**
  `physis-constants` versions `m_mu_u` as the CODATA 2018 hull
  `0.1134289259(25)` u from JPCRD 50, 033105 table XXXI (Muon, mu-).
  This is not the kg hull `m_mu`, not electron molar mass `M_e`, not
  the electron-muon mass ratio `me_mmu`, not the proton mass, not an
  SI defining Ratio, not the Thomson cross section, and not P3N.
  Electron mass is still not stored (`10^{42}` overflows `i128`).
  `physis_model` `muon_mass_in_u()` Qty locksteps to the recommended
  centre inside the hull. Adding `m_mu_u` to LEDGER changes the ledger
  bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`,
  `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`,
  `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`,
  `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`,
  `mu_e_mu0h`, `m_mu`, and `m_p` hashes are unchanged. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4. Verified: `m_mu_u` hash `ced234733b80023dd6d8687ce99efc8473defe15f63b74f3ecde00ece485515d`; node
  `d9dd36e1db3fe1aa782b3cfb99db87ba10250a4f0d945607d0cfa0ad6b163b78`; ledger node `d49a8837f61dc3d91153c268de4c6e033e8643f7d43857181c4a3acf964c1a52`. `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, and `m_p`
  hashes and nodes unchanged.

- **CODATA 2018 muon mass is a one-sigma Interval.**
  `physis-constants` versions `m_mu` as the CODATA 2018 hull
  `1.883531627(42)×10^{-28}` kg from JPCRD 50, 033105 table XXXI
  (Muon, mu-). This is not the electron-muon mass ratio `me_mmu`, not
  the proton mass, not the u-row, not an SI defining Ratio, not the
  Thomson cross section, and not P3N. Electron mass is still not stored
  (`10^{42}` overflows `i128`). `physis_model` `muon_mass()` Qty
  locksteps to the recommended centre inside the hull. Adding `m_mu` to
  LEDGER changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`,
  `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, and `m_p` hashes are unchanged.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified: `m_mu` hash
  `b1e0e67d46205c048709815e1215184c1b77afbcb0f197099085fbfc7d3bb016`;
  node
  `3cf58d635727710c293a539a68c0bce2aeadc9d41fa8a8dd43c238dfa58ad890`;
  ledger node
  `fd04fd5ac4d0c95aef6040eaf7b8837c2409763d0472aeaaf51581ea41080644`.
  `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, and `m_p` hashes
  and nodes unchanged.

- **CODATA 2018 electron to shielded-helion magnetic-moment ratio is a one-sigma Interval.**
  `physis-constants` versions `mu_e_mu0h` as the CODATA 2018 hull
  `864.058257(10)` from JPCRD 50, 033105 table XXXI (Electron, e-).
  This is the shielded helion in spherical gas at 25 °C, not the
  electron-helion mass ratio `me_mh`, not the shielded-proton ratio
  `mu_e_mu0p`, not vacuum permeability `mu0`, not an SI defining Ratio,
  not the Thomson cross section, and not P3N. `physis_model`
  `electron_to_shielded_helion_magnetic_moment_ratio()` Qty locksteps to
  the recommended centre inside the hull. Adding `mu_e_mu0h` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, `mu_e_mud`, and `m_p` hashes are unchanged. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4. Verified: `mu_e_mu0h` hash
  `3e3e29f0ac633705b8d8467b80b0cd229b07f4d7ba44fe32b84730261c576a9b`;
  node
  `f6b2ab92d421f6139a457f76b4898616573c38cef1e29d29941e0eb41c795e30`;
  ledger node
  `a9e7c73c18ae73364cc76cb9f38fa07c5351bde758c7dbbd59c221dae27ad4be`.
  `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, and `m_p` hashes and nodes
  unchanged.

- **CODATA 2018 electron-deuteron magnetic-moment ratio is a one-sigma Interval.**
  `physis-constants` versions `mu_e_mud` as the CODATA 2018 hull
  `−2143.9234915(56)` from JPCRD 50, 033105 table XXXI (Electron, e-).
  This is not the electron-deuteron mass ratio `me_md`, not an SI defining
  Ratio, not the Thomson cross section, and not P3N. `physis_model`
  `electron_deuteron_magnetic_moment_ratio()` Qty locksteps to the
  recommended signed centre inside the hull. Adding `mu_e_mud` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`,
  `mu_e_mun`, and `m_p` hashes are unchanged. Theories still evaluate
  with `f64` Qty. That is not a kernel proof, not Canonical, not P4.
  Encode pins unchanged. Unique-vacuum graph id unchanged. P3N count
  stays 4. Verified: `mu_e_mud` hash
  `7db59dc912a6c2a301f669f52d7353b27672a07b917e2f8b92b03c1f9acaaa64`;
  node
  `7a29b2b885a9c1ec2491ac30d0f7408fc89c2d7319e3bb511ab7a3892fef4d33`;
  ledger node
  `32eb3a5730a680353318d4e91154a1fb78d576c6645f6878aa6605e80c2f9487`.
  `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, `mu_e_mun`, and `m_p` hashes and nodes unchanged.

- **CODATA 2018 electron-neutron magnetic-moment ratio is a one-sigma Interval.**
  `physis-constants` versions `mu_e_mun` as the CODATA 2018 hull
  `960.92050(23)` from JPCRD 50, 033105 table XXXI (Electron, e-).
  This is not the electron-neutron mass ratio `me_mn`, not an SI defining
  Ratio, not the Thomson cross section, and not P3N. `physis_model`
  `electron_neutron_magnetic_moment_ratio()` Qty locksteps to the
  recommended centre inside the hull. Adding `mu_e_mun` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, and
  `m_p` hashes are unchanged. Theories still evaluate with `f64` Qty.
  That is not a kernel proof, not Canonical, not P4. Encode pins
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `mu_e_mun` hash
  `9abd0d4216937c89cafceaa4f418b8e8b65a2216df12b3bbc6a1976b1f5c8df2`;
  node
  `aee3c0c42e091e2c5f26b3d9466846186e6d1e70693c4c67deabf9f3a09bc4dc`;
  ledger node
  `baf6ce1a3939493711d2009f9dd3b82373357f38a12a994079167c0c633f0620`.
  `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  `mu_e_mu0p`, and `m_p` hashes and nodes unchanged.

- **CODATA 2018 electron to shielded-proton magnetic-moment ratio is a one-sigma Interval.**
  `physis-constants` versions `mu_e_mu0p` as the CODATA 2018 hull
  `−658.2275971(72)` from JPCRD 50, 033105 table XXXI (Electron, e-).
  This is the shielded proton in spherical H2O at 25 °C, not the
  free-proton ratio `mu_e_mup`, not vacuum permeability `mu0`, not the
  electron-proton mass ratio, not an SI defining Ratio, not the Thomson
  cross section, and not P3N. `physis_model`
  `electron_to_shielded_proton_magnetic_moment_ratio()` Qty locksteps to
  the recommended signed centre inside the hull. Adding `mu_e_mu0p` to
  LEDGER changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`,
  `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, and `m_p` hashes are
  unchanged. Theories still evaluate with `f64` Qty. That is not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified:
  `mu_e_mu0p` hash
  `a3028069b2f88c67432e3c555655438a64bd7b150b2add2b6539e38b3e2df199`;
  node
  `444c8953846cb45fe6790497b60c5dc1050cb39edc0f55d4f7c122a26e1d2279`;
  ledger node
  `c87e2d5cf2553835c9e136ff72f77d57049ba18478bce33c4fefdade0bdb14be`.
  `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`,
  and `m_p` hashes and nodes unchanged.

- **CODATA 2018 electron-proton magnetic-moment ratio is a one-sigma Interval.**
  `physis-constants` versions `mu_e_mup` as the CODATA 2018 hull
  `−658.21068789(20)` from JPCRD 50, 033105 table XXXI (Electron, e-).
  This is not the electron-proton mass ratio `me_mp`, not the
  shielded-proton moment ratio, not an SI defining Ratio, not the
  Thomson cross section, and not P3N. `physis_model`
  `electron_proton_magnetic_moment_ratio()` Qty locksteps to the
  recommended signed centre inside the hull. Adding `mu_e_mup` to
  LEDGER changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`,
  `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, and `m_p` hashes are unchanged.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified: `mu_e_mup` hash
  `13a0d90f76fb16f948196cf56fb9d54e90ccc43ad4ff613f27873de735ba7b5b`;
  node
  `c5b40558043871b42fac243c16485e1fec42d13d48622fe406ce1a65b33a8a3e`;
  ledger node
  `952e5bde58b546c812218769899ec5d1e02a22d27552e1de1ea06c7d88d8a675`.
  `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, and `m_p`
  hashes and nodes unchanged.

- **CODATA 2018 electron-muon magnetic-moment ratio is a one-sigma Interval.**
  `physis-constants` versions `mu_e_mmu` as the CODATA 2018 hull
  `206.7669883(46)` from JPCRD 50, 033105 table XXXI (Electron, e-).
  This is not the electron-muon mass ratio `me_mmu`, not an SI defining
  Ratio, not the Thomson cross section, and not P3N. `physis_model`
  `electron_muon_magnetic_moment_ratio()` Qty locksteps to the
  recommended centre inside the hull. Adding `mu_e_mmu` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, `ae`, `ge`, and `m_p` hashes are unchanged. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4. Verified: `mu_e_mmu` hash
  `125652aec9ee47a2db2df2ae81c39cfeb8d9b4037098829e64b78873deb56559`;
  node
  `12906f3612b3e923097deac331dfecbe0a8b7a03cf9232065aa0a3408a47b1b6`;
  ledger node
  `1c823c97af139995912fdcc794d293c2c22d9598617ead23608e53e3214da075`.
  `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, and `m_p` hashes and nodes
  unchanged.

- **CODATA 2018 electron g-factor is a one-sigma Interval.**
  `physis-constants` versions `ge` as the CODATA 2018 hull
  `−2.00231930436256(35)` from JPCRD 50, 033105 table XXXI
  (Electron, e-). This is not an SI defining Ratio, not the
  magnetic-moment anomaly, not the signed Bohr-magneton ratio, not the
  Thomson cross section, and not P3N. `physis_model`
  `electron_g_factor()` Qty locksteps to the recommended signed centre
  inside the hull. Adding `ge` to LEDGER changes the ledger bundle pin.
  The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`,
  `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`,
  `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, and `m_p`
  hashes are unchanged. Theories still evaluate with `f64` Qty. That is
  not a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified: `ge`
  hash
  `8e1daf3628381ffa7dce3fafc5e65038038eb74b5537cf7adb95702f5d0e0050`;
  node
  `98a79140e37ef1b8e6df0de890bd7dd704c443d879935fdcd62df8aa232540c1`;
  ledger node
  `ef75a6f09d5512dd848a7fb3423bf724d62117d02bc2047f9666c0e2d98f17b9`.
  `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, and `m_p` hashes and nodes
  unchanged.

- **CODATA 2018 electron magnetic-moment anomaly is a one-sigma Interval.**
  `physis-constants` versions `ae` as the CODATA 2018 hull
  `1.15965218128(18)×10^{-3}` from JPCRD 50, 033105 table XXXI
  (Electron, e-). This is not an SI defining Ratio, not the signed
  Bohr-magneton ratio, not the g-factor, not the Thomson cross section,
  and not P3N. `physis_model` `electron_magnetic_moment_anomaly()` Qty
  locksteps to the recommended centre inside the hull. Adding `ae` to
  LEDGER changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`,
  `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`,
  `mu_e_muN`, and `m_p` hashes are unchanged. Theories still evaluate
  with `f64` Qty. That is not a kernel proof, not Canonical, not P4.
  Encode pins unchanged. Unique-vacuum graph id unchanged. P3N count
  stays 4. Verified: `ae` hash
  `0fb8666d816320352cbc8e24b896bbb2adc59a085d3b469659d41c6447c82da5`;
  node
  `7ca6857af40ac6cf8f3b25125278adbff8732302c4ef9e8b4eb0889087f312bb`;
  ledger node
  `72ac634ed591eb5c7cd7901629ce3aa452648e603cee80a0c69d949b8164195d`.
  `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, `mu_e_muN`, and `m_p` hashes and nodes unchanged.

- **CODATA 2018 electron magnetic moment to nuclear magneton ratio is a one-sigma Interval.**
  `physis-constants` versions `mu_e_muN` as the CODATA 2018 hull
  `−1838.28197188(11)` from JPCRD 50, 033105 table XXXI (Electron, e-).
  This is not an SI defining Ratio, not the g-factor, not the
  magnetic-moment anomaly, not the Thomson cross section, and not P3N.
  `physis_model` `electron_magnetic_moment_to_nuclear_magneton()` Qty
  locksteps to the recommended signed centre inside the hull. Adding
  `mu_e_muN` to LEDGER changes the ledger bundle pin. The `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`,
  `mu_e_muB`, and `m_p` hashes are unchanged. Theories still evaluate
  with `f64` Qty. That is not a kernel proof, not Canonical, not P4.
  Encode pins unchanged. Unique-vacuum graph id unchanged. P3N count
  stays 4. Verified: `mu_e_muN` hash
  `2a82c539bc621b71977129a26433da37e94f1afd8b38e50c031da0133e2196ca`;
  node
  `fe37bac9de51edecd6c7fbca4718fe5995cbef58e829b91a03f2875e284db9c0`;
  ledger node
  `f388cfbf0d201f1a108c6e1dd33d94282e9d6f230c4cda6f413a073d30d8b8a0`.
  `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, `mu_e_muB`, and `m_p` hashes and nodes unchanged.

- **CODATA 2018 electron magnetic moment to Bohr magneton ratio is a one-sigma Interval.**
  `physis-constants` versions `mu_e_muB` as the CODATA 2018 hull
  `−1.00115965218128(18)` from JPCRD 50, 033105 table XXXI
  (Electron, e-). This is not an SI defining Ratio, not the g-factor,
  not the magnetic-moment anomaly, not the Thomson cross section, and
  not P3N. `physis_model` `electron_magnetic_moment_to_bohr_magneton()`
  Qty locksteps to the recommended signed centre inside the hull.
  Adding `mu_e_muB` to LEDGER changes the ledger bundle pin. The `G`,
  `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, and `m_p` hashes are unchanged. Theories still evaluate with
  `f64` Qty. That is not a kernel proof, not Canonical, not P4. Encode
  pins unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `mu_e_muB` hash
  `5d4db81093e3f34e08d258ab214de2fb6649d8e7f07cd37c2f5f625a89b52926`;
  node
  `2297f4ce64d7c1bd8e9ebdfde769d13acfd03f4334913adcc49a57346b1bbcd8`;
  ledger node
  `93ef99c20238bcc776df972d42a184ef1a7cc0a3047f75c53220cd0f92e20112`.
  `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  `mu_e`, and `m_p` hashes and nodes unchanged.

- **CODATA 2018 electron magnetic moment is a one-sigma Interval.**
  `physis-constants` versions `mu_e` as the CODATA 2018 hull
  `−9.2847647043(28)×10^{-24}` J T⁻¹ from JPCRD 50, 033105 table XXXI
  (Electron, e-). This is not an SI defining Ratio, not the
  Bohr-magneton ratio, not the Thomson cross section, and not P3N.
  `physis_model` `electron_magnetic_moment()` Qty locksteps to the
  recommended signed centre inside the hull. Adding `mu_e` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, and `m_p` hashes are
  unchanged. Theories still evaluate with `f64` Qty. That is not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified:
  `mu_e` hash
  `e48d03baa8e8b2f62d1ea5c19a7010b583cdfba3f4f9c3d2b55877817d36c9b8`;
  node
  `5ed9218a55b4eaa8b15614c412c1454a7be21e3a43a317c39275aa68095d5a0d`;
  ledger node
  `af8bab34f958743bc35430dd5bc547441101b171f7b7627ca9a46047f8e2e0e8`.
  `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`,
  and `m_p` hashes and nodes unchanged.

- **CODATA 2018 classical electron radius is a one-sigma Interval.**
  `physis-constants` versions `re` as the CODATA 2018 hull
  `2.8179403262(13)×10^{-15}` m from JPCRD 50, 033105 table XXXI
  (Electron, e-). This is not an SI defining Ratio, not a certificate
  of `α² a₀`, not the Thomson cross section, and not P3N. `physis_model`
  `classical_electron_radius()` Qty locksteps to the recommended centre
  inside the hull. Adding `re` to LEDGER changes the ledger bundle pin.
  The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`,
  `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`,
  `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  `lambda_C`, and `m_p` hashes are unchanged. Theories still evaluate
  with `f64` Qty. That is not a kernel proof, not Canonical, not P4.
  Encode pins unchanged. Unique-vacuum graph id unchanged. P3N count
  stays 4. Verified: `re` hash
  `1b8dfc7aa2f90183fd50dab61cf3361f57c3c906e6a221ffa3b2ef17302a38d4`;
  node
  `bd8a6f5f629ba9df37a0246f420d98c4bbde1d82cdcaaa8d4f9c7796ba239c23`;
  ledger node
  `f1362def92e68412b45d4326f2c60f014a7b21e6ed42c6f7136ebf461321aa2b`.
  `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, and
  `m_p` hashes and nodes unchanged.

- **CODATA 2018 Compton wavelength is a one-sigma Interval.**
  `physis-constants` versions `lambda_C` as the CODATA 2018 hull
  `2.42631023867(73)×10^{-12}` m from JPCRD 50, 033105 table XXXI
  (Electron, e-). This is not an SI defining Ratio, not a certificate
  of `2π ƛ_C`, not the reduced Compton wavelength, and not P3N.
  `physis_model` `compton_wavelength()` Qty locksteps to the recommended
  centre inside the hull. Adding `lambda_C` to LEDGER changes the ledger
  bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`,
  `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`,
  `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`,
  and `m_p` hashes are unchanged. Theories still evaluate with `f64`
  Qty. That is not a kernel proof, not Canonical, not P4. Encode pins
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `lambda_C` hash
  `6280f2b2f61adf3ae0fa3e65f3b12cfb4982f6601027d98552f541246198c3d8`;
  node
  `4c83c25a7c4f517afc2e092809b141dffc97ae12307b4676cb01da5ab73716e3`;
  ledger node
  `ca37826403917e2efb979285209a2b4befd41bb0bdcd0118ce5470649babe0c4`.
  `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, and `m_p` hashes
  and nodes unchanged.

- **CODATA 2018 reduced Compton wavelength is a one-sigma Interval.**
  `physis-constants` versions `lambdabar_C` as the CODATA 2018 hull
  `3.8615926796(12)×10^{-13}` m from JPCRD 50, 033105 table XXXI
  (Electron, e-). This is not an SI defining Ratio, not a certificate
  of `α a₀`, not the Compton wavelength, and not P3N. `physis_model`
  `reduced_compton_wavelength()` Qty locksteps to the recommended
  centre inside the hull. Adding `lambdabar_C` to LEDGER changes the
  ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`,
  `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`,
  `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`,
  `M_e`, and `m_p` hashes are unchanged. Theories still evaluate with
  `f64` Qty. That is not a kernel proof, not Canonical, not P4. Encode
  pins unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `lambdabar_C` hash
  `0ed48571f065fc19458ea3c8fd493fd00de18a7d196669f81bb93c50779bc625`;
  node
  `3fd48f3a014e92dae7062468ea0d7df4e4e1e44da7a6a9a6cccea5a5a4ffcc0d`;
  ledger node
  `42c800cb1f4b81008acf78d6797933f4d05d9ac1516ca397b9d26737a904b811`.
  `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, `M_e`, and `m_p` hashes and nodes
  unchanged.

- **CODATA 2018 electron molar mass is a one-sigma Interval.**
  `physis-constants` versions `M_e` as the CODATA 2018 hull
  `5.4857990888(17)×10^{-7}` kg mol⁻¹ from JPCRD 50, 033105 table XXXI
  (Electron, e-). This is not an SI defining Ratio, not electron mass
  in kg, not the mass-in-u row, and not P3N. After SI 2019 this is a
  different recommended value from `A_r(e) × 10^{-3}`. `physis_model`
  `electron_molar_mass()` Qty locksteps to the recommended centre
  inside the hull. Adding `M_e` to LEDGER changes the ledger bundle
  pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`,
  `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`,
  `me_mt`, `me_mh`, `me_malpha`, `e_me`, and `m_p` hashes are unchanged.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified: `M_e` hash
  `0a8b3285a4969854567b59db2ebf9449268df86ffdbb461e3b9c1db0955eb804`;
  node
  `da1692471def8d3d930d45de5d4e089231c2d18fc859d73feeb22ffe89075692`;
  ledger node
  `4ed817292bc4f07af2828c0d74a59e2db52b734c1afc52f0f0d3c79985082f09`.
  `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, `e_me`, and `m_p` hashes and nodes unchanged.

- **CODATA 2018 electron charge to mass quotient is a one-sigma Interval.**
  `physis-constants` versions `e_me` as the CODATA 2018 hull
  `−1.75882001076(53)×10^{11}` C kg⁻¹ from JPCRD 50, 033105 table XXXI
  (Electron, e-). This is not an SI defining Ratio, not electron mass,
  and not P3N. `physis_model` `electron_charge_to_mass()` Qty locksteps
  to the recommended signed centre inside the hull. Adding `e_me` to
  LEDGER changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`,
  `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`,
  and `m_p` hashes are unchanged. Theories still evaluate with `f64`
  Qty. That is not a kernel proof, not Canonical, not P4. Encode pins
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `e_me` hash
  `bfe24e8de43e90dbc8a28472f99ed206f07566fa1a4fa6c6d14356adf4e89b22`;
  node
  `4180ebda17cac1399d5888468d4686d9874499a1e6b2c386a3ccbe58f8039f36`;
  ledger node
  `b95e3d3313f4938bc7db2008a8614c812529e733b571e9ab1810c973ed9b540d`.
  `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, `me_malpha`, and `m_p` hashes and nodes unchanged.

- **CODATA 2018 electron-alpha mass ratio is a one-sigma Interval.**
  `physis-constants` versions `me_malpha` as the CODATA 2018 hull
  `1.370933554787(45)×10^{-4}` from JPCRD 50, 033105 table XXXI
  (Electron, e-). This is not an SI defining Ratio, not electron mass,
  and not P3N. `physis_model` `electron_alpha_mass_ratio()` Qty
  locksteps to the recommended centre inside the hull. Adding
  `me_malpha` to LEDGER changes the ledger bundle pin. The `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`,
  `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`,
  and `m_p` hashes are unchanged. Theories still evaluate with `f64`
  Qty. That is not a kernel proof, not Canonical, not P4. Encode pins
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `me_malpha` hash
  `3407529f38a47a2cf983c5418482d3d9bab5243e08258bbe88c06a2f42e0baa3`;
  node
  `ddb38fbd88d7250c7aea0e87e0bd2c44b32d5b5b0fd9eb1b0689bb9aa3315545`;
  ledger node
  `c18aa0d677306b849580bd5ccd97643e249197cd528e0b1878e2c8f1fd8b0216`.
  `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`,
  `me_mh`, and `m_p` hashes and nodes unchanged.

- **CODATA 2018 electron-helion mass ratio is a one-sigma Interval.**
  `physis-constants` versions `me_mh` as the CODATA 2018 hull
  `1.819543074573(79)×10^{-4}` from JPCRD 50, 033105 table XXXI
  (Electron, e-). This is not an SI defining Ratio, not electron mass,
  and not P3N. `physis_model` `electron_helion_mass_ratio()` Qty
  locksteps to the recommended centre inside the hull. Adding `me_mh`
  to LEDGER changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`,
  `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, and `m_p` hashes are
  unchanged. Theories still evaluate with `f64` Qty. That is not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified:
  `me_mh` hash
  `0fb8f5fde9e76fcf2c24d73267f36cd3a5b40ca9f27f24e47d9807ec4206055e`;
  node
  `b55534bac40b377d7b8c6123de509a2b65cde4d75fe280d46aefa30f83e72890`;
  ledger node
  `0e012902475c0c0e7118efdb898c447dc748e3b584ef5f1765947662cb141f29`.
  `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, and
  `m_p` hashes and nodes unchanged.

- **CODATA 2018 electron-triton mass ratio is a one-sigma Interval.**
  `physis-constants` versions `me_mt` as the CODATA 2018 hull
  `1.819200062251(90)×10^{-4}` from JPCRD 50, 033105 table XXXI
  (Electron, e-). This is not an SI defining Ratio, not electron mass,
  and not P3N. `physis_model` `electron_triton_mass_ratio()` Qty
  locksteps to the recommended centre inside the hull. Adding `me_mt`
  to LEDGER changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`,
  `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, `me_md`, and `m_p` hashes are unchanged.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified: `me_mt` hash
  `2f8187d744269836cf0fbc123f8cb7d60107215e65be109daf2ae67c8116afd1`;
  node
  `3d9b3ce3c7ecca0e131e0232f308ce878696a268e263286e133c8edc441eb7f0`;
  ledger node
  `431065e4bf6b088297eb51c40ab696da3f6d5c99c43322f1a3f6cb3589bb115f`.
  `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, and `m_p`
  hashes and nodes unchanged.

- **CODATA 2018 electron-deuteron mass ratio is a one-sigma Interval.**
  `physis-constants` versions `me_md` as the CODATA 2018 hull
  `2.724437107462(96)×10^{-4}` from JPCRD 50, 033105 table XXXI
  (Electron, e-). This is not an SI defining Ratio, not electron mass,
  and not P3N. `physis_model` `electron_deuteron_mass_ratio()` Qty
  locksteps to the recommended centre inside the hull. Adding `me_md`
  to LEDGER changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`,
  `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, `me_mn`, and `m_p` hashes are unchanged. Theories
  still evaluate with `f64` Qty. That is not a kernel proof, not
  Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified: `me_md` hash
  `2aa5fe69f8cdd03f44e77b006a3b6ea90d48e1b8aec71275e184c4e529f0f76c`;
  node
  `a2b8e4d5a5cdff854b67986773f186e1f427dc9cfc0d6d92f3a01ee81bdd26e5`;
  ledger node
  `c399bfc08a05cedfd6ba8b2b46d70ea2684cb4a418f6c5c8ba8b8482d8b20372`.
  `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, and `m_p` hashes and
  nodes unchanged.

- **CODATA 2018 electron-neutron mass ratio is a one-sigma Interval.**
  `physis-constants` versions `me_mn` as the CODATA 2018 hull
  `5.4386734424(26)×10^{-4}` from JPCRD 50, 033105 table XXXI
  (Electron, e-). This is not an SI defining Ratio, not electron mass,
  and not P3N. `physis_model` `electron_neutron_mass_ratio()` Qty
  locksteps to the recommended centre inside the hull. Adding `me_mn`
  to LEDGER changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`,
  `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, `me_mp`, and `m_p` hashes are unchanged. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4. Verified: `me_mn` hash
  `e271d2015c7b39491daebf2a1d532ebe4c4dacf8228b3f7fc4d258be7b79ecba`;
  node
  `deeb5e2665cabc16ffa607d446a4018cabf8b2b427fdb0b81184384113089bb3`;
  ledger node
  `57100ca7cff3357b06b19c57c698a842b660edf8056bbedfa8eb7fde79dc07d2`.
  `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, and `m_p` hashes and nodes
  unchanged.

- **CODATA 2018 electron-proton mass ratio is a one-sigma Interval.**
  `physis-constants` versions `me_mp` as the CODATA 2018 hull
  `5.44617021487(33)×10^{-4}` from JPCRD 50, 033105 table XXXI
  (Electron, e-). This is not an SI defining Ratio, not electron mass,
  and not P3N. `physis_model` `electron_proton_mass_ratio()` Qty
  locksteps to the recommended centre inside the hull. Adding `me_mp`
  to LEDGER changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`,
  `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`,
  `me_mmu`, and `m_p` hashes are unchanged. Theories still evaluate
  with `f64` Qty. That is not a kernel proof, not Canonical, not P4.
  Encode pins unchanged. Unique-vacuum graph id unchanged. P3N count
  stays 4. Verified: `me_mp` hash
  `b573fa37eb0080e54bc71e3bf41170421c2bae2911609e1d11ffc129448a2e7b`;
  node
  `b4fd3e8b7678afd9bb4aea49c3b06c9756ab3d6fced7b4b49b25c322134bf3f2`;
  ledger node
  `0c7002e55648a00f75ede864e5a73b0c56263543b63b972861052ecba09febab`.
  `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, `me_mmu`, and `m_p` hashes and nodes unchanged.

- **CODATA 2018 electron-muon mass ratio is a one-sigma Interval.**
  `physis-constants` versions `me_mmu` as the CODATA 2018 hull
  `4.83633169(11)×10^{-3}` from JPCRD 50, 033105 table XXXI
  (Electron, e-). This is not an SI defining Ratio, not electron mass,
  and not P3N. The quantum of circulation is not stored (`π`).
  `physis_model` `electron_muon_mass_ratio()` Qty locksteps to the
  recommended centre inside the hull. Adding `me_mmu` to LEDGER
  changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`, `Z0`,
  `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, and
  `m_p` hashes are unchanged. Theories still evaluate with `f64` Qty.
  That is not a kernel proof, not Canonical, not P4. Encode pins
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `me_mmu` hash
  `d57979e61fa03bae0a3b0dc5e2cff20df53cdcb76b772cf6ea2589e77c9c3cb2`;
  node
  `60d9b01d547b5ad4307443e4ba7749adb42c4da1343f16a35f194c80bbc35088`;
  ledger node
  `ee7e2d52efc48e80a2f003606bc954358bbedcbcf1e60dd137f14da7fd633cac`.
  `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`,
  `Rinf`, `a0`, `Eh`, and `m_p` hashes and nodes unchanged.

- **CODATA 2018 Rydberg energy equivalent is a one-sigma Interval.**
  `physis-constants` versions `hcRinf` as the CODATA 2018 hull
  `2.1798723611035(42)×10^{-18}` J from JPCRD 50, 033105 table XXXI
  (ATOMIC AND NUCLEAR). This is not an SI defining Ratio, not the eV
  conversion, and not P3N. `physis_model` `rydberg_energy_equivalent()`
  Qty locksteps to the recommended centre inside the hull. Adding
  `hcRinf` to LEDGER changes the ledger bundle pin. The `G`, `mu0`,
  `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `Rinf`, `a0`, `Eh`,
  and `m_p` hashes are unchanged. Theories still evaluate with `f64`
  Qty. That is not a kernel proof, not Canonical, not P4. Encode pins
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `hcRinf` hash
  `0d0308e874e54cb3d02570c972232b0d26c2d1d64b493880a1bb7ce4ff7827b2`;
  node
  `f7c095d695e231cfaee92b74cd8eb2961462727d1068401ee84953d069af4cbd`;
  ledger node
  `4bbea8428644c09de79c28d441929157bb2872e753fdd15bc0dbda7cf533a3dd`.
  `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `Rinf`,
  `a0`, `Eh`, and `m_p` hashes and nodes unchanged.

- **CODATA 2018 Rydberg frequency is a one-sigma Interval.**
  `physis-constants` versions `cRinf` as the CODATA 2018 hull
  `3.2898419602508(64)×10^{15}` Hz from JPCRD 50, 033105 table XXXI
  (ATOMIC AND NUCLEAR). This is not an SI defining Ratio, not the
  energy equivalent `hcR∞`, and not P3N. `physis_model`
  `rydberg_frequency()` Qty locksteps to the recommended centre inside
  the hull. Adding `cRinf` to LEDGER changes the ledger bundle pin.
  The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `Rinf`, `a0`,
  `Eh`, and `m_p` hashes are unchanged. Theories still evaluate with
  `f64` Qty. That is not a kernel proof, not Canonical, not P4. Encode
  pins unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `cRinf` hash
  `c7c49f18cb4f9905decad406f7a835f59588f34483afa3e4751097451d5d9969`;
  node
  `8fca9d435d8a31d1fafdac9a8825ce7f1535bf04eaf82785a1c62f66c900e60e`;
  ledger node
  `1ceae7214eccc32fe8f6b6134ec22afb85cd7e2d7a42d8aa7fb0112920505b10`.
  `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `Rinf`, `a0`, `Eh`,
  and `m_p` hashes and nodes unchanged.

- **CODATA 2018 Hartree energy is a one-sigma Interval.**
  `physis-constants` versions `Eh` as the CODATA 2018 hull
  `4.3597447222071(85)×10^{-18}` J from JPCRD 50, 033105 table XXXI
  (ATOMIC AND NUCLEAR). This is not an SI defining Ratio, not the eV
  conversion, and not P3N. `physis_model` `hartree_energy()` Qty
  locksteps to the recommended centre inside the hull. Adding `Eh` to
  LEDGER changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`,
  `Z0`, `alpha`, `inv_alpha`, `Rinf`, `a0`, and `m_p` hashes are
  unchanged. Theories still evaluate with `f64` Qty. That is not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified: `Eh`
  hash
  `c4606c77e55763a397f633ef0f3ace1328d3e1e8781428baf97554c97f4fba5a`;
  node
  `84818158c407563a9a514c8eedc85ee7303b0d96f09f09610bda6684582cc82e`;
  ledger node
  `8abfcf2eb2430a79300861a147f32f87bc0f42479165397b7ab13ef0b7ecfb70`.
  `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `Rinf`, `a0`, and
  `m_p` hashes and nodes unchanged.

- **CODATA 2018 Bohr radius is a one-sigma Interval.**
  `physis-constants` versions `a0` as the CODATA 2018 hull
  `5.29177210903(80)×10^{-11}` m from JPCRD 50, 033105 table XXXI
  (ATOMIC AND NUCLEAR). This is not an SI defining Ratio, not the
  Hartree energy, and not P3N. `physis_model` `bohr_radius()` Qty
  locksteps to the recommended centre inside the hull. Adding `a0` to
  LEDGER changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`,
  `Z0`, `alpha`, `inv_alpha`, `Rinf`, and `m_p` hashes are unchanged.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4.   Verified: `a0` hash
  `5d5098fcd983d3db221e4b4047e73de5061985c31a91ccdf12cd122b620eaf29`;
  node
  `01663e8bd28309970cefc37bd3dc5023c54a70ded784fb04d94ace095abdd475`;
  ledger node
  `9e68d473306527e7724874f9afbf9f2d8271c1b71fa650c85766fcf34ffd2ffc`.
  `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `Rinf`, and `m_p`
  hashes and nodes unchanged.

- **CODATA 2018 Rydberg constant is a one-sigma Interval.**
  `physis-constants` versions `Rinf` as the CODATA 2018 hull
  `10973731.568160(21)` m⁻¹ from JPCRD 50, 033105 table XXXI
  (ATOMIC AND NUCLEAR). This is not an SI defining Ratio, not the
  Rydberg frequency `c R∞`, not the Bohr radius, and not P3N.
  `physis_model` `rydberg()` Qty locksteps to the recommended centre
  inside the hull. Adding `Rinf` to LEDGER changes the ledger bundle
  pin. The `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, and
  `m_p` hashes are unchanged. Theories still evaluate with `f64` Qty.
  That is not a kernel proof, not Canonical, not P4. Encode pins
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `Rinf` hash
  `fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090`;
  node
  `0fb78b2d6e881df7b19d8a55878f642e27dc4d51a8f74ffe0c1e28e9d93380ac`;
  ledger node
  `fa7c94814b87f60480fde342ab6841258d7783831c4d2bf84849999614216b66`.
  `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, and `m_p` hashes
  and nodes unchanged.

- **CODATA 2018 inverse fine-structure is a one-sigma Interval.**
  `physis-constants` versions `inv_alpha` as the CODATA 2018 hull
  `137.035999084(21)` from JPCRD 50, 033105 table XXXI (ATOMIC AND
  NUCLEAR). This is a different recommended value from `alpha`, not
  `1/α` as a derived Ratio, not Rydberg, and not P3N. `physis_model`
  `inv_alpha()` Qty locksteps to the recommended centre inside the hull.
  Adding `inv_alpha` to LEDGER changes the ledger bundle pin. The `G`,
  `mu0`, `epsilon0`, `Z0`, `alpha`, and `m_p` hashes are unchanged.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified: `inv_alpha` hash
  `4b7050d77da09c5322877eaf83e94ebba7b84c99bad8ba3713b0e5fe91128482`;
  node
  `6943c43fe01b2b9dbde1c0bd147f0293a69cb15bb2e44877ea7e68013f6dce0e`;
  ledger node
  `95373c2218069b60b99e1dd4864d13f058f64292ed1fb270d30c581dfe405c49`.
  `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, and `m_p` hashes and nodes unchanged.

- **CODATA 2018 characteristic impedance is a one-sigma Interval.**
  `physis-constants` versions `Z0` as the CODATA 2018 hull
  `376.730313668(57)` ohm from JPCRD 50, 033105 table XXXI
  (UNIVERSAL). After SI 2019 this is the derived hull `μ₀ c`, not an
  SI defining Ratio, not `Y₀`, and not P3N. `physis_model` `z0()` Qty
  locksteps to the recommended centre inside the hull. Adding `Z0` to
  LEDGER changes the ledger bundle pin. The `G`, `mu0`, `epsilon0`,
  `alpha`, and `m_p` hashes are unchanged. Theories still evaluate
  with `f64` Qty. That is not a kernel proof, not Canonical, not P4.
  Encode pins unchanged. Unique-vacuum graph id unchanged. P3N count
  stays 4. Verified: `Z0` hash
  `6f72c1c5833dc722ac6fb5223f982879499ff412157c6e6c9851d77088991316`;
  node
  `e7ebeaa7b1b18ebed7aa269cd7b4b322842d5f674106efb8b1ad1cda4d4ac77e`;
  ledger node
  `e3ea0ddd0afbc14654fba101f73e66c80bfd3985a66e419d58a752e342a2a40a`.
  `G`, `mu0`, `epsilon0`, `alpha`, and `m_p` hashes and nodes unchanged.

- **CODATA 2018 vacuum permittivity is a one-sigma Interval.**
  `physis-constants` versions `epsilon0` as the CODATA 2018 hull
  `8.8541878128(13)×10^{-12}` F m⁻¹ from JPCRD 50, 033105 table XXXI
  (UNIVERSAL). After SI 2019 this is the derived hull `1/(μ₀ c²)`, not
  an SI defining Ratio, not `Z₀`, and not P3N. `physis_model`
  `epsilon0()` Qty locksteps to the recommended centre inside the hull.
  Adding `epsilon0` to LEDGER changes the ledger bundle pin. The `G`,
  `mu0`, `alpha`, and `m_p` hashes are unchanged. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4. Verified: `epsilon0` hash
  `fadaf2a47a8161ba2727a4c2ff6b842f7c9e6add2edd67cd5496a7a753f22d80`;
  node
  `0b2262eee35047174ebb10962a4aaf06481a8e18e9b44a3873a129f91873a8b7`;
  ledger node
  `6e6fbf4591060aee4df351393fdc5fd5050537371de589268d4dece3e65ff31b`.
  `G`, `mu0`, `alpha`, and `m_p` hashes and nodes unchanged.

- **CODATA 2018 vacuum permeability is a one-sigma Interval.**
  `physis-constants` versions `mu0` as the CODATA 2018 hull
  `1.25663706212(19)×10^{-6}` N A⁻² from JPCRD 50, 033105 table XXXI
  (UNIVERSAL). After SI 2019 this is a measured hull, not exact
  `4π×10^{-7}`, not `ε₀`, and not P3N. `physis_model` `mu0()` Qty
  locksteps to the recommended centre inside the hull. Adding `mu0` to
  LEDGER changes the ledger bundle pin. The `G`, `alpha`, and `m_p`
  hashes are unchanged. Theories still evaluate with `f64` Qty. That is
  not a kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified:
  `mu0` hash
  `fa1264a6ce514520c9c2d9131fee2c71cacd4ce5fe615ea4dd424fd23de35cd7`;
  node
  `2b652a4d95e792481d77b5358af0426e6425619a32e86965bd6f19cbec20ae9e`;
  ledger node
  `c96218f76b835484231479e0130bb12a22019c82bebd8372c52767f2a00dff9e`.
  `G`, `alpha`, and `m_p` hashes and nodes unchanged.

- **CODATA 2018 proton mass is a one-sigma Interval.**
  `physis-constants` versions `m_p` as the CODATA 2018 hull
  `1.67262192369(51)×10^{-27}` kg from JPCRD 50, 033105 table XXXI
  (Proton, p). That is a measured hull, not an SI defining Ratio, not
  electron mass, and not P3N. `10^{38}` fits `i128`; `m_e` still does
  not (`10^{42}` overflows). `physis_model` `proton_mass()` Qty
  locksteps to the recommended centre inside the hull. Adding `m_p` to
  LEDGER changes the ledger bundle pin. The `G` and `alpha` hashes are
  unchanged. Theories still evaluate with `f64` Qty. That is not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified:
  `m_p` hash
  `ffd371a69f7ec3d9bac8dcf57e0126709fd3f63c35561e717d9886d2fb1f88c8`;
  node
  `3fcee64bc46c5e13e69bc0d822e66e701b793910803445e5d4689ead316028a0`;
  ledger node
  `e3071c5cb052b899858040570c13196947ccb7b40000b0aab749cb2fcb6ddd53`.
  `G` and `alpha` hashes and nodes unchanged.

- **CODATA 2018 fine-structure constant is a one-sigma Interval.**
  `physis-constants` versions `alpha` as the CODATA 2018 hull
  `7.2973525693(11)×10^{-3}` from JPCRD 50, 033105 table XXXI
  (ATOMIC AND NUCLEAR). That is a measured hull, not an SI defining
  Ratio, not inverse-α, and not P3N. `physis_model`
  `fine_structure_constant()` Qty locksteps to the recommended centre
  inside the hull. Adding `alpha` to LEDGER changes the ledger bundle
  pin. The `G` hash is unchanged. Theories still evaluate with `f64`
  Qty. That is not a kernel proof, not Canonical, not P4. Encode pins
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `alpha` hash
  `cef64589acdbd1ed4cb5f5f631658978c01477248f334b1d3563e57314644b38`;
  node
  `b2b54749bb1e674d72e0b1c7ffa688dbd1cabb8a8a481db3f94bfeba9735f073`;
  ledger node
  `fad9707080f60d4a4f4d1bf8fbf6060f0d58bc4999aea57af21b9e531fec337a`.
  `G` hash and node unchanged.

- **SI 2019 electronvolt is an exact Ratio.**
  `physis-constants` versions `eV` as `1.602176634×10^{-19}` J from BIPM
  Brochure 9th ed. table 8. Same SI 2019 decimal as `e`, unit joule not
  coulomb. Qty locksteps via IEEE rounding of the SI decimal, not
  `Ratio::to_f64` of the reduced fraction. Adding `eV` to LEDGER
  changes the ledger bundle pin. The `au` hash is unchanged. Theories
  still evaluate with `f64` Qty. That is not a kernel proof, not
  Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified: `eV` hash
  `d5514de9cbef3f6990067899529d34f20b4349ca3b20ba18c9a5932c8c6b6c0f`;
  node
  `94271807b581af8f9842b3022f06dd8282ec141b9d2a5519d33885d76335e66f`;
  ledger node
  `56663fe60a0706f1bc17046451b4aeca4c6443810be420d8a75c0f6bae474034`.
  `au` hash and node unchanged.

- **IAU 2015 nominal solar luminosity is an exact Ratio.**
  `physis-constants` versions `L_sun` as `3.828×10^26` W from Prša et
  al., Astron. J. 152, 41 table 1 (IAU 2015 Resolution B3). That is a
  conversion ruler, not a measured solar luminosity and not P3N.
  `physis_model` `solar_luminosity()` Qty locksteps via integer
  `to_f64`. Adding `L_sun` to LEDGER changes the ledger bundle pin.
  The `GM_sun` and `R_sun` hashes are unchanged. Theories still
  evaluate with `f64` Qty. That is not a kernel proof, not Canonical,
  not P4. Encode pins unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4. Verified: `L_sun` hash
  `444f85fba501ddec8fb08ba403c1b869cc78a2284df5466a56a617043807bbc4`;
  node
  `1c513225d8721ad42106237a67dc8755fbf7f3449f0585f185820a01ea1d9f17`;
  ledger node
  `f8561e25367d90bc433ce45465191f57563ee909cb0635a725a00c386b63d8c3`.
  `GM_sun` and `R_sun` hashes and nodes unchanged.

- **IAU 2015 nominal solar radius is an exact Ratio.**
  `physis-constants` versions `R_sun` as `695700000` m from Prša et al.,
  Astron. J. 152, 41 table 1 (IAU 2015 Resolution B3). That is a
  conversion ruler, not a measured photospheric radius and not P3N.
  `L_☉^N` is not stored. `physis_model` `solar_radius()` Qty locksteps
  via integer `to_f64`. Adding `R_sun` to LEDGER changes the ledger
  bundle pin. The `GM_sun` hash is unchanged. Theories still evaluate
  with `f64` Qty. That is not a kernel proof, not Canonical, not P4.
  Encode pins unchanged. Unique-vacuum graph id unchanged. P3N count
  stays 4. Verified: `R_sun` hash
  `cb7f91f2d0663d2d8ff8b0e3009f6e0772a126220d04ed658fc793db7e5cc6b4`;
  node
  `ee886ab3541dcb83a8cb4dc11d66a476cfe0c747b177cd85b2f00a7a3921ef5c`;
  ledger node
  `23bb7b95e12a0d9ae62b137b978b80777da39b979f0808439f1d46207355dd93`.
  `GM_sun` hash and node unchanged.

- **IAU 2015 nominal solar GM is an exact Ratio.**
  `physis-constants` versions `GM_sun` as `1.3271244×10^20` m³ s⁻² from
  Prša et al., Astron. J. 152, 41 table 1 (IAU 2015 Resolution B3).
  That is a conversion ruler, not a measured solar mass, not CODATA
  `G`, and not P3N. `R_☉^N` and `L_☉^N` are not stored. `physis_model`
  `solar_gm()` Qty locksteps via integer `to_f64`. Adding `GM_sun` to
  LEDGER changes the ledger bundle pin. Theories still evaluate with
  `f64` Qty. That is not a kernel proof, not Canonical, not P4. Encode
  pins unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `GM_sun` hash
  `636001001c4ed9cd5e6661241e5ad5e5db09c8419a3fe79790143162b7af3a58`;
  node
  `3889862dda7c8968970b169e5e32e8c555c9d2eaada8f9f1ef93e17cd82965d2`;
  ledger node
  `a58016890420407b870fdd77ac7aedcc793a135846e434ec13a8656a753d6b45`.

- **IAU 2012 astronomical unit is an exact Ratio.**
  `physis-constants` versions `au` as `149597870700` m from BIPM
  Brochure 9th ed. table 8 (IAU 2012 Resolution B2). That is a
  conventional length, not an SI defining constant and not P3N. The
  parsec stays `(648000/π) au` and is not a Ratio. `physis_model`
  exposes `astronomical_unit()` Qty and locksteps it via integer
  `to_f64`. Adding `au` to LEDGER changes the ledger bundle pin.
  Theories still evaluate with `f64` Qty. That is not a kernel proof,
  not Canonical, not P4. Encode pins unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified: `au` hash
  `d3441603d75b565016c25cc955783fbb76b4050ee22befcef0c0e3896e873a0b`;
  node
  `9e736e474a9756cc084b206154d675e3f984376d9ecc07d7e1894ed470d5235d`;
  ledger node
  `ca85a4448a25c85f99edd17b65d11c8723c62c6cafe5710c86ef09def921930c`.

- **`physis_model` Qty floats lockstep the versioned ledger.**
  `c`, `e`, `k`, and `h` `f64` Qty values lockstep the versioned
  ledger. `c` is an integer `Ratio` (`to_f64`). `e` and `k` match
  IEEE rounding of the SI decimal (`SciExact::to_f64`), not
  `Ratio::to_f64` of the reduced fraction. `h` matches `SciExact`
  `to_f64` (still not a `Ratio`). CODATA 2018 `G`'s Qty is the
  recommended centre inside the one-sigma hull, not an exact `Ratio`.
  `ħ` is not a ledger entry. Theories still evaluate with `f64` Qty.
  That is not a kernel proof, not Canonical, not P4. Encode pins
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.

### Protocol

- **`physis loop` rebuilds the versioned constants ledger.**
  After cite, the lab independently reconstructs every LEDGER entry
  into the same VersionedConstant bundle as `physis constant` with no
  name, and journals an empty `name`. That is not P3N, not P3S, not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified:
  loop line `constant  ledger
  2a2ad9dc2e70d8f1505206d605876242fe6ba8665146376a4a370ed6a74bab84`.

- **`physis constant` with no name rebuilds the full LEDGER.**
  `provenance-auditor` independently reconstructs every versioned
  constant in catalog order, stores each `VersionedConstant` node, and
  bundles them under one ledger node whose payload is `name nodehex`
  lines. An empty journal `name` is that bundle; a recorded
  `node_hash` is not deserialized. Named `physis constant G` is
  unchanged. That is not P3N, not P3S, not a kernel proof, not
  Canonical, not P4. Theories still use `physis_model` `f64` Qty.
  Encode pins unchanged. Unique-vacuum graph id unchanged. P3N count
  stays 4. Verified: explorer blocked; restore rebuilds; G node
  `f320ea2da0141f16c191acd3001a6fe0b5074fc73d4768fa91f42d8e85abc52c`;
  ledger node
  `2a2ad9dc2e70d8f1505206d605876242fe6ba8665146376a4a370ed6a74bab84`.

- **`physis constant` independently rebuilds the versioned ledger.**
  `provenance-auditor` reconstructs SI 2019 `Ratio` / `SciExact` entries
  and CODATA 2018 `G` from live constructors, stores a
  `VersionedConstant` node, and journals a rebuild-only event. A
  recorded `node_hash` is not deserialized. Unknown names fail closed.
  That is not P3N, not P3S, not a kernel proof, not Canonical, not P4.
  Theories still use `physis_model` `f64` Qty. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified: `G`,
  `c`, and `h` hashes; restore rebuilds; explorer/reviewer blocked;
  node
  `f320ea2da0141f16c191acd3001a6fe0b5074fc73d4768fa91f42d8e85abc52c`.

### Constants

- **SI 2019 Planck h is SciExact, not a Ratio.**
  `physis-constants` versions `h` as `662607015e-42` J s from BIPM
  Brochure 9th ed. table 1. The reduced denominator `10^42` overflows
  `i128`, so this is not a `Ratio`. `ħ` is not a terminating decimal
  and is not stored. Theories still use `physis_model` `f64` Qty
  constants. That is not a kernel proof, not Canonical, not P4. Encode
  pins unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: `to_ratio` is `None`; hash
  `50a96a8715769547a90cba69b0775d8892d79f2fa32465ad13a6d73b2d111eef`.

- **CODATA 2018 Newtonian G is a one-sigma Interval.**
  `physis-constants` versions `G` as the recommended hull
  `6.67430(15)×10⁻¹¹` from JPCRD 50, 033105 table XXXI, not an SI
  defining `Ratio` and not P3N. Planck's `h` is still not a `Ratio`.
  Theories still use `physis_model` `f64` Qty constants. That is not a
  kernel proof, not Canonical, not P4. Encode pins unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified:
  one-sigma hull; hash
  `ebbfc13ea8fba734da50b679d9eaf236638b244cdcc350c0b14cdd6696850e92`.

- **SI 2019 defining constants that fit in Ratio.**
  `physis-constants` versions `c`, `Δν_Cs`, `e`, `k`, `N_A`, and
  `K_cd` as exact SI 2019 `Ratio` values with BIPM Brochure 9th ed.
  table 1 locators. Planck's `h` is SI-exact but is not a `Ratio`
  here: `10^42` overflows `i128`. Theories still use `physis_model`
  `f64` Qty constants. That is not a kernel proof, not Canonical, not
  P4. Encode pins unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4. Verified: exact fractions; hashes stable; `10^42`
  does not fit.

### Encoding

- **Encode lists each bound catalog identity by claim id.**
  Live encode still hashes the canonical IR bytes. After reconstruct
  it prints one `catalog identity tree  <claim-id>` line per catalog
  tree in the equations, in catalog order. `lean_ref` still fails
  closed if it names a catalog type whose tree is missing. Token
  packages print no such line. That is not a kernel proof, not P3S,
  not Canonical, not P4. Encode pins unchanged. Verified: de-rham
  prints `dec.d-squared-zero`; special-relativity prints interval,
  composition, and mass-shell; GR and Planck skip; pins
  `187ee7fd592ffb31a1e5f31fea50d158f7b67bd97f6fbf292c139683445006a6`
  and
  `faecac5791ad5650337c61dcb10e45d5eb36ca24c0423df51891673ba3da3ef6`.

- **Special relativity live package carries all three catalog identity trees.**
  `boost lorentz` stays the evaluator encoding. The package lists the
  interval, Einstein composition, and mass-shell polynomials. `lean_ref`
  remains the catalog interval type, so live `encode` still binds that
  tree. A package missing composition or mass-shell fails closed. That
  is not a Physlib pointer without the tree, not a kernel proof, not
  P3S, not Canonical, not P4. GR and Planck stay token packages.
  de-rham coboundary pin unchanged. Verified: encode special-relativity
  `equations  4` and `claims     3`; pin
  `faecac5791ad5650337c61dcb10e45d5eb36ca24c0423df51891673ba3da3ef6`.

- **Special relativity live package carries the catalog interval tree.**
  `boost lorentz` stays the evaluator encoding. The package also lists
  the interval identity polynomial and `lean_ref` is the catalog type,
  so live `encode` binds the tree. A token-only Lorentz package fails
  closed. That is not a Physlib pointer without the tree, not a kernel
  proof, not P3S, not Canonical, not P4. GR and Planck stay token
  packages. de-rham coboundary pin unchanged. Verified: encode
  special-relativity bound the interval tree and `equations  2`; pin
  `91f188d526d4190ba611631b2f41818ee9a46c3e924fe8297d26592b9819691e`.

- **Live encode binds `lean_ref` to the catalog identity tree.**
  `physis encode` still stores the package hash of the canonical IR
  bytes. A live package with `lean_ref` must parse an equation whose
  canonical tree is the catalog identity named by that type. Token
  packages (Planck–Bose, Einstein–Hilbert, …) have no `lean_ref` and
  skip. A catalog type without the tree, or a Physlib pointer that is
  not a catalog type, fails closed. That is not a kernel proof, not
  P3S, not Canonical, not P4. de-rham pin unchanged. Verified: de-rham
  bound the coboundary tree; planck does not; pin
  `187ee7fd592ffb31a1e5f31fea50d158f7b67bd97f6fbf292c139683445006a6`.

### Validated numerics

- **GQW mixing-angle enclosure is sourced PDG input σ.**
  One-loop `sin²θ_W(M_Z)` is a rational function of recorded PDG
  `α_em⁻¹ = 127.951` and `α_s = 0.1179` and the SM/MSSM betas;
  `2π` cancels. The interval cell encloses that algebraic function by
  the sourced PDG 2022 one-sigma hulls `α_s(M_Z) = 0.1179(9)` and
  `α_em⁻¹(M_Z) = 127.951 ± 0.009` (Physical Constants / Electroweak
  reviews), not a 3% remainder certificate and not P3N. Interval
  add/mul/div is the Minkowski sum and four-corner product. Gaussian
  NLL still snaps to the PDG `10^{-5}` scale so the likelihood lives
  at the dataset σ. Minimal SU(5) is disjoint from the PDG mixing-angle
  hull (excluded). One-loop MSSM overlaps that hull but is not contained
  in it (insufficient-precision). The heuristic
  3% cell can still hold under MSSM. `enclose` still refuses the GQW
  cell. Unique-vacuum graph id unchanged. P3N count stays 4. Verified:
  exact SM centre `12588941801/60643400058`; MSSM
  `522562687/2262813435`; input-interval exclusion for SU(5); NLL still
  smaller for MSSM.

### Computed theorems

- **De Rham down Laplacian is an IR mutation**
  (`de-rham`, `add-down-laplacian`). The catalog coboundary
  identity is still the live encoding; appending `laplacian down`
  assembles Δ₁ from `d₀d₀ᵀ` only and on the disk flips
  `dec.hodge-harmonic` holds to fails. The residual is
  `dim ker d₀d₀ᵀ = 1` vs `b₁ = 0`, not a unit flag and not the
  `shape` knob. The coboundary sign remains a separate IR fork
  (`add-sign-flip`) that flips `dec.d-squared-zero` while Hodge
  still holds. On the circle overlay, Hodge still holds (no faces,
  live Hodge is already down-only). `shape` stays a knob and still
  flips Poincaré. Mutants stay `de-rham`; they are not a silent
  Maxwell install. Mutants are not installed, not journaled, and
  not Canonical or P4. Catalog d² hash unchanged. Unique-vacuum
  graph id unchanged. P3N count stays 4. Verified: IR round-trip;
  set down_laplacian is unknown; hypothesize de-rham; live
  coboundary restored; encode pin
  `187ee7fd592ffb31a1e5f31fea50d158f7b67bd97f6fbf292c139683445006a6`.

- **GR Brans-Dicke scalar-tensor is an IR mutation**
  (`general-relativity`, `add-brans-dicke`). Einstein-Hilbert is still
  the live encoding (`action einstein-hilbert`); appending
  `action brans-dicke` uses PPN `γ = (ω+1)/(ω+2)` at `ω = 1` and flips
  `predictivity.unique-vacuum`, `gr.eddington-deflection`, and
  `gr.mercury-perihelion` holds to fails. The residual is the PPN
  light factor `(1+γ)/2 = 5/6` times the GR Schwarzschild integral, not
  a unit flag and not Soldner `2GM/(c²R)`. Quadratic curvature remains
  a separate IR fork (`add-r-squared`) that flips uniqueness while
  Eddington and Mercury still hold. `dim` stays a knob and still makes
  the 4D solar tests inapplicable. Mutants stay `general-relativity`;
  they are not a silent Newton install. Mutants are not installed, not
  journaled, and not Canonical or P4. Catalog d² hash unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified: IR
  round-trip; set brans_dicke is unknown; hypothesize
  general-relativity; live Einstein-Hilbert restored; encode pin
  `8e99553456fa93c2774e4021eb87bb4dd0547f457cf549ec4bf11859313f7be0`.

- **Planck zero-point vacuum is an IR mutation**
  (`planck`, `add-zero-point`). Planck–Bose is still the live encoding
  (`mode planck-bose`); appending `mode zero-point` uses
  ⟨E⟩ = hν/2 + Bose and flips `thermo.uv-finite`,
  `thermo.stefan-boltzmann`, and `thermo.wien-displacement` holds to
  fails. The vacuum integral diverges as ν_max⁴ (doubling ≈ 16, not
  Rayleigh–Jeans ν³). The infrared still matches Rayleigh–Jeans at
  hν = 0.01 kT. Equipartition still fails: at hν = 8 kT the ratio is
  ⟨E⟩/kT ≈ 4 from the zero-point piece, not freeze-out ≪ kT. That is
  not a knob and not `quantum false`. Truncated Wien occupation remains
  a separate IR fork (`add-wien`) that flips only the infrared
  correspondence. `quantum` stays a knob and still restores the
  ultraviolet catastrophe and equipartition. IR correspondence keeps
  the catalog `hν = 0.01 kT` domain. Mutants stay `planck`; they are
  not a silent `rayleigh-jeans` install. `rayleigh-jeans` has no
  package. Mutants are not installed, not journaled, and not Canonical
  or P4. Catalog d² hash unchanged. Unique-vacuum graph id unchanged.
  P3N count stays 4. Verified: IR round-trip; set zero_point is
  unknown; hypothesize planck; live Bose restored; encode pin
  `7f7e69662ab0960948a1dc7c965078eddda2687e31ea7eebfdc2ab93aa69807b`.

- **SR minus-uv composition is an IR mutation**
  (`special-relativity`, `add-minus-uv`). Exact Lorentz is still the
  live encoding (`boost lorentz`); appending `compose minus-uv` uses
  `w = (u+v)/(1−uv)` and flips `sr.subluminal-composition` holds to
  fails. The residual is the mismatch with Einstein addition
  `(u+v)/(1+uv)` at `0.8c ⊕ 0.7c` and is evidence, not the encoding:
  tiny speeds stay subluminal and the composition cell still fails.
  Interval and mass-shell still hold (Lorentz boosts). That is not a
  knob and not Galilean `u+v`. Truncated binomial γ remains a separate
  IR fork (`add-binomial-gamma`) that flips interval and mass-shell
  while composition still holds. `absolute_time` stays a knob and still
  flips all three claims. Composition keeps the catalog collinear
  domain. Mutants stay `special-relativity`; they are not a silent
  Galilean install. Mutants are not installed, not journaled, and not
  Canonical or P4. Catalog d² hash unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified: IR round-trip; set minus_uv
  is unknown; hypothesize special-relativity; live Lorentz restored;
  encode pin
  `4e8c15ecdfc2a60f3bf481898c7cb3e852d22ab9884f377b7728662c3f830c1e`.

- **Type I Chan-Paton SO(16) is an IR mutation**
  (`type-i`, `add-chan-paton-16`). Complete `Chan-Paton SO(32)` is the
  live encoding; appending `Chan-Paton SO(16)` flips
  `consistency.anomaly-cancellation` holds to fails (dimension 120 is
  not a Green-Schwarz solution). SM still embeds via SO(10), so that
  cell is not a unit flag and not the `kind` or `total_dim` knob.
  Unique-vacuum still fails. Mutants stay `type-i`. Heterotic SO(32)
  keeps `add-so16` and does not grow this mutation. Type II has no
  package. Mutants are not installed, not journaled, and not Canonical
  or P4. Catalog d² hash unchanged. Unique-vacuum graph id unchanged.
  P3N count stays 4. Verified: IR round-trip; set chan_paton_16 is
  unknown; hypothesize type-i; live Chan-Paton SO(32) restored; encode
  pin `87e40657853eb6ccd781d9c69134187f055979177276e4857884741af57e114d`.

- **Heterotic SO(16) is an IR mutation**
  (`heterotic-so32`, `add-so16`). Complete `SO(32)` is the live
  encoding; appending `SO(16)` flips
  `consistency.anomaly-cancellation` holds to fails (dimension 120 is
  not a Green-Schwarz solution). SM still embeds via SO(10), so that
  cell is not a unit flag and not the `kind` or `total_dim` knob.
  Unique-vacuum still fails. Mutants stay `heterotic-so32`. Type I
  shares SO(32) gauge but has no package. Mutants are not installed,
  not journaled, and not Canonical or P4. Catalog d² hash unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified: IR
  round-trip; set so16 is unknown; hypothesize heterotic-so32; live
  SO(32) restored; encode pin `8931d99fcd313e83cc90e75a76c684853912b1c31fffd279aea84a04d274e9c2`.

- **Heterotic missing E8 is an IR mutation**
  (`heterotic-e8e8`, `add-missing-e8`). Complete `E8 x E8` is the live
  encoding; appending `missing E8` flips
  `consistency.anomaly-cancellation` holds to fails (dimension 248 is
  not a Green-Schwarz solution). SM still embeds in the remaining E8,
  so that cell is not a unit flag and not the `kind` or `total_dim`
  knob. Unique-vacuum still fails. Mutants stay `heterotic-e8e8`.
  Other string constructions have no package. Mutants are not
  installed, not journaled, and not Canonical or P4. Catalog d² hash
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: IR round-trip; set missing_e8 is unknown; hypothesize
  heterotic-e8e8; live E8 x E8 restored; encode pin
  `c6cab84980b2320e96b4393a373de44a6fbbcdb31d54d350003e7294b61a7329`.

- **Dulong–Petit quartic virial is an IR mutation**
  (`dulong-petit`, `add-quartic`). Harmonic `U = 3 N k T` is the live
  encoding; appending `U = 9/4 N k T` flips `thermo.dulong-petit` holds
  to fails. The third law still fails (`C_V` stays `9/4 N k`), so that
  cell is not a unit flag and not Einstein freeze-out. High-T
  correspondence also fails: quartic virial is not `3 N k` at any T.
  That is not a knob. `quantum` and `spectrum` stay knobs. Mutants stay
  `dulong-petit`. `einstein-solid` has no package. Mutants are not
  installed, not journaled, and not Canonical or P4. Catalog d² hash
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Verified: IR round-trip; set anharmonic is unknown; hypothesize
  dulong-petit; live harmonic restored; encode pin
  `82138399bbfc4f442d125df64e3bc31833ae23f11070f5c8dbd0460b4531eaea`.

- **Observer-geometry missing Spin(10) is an IR mutation**
  (`observer-geometry`, `add-missing-spin10`). `Spin(10) on 10-fibre` is
  the live encoding; appending `missing Spin(10)` flips
  `empirical.sm-gauge` holds to fails. Uniqueness still holds as the
  program axiom, so that cell is not a unit flag and not the
  `unique_vacuum` or `derive_gauge` knob. Mutants stay
  `observer-geometry`. Mutants are not installed, not journaled, and not
  Canonical or P4. Catalog d² hash unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified: IR round-trip; set
  missing_spin10 is unknown; hypothesize observer-geometry; live Spin(10)
  restored; encode pin
  `fefb1522c8782cc9e2ceee5af785cca9a3c296ee4dfc174ed65e0fd0c51fcd30`.

- **SM missing e_R is an IR mutation**
  (`standard-model`, `add-missing-eR`). Complete `Q_L + uRc + dRc + L_L
  + eRc` is the live encoding; appending `missing e_R` flips
  `consistency.anomaly-cancellation` holds to fails (`ΣY = -1`). The
  hypercharge quadratic and hydrogen `Q = T₃ + Y` still hold as
  five-field identities, so those P3N cells are not a unit flag and not
  the measured `generations` knob. Mutants stay `standard-model`.
  Mutants are not installed, not journaled, and not Canonical or P4.
  Catalog d² hash unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4. Verified: IR round-trip; set missing_e_r is unknown;
  hypothesize standard-model; live Weyl content restored; encode pin
  `860f037bdf4e717007487d9539836f5201adc6d456dc475f05e2e8470781013d`.

- **Debye 2D continuum is an IR mutation**
  (`debye-solid`, `add-2d`). 3D `ω²` is the live encoding; appending
  `g(w) = w` flips `thermo.debye-t3` holds to fails. Freeze-out still
  holds (`C_V ∝ T² → 0`), so the third-law cell is not a unit flag and
  not Einstein freeze-out. That is not a knob. `spectrum` and `quantum`
  stay knobs. Mutants stay `debye-solid`. `einstein-solid` and
  `dulong-petit` have no package. Mutants are not installed, not
  journaled, and not Canonical or P4. Catalog d² hash unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified: IR
  round-trip; set two_d is unknown; hypothesize debye-solid; live 3D
  ω² restored; encode pin
  `dd817e70efdc2efede016101efe3e7b88558cd95f8260b30fc9a130301892b16`.

- **SU(5) missing 10 is an IR mutation**
  (`su5-gut`, `add-missing-10`). Complete `5bar + 10` is the live
  encoding; appending `missing 10` flips `gut.sm-embedding` holds
  to fails. `Tr Q = 0` and GUT-scale `3/8` still hold as SU(5)
  generator identities, so the P3N cell is not a unit flag and not
  MSSM matter. That is not a knob. `supersymmetric` stays a knob.
  Mutants stay `su5-gut`. Mutants are not installed, not journaled,
  and not Canonical or P4. Catalog d² hash unchanged. Unique-vacuum
  graph id unchanged. P3N count stays 4. Verified: IR round-trip;
  set missing_10 is unknown; hypothesize su5-gut; live 5bar + 10
  restored; encode pin
  `fc8614b387c901cc2806fbf456e05d5221131de9cb0d5205e5e4e7ea6a10309e`.

- **Olbers tired light is an IR mutation**
  (`olbers-static`, `add-tired-light`). Inverse-square Euclidean
  shells are the live encoding (`dF = rho dr`); appending
  `tired light` uses `dF ∝ e^{-Hr/c} dr` and flips
  `astro.shell-cancellation` holds to fails. The energy integral
  converges so `astro.sky-finite` fails to holds, while covering
  `τ = n σ R` still diverges so `astro.night-sky-dark` stays fails.
  The residual is `dF/dr(2r)/dF/dr(r) = e^{-0.1}` at `r = 0.1 c/H`
  and is evidence, not a unit flag and not Hubble dimming. That is
  not a knob. `finite_age` and `expanding` stay knobs.
  `olbers-horizon` has no package. Mutants stay `olbers-static`.
  Mutants are not installed, not journaled, and not Canonical or P4.
  Catalog d² hash unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4. Verified: IR round-trip; set tired is unknown;
  hypothesize olbers-static; live Euclidean shells restored; encode
  pin
  `dc1ea0aa82ee79cda7ab53071e43ccb40b56c77a609fc948a8b194864994ffd2`.

- **Turing-machine halt oracle is an IR mutation**
  (`turing-machine`, `add-oracle`). The unrelativized machine is the
  live encoding (`tm`); appending `oracle halt` decides the RE halt
  set and flips `comp.halts` undecidable to holds. Turing completeness
  still holds. The residual is that unrelativized halt is RE-complete
  (`0'` decides it) and is evidence, not a tape simulator. That is
  not a knob. `tape_bound` stays a knob and still restores decidability
  by making the machine a finite automaton. `nondeterministic` stays a
  knob. Halt names the unrelativized-TM domain. Mutants stay
  `turing-machine`; they are not a silent combinational-circuit
  install. Mutants are not installed, not journaled, and not Canonical
  or P4. Catalog d² hash unchanged. Unique-vacuum graph id unchanged.
  P3N count stays 4. Verified: IR round-trip; set oracle is unknown;
  hypothesize turing-machine; live unrelativized TM restored; encode
  pin
  `63961d0b197deadfeb9fbbbfbf8c7c4b27f5d83a29e5e7bc75e66dbab076332f`.

- **De Rham coboundary identity is an IR package**
  (`de-rham`, `add-sign-flip`). The catalog polynomial
  `(b − a) − (c − a) + (c − b)` is the live encoding, with
  `lean_ref` pointing at Physlib `d_squared_zero`. Flipping the
  first minus fails `dec.d-squared-zero` from that encoding. The
  residual is 2 at `(a,b,c)=(1,0,0)` and is evidence, not the
  encoding. That is not a knob. Poincaré, Betti, and Hodge still
  follow the `shape` knob. Mutants stay `de-rham`; they are not a
  silent Maxwell install. Mutants are not installed, not
  journaled, and not Canonical or P4. Catalog d² hash unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified:
  IR round-trip; set sign_flip is unknown; hypothesize de-rham;
  live coboundary restored; encode pin
  `187ee7fd592ffb31a1e5f31fea50d158f7b67bd97f6fbf292c139683445006a6`.

- **Planck Wien occupation is an IR mutation**
  (`planck`, `add-wien`). Planck–Bose is the live encoding
  (`mode planck-bose`); appending `mode wien` uses ⟨E⟩ = hν e^{−x}
  and flips `thermo.rj-ir-limit` holds to fails. The residual is
  |u − u_RJ|/u_RJ ≈ 0.99 at hν = 0.01 kT and is evidence, not the
  encoding. That is not a knob. UV-finite, Stefan–Boltzmann T⁴, and
  a spectral peak still hold on the Wien fork. `quantum` stays a
  knob and still restores the ultraviolet catastrophe. IR
  correspondence keeps the catalog `hν = 0.01 kT` domain. Mutants
  stay `planck`; they are not a silent `rayleigh-jeans` install.
  `rayleigh-jeans` has no package. Mutants are not installed, not
  journaled, and not Canonical or P4. Catalog d² hash unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified:
  IR round-trip; set wien is unknown; hypothesize planck; live Bose
  restored; encode pin
  `7f7e69662ab0960948a1dc7c965078eddda2687e31ea7eebfdc2ab93aa69807b`.

- **SR truncated binomial γ is an IR mutation**
  (`special-relativity`, `add-binomial-gamma`). Exact Lorentz is the
  live encoding (`boost lorentz`); appending `boost binomial-gamma`
  uses γ = 1 + β²/2 and flips `sr.invariant-interval` and
  `sr.energy-momentum-invariant` holds to fails. The residual is
  γ_L − γ_bin ≈ 0.07 at β = 0.6 and is evidence, not the encoding:
  β → 0 recovers Lorentz and the interval cell still fails. That is
  not a knob. Velocity composition still holds (Einstein addition).
  `absolute_time` stays a knob and still flips all three claims.
  Interval keeps the catalog Minkowski domain. Mutants stay
  `special-relativity`; they are not a silent Galilean install.
  Mutants are not installed, not journaled, and not Canonical or P4.
  Catalog d² hash unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4. Verified: IR round-trip; set binomial_gamma is
  unknown; hypothesize special-relativity; live Lorentz restored;
  encode pin
  `4e8c15ecdfc2a60f3bf481898c7cb3e852d22ab9884f377b7728662c3f830c1e`.

- **GR quadratic curvature is an IR mutation**
  (`general-relativity`, `add-r-squared`). Einstein-Hilbert is the live
  encoding (`action einstein-hilbert`); appending `action r-squared`
  makes the classical action not unique Einstein gravity plus Λ and
  flips `predictivity.unique-vacuum` holds to fails. The residual is
  ξ = 1 and is evidence, not the encoding: ξ → 0 recovers
  Einstein-Hilbert and the cell still fails, including in D=5 where
  solar tests are inapplicable and at Λ = 0. That is not a knob.
  Eddington and Mercury still hold (R² is not a 3GM Binet term).
  `dim` / `cosmological_constant` stay knobs. Unique-vacuum still names
  classical Einstein-Hilbert plus Λ. Mutants stay
  `general-relativity`; they are not a silent Newton or Standard-Model
  install. Mutants are not installed, not journaled, and not Canonical
  or P4. Catalog d² hash unchanged. Unique-vacuum graph id unchanged.
  P3N count stays 4. Verified: IR round-trip; set r_squared is unknown;
  hypothesize general-relativity; live Einstein-Hilbert restored;
  encode pin
  `8e99553456fa93c2774e4021eb87bb4dd0547f457cf549ec4bf11859313f7be0`.

- **Wilson fundamental Higgs is an IR mutation**
  (`wilson-u1`, `wilson-su2`, `wilson-su3`, `add-higgs`). Unimproved
  1×1 plaquettes are the live encoding (`wilson-plaquette 1x1`);
  appending `higgs fundamental` screens static charges and flips
  `gauge.confining` holds to fails. The residual is a Yukawa mass
  m_H = 1 in lattice units and is evidence, not the encoding: v → 0
  recovers pure-gauge confinement and the cell still fails, including
  SU(N) at weak coupling where the live encoding still holds. That is
  not a knob. Locality still holds (Higgs is not a 2×1 rectangle).
  The gauge-action strong-coupling area law still holds at default β.
  Rectangle 2×1 remains a separate locality fork (`add-rectangle`).
  `dimension` / `beta` / `sites_per_side` stay knobs. Confining names
  pure Wilson gauge field. Mutants stay the same Wilson object; they
  are not a silent install of another group. Mutants are not
  installed, not journaled, and not Canonical or P4. Catalog d² hash
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Live encode pins unchanged. Verified: IR round-trip; set higgs is
  unknown; hypothesize wilson-u1 / wilson-su3; live 1×1 restored;
  encode pins
  `d9644435e8775eeb95d5e81638ad61a589686d65ff6929caf0ec3c2769d4423a`
  (U(1)),
  `32f36c4b5c3dc442b1c1fa970c1949c12fd0601b640f6c784d2317fcb742897a`
  (SU(2)),
  `03bd82af34a6e36ee04985c243a0e2a35ab9fe56a1b28d3ad0bb63ea8461d8d3`
  (SU(3)).

- **Linear-medium Pasteur chirality is an IR mutation**
  (`linear-medium`, `add-chiral`). Isotropic linear D = εE, B = μH is
  the live encoding (`constitutive isotropic-linear`); appending
  `constitutive chiral` makes circular birefringence n_L ≠ n_R and
  flips `em.constitutive-linear` holds to fails. The residual is
  2κ = 0.2 and is evidence, not the encoding: κ → 0 recovers a unique
  index and the cell still fails. That is not a knob. Gauss and
  charge conservation still hold. Tellegen magnetoelectric mixing
  remains a separate n₊ ≠ n₋ fork (`add-tellegen`). `epsilon_r` /
  `mu_r` stay knobs. Constitutive names isotropic linear D = εE,
  B = μH. Mutants stay `linear-medium`; they are not a silent
  Maxwell-vacuum or ohm-circuit install. Mutants are not installed,
  not journaled, and not Canonical or P4. Catalog d² hash unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Live encode
  pin unchanged. Verified: IR round-trip; set chiral is unknown;
  hypothesize linear-medium; live isotropic-linear restored; encode pin
  `35df991eb0911875613084efff07327ed6821b5580bfbccb85dd08387c3722eb`.

- **Landauer Maxwell demon is an IR mutation**
  (`landauer-engine`, `add-demon`). The kT ln2 bound is the live
  encoding (`erase kT ln2`); appending `erase demon` makes the
  encoding dissipate 0 while bits are still erased and flips
  `info.landauer-cost` holds to fails. The residual is the unpaid
  N kT ln2 floor (~2.87e-21 J at 300 K) and is evidence, not the
  encoding: T = 0 makes that floor 0 and the cell still fails. That
  is not a knob. Thermodynamic freedom still fails (bits are erased).
  Dropped ln2 remains a separate cost fork (`add-kt`). `reversible` /
  `bits_erased` / `temperature_k` stay knobs. Cost names kT ln2
  Landauer bound. Mutants stay `landauer-engine`; they are not a
  silent Turing-machine install. Mutants are not installed, not
  journaled, and not Canonical or P4. Catalog d² hash unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Live encode
  pin unchanged. Verified: IR round-trip; set demon is unknown;
  hypothesize landauer-engine; live kT ln2 restored; encode pin
  `94e8b44c1e141f6e4cbff91a409b805361e5fe00a925121348b62cdbc3e187a9`.

- **Combinational multi-driven net is an IR mutation**
  (`combinational-circuit`, `add-contention`). Unique NAND drivers are
  the live encoding (`nand 0 1 -> 2`); appending `nand 0 0 -> 2` puts a
  second driver on the same wire and flips `comp.deterministic` holds
  to fails. The truth-table disagreement is 0.25 (NAND vs NOT on 1 of
  4 rows) and is evidence, not the encoding: two identical NANDs agree
  everywhere and the cell still fails. That is not a knob. Acyclicity
  and combinational halting still hold on the mutant. Feedback remains
  a separate cycle fork (`add-feedback`). `turing-machine`
  `nondeterministic` stays a knob. Determinism names unique NAND
  drivers; TM determinism stays encoding-wide. Mutants stay
  `combinational-circuit`; they are not a silent Turing-machine
  install. Mutants are not installed, not journaled, and not Canonical
  or P4. Catalog d² hash unchanged. Unique-vacuum graph id unchanged.
  P3N count stays 4. Live encode pin unchanged. Verified: IR
  round-trip; set contention is unknown; hypothesize
  combinational-circuit; live unique-driver netlist restored; encode
  pin
  `762aa72d9eace0c61026eca6ebf71b37f26608797a6786c60b92ba06af4ad8ea`.

- **Newtonian Yukawa potential is an IR mutation**
  (`newtonian-gravity`, `add-yukawa`). Inverse-square Binet is the live
  encoding (`binet inverse-square`); appending `potential yukawa` makes
  the impulse-approximation Soldner factor μR K₁(μR) equal 0.601907 at
  grazing μR = 1 and flips `gr.newton-half-deflection` holds to fails.
  The residual 1 − μR K₁ is 0.398 and is evidence, not the encoding:
  μ → 0 recovers Newton and the cell still fails. That is not a knob.
  Eddington and Mercury still fail (Yukawa is not GR). Schwarzschild
  3GM u² remains a separate Eddington/Mercury fork (`add-schwarzschild`).
  `general-relativity` keeps `dim`. Half-angle names inverse-square
  Binet rhs; GR solar cells stay encoding-wide. Mutants stay
  `newtonian-gravity`; they are not a silent GR install. Mutants are
  not installed, not journaled, and not Canonical or P4. Catalog d²
  hash unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Live encode pin unchanged. Verified: IR round-trip; set yukawa is
  unknown; hypothesize newtonian-gravity; live inverse-square restored;
  encode pin
  `e6e7c4222c571adcf6f526a27ab5e0572fb41d92361c7f3ce393e71e23184078`.

- **Dirac next-nearest hopping is an IR mutation**
  (`dirac-fermion`, `add-next-nearest`). Nearest-neighbour naive hopping
  is the live encoding (`dirac naive`); appending `dirac nnn` includes
  distance-2 hopping and flips `field.local` holds to fails. The sampled
  residual max |c sin(2ka)/a| is 0.5 on the default lattice and is
  evidence, not the encoding: on N = 4 every lattice mode has sin(2ka) = 0
  and the cell still fails. That is not a knob. Doubling still fails
  (sin(ka) and sin(2ka) share the Brillouin-edge zero). Wilson r remains
  a separate doubling fork (`add-wilson`). `sites` / `mass` / `spacing`
  stay knobs. Locality names nearest-neighbour 1D lattice Dirac;
  Klein–Gordon locality stays nearest-neighbour 1D periodic lattice.
  Mutants stay `dirac-fermion`; they are not a silent Klein–Gordon
  install. Mutants are not installed, not journaled, and not Canonical
  or P4. Catalog d² hash unchanged. Unique-vacuum graph id unchanged.
  P3N count stays 4. Live encode pin unchanged. Verified: IR round-trip;
  set next_nearest is unknown; hypothesize dirac-fermion; live naive
  hopping restored; encode pin
  `62ea25b78eaf5a7d934db096943e401135acf490c4594fc8a0621478581a521a`.

- **Ohm-circuit unlumped mesh flux is an IR mutation**
  (`ohm-circuit`, `add-flux`). Lumped Kirchhoff voltage is the live
  encoding (`branch R 0 1`); appending `loop dPhi/dt` makes the Faraday
  residual of ∮E·dl + dΦ/dt equal dB/dt × L² = 0.01 and flips
  `em.faraday` holds to fails. That is not a knob. KCL and the
  quasi-static cell still hold on the mutant. Transmission-line delay
  remains a separate KCL fork (`add-tline`). `frequency_hz` stays a
  knob and still flips `em.quasi-static-valid`. Faraday names lumped
  Kirchhoff voltage; Maxwell Faraday stays source-free homogeneous
  dF=0; linear-medium Faraday stays encoding-wide. Mutants stay
  `ohm-circuit`. Mutants are not installed, not journaled, and not
  Canonical or P4. Catalog d² hash unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Live encode pin unchanged. Verified:
  IR round-trip; set flux is unknown; hypothesize ohm-circuit; live
  lumped KVL restored; encode pin
  `fb14d2c8a8cf2c51fe67c2f334a9307860c6ebb5cfbeca1c35467d61f1387af1`.

- **Bell-test PR-box correlator is an IR mutation**
  (`bell-test`, `add-pr-box`). Hilbert-space CHSH is the live encoding
  (`state singlet`); appending `correlator pr-box` makes the CHSH
  combination of `E = (−1)^{xy}` equal 4, which exceeds Tsirelson's
  `2√2` and flips `quantum.tsirelson-bound` holds to fails. That is
  not a knob. Bell violation still holds (`S = 4 > 2`). The operator
  correlator fails (PR E is not `−cos(a−b)`). Product ket remains a
  separate Bell-violation fork (`add-product`). `visibility` stays a
  Werner mixedness knob and still flips `quantum.bell-violation` on
  the singlet. Tsirelson names Hilbert-space CHSH (Tsirelson 2√2).
  Mutants stay `bell-test`. Mutants are not installed, not journaled,
  and not Canonical or P4. Catalog d² hash unchanged. Unique-vacuum
  graph id unchanged. P3N count stays 4. Live encode pin unchanged.
  Verified: IR round-trip; set prbox is unknown; hypothesize
  bell-test; live singlet restored; encode pin
  `4a54aa1db88b053ef04a53593732c435331a71dcc0f8ad3749e7cbb6786990dc`.

- **Maxwell vacuum Proca mass is an IR mutation**
  (`maxwell-vacuum`, `add-proca`). Massless Coulomb Gauss is the live
  encoding; appending `proca m2 A` makes the Coulomb residual of
  ∇·E + m²φ the Proca mass term and flips `em.gauss` holds to fails.
  That is not a knob. Faraday, Ampère, wave-speed, Lorentz, charge
  conservation, and constitutive-linear still hold on the mutant.
  Magnetic current remains a separate Faraday fork (`add-monopole`).
  Mutants stay `maxwell-vacuum`; they are not a silent linear-medium
  install. `epsilon_r` / `mu_r` stay on linear-medium. Maxwell Gauss
  names source-free massless Maxwell; linear-medium Gauss and
  ohm-circuit Gauss stay encoding-wide. Mutants are not installed, not
  journaled, and not Canonical or P4. Catalog d² hash unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Live encode pin
  unchanged. Verified: IR round-trip; set proca is unknown; hypothesize
  maxwell-vacuum; live massless Gauss restored; encode pin
  `f6f47f600c798018d8cea30121512950f0066f56406aa7be34575f4fae034cc3`.

- **Ideal-gas degenerate Fermi statistics is an IR mutation**
  (`ideal-gas`, `add-fermi`). Maxwell–Boltzmann statistics are the live
  encoding (`gas maxwell-boltzmann`); appending `gas fermi` makes the
  Sommerfeld heat capacity `C_V = (π²/2) N k (T/T_F)` instead of
  `(3/2) N k` and flips `thermo.equipartition` holds to fails. That is
  not a knob. Third-law also fails to holds (Fermi S/Nk → 0). The second
  law still holds on the default expansion. `temperature` /
  `volume_ratio` / `particles` stay knobs; `volume_ratio` still flips
  `thermo.second-law`. Bose statistics remain a separate fork
  (`add-bose`). Mutants stay `ideal-gas`; they are not a silent
  Einstein-solid install. Equipartition names classical C_V = 3/2 Nk.
  Mutants are not installed, not journaled, and not Canonical or P4.
  Catalog d² hash unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4. Live encode pin unchanged. Verified: IR round-trip;
  set fermi is unknown; hypothesize ideal-gas; live Maxwell–Boltzmann
  restored; encode pin
  `fb1dbc123bf6f00bc62cb49b4ba5df49a6b22aba81c6d9434e817c714ea18e06`.

- **Klein–Gordon unbounded minus-φ⁴ is an IR mutation**
  (`klein-gordon`, `add-quartic`). The live scalar potential is
  quadratic; appending `potential minus-phi4` makes
  `V(φ) = ½ m² φ² − φ⁴/4` run to −∞ and flips `field.stable` holds to
  fails. That is not a knob. `sites` / `mass_squared` / `spacing` stay
  knobs; `mass_squared: -1` still produces a tachyon. Next-nearest
  locality is still a separate fork (`add-next-nearest`). Stability
  names the quadratic Klein-Gordon potential. Mutants stay
  `klein-gordon`; they are not a silent Dirac install. Mutants are
  not installed, not journaled, and not Canonical or P4. Catalog d²
  hash unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Live encode pin unchanged. Verified: IR round-trip; set quartic is
  unknown; hypothesize klein-gordon; live quadratic potential restored;
  encode pin
  `32b0997d38afb977615e8fc6527ee5d766271e8a31fb5c882912ca740a3b4e4f`.

- **Naive Dirac doubling is an IR mutation**
  (`dirac-fermion`, `add-wilson`). Naive 1D lattice Dirac is the live
  encoding (`dirac naive`); `sin(ka) = 0` at `k = 0` and `k = π/a` so
  `fermion.no-doublers` fails. Appending `dirac wilson` lifts the edge
  copy to mass `m + 2r/a` and flips that cell fails to holds. That is
  not a knob. `sites` / `mass` / `spacing` stay knobs. Mutants stay
  `dirac-fermion`; they are not a silent Klein–Gordon install.
  `mass_squared` stays on klein-gordon. Dirac no-doublers names naive
  1D lattice Dirac; Klein–Gordon locality stays named nearest-neighbour;
  Dirac locality stays encoding-wide (both encodings are nearest-
  neighbour). Mutants are not installed, not journaled, and not
  Canonical or P4. Catalog d² hash unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified: IR round-trip; set wilson
  is unknown; hypothesize dirac-fermion; live naive operator restored;
  encode pin
  `62ea25b78eaf5a7d934db096943e401135acf490c4594fc8a0621478581a521a`.

- **Landauer dropped-ln2 bound is an IR mutation**
  (`landauer-engine`, `add-kt`). `kT ln2` is the live
  encoding (`erase kT ln2`); appending `erase kT` makes the
  encoding energy `N kT` instead of `N kT ln2` and flips
  `info.landauer-cost` holds to fails. That is not a knob.
  Thermodynamically-free still fails on the default irreversible
  one-bit mutant. `temperature_k` / `bits_erased` / `reversible`
  stay knobs; `reversible` still flips `info.thermodynamically-free`.
  Mutants stay `landauer-engine`; they are not a silent
  Turing-machine install. Landauer cost names the kT ln2 bound.
  Mutants are not installed, not journaled, and not Canonical or
  P4. Catalog d² hash unchanged. Unique-vacuum graph id unchanged.
  P3N count stays 4. Verified: IR round-trip; set kt is unknown;
  hypothesize landauer-engine; live kT ln2 restored; encode pin
  `94e8b44c1e141f6e4cbff91a409b805361e5fe00a925121348b62cdbc3e187a9`.

- **Ideal-gas Bose statistics is an IR mutation**
  (`ideal-gas`, `add-bose`). Maxwell–Boltzmann statistics are the live
  encoding (`gas maxwell-boltzmann`); appending `gas bose` makes the
  low-T Bose entropy S/Nk ∝ (T/T_c)^{3/2} vanish and flips
  `thermo.third-law` fails to holds. That is not a knob.
  Equipartition at 300 K and the second law still hold on the mutant.
  Mutants stay `ideal-gas`; they are not a silent Einstein-solid
  install. `temperature` / `volume_ratio` / `particles` stay knobs.
  Ideal-gas third law names classical Sackur–Tetrode; Einstein-solid
  third law stays encoding-wide. Mutants are not installed, not
  journaled, and not Canonical or P4. Catalog d² hash unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified: IR
  round-trip; set bose is unknown; hypothesize ideal-gas; live
  Maxwell–Boltzmann restored; encode pin
  `fb1dbc123bf6f00bc62cb49b4ba5df49a6b22aba81c6d9434e817c714ea18e06`.

- **Maxwell vacuum magnetic current is an IR mutation**
  (`maxwell-vacuum`, `add-monopole`). Homogeneous Faraday is the live
  Bianchi encoding (`maxwell dF=0`); appending `dF = *j_m` makes the
  plane-wave residual of ∇×E + ∂B/∂t + J_m the uniform magnetic
  current and flips `em.faraday` holds to fails. That is not a knob.
  Gauss, Ampère, wave-speed, Lorentz, charge conservation, and
  constitutive-linear still hold on the mutant. Mutants stay
  `maxwell-vacuum`; they are not a silent linear-medium install.
  `epsilon_r` / `mu_r` stay on linear-medium. Maxwell Faraday names
  source-free homogeneous dF=0; linear-medium Faraday and ohm-circuit
  KVL stay encoding-wide. Mutants are not installed, not journaled,
  and not Canonical or P4. Catalog d² hash unchanged. Unique-vacuum
  graph id unchanged. P3N count stays 4. Verified: IR round-trip; set
  monopole is unknown; hypothesize maxwell-vacuum; live homogeneous
  Faraday restored; encode pin
  `f6f47f600c798018d8cea30121512950f0066f56406aa7be34575f4fae034cc3`.

- **Linear-medium Tellegen mixing is an IR mutation**
  (`linear-medium`, `add-tellegen`). The constitutive law is isotropic
  linear (`constitutive isotropic-linear`); appending
  `constitutive tellegen` splits the unique index n = √(ε_r μ_r) into
  n₊ ≠ n₋ and flips `em.constitutive-linear` holds to fails. That is
  not a knob. `epsilon_r` / `mu_r` still flip `em.wave-speed-c` and
  `em.lorentz-invariance` independently. Maxwell vacuum Holds
  encoding-wide (unit medium); ohm-circuit is inapplicable. Mutants
  are not installed, not journaled, and not Canonical or P4. Catalog
  d² hash unchanged. Unique-vacuum graph id unchanged. P3N count
  stays 4. Verified: IR round-trip; set tellegen is unknown;
  hypothesize linear-medium; live constitutive law restored; encode pin
  `35df991eb0911875613084efff07327ed6821b5580bfbccb85dd08387c3722eb`.

- **Newtonian 3GM u² Binet term is an IR mutation**
  (`newtonian-gravity`, `add-schwarzschild`). Inverse-square light and
  orbits are the live Binet rhs (`binet inverse-square`); appending
  `binet 3GM u^2` flips `gr.newton-half-deflection` holds to fails and
  `gr.eddington-deflection` / `gr.mercury-perihelion` fails to holds.
  That is not a knob. Mutants stay `newtonian-gravity`; they are not a
  silent GR install. `general-relativity` keeps `dim`. Mutants are not
  journaled, and not Canonical or P4. Catalog d² hash unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified: IR
  round-trip; set schwarzschild is unknown; hypothesize
  newtonian-gravity; live inverse-square restored; encode pin
  `e6e7c4222c571adcf6f526a27ab5e0572fb41d92361c7f3ce393e71e23184078`.

- **Bell-test product ket is an IR mutation**
  (`bell-test`, `add-product`). The CHSH lab ket is the two-qubit singlet
  (`state singlet`); appending `state product` flips
  `quantum.bell-violation` and `quantum.correlator-from-operators` holds
  to fails. That is not a knob. `visibility` still scales the singlet
  Werner mixture independently. Mutants are not installed, not
  journaled, and not Canonical or P4. Catalog d² hash unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified: IR
  round-trip; set product is unknown; hypothesize bell-test; live
  singlet restored; encode pin
  `4a54aa1db88b053ef04a53593732c435331a71dcc0f8ad3749e7cbb6786990dc`.

- **Ohm-circuit transmission-line delay is an IR mutation**
  (`ohm-circuit`, `add-tline`). Kirchhoff current law is the lumped
  branch netlist (`branch R 0 1`); appending `tline 0 1` flips
  `em.charge-conservation` holds to fails. That is not a knob.
  `frequency_hz` still flips `em.quasi-static-valid` independently.
  Mutants are not installed, not journaled, and not Canonical or P4.
  Catalog d² hash unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4. Verified: IR round-trip; set tline is unknown;
  hypothesize ohm-circuit; live netlist restored; encode pin
  `fb14d2c8a8cf2c51fe67c2f334a9307860c6ebb5cfbeca1c35467d61f1387af1`.

- **Wilson SU(N) 2×1 rectangle is an IR mutation**
  (`wilson-su2`, `wilson-su3`, `add-rectangle`). Same stencil dialect as
  U(1): locality is the unimproved 1×1 Wilson stencil; appending
  `wilson-rectangle 2x1` flips `gauge.local` holds to fails. Exact 2D
  area-law factorization does not apply to the rectangle encoding.
  That is not a knob. Mutants are not installed, not journaled, and not
  Canonical or P4. 4D SU(N) confinement stays a conjecture Holds.
  Catalog d² hash unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4. Verified: IR round-trip; set rectangle is unknown;
  hypothesize wilson-su3; live stencil restored; encode pins
  `32f36c4b5c3dc442b1c1fa970c1949c12fd0601b640f6c784d2317fcb742897a`
  (SU(2)) and
  `03bd82af34a6e36ee04985c243a0e2a35ab9fe56a1b28d3ad0bb63ea8461d8d3`
  (SU(3)).

- **Wilson U(1) 2×1 rectangle is an IR mutation**
  (`wilson-u1`, `add-rectangle`). Locality is the unimproved 1×1
  Wilson stencil; appending `wilson-rectangle 2x1` flips `gauge.local`
  holds to fails. Exact 2D area-law factorization does not apply to the
  rectangle encoding. That is not a knob. Mutants are not installed,
  not journaled, and not Canonical or P4.
  Catalog d² hash unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4. Verified: IR round-trip; set rectangle is unknown;
  hypothesize wilson-u1; live stencil restored; encode pin
  `d9644435e8775eeb95d5e81638ad61a589686d65ff6929caf0ec3c2769d4423a`.

- **Independent from_lab judge is a unique judge op**
  (`physis judge`, `Role::Judge`). Rebuilds `Judgment::from_lab` from live
  evaluator axes and receipts into a JudgmentProjection DAG. Unique-vacuum
  is heuristic failed, not logical proved. Super-K is empirical excluded.
  GQW NLL is statistical computed. GUT-scale 3/8 is numeric certified.
  Poincaré stays logical undetermined. Catalog d² is logical proved only
  after a live dual-check. A forged `projection_hash` cannot mint.
  Not P3S, not Canonical, not P4. Loop judges after encode. Unique-vacuum
  graph id unchanged. P3N count stays 4. Unique-vacuum projection
  `0dadce8d7bfc005efc32e47917f75b4c17ea77900ec9f6592010fd81f0f1ea76`.
  GUT-scale 3/8 projection
  `40c991698dbff52a5614093b98edcc3478a3702ddcb5cc545f9818af4a6448ae`.
  Verified: role gates; journal restore; JSONL cannot mint proved.

- **Independent IR package encode is a unique encoding-auditor op**
  (`physis encode`, `Role::EncodingAuditor`). Parses, round-trips, and
  reconstructs live theory IR packages (`combinational-circuit` NAND
  netlist, `klein-gordon` nearest-neighbour stencil, `wilson-u1` /
  `wilson-su2` / `wilson-su3` 1×1 plaquettes, `ohm-circuit` lumped
  branches, `bell-test` singlet ket, `newtonian-gravity` inverse-square
  Binet rhs, `linear-medium` isotropic-linear constitutive law). A forged
  `package_hash` cannot mint. Refuses theories with no package.
  Hypothesize mutants are not installed. Not P3S, not a kernel receipt,
  not Canonical, not P4. Loop encodes after cite. Unique-vacuum graph
  id unchanged. P3N count stays 4. Combinational-circuit package id
  `762aa72d9eace0c61026eca6ebf71b37f26608797a6786c60b92ba06af4ad8ea`.
  Klein-Gordon package id
  `32b0997d38afb977615e8fc6527ee5d766271e8a31fb5c882912ca740a3b4e4f`.
  Wilson U(1) package id
  `d9644435e8775eeb95d5e81638ad61a589686d65ff6929caf0ec3c2769d4423a`.
  Wilson SU(2) package id
  `32f36c4b5c3dc442b1c1fa970c1949c12fd0601b640f6c784d2317fcb742897a`.
  Wilson SU(3) package id
  `03bd82af34a6e36ee04985c243a0e2a35ab9fe56a1b28d3ad0bb63ea8461d8d3`.
  Ohm-circuit package id
  `fb14d2c8a8cf2c51fe67c2f334a9307860c6ebb5cfbeca1c35467d61f1387af1`.
  Bell-test package id
  `4a54aa1db88b053ef04a53593732c435331a71dcc0f8ad3749e7cbb6786990dc`.
  Newtonian-gravity package id
  `e6e7c4222c571adcf6f526a27ab5e0572fb41d92361c7f3ce393e71e23184078`.
  Linear-medium package id
  `35df991eb0911875613084efff07327ed6821b5580bfbccb85dd08387c3722eb`.
  Verified: role gates; journal restore; hypothesize does not change
  the live package id.

- **Independent SourceRecord cite is a unique provenance-auditor op**
  (`physis cite`, `Role::ProvenanceAuditor`). Rebuilds live dataset
  (PDG, Super-K) and catalog-dossier locators via `SourceRecord::recheck`.
  A forged `source_hash` cannot mint. Not P3S, not a kernel receipt,
  not Canonical, not P4. Unique-vacuum and GUT-scale 3/8 refuse. Loop
  cites after enclose. Catalog d² hash unchanged. Unique-vacuum graph
  id unchanged. P3N count stays 4. Super-K source id
  `26467998781b7d501f90a1dc762d3c16ae636f867ea61152923c505e1ad3bbef`.
  Verified: recheck rejects a tampered
  hash; role gates; journal restore.

- **Independent Ratio enclose is a unique numerical-verifier op**
  (`physis enclose`, `Role::NumericalVerifier`). Parses live
  `CertifiedNumeric` overlay strings as canonical `Ratio` (`3/8`, `0`,
  `-1/2`) and stores a content-addressed `NumericCertificate`. Succeeds
  on the four P3N cells; refuses unique-vacuum, Super-K, GQW NLL, and
  Poincaré. Restore rebuilds from live strings; a forged
  `certificate_hash` cannot mint. Not a kernel receipt, not Canonical,
  not P4. `inspect trust P3N` stays count 4. Loop runs enclose after
  falsify. Catalog d² hash unchanged. Unique-vacuum graph id unchanged.
  GUT-scale 3/8 certificate
  `0967e9f42ec9ff0fd8e29fecc5bb5a3ed9aba4974ac77b0e5217a4bb634ec202`.
  Verified: parse_display rejects `6/16` and `0.23122`; role gates;
  journal restore; P3N count.

- **Correspondence cells name a DomainOfValidity**
  (`thermo.high-t-classical`, `thermo.debye-t3`, `thermo.rj-ir-limit`,
  `gauge.exact-area-law-2d`). High-T is `T/Θ ≥ 8`; Debye T³ is the
  `Θ/20` probe; Rayleigh–Jeans correspondence is `hν = 0.01 kT`; the
  exact area law is 2D plaquette factorization. Dulong–Petit at the
  current T stays encoding-wide. Poincaré, Maxwell's quasi-static copy,
  and GUT `Tr Q` stay encoding-wide. Not Canonical, not P4. Catalog d²
  hash unchanged. Unique-vacuum graph id unchanged. Verified: unit
  domain tests; why prints the regimes; encoding-wide cells stay wide.

- **Klein–Gordon next-nearest coupling is an IR mutation**
  (`klein-gordon`, `add-next-nearest`). Locality is nearest-neighbour on
  a 1D periodic lattice; appending `laplacian nnn` flips `field.local`
  holds to fails. That is not a knob. Mutants are not installed, not
  journaled, and not Canonical or P4. Catalog d² hash unchanged.
  Unique-vacuum graph id unchanged. Verified: IR round-trip; set
  next_nearest is unknown; hypothesize klein-gordon; live stencil
  restored.

- **Evidence graphs persist across `--journal` restore**
  (`JournalEvent::Evidence`). Restore rebuilds the DAG from live
  evaluations. A tampered `graph_hash` cannot mint the snapshot.
  Restore does not journal again. `replay_journal` still certifies only
  `set-knob`. Not Canonical, not P4. Catalog d² hash unchanged.
  Verified: unique-vacuum reconstitutes graph
  `6ee50cdc3de02838465b178b47061d8d5b36d6c135baf40f80988ff640a36bc9`;
  a forged hash is absent from the store; two-process CLI prints the
  same id; dump-only restore does not append.

- **Inverse query over projected judgments**
  (`physis inspect judgment`). `statistical-computed` lists the PDG
  GQW cell; `empirical-excluded` lists Super-K `p→e+π0`;
  `logical-proved` is empty until a dual-checked receipt exists.
  Explorer may observe. Does not mint. Catalog d² hash unchanged.
  Verified: count 1 statistical; Super-K is not that row; prove d²
  appears under logical-proved; unknown labels refuse.

- **PDG mixing angle carries a Gaussian likelihood**
  (`LikelihoodModel::Gaussian`, `Verdict::with_statistical_nll`).
  `gut.weinberg-angle-mz-interval` keeps interval-subset on the empirical
  axis and projects `statistical computed` from the exact NLL of the
  five-decimal GQW centre versus PDG σ = 10^{-5}. Super-K `p→e+π0` stays
  a one-sided hull: `from_lab` without an NLL overlay is still
  `empirical excluded` / `empirical compatible`. Not P3N, not a kernel
  proof, not an LLM score. Catalog d² hash unchanged. Verified: NLL of
  μ is 0 and of μ+σ is 1/2; Super-K compare_gaussian ignores a point;
  why prints `nll`; MSSM NLL is smaller than minimal SU(5); P3N count
  stays 4.

- **Proposers cannot remint their own receipts**
  (`Role::ReplicationAgent`, `Role::EmpiricalAnalyst`).
  `proof-searcher` may `prove` and cannot `reproduce`. `explorer` may
  observe and cannot `score`. `replication-agent` remints in-process
  (still not P4). `empirical-analyst` scores the empirical-target
  fixture. Lab still runs the full protocol. Catalog d² hash unchanged.
  Verified: role unit tests; lab prove-then-reproduce refusal;
  explorer cannot score.

- **Hypothesis search mutates IR packages, not only knobs**
  (`physis hypothesize`, `physis-ir::apply_mutation`).
  `combinational-circuit` is a NAND netlist in a theory package.
  `add-feedback` appends a gate equation; that is not a knob.
  Cycle detection flips `comp.acyclic` holds to fails and takes
  `comp.halts` out of the combinational domain (inapplicable).
  Mutants are not installed, not journaled, and not Canonical or P4.
  Catalog d² hash unchanged. Verified: IR round-trip; set feedback
  is unknown; hypothesize combinational-circuit; live netlist restored.

- **Evidence graphs are a store DAG**
  (`physis evidence <claim>`, `NodeKind::Evidence`). Each FormalClaim
  identity is a Statement node; each theory evaluation is an Evaluation
  parented by that statement; the Evidence root's parents are the
  evaluation ids. A verdict flip is a new graph. Same live lab is the
  same graph id. The snapshot is not deserialized as authority, not
  Canonical, and not P4. Catalog d² hash unchanged. Verified: unique-vacuum
  has 4 statements / 10 evaluations / 1 graph; descendants of the
  string statement exclude the SM evaluation; turning unique_vacuum off
  mints a new graph while the old one remains a descendant of the old
  observer-geometry eval; Super-K and quasi-static still print their
  encoding contract.

- **Unique-vacuum encodings name a DomainOfValidity**
  (`predictivity.unique-vacuum`). String constructions share the
  flux/moduli landscape regime; observer-geometry names the
  unique_vacuum program axiom; GR names classical Einstein-Hilbert
  plus Λ; the SM names the Higgs vacuum given its parameters. They
  remain Asserted (heuristic/conjecture). Not Canonical, not P3N,
  not a kernel proof. Catalog d² hash unchanged. Verified: the four
  hashes stay distinct; string kinds share one hash; evidence prints
  named regimes and not encoding-wide.

- **Super-K p→e+π0 is a Dataset**
  (`sk-2020-p-e-pi0`, Takenaka et al., Phys. Rev. D 102, 112011).
  `gut.proton-lifetime-sk` compares the dim-6 M_GUT^4 lifetime scaling
  to that 90% CL allowed hull. The 2.4×10^34 yr figure is the published
  lower limit, not an invented number. Minimal SU(5) is excluded; MSSM
  dim-6 is compatible. Decade envelope stands in for missing matrix
  elements. Not P3N, not a kernel proof, not dimension-5 operators.
  Domain names p→e+π0 / dim-6 / Super-K 90% CL. Catalog d² hash
  unchanged. Verified: unit tests on the Dataset and SU(5); CLI why /
  set / inspect gap / evidence; P3N count stays 4.

- **Evidence graphs group by FormalClaim, not slug**
  (`physis evidence <claim>`). Distinct statement hashes that share a
  lab id are competing encodings, not one theorem. Competing
  evaluations are listed per identity. Confidence is the derived
  TrustProfile; there is no numeric score, no Canonical, and no P4.
  `predictivity.unique-vacuum` and `em.quasi-static-valid` are the
  load-bearing examples. Catalog d² hash unchanged. Verified: unique
  vacuum has multiple encodings; ohm-circuit names λ, Maxwell stays
  encoding-wide; explorer may observe.

- **Hypothesis search is constrained structural mutation**
  (`physis hypothesize [theory]` probes chosen and fitted knobs for
  scientific-axis diffs and restores). Measured knobs (generations,
  observed_dim) stay frozen: they are nature, not a hypothesis about
  the encoding. Fitted probes are tagged accommodate. Explorer may
  observe; the command does not journal, persist, or mint. The research
  loop uses this search instead of listing unproved catalog slugs.
  Catalog d² hash unchanged. Verified: type-iib finds critical-dimension;
  standard-model does not propose generations; klein-gordon coarse
  spacing is numeric unresolved; knobs restored; explorer permitted.

- **Knob diffs are scientific-axis, not kind-only**
  (`VerdictDiff` records derivation, empirical, and projected judgment
  labels plus the statement hash; `diff_verdicts` emits a row when any
  of those axes move). `set klein-gordon spacing 100` is
  `holds → undecidable` **and** `not-applicable → inconclusive` /
  `logical undetermined → numeric unresolved`, not a failed theorem.
  `set su5-gut supersymmetric true` moves the PDG interval from empirical
  excluded to inconclusive. Pre-axis journals still replay: extra fields
  compare only when the record carries them; a forged empirical string
  does not certify. Set-time judgment uses `from_lab` without a receipt.
  Catalog d² hash unchanged. Verified: unit tests on Klein–Gordon and
  SU(5); legacy JSONL strip; tampered empirical axis; CLI `set`.

- **Claim cannot rebind the lab slug**
  (`Claim.id` is private; `id()` / `id_str()` are the getters).
  A public assignment cannot attach a kernel receipt to a different
  slug. Lemma edges stay public and are not in the statement hash.
  Catalog d² hash unchanged. Verified: compile-fail against assigning
  `Claim.id`. Live prove of catalog d² is still Lean+nanoda. `fmt`,
  `clippy -D warnings`, full suite, CLI.

- **Claim cannot assign class or rebind commitments**
  (`Claim` class, layer, assumptions, domain, and commitments are
  private). Overlays remain `with_commitments`, `with_domain`, and
  `with_assumptions`. The slug and lemma edges stay public. Catalog d²
  hash unchanged. Verified: compile-fail against assigning
  `Claim.class`. Live prove of catalog d² is still Lean+nanoda. `fmt`,
  `clippy -D warnings`, full suite, CLI.

- **Claim cannot rebind the hashed sentence**
  (`Claim.statement` is private; `statement()` is the getter).
  Same-module mutation still cannot keep a stale hash; a public
  assignment cannot rebind a kernel receipt. Catalog d² hash unchanged.
  Verified: compile-fail against `Claim.statement.push_str`. Live prove
  of catalog d² is still Lean+nanoda. `fmt`, `clippy -D warnings`, full
  suite, CLI.

- **Verdict cannot assign CertifiedNumeric**
  (`Verdict` derivation, empirical, semantic, and enclosure fields are
  private). Overlay builders (`with_certified_numeric`,
  `with_cross_checked`, `with_empirical`, `with_intractable`) remain the
  only assignment path. Catalog d² hash unchanged. Verified: compile-fail
  against assigning `Verdict.derivation`. Live prove of catalog d² is
  still Lean+nanoda; `why` of GUT-scale 3/8 is still `numeric certified`
  with enclosure `[3/8, 3/8]`. `fmt`, `clippy -D warnings`, full suite,
  CLI.

- **Claim cannot assign CertifiedNumeric**
  (`Claim` derivation, empirical, and semantic fields are private).
  Constructors tag derivation from class; `CertifiedNumeric` and
  encoding-review overlays live on `Verdict`. Catalog d² hash unchanged.
  Verified: compile-fail against assigning `Claim.derivation` and
  `Claim.semantic`. Live prove of catalog d² is still Lean+nanoda.
  `fmt`, `clippy -D warnings`, full suite, CLI.

- **Verdict is not JSON-mintable**
  (`Verdict` has no Deserialize). Journal diffs still store
  `VerdictKind` only. JSON cannot mint a `certified-numeric` overlay or
  an encoding-review tag. Catalog d² hash unchanged. Verified:
  compile-fail against Verdict Deserialize. Live prove of catalog d² is
  still Lean+nanoda. `fmt`, `clippy -D warnings`, full suite, CLI.

- **Canonical is not a mintable semantic tag**
  (`SemanticAssurance` has no `Canonical` variant; P3S is taken from the
  review store of the live statement hash, not `Verdict.semantic`).
  Encoding review still tops out at adversarially-reviewed. Catalog d²
  hash unchanged. Verified: compile-fail against
  `SemanticAssurance::Canonical`; inspect P3S is empty until review.
  Live prove of catalog d² is still Lean+nanoda. `fmt`,
  `clippy -D warnings`, full suite, CLI.

- **StatisticalJudgment cannot mint computed**
  (`StatisticalJudgment` is a transparent wrapper with a private kind).
  There is no public `Computed` constructor. `from_lab` does not project
  a statistical object for any claim class. A reserved crate-private
  computed value is `statistical computed`, not `logical proved`. Catalog
  d² hash unchanged. Verified: compile-fail against
  `StatisticalJudgment::Computed` and a kind struct literal; from_lab
  never returns Statistical. Live prove of catalog d² is still
  Lean+nanoda. `fmt`, `clippy -D warnings`, full suite, CLI.

- **HeuristicJudgment cannot mint suggestive**
  (`HeuristicJudgment` is a transparent wrapper with a private kind).
  Suggestive / failed are produced only by `from_lab`. A heuristic Holds
  is `heuristic suggestive`, not `logical proved`. Catalog d² hash
  unchanged. Verified: compile-fail against `HeuristicJudgment::Suggestive`
  and a kind struct literal; heuristic Holds is suggestive. Live prove
  of catalog d² is still Lean+nanoda; `why` of GQW at `M_Z` is still
  heuristic. `fmt`, `clippy -D warnings`, full suite, CLI.

- **EmpiricalJudgment cannot mint compatible**
  (`EmpiricalJudgment` is a transparent wrapper with a private kind).
  Compatible / excluded are produced only by `from_lab` from a
  registered empirical overlay. Evaluator Holds with `Untested` stays
  `empirical inconclusive`. Catalog d² hash unchanged. Verified:
  compile-fail against `EmpiricalJudgment::Compatible` and a kind
  struct literal; untested Holds is not compatible. Live prove of
  catalog d² is still Lean+nanoda; `why` of the PDG mixing-angle
  interval is still empirical excluded / inconclusive as encoded.
  `fmt`, `clippy -D warnings`, full suite, CLI.

- **NumericJudgment cannot mint certified**
  (`NumericJudgment` is a transparent wrapper with a private kind).
  A certified enclosure is produced only by `from_lab` from a
  `CertifiedNumeric` Holds. JSON still cannot mint it; a Rust
  `Certified` variant cannot be constructed either. Catalog d² hash
  unchanged. Verified: compile-fail against `NumericJudgment::Certified`
  and a kind struct literal; from_lab CertifiedNumeric Holds is
  `numeric certified` with the display enclosure. Live prove of catalog
  d² is still Lean+nanoda; `why` of GUT-scale 3/8 is still `numeric
  certified`. `fmt`, `clippy -D warnings`, full suite, CLI.

- **Claim statement hash is derived**
  (`Claim::statement_hash` is a getter; there is no stored field and
  no Deserialize). Mutating the English statement cannot keep a stale
  catalog hash attached to a kernel receipt. JSON cannot mint a Claim
  identity. Catalog d² hash unchanged. Verified: compile-fail against
  assigning `statement_hash` and Deserialize; mutating the sentence
  changes the getter; from_claim follows the live sentence. Live prove
  of catalog d² is still Lean+nanoda. `fmt`, `clippy -D warnings`,
  full suite, CLI.

- **LogicalJudgment cannot mint proved**
  (`LogicalJudgment` is a transparent wrapper with a private kind).
  JSON still cannot mint `logical proved`; a Rust `Proved` variant
  cannot be constructed outside `from_lab` either. Catalog d² hash
  unchanged. Verified: compile-fail against `LogicalJudgment::Proved`
  and a kind struct literal; from_lab Holds is not proved until a
  receipt. Live prove of catalog d² is still Lean+nanoda; why after
  prove is still `logical proved`. `fmt`, `clippy -D warnings`, full
  suite, CLI.

- **FormalClaim is not JSON-mintable**
  (`FormalClaim` private fields, no Deserialize). `from_claim` is the
  only constructor and recomputes the statement hash from the live
  sentence: a forged hash on `Claim` is not copied through. JSON cannot
  mint a catalog identity. Catalog d² hash unchanged. Verified:
  compile-fail against FormalClaim literals and Deserialize;
  `from_claim` restores an honest hash. Live prove of catalog d² is
  still Lean+nanoda. `fmt`, `clippy -D warnings`, full suite, CLI.

- **Judgment is not JSON-mintable**
  (`Judgment` and `LogicalJudgment` have no Deserialize). `from_lab`
  projects evaluator plus receipts; JSON cannot mint `logical proved`.
  Evaluator Holds without a dual-checked receipt stays undetermined.
  Catalog d² hash unchanged. Verified: compile-fail against Judgment
  Deserialize; from_lab Holds is not Proved until a receipt. Live
  prove of catalog d² is still Lean+nanoda; why after prove is still
  `logical proved`. `fmt`, `clippy -D warnings`, full suite, CLI.

- **Long-wavelength cells name a DomainOfValidity**
  (`field.dispersion-continuum-limit`, ohm-circuit `em.quasi-static-valid`).
  Encoding-wide "long-wavelength" is a hidden regime. Dispersion is the
  longest non-zero lattice mode, not the Nyquist mode and not the
  Richardson `|k a| < 1` probe. Lumped validity is `λ > 100 ×` circuit
  size; Maxwell's inapplicable copy stays encoding-wide. Catalog d² hash
  unchanged. Verified: those cells are not encoding-wide; `field.stable`
  and Maxwell quasi-static are; `why` names the regime. Live prove of
  catalog d² is still Lean+nanoda. `fmt`, `clippy -D warnings`, full
  suite, CLI.

- **Hodge P2 names a discrete DomainOfValidity**
  (`dec.hodge-harmonic`). Encoding-wide "Hodge theorem" is a hidden
  regime: the cell is combinatorial Laplacian nullity vs coboundary
  `b₁` on a finite simplicial complex, not the smooth Hodge theorem.
  Euler–Poincaré and Poincaré stay encoding-wide. Catalog d² hash
  unchanged. Verified: Hodge is not encoding-wide; Euler and Poincaré
  are; `inspect trust P2` stays count 1; `why` names finite simplicial
  1-cochains. Live prove of catalog d² is still Lean+nanoda. `fmt`,
  `clippy -D warnings`, full suite, CLI.

- **GUT mixing-angle and SM P3N cells name a DomainOfValidity**
  Encoding-wide with a filled boundary or dataset is a hidden regime.
  `gut.weinberg-angle` is unification-scale, not `sin²θ_W(M_Z)`. GQW
  and the PDG interval name `M_Z`. SM anomalies, hypercharge, and
  hydrogen name one generation / hydrogen. Super-K proton lifetime
  stays encoding-wide until Super-K is a Dataset. Verified: those
  cells are not encoding-wide; `why` names the regime; Poincaré, Tr Q,
  and Super-K stay encoding-wide. Catalog d² hash unchanged. `fmt`,
  `clippy -D warnings`, full suite, CLI.

- **Catalog claims name their axioms as AssumptionSet**
  (`IdentitySpec::assumption_set`, `Claim::with_assumptions`). Hidden
  coboundary / Minkowski / Einstein-addition hypotheses are a new
  identity, not a silent `encoding-is-the-model`. Lean kernel axioms
  stay on the receipt. Poincaré does not inherit the catalog coboundary
  set. Verified: named domain without catalog axioms cannot mint;
  `why` lists `discrete-coboundary` before prove. Live prove is still
  Lean+nanoda. `fmt`, `clippy -D warnings`, full suite, CLI.

- **Catalog identities name a DomainOfValidity**
  (`IdentitySpec.domain`). Encoding-wide Physlib forall is not the
  catalog FormalClaim: filling in the regime is a new hash. `why`
  prints regimes and approximations. Poincaré stays encoding-wide.
  Verified: encoding-wide physlib d² cannot mint ExactIdentity;
  live catalog claims are not encoding-wide; `why` shows d² simplex
  coboundary and interval `|β| < 1`. Live prove is still Lean+nanoda
  (new challenge hash: domain is in the FormalClaim). `fmt`,
  `clippy -D warnings`, full suite, CLI.

- **Journal restore binds prove/review to FormalClaim identity**
  (`statement_hash` on prove/review events). Restore remints a prove
  only when the recorded challenge is `Challenge::generate` of the live
  identity, and a review only when the recorded statement hash is that
  identity. A matching slug with unspecified commitments, a wrong
  challenge hash, or a slug-only legacy review line does not mint P3F
  or P3S. Verified: stale-identity journal prove/review stay
  unproved/unreviewed; live hashes still restore. Live d² prove/review
  journal restores P3F+P3S; forged hashes do not. `fmt`,
  `clippy -D warnings`, full suite, CLI.

- **Challenge is generate-only**
  (`Challenge` private fields, no Deserialize). The solver cannot
  construct the obligation it is judged against. `generate` fills the
  Lean type and polynomial from the matching catalog FormalClaim.
  Verified: compile-fail against Challenge literals and Deserialize;
  a generated challenge is hash-consistent; unspecified slug still
  cannot mint ExactIdentity. Live `prove` of catalog d² is still
  Lean+nanoda; challenge hash unchanged. `fmt`, `clippy -D warnings`,
  full suite, CLI.

- **Catalog obligation is the FormalClaim, not the slug**
  (`IdentitySpec::lab_claim`, `lookup_matching`, `bind_catalog`).
  ExactIdentity and encoding review mint only when the live identity is
  the catalog sentence (quantifier, units, constants, conventions,
  Physlib). The same slug with unspecified commitments cannot borrow
  the polynomial, the Lean type, or the dossier. Live theories host
  catalog claims from the spec so hashes cannot drift. Verified:
  unspecified d² ExactIdentity is NoExactIdentity even if the catalog
  polynomial is attached; review is WrongIdentity; live prove/review
  still mint. `fmt`, `clippy -D warnings`, full suite, CLI (live d²
  Lean+nanoda P3F+P3S; identity hash unchanged).

- **P3S keys to statement hash, not slug**
  (`SemanticRecord.statement_hash`, `Lab::semantic_tag`). Encoding
  review mints against the live `FormalClaim`. Changing commitments
  keeps the lab slug and is a new identity: a stale `by_claim` lookup
  is not P3S. `why`, epistemics, inspect, and journal restore match.
  Verified: stale unspecified d² review does not inspect as P3S;
  reviewing the live identity does. `fmt`, `clippy -D warnings`, full
  suite, CLI (`review` identity hash matches `why`).

- **P3F keys to statement hash, not slug**
  (`Lab::has_live_receipt`). A dual-checked receipt proves the live
  `statement_hash`. Changing commitments keeps the lab slug and is a
  new identity: a stale `by_claim` lookup is not P3F, not a closed
  MissingTheorem gap, and not a reproduce. `why` already judged by
  hash; trust, inspect, gaps, the loop, and reproduce now match.
  Verified: stale unspecified d² receipt does not inspect as P3F;
  proving the live identity does. `fmt`, `clippy -D warnings`, suite.

- **FormalClaim identity is first-class**
  (`ClaimCommitments`, `Quantifier`). The statement hash commits to
  quantifiers, units, constants, boundary conditions, conventions,
  theory version, definitions, datasets, and formal-library identity,
  not just English prose. Changing forall/exists, a sign, a unit, a
  constant, or a boundary is a new hash; the lab slug is unchanged.
  Catalog identities (`d²`, interval, composition, mass shell) are
  `forall` in `physlib:unversioned`. Poincaré is not a catalog
  polynomial. `field.second-order-accurate` names `|k a| < 1` as its
  domain. GUT-scale `3/8` commits to the unification-scale boundary;
  the PDG interval cell commits to `pdg-2024-sin2theta`. Lean compiler
  versions stay on the receipt. Verified: unit tests per axis, why
  identity fields, `fmt`, `clippy -D warnings`, full suite, CLI.

- **P3N why is numeric certified**
  (`Judgment::from_lab`, `Verdict.numeric_lo` / `numeric_hi`).
  `CertifiedNumeric` Holds projects `numeric certified` with a display
  enclosure (`[0, 0]`, `[-1/2, -1/2]`, `[3/8, 3/8]`), not `logical
  undetermined`. Evaluator Holds without a receipt stays logical
  undetermined; dual-checked catalog stays `logical proved`; a coarse
  lattice stays `numeric unresolved`. Enclosure strings are display,
  not authority. Not Lean, not P3F, not P4. Verified: unit tests,
  why/inspect, `fmt`, `clippy -D warnings`, full suite, CLI.

- **Coarse second-order is insufficient precision**
  (`field.second-order-accurate`). The O(a²) identity is a long-wavelength
  statement (`|k a| < 1` at the probe). Outside that domain, Richardson
  `p` is not a stencil verdict: the cell is `undecidable` /
  `inconclusive`, the gap is `InsufficientPrecision`, and `why`
  projects `numeric unresolved`. Not a failed theorem, not P3N, not
  Lean, not P3F, not P4. `set klein-gordon spacing 100` is the knob
  diff. Verified: unit tests, inspect/why, `fmt`, `clippy -D warnings`,
  full suite, CLI.

- **GUT Tr Q does not earn P3N**
  (`gut.charge-quantization`). `Tr Q = ΣY` is the gravitational
  `[grav]²U(1)` sum already certified on
  `consistency.anomaly-cancellation`. `Q = T₃ + Y` and `Σ T₃ = 0` make
  that an interpretation, not a second identity. The cell stays
  `executed`. GUT-scale `3/8` stays `CertifiedNumeric` / P3N.
  `inspect trust P3N` lists four cells. Verified: unit tests,
  inspect/why, `fmt`, `clippy -D warnings`, full suite, CLI.

- **GUT-scale 3/8 and Tr Q earn P3N**
  (`gut.weinberg-angle`, `gut.charge-quantization`). Weyl colour and weak
  dimensions are integers. `Tr(T₃²)/Tr(Q²) = 2 / (16/3) = 3/8` in Q, and
  `ΣY = 0`. Those overlays are `CertifiedNumeric` / P3N, not Lean, not
  P3F, not P4, and not Georgi–Quinn–Weinberg running at `M_Z` or the 3%
  band. `inspect trust P3N` lists five cells. Verified: exact Ratio
  tests, inspect/why P3N, `fmt`, `clippy -D warnings`, full suite, CLI.

- **Hydrogen neutrality from T3+Y earns P3N**
  (`empirical.charge-quantization`, `Q = T₃ + Y`). Derived hypercharges
  give `Q_u = 2/3`, `Q_d = −1/3`, `Q_e = −1`; left-handed `T₃+Y` matches
  `−Y` of conjugate singlets; `2 Q_u + Q_d + Q_e = 0`. That overlay is
  `CertifiedNumeric` / P3N, not a catalog lookup of charge-thirds, not
  GUT `Tr Q`, not Lean, not P3F, not P4. A left/right mismatch Fails
  without P3N. `inspect trust P3N` lists three SM cells. Verified: unit
  tests, inspect/why P3N, `fmt`, `clippy -D warnings`, full suite, CLI.

- **Hodge earns P2; Euler–Poincaré does not**
  (`dec.hodge-harmonic`, `Verdict::with_cross_checked`). Laplacian
  nullity versus coboundary `b₁` is a second matrix: forgetting the up
  or down term disagrees. That overlay is `CrossChecked` / P2, not a
  Lean receipt, not P3N, not P3F, not P4. Euler–Poincaré stays
  `executed`: `b₀−b₁+b₂ ≡ V−E+F` is rank-cancellation of the Betti
  formulas, not a second path. Poincaré stays `executed`.
  `inspect trust P2` lists Hodge (count 1). Verified: unit tests,
  inspect/why P2, `fmt`, `clippy -D warnings`, full suite, CLI.

- **Exact SM hypercharge solve earns P3N**
  (`sm.hypercharge-derivation`, `Ratio::checked_sqrt`,
  `Verdict::with_certified_numeric`). Fixing `Y_Q = 1/6`, the four
  anomaly conditions are solved in Q. The `{Y_u, Y_d}` quadratic has
  discriminant `1`, a square; the roots are `{−2/3, 1/3}`. That overlay
  is `CertifiedNumeric` / P3N, not a Lean receipt, not P3F, not P4, and
  not the heuristic 3% GQW band. A non-square discriminant Fails without
  P3N. `inspect trust P3N` lists both SM cells (count 2). Verified:
  Ratio sqrt tests, exact derivation tests, inspect/why P3N, `fmt`,
  `clippy -D warnings`, full suite, CLI.

- **Exact rational square roots**
  (`Ratio::checked_sqrt`, `Div`/`Neg`/`Ord`). `Some` only when numerator
  and denominator are perfect squares. Used by the hypercharge quadratic.
  Not a kernel proof. Verified: unit tests including the SM discriminant
  `1`, `fmt`, `clippy -D warnings`.

- **Exact SM anomalies earn P3N**
  (`consistency.anomaly-cancellation`, `Ratio`, `Verdict::with_certified_numeric`).
  The four chiral gauge sums over one generation are exact rationals
  that vanish, plus an even Witten doublet count. That overlay is
  `CertifiedNumeric` / P3N, not a Lean receipt, not P3F, not P4, and
  not the heuristic 3% GQW band. Green-Schwarz stays encoded. Verified:
  Ratio cubic identity, SM unit tests, inspect/why P3N, `fmt`,
  `clippy -D warnings`, full suite, CLI.

- **Mass shell is a catalog identity**
  (`sr.energy-momentum-invariant`, Physlib `energy_momentum_invariant`).
  The polynomial `(E−βp)² − (p−βE)² − (1−β²)(E²−p²) ≡ 0` is the
  Minkowski bilinear form on 4-momentum: the interval identity with
  `(t,x) → (E,p)`, not a new postulate. Dual-expanded and
  kernel-checked. The typed rest-mass check remains the evaluator.
  Galilean `E' = E`, `p' = p − βE` is not an identity. The claim
  depends on `sr.invariant-interval`. Not Mathlib-scale. Not P4.
  Verified: expander and parse tests, exact mint, Lean+nanoda when
  wired, loop prove, `fmt`, `clippy -D warnings`, full suite, CLI.

- **Super-K prose is not a dataset**
  (`gut.proton-lifetime-sk`, `super_kamiokande_proton_lifetime`).
  The empirical proton-lifetime cell stays `untested` because no
  Super-Kamiokande Dataset is registered. The heuristic
  `gut.proton-decay-viable` still quotes Super-K as M_GUT prose and
  still flips with SUSY. Do not mint a lifetime number to close the
  gap. `inspect gap missing-dataset` lists the cell. Not P4.
  Verified: physis-data None test, GUT and lab tests, `fmt`,
  `clippy -D warnings`, full suite, CLI.

- **Decidable is not feasible**
  (`comp.feasible-decision`, `gap_for`). Circuit equivalence is
  coNP-complete; a bounded tape has a finite configuration graph this
  lab does not enumerate. Those evaluations are `ComputationallyIntractable`,
  not Rice / the halting problem. Unbounded TM feasible-decision is
  inapplicable (the obstruction is computability). Combinational
  `comp.decidable-equivalence` still Holds. No simulator. Not P vs NP
  as holds/fails. Not P4. Verified: computation and gap_for unit tests,
  inspect/gaps lab tests, tape_bound knob diff, `fmt`,
  `clippy -D warnings`, full suite, CLI.

- **Overlap without containment is insufficient precision**
  (`EmpiricalReceipt`, `gap_for`, `gut.weinberg-angle-mz-interval`).
  Compatible now means the prediction interval is a subset of the data
  hull, not merely overlapping. A wide theory envelope that overlaps
  PDG `sin²θ_W(M_Z)` is `inconclusive`, not compatible. Minimal SU(5)
  remains excluded. Turning on SUSY makes the 3% GQW band overlap
  without fitting inside the PDG hull, so `inspect gap
  insufficient-precision` lists the empirical cell; the heuristic GQW
  cell can still hold. The 3% band is the existing heuristic threshold,
  not a remainder certificate, and is not GUT-scale `3/8`. Not P4.
  Verified: interval contains tests, three-way receipt tests, gap_for
  unit test, GUT and lab tests, `fmt`, `clippy -D warnings`, full
  suite, CLI.

- **Einstein composition is a catalog identity**
  (`sr.subluminal-composition`, Physlib `subluminal_composition`).
  The polynomial `(1+uv)² − (u+v)² − (1−u²)(1−v²) ≡ 0` is dual-expanded
  and kernel-checked. `|w|<1` over ℝ remains the evaluator. Galilean
  addition is not an identity. The claim depends on
  `sr.invariant-interval`. Verified: expander and parse tests, exact
  mint, Lean+nanoda when wired, loop prove, `fmt`, `clippy -D warnings`,
  full suite, CLI.

- **Trust gates reproduce and loop-review**
  (`Lab::exec` trust check). Role, then trust, then budget.
  `reproduce` requires P3F and does not spend prove budget on a
  refusal. The research loop will not raise P3S on an unproved catalog
  identity. Standalone `physis review` stays encoding-axis (orthogonal
  to kernel proof). Not P4. Verified: loop-review-requires-P3F,
  reproduce-does-not-spend-budget, `fmt`, `clippy -D warnings`, full
  suite, CLI.

- **Live lemma edges in the gap graph**
  (`Claim::depends_on`, `physis gaps`, `physis why`). Poincaré
  (`dec.closed-equals-exact`) records a live edge to `dec.d-squared-zero`.
  The edge is rebuilt from the theory encoding, never deserialized as
  authority, and is not part of the statement hash. Proving d² flips the
  edge to `have receipt`; Poincaré itself still `needs receipt`. Verified:
  statement-hash invariance unit test, gap-graph lab test, `fmt`,
  `clippy -D warnings`, full suite, CLI.

- **Failing evaluations are not missing theorems**
  (`gap_for`). `MissingTheorem` is only for evaluator-`Holds`
  mathematical / model-internal / phenomenological claims without a
  dual-checked receipt. Combinational `comp.turing-complete` Fails and
  leaves the gap graph; combinational `comp.halts` Holds and still
  `needs receipt` until proved. Verified: gap_for unit test, inspect
  and `physis gaps` lab tests, `fmt`, `clippy -D warnings`, full suite,
  CLI.

- **Live knowledge-gap graph**
  (`physis gaps`, `NodeKind::KnowledgeGap`). Rebuilt from current
  verdicts and receipts, content-addressed in the artifact DAG, never
  deserialized as authority. A proved catalog identity leaves
  `needs receipt`. Explorer may observe the graph. Verified: gap graph
  hash moves after prove, `fmt`, `clippy -D warnings`, full suite, CLI.

- **In-process reproduce is not P4**
  (`physis reproduce`). Requires a prior receipt, remints through
  `verify`, and matches challenge hash plus checkers. The output says
  `not P4 (same binary, same process)`. `inspect trust P4` stays 0.
  Explorer cannot reproduce. Verified: lab test, `fmt`,
  `clippy -D warnings`, full suite, CLI.

- **Agent roles propose; they do not mint**
  (`Role`, `ResearchBudget`, `physis formalize`). Explorer / formalizer /
  proof-searcher / falsifier / reviewer / auditor are processes that
  `exec` may refuse. `formalize` prints the catalog encoding as
  untrusted bytes and does not call `verify`. `--budget prove=N` is a
  research cap: a spent slot cannot mint. `loop` and `replay` stay
  lab-only. Journal restore reconstitutes as the lab, then the live
  command is role-gated. `loop` spends prove/review slots for each
  inner remint; a zero prove budget cannot mint through the cycle.
  P4 is still not assigned. Verified: role permit tests,
  explorer-does-not-mint, formalizer-without-receipt, spent-budget
  second prove, loop-respects-zero-prove-budget, `fmt`,
  `clippy -D warnings`, full suite, CLI.

- **Chosen knobs vs measured ones, and inverse inspect**
  (`ParameterOrigin` on every `KnobSpec`, `physis inspect`). String
  `observed_dim` and SM `generations` are measured; compact radius and
  dilaton are fitted; `euler_number` stays chosen (accommodate, not
  derive). `physis knobs` prints the origin. `physis inspect
  trust|class|origin|gap <value>` lists matching claims or knobs. P3F
  is empty until `prove`. A proved catalog identity leaves the
  `missing-theorem` gap. Information-layer Undecidable (halting, Rice)
  is `logically-undecidable`; other undecidable evaluations are
  `unsupported-formal-primitive`. The TM equivalence claim lives on the
  information layer with halting (Rice). P vs NP stays a scientific open
  problem. CLI `loop` is wired (it was listed in usage but not parsed).
  Verified: origin unit tests, inspect lab test, `fmt`,
  `clippy -D warnings`, full suite, CLI.

- **Derived trust profiles and axiom closure**
  (`physis-core` TrustProfile / Judgment projection, `AxiomLedger`
  defaults, `physis why` / `epistemics`). P3F is earned only by a
  dual-checked verifier receipt, P3S by encoding review, P0 by asserted
  conjectures. Evaluator `holds` is `logical undetermined`, not proved.
  A kernel proof that is still Unreviewed prints a danger note. P4 is
  not assigned from an in-process remint. `propose` cannot mark an axiom
  Accepted. Verified: compile-fail against TrustProfile literals and
  Deserialize, P3F-requires-receipt unit tests, why/epistemics CLI.

- **Level-3 Lean kernel + nanoda dual replay**
  (`formal/physlib`, `physis-proof` Physlib type matching, `physis-verifier`
  lake sandbox). `verify` on clean Lean source whose theorem type matches
  the trusted challenge compiles with Lean 4.34, exports that declaration
  with lean4export 3.1.0, and typechecks the export with nanoda 0.4.16.
  Vacuous `True` is `StatementMismatch`. Export bytes alone stay
  `LeanPipelineNotWired`. Receipts list `propext`, `Quot.sound`, and
  `Classical.choice`. `physis prove` prefers this backend when the tools
  are present; otherwise it still mints `ExactCertificate`. CI installs
  elan and lean4export. MSRV is 1.85 (nanoda / edition 2024). Verified:
  Physlib type match, dual-kernel mint for d² and the Lorentz interval,
  CLI `prove`, `fmt`, `clippy -D warnings`, full suite.

- **Level-3 semantic review and research loop**
  (`physis-semantic`, `physis-agent`, `physis-proof` infix parser).
  `physis review` raises `SemanticAssurance` only from a trusted dossier:
  a precise source record, a second IR encoding whose unexpanded tree
  matches the catalog identity (vacuous `0` is not `d² = 0`), and a
  passing red-team corpus. `Canonical` is never assigned. Journal restore
  re-runs review; it does not deserialize the tag. `physis loop` is one
  scheduled cycle: observe, hypothesize unproved catalog identities,
  prove, falsify, replicate receipts, design, audit, review. Sweep and
  sensitivity probes are not journaled. Lean kernel + nanoda replay is
  still unwired. Verified: compile-fail against SemanticRecord literals
  and Deserialize, vacuous-zero rejection, review restore, loop restores
  `total_dim`, journal tip test uses frozen timestamps (wall-clock `t` is
  in the Merkle payload), `fmt`, `clippy -D warnings`, full suite, CLI.

- **Level-3 dual-check receipts and protocol v2**
  (`physis-proof`, `physis-verifier`, `physis-audit`, `physis-numeric`,
  `physis-provenance`, `physis-store`, `physis-data`, `physis-ir`,
  `physis-constants`, `physis-agent`). `Verified` is not Deserialize, so
  JSON cannot forge a kernel proof. `verify` runs two independent expanders
  on catalogued identities (discrete d squared = 0, Lorentz interval) and
  refuses Lean source that contains axiom, sorry, or admit, or that lacks
  dual kernels. `physis prove` records a receipt; `physis why` then prints
  it. Conjectures cannot be proved by the exact backend. `physis falsify`,
  `sweep`, `branch`, `compare`, `design`, `sensitivity`, and `audit` are
  first-class ops. Provenance rejects textbook slogans. Intervals exclude
  SU(5) 3/8 from the MZ mixing-angle enclosure. The artifact DAG invalidates
  only descendants. Lean kernel + nanoda replay is typed but not wired.
  Verified: compile-fail against Deserialize of Verified, identity mutation
  and sorry/axiom rejection, red-team corpus, prove/why/falsify/sweep/audit
  lab tests, `fmt`, `clippy -D warnings`, full suite, CLI.

- **Level-3 Milestone 1: theorem is no longer an enum**
  (`crates/physis-core` assurance/assumption/artifact/axiom/formal,
  `crates/physis-verifier`, `specs/020-proof-carrying.md`). Former
  `Epistemic::Theorem` is `ClaimClass::ModelInternal` (or mathematical /
  phenomenological) with `DerivationAssurance::Executed`. There is no
  `MachineProved` variant; `Verified<T>` can be minted only inside
  `physis-verifier` (`pub(crate)`). Every claim has a SHA-256 statement
  identity, a non-empty assumption set, and a domain of validity.
  Semantic assurance starts `Unreviewed`. `physis epistemics` prints
  derivation/class/semantic ledgers and an explicit zero kernel-proof
  row. `physis why` prints assumptions and `kernel proof: none`.
  Existing labs still evaluate; they are not silently promoted.
  Verified: compile-fail against `DerivationAssurance::MachineProved` and
  `Verified` struct literals, statement-hash sensitivity, axiom propose
  is unreviewed, no lab verdict is a kernel proof, `fmt`,
  `clippy -D warnings`, full suite, CLI `epistemics` / `why`.

- **Olbers' paradox: infinite static Euclidean starlight on trial**
  (`crates/physis-theory/src/olbers.rs`, `specs/019-olbers.md`, typed
  `Irradiance` / `LuminosityDensity` in `physis-core`). A standing
  19th-century cosmology (`olbers-static`) holds inverse-square shell
  cancellation (`dF = ρ_L dr`) and **fails** a finite sky and a dark night:
  the improper integral is `F ∝ R` (`F(2R)/F(R) = 2` at a Hubble-time probe,
  independent of cutoff) and `τ = n σ R → ∞`. A finite-age horizon
  (`olbers-horizon`) keeps cancellation and holds both observations
  (`F = ρ_L c t`, `τ ~ 10⁻¹⁵`). Hubble dimming is an independent knob:
  `dF = ρ_L dr / (1+z)²` saturates at `ρ_L c/H`, so cancellation fails and
  the sky stays dark. `set olbers-static finite_age true` flips the two
  catastrophe cells; `set olbers-horizon age_yr 1e26` makes `τ ≳ 1` (a
  merely finite universe is not automatically dark). Flux is
  `Qty<Irradiance>`, not energy. Linear Hubble `z = H r/c` is not a full
  FLRW integral; `ρ_L` is a cosmic mean, not the solar neighbourhood packed
  to infinity. Verified: shell ratio 1 vs Hubble fall-off, flux doubling,
  expanding saturation ≈ 2 (static 100), Hubble-time `τ ~ 10⁻¹⁵`, ancient
  `τ ≳ 1`, cutoff independence, typed `ρ_L × length`, lab knob-diffs,
  `fmt`, `clippy -D warnings`, full suite, and the CLI experiment. Bugbot
  follow-up: `note()` and `astro.night-sky-dark` share `verdict_radius`
  (`c t` or `c/H`, never the cutoff), and `τ = n σ R` is a typed
  `Qty<Dimensionless>` rather than an `f64` after `.value()`.

- **Rayleigh–Jeans vs Planck: the ultraviolet catastrophe as a computed theorem**
  (`crates/physis-theory/src/blackbody.rs`, `specs/016-blackbody.md`, typed
  `EnergyDensity` / `StefanBoltzmann` in `physis-core`). Classical equipartition
  of electromagnetic cavity modes is a first-class theory (`rayleigh-jeans`):
  it *holds* `thermo.mode-equipartition` and **fails** finite energy, the T⁴
  law, and Wien's peak — `u(2ν_max)/u(ν_max) = 8` and `u(2T)/u(T) = 2`, not 16.
  Planck's Bose occupation (`planck`) reverses the matrix: UV modes freeze out,
  `u = a T⁴` matches the typed Stefan–Boltzmann constant derived from exact SI
  `h`, `k_B`, `c`, and `λ_max T` matches `hc/(k x)`. `set planck quantum false`
  restores the catastrophe. Verified: Bose integral `π⁴/15`, numeric `u` vs
  analytic `aT⁴`, RJ octupling, Wien temperature-independence, lab knob-diff,
  `fmt`, `clippy -D warnings`, full suite, and the CLI experiment. Bugbot
  follow-up: Planck's `uv-finite` / `stefan-boltzmann` verdicts use the
  improper integral `u_∞ = a T⁴` (not the current cutoff, which can sit in
  the infrared), and Wien's classical failure is a sampled absence of an
  interior peak.

- **Georgi–Quinn–Weinberg: run 3/8 down to M_Z**
  (`crates/physis-theory/src/rge.rs`, `crates/physis-theory/src/gut.rs`,
  `specs/013-grand-unification.md`). The GUT-scale theorem `sin²θ_W = 3/8` is
  a boundary condition. Predicting `sin²θ_W(M_Z)` from `α_em` and `α_s`
  (no measured mixing angle) is a one-loop computation:
  minimal SU(5) lands at ≈0.207 and **fails** `gut.weinberg-angle-mz`; the
  MSSM lands on 0.231 at `M_U ≈ 2×10¹⁶ GeV` and holds as a heuristic.
  `set su5-gut supersymmetric true` flips that cell with unification and
  proton decay. The 3/8 hold evidence no longer quotes the M_Z measurement.
  Verified: SM GQW ≈0.207 vs 0.231, MSSM match, lab knob-diff, `fmt`,
  `clippy -D warnings`, full suite, and `run su5-gut`.

- **2-sphere: χ = 2, b₂ = 1, Poincaré still holds**
  (`crates/physis-theory/src/dec.rs`, `specs/015-exterior-calculus.md`). The
  boundary of a tetrahedron is a first-class `sphere` shape: 4 vertices, 6
  edges, 4 triangles, every edge bordering two faces. It shares `b₁ = 0`
  with the disk (closed = exact) but **holds** `dec.fundamental-class`
  (`b₂ = 1`, `χ = 2`) which the disk, circle, and Klein bottle fail. `set
  de-rham shape sphere` flips that claim fails → holds without touching
  Poincaré — the homology contrast "a 2-cycle, not a 1-hole." Verified:
  tetrahedron Betti numbers and closed-surface check, disk vs S² share b₁
  but not b₂/χ, lab knob-diff, `fmt`, `clippy -D warnings`, full suite, and
  `run de-rham` / `set de-rham shape sphere`.

- **Einstein vs Debye: exponential freeze-out on trial against T³**
  (`crates/physis-theory/src/solid.rs`, `specs/017-einstein-solid.md`, typed
  `HeatCapacity` in `physis-core`). Einstein's 1907 Bose oscillators hold the
  third law but **fail** the observed low-T phonon law: `C_V(2T)/C_V(T)` at
  `Θ/20` is exponential (≫ 8), not 8. Debye's 1912 `ω²` density of states
  (`debye-solid`) reverses that cell: the improper Bose integrals recover
  `π⁴/15` and `4π⁴/15`, and `C_V = (12/5) π⁴ N k (T/Θ_D)³` is a sampled
  theorem. `set einstein-solid spectrum debye` flips `thermo.debye-t3`
  fails → holds without restoring Dulong–Petit. `C_V` is `Qty<HeatCapacity>`,
  not energy. The `thermo` experiment now shares the third-law row across
  ideal gas, Dulong–Petit, Einstein, and Debye. Verified: Debye integrals vs
  `π⁴/15` and `4π⁴/15`, C_V vs dU/dT, Einstein over-freeze at T/Θ=0.2,
  spectrum knob-diff, `fmt`, `clippy -D warnings`, full suite, and the CLI
  experiment.

- **Dulong–Petit vs Einstein: classical solid heat capacity on trial**
  (`crates/physis-theory/src/solid.rs`, `specs/017-einstein-solid.md`). The
  1819 standing theory `C_V = 3 N k` independent of T is a first-class object
  (`dulong-petit`): it holds that axiom and **fails** the third law. Einstein's
  Bose oscillators (`einstein-solid`) reverse the matrix at `T/Θ_E = 0.2`:
  `C_V/(3Nk) ≈ 0.17`, `C_V(Θ_E/40) → 0`. Raising `temperature` to 4000 K
  recovers Dulong–Petit as correspondence without resurrecting the third-law
  failure. The `thermo` experiment now shares the third-law row across ideal
  gas, Dulong–Petit, and Einstein.

- **Newton vs Einstein: Eddington 1.75″ and Mercury 43″ as computed theorems**
  (`crates/physis-theory/src/gravity.rs`, `specs/018-light-deflection.md`).
  Inverse-square gravity is a first-class theory (`newtonian-gravity`): RK4 on
  the Binet equation holds Soldner's `2 GM/(c² R) ≈ 0.87″` and a closed
  Mercury ellipse, and **fails** Eddington's 1.75″ and the 43″/century
  remainder. Schwarzschild geodesics reverse the matrix (`u'' + u = 3 (GM/c²)
  u²` for light; Kepler plus that term for Mercury). `GM/c²` is a typed
  length from the IAU solar `GM`. `set general-relativity dim 5` makes the
  4D solar tests inapplicable. Bugbot follow-up: Mercury's RK4 now starts at
  the Kepler perihelion `u = 1/(a(1−e))`, not `(1+e)/a`.

- **Two-loop RG running for gauge-coupling unification**
  (`crates/physis-theory/src/rge.rs`, `crates/physis-theory/src/gut.rs`,
  `specs/013-grand-unification.md`). `GaugeRunning` now integrates the *coupled
  two-loop* RGEs `d(α_i⁻¹)/dt = −b_i/2π − (1/8π²)·Σ_j b_ij α_j` with a hand-rolled
  RK4 stepper (the standard gauge two-loop matrices `b_ij` for the SM and MSSM),
  finds the `α_1⁻¹ = α_2⁻¹` crossing, and reports the residual `α_3⁻¹` gap and the
  two-loop `M_GUT`. The `gut.coupling-unification` verdict now shows both loops:
  minimal SU(5) misses (≈12% gap at `M_GUT ≈ 10¹³ GeV`), the MSSM meets to a few
  percent at `M_GUT ≈ 3×10¹⁶ GeV`. Verified: two new tests (MSSM unifies far
  better than the SM at two loops; two-loop scale refines the one-loop estimate),
  `fmt`, `clippy -D warnings`, full suite, and the CLI knob diff.

- **Klein bottle: non-orientability and torsion-invisibility over ℝ**
  (`crates/physis-theory/src/dec.rs`, `specs/015-exterior-calculus.md`). Added
  `Complex::klein_bottle()` (a 4×4 grid glued into a torus one way and with a flip
  the other) and a `klein` option to the `shape` knob. It is the sharpest
  homology contrast in the lab: same Euler characteristic as the torus (`χ = 0`)
  but `b₁ = 1` (not 2 — the `ℤ/2` torsion in `H₁(K;ℤ)` is invisible to real
  coefficients) and `b₂ = 0` (not 1 — non-orientable surfaces have no fundamental
  class over ℝ). Added `Complex::is_closed_surface()` (every edge borders exactly
  two triangles) as a validity check for the surface constructions. Verified:
  two new tests (Klein real homology + closed-surface check, and via the knob),
  `fmt`, `clippy -D warnings`, full suite, and the CLI knob diff.

- **Exact 2D SU(N) confinement from the quadratic Casimir**
  (`crates/physis-theory/src/gauge_field.rs`, `specs/010-continuum.md`). Extended
  `gauge.exact-area-law-2d` to the non-abelian `wilson-su2`/`wilson-su3`
  theories. Two-dimensional Yang–Mills is exactly solvable, so the fundamental
  Wilson loop has string tension `σ = (g²/2)·C₂(fund) = (N²−1)/(2β)` (from the
  quadratic Casimir `C₂(fund) = (N²−1)/(2N)`), positive at every finite `β` — 2D
  SU(N) confines at *all* couplings, a **theorem**, in pointed contrast to the 4D
  mass gap, which stays a `conjecture`. `set wilson-su3 dimension 2` flips the
  claim `inapplicable → holds` (σ = 2/3 at β=6). Added `su_casimir_fundamental`
  and `exact_2d_string_tension_sun`. Verified: two new tests (SU(2)/SU(3) confine
  for β ∈ [0.5, 50], Casimir values, 2D-only applicability), `fmt`,
  `clippy -D warnings`, full suite, and the CLI knob diff.

- **Torus homology: a non-trivial `b₁ = 2` check for the DEC machinery**
  (`crates/physis-theory/src/dec.rs`, `specs/015-exterior-calculus.md`). Added
  `Complex::torus()` — a triangulated 3×3 flat torus (9 vertices, 27 edges, 18
  triangles) — and generalized the `de-rham` knob from a `filled` bool to a
  `shape` **choice** (`disk`/`circle`/`torus`, exercising the `Choice` knob
  domain). The torus is a genuine stress test of the Betti/Hodge code beyond the
  minimal disk/circle: all invariants come out to the textbook values `b₀ = 1`,
  `b₁ = 2`, `b₂ = 1`, `χ = 0`, with the Hodge Laplacian's harmonic dimension
  matching `b₁ = 2`. `set de-rham shape torus` flips `closed-equals-exact` to
  `fails` (two independent 1-cycles). Verified: two new tests (torus invariants,
  torus via the knob incl. rejecting an unknown shape) plus updated knob tests,
  `fmt`, `clippy -D warnings`, full suite, and the CLI knob diff.

- **Euler–Poincaré and Hodge theorems on the de Rham complex**
  (`crates/physis-theory/src/dec.rs`, `specs/015-exterior-calculus.md`). Two more
  computed theorems on `de-rham`, each cross-checking a classical invariant two
  independent ways: `dec.euler-poincare` confirms `χ = V−E+F = b₀−b₁+b₂` (Euler
  characteristic from cell counts vs. from Betti numbers), and
  `dec.hodge-harmonic` confirms `dim(harmonic 1-forms) = b₁` by computing the
  nullity of the combinatorial Hodge Laplacian `Δ₁ = d₀d₀ᵀ + d₁ᵀd₁` and matching
  it to the first Betti number (harmonic representatives ≅ cohomology). Added
  `transpose`/`matmul`/`matadd` helpers and `betti2`/`euler_from_cells`/
  `euler_from_betti`/`harmonic1_dim` to `Complex`. Disk: `χ=1`, harmonic dim `0`;
  circle: `χ=0`, harmonic dim `1`. Verified: four new tests, `fmt`,
  `clippy -D warnings`, full suite, and `run de-rham` across the `filled` knob.

- **Discrete exterior calculus: `d²=0` and Betti numbers with type-level grade**
  (new `crates/physis-theory/src/dec.rs`, new `specs/015-exterior-calculus.md`).
  New `de-rham` theory on the mathematical layer. Differential-form grade is a
  Rust type parameter (`Cochain<G0/G1/G2>`), so the exterior derivative `d`
  provably raises grade by one and mixing grades is a compile error (guarded by a
  `compile_fail` doctest, mirroring the `Qty<D>` contracts). Three computed
  theorems: `dec.d-squared-zero` (`d₁∘d₀ = 0` exactly — the `curl grad = 0` /
  `dF=0`-from-`F=dA` identity), `dec.first-betti-number` (holes counted as
  `n_edges − rank(d₁) − rank(d₀)` via Gaussian elimination), and
  `dec.closed-equals-exact` (the Poincaré lemma). A `filled` knob removes the
  triangle's face, turning the disk into a circle: `b₁` jumps `0 → 1` and
  `closed-equals-exact` flips `holds → fails` — topology detected mechanically by
  linear algebra on the coboundary. Registered in `Lab::standard()`. Verified:
  five new tests, a compile-fail doctest, `fmt`, `clippy -D warnings`, full
  suite, and the `run de-rham` + knob diff.

- **CHSH correlator derived from the two-qubit operators**
  (`crates/physis-model/src/quantum.rs`, `crates/physis-theory/src/quantum.rs`,
  `specs/012-quantum-foundations.md`). Added `spin_measurement(θ) = cos θ·σ_z +
  sin θ·σ_x`, a Kronecker product `tensor2`, a 4×4 `apply_mat4`, and
  `expectation4` to `physis-model`. The Bell test's correlator `E(a,b) =
  −cos(a−b)` is now the genuine operator expectation `⟨ψ⁻|σ(a)⊗σ(b)|ψ⁻⟩`,
  verified against the closed form by a new `quantum.correlator-from-operators`
  theorem — so the quantum prediction *emerges from the formalism* rather than
  being an assumed cosine. The CHSH angle convention was updated accordingly
  `(0, 90°, 45°, 135°)`, still saturating `2√2`. Verified: three new
  `physis-model` tests (spin eigenvalues ±1, singlet correlator = −cos Δ) and a
  `physis-theory` test, `fmt`, `clippy -D warnings`, full suite, and `run
  bell-test` (now five holding theorems).

- **CHSH bounds derived, not asserted (Tsirelson by maximization, classical by
  enumeration)** (`crates/physis-theory/src/quantum.rs`,
  `specs/012-quantum-foundations.md`). `quantum.tsirelson-bound` is now computed:
  a brute-force maximization of `|S|` over a 90³ grid of measurement angles finds
  `|S|max ≈ 2.827`, confirming no quantum strategy exceeds `2√2`. A new
  `quantum.local-realism-bound` claim *derives* the classical CHSH bound by
  enumerating all `2⁴` deterministic ±1 strategies and finding the maximum is
  exactly 2 — the threshold falls out of the model rather than being asserted.
  Together they mechanize why `2 < S ≤ 2√2` is the signature of quantum
  nonlocality. Verified: three new tests (Tsirelson maximization, classical
  enumeration, quantum-beats-classical), `fmt`, `clippy -D warnings`, full suite,
  and the `bell` experiment matrix (now four rows).

- **Exact 2D lattice-gauge confinement from the Bessel-function ratio**
  (`crates/physis-theory/src/gauge_field.rs`, `specs/010-continuum.md`). New
  `gauge.exact-area-law-2d` claim on `wilson-u1`: in two dimensions the gauge
  integral factorizes plaquette by plaquette, so the Wilson loop is *exactly*
  `⟨W⟩ = (I₁(β)/I₀(β))^Area` with string tension `σ = −ln(I₁(β)/I₀(β))`. Since
  `0 < I₁/I₀ < 1` for every finite `β`, `σ > 0` always — 2D compact U(1) confines
  at **all** couplings, a theorem (not the strong-coupling approximation). The
  modified Bessel ratio is computed by a convergent, overflow-free series
  (`bessel_i1_over_i0`); the claim is `inapplicable` in D > 2, honestly leaving
  4D as the open mass-gap problem. `set wilson-u1 dimension 2` flips it
  `inapplicable → holds` (σ = 0.807 at β=1, 0.053 at β=10, both > 0). Verified:
  four new tests (Bessel values, 2D confinement at β ∈ [0.1, 50], 2D-only
  applicability, monotone tension), `fmt`, `clippy -D warnings`, full suite, and
  the CLI knob diff.

- **Special relativity: the Galilean→Einstein revolution as one knob**
  (new `crates/physis-theory/src/special_relativity.rs`, new
  `specs/014-special-relativity.md`). New `special-relativity` theory with three
  computed theorems — `sr.invariant-interval` (`s² = (cΔt)² − Δx²` unchanged by
  a boost), `sr.subluminal-composition` (`0.8c ⊕ 0.7c ≈ 0.9615c < c`), and
  `sr.energy-momentum-invariant` (`E² − (pc)² = (mc²)²`, with `pc` and `mc²`
  built from *typed* `Qty<Energy>` so the dimensions are compiler-checked). An
  `absolute_time` knob replaces Lorentz boosts with Galilean ones and flips all
  three `holds → fails` at once — the pre-1905 worldview, mechanized. Added a
  `Momentum` (`M L T⁻¹`) type alias to `physis-core`. Registered in
  `Lab::standard()`. Verified: four new tests, `fmt`, `clippy -D warnings`, full
  suite, and the `run special-relativity` + knob diff.

- **Gauge-coupling unification computed by one-loop RG running**
  (new `crates/physis-theory/src/rge.rs`, `specs/013-grand-unification.md`).
  `gut.coupling-unification` is no longer an asserted sentence: `GaugeRunning`
  runs the three inverse couplings `α_i⁻¹(μ) = α_i⁻¹(M_Z) − (b_i/2π)·ln(μ/M_Z)`
  from the measured electroweak inputs at `M_Z`, fixes the unification point
  from the `α_1`/`α_2` crossing, and **predicts** `α_3(M_Z)`. The minimal SM
  misses by ~40% (`M_GUT ≈ 10¹³ GeV`); the MSSM agrees to ~1% (`M_GUT ≈
  2×10¹⁶ GeV`) — the celebrated near-success, now computed. The
  `supersymmetric` knob switches the beta coefficients `(41/10,−19/6,−7) →
  (33/5,1,−3)`, flipping the verdict `fails → holds`; the same computed `M_GUT`
  feeds the proton-decay verdict (rate ∝ `M_GUT⁻⁴`). Verdicts stay `Heuristic`
  (one loop is approximate) but carry the genuinely computed numbers as
  evidence. New PDG constants `inverse_alpha_em_mz`, `weak_mixing_angle_sin2_mz`,
  `z_mass_gev`. Verified: four new tests (SM misses, MSSM unifies, SUSY beats
  SM, verdict carries computed numbers), `fmt`, `clippy -D warnings`, full
  suite, and the `run su5-gut` + knob diff.

- **SU(5) grand unification: `sin²θ_W = 3/8` and charge quantization derived**
  (new `crates/physis-theory/src/gut.rs`, new `specs/013-grand-unification.md`).
  New `su5-gut` theory (Georgi–Glashow SU(5)) one layer above the SM. Two
  computed theorems from embedding one generation in a complete SU(5) multiplet,
  both from the same `SM_WEYL_FIELDS` table the anomalies use: `Tr Q = 0` forces
  charge quantization (`gut.charge-quantization`), and `sin²θ_W = Tr(T₃²)/Tr(Q²)
  = 3/8` at the unification scale (`gut.weinberg-angle`, with an honesty note
  that the measured `M_Z` value differs by RG running). It is also honest about
  failure: minimal (non-SUSY) SU(5) `fails` both `gut.coupling-unification` and
  `gut.proton-decay-viable` (excluded by Super-Kamiokande), and a
  `supersymmetric` knob flips both `fails → holds` as heuristics. New helpers
  `gut_weinberg_sin2` / `gut_trace_charge` in `standard_model.rs`; registered in
  `Lab::standard()`. Verified: four new tests, `fmt`, `clippy -D warnings`, full
  suite, and the `run su5-gut` + SUSY knob diff.

- **Standard Model hypercharges *derived* from anomaly cancellation**
  (`crates/physis-theory/src/standard_model.rs`, `specs/005-string-critique.md`).
  New `sm.hypercharge-derivation` claim (computed `theorem`): fixing only the
  normalization `Y_Q = 1/6`, the code solves the four anomaly conditions and
  recovers every hypercharge — `Y_L = −1/2`, `Y_e = 1`, and the `[U(1)]³` cubic
  forces `{Y_u, Y_d} = {−2/3, 1/3}` (`StandardModel::derive_hypercharges`). The
  charges are a *consequence* of consistency, not an input — the mechanized form
  of "accommodate vs derive". The fermion content was refactored into a richer
  `WeylField` table (separate SU(3)/SU(2) dimensions), and
  `consistency.anomaly-cancellation` was strengthened to check **all four**
  gauge anomalies (`[SU(3)]²U(1)`, `[SU(2)]²U(1)`, `[grav]²U(1)`, `[U(1)]³`)
  rather than only the two hypercharge sums. Verified: three new tests
  (all-four-anomalies, hypercharges-derived, derivation-claim-holds), `fmt`,
  `clippy -D warnings`, full suite, and `run standard-model`.

### Domain reuse

- **Landauer's principle: a computation ↔ thermodynamics bridge**
  (`crates/physis-theory/src/computation.rs`, `specs/009-computation.md`).
  New `landauer-engine` theory — the first object that reuses substrate from two
  domains at once. Erasing a logical bit dissipates at least `k_B·T·ln2` of
  energy (Landauer 1961); a logically reversible computation erases nothing and
  can be free (Bennett 1973). The bound is **computed from the typed Boltzmann
  constant**, so its units are checked at compile time: `k_boltzmann()` (J/K) ×
  `kelvin(T)` (K) × `N·ln2` is a `Qty<Energy>`. `info.landauer-cost` holds as a
  theorem with the computed floor as evidence (one bit at 300 K = `2.871e-21 J`),
  and `set landauer-engine reversible true` flips `info.thermodynamically-free`
  `fails → holds` — a cross-domain knob → verdict diff. Knobs: `temperature_k`,
  `bits_erased`, `reversible`. Registered in `Lab::standard()`. Verified: five
  new tests (typed-energy bound, erasure forces dissipation, reversibility knob
  removes it, linear scaling in bits and temperature), `fmt`, `clippy -D
  warnings`, full workspace suite, and the CLI knob diff.

- **Quantum foundations: a fifth domain (CHSH Bell test)**
  (`crates/physis-theory/src/quantum.rs`, new `specs/012-quantum-foundations.md`).
  New `bell-test` theory and `physis experiment bell`, giving the `quantum` layer
  its first `Theory`. Puts local realism on trial and mechanically refutes it:
  the CHSH correlator `S = |E(a,b) − E(a,b′) + E(a′,b) + E(a′,b′)|` with
  `E(a,b) = −V·cos(2(a−b))` and the optimal angles computes to `V·2√2`, so at full
  visibility `quantum.bell-violation` holds (`S = 2√2 > 2`, refuting local hidden
  variables) while `quantum.tsirelson-bound` holds (`S ≤ 2√2`) and
  `quantum.born-normalization` is checked from the singlet ket. A `visibility`
  knob turns the violation off below `1/√2`. Registered in `Lab::standard()` and
  the experiments list.

- **Thermodynamics: a fourth domain on the statistical layer**
  (`crates/physis-theory/src/thermo.rs`, new `specs/011-thermodynamics.md`).
  New `ideal-gas` theory (monatomic classical ideal gas) and `physis experiment
  thermo`, populating the previously-empty `statistical` layer. Exercises the
  type system on `Qty<Temperature>` vs `Qty<Energy>` (`k_B·T` is an energy by
  construction). Claims: `thermo.equipartition` (computed `C_v = dU/dT =
  (3/2)Nk`), `thermo.second-law` (computed `ΔS = Nk ln(V_f/V_i) ≥ 0`,
  knob-sensitive — a compression flips it to `fails`), and `thermo.third-law`
  which **fails honestly**: a classical ideal gas has `S ∝ ln T → −∞`, so it
  cannot satisfy the third law without quantum statistics. Adds
  `k_boltzmann` to `physis-model::constants`. Registered in `Lab::standard()`
  and the experiments list.

### Type system

- **More compile-fail contracts** (`crates/physis-core/src/lib.rs`). Added two
  `compile_fail` doctests to the "illegal states are unrepresentable" proof set:
  subtracting a length from an energy, and assigning `mass × time` to a
  `Qty<Energy>`. With the original mass+length example, the type system's
  dimensional safety is now proven by three compile-fail contracts.

### Documentation

- **README refresh for five domains.** The top-level README now presents physis
  as a five-domain laboratory (fundamental physics, electromagnetism,
  computation, thermodynamics, quantum foundations) with a "Five domains, one
  substrate" table of the seven experiments and what each scrutinizes, plus the
  `epistemics` ledger and `--json` structured output — while keeping
  string-critique as the flagship.

### Tooling

- **`physis epistemics` knowledge ledger** (`crates/physis-agent`, `crates/physis`).
  Tallies every verdict across all lab theories by epistemic tag
  (theorem / encoded-fact / conjecture / heuristic / open) and verdict kind — the
  mission's core metric, mechanically counted. Current state: 54 theorems, 75
  encoded-facts, 13 conjectures, 21 heuristics, 11 open (all 11 honestly
  `undecidable`), over 174 claim-evaluations. Composes with `--json`; tested.

- **`--json` structured output** (`crates/physis`). A global `--json` flag makes
  the CLI emit the full typed `Response` as JSON — status, text, and the
  structured `report` (claim matrix) / `diffs` (verdict changes) — so a
  long-horizon agent consumes typed data instead of parsing prose. Works with
  any command and composes with `--journal`. A test asserts the response
  serializes with the matrix and knob-diff content.

- **`physis experiments` command** (`crates/physis-agent`, `crates/physis`). Lists
  the five available experiments (string-critique, em-vacuum, computation,
  field-modes, gauge-lattice) with one-line descriptions, so the growing set of
  labs is discoverable without reading the source. A consistency test asserts
  every listed experiment actually runs.

### M4 — Continuum

- **Charge conservation backed by a computed identity** (`crates/physis-theory/src/em.rs`).
  `em.charge-conservation` (already a theorem) now carries computed evidence: the
  vector-calculus identity `∇·(∇×A) = 0` — the mechanism behind the continuity
  equation `∂ρ/∂t + ∇·J = 0` — is verified numerically (4-point mixed-partial
  stencil) to residual ≈ 0. Test included.

- **Gauss's law verified on a Coulomb field** (`crates/physis-theory/src/em.rs`).
  `em.gauss` is now a **computed theorem** in vacuum: a Coulomb field `E = r̂/r²`
  is checked by central finite differences to have `∇·E = 0` away from the source
  (residual ≲ 1e-4). With this, all three vacuum Maxwell laws (`gauss`,
  `faraday`, `ampere`) are computed theorems; a medium keeps the encoded-fact
  macroscopic forms. Test asserts the residual and both the vacuum-theorem and
  medium-encoded-fact tags.

- **Maxwell homogeneous equations verified numerically** (`crates/physis-theory/src/em.rs`).
  `em.faraday` and `em.ampere` are now **computed theorems** in vacuum: a plane
  wave `E = ŷ cos(x−t)`, `B = ẑ cos(x−t)` is checked by central finite differences
  to satisfy `∂B/∂t + ∇×E = 0` and `∂E/∂t − ∇×B = 0` to residual ≲ 1e-6,
  promoting them from encoded facts. In a medium they revert to encoded facts
  (macroscopic form). Tests assert the residuals and the vacuum epistemic tag.

- **M4 computed strong-coupling area law** (`crates/physis-theory/src/gauge_field.rs`).
  New `gauge.strong-coupling-area-law` claim backed by a real computation: the
  leading strong-coupling string tension `σ = −ln(β/2N²)` (first term of the
  convergent Wilson-loop expansion) for both compact U(1) and SU(N). `σ > 0` is a
  genuine area-law theorem; it fails once the coupling is too weak for the
  expansion. This is the *computed* companion to the physical (heuristic/
  conjecture) `gauge.confining` verdict. Knob-sensitive: `set wilson-su3 beta
  100` flips it holds→fails. Tests check the knob diff and the closed form.

- **M4 second-order accuracy, computed** (`crates/physis-theory/src/continuum.rs`).
  New `field.second-order-accurate` claim for `klein-gordon`: the empirical
  convergence order `p = log2(err(a)/err(a/2))` of the discrete Laplacian at a
  fixed physical wavenumber is *computed* and verified to be ≈ 2 (error ∝ a²).
  This promotes the continuum limit from asserted to a computed numerical-order
  theorem. `set klein-gordon spacing 100` leaves the second-order regime and the
  claim fails. Tests included.

- **M4 non-abelian gauge fields (QED vs QCD)** (`crates/physis-theory/src/gauge_field.rs`).
  New `WilsonSun` theories `wilson-su2` and `wilson-su3` contrast with compact
  U(1) in the `gauge-lattice` matrix. New `gauge.asymptotic-freedom` claim: U(1)
  `fails` (Landau pole), SU(N) `holds` (Gross–Wilczek–Politzer). SU(N) 4D
  confinement `holds` as a **conjecture** — the Yang–Mills mass-gap Millennium
  Problem — while U(1) deconfines in 4D above β≈1.01. Registered in
  `Lab::standard()`; tests cover the asymptotic-freedom contrast and the
  conjecture tag.

- **Architecture: `Theory::world()` returns `Option<World>`** (`framework.rs` and
  all theories). Non-physics domains no longer borrow a physics-shaped
  spacetime. Computation (`combinational-circuit`, `turing-machine`) returns
  `None` and describes itself via the new `Theory::note()`; the scalar field
  reports an honest 1+1 D world instead of 3+1 Minkowski. `physis score` now
  reports a non-physics theory as such rather than faking a physics score, and
  `critique::report_of` handles the optional world. Removes the placeholder-world
  rough edge documented in `specs/009`/`specs/010`. All existing tests stay
  green.

- **M4 gauge field on links** (`crates/physis-theory/src/gauge_field.rs`,
  `specs/010`, `plans/005`). New theory `wilson-u1`: compact U(1) lattice gauge
  theory whose degrees of freedom live on links, with the Wilson plaquette
  action. `gauge.invariant` and `gauge.local` are structural theorems;
  `gauge.confining` is a theorem (encoded) at all β in 2D/3D and a knob-sensitive
  heuristic across the 4D transition near β ≈ 1.01. New `physis experiment
  gauge-lattice`; registered in `Lab::standard()`. `set wilson-u1 beta 2`
  deconfines the 4D theory (Coulomb phase); `set wilson-u1 dimension 3` confines
  at any β. Tests cover both.

- **M4 seed: a field as an actual local object** (`crates/physis-theory/src/continuum.rs`,
  new `specs/010-continuum.md`, `plans/005-m4-continuum.md`). `klein-gordon` is a
  real scalar field on a finite 1D periodic lattice — N sites coupled by a
  nearest-neighbour discrete Laplacian — so its normal modes
  `ω_j² = m² + (4/a²) sin²(π j / N)` are **computed, not tabulated**. Claims are
  theorems of that computation: `field.finite-modes`, `field.dispersion-continuum-limit`
  (long-wavelength error < 5%), `field.stable` (min ω² ≥ 0), `field.causal`
  (group velocity ≤ c), `field.local`. New `physis experiment field-modes`.
  `set klein-gordon mass_squared -1` produces a genuine computed tachyon:
  `field.stable` and `field.causal` both flip to `fails` — the same instability
  notion as the bosonic-string tachyon, here computed from the spectrum.

### M3 — Domain reuse

- **M3 Computation, the third domain** (`crates/physis-theory/src/computation.rs`,
  new `specs/009-computation.md`). A second reuse on the same substrate. New
  theories `combinational-circuit` and `turing-machine` (`tape_bound` knob) and
  the `physis experiment computation` matrix.
  - **The halting problem as an honest `Undecidable`:** the unbounded Turing
    machine's `comp.halts` is `undecidable` (Turing 1936); `comp.decidable-equivalence`
    is `undecidable` (Rice). Setting `tape_bound` to a finite value flips halts,
    turing-complete, decidable-equivalence, and resource-bounded — a finite
    automaton decides halting but loses Turing completeness.
  - Registered in `Lab::standard()`; `run`/`knobs`/`set` work from the CLI.
  - Documented rough edge: `World` is physics-shaped, so computational objects
    use a degenerate placeholder world (generalizing the projection is future
    work).

- **M3 Computation: complexity claims** (`crates/physis-theory/src/computation.rs`).
  Added `comp.p-equals-np`, encoded as `undecidable`/`open` for the Turing
  machine — the lab honestly refuses to record holds/fails for a famous open
  problem — and `inapplicable` for a fixed circuit. Added a `nondeterministic`
  knob to `turing-machine` that flips `comp.deterministic`. Tests cover both.

- **M3 Ohm-circuit control** (`crates/physis-theory/src/em.rs`). Added
  `ohm-circuit`, lumped circuit theory as the quasi-static effective limit of
  Maxwell, to the `em-vacuum` matrix (now three EM objects). Charge conservation
  is Kirchhoff's current law (`holds`), wave propagation is dropped
  (`em.wave-speed-c` inapplicable), and the theory has a preferred frame
  (`em.lorentz-invariance` fails). New `em.quasi-static-valid` claim: the
  `frequency_hz` knob flips it via typed `Qty<Length>` wavelengths when `c/f`
  stops dwarfing the circuit. Registered in `Lab::standard()`.

- **M3 Electromagnetism, the second domain** (`crates/physis-theory/src/em.rs`,
  `crates/physis-model/src/constants.rs`, new `specs/008-electromagnetism.md`).
  Proves the workspace hosts a second science without forking core: `Qty`,
  layers, knobs, claims, verdicts, the `Theory` trait, and the experiment matrix
  all carry classical electromagnetism.
  - New theories `maxwell-vacuum` and `linear-medium` (`ε_r`, `μ_r` knobs) and
    the `physis experiment em-vacuum` matrix.
  - **A real theorem:** typed `ε₀` and `μ₀` constants give `ε₀·μ₀·c² = 1` as a
    `Qty<Dimensionless>` that type-checks and evaluates to 1 — i.e.
    `1/√(ε₀μ₀) = c`, checked, not tabulated.
  - Knob → verdict: a linear medium with `n = √(ε_r μ_r) > 1` slows light and
    selects a rest frame, so `em.wave-speed-c` and `em.lorentz-invariance` fail;
    setting `epsilon_r = 1` flips them back to hold.
  - The experiment builder was generalized (`critique::report_from_rows`,
    `ExperimentReport` gained `rows`/`notes`) so each domain supplies its own
    theory list and claim rows — string-critique and em-vacuum share the
    machinery.

### M2 — Empirical contact

- **Charge quantization computed from the catalog** (`crates/physis-theory/src/standard_model.rs`).
  New `empirical.charge-quantization` claim (a computed `theorem`): the net charge
  of a hydrogen atom (proton `uud` + electron) is summed from the particle
  catalog's electric charges and is exactly zero — `2·Q(u) + Q(d) + Q(e⁻) = 0`
  in units of e/3. Shown on `run standard-model`; test asserts neutrality and the
  theorem tag.

- **SM anomaly cancellation is now computed** (`crates/physis-theory/src/standard_model.rs`).
  `consistency.anomaly-cancellation` for the Standard Model was an `encoded-fact`;
  it is now a **computed `theorem`**. The hypercharges of one generation's
  left-handed Weyl fermions are summed in code: `ΣY = 0` (grav/mixed anomalies)
  and `ΣY³ = 0` (the [U(1)]³ anomaly), with an even SU(2) doublet count (Witten).
  Tests assert both the vanishing sums and the theorem tag.

- **M2.5 Empirical target as data** (`data/empirical-world.json`,
  `crates/physis-theory/src/target.rs`, new `physis score` CLI verb). The
  low-energy requirements (observed 3+1, gauge ⊃ SM, chiral fermions, three
  generations, gravity) live in a checked-in JSON fixture parsed by serde, not a
  hand-written `empirical_target()` body. `score(target, theory)` grades any
  theory's projected `World`; `physis score <theory>` prints the scorecard.
  Honest, illustrative results: heterotic E₈×E₈ 5/5, Standard Model 4/5 (misses
  only gravity), Type IIB 4/5 (no perturbative SM gauge), GR 2/5. Tests cover
  each case. **M2 complete** (all five items, both "Done when" criteria).

- **M2.3 Coupling constants as typed quantities** (`crates/physis-model/src/constants.rs`).
  `fine_structure_constant` (α) and `strong_coupling_mz` (α_s) are
  `Qty<Dimensionless>`; `fermi_coupling` (G_F) is a typed `energy⁻²` quantity
  (SI J⁻²). The dimension is mechanically enforced: `G_F · E · E` type-checks to
  `Qty<Dimensionless>` by construction (test), and multiplying G_F by anything
  else is a compile error. Existence only — running the couplings is M4.

- **M2.2 Neutrino masses as a knob + claim** (`crates/physis-theory/src/standard_model.rs`).
  Added the `neutrino_masses` knob and the `empirical.neutrino-masses` claim.
  The minimal SM (default) now *fails* it honestly ("stores neutrino masses as
  0, but oscillations prove they are nonzero") instead of silently storing 0;
  setting the knob makes it hold. Addresses the `specs/002` "known lie". This
  claim lives on the Standard Model object (visible via `run standard-model`),
  not in the string-critique matrix rows. Knob-diff test included.

- **M2.1 SM embedding verified by code** (`crates/physis-model/src/gauge.rs`).
  `GaugeGroup::sm_embed` no longer asks "is this group literally SU(5)?" via an
  equality table. It now calls `verified_contains_sm`, which walks the standard
  maximal-subgroup chain (E₈ ⊃ E₆ ⊃ SO(10) ⊃ SU(5) ⊃ SM, plus SO(32) ⊃ SO(10)
  and Spin(10) ⊃ SU(5)) and checks the necessary rank and dimension
  inequalities at each step, backed by `SimpleGroup::dimension`/`rank`. The
  `empirical.sm-gauge` verdicts (strings + observer-geometry) now carry the
  verified chain as evidence. Retag policy honored: still `EncodedFact` (the
  check is necessary-but-not-sufficient and the chain is encoded), not
  `Theorem`. Satisfies M2's "SM ⊂ SU(5) verified by code" criterion. Tests in
  `gauge.rs` (chains verify, off-chain groups do not, monotonic rank/dim).

- **M2.4 Typed lengths for hidden extra dimensions** (`crates/physis-theory/src/strings.rs`).
  `empirical.hidden-extra-dims` no longer compares a raw float to a magic
  `1e16`. It now builds a typed effective radius `Qty<Length>` (Kähler volume ×
  √g_s × Planck length via `StringTheory::effective_radius`) and compares it to
  `Scale::Electroweak.typical_length()` — the shortest length we currently
  probe. The threshold is physics (an electroweak probe length), not a
  hand-tuned constant. Satisfies one of M2's "Done when" criteria.

### M1 — String lab

- **Critical dimension derived from the conformal anomaly** (`crates/physis-theory/src/strings.rs`).
  `consistency.critical-dimension` no longer relies only on a table: the critical
  dimension is computed from central-charge cancellation `c_matter·D + c_ghost = 0`
  (`StringKind::worldsheet_central_charge` / `critical_dim_from_anomaly`), giving
  26 for the bosonic string (`1·D − 26`) and 10 for the superstring
  (`(3/2)·D − 15`). The verdict carries the cancellation equation as evidence; a
  test asserts the derived value matches the table for every worldsheet kind, and
  M-theory (11D SUGRA) has no worldsheet anomaly.

- **Unified tachyon stability notion across domains** (`crates/physis-theory/src/strings.rs`).
  `consistency.no-tachyon` is now computed from a string ground-state mass²
  (`α'm²`): bosonic `−1` (tachyon), superstring with GSO `0` (no tachyon),
  GSO off `−1/2` (tachyon returns). It fails exactly when `m² < 0` — the *same*
  criterion as the scalar field's `field.stable` (`min ω² < 0`), so the string
  and continuum labs share one notion of tachyonic instability (per `plans/005`).
  Verdicts unchanged; now computed with the mass² in the evidence.

- **"Why three generations?" made mechanical** (`crates/physis-theory/src/strings.rs`).
  New `euler_number` knob (Calabi–Yau Euler characteristic χ). `empirical.three-generations`
  is `undecidable` until a topology is chosen, then it is the computed topological
  count `|χ|/2`: `set heterotic-e8e8 euler_number 6` → 3 generations (`holds`);
  `euler_number 8` → 4 (`fails`). This encodes the accommodate-vs-derive critique
  directly — string theory can *fit* three generations by choosing χ = ±6, but
  nothing *derives* why χ = ±6; the knob is the unexplained choice. Docs:
  docs/KNOBS, docs/STRING-EXPERIMENT. Knob-diff test included.

- **M1.5 Retire observer-geometry's magic 14** (`crates/physis-theory/src/geometry.rs`).
  The total dimension is now `observed_dim + fibre_dim` rather than a literal
  14. The `total_dim` knob is replaced by `fibre_dim` (default 10). The 10 is
  justified by a toy constraint: Spin(10) acts on a 10-dimensional space, so a
  fibre smaller than 10 cannot host the conjectured gauge group — setting
  `fibre_dim < 10` with `derive_gauge=true` flips `empirical.sm-gauge` to
  `fails`. So `14 = 4 + 10` is the minimal geometric carrier, not a magic
  number. (Knob rename: `observer-geometry total_dim` → `fibre_dim`.)

- **M1.4 Moduli as knobs** (`crates/physis-theory/src/strings.rs`). Added
  `dilaton` (string coupling g_s = e^φ) and heuristic moduli counts `h11`
  (Kähler / size) and `h21` (complex structure / shape). `unique-vacuum` now
  scales as `flux_bits × (h11 + h21)` — zeroing either the flux or the moduli
  collapses the landscape and restores uniqueness — and `hidden-extra-dims`
  uses the effective radius `compact_radius_planck · √g_s`, so the dilaton and
  the Kähler volume can both make extra dimensions visible. Constructors were
  refactored through a shared `StringTheory::new`. Knob→verdict-diff tests:
  moduli drive the landscape, zero flux restores uniqueness, and the
  Kähler volume + dilaton expose extra dimensions.

- **M1.3 Anomaly cancellation as a claim** (`crates/physis-model/src/gauge.rs`,
  `crates/physis-theory/src/{claims,strings,standard_model}.rs`). New matrix row
  `consistency.anomaly-cancellation`. The Green–Schwarz condition is a mechanical
  predicate — `GaugeGroup::gs_anomaly_free_10d`, backed by a real
  `GaugeGroup::dimension` computation — that holds for exactly SO(32) and E₈×E₈
  (dimension 496) and rejects a fake `SU(3)`/`E8`/SM gauge choice. So heterotic
  gauge groups are "not a menu; Green–Schwarz is the reason." Type II and
  M-theory hold for their own (non-GS) reasons; the SM holds (per-generation
  cancellation); the bosonic string is inapplicable (non-chiral); and off the
  critical dimension the claim is `undecidable`. Encoded as `EncodedFact`; a
  typed anomaly polynomial is deferred. Tests in `gauge.rs` (predicate),
  `strings.rs`, and `standard_model.rs`.

- **M1.2 More constructions first-class** (`crates/physis-theory/src/strings.rs`,
  `critique.rs`, `lab.rs`). Type I, Type IIA, heterotic SO(32), and M-theory are
  now constructed and registered in both the default lab and the `string-critique`
  matrix — ten objects in all. Their distinctive verdicts are pinned by tests:
  SO(32) constructions (Type I, heterotic SO(32)) carry an encoded SM embedding
  (`sm-gauge` holds); Type IIA and M-theory have no perturbative GUT group
  (`sm-gauge` undecidable); M-theory sits at critical dimension 11; and every
  default string construction fails `unique-vacuum` as a heuristic.

- **M1.1 Journal replay** (`crates/physis-agent/src/replay.rs`).
  Deterministic replay of a recorded JSONL journal onto a fresh
  `Lab::standard()`. `replay_journal` re-applies every `set-knob`, recomputes
  the verdict diffs, and checks them against what was recorded. A faithful
  replay is a mechanical proof of reproducibility; the CLI `physis replay
  <file.jsonl>` exits non-zero on any mismatch or failed turn.
  - New: `Journal::from_jsonl`, `ReplayReport`/`ReplayStep`, `Command::Replay`.
  - `VerdictDiff` gained `PartialEq`/`Eq` so recorded and recomputed diffs are
    comparable.
  - Tests: round-trip faithfulness, tamper detection, failed-turn reporting,
    empty-journal, and multi-run resume (`crates/physis-agent/src/replay.rs`).

- **M1.6 Session persistence** (`crates/physis/src/main.rs`).
  `physis --journal <file.jsonl>` records a session across process runs. On
  each run the lab loads the file and **restores prior state**
  (`Lab::restore_from_journal`) before applying the new turn, so a multi-run
  session is a single coherent, replayable session rather than a bag of
  independent one-shot diffs.

### Fixed

- **Journals were silently unreadable from disk.** Journal event timestamps
  were `u128`, and serde's internally tagged enum representation drops 128-bit
  integers on deserialization — so every event written by `Journal::file` was
  dropped when the file was reloaded. Timestamps are now `u64` Unix millis
  (range good for ~584 million years). Caught by the M1.1 replay round-trip
  test, not by inspection.
- **Journal events recorded `t: 0`.** `set-knob`/`run`/`experiment` events were
  stamped with `0` instead of the real time. They now use stamping constructors
  (`JournalEvent::set_knob` / `run` / `experiment`).

### Verification

Every change above is covered by `cargo fmt --all -- --check`,
`cargo clippy --workspace --all-targets -- -D warnings`, and
`cargo test --workspace` (all suites green — unit tests + doctests, including
the compile-fail proof that mass cannot be added to length and the replay
round-trip), plus the CLI record→replay loop demonstrated end-to-end.
Agentically reviewed with bugbot; the malformed/empty-journal certification
gap it flagged is fixed and covered by tests.

## Environment

- **Cloud Agent environment** (`.cursor/environment.json`). Repo-managed config
  for the pure-Rust workspace: toolchain from `rust-toolchain.toml`, `install`
  fetches and warm-builds all targets. Validated on a fresh build + fresh
  Cloud Agent.
