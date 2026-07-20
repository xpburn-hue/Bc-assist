use crate::models::VelocityFps;

pub trait Chronograph {
    fn muzzle_velocity(&self) -> VelocityFps;
}

#[derive(Debug, Clone, Copy)]
pub struct ManualChronograph {
    pub velocity: VelocityFps,
}

impl Chronograph for ManualChronograph {
    fn muzzle_velocity(&self) -> VelocityFps {
        self.velocity
    }
}
