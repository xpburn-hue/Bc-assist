#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DragTableEntry {
    pub mach: f64,
    pub coefficient: f64,
}

pub fn interpolate(entries: &[DragTableEntry], mach: f64) -> f64 {
    if entries.is_empty() {
        return 0.0;
    }

    if mach <= entries[0].mach {
        return entries[0].coefficient;
    }

    for pair in entries.windows(2) {
        let lower = pair[0];
        let upper = pair[1];

        if mach <= upper.mach {
            let fraction = (mach - lower.mach) / (upper.mach - lower.mach);
            return lower.coefficient
                + fraction * (upper.coefficient - lower.coefficient);
        }
    }

    entries.last().unwrap().coefficient
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpolates_between_points() {
        let table = [
            DragTableEntry {
                mach: 0.0,
                coefficient: 1.0,
            },
            DragTableEntry {
                mach: 1.0,
                coefficient: 2.0,
            },
        ];

        assert_eq!(interpolate(&table, 0.5), 1.5);
    }
}
