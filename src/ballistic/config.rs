use super::atmosphere::Atmosphere;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationMethod {
    Euler,
    Rk4,
}

#[derive(Debug, Clone, Copy)]
pub struct SolverConfig {
    pub step_size_yards: f64,
    pub atmosphere: Atmosphere,
    pub integration_method: IntegrationMethod,
}

impl Default for SolverConfig {
    fn default() -> Self {
        Self {
            step_size_yards: 1.0,
            atmosphere: Atmosphere::standard(),
            integration_method: IntegrationMethod::Rk4,
        }
    }
}
