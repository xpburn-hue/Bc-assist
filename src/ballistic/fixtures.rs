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

    pub fn sample_count(&self) -> usize {
        self.samples.len()
    }

    pub fn max_distance_yards(&self) -> Option<f64> {
        self.samples.last().map(|sample| sample.distance_yards)
    }

    pub fn velocity_at_distance(&self, distance_yards: f64) -> Option<f64> {
        if self.samples.is_empty() {
            return None;
        }

        if distance_yards <= self.samples[0].distance_yards {
            return Some(self.samples[0].velocity_fps);
        }

        for pair in self.samples.windows(2) {
            let lower = pair[0];
            let upper = pair[1];

            if distance_yards <= upper.distance_yards {
                let fraction = (distance_yards - lower.distance_yards)
                    / (upper.distance_yards - lower.distance_yards);

                return Some(
                    lower.velocity_fps + fraction * (upper.velocity_fps - lower.velocity_fps),
                );
            }
        }

        self.samples.last().map(|sample| sample.velocity_fps)
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
        assert!(EXAMPLE_308_175_SMK_DATA.is_valid());
    }

    #[test]
    fn dataset_queries_work() {
        assert_eq!(EXAMPLE_308_175_SMK_DATA.sample_count(), 3);
        assert_eq!(EXAMPLE_308_175_SMK_DATA.max_distance_yards(), Some(300.0));
        assert_eq!(
            EXAMPLE_308_175_SMK_DATA.velocity_at_distance(100.0),
            Some(2400.0)
        );
        assert_eq!(
            EXAMPLE_308_175_SMK_DATA.velocity_at_distance(200.0),
            Some(2250.0)
        );
    }
}
