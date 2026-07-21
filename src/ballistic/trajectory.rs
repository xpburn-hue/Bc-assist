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
        while state.position_x / 3.0 <= max_distance_yards {
            trajectory.add_point(TrajectoryPoint {
                distance: DistanceYards(state.position_x / 3.0),
                velocity_fps: state.velocity_x,
                drop_feet: state.position_y,
                time_of_flight_seconds: time,
                energy_ft_lbs: 0.0,
            });
            let dt = (self.config.step_size_yards * 3.0) / state.velocity_x.max(1.0);
            state = match self.config.integration_method {
                IntegrationMethod::Euler => euler_step_state(state, time, dt, &self.drag_model),
                IntegrationMethod::RK4 => rk4_step_state(state, time, dt, &self.drag_model),
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

fn euler_step_state<D: DragFunction>(
    state: StateVector,
    time: f64,
    dt: f64,
    drag: &D,
) -> StateVector {
    StateVector::from_vec(&euler_step(time, &state.as_vec(), dt, |_t, y| {
        derivative(&StateVector::from_vec(y), drag)
    }))
}

fn rk4_step_state<D: DragFunction>(
    state: StateVector,
    time: f64,
    dt: f64,
    drag: &D,
) -> StateVector {
    StateVector::from_vec(&rk4_step(time, &state.as_vec(), dt, |_t, y| {
        derivative(&StateVector::from_vec(y), drag)
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ballistic::outputs::from_trajectory;
    use crate::ballistic::projectile::Projectile;

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

    #[test]
    fn step_size_regression_produces_consistent_results() {
        let coarse = PointMassSolver::new(
            super::super::drag::g1::G1,
            SolverConfig {
                step_size_yards: 1.0,
                ..Default::default()
            },
        )
        .solve(2800.0, 300.0);
        let fine = PointMassSolver::new(
            super::super::drag::g1::G1,
            SolverConfig {
                step_size_yards: 0.25,
                ..Default::default()
            },
        )
        .solve(2800.0, 300.0);
        assert!(fine.points.len() > coarse.points.len());
        for d in [100.0, 200.0, 300.0] {
            let c = coarse
                .points
                .iter()
                .min_by_key(|p| ((p.distance.0 - d).abs() * 1000.0) as i64)
                .unwrap();
            let f = fine
                .points
                .iter()
                .min_by_key(|p| ((p.distance.0 - d).abs() * 1000.0) as i64)
                .unwrap();
            assert!((c.velocity_fps - f.velocity_fps).abs() < 20.0);
            assert!((c.drop_feet - f.drop_feet).abs() < 0.25);
        }
    }

    #[test]
    fn trajectory_values_are_monotonic() {
        let trajectory = PointMassSolver::new(super::super::drag::g1::G1, SolverConfig::default())
            .solve(2800.0, 300.0);
        let projectile = Projectile::new(175.0, 0.505, 2800.0, 0.308);
        let table = from_trajectory(&trajectory, &projectile);
        let a = table.at_distance(DistanceYards(100.0)).unwrap();
        let b = table.at_distance(DistanceYards(200.0)).unwrap();
        let c = table.at_distance(DistanceYards(300.0)).unwrap();
        assert!(a.velocity_fps > b.velocity_fps && b.velocity_fps > c.velocity_fps);
        assert!(a.energy_ft_lbs > b.energy_ft_lbs && b.energy_ft_lbs > c.energy_ft_lbs);
        assert!(
            a.time_of_flight_seconds < b.time_of_flight_seconds
                && b.time_of_flight_seconds < c.time_of_flight_seconds
        );
        assert!(a.drop_feet > b.drop_feet && b.drop_feet > c.drop_feet);
    }

    #[test]
    fn solver_generates_output_table() {
        let solver =
            PointMassSolver::new(super::super::drag::g1::G1, SolverConfig::default());
        let projectile = Projectile::new(175.0, 0.505, 2800.0, 0.308);

        let table = solver.solve_table(2800.0, &projectile, 300.0);

        let a = table.at_distance(DistanceYards(100.0)).unwrap();
        let b = table.at_distance(DistanceYards(200.0)).unwrap();
        let c = table.at_distance(DistanceYards(300.0)).unwrap();

        assert!(a.velocity_fps > b.velocity_fps);
        assert!(b.velocity_fps > c.velocity_fps);
        assert!(a.energy_ft_lbs > b.energy_ft_lbs);
        assert!(b.energy_ft_lbs > c.energy_ft_lbs);
    }
}
