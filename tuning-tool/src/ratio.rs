use crate::cents::Cents;
use crate::types::f64_newtype;

f64_newtype!(Ratio, pub(crate));

impl Ratio {
    // c.f. valueToCents
    pub(crate) fn to_cents(&self) -> Cents {
        Cents(1200f64 * self.0.log2())
    }
}

#[cfg(test)]
mod tests {
    use crate::approx_eq::ApproxEq;
    use crate::ratio::Ratio;

    #[test]
    fn to_cents() {
        assert!(Ratio(3f64 / 2f64).to_cents().0.approx_eq(701.9550008653874));
    }
}
