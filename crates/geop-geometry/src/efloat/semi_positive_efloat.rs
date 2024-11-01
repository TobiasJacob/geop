use std::ops::{Add, Div, Mul, Neg, Sub};

use float_next_after::NextAfter;

use super::{efloat::EFloat64, nonzero_efloat::NonzeroEFloat64, positive_efloat::PositiveEFloat64};

// Wrapper for EFloat64 that ensures that the value is larger or equal to 0.
#[derive(Debug, Clone, Copy)]
pub struct SemiPositiveEFloat64 {
    pub as_float: EFloat64,
}

impl SemiPositiveEFloat64 {
    pub fn zero() -> Self {
        Self {
            as_float: EFloat64::zero(),
        }
    }

    pub fn one() -> Self {
        Self {
            as_float: EFloat64::one(),
        }
    }

    pub fn two_pi() -> Self {
        Self {
            as_float: EFloat64::two_pi(),
        }
    }

    pub fn sqrt(&self) -> Self {
        let s_u = self.as_float.upper_bound.sqrt();
        let s_l = self.as_float.lower_bound.sqrt();
        Self {
            as_float: EFloat64 {
                upper_bound: s_u.next_after(f64::INFINITY),
                lower_bound: s_l.next_after(f64::NEG_INFINITY),
            },
        }
    }
}

impl Neg for SemiPositiveEFloat64 {
    type Output = EFloat64;

    fn neg(self) -> EFloat64 {
        -self.as_float
    }
}

impl Add for SemiPositiveEFloat64 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            as_float: self.as_float + other.as_float,
        }
    }
}

impl Sub for SemiPositiveEFloat64 {
    type Output = EFloat64;

    fn sub(self, other: Self) -> EFloat64 {
        self.as_float - other.as_float
    }
}

impl Mul for SemiPositiveEFloat64 {
    type Output = SemiPositiveEFloat64;

    fn mul(self, other: Self) -> Self {
        Self {
            as_float: self.as_float * other.as_float,
        }
    }
}

impl Div<NonzeroEFloat64> for SemiPositiveEFloat64 {
    type Output = NonzeroEFloat64;

    // Can only divide by positive numbers.
    fn div(self, other: NonzeroEFloat64) -> NonzeroEFloat64 {
        NonzeroEFloat64 {
            value: self.as_float / other,
        }
    }
}

impl Div<PositiveEFloat64> for SemiPositiveEFloat64 {
    type Output = SemiPositiveEFloat64;

    // Can only divide by positive numbers.
    fn div(self, other: PositiveEFloat64) -> SemiPositiveEFloat64 {
        SemiPositiveEFloat64 {
            as_float: self.as_float / other,
        }
    }
}

impl PartialEq<f64> for SemiPositiveEFloat64 {
    fn eq(&self, other: &f64) -> bool {
        self.as_float == *other
    }
}
