#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BallisticOutput {
    pub velocity_fps: f64,
    pub energy_ft_lbs: f64,
    pub time_of_flight_seconds: f64,
    pub drop_feet: f64,
}

pub fn energy_ft_lbs(mass_grains: f64, velocity_fps: f64) -> f64 {
    (mass_grains * velocity_fps.powi(2)) / 450240.0
}

pub fn feet_to_inches(feet: f64) -> f64 {
    feet * 12.0
}

pub fn feet_to_centimeters(feet: f64) -> f64 {
    feet * 30.48
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_energy() {
        let energy = energy_ft_lbs(175.0, 2600.0);
        assert!((energy - 2627.0).abs() < 2.0);
    }
}
