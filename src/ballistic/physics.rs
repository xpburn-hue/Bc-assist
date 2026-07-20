use super::state::StateVector;

pub trait DragModel {
    fn acceleration(&self, state: &StateVector) -> (f64, f64);
}

#[derive(Debug, Clone, Copy)]
pub struct NoDrag;

impl DragModel for NoDrag {
    fn acceleration(&self, _state: &StateVector) -> (f64, f64) {
        (0.0, 0.0)
    }
}

pub const GRAVITY: f64 = -9.80665;

pub fn acceleration<D: DragModel>(state: &StateVector, drag: &D) -> (f64, f64) {
    let (drag_x, drag_y) = drag.acceleration(state);
    (drag_x, GRAVITY + drag_y)
}
