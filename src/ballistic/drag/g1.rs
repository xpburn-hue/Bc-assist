use super::lookup::coefficient_for_velocity;
use super::table::DragTableEntry;
use super::DragFunction;

#[derive(Debug, Clone, Copy)]
pub struct G1;

const G1_TABLE: &[DragTableEntry] = &[
    DragTableEntry {
        mach: 0.0,
        coefficient: 0.2629,
    },
    DragTableEntry {
        mach: 1.0,
        coefficient: 0.5191,
    },
    DragTableEntry {
        mach: 2.0,
        coefficient: 0.3800,
    },
];

impl DragFunction for G1 {
    fn retardation(&self, velocity_fps: f64) -> f64 {
        velocity_fps * coefficient_for_velocity(G1_TABLE, velocity_fps)
    }
}
