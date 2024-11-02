use std::{
    f64::consts::PI,
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

use float_next_after::NextAfter;

#[derive(Debug, Clone, Copy)]
pub struct EFloat64 {
    pub upper_bound: f64,
    pub lower_bound: f64,
}

const TWO_PI: f64 = 2.0 * PI;

impl EFloat64 {
    pub fn new(upper_bound: f64, lower_bound: f64) -> Self {
        assert!(upper_bound >= lower_bound);
        assert!(
            upper_bound - lower_bound <= 1E-3,
            "upper: {}, lower: {}",
            upper_bound,
            lower_bound
        );
        assert!(upper_bound.is_finite());
        assert!(lower_bound.is_finite());
        Self {
            upper_bound,
            lower_bound,
        }
    }

    pub fn from(value: f64) -> Self {
        assert!(value.is_finite());
        EFloat64::new(value, value)
    }

    pub fn zero() -> Self {
        EFloat64::new(0.0, 0.0)
    }

    pub fn one() -> Self {
        EFloat64::new(1.0, 1.0)
    }

    pub fn two() -> Self {
        EFloat64::new(2.0, 2.0)
    }

    pub fn half_pi() -> Self {
        EFloat64::new(std::f64::consts::FRAC_PI_2, std::f64::consts::FRAC_PI_2)
    }

    pub fn pi() -> Self {
        EFloat64::new(PI, PI)
    }

    pub fn two_pi() -> Self {
        EFloat64::new(2.0 * PI, 2.0 * PI)
    }

    pub fn sqrt(&self) -> Option<Self> {
        if self.upper_bound < 0.0 {
            return None;
        }
        Some(EFloat64::new(
            self.upper_bound.sqrt() + 1E-15,
            self.lower_bound.max(0.0).sqrt() - 1E-15,
        ))
    }

    // TODO: Add extremal values if max or min values are in the range.
    pub fn sin(&self) -> Self {
        let upper_bound = ((self.upper_bound % TWO_PI) + TWO_PI) % TWO_PI;
        let mut lower_bound = ((self.lower_bound % TWO_PI) + TWO_PI) % TWO_PI;
        if lower_bound > upper_bound {
            lower_bound -= TWO_PI;
        }
        let s_u = self.upper_bound.sin();
        let s_l = self.lower_bound.sin();

        // Check if PI/2 and -PI/2 are in the range.
        if lower_bound <= -PI / 2.0 && upper_bound >= PI / 2.0 {
            return EFloat64::new(1.0, -1.0);
        }

        // PI/2 is in the range.
        if lower_bound <= PI / 2.0 && upper_bound >= PI / 2.0 {
            return EFloat64::new(1.0, s_u.min(s_l) - 1E-15);
        }
        // Check if 3PI/2 is in the range.
        if lower_bound <= 3.0 * PI / 2.0 && upper_bound >= 3.0 * PI / 2.0 {
            return EFloat64::new(s_u.max(s_l) + 1E-15, -1.0);
        }
        // Check if -PI/2 is in the range.
        if lower_bound <= -PI / 2.0 && upper_bound >= -PI / 2.0 {
            return EFloat64::new(1.0, s_u.min(s_l) - 1E-15);
        }
        // Check if -3PI/2 is in the range.
        if lower_bound <= -3.0 * PI / 2.0 && upper_bound >= -3.0 * PI / 2.0 {
            return EFloat64::new(s_u.max(s_l) + 1E-15, -1.0);
        }

        EFloat64::new(s_u.max(s_l) + 1E-15, s_u.min(s_l) - 1E-15)
    }

    pub fn cos(&self) -> Self {
        let upper_bound = ((self.upper_bound % TWO_PI) + TWO_PI) % TWO_PI;
        let mut lower_bound = ((self.lower_bound % TWO_PI) + TWO_PI) % TWO_PI;
        if lower_bound > upper_bound {
            lower_bound -= TWO_PI;
        }

        let s_u = self.upper_bound.cos();
        let s_l = self.lower_bound.cos();
        // Check if 0 and PI are in the range.
        if lower_bound <= 0.0 && upper_bound >= PI {
            return EFloat64::new(1.0, -1.0);
        }
        // Check if 0 an -PI are in the range.
        if lower_bound <= -PI && upper_bound >= 0.0 {
            return EFloat64::new(1.0, -1.0);
        }

        // Check if 0 is in the range.
        if lower_bound <= 0.0 && upper_bound >= 0.0 {
            return EFloat64::new(1.0, s_u.min(s_l) - 1E-15);
        }
        // Check if PI is in the range.
        if lower_bound <= PI && upper_bound >= PI {
            return EFloat64::new(s_u.max(s_l) + 1E-15, -1.0);
        }
        // Check if -PI is in the range.
        if lower_bound <= -PI && upper_bound >= -PI {
            return EFloat64::new(s_u.max(s_l) + 1E-15, -1.0);
        }

        EFloat64::new(s_u.max(s_l) + 1E-15, s_u.min(s_l) - 1E-15)
    }

    pub fn acos(&self) -> Self {
        let s_u = self.upper_bound.acos();
        let s_l = self.lower_bound.acos();
        EFloat64::new(s_u.max(s_l) + 1E-15, s_u.min(s_l) - 1E-15)
    }

    pub fn square(&self) -> EFloat64 {
        let s_u = self.upper_bound * self.upper_bound;
        let s_l = self.lower_bound * self.lower_bound;
        EFloat64::new(
            s_u.max(s_l).next_after(f64::INFINITY),
            s_u.min(s_l).next_after(f64::NEG_INFINITY),
        )
    }

    pub fn atan2(&self, x: EFloat64) -> EFloat64 {
        let a1 = self.lower_bound.atan2(x.lower_bound);
        let a2 = self.lower_bound.atan2(x.upper_bound);
        let a3 = self.upper_bound.atan2(x.lower_bound);
        let a4 = self.upper_bound.atan2(x.upper_bound);

        let upper_bound = a1.max(a2).max(a3).max(a4);
        let lower_bound = a1.min(a2).min(a3).min(a4);

        // This is for the case that atan2 wraps around.
        if upper_bound - lower_bound <= PI {
            return EFloat64::new(upper_bound, lower_bound);
        }

        let a1 = (a1 + TWO_PI) % TWO_PI;
        let a2 = (a2 + TWO_PI) % TWO_PI;
        let a3 = (a3 + TWO_PI) % TWO_PI;
        let a4 = (a4 + TWO_PI) % TWO_PI;

        EFloat64::new(
            a1.max(a2).max(a3).max(a4) + 1E-15,
            a1.min(a2).min(a3).min(a4) - 1E-15,
        )
    }

    pub fn abs(&self) -> Self {
        if self.lower_bound >= 0.0 {
            return *self;
        }
        if self.upper_bound <= 0.0 {
            return -*self;
        }
        Self {
            upper_bound: (-self.lower_bound)
                .max(self.upper_bound)
                .next_after(f64::INFINITY),
            lower_bound: 0.0,
        }
    }

    pub fn powi(&self, n: i32) -> Self {
        let s_u = self.upper_bound.powi(n);
        let s_l = self.lower_bound.powi(n);
        Self {
            upper_bound: s_u.next_after(f64::INFINITY),
            lower_bound: s_l.next_after(f64::NEG_INFINITY),
        }
    }

    pub fn max(self, other: Self) -> Self {
        Self {
            upper_bound: self.upper_bound.max(other.upper_bound),
            lower_bound: self.lower_bound.max(other.lower_bound),
        }
    }

    pub fn min(self, other: Self) -> Self {
        Self {
            upper_bound: self.upper_bound.min(other.upper_bound),
            lower_bound: self.lower_bound.min(other.lower_bound),
        }
    }

    pub fn ceil(&self) -> usize {
        self.upper_bound.ceil() as usize
    }

    pub fn floor(&self) -> usize {
        self.lower_bound.floor() as usize
    }
}

impl Neg for EFloat64 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            upper_bound: -self.lower_bound,
            lower_bound: -self.upper_bound,
        }
    }
}

