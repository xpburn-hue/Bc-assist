use super::projectile::Projectile;
use super::trajectory::Trajectory;
use crate::models::DistanceYards;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BallisticOutput {
    pub velocity_fps: f64,
    pub energy_ft_lbs: f64,
    pub time_of_flight_seconds: f64,
    pub drop_feet: f64,
}

#[derive(Debug, Clone, Default)]
pub struct TrajectoryTable {
    pub points: Vec<(DistanceYards, BallisticOutput)>,
}

impl TrajectoryTable {
    pub fn new(points: Vec<(DistanceYards, BallisticOutput)>) -> Self {
        Self { points }
    }

    pub fn at_index(&self, index: usize) -> Option<BallisticOutput> {
        self.points.get(index).map(|(_, output)| *output)
    }

    pub fn at_distance(&self, distance: DistanceYards) -> Option<BallisticOutput> {
        self.points
            .iter()
            .min_by(|(left, _), (right, _)| {
                (left.0 - distance.0)
                    .abs()
                    .partial_cmp(&(right.0 - distance.0).abs())
                    .unwrap()
            })
            .map(|(_, output)| *output)
    }
}

pub fn from_trajectory(trajectory: &Trajectory, projectile: &Projectile) -> TrajectoryTable {
    TrajectoryTable::new(
        trajectory
            .points
            .iter()
            .map(|point| {
                (
                    point.distance,
                    BallisticOutput {
                        velocity_fps: point.velocity_fps,
                        energy_ft_lbs: energy_ft_lbs(projectile.weight.0, point.velocity_fps),
                        time_of_flight_seconds: point.time_of_flight_seconds,
                        drop_feet: point.drop_feet,
                    },
                )
            })
            .collect(),
    )
}

pub fn energy_ft_lbs(mass_grains: f64, velocity_fps: f64) -> f64 {
    (mass_grains * velocity_fps.powi(2)) / 450240.0
}

pub fn feet_to_inches(feet: f64) -> f64 {
    feet * 12.0
}

pub fn feet_to_centimeters(feet: f64) -> f64 {
    feet * 30.48
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_energy() {
        let energy = energy_ft_lbs(175.0, 2600.0);
        assert!((energy - 2627.0).abs() < 2.0);
    }

    #[test]
    fn finds_nearest_distance() {
        let table = TrajectoryTable::new(vec![
            (
                DistanceYards(100.0),
                BallisticOutput {
                    velocity_fps: 2500.0,
                    energy_ft_lbs: 0.0,
                    time_of_flight_seconds: 0.1,
                    drop_feet: 0.0,
                },
            ),
            (
                DistanceYards(200.0),
                BallisticOutput {
                    velocity_fps: 2300.0,
                    energy_ft_lbs: 0.0,
                    time_of_flight_seconds: 0.2,
                    drop_feet: 1.0,
                },
            ),
        ]);

        assert_eq!(table.at_distance(DistanceYards(180.0)).unwrap().velocity_fps, 2300.0);
    }
}
