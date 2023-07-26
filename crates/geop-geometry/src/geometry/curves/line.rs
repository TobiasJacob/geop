use crate::geometry::points::point::Point;

use super::curve::Curve;

pub struct Line {
    pub basis: Point,
    pub direction: Point,
    pub is_normalized: bool
}

impl Line {
    pub fn new(basis: Point, slope: Point) -> Line {
        Line {
            basis,
            direction: slope,
            is_normalized: false
        }
    }
}

impl Curve for Line {
    fn project(&self, p: &Point) -> f64 {
        let v = *p - self.basis;
        v.dot(self.direction) / self.direction.norm()
    }

    fn point_at(&self, u: f64) -> Point {
        self.basis + self.direction * u
    }

    fn derivative(&self, u: f64) -> Point {
        self.direction
    }

    fn normalize(&mut self) {
        if !self.is_normalized {
            self.direction = self.direction / self.direction.norm();
            self.is_normalized = true;
        }
    }

    fn is_normalized(&self) -> bool {
        self.is_normalized
    }
}
