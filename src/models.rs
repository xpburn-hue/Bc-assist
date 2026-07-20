#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BallisticCoefficient(pub f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VelocityFps(pub f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DistanceYards(pub f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TemperatureF(pub f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PressureInHg(pub f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HumidityPercent(pub f64);
