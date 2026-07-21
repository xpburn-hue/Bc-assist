use super::drag::DragFunction;
use super::fixtures::ProjectileDataset;
use super::trajectory::{PointMassSolver, Trajectory, TrajectoryPoint};

#[derive(Debug, Clone, Copy)]
pub struct RegressionPoint {
    pub distance_yards: f64,
    pub expected_velocity_fps: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct RegressionResult {
    pub samples_checked: usize,
    pub max_velocity_error_fps: f64,
    pub average_velocity_error_fps: f64,
    pub passed: bool,
}

pub fn velocity_error(actual: f64, expected: f64) -> f64 {
    actual - expected
}

pub fn within_tolerance(actual: f64, expected: f64, tolerance_fps: f64) -> bool {
    velocity_error(actual, expected).abs() <= tolerance_fps
}

pub fn compare_point(
    point: &TrajectoryPoint,
    expected: RegressionPoint,
    tolerance_fps: f64,
) -> bool {
    within_tolerance(
        point.velocity_fps,
        expected.expected_velocity_fps,
        tolerance_fps,
    )
}

pub fn compare_trajectory_to_dataset(
    trajectory: &Trajectory,
    dataset: &ProjectileDataset,
    tolerance_fps: f64,
) -> RegressionResult {
    let errors: Vec<f64> = dataset
        .samples
        .iter()
        .filter_map(|sample| {
            let expected_velocity = dataset.velocity_at_distance(sample.distance_yards)?;
            let actual_velocity = trajectory
                .points
                .iter()
                .min_by(|a, b| {
                    (a.distance.0 - sample.distance_yards)
                        .abs()
                        .partial_cmp(&(b.distance.0 - sample.distance_yards).abs())
                        .unwrap()
                })
                .map(|point| point.velocity_fps)?;

            Some(velocity_error(actual_velocity, expected_velocity).abs())
        })
        .collect();

    let samples_checked = errors.len();
    let max_velocity_error_fps = errors.iter().copied().fold(0.0, f64::max);
    let average_velocity_error_fps = if samples_checked == 0 {
        0.0
    } else {
        errors.iter().sum::<f64>() / samples_checked as f64
    };

    RegressionResult {
        samples_checked,
        max_velocity_error_fps,
        average_velocity_error_fps,
        passed: samples_checked == dataset.sample_count()
            && max_velocity_error_fps <= tolerance_fps,
    }
}

pub fn run_fixture<D: DragFunction>(
    solver: &PointMassSolver<D>,
    dataset: &ProjectileDataset,
) -> RegressionResult {
    let max_distance = dataset.max_distance_yards().unwrap_or_default();
    compare_trajectory_to_dataset(
        &solver.solve(dataset.fixture.muzzle_velocity_fps, max_distance),
        dataset,
        0.0,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ballistic::fixtures::EXAMPLE_308_175_SMK_DATA;

    #[test]
    fn accepts_velocity_within_tolerance() {
        assert!(within_tolerance(2500.0, 2495.0, 10.0));
        assert!(!within_tolerance(2500.0, 2480.0, 10.0));
    }

    #[test]
    fn dataset_regression_can_pass() {
        let trajectory = Trajectory {
            points: EXAMPLE_308_175_SMK_DATA
                .samples
                .iter()
                .map(|sample| TrajectoryPoint {
                    distance: crate::models::DistanceYards(sample.distance_yards),
                    velocity_fps: sample.velocity_fps,
                    drop_feet: 0.0,
                    time_of_flight_seconds: 0.0,
                    energy_ft_lbs: 0.0,
                })
                .collect(),
        };

        let result = compare_trajectory_to_dataset(&trajectory, &EXAMPLE_308_175_SMK_DATA, 0.1);
        assert!(result.passed);
    }

    #[test]
    fn dataset_regression_can_fail() {
        let trajectory = Trajectory {
            points: vec![TrajectoryPoint {
                distance: crate::models::DistanceYards(0.0),
                velocity_fps: 2000.0,
                drop_feet: 0.0,
                time_of_flight_seconds: 0.0,
                energy_ft_lbs: 0.0,
            }],
        };

        let result = compare_trajectory_to_dataset(&trajectory, &EXAMPLE_308_175_SMK_DATA, 10.0);
        assert!(!result.passed);
    }
}
