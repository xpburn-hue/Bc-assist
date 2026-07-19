use crate::models::DistanceYards;

#[derive(Debug, Clone, Copy)]
pub struct TimeOfFlightMeasurement {
    pub distance: DistanceYards,
    pub seconds: f64,
}

pub trait TimeOfFlightSensor {
    fn measure(&self) -> TimeOfFlightMeasurement;
}

#[derive(Debug, Clone, Copy)]
pub struct ManualTimeOfFlightSensor {
    pub measurement: TimeOfFlightMeasurement,
}

impl TimeOfFlightSensor for ManualTimeOfFlightSensor {
    fn measure(&self) -> TimeOfFlightMeasurement {
        self.measurement
    }
}
