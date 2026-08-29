//! Particle spectrum: flavors, quantum numbers, empirical status.
//!
//! The smallest empirically confirmed "stuff" in this workspace is the
//! Standard Model spectrum: quarks, leptons, gauge bosons, Higgs.
//! Gravitons and string excitations are marked hypothetical.

use serde::{Deserialize, Serialize};

/// Named species.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Flavor {
    /// e⁻
    Electron,
    /// μ⁻
    Muon,
    /// τ⁻
    Tau,
    /// ν_e
    NuE,
    /// ν_μ
    NuMu,
    /// ν_τ
    NuTau,
    /// u
    Up,
    /// d
    Down,
    /// s
    Strange,
    /// c
    Charm,
    /// b
    Bottom,
    /// t
    Top,
    /// γ
    Photon,
    /// W⁺
    WPlus,
    /// W⁻
    WMinus,
    /// Z
    Z,
    /// g
    Gluon,
    /// H
    Higgs,
    /// Hypothetical massless spin-2.
    Graviton,
}

impl Flavor {
    /// Symbol.
    pub const fn symbol(self) -> &'static str {
        match self {
            Flavor::Electron => "e⁻",
            Flavor::Muon => "μ⁻",
            Flavor::Tau => "τ⁻",
            Flavor::NuE => "ν_e",
            Flavor::NuMu => "ν_μ",
            Flavor::NuTau => "ν_τ",
            Flavor::Up => "u",
            Flavor::Down => "d",
            Flavor::Strange => "s",
            Flavor::Charm => "c",
            Flavor::Bottom => "b",
            Flavor::Top => "t",
            Flavor::Photon => "γ",
            Flavor::WPlus => "W⁺",
            Flavor::WMinus => "W⁻",
            Flavor::Z => "Z",
            Flavor::Gluon => "g",
            Flavor::Higgs => "H",
            Flavor::Graviton => "G",
        }
    }
}

/// Has this species been observed?
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EmpiricalStatus {
    /// Directly observed.
    Observed,
    /// Required by a theory, not seen.
    Hypothetical,
}

/// One species in a spectrum.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Species {
    /// Flavor tag.
    pub flavor: Flavor,
    /// Spin × 2 (1 = fermion, 0 = scalar, 2 = photon, 4 = graviton).
    pub spin_times_two: u8,
    /// Electric charge in units of e/3 (electron = -3).
    pub charge_thirds: i8,
    /// Color triplet / octet?
    pub colored: bool,
    /// Rest mass in eV/c². Zero means exactly massless in this encoding.
    pub mass_ev: f64,
    /// Observation status.
    pub status: EmpiricalStatus,
}

impl Species {
    fn f(
        flavor: Flavor,
        spin_times_two: u8,
        charge_thirds: i8,
        colored: bool,
        mass_ev: f64,
        status: EmpiricalStatus,
    ) -> Self {
        Self {
            flavor,
            spin_times_two,
            charge_thirds,
            colored,
            mass_ev,
            status,
        }
    }

    /// Electron.
    pub fn electron() -> Self {
        Self::f(
            Flavor::Electron,
            1,
            -3,
            false,
            510_998.95,
            EmpiricalStatus::Observed,
        )
    }

    /// Photon.
    pub fn photon() -> Self {
        Self::f(Flavor::Photon, 2, 0, false, 0.0, EmpiricalStatus::Observed)
    }

    /// Higgs.
    pub fn higgs() -> Self {
        Self::f(
            Flavor::Higgs,
            0,
            0,
            false,
            125_250_000_000.0,
            EmpiricalStatus::Observed,
        )
    }

    /// Hypothetical graviton.
    pub fn graviton() -> Self {
        Self::f(
            Flavor::Graviton,
            4,
            0,
            false,
            0.0,
            EmpiricalStatus::Hypothetical,
        )
    }

    /// True if this is a fermion.
    pub fn is_fermion(&self) -> bool {
        self.spin_times_two % 2 == 1
    }
}

/// A list of species a theory claims exist at low energy.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Spectrum {
    /// Species.
    pub species: Vec<Species>,
}

impl Spectrum {
    /// Empty.
    pub fn empty() -> Self {
        Self { species: vec![] }
    }

    /// Standard Model fermions + gauge bosons + Higgs. No graviton.
    pub fn standard_model() -> Self {
        use EmpiricalStatus::Observed as O;
        let mut s = Self { species: vec![] };
        // leptons
        s.species.push(Species::electron());
        s.species
            .push(Species::f(Flavor::Muon, 1, -3, false, 105_658_375.0, O));
        s.species
            .push(Species::f(Flavor::Tau, 1, -3, false, 1_776_860_000.0, O));
        s.species.push(Species::f(Flavor::NuE, 1, 0, false, 0.0, O));
        s.species
            .push(Species::f(Flavor::NuMu, 1, 0, false, 0.0, O));
        s.species
            .push(Species::f(Flavor::NuTau, 1, 0, false, 0.0, O));
        // quarks (approx current masses)
        s.species
            .push(Species::f(Flavor::Up, 1, 2, true, 2_160_000.0, O));
        s.species
            .push(Species::f(Flavor::Down, 1, -1, true, 4_670_000.0, O));
        s.species
            .push(Species::f(Flavor::Strange, 1, -1, true, 93_400_000.0, O));
        s.species
            .push(Species::f(Flavor::Charm, 1, 2, true, 1_270_000_000.0, O));
        s.species
            .push(Species::f(Flavor::Bottom, 1, -1, true, 4_180_000_000.0, O));
        s.species
            .push(Species::f(Flavor::Top, 1, 2, true, 172_690_000_000.0, O));
        // bosons
        s.species.push(Species::photon());
        s.species
            .push(Species::f(Flavor::WPlus, 2, 3, false, 80_369_200_000.0, O));
        s.species.push(Species::f(
            Flavor::WMinus,
            2,
            -3,
            false,
            80_369_200_000.0,
            O,
        ));
        s.species
            .push(Species::f(Flavor::Z, 2, 0, false, 91_188_000_000.0, O));
        s.species
            .push(Species::f(Flavor::Gluon, 2, 0, true, 0.0, O));
        s.species.push(Species::higgs());
        s
    }

    /// SM plus a hypothetical graviton (closed-string / GR quantum).
    pub fn standard_model_plus_graviton() -> Self {
        let mut s = Self::standard_model();
        s.species.push(Species::graviton());
        s
    }

    /// Any fermions?
    pub fn has_fermions(&self) -> bool {
        self.species.iter().any(|p| p.is_fermion())
    }

    /// Any observed species?
    pub fn has_observed(&self) -> bool {
        self.species
            .iter()
            .any(|p| p.status == EmpiricalStatus::Observed)
    }

    /// Contains a graviton (hypothetical or otherwise).
    pub fn has_graviton(&self) -> bool {
        self.species.iter().any(|p| p.flavor == Flavor::Graviton)
    }

    /// Generation count heuristic: charged leptons.
    pub fn charged_lepton_generations(&self) -> usize {
        self.species
            .iter()
            .filter(|p| matches!(p.flavor, Flavor::Electron | Flavor::Muon | Flavor::Tau))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sm_has_three_charged_leptons() {
        let s = Spectrum::standard_model();
        assert_eq!(s.charged_lepton_generations(), 3);
        assert!(s.has_fermions());
        assert!(!s.has_graviton());
    }
}
