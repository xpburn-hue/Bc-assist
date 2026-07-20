//! Numerical integration utilities for ballistic solvers.

/// State transition function used by the Runge-Kutta integrator.
pub type DerivativeFn<S> = fn(&S, f64) -> S;

/// Generic fourth-order Runge-Kutta integrator.
///
/// The caller supplies the current state, time, step size, and derivative
/// function. This keeps the numerical method independent of the point-mass
/// model and allows future solvers to reuse the same integration core.
pub fn rk4_step<S>(state: S, time: f64, step: f64, derivative: DerivativeFn<S>) -> S
where
    S: Copy,
    S: std::ops::Add<Output = S>,
    S: std::ops::Mul<f64, Output = S>,
{
    let k1 = derivative(&state, time);
    let k2 = derivative(&(state + k1 * (step * 0.5)), time + step * 0.5);
    let k3 = derivative(&(state + k2 * (step * 0.5)), time + step * 0.5);
    let k4 = derivative(&(state + k3 * step), time + step);

    state + (k1 + k2 * 2.0 + k3 * 2.0 + k4) * (step / 6.0)
}

#[cfg(test)]
mod tests {
    use super::rk4_step;

    fn exponential(state: &f64, _time: f64) -> f64 {
        *state
    }

    #[test]
    fn integrates_exponential_growth() {
        let result = rk4_step(1.0, 0.0, 0.1, exponential);
        assert!((result - 1.1051708).abs() < 1e-6);
    }
}
