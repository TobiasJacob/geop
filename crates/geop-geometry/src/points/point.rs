use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::efloat::{
    efloat::EFloat64, positive_efloat::PositiveEFloat64, semi_positive_efloat::SemiPositiveEFloat64,
};

use super::{nonzero_point::NonZeroPoint, normalized_point::NormalizedPoint};

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: EFloat64,
    pub y: EFloat64,
    pub z: EFloat64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point {
            x: EFloat64::new(x),
            y: EFloat64::new(y),
            z: EFloat64::new(z),
        }
    }

    pub fn from_efloat(x: EFloat64, y: EFloat64, z: EFloat64) -> Point {
        Point { x, y, z }
    }

    pub fn norm_sq(self) -> SemiPositiveEFloat64 {
        self.x.square() + self.y.square() + self.z.square()
    }

    pub fn norm(self) -> SemiPositiveEFloat64 {
        self.norm_sq().sqrt()
    }

    pub fn is_zero(self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0
    }

    pub fn dot(self, other: Point) -> EFloat64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Point) -> Point {
        Point::from_efloat(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    // TODO: This operation should return a Result, as the operation is not always possible. It fails when norm is zero. This is not as rare as it seems. It helps users to catch bugs.
    pub fn normalize(self) -> Option<NormalizedPoint> {
        let norm = self.norm().value.as_positive()?;
        Some(NormalizedPoint {
            value: Point::from_efloat(self.x / norm, self.y / norm, self.z / norm),
        })
    }

    pub fn is_parallel(self, other: Point) -> bool {
        let cross = self.cross(other);
        cross.is_zero()
    }

    pub fn is_perpendicular(self, other: Point) -> bool {
        let dot = self.dot(other);
        dot == 0.0
    }

    pub fn angle(&self, other: Point) -> Option<SemiPositiveEFloat64> {
        let dot = self.dot(other);
        let norm = (self.norm() * other.norm()).value.as_positive()?;
        let dot_norm = dot / norm;
        if dot_norm >= 1.0 {
            return Some(EFloat64::new(0.0).expect_semi_positive());
        }
        if dot_norm <= -1.0 {
            return Some(EFloat64::new(std::f64::consts::PI).expect_semi_positive());
        }
        Some(dot_norm.acos().expect_semi_positive())
    }

    // Oriented angle between two vectors around a normal vector. Measured from self to other.
    pub fn angle2(&self, other: Point, normal: Point) -> Option<EFloat64> {
        let cross = self.cross(other);
        let angle = self.angle(other)?;
        if cross.dot(normal) < 0.0 {
            return Some(-angle.value);
        }
        Some(angle.value)
    }

    pub fn zero() -> Point {
        Point::from_efloat(EFloat64::zero(), EFloat64::zero(), EFloat64::zero())
    }

    pub fn unit_x() -> Point {
        Point::from_efloat(EFloat64::one(), EFloat64::zero(), EFloat64::zero())
    }

    pub fn unit_y() -> Point {
        Point::from_efloat(EFloat64::zero(), EFloat64::one(), EFloat64::zero())
    }

    pub fn unit_z() -> Point {
        Point::from_efloat(EFloat64::zero(), EFloat64::zero(), EFloat64::one())
    }

    pub fn as_non_zero(self) -> Option<NonZeroPoint> {
        if self.is_zero() {
            None
        } else {
            Some(NonZeroPoint { value: self })
        }
    }

    pub fn expect_non_zero(self) -> NonZeroPoint {
        assert!(!self.is_zero());
        NonZeroPoint { value: self }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Point) -> Point {
        Point::from_efloat(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Point) -> Point {
        Point::from_efloat(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Add<EFloat64> for Point {
    type Output = Self;

    fn add(self, other: EFloat64) -> Point {
        Point::from_efloat(self.x + other, self.y + other, self.z + other)
    }
}

impl Sub<EFloat64> for Point {
    type Output = Self;

    fn sub(self, other: EFloat64) -> Point {
        Point::from_efloat(self.x - other, self.y - other, self.z - other)
    }
}

impl Mul<EFloat64> for Point {
    type Output = Self;

    fn mul(self, other: EFloat64) -> Point {
        Point::from_efloat(self.x * other, self.y * other, self.z * other)
    }
}

impl Mul<Point> for EFloat64 {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        Point::from_efloat(self * other.x, self * other.y, self * other.z)
    }
}

impl Div<PositiveEFloat64> for Point {
    type Output = Self;

    fn div(self, other: PositiveEFloat64) -> Point {
        Point::from_efloat(self.x / other, self.y / other, self.z / other)
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Point {
        Point::from_efloat(-self.x, -self.y, -self.z)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        (self.x - other.x) == 0.0 && (self.y - other.y) == 0.0 && (self.z - other.z) == 0.0
    }
}
