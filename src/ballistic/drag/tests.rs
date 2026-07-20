#[cfg(test)]
mod tests {
    use super::super::{g1::G1, g7::G7, DragFunction};

    #[test]
    fn drag_models_return_positive_retardation() {
        assert!(G1.retardation(2500.0) > 0.0);
        assert!(G7.retardation(2500.0) > 0.0);
    }

    #[test]
    fn g7_has_lower_placeholder_drag_than_g1() {
        assert!(G7.retardation(2500.0) < G1.retardation(2500.0));
    }
}
