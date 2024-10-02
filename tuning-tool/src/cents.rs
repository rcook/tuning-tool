use crate::ratio::Ratio;
use crate::types::f64_newtype;

f64_newtype!(Cents, pub(crate));

impl Cents {
    // c.f. centsToValue
    pub(crate) fn to_ratio(&self) -> Ratio {
        Ratio(2f64.powf(self.0 / 1200f64))
    }
}

#[cfg(test)]
mod tests {
    use crate::approx_eq::ApproxEq;
    use crate::cents::Cents;

    #[test]
    fn to_ratio() {
        assert!(Cents(1200f64).to_ratio().0.approx_eq(2f64));
    }
}
