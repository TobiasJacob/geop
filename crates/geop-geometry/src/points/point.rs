use std::ops::{Add, Sub, Mul, Div, Neg};

use crate::EQ_THRESHOLD;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }

    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn is_normalized(&self) -> bool {
        (self.x * self.x + self.y * self.y + self.z * self.z) < EQ_THRESHOLD
    }

    pub fn dot(&self, other: Point) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, v_slope: Point) -> Point {
        Point::new(
            self.y * v_slope.z - self.z * v_slope.y,
            self.z * v_slope.x - self.x * v_slope.z,
            self.x * v_slope.y - self.y * v_slope.x
        )
    }

    pub fn normalize(&self) -> Point {
        let norm = self.norm();
        Point::new(self.x / norm, self.y / norm, self.z / norm)
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
        (self.x - other.x).abs() < EQ_THRESHOLD && (self.y - other.y).abs() < EQ_THRESHOLD && (self.z - other.z).abs() < EQ_THRESHOLD
    }
}