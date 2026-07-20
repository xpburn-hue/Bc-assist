use crate::models::DistanceYards;
use super::drag::DragFunction;

#[derive(Debug, Clone, Copy)]
pub struct TrajectoryPoint {
    pub distance: DistanceYards,
    pub velocity_fps: f64,
    pub time_of_flight_seconds: f64,
    pub energy_ft_lbs: f64,
}

#[derive(Debug, Clone, Default)]
pub struct Trajectory {
    pub points: Vec<TrajectoryPoint>,
}

impl Trajectory {
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }

    pub fn add_point(&mut self, point: TrajectoryPoint) {
        self.points.push(point);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PointMassSolver {
    pub drag_coefficient: f64,
}

impl PointMassSolver {
    pub fn new(drag_coefficient: f64) -> Self {
        Self { drag_coefficient }
    }

    /// Simple first-order velocity decay model.
    /// This is intentionally not a full ballistic solver; it provides the
    /// integration interface that later drag models will replace.
    pub fn solve(
        &self,
        muzzle_velocity_fps: f64,
        bullet_weight_grains: f64,
        max_distance_yards: f64,
        step_yards: f64,
    ) -> Trajectory {
        let mut trajectory = Trajectory::new();
        let mut velocity = muzzle_velocity_fps;
        let mut time = 0.0;
        let mut distance = 0.0;

        while distance <= max_distance_yards {
            trajectory.add_point(TrajectoryPoint {
                distance: DistanceYards(distance),
                velocity_fps: velocity,
                time_of_flight_seconds: time,
                energy_ft_lbs: 0.5 * (bullet_weight_grains / 7000.0 / 32.174) * velocity * velocity,
            });

            let feet = step_yards * 3.0;
            time += feet / velocity;
            velocity *= (-self.drag_coefficient * feet / 1000.0).exp();
            distance += step_yards;
        }

        trajectory
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trajectory_generates_points() {
        let solver = PointMassSolver::new(0.5);
        let result = solver.solve(2500.0, 130.0, 100.0, 50.0);

        assert_eq!(result.points.len(), 3);
        assert!(result.points[1].velocity_fps < 2500.0);
    }
}

pub struct PointMassSolver<D: DragFunction> {
    pub drag_model: D,
}

impl<D: DragFunction> PointMassSolver<D> {
    pub fn solve(
        &self,
        muzzle_velocity_fps: f64,
        max_distance_yards: f64,
    ) -> Trajectory {
        let mut trajectory = Trajectory::new();
        let mut velocity = muzzle_velocity_fps;
        let mut time = 0.0;
        let mut distance = 0.0;

        while distance <= max_distance_yards {
            trajectory.add_point(TrajectoryPoint {
                distance: DistanceYards(distance),
                velocity_fps: velocity,
                time_of_flight_seconds: time,
                energy_ft_lbs: 0.0,
            });

            let feet = 25.0 * 3.0;
            time += feet / velocity;
            velocity -= self.drag_model.retardation(velocity) * feet;
            distance += 25.0;
        }

        trajectory
    }
}

