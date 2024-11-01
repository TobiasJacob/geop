use std::ops::{Mul, Neg};

use crate::efloat::efloat::EFloat64;

use super::point::Point;

#[derive(Debug, Copy, Clone)]
pub struct NormalizedPoint {
    pub value: Point,
}

impl NormalizedPoint {
    pub fn parallel_decomposition(self, other: Point) -> Point {
        let dot = self.value.dot(other);
        self.value * dot
    }

    pub fn perpendicular_decomposition(self, other: Point) -> Point {
        other - self.parallel_decomposition(other)
    }

    pub fn parallel_distance(self, other: Point) -> EFloat64 {
        self.value.dot(other)
    }
}

impl Neg for NormalizedPoint {
    type Output = Self;

    fn neg(self) -> Self {
        Self { value: -self.value }
    }
}

impl PartialEq for NormalizedPoint {
    fn eq(&self, other: &NormalizedPoint) -> bool {
        self.value == other.value
    }
}

impl Mul<EFloat64> for NormalizedPoint {
    type Output = Point;

    fn mul(self, other: EFloat64) -> Point {
        self.value * other
    }
}
