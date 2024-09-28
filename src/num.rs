const DEFAULT_SCALE: f64 = 1000000f64;

pub(crate) fn round_default_scale(value: f64) -> f64 {
    round_with_scale(value, DEFAULT_SCALE)
}

pub(crate) fn round_with_scale(value: f64, scale: f64) -> f64 {
    (value * scale).round() / scale
}
