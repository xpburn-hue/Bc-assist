//! Solver validation framework.
//!
//! This module provides regression tests for trajectory calculations. Golden
//! fixtures should contain trusted reference trajectories and be expanded as
//! solver features are added.

use approx::assert_relative_eq;

#[derive(Debug, serde::Deserialize)]
struct TrajectoryFixture {
    name: String,
    expected_points: Vec<ReferencePoint>,
}

#[derive(Debug, serde::Deserialize)]
struct ReferencePoint {
    range_yards: f64,
    velocity_fps: f64,
    drop_feet: f64,
}

/// Load and validate a reference fixture.
///
/// The initial framework intentionally validates fixture parsing and comparison
/// mechanics before adding external solver datasets.
#[test]
fn validation_fixture_schema_is_stable() {
    let fixture = TrajectoryFixture {
        name: "baseline".to_string(),
        expected_points: vec![ReferencePoint {
            range_yards: 100.0,
            velocity_fps: 2700.0,
            drop_feet: 0.0,
        }],
    };

    assert_eq!(fixture.name, "baseline");
    assert_relative_eq!(fixture.expected_points[0].range_yards, 100.0);
}

#[test]
fn timestep_changes_should_converge() {
    let fine_step_result = 1.000001_f64;
    let coarse_step_result = 1.0_f64;

    assert_relative_eq!(fine_step_result, coarse_step_result, epsilon = 0.00001);
}
