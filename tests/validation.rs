mod validation;

use approx::assert_relative_eq;
use bc_assist::ballistic::config::SolverConfig;
use bc_assist::ballistic::drag::g7::G7;
use bc_assist::ballistic::trajectory::PointMassSolver;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GoldenFixture {
    name: String,
    conditions: FixtureConditions,
    points: Vec<GoldenPoint>,
    tolerances: FixtureTolerances,
}

#[derive(Debug, Deserialize)]
struct FixtureConditions {
    muzzle_velocity_fps: f64,
    max_distance_yards: f64,
}

#[derive(Debug, Deserialize)]
struct GoldenPoint {
    range_yards: f64,
    velocity_fps: f64,
    drop_feet: f64,
    time_of_flight_seconds: f64,
}

#[derive(Debug, Deserialize)]
struct FixtureTolerances {
    velocity_fps: f64,
    drop_feet: f64,
    time_seconds: f64,
}

fn load_fixture() -> GoldenFixture {
    serde_json::from_str(include_str!("validation/fixtures/g7_baseline.json"))
        .expect("golden fixture should parse")
}

#[test]
fn golden_fixture_matches_solver_output() {
    let fixture = load_fixture();
    let solver = PointMassSolver::new(G7, SolverConfig::default());
    let trajectory = solver.solve(
        fixture.conditions.muzzle_velocity_fps,
        fixture.conditions.max_distance_yards,
    );

    for expected in fixture.points {
        let actual = trajectory
            .points
            .iter()
            .find(|point| (point.range_yards - expected.range_yards).abs() < 0.5)
            .expect("trajectory should contain fixture range");

        assert_relative_eq!(
            actual.velocity_fps,
            expected.velocity_fps,
            epsilon = fixture.tolerances.velocity_fps
        );
        assert_relative_eq!(
            actual.drop_feet,
            expected.drop_feet,
            epsilon = fixture.tolerances.drop_feet
        );
        assert_relative_eq!(
            actual.time_of_flight_seconds,
            expected.time_of_flight_seconds,
            epsilon = fixture.tolerances.time_seconds
        );
    }
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
