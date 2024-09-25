pub(crate) trait ApproxEq {
    fn approx_eq(&self, other: Self) -> bool;
    fn approx_eq_with_epsilon(&self, other: Self, epsilon: Self) -> bool;
}

impl ApproxEq for f64 {
    fn approx_eq(&self, other: f64) -> bool {
        self.approx_eq_with_epsilon(other, f64::EPSILON)
    }

    fn approx_eq_with_epsilon(&self, other: f64, epsilon: f64) -> bool {
        (self - other).abs() < epsilon
    }
}
