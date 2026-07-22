use super::config::{IntegrationMethod, SolverConfig};
use super::drag::DragFunction;
use super::integrator::{euler_step, rk4_step};
use super::outputs::{from_trajectory, TrajectoryTable};
use super::projectile::Projectile;
use super::state::StateVector;
use crate::models::DistanceYards;

#[derive(Debug, Clone, Copy)]
pub struct TrajectoryPoint {
    pub distance: DistanceYards,
    pub velocity_fps: f64,
    pub drop_feet: f64,
    pub drift_feet: f64,
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
            position_z: 0.0,
            velocity_x: muzzle_velocity_fps,
            velocity_y: 0.0,
            velocity_z: 0.0,
        };
        let mut time = 0.0;
        while state.position_x / 3.0 <= max_distance_yards {
            trajectory.add_point(TrajectoryPoint {
                distance: DistanceYards(state.position_x / 3.0),
                velocity_fps: state.velocity_x,
                drop_feet: state.position_y,
                drift_feet: state.position_z,
                time_of_flight_seconds: time,
                energy_ft_lbs: 0.0,
            });
            let dt = (self.config.step_size_yards * 3.0) / state.velocity_x.max(1.0);
            state = match self.config.integration_method {
                IntegrationMethod::Euler => euler_step_state(
                    state,
                    time,
                    dt,
                    &self.drag_model,
                    self.config.atmosphere.density_ratio(),
                ),
                IntegrationMethod::RK4 => rk4_step_state(
                    state,
                    time,
                    dt,
                    &self.drag_model,
                    self.config.atmosphere.density_ratio(),
                ),
            };
            time += dt;
        }
        trajectory
    }

    pub fn solve_table(
        &self,
        muzzle_velocity_fps: f64,
        projectile: &Projectile,
        max_distance_yards: f64,
    ) -> TrajectoryTable {
        let trajectory = self.solve(muzzle_velocity_fps, max_distance_yards);
        from_trajectory(&trajectory, projectile)
    }
}

fn derivative<D: DragFunction>(state: &StateVector, drag: &D, density_ratio: f64) -> Vec<f64> {
    let speed = (state.velocity_x.powi(2) + state.velocity_y.powi(2) + state.velocity_z.powi(2)).sqrt();
    let drag_accel = if speed > 0.0 {
        drag.retardation(speed) * density_ratio
    } else {
        0.0
    };
    vec![
        state.velocity_x,
        state.velocity_y,
        state.velocity_z,
        -drag_accel * state.velocity_x / speed.max(1.0),
        -9.80665 - drag_accel * state.velocity_y / speed.max(1.0),
        -drag_accel * state.velocity_z / speed.max(1.0),
    ]
}

fn euler_step_state<D: DragFunction>(
    state: StateVector,
    time: f64,
    dt: f64,
    drag: &D,
    density_ratio: f64,
) -> StateVector {
    StateVector::from_vec(&euler_step(time, &state.as_vec(), dt, |_t, y| {
        derivative(&StateVector::from_vec(y), drag, density_ratio)
    }))
}

fn rk4_step_state<D: DragFunction>(
    state: StateVector,
    time: f64,
    dt: f64,
    drag: &D,
    density_ratio: f64,
) -> StateVector {
    StateVector::from_vec(&rk4_step(time, &state.as_vec(), dt, |_t, y| {
        derivative(&StateVector::from_vec(y), drag, density_ratio)
    }))
}
