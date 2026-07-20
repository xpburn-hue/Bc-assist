/// Atmospheric conditions used for ballistic calculations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Atmosphere {
    pub temperature_c: f64,
    pub pressure_hpa: f64,
    pub humidity_percent: f64,
}

impl Atmosphere {
    pub fn standard() -> Self {
        Self {
            temperature_c: 15.0,
            pressure_hpa: 1013.25,
            humidity_percent: 0.0,
        }
    }

    /// Relative air density compared with ICAO standard atmosphere.
    pub fn density_ratio(&self) -> f64 {
        let standard_temperature_k = 288.15;
        let temperature_k = self.temperature_c + 273.15;

        (self.pressure_hpa / 1013.25) * (standard_temperature_k / temperature_k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_atmosphere_has_unit_density_ratio() {
        assert!((Atmosphere::standard().density_ratio() - 1.0).abs() < f64::EPSILON);
    }
}
