use std::ops::{Mul, Neg};

use crate::efloat::{efloat::EFloat64, positive_efloat::PositiveEFloat64};

use super::{normalized_point::NormalizedPoint, point::Point};

#[derive(Debug, Copy, Clone)]
pub struct NonZeroPoint {
    pub as_point: Point,
}

impl NonZeroPoint {
    pub fn normalize(self) -> NormalizedPoint {
        self.as_point.normalize().unwrap()
    }

    pub fn norm_sq(self) -> PositiveEFloat64 {
        self.as_point.norm_sq().as_float.expect_positive()
    }

    pub fn norm(self) -> PositiveEFloat64 {
        self.as_point.norm().as_float.expect_positive()
    }

    pub fn parallel_decomposition(self, other: impl Into<Point>) -> Point {
        self.as_point.dot(other) / self.norm_sq() * self.as_point
    }

    pub fn perpendicular_decomposition(self, other: impl Into<Point>) -> Point {
        let other = other.into();
        other - self.parallel_decomposition(other)
    }

    pub fn parallel_distance(self, other: impl Into<Point>) -> EFloat64 {
        self.as_point.dot(other) / self.norm()
    }
}

impl Neg for NonZeroPoint {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            as_point: -self.as_point,
        }
    }
}

impl PartialEq for NonZeroPoint {
    fn eq(&self, other: &NonZeroPoint) -> bool {
        self.as_point == other.as_point
    }
}

impl Mul<EFloat64> for NonZeroPoint {
    type Output = Point;

    fn mul(self, other: EFloat64) -> Point {
        self.as_point * other
    }
}
