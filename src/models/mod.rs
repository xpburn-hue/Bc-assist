#[derive(Debug, Clone, Copy)]
pub struct DistanceYards(pub f64);

#[derive(Debug, Clone, Copy)]
pub struct VelocityFps(pub f64);

#[derive(Debug, Clone, Copy)]
pub struct BulletWeightGrains(pub f64);

#[derive(Debug, Clone, Copy)]
pub struct BallisticCoefficient(pub f64);

/// Atmospheric pressure in inches of mercury.
/// Commonly used by ballistic calculators and weather stations.
#[derive(Debug, Clone, Copy)]
pub struct PressureInHg(pub f64);

/// Atmospheric pressure in pascals.
/// SI unit for pressure calculations.
#[derive(Debug, Clone, Copy)]
pub struct PressurePascal(pub f64);

impl PressureInHg {
    pub fn to_pascal(self) -> PressurePascal {
        PressurePascal(self.0 * 3386.389)
    }
}

impl PressurePascal {
    pub fn to_inhg(self) -> PressureInHg {
        PressureInHg(self.0 / 3386.389)
    }
}