impl Add for EFloat64 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            upper_bound: (self.upper_bound + other.upper_bound).next_after(f64::INFINITY),
            lower_bound: (self.lower_bound + other.lower_bound).next_after(f64::NEG_INFINITY),
        }
    }
}

impl Sub for EFloat64 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            upper_bound: (self.upper_bound - other.lower_bound).next_after(f64::INFINITY),
            lower_bound: (self.lower_bound - other.upper_bound).next_after(f64::NEG_INFINITY),
        }
    }
}

impl Mul for EFloat64 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let p1 = self.lower_bound * other.lower_bound;
        let p2 = self.lower_bound * other.upper_bound;
        let p3 = self.upper_bound * other.lower_bound;
        let p4 = self.upper_bound * other.upper_bound;
        Self {
            upper_bound: p1.max(p2).max(p3).max(p4).next_after(f64::INFINITY),
            lower_bound: p1.min(p2).min(p3).min(p4).next_after(f64::NEG_INFINITY),
        }
    }
}

impl Div<EFloat64> for EFloat64 {
    type Output = Option<EFloat64>;

    // Can only divide by positive numbers.
    fn div(self, other: EFloat64) -> Option<EFloat64> {
        if other.lower_bound <= 0.0 && other.upper_bound >= 0.0 {
            return None;
        }
        let d1 = self.lower_bound / other.lower_bound;
        let d2 = self.lower_bound / other.upper_bound;
        let d3 = self.upper_bound / other.lower_bound;
        let d4 = self.upper_bound / other.upper_bound;
        Some(Self {
            upper_bound: d1.max(d2).max(d3).max(d4).next_after(f64::INFINITY),
            lower_bound: d1.min(d2).min(d3).min(d4).next_after(f64::NEG_INFINITY),
        })
    }
}

impl PartialEq<f64> for EFloat64 {
    fn eq(&self, other: &f64) -> bool {
        self.lower_bound <= *other && *other <= self.upper_bound
    }
}

impl PartialEq<EFloat64> for EFloat64 {
    fn eq(&self, other: &EFloat64) -> bool {
        self.lower_bound <= other.upper_bound && other.lower_bound <= self.upper_bound
    }
}

// Now < and > operators
impl PartialOrd<f64> for EFloat64 {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        if self.upper_bound < *other {
            Some(std::cmp::Ordering::Less)
        } else if self.lower_bound > *other {
            Some(std::cmp::Ordering::Greater)
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}

impl Display for EFloat64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Write Scientific notation
        write!(
            f,
            "[{:.2e} +- {:.2e}]",
            (self.upper_bound + self.lower_bound) / 2.0,
            (self.upper_bound - self.lower_bound) / 2.0
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_efloat_add() {
        let a = EFloat64::from(0.1);
        let b = EFloat64::from(0.2);
        let c = a + b;
        println!("c: {:?}", c);
        assert!(c == 0.3);
    }

    #[test]
    fn test_efloat_add_f64() {
        let a = EFloat64::from(2.0);
        let b = EFloat64::from(8.0);
        assert!((b.sqrt().unwrap() * a.sqrt().unwrap()) == 4.0);
    }
}
