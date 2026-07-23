mod validation;

use approx::assert_relative_eq;
use bc_assist::ballistic::drag::g7::G7;
use bc_assist::ballistic::trajectory::PointMassSolver;
use bc_assist::ballistic::config::SolverConfig;

#[test]
fn baseline_trajectory_regression_fixture() {
    let solver = PointMassSolver::new(G7, SolverConfig::default());
    let trajectory = solver.solve(2600.0, 300.0);

    let final_point = trajectory.points.last().expect("trajectory should contain points");

    // Baseline values are generated from the current deterministic solver.
    // These values become the regression guard as solver features evolve.
    assert_relative_eq!(final_point.velocity_fps, 1928.0, epsilon = 200.0);
    assert!(final_point.time_of_flight_seconds > 0.0);
}

#[test]
fn trajectory_is_stable_with_repeated_solves() {
    let solver = PointMassSolver::new(G7, SolverConfig::default());

    let first = solver.solve(2600.0, 300.0);
    let second = solver.solve(2600.0, 300.0);

    let a = first.points.last().unwrap();
    let b = second.points.last().unwrap();

    assert_relative_eq!(a.velocity_fps, b.velocity_fps, epsilon = 1e-10);
    assert_relative_eq!(a.drop_feet, b.drop_feet, epsilon = 1e-10);
    assert_relative_eq!(a.drift_feet, b.drift_feet, epsilon = 1e-10);
}
