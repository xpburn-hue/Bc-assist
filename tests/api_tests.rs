use bc_assist::api::{calculate_trajectory, request::TrajectoryRequest};

#[test]
fn trajectory_request_returns_response() {
    let request = TrajectoryRequest {
        distance_yards: 100.0,
    };

    let response = calculate_trajectory(request);

    assert_eq!(response.requested_distance_yards, 100.0);
    assert_eq!(response.status, "ready");
}
