use crate::models::{BallisticCoefficient, BulletWeightGrains, VelocityFps};

#[derive(Debug, Clone)]
pub struct Projectile {
    pub weight: BulletWeightGrains,
    pub bc: BallisticCoefficient,
    pub muzzle_velocity: VelocityFps,
    pub diameter_inches: f64,
}

impl Projectile {
    pub fn new(weight: f64, bc: f64, velocity: f64, diameter_inches: f64) -> Self {
        Self {
            weight: BulletWeightGrains(weight),
            bc: BallisticCoefficient(bc),
            muzzle_velocity: VelocityFps(velocity),
            diameter_inches,
        }
    }
}
