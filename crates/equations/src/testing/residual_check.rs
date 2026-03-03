pub fn residual_is_zero(value: f64, abs: f64, rel: f64, scale: f64) -> bool {
    value.abs() <= abs + rel * scale.abs().max(1.0)
}
