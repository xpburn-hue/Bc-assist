use super::drag::DragFunction;
use super::trajectory::{PointMassSolver, Trajectory, TrajectoryPoint};

#[derive(Debug, Clone, Copy)]
pub struct RegressionPoint {
    pub distance_yards: f64,
    pub expected_velocity_fps: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct RegressionResult {
    pub expected: RegressionPoint,
    pub actual_velocity_fps: f64,
    pub error_fps: f64,
}

pub fn velocity_error(actual: f64, expected: f64) -> f64 {
    actual - expected
}

pub fn within_tolerance(actual: f64, expected: f64, tolerance_fps: f64) -> bool {
    velocity_error(actual, expected).abs() <= tolerance_fps
}

pub fn compare_point(point: &TrajectoryPoint, expected: RegressionPoint, tolerance_fps: f64) -> bool {
    within_tolerance(
        point.velocity_fps,
        expected.expected_velocity_fps,
        tolerance_fps,
    )
}

pub fn compare_trajectory(
    trajectory: &Trajectory,
    expected: &[RegressionPoint],
) -> Vec<RegressionResult> {
    expected
        .iter()
        .map(|sample| {
            let actual_velocity_fps = trajectory
                .points
                .iter()
                .min_by(|a, b| {
                    (a.distance.0 - sample.distance_yards)
                        .abs()
                        .partial_cmp(&(b.distance.0 - sample.distance_yards).abs())
                        .unwrap()
                })
                .map(|point| point.velocity_fps)
                .unwrap_or_default();

            RegressionResult {
                expected: *sample,
                actual_velocity_fps,
                error_fps: velocity_error(actual_velocity_fps, sample.expected_velocity_fps),
            }
        })
        .collect()
}

pub fn run_fixture<D: DragFunction>(
    solver: &PointMassSolver<D>,
    muzzle_velocity_fps: f64,
    expected: &[RegressionPoint],
) -> Vec<RegressionResult> {
    let max_distance = expected
        .iter()
        .map(|point| point.distance_yards)
        .fold(0.0, f64::max);

    compare_trajectory(&solver.solve(muzzle_velocity_fps, max_distance), expected)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_velocity_within_tolerance() {
        assert!(within_tolerance(2500.0, 2495.0, 10.0));
        assert!(!within_tolerance(2500.0, 2480.0, 10.0));
    }
}
