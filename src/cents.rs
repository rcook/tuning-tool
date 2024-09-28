use crate::ratio::Ratio;

pub(crate) struct Cents(pub(crate) f64);

impl Cents {
    // c.f. centsToValue
    pub(crate) fn to_ratio(&self) -> Ratio {
        Ratio(2f64.powf(self.0 / 1200f64))
    }
}
