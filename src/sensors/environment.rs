use crate::models::{PressureInHg, TemperatureF};

#[derive(Debug, Clone, Copy)]
pub struct EnvironmentReading {
    pub temperature: TemperatureF,
    pub pressure: PressureInHg,
    pub humidity_percent: f64,
}

pub trait EnvironmentSensor {
    fn read(&self) -> EnvironmentReading;
}

#[derive(Debug, Clone, Copy)]
pub struct ManualEnvironmentSensor {
    pub reading: EnvironmentReading,
}

impl EnvironmentSensor for ManualEnvironmentSensor {
    fn read(&self) -> EnvironmentReading {
        self.reading
    }
}
