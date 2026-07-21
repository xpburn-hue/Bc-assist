/// First-order Euler integration step for generic state vectors.
pub fn euler_step<F>(t: f64, y: &[f64], dt: f64, derivative: F) -> Vec<f64>
where
    F: Fn(f64, &[f64]) -> Vec<f64>,
{
    let dy = derivative(t, y);
    y.iter()
        .enumerate()
        .map(|(i, value)| value + dt * dy[i])
        .collect()
}
