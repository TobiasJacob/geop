use std::ops::{Mul, Neg};

use crate::efloat::efloat::EFloat64;

use super::{nonzero_point::NonZeroPoint, point::Point};

#[derive(Debug, Copy, Clone)]
pub struct NormalizedPoint {
    value: Point,
}

// Convert to Point
impl From<NormalizedPoint> for Point {
    fn from(normalized_point: NormalizedPoint) -> Point {
        normalized_point.value
    }
}

impl NormalizedPoint {
    pub fn expect_normalized(as_point: Point) -> Self {
        let norm = as_point.norm().as_efloat();
        assert!(norm != 0.0);
        Self { value: as_point }
    }

    pub fn as_point(self) -> Point {
        self.value
    }

    pub fn as_nonzero(self) -> NonZeroPoint {
        NonZeroPoint::expect_nonzero(self.value)
    }

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
