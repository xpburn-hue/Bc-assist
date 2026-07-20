use super::atmosphere::Atmosphere;
use super::bc::BallisticCoefficient;
use super::drag::bc::retardation_with_bc;
use super::drag::table::DragTableEntry;
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

#[derive(Debug, Clone, Copy)]
pub struct BallisticDrag {
    pub table: &'static [DragTableEntry],
    pub bc: BallisticCoefficient,
    pub atmosphere: Atmosphere,
}

impl DragModel for BallisticDrag {
    fn acceleration(&self, state: &StateVector) -> (f64, f64) {
        let speed = (state.velocity_x.powi(2) + state.velocity_y.powi(2)).sqrt();
        let drag = retardation_with_bc(self.table, speed, self.bc, self.atmosphere);

        if speed <= 0.0 {
            return (0.0, 0.0);
        }

        (
            -drag * state.velocity_x / speed,
            -drag * state.velocity_y / speed,
        )
    }
}

pub const GRAVITY: f64 = -9.80665;

pub fn acceleration<D: DragModel>(state: &StateVector, drag: &D) -> (f64, f64) {
    let (drag_x, drag_y) = drag.acceleration(state);
    (drag_x, GRAVITY + drag_y)
}
