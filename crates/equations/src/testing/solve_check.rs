pub fn values_close(expected: f64, actual: f64, abs: f64, rel: f64) -> bool {
    let diff = (expected - actual).abs();
    diff <= abs + rel * expected.abs().max(actual.abs()).max(1.0)
}
