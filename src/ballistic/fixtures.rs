use super::bc::{BallisticCoefficient, DragCurve};

#[derive(Debug, Clone, Copy)]
pub struct ProjectileFixture {
    pub name: &'static str,
    pub mass_grains: f64,
    pub muzzle_velocity_fps: f64,
    pub bc: BallisticCoefficient,
}

pub const EXAMPLE_308_175_SMK: ProjectileFixture = ProjectileFixture {
    name: "308 175gr Match Projectile",
    mass_grains: 175.0,
    muzzle_velocity_fps: 2600.0,
    bc: BallisticCoefficient {
        value: 0.496,
        curve: DragCurve::G7,
    },
};

pub fn fixture_velocity_samples() -> &'static [(f64, f64)] {
    &[
        (0.0, 2600.0),
        (100.0, 2400.0),
        (300.0, 2100.0),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_has_valid_parameters() {
        assert!(EXAMPLE_308_175_SMK.mass_grains > 0.0);
        assert!(EXAMPLE_308_175_SMK.bc.value > 0.0);
        assert!(!fixture_velocity_samples().is_empty());
    }
}
