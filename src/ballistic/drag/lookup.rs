use super::table::{interpolate, DragTableEntry};

pub fn coefficient_for_velocity(entries: &[DragTableEntry], velocity_fps: f64) -> f64 {
    let mach = velocity_fps / 1125.0;
    interpolate(entries, mach)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_velocity_to_mach_for_lookup() {
        let table = [
            DragTableEntry {
                mach: 0.0,
                coefficient: 0.0,
            },
            DragTableEntry {
                mach: 1.0,
                coefficient: 1.0,
            },
        ];

        assert!((coefficient_for_velocity(&table, 1125.0) - 1.0).abs() < f64::EPSILON);
    }
}
