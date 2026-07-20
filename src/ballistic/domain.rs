use crate::models::{BallisticCoefficient, DistanceYards, PressureInHg, TemperatureF, VelocityFps};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DragModel {
    G1,
    G7,
}

#[derive(Debug, Clone, Copy)]
pub struct Projectile {
    pub weight_grains: f64,
    pub diameter_inches: f64,
    pub bc: BallisticCoefficient,
    pub drag_model: DragModel,
}

#[derive(Debug, Clone, Copy)]
pub struct Atmosphere {
    pub temperature: TemperatureF,
    pub pressure: PressureInHg,
    pub humidity_percent: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct ShotMeasurement {
    pub distance: DistanceYards,
    pub muzzle_velocity: VelocityFps,
}
