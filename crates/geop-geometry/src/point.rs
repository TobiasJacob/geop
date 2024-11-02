use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::efloat::EFloat64;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: EFloat64,
    pub y: EFloat64,
    pub z: EFloat64,
}

impl Point {
    pub fn new(x: EFloat64, y: EFloat64, z: EFloat64) -> Point {
        Point { x, y, z }
    }

    pub fn from_f64(x: f64, y: f64, z: f64) -> Point {
        Point {
            x: EFloat64::from(x),
            y: EFloat64::from(y),
            z: EFloat64::from(z),
        }
    }

    pub fn norm_sq(self) -> EFloat64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn norm(self) -> EFloat64 {
        self.norm_sq().sqrt().unwrap()
    }

    pub fn is_normalized(self) -> bool {
        return self.norm_sq() == 1.0;
    }

    pub fn is_zero(self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0
    }

    pub fn dot(self, other: Point) -> EFloat64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Point) -> Point {
        Point::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn normalize(self) -> Option<Point> {
        let norm = self.norm();
        Some(Point::new(
            (self.x / norm)?,
            (self.y / norm)?,
            (self.z / norm)?,
        ))
    }

    pub fn is_parallel(self, other: Point) -> bool {
        let cross = self.cross(other);
        cross.is_zero()
    }

    pub fn is_perpendicular(self, other: Point) -> bool {
        let dot = self.dot(other);
        dot == 0.0
    }

    pub fn angle(&self, other: Point) -> Option<EFloat64> {
        let dot = self.dot(other);
        let norm = self.norm() * other.norm();
        let dot_norm = (dot / norm)?;
        if dot_norm >= 1.0 {
            return Some(EFloat64::zero());
        }
        if dot_norm <= -1.0 {
            return Some(EFloat64::pi());
        }
        Some(dot_norm.acos())
    }

    // Oriented angle between two vectors around a normal vector. Measured from self to other.
    pub fn angle2(&self, other: Point, normal: Point) -> Option<EFloat64> {
        let cross = self.cross(other);
        let angle = self.angle(other)?;
        if cross.dot(normal) < 0.0 {
            return Some(-angle);
        }
        Some(angle)
    }

    pub fn zero() -> Point {
        Point::new(EFloat64::zero(), EFloat64::zero(), EFloat64::zero())
    }

    pub fn unit_x() -> Point {
        Point::new(EFloat64::one(), EFloat64::zero(), EFloat64::zero())
    }

    pub fn unit_y() -> Point {
        Point::new(EFloat64::zero(), EFloat64::one(), EFloat64::zero())
    }

    pub fn unit_z() -> Point {
        Point::new(EFloat64::zero(), EFloat64::zero(), EFloat64::one())
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Add<EFloat64> for Point {
    type Output = Self;

    fn add(self, other: EFloat64) -> Point {
        Point::new(self.x + other, self.y + other, self.z + other)
    }
}

impl Sub<EFloat64> for Point {
    type Output = Self;

    fn sub(self, other: EFloat64) -> Point {
        Point::new(self.x - other, self.y - other, self.z - other)
    }
}

impl Mul<EFloat64> for Point {
    type Output = Self;

    fn mul(self, other: EFloat64) -> Point {
        Point::new(self.x * other, self.y * other, self.z * other)
    }
}

impl Mul<Point> for EFloat64 {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        Point::new(self * other.x, self * other.y, self * other.z)
    }
}

impl Div<EFloat64> for Point {
    type Output = Option<Point>;

    fn div(self, other: EFloat64) -> Option<Point> {
        Some(Point::new(
            (self.x / other)?,
            (self.y / other)?,
            (self.z / other)?,
        ))
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Point {
        Point::new(-self.x, -self.y, -self.z)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        (self.x - other.x) == 0.0 && (self.y - other.y) == 0.0 && (self.z - other.z) == 0.0
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point({}, {}, {})", self.x, self.y, self.z)
    }
}
