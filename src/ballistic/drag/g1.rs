use super::DragFunction;

#[derive(Debug, Clone, Copy)]
pub struct G1;

impl DragFunction for G1 {
    fn retardation(&self, velocity_fps: f64) -> f64 {
        // Placeholder coefficient pending published G1 drag table integration.
        velocity_fps * 0.0001
    }
}
