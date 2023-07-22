use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Copy, Clone)]
pub struct Point2d {
    x: f64,
    y: f64,
}

impl Point2d {
    pub fn new(x: f64, y: f64) -> Point2d {
        Point2d { x, y }
    }

    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn dot(&self, other: Point2d) -> f64 {
        self.x * other.x + self.y * other.y
    }
}


impl Add for Point2d {
    type Output = Self;

    fn add(self, other: Point2d) -> Point2d {
        Point2d::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Point2d {
    type Output = Self;

    fn sub(self, other: Point2d) -> Point2d {
        Point2d::new(self.x - other.x, self.y - other.y)
    }
}

impl Add<f64> for Point2d {
    type Output = Self;

    fn add(self, other: f64) -> Point2d {
        Point2d::new(self.x + other, self.y + other)
    }
}

impl Sub<f64> for Point2d {
    type Output = Self;

    fn sub(self, other: f64) -> Point2d {
        Point2d::new(self.x - other, self.y - other)
    }
}

impl Mul<f64> for Point2d {
    type Output = Self;

    fn mul(self, other: f64) -> Point2d {
        Point2d::new(self.x * other, self.y * other)
    }
}

impl Div<f64> for Point2d {
    type Output = Self;

    fn div(self, other: f64) -> Point2d {
        Point2d::new(self.x / other, self.y / other)
    }
}
