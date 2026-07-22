use super::atmosphere::Atmosphere;
use super::wind::Wind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationMethod {
    Euler,
    RK4,
}

#[derive(Debug, Clone, Copy)]
pub struct SolverConfig {
    pub step_size_yards: f64,
    pub atmosphere: Atmosphere,
    pub wind: Wind,
    pub integration_method: IntegrationMethod,
}

impl Default for SolverConfig {
    fn default() -> Self {
        Self {
            step_size_yards: 1.0,
            atmosphere: Atmosphere::standard(),
            wind: Wind::calm(),
            integration_method: IntegrationMethod::RK4,
        }
    }
}
