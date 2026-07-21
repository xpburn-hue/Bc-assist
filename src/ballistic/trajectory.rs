use super::config::{IntegrationMethod, SolverConfig};
use super::drag::DragFunction;
use super::integrator::{euler_step, rk4_step};
use super::state::StateVector;
use crate::models::DistanceYards;

#[derive(Debug, Clone, Copy)]
pub struct TrajectoryPoint {
    pub distance: DistanceYards,
    pub velocity_fps: f64,
    pub drop_feet: f64,
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
pub struct PointMassSolver<D: DragFunction> {
    pub drag_model: D,
    pub config: SolverConfig,
}

impl<D: DragFunction> PointMassSolver<D> {
    pub fn new(drag_model: D, config: SolverConfig) -> Self {
        Self { drag_model, config }
    }

    pub fn solve(&self, muzzle_velocity_fps: f64, max_distance_yards: f64) -> Trajectory {
        let mut trajectory = Trajectory::new();
        let mut state = StateVector {
            position_x: 0.0,
            position_y: 0.0,
            velocity_x: muzzle_velocity_fps,
            velocity_y: 0.0,
        };
        let mut time = 0.0;
        let step = self.config.step_size_yards;

        while state.position_x / 3.0 <= max_distance_yards {
            trajectory.add_point(TrajectoryPoint {
                distance: DistanceYards(state.position_x / 3.0),
                velocity_fps: state.velocity_x,
                drop_feet: state.position_y,
                time_of_flight_seconds: time,
                energy_ft_lbs: 0.0,
            });
            let dt = (step * 3.0) / state.velocity_x.max(1.0);
            state = match self.config.integration_method {
                IntegrationMethod::Euler => euler_step_state(state, time, dt, &self.drag_model),
                IntegrationMethod::RK4 => rk4_step_state(state, time, dt, &self.drag_model),
            };
            time += dt;
        }
        trajectory
    }
}

fn derivative<D: DragFunction>(state: &StateVector, drag: &D) -> Vec<f64> {
    let speed = (state.velocity_x.powi(2) + state.velocity_y.powi(2)).sqrt();
    let drag_accel = if speed > 0.0 {
        drag.retardation(speed)
    } else {
        0.0
    };
    vec![
        state.velocity_x,
        state.velocity_y,
        -drag_accel * state.velocity_x / speed.max(1.0),
        -9.80665 - drag_accel * state.velocity_y / speed.max(1.0),
    ]
}

fn euler_step_state<D: DragFunction>(state: StateVector, time: f64, dt: f64, drag: &D) -> StateVector {
    StateVector::from_vec(&euler_step(time, &state.as_vec(), dt, |_t, y| {
        derivative(&StateVector::from_vec(y), drag)
    }))
}

fn rk4_step_state<D: DragFunction>(state: StateVector, time: f64, dt: f64, drag: &D) -> StateVector {
    StateVector::from_vec(&rk4_step(time, &state.as_vec(), dt, |_t, y| {
        derivative(&StateVector::from_vec(y), drag)
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integration_methods_produce_paths() {
        let euler = PointMassSolver::new(
            super::super::drag::g1::G1,
            SolverConfig {
                integration_method: IntegrationMethod::Euler,
                ..Default::default()
            },
        );
        let rk4 = PointMassSolver::new(super::super::drag::g1::G1, SolverConfig::default());
        assert_ne!(euler.solve(2800.0, 100.0).points.len(), 0);
        assert_ne!(rk4.solve(2800.0, 100.0).points.len(), 0);
    }
}
