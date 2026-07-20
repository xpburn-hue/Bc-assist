use super::bc::{BallisticCoefficient, DragCurve};

#[derive(Debug, Clone, Copy)]
pub struct ProjectileFixture {
    pub name: &'static str,
    pub mass_grains: f64,
    pub muzzle_velocity_fps: f64,
    pub bc: BallisticCoefficient,
}

#[derive(Debug, Clone, Copy)]
pub struct VelocitySample {
    pub distance_yards: f64,
    pub velocity_fps: f64,
}

#[derive(Debug)]
pub struct ProjectileDataset {
    pub fixture: ProjectileFixture,
    pub samples: &'static [VelocitySample],
}

impl ProjectileDataset {
    pub fn is_valid(&self) -> bool {
        self.fixture.mass_grains > 0.0
            && self.fixture.muzzle_velocity_fps > 0.0
            && self.fixture.bc.value > 0.0
            && !self.samples.is_empty()
            && self.samples.windows(2).all(|pair| {
                pair[1].distance_yards >= pair[0].distance_yards
                    && pair[1].velocity_fps <= pair[0].velocity_fps
            })
    }
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

pub const EXAMPLE_308_175_SMK_DATA: ProjectileDataset = ProjectileDataset {
    fixture: EXAMPLE_308_175_SMK,
    samples: &[
        VelocitySample {
            distance_yards: 0.0,
            velocity_fps: 2600.0,
        },
        VelocitySample {
            distance_yards: 100.0,
            velocity_fps: 2400.0,
        },
        VelocitySample {
            distance_yards: 300.0,
            velocity_fps: 2100.0,
        },
    ],
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_has_valid_parameters() {
        assert!(EXAMPLE_308_175_SMK.mass_grains > 0.0);
        assert!(EXAMPLE_308_175_SMK.bc.value > 0.0);
        assert!(!EXAMPLE_308_175_SMK_DATA.samples.is_empty());
    }

    #[test]
    fn dataset_has_valid_samples() {
        assert!(EXAMPLE_308_175_SMK_DATA.is_valid());
    }
}
