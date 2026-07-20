use super::bc::DragCurve;

/// A point in a standard ballistic drag table.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DragTableEntry {
    pub mach: f64,
    pub coefficient: f64,
}

/// Lookup data for standard drag curves.
#[derive(Debug, Clone, Copy)]
pub struct DragTable {
    pub curve: DragCurve,
    pub entries: &'static [DragTableEntry],
}

// Initial placeholder tables. Values will be expanded with full reference data.
const G1_TABLE: &[DragTableEntry] = &[
    DragTableEntry {
        mach: 0.0,
        coefficient: 0.2629,
    },
    DragTableEntry {
        mach: 1.0,
        coefficient: 0.5191,
    },
];

const G7_TABLE: &[DragTableEntry] = &[
    DragTableEntry {
        mach: 0.0,
        coefficient: 0.1198,
    },
    DragTableEntry {
        mach: 1.0,
        coefficient: 0.2100,
    },
];

pub fn table_for_curve(curve: DragCurve) -> DragTable {
    match curve {
        DragCurve::G1 => DragTable {
            curve,
            entries: G1_TABLE,
        },
        DragCurve::G7 => DragTable {
            curve,
            entries: G7_TABLE,
        },
    }
}
