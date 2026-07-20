pub mod request;
pub mod response;

use request::TrajectoryRequest;
use response::TrajectoryResponse;

/// API boundary placeholder for trajectory calculation.
///
/// This currently validates request flow only. The ballistic solver pipeline
/// will be connected in a later change once projectile and environment inputs
/// are included in the request model.
pub fn calculate_trajectory(request: TrajectoryRequest) -> TrajectoryResponse {
    TrajectoryResponse {
        requested_distance_yards: request.distance_yards,
        status: "solver_not_connected".to_string(),
    }
}
