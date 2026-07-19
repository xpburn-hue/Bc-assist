use bc_assist::models::{BallisticCoefficient, DistanceYards, VelocityFps};

#[test]
fn distance_preserves_value() {
    let distance = DistanceYards(100.0);
    assert_eq!(distance.0, 100.0);
}

#[test]
fn velocity_preserves_value() {
    let velocity = VelocityFps(2500.0);
    assert_eq!(velocity.0, 2500.0);
}

#[test]
fn bc_preserves_value() {
    let bc = BallisticCoefficient(0.620);
    assert_eq!(bc.0, 0.620);
}
