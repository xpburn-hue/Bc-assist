use bc_assist::api::{calculate_trajectory, request::TrajectoryRequest};

fn main() {
    let request = TrajectoryRequest {
        distance_yards: 100.0,
    };

    let response = calculate_trajectory(request);
    println!(
        "{} yards: {}",
        response.requested_distance_yards, response.status
    );
}
