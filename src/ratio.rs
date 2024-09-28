use crate::cents::Cents;

pub(crate) struct Ratio(pub(crate) f64);

impl Ratio {
    // c.f. valueToCents
    pub(crate) fn to_cents(&self) -> Cents {
        Cents(1200f64 * self.0.log2())
    }
}
