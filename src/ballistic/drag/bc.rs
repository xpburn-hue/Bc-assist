use super::lookup::coefficient_for_velocity;
use super::table::DragTableEntry;
use crate::ballistic::atmosphere::Atmosphere;
use crate::ballistic::bc::BallisticCoefficient;

pub fn retardation_with_bc(
    entries: &[DragTableEntry],
    velocity_fps: f64,
    bc: BallisticCoefficient,
    atmosphere: Atmosphere,
) -> f64 {
    if bc.value <= 0.0 {
        return 0.0;
    }

    velocity_fps * coefficient_for_velocity(entries, velocity_fps) / bc.value
        * atmosphere.density_ratio()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ballistic::bc::DragCurve;

    #[test]
    fn scales_retardation_by_bc_and_atmosphere() {
        let table = [DragTableEntry {
            mach: 0.0,
            coefficient: 1.0,
        }];

        let bc = BallisticCoefficient::new(2.0, DragCurve::G1);
        let atmosphere = Atmosphere::standard();

        assert_eq!(retardation_with_bc(&table, 100.0, bc, atmosphere), 50.0);
    }
}
