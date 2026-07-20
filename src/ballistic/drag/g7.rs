use super::lookup::coefficient_for_velocity;
use super::table::DragTableEntry;
use super::DragFunction;

#[derive(Debug, Clone, Copy)]
pub struct G7;

const G7_TABLE: &[DragTableEntry] = &[
    DragTableEntry {
        mach: 0.0,
        coefficient: 0.1198,
    },
    DragTableEntry {
        mach: 1.0,
        coefficient: 0.2100,
    },
    DragTableEntry {
        mach: 2.0,
        coefficient: 0.1500,
    },
];

impl DragFunction for G7 {
    fn retardation(&self, velocity_fps: f64) -> f64 {
        velocity_fps * coefficient_for_velocity(G7_TABLE, velocity_fps)
    }
}
