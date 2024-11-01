use std::ops::{Mul, Neg};

use crate::efloat::{efloat::EFloat64, positive_efloat::PositiveEFloat64};

use super::{normalized_point::NormalizedPoint, point::Point};

#[derive(Debug, Copy, Clone)]
pub struct NonZeroPoint {
    pub value: Point,
}

impl NonZeroPoint {
    pub fn normalize(self) -> NormalizedPoint {
        self.value.normalize().unwrap()
    }

    pub fn norm_sq(self) -> PositiveEFloat64 {
        self.value.norm_sq().value.expect_positive()
    }

    pub fn norm(self) -> PositiveEFloat64 {
        self.value.norm().value.expect_positive()
    }

    pub fn parallel_decomposition(self, other: Point) -> Point {
        self.value.dot(other) / self.norm_sq() * self.value
    }

    pub fn perpendicular_decomposition(self, other: Point) -> Point {
        other - self.parallel_decomposition(other)
    }

    pub fn parallel_distance(self, other: Point) -> EFloat64 {
        self.value.dot(other) / self.norm()
    }
}

impl Neg for NonZeroPoint {
    type Output = Self;

    fn neg(self) -> Self {
        Self { value: -self.value }
    }
}

impl PartialEq for NonZeroPoint {
    fn eq(&self, other: &NonZeroPoint) -> bool {
        self.value == other.value
    }
}

impl Mul<EFloat64> for NonZeroPoint {
    type Output = Point;

    fn mul(self, other: EFloat64) -> Point {
        self.value * other
    }
}