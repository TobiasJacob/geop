use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::EQ_THRESHOLD;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }

    pub fn norm(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn norm_sq(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn is_normalized(self) -> bool {
        return (self.norm_sq() - 1.0).abs() < EQ_THRESHOLD;
    }

    pub fn is_zero(self) -> bool {
        self.x.abs() < EQ_THRESHOLD && self.y.abs() < EQ_THRESHOLD && self.z.abs() < EQ_THRESHOLD
    }

    pub fn dot(self, other: Point) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Point) -> Point {
        Point::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    // TODO: This operation should return a Result, as the operation is not always possible. It fails when norm is zero. This is not as rare as it seems. It helps users to catch bugs.
    pub fn normalize(self) -> Point {
        let norm = self.norm();
        assert!(norm > EQ_THRESHOLD);
        Point::new(self.x / norm, self.y / norm, self.z / norm)
    }

    pub fn is_parallel(self, other: Point) -> bool {
        let cross = self.cross(other);
        cross.is_zero()
    }

    pub fn is_perpendicular(self, other: Point) -> bool {
        let dot = self.dot(other);
        dot.abs() < EQ_THRESHOLD
    }

    pub fn angle(&self, other: Point) -> f64 {
        let dot = self.dot(other);
        let norm = self.norm() * other.norm();
        assert!(norm > EQ_THRESHOLD);
        let dot_norm = dot / norm;
        if dot_norm > 1.0 - EQ_THRESHOLD {
            return 0.0;
        }
        if dot_norm < -1.0 + EQ_THRESHOLD {
            return std::f64::consts::PI;
        }
        dot_norm.acos()
    }

    // Oriented angle between two vectors around a normal vector. Measured from self to other.
    pub fn angle2(&self, other: Point, normal: Point) -> f64 {
        let cross = self.cross(other);
        let angle = self.angle(other);
        if cross.dot(normal) < 0.0 {
            return -angle;
        }
        angle
    }

    pub fn zero() -> Point {
        Point::new(0.0, 0.0, 0.0)
    }

    pub fn unit_x() -> Point {
        Point::new(1.0, 0.0, 0.0)
    }

    pub fn unit_y() -> Point {
        Point::new(0.0, 1.0, 0.0)
    }

    pub fn unit_z() -> Point {
        Point::new(0.0, 0.0, 1.0)
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

impl Add<f64> for Point {
    type Output = Self;

    fn add(self, other: f64) -> Point {
        Point::new(self.x + other, self.y + other, self.z + other)
    }
}

impl Sub<f64> for Point {
    type Output = Self;

    fn sub(self, other: f64) -> Point {
        Point::new(self.x - other, self.y - other, self.z - other)
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, other: f64) -> Point {
        Point::new(self.x * other, self.y * other, self.z * other)
    }
}

impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        Point::new(self * other.x, self * other.y, self * other.z)
    }
}

impl Div<f64> for Point {
    type Output = Self;

    fn div(self, other: f64) -> Point {
        Point::new(self.x / other, self.y / other, self.z / other)
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
        (self.x - other.x).abs() < EQ_THRESHOLD
            && (self.y - other.y).abs() < EQ_THRESHOLD
            && (self.z - other.z).abs() < EQ_THRESHOLD
    }
}
