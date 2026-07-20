use serde::{Deserialize, Serialize};

/// Standard drag curve references used for ballistic coefficients.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DragCurve {
    G1,
    G7,
}

/// Ballistic coefficient describing projectile drag characteristics.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BallisticCoefficient {
    pub value: f64,
    pub curve: DragCurve,
}

impl BallisticCoefficient {
    pub fn new(value: f64, curve: DragCurve) -> Self {
        Self { value, curve }
    }
}
