pub mod g1;
pub mod g7;

pub trait DragFunction {
    fn retardation(&self, velocity_fps: f64) -> f64;
}
