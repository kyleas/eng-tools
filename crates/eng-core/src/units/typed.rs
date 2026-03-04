use uom::si::f64::{
    Acceleration as UomAcceleration, Area as UomArea, DynamicViscosity as UomDynamicViscosity,
    Energy as UomEnergy, Length as UomLength, Mass as UomMass, MassDensity as UomMassDensity,
    MassRate as UomMassRate, Power as UomPower, Pressure as UomPressure, Ratio as UomRatio,
    TemperatureInterval as UomTemperatureInterval,
    ThermodynamicTemperature as UomThermodynamicTemperature, Time as UomTime,
    Velocity as UomVelocity, Volume as UomVolume,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantityKind {
    Pressure,
    Length,
    Area,
    Density,
    DynamicViscosity,
    Force,
    Moment,
    Temperature,
    ThermalConductivity,
    HeatTransferCoefficient,
    MassFlowRate,
    VolumetricFlowRate,
    Dimensionless,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DimensionSignature {
    pub m: i8,
    pub l: i8,
    pub t: i8,
    pub th: i8,
    pub n: i8,
}

impl DimensionSignature {
    pub const fn new(m: i8, l: i8, t: i8, th: i8, n: i8) -> Self {
        Self { m, l, t, th, n }
    }

    pub const fn dimless() -> Self {
        Self::new(0, 0, 0, 0, 0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UnitInput {
    pub value_si: f64,
    pub kind: QuantityKind,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExprInput {
    pub value_si: f64,
    pub signature: DimensionSignature,
}

impl ExprInput {
    pub const fn new(value_si: f64, signature: DimensionSignature) -> Self {
        Self {
            value_si,
            signature,
        }
    }
}

pub type Accel = UomAcceleration;
pub type Area = UomArea;
pub type DynVisc = UomDynamicViscosity;
pub type Energy = UomEnergy;
pub type Length = UomLength;
pub type Mass = UomMass;
pub type Density = UomMassDensity;
pub type MassRate = UomMassRate;
pub type Power = UomPower;
pub type Pressure = UomPressure;
pub type Ratio = UomRatio;
pub type TempInterval = UomTemperatureInterval;
pub type Temperature = UomThermodynamicTemperature;
pub type Time = UomTime;
pub type Velocity = UomVelocity;
pub type Volume = UomVolume;

#[inline]
pub fn pa(v: f64) -> Pressure {
    use uom::si::pressure::pascal;
    Pressure::new::<pascal>(v)
}

#[inline]
pub fn k(v: f64) -> Temperature {
    use uom::si::thermodynamic_temperature::kelvin;
    Temperature::new::<kelvin>(v)
}

#[inline]
pub fn kgps(v: f64) -> MassRate {
    use uom::si::mass_rate::kilogram_per_second;
    MassRate::new::<kilogram_per_second>(v)
}

#[inline]
pub fn m(v: f64) -> Length {
    use uom::si::length::meter;
    Length::new::<meter>(v)
}

#[inline]
pub fn s(v: f64) -> Time {
    use uom::si::time::second;
    Time::new::<second>(v)
}

#[inline]
pub fn unitless(v: f64) -> Ratio {
    use uom::si::ratio::ratio;
    Ratio::new::<ratio>(v)
}

pub mod pressure {
    use super::{QuantityKind, UnitInput};
    pub fn pa(v: f64) -> UnitInput {
        UnitInput {
            value_si: v,
            kind: QuantityKind::Pressure,
        }
    }
    pub fn kpa(v: f64) -> UnitInput {
        UnitInput {
            value_si: v * 1e3,
            kind: QuantityKind::Pressure,
        }
    }
    pub fn mpa(v: f64) -> UnitInput {
        UnitInput {
            value_si: v * 1e6,
            kind: QuantityKind::Pressure,
        }
    }
    pub fn bar(v: f64) -> UnitInput {
        UnitInput {
            value_si: v * 1e5,
            kind: QuantityKind::Pressure,
        }
    }
    pub fn psia(v: f64) -> UnitInput {
        UnitInput {
            value_si: v * 6_894.757_293_168,
            kind: QuantityKind::Pressure,
        }
    }
}

pub mod length {
    use super::{QuantityKind, UnitInput};
    pub fn m(v: f64) -> UnitInput {
        UnitInput {
            value_si: v,
            kind: QuantityKind::Length,
        }
    }
    pub fn mm(v: f64) -> UnitInput {
        UnitInput {
            value_si: v * 1e-3,
            kind: QuantityKind::Length,
        }
    }
    pub fn cm(v: f64) -> UnitInput {
        UnitInput {
            value_si: v * 1e-2,
            kind: QuantityKind::Length,
        }
    }
    pub fn inch(v: f64) -> UnitInput {
        UnitInput {
            value_si: v * 0.0254,
            kind: QuantityKind::Length,
        }
    }
    pub fn ft(v: f64) -> UnitInput {
        UnitInput {
            value_si: v * 0.3048,
            kind: QuantityKind::Length,
        }
    }
}

pub mod area {
    use super::{QuantityKind, UnitInput};
    pub fn m2(v: f64) -> UnitInput {
        UnitInput {
            value_si: v,
            kind: QuantityKind::Area,
        }
    }
    pub fn mm2(v: f64) -> UnitInput {
        UnitInput {
            value_si: v * 1e-6,
            kind: QuantityKind::Area,
        }
    }
    pub fn cm2(v: f64) -> UnitInput {
        UnitInput {
            value_si: v * 1e-4,
            kind: QuantityKind::Area,
        }
    }
    pub fn in2(v: f64) -> UnitInput {
        UnitInput {
            value_si: v * 0.000_645_16,
            kind: QuantityKind::Area,
        }
    }
}

pub mod density {
    use super::{QuantityKind, UnitInput};
    pub fn kg_per_m3(v: f64) -> UnitInput {
        UnitInput {
            value_si: v,
            kind: QuantityKind::Density,
        }
    }
    pub fn lbm_per_ft3(v: f64) -> UnitInput {
        UnitInput {
            value_si: v * 16.018_463_373_960_14,
            kind: QuantityKind::Density,
        }
    }
}

pub mod viscosity {
    use super::{QuantityKind, UnitInput};
    pub fn pa_s(v: f64) -> UnitInput {
        UnitInput {
            value_si: v,
            kind: QuantityKind::DynamicViscosity,
        }
    }
    pub fn cp(v: f64) -> UnitInput {
        UnitInput {
            value_si: v * 1e-3,
            kind: QuantityKind::DynamicViscosity,
        }
    }
}

pub mod force {
    use super::{QuantityKind, UnitInput};
    pub fn n(v: f64) -> UnitInput {
        UnitInput {
            value_si: v,
            kind: QuantityKind::Force,
        }
    }
    pub fn kn(v: f64) -> UnitInput {
        UnitInput {
            value_si: v * 1e3,
            kind: QuantityKind::Force,
        }
    }
    pub fn lbf(v: f64) -> UnitInput {
        UnitInput {
            value_si: v * 4.448_221_615_260_5,
            kind: QuantityKind::Force,
        }
    }
}

pub mod moment {
    use super::{QuantityKind, UnitInput};
    pub fn n_m(v: f64) -> UnitInput {
        UnitInput {
            value_si: v,
            kind: QuantityKind::Moment,
        }
    }
    pub fn lbf_ft(v: f64) -> UnitInput {
        UnitInput {
            value_si: v * 1.355_817_948_331_400_4,
            kind: QuantityKind::Moment,
        }
    }
}

pub mod temperature {
    use super::{QuantityKind, UnitInput};
    pub fn k(v: f64) -> UnitInput {
        UnitInput {
            value_si: v,
            kind: QuantityKind::Temperature,
        }
    }
}

pub mod thermal_conductivity {
    use super::{QuantityKind, UnitInput};
    pub fn w_per_m_k(v: f64) -> UnitInput {
        UnitInput {
            value_si: v,
            kind: QuantityKind::ThermalConductivity,
        }
    }
}

pub mod heat_transfer_coefficient {
    use super::{QuantityKind, UnitInput};
    pub fn w_per_m2_k(v: f64) -> UnitInput {
        UnitInput {
            value_si: v,
            kind: QuantityKind::HeatTransferCoefficient,
        }
    }
}

pub mod mass_flow_rate {
    use super::{QuantityKind, UnitInput};
    pub fn kg_per_s(v: f64) -> UnitInput {
        UnitInput {
            value_si: v,
            kind: QuantityKind::MassFlowRate,
        }
    }
    pub fn lbm_per_s(v: f64) -> UnitInput {
        UnitInput {
            value_si: v * 0.453_592_37,
            kind: QuantityKind::MassFlowRate,
        }
    }
}

pub mod volumetric_flow_rate {
    use super::{QuantityKind, UnitInput};
    pub fn m3_per_s(v: f64) -> UnitInput {
        UnitInput {
            value_si: v,
            kind: QuantityKind::VolumetricFlowRate,
        }
    }
    pub fn l_per_s(v: f64) -> UnitInput {
        UnitInput {
            value_si: v * 1e-3,
            kind: QuantityKind::VolumetricFlowRate,
        }
    }
    pub fn gal_per_min(v: f64) -> UnitInput {
        UnitInput {
            value_si: v * 0.003_785_411_784 / 60.0,
            kind: QuantityKind::VolumetricFlowRate,
        }
    }
}

pub mod dimensionless {
    use super::{QuantityKind, UnitInput};
    pub fn ratio(v: f64) -> UnitInput {
        UnitInput {
            value_si: v,
            kind: QuantityKind::Dimensionless,
        }
    }
}

pub mod constants {
    use super::*;

    pub const G0_MPS2: f64 = 9.806_65;

    #[inline]
    pub fn g0() -> Accel {
        use uom::si::acceleration::meter_per_second_squared;
        Accel::new::<meter_per_second_squared>(G0_MPS2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructors_smoke() {
        let _p = pa(101_325.0);
        let _t = k(300.0);
        let _mdot = kgps(1.2);
        let _l = m(2.0);
        let _dt = s(0.1);
        let _r = unitless(0.5);
        let _g0 = constants::g0();
    }

    #[test]
    fn typed_inputs_convert() {
        let p = pressure::mpa(2.5);
        assert!((p.value_si - 2.5e6).abs() < 1e-9);
        let mu = viscosity::cp(1.0);
        assert!((mu.value_si - 1.0e-3).abs() < 1e-12);
    }
}
