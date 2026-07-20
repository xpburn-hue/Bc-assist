use super::drag::DragFunction;
use super::integrator::rk4_step;
use super::physics::{acceleration, DragModel, NoDrag};
use super::state::StateVector;
use crate::models::DistanceYards;

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
    pub fn new() -> Self { Self { points: Vec::new() } }
    pub fn add_point(&mut self, point: TrajectoryPoint) { self.points.push(point); }
}

#[derive(Debug, Clone, Copy)]
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

pub fn rk4_step_state<D: DragModel>(state: StateVector, time: f64, dt: f64, drag: D) -> StateVector {
    StateVector::from_vec(&rk4_step(time, &state.as_vec(), dt, |t, y| {
        let current = StateVector::from_vec(y);
        let (ax, ay) = acceleration(&current, &drag);
        vec![current.velocity_x, current.velocity_y, ax, ay]
    }))
}

pub fn free_flight_step(state: StateVector, time: f64, dt: f64) -> StateVector {
    rk4_step_state(state, time, dt, NoDrag)
}
