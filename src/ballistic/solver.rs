use crate::models::{BallisticCoefficient, DistanceYards, VelocityFps};

#[derive(Debug, Clone, Copy)]
pub struct TrajectorySolver;

#[derive(Debug, Clone, Copy)]
pub struct BcSolver;

impl BcSolver {
    /// Placeholder for future drag-model fitting from velocity/time-of-flight data.
    pub fn estimate_bc(
        _distance: DistanceYards,
        _velocity: VelocityFps,
        current_bc: BallisticCoefficient,
    ) -> BallisticCoefficient {
        current_bc
    }
}
