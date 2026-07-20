pub mod request;
pub mod response;

use request::TrajectoryRequest;
use response::TrajectoryResponse;

pub fn calculate_trajectory(request: TrajectoryRequest) -> TrajectoryResponse {
    TrajectoryResponse {
        requested_distance_yards: request.distance_yards,
        status: "ready".to_string(),
    }
}
