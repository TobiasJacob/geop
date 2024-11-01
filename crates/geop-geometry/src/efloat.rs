use std::ops::{Add, Div, Mul, Neg, Sub};

use float_next_after::NextAfter;

#[derive(Debug, Clone, Copy)]
pub struct EFloat64 {
    pub upper_bound: f64,
    pub lower_bound: f64,
}

impl EFloat64 {
    pub fn new(value: f64) -> Self {
        Self {
            upper_bound: value,
            lower_bound: value,
        }
    }

    pub fn zero() -> Self {
        Self {
            upper_bound: 0.0,
            lower_bound: 0.0,
        }
    }

    pub fn one() -> Self {
        Self {
            upper_bound: 1.0,
            lower_bound: 1.0,
        }
    }

    pub fn pi() -> Self {
        Self {
            upper_bound: std::f64::consts::PI,
            lower_bound: std::f64::consts::PI,
        }
    }

    pub fn two_pi() -> Self {
        Self {
            upper_bound: 2.0 * std::f64::consts::PI,
            lower_bound: 2.0 * std::f64::consts::PI,
        }
    }

    pub fn sqrt(&self) -> Option<Self> {
        if self.lower_bound < 0.0 {
            return None;
        }
        Some(Self {
            upper_bound: self.upper_bound.sqrt().next_after(f64::INFINITY),
            lower_bound: self.lower_bound.sqrt().next_after(f64::NEG_INFINITY),
        })
    }

    pub fn sin(&self) -> Self {
        let s_u = self.upper_bound.sin();
        let s_l = self.lower_bound.sin();
        Self {
            upper_bound: f64::max(s_u, s_l).next_after(f64::INFINITY),
            lower_bound: f64::min(s_u, s_l).next_after(f64::NEG_INFINITY),
        }
    }

    pub fn cos(&self) -> Self {
        let s_u = self.upper_bound.cos();
        let s_l = self.lower_bound.cos();
        Self {
            upper_bound: f64::max(s_u, s_l).next_after(f64::INFINITY),
            lower_bound: f64::min(s_u, s_l).next_after(f64::NEG_INFINITY),
        }
    }

    pub fn acos(&self) -> Self {
        let s_u = self.upper_bound.acos();
        let s_l = self.lower_bound.acos();
        Self {
            upper_bound: f64::max(s_u, s_l).next_after(f64::INFINITY),
            lower_bound: f64::min(s_u, s_l).next_after(f64::NEG_INFINITY),
        }
    }

    pub fn square(&self) -> EFloat64 {
        let s_u = self.upper_bound * self.upper_bound;
        let s_l = self.lower_bound * self.lower_bound;
        EFloat64 {
            upper_bound: s_u.next_after(f64::INFINITY),
            lower_bound: s_l.next_after(f64::NEG_INFINITY),
        }
    }

    pub fn atan2(&self, x: EFloat64) -> EFloat64 {
        let a1 = self.lower_bound.atan2(x.lower_bound);
        let a2 = self.lower_bound.atan2(x.upper_bound);
        let a3 = self.upper_bound.atan2(x.lower_bound);
        let a4 = self.upper_bound.atan2(x.upper_bound);
        EFloat64 {
            upper_bound: a1.max(a2).max(a3).max(a4).next_after(f64::INFINITY),
            lower_bound: a1.min(a2).min(a3).min(a4).next_after(f64::NEG_INFINITY),
        }
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
        if other == 0.0 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_efloat_add() {
        let a = EFloat64::new(0.1);
        let b = EFloat64::new(0.2);
        let c = a + b;
        println!("c: {:?}", c);
        assert!(c == 0.3);
    }

    #[test]
    fn test_efloat_add_f64() {
        let a = EFloat64::new(2.0);
        let b = EFloat64::new(8.0);
        assert!((b.sqrt().unwrap() * a.sqrt().unwrap()) == 4.0);
    }
}
