use std::ops::{Mul, Neg};

use crate::efloat::efloat::EFloat64;

use super::point::Point;

#[derive(Debug, Copy, Clone)]
pub struct NormalizedPoint {
    pub as_point: Point,
}

// Convert to Point
impl From<NormalizedPoint> for Point {
    fn from(normalized_point: NormalizedPoint) -> Point {
        normalized_point.as_point
    }
}

impl NormalizedPoint {
    pub fn parallel_decomposition(self, other: Point) -> Point {
        let dot = self.as_point.dot(other);
        self.as_point * dot
    }

    pub fn perpendicular_decomposition(self, other: Point) -> Point {
        other - self.parallel_decomposition(other)
    }

    pub fn parallel_distance(self, other: Point) -> EFloat64 {
        self.as_point.dot(other)
    }
}

impl Neg for NormalizedPoint {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            as_point: -self.as_point,
        }
    }
}

impl PartialEq for NormalizedPoint {
    fn eq(&self, other: &NormalizedPoint) -> bool {
        self.as_point == other.as_point
    }
}

impl Mul<EFloat64> for NormalizedPoint {
    type Output = Point;

    fn mul(self, other: EFloat64) -> Point {
        self.as_point * other
    }
}
