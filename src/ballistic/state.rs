/// Point-mass state vector used by the numerical trajectory solver.
///
/// State ordering:
/// [position_x, position_y, velocity_x, velocity_y]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StateVector {
    pub position_x: f64,
    pub position_y: f64,
    pub velocity_x: f64,
    pub velocity_y: f64,
}

impl StateVector {
    pub fn as_vec(self) -> Vec<f64> {
        vec![
            self.position_x,
            self.position_y,
            self.velocity_x,
            self.velocity_y,
        ]
    }

    pub fn from_vec(values: &[f64]) -> Self {
        assert_eq!(values.len(), 4);
        Self {
            position_x: values[0],
            position_y: values[1],
            velocity_x: values[2],
            velocity_y: values[3],
        }
    }
}
