use std::ops::{Add, Div, Mul, Neg, Sub};

use float_next_after::NextAfter;

use super::{efloat::EFloat64, positive_efloat::PositiveEFloat64};

// Wrapper for EFloat64 that ensures that the value is larger than 0.
#[derive(Debug, Clone, Copy)]
pub struct NonzeroEFloat64 {
    pub value: EFloat64,
}

impl NonzeroEFloat64 {
    pub fn one() -> Self {
        Self {
            value: EFloat64::one(),
        }
    }

    pub fn two_pi() -> Self {
        Self {
            value: EFloat64::two_pi(),
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
            value: self.value * self.value,
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

impl Neg for NonzeroEFloat64 {
    type Output = NonzeroEFloat64;

    fn neg(self) -> NonzeroEFloat64 {
        Self { value: -self.value }
    }
}

impl Add for NonzeroEFloat64 {
    type Output = EFloat64;

    fn add(self, other: Self) -> EFloat64 {
        self.value + other.value
    }
}

impl Sub for NonzeroEFloat64 {
    type Output = EFloat64;

    fn sub(self, other: Self) -> EFloat64 {
        self.value - other.value
    }
}

impl Mul for NonzeroEFloat64 {
    type Output = NonzeroEFloat64;

    fn mul(self, other: Self) -> Self {
        Self {
            value: self.value * other.value,
        }
    }
}

impl Div<NonzeroEFloat64> for NonzeroEFloat64 {
    type Output = NonzeroEFloat64;

    // Can only divide by positive numbers.
    fn div(self, other: NonzeroEFloat64) -> NonzeroEFloat64 {
        NonzeroEFloat64 {
            value: self.value / other,
        }
    }
}

impl Div<PositiveEFloat64> for NonzeroEFloat64 {
    type Output = NonzeroEFloat64;

    // Can only divide by positive numbers.
    fn div(self, other: PositiveEFloat64) -> NonzeroEFloat64 {
        NonzeroEFloat64 {
            value: self.value / other,
        }
    }
}

impl PartialEq<f64> for NonzeroEFloat64 {
    fn eq(&self, other: &f64) -> bool {
        self.value == *other
    }
}
