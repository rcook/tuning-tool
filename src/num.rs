pub(crate) fn is_u7(b: u8) -> bool {
    b < 128
}

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

#[cfg(test)]
mod tests {
    use crate::num::ApproxEq;

    #[test]
    fn basics() {
        assert!(0f64.approx_eq(0f64));
        assert!(!0.0001f64.approx_eq(0f64));
        assert!(!0.0001f64.approx_eq_with_epsilon(0f64, 0.00001f64));
        assert!(!0.0001f64.approx_eq_with_epsilon(0f64, 0.0001f64));
        assert!(0.0001f64.approx_eq_with_epsilon(0f64, 0.001f64));
        assert!(0.0001f64.approx_eq_with_epsilon(0f64, 0.01f64));
        assert!(0.0001f64.approx_eq_with_epsilon(0f64, 0.1f64));
    }
}
