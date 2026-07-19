use bc_assist::models::{
    BallisticCoefficient, DistanceYards, PressureInHg, PressurePascal, VelocityFps,
};

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

#[test]
fn pressure_converts_inhg_to_pascal() {
    let pressure = PressureInHg(29.92);
    let pascal = pressure.to_pascal();

    assert!((pascal.0 - 101325.0).abs() < 100.0);
}

#[test]
fn pressure_converts_pascal_to_inhg() {
    let pressure = PressurePascal(101325.0);
    let inhg = pressure.to_inhg();

    assert!((inhg.0 - 29.92).abs() < 0.05);
}
