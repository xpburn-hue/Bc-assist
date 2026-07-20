use super::trajectory::TrajectoryPoint;

#[derive(Debug, Clone, Copy)]
pub struct RegressionPoint {
    pub distance_yards: f64,
    pub expected_velocity_fps: f64,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_velocity_within_tolerance() {
        assert!(within_tolerance(2500.0, 2495.0, 10.0));
        assert!(!within_tolerance(2500.0, 2480.0, 10.0));
    }
}
