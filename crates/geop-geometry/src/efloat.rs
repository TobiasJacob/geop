use std::ops::{Add, Mul};

use float_next_after::NextAfter;

// TODO: Use this in the future for doing calculations with floating point numbers.
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

    pub fn is_eq(&self, other: f64) -> bool {
        self.lower_bound <= other && other <= self.upper_bound
    }

    pub fn sqrt(&self) -> Self {
        Self {
            upper_bound: self.upper_bound.sqrt().next_after(f64::INFINITY),
            lower_bound: self.lower_bound.sqrt().next_after(f64::NEG_INFINITY),
        }
    }

    pub fn sin(&self) -> Self {
        let s_u = self.upper_bound.sin();
        let s_l = self.lower_bound.sin();
        Self {
            upper_bound: f64::max(s_u, s_l).next_after(f64::INFINITY),
            lower_bound: f64::min(s_u, s_l).next_after(f64::NEG_INFINITY),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_efloat_add() {
        let a = EFloat64::new(0.1);
        let b = EFloat64::new(0.2);
        let c = a + b;
        println!("c: {:?}", c);
        assert!(c.is_eq(0.3));
    }

    #[test]
    fn test_efloat_add_f64() {
        let a = EFloat64::new(2.0);
        let b = EFloat64::new(8.0);
        assert!((b.sqrt() * a.sqrt()).is_eq(4.0));
    }
}
