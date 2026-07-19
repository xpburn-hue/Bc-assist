use super::DragFunction;

#[derive(Debug, Clone, Copy)]
pub struct G7;

impl DragFunction for G7 {
    fn retardation(&self, velocity_fps: f64) -> f64 {
        // Placeholder coefficient pending published G7 drag table integration.
        velocity_fps * 0.00008
    }
}
