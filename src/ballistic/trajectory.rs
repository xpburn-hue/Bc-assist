use crate::models::DistanceYards;
use super::drag::DragFunction;

#[derive(Debug, Clone, Copy)]
pub struct TrajectoryPoint {
    pub distance: DistanceYards,
    pub velocity_fps: f64,
    pub time_of_flight_seconds: f64,
}

#[derive(Debug, Clone, Default)]
pub struct Trajectory {
    pub points: Vec<TrajectoryPoint>,
}

impl Trajectory {
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }
}

pub struct PointMassSolver<D: DragFunction> {
    pub drag_model: D,
}

impl<D: DragFunction> PointMassSolver<D> {
    pub fn solve(&self, muzzle_velocity_fps: f64, max_distance_yards: f64) -> Trajectory {
        let mut trajectory = Trajectory::new();
        let mut velocity = muzzle_velocity_fps;
        let mut time = 0.0;
        let mut distance = 0.0;

        while distance <= max_distance_yards {
            trajectory.points.push(TrajectoryPoint {
                distance: DistanceYards(distance),
                velocity_fps: velocity,
                time_of_flight_seconds: time,
            });

            let feet = 25.0 * 3.0;
            time += feet / velocity;
            velocity -= self.drag_model.retardation(velocity) * feet;
            distance += 25.0;
        }

        trajectory
    }
}
