/// Canonical property metadata for fluid sweeps and plotting.
///
/// This module defines all available thermodynamic properties that can be
/// plotted or swept, their display labels, units, and validation.
/// Used by sweep configuration dropdowns and axis labeling.
/// A thermodynamic property that can be swept or plotted
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FluidProperty {
    #[default]
    Temperature,
    Pressure,
    Density,
    Enthalpy,
    Entropy,
    SpecificHeatCp,
    SpecificHeatCv,
    HeatCapacityRatio,
    SpeedOfSound,
}

impl FluidProperty {
    /// All available properties
    pub fn all() -> &'static [FluidProperty] {
        &[
            FluidProperty::Temperature,
            FluidProperty::Pressure,
            FluidProperty::Density,
            FluidProperty::Enthalpy,
            FluidProperty::Entropy,
            FluidProperty::SpecificHeatCp,
            FluidProperty::SpecificHeatCv,
            FluidProperty::HeatCapacityRatio,
            FluidProperty::SpeedOfSound,
        ]
    }

    /// Short display label (for UI selectors)
    pub fn label(&self) -> &'static str {
        match self {
            FluidProperty::Temperature => "Temperature (K)",
            FluidProperty::Pressure => "Pressure (Pa)",
            FluidProperty::Density => "Density (kg/m³)",
            FluidProperty::Enthalpy => "Enthalpy (J/kg)",
            FluidProperty::Entropy => "Entropy (J/kg·K)",
            FluidProperty::SpecificHeatCp => "Cp (J/kg·K)",
            FluidProperty::SpecificHeatCv => "Cv (J/kg·K)",
            FluidProperty::HeatCapacityRatio => "γ = Cp/Cv",
            FluidProperty::SpeedOfSound => "Speed of Sound (m/s)",
        }
    }

    /// Axis label for plotting
    #[allow(dead_code)]
    pub fn axis_label(&self) -> &'static str {
        match self {
            FluidProperty::Temperature => "Temperature (K)",
            FluidProperty::Pressure => "Pressure (Pa)",
            FluidProperty::Density => "Density (kg/m³)",
            FluidProperty::Enthalpy => "Enthalpy (J/kg)",
            FluidProperty::Entropy => "Entropy (J/kg·K)",
            FluidProperty::SpecificHeatCp => "Cp (J/kg·K)",
            FluidProperty::SpecificHeatCv => "Cv (J/kg·K)",
            FluidProperty::HeatCapacityRatio => "γ",
            FluidProperty::SpeedOfSound => "Speed of Sound (m/s)",
        }
    }

    /// Canonical string representation (lowercase, for matching in generator)
    pub fn canonical_name(&self) -> &'static str {
        match self {
            FluidProperty::Temperature => "temperature",
            FluidProperty::Pressure => "pressure",
            FluidProperty::Density => "density",
            FluidProperty::Enthalpy => "enthalpy",
            FluidProperty::Entropy => "entropy",
            FluidProperty::SpecificHeatCp => "cp",
            FluidProperty::SpecificHeatCv => "cv",
            FluidProperty::HeatCapacityRatio => "gamma",
            FluidProperty::SpeedOfSound => "speed_of_sound",
        }
    }

    /// Parse from user string (case-insensitive, supports aliases)
    #[allow(dead_code)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "temperature" | "t" => Some(FluidProperty::Temperature),
            "pressure" | "p" => Some(FluidProperty::Pressure),
            "density" | "rho" | "ρ" => Some(FluidProperty::Density),
            "enthalpy" | "h" => Some(FluidProperty::Enthalpy),
            "entropy" | "s" => Some(FluidProperty::Entropy),
            "cp" | "specific_heat_cp" => Some(FluidProperty::SpecificHeatCp),
            "cv" | "specific_heat_cv" => Some(FluidProperty::SpecificHeatCv),
            "gamma" | "κ" | "heat_capacity_ratio" => Some(FluidProperty::HeatCapacityRatio),
            "speed_of_sound" | "a" | "c" | "sound_speed" => Some(FluidProperty::SpeedOfSound),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn property_roundtrip() {
        for prop in FluidProperty::all() {
            let name = prop.canonical_name();
            let parsed = FluidProperty::from_str(name).expect("Failed to parse canonical name");
            assert_eq!(*prop, parsed);
        }
    }

    #[test]
    fn alias_parsing() {
        assert_eq!(
            FluidProperty::from_str("t"),
            Some(FluidProperty::Temperature)
        );
        assert_eq!(FluidProperty::from_str("rho"), Some(FluidProperty::Density));
        assert_eq!(FluidProperty::from_str("h"), Some(FluidProperty::Enthalpy));
    }
}
