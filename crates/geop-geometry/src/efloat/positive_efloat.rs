use std::ops::{Add, Div, Mul, Neg, Sub};

use float_next_after::NextAfter;

use super::{efloat::EFloat64, nonzero_efloat::NonzeroEFloat64};

// Wrapper for EFloat64 that ensures that the value is larger than 0.
#[derive(Debug, Clone, Copy)]
pub struct PositiveEFloat64 {
    pub as_efloat: EFloat64,
}

impl PositiveEFloat64 {
    pub fn one() -> Self {
        Self {
            as_efloat: EFloat64::one(),
        }
    }

    pub fn two_pi() -> Self {
        Self {
            as_efloat: EFloat64::two_pi(),
        }
    }

    // pub fn sin(&self) -> EFloat64 {
    //     self.value.sin()
    // }

    // pub fn cos(&self) -> EFloat64 {
    //     self.value.cos()
    // }

    // pub fn acos(&self) -> Self {
    //     let s_u = self.upper_bound.acos();
    //     let s_l = self.lower_bound.acos();
    //     Self {
    //         upper_bound: f64::max(s_u, s_l).next_after(f64::INFINITY),
    //         lower_bound: f64::min(s_u, s_l).next_after(f64::NEG_INFINITY),
    //     }
    // }

    pub fn square(&self) -> PositiveEFloat64 {
        PositiveEFloat64 {
            as_efloat: self.as_efloat * self.as_efloat,
        }
    }

    // pub fn atan2(&self, x: EFloat64) -> EFloat64 {
    //     let a1 = self.lower_bound.atan2(x.lower_bound);
    //     let a2 = self.lower_bound.atan2(x.upper_bound);
    //     let a3 = self.upper_bound.atan2(x.lower_bound);
    //     let a4 = self.upper_bound.atan2(x.upper_bound);
    //     EFloat64 {
    //         upper_bound: a1.max(a2).max(a3).max(a4).next_after(f64::INFINITY),
    //         lower_bound: a1.min(a2).min(a3).min(a4).next_after(f64::NEG_INFINITY),
    //     }
    // }
}

impl Neg for PositiveEFloat64 {
    type Output = EFloat64;

    fn neg(self) -> EFloat64 {
        -self.as_efloat
    }
}

impl Add for PositiveEFloat64 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            as_efloat: self.as_efloat + other.as_efloat,
        }
    }
}

impl Sub for PositiveEFloat64 {
    type Output = EFloat64;

    fn sub(self, other: Self) -> EFloat64 {
        self.as_efloat - other.as_efloat
    }
}

impl Mul for PositiveEFloat64 {
    type Output = PositiveEFloat64;

    fn mul(self, other: Self) -> Self {
        Self {
            as_efloat: self.as_efloat * other.as_efloat,
        }
    }
}

impl Div<NonzeroEFloat64> for PositiveEFloat64 {
    type Output = NonzeroEFloat64;

    // Can only divide by positive numbers.
    fn div(self, other: NonzeroEFloat64) -> NonzeroEFloat64 {
        NonzeroEFloat64 {
            as_efloat: self.as_efloat / other,
        }
    }
}

impl Div<PositiveEFloat64> for PositiveEFloat64 {
    type Output = PositiveEFloat64;

    // Can only divide by positive numbers.
    fn div(self, other: PositiveEFloat64) -> PositiveEFloat64 {
        PositiveEFloat64 {
            as_efloat: self.as_efloat / other,
        }
    }
}

impl PartialEq<f64> for PositiveEFloat64 {
    fn eq(&self, other: &f64) -> bool {
        self.as_efloat == *other
    }
}
