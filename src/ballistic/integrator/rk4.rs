/// Fourth-order Runge-Kutta integration step for generic state vectors.
///
/// Solves the differential equation:
///
/// dy/dt = f(t, y)
///
pub fn rk4_step<F>(t: f64, y: &[f64], dt: f64, derivative: F) -> Vec<f64>
where
    F: Fn(f64, &[f64]) -> Vec<f64>,
{
    let k1 = derivative(t, y);
    let k2 = derivative(t + dt * 0.5, &add_scaled(y, &k1, dt * 0.5));
    let k3 = derivative(t + dt * 0.5, &add_scaled(y, &k2, dt * 0.5));
    let k4 = derivative(t + dt, &add_scaled(y, &k3, dt));

    y.iter()
        .enumerate()
        .map(|(i, value)| value + dt / 6.0 * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]))
        .collect()
}

fn add_scaled(y: &[f64], k: &[f64], scale: f64) -> Vec<f64> {
    y.iter()
        .enumerate()
        .map(|(i, value)| value + scale * k[i])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::rk4_step;

    #[test]
    fn exponential_growth() {
        let result = rk4_step(0.0, &[1.0], 1.0, |_t, y| vec![y[0]]);
        assert!((result[0] - std::f64::consts::E).abs() < 0.001);
    }

    #[test]
    fn constant_acceleration() {
        let g = 9.80665;
        let result = rk4_step(0.0, &[0.0], 2.0, |_t, _y| vec![g]);
        assert!((result[0] - 2.0 * g).abs() < 1e-10);
    }
}
