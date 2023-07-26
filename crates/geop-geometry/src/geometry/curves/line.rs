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

    fn project(&self, x: &Point) -> f64 {
        let v = *x - self.basis;
        v.dot(self.direction) / self.direction.norm()
    }
}

impl Curve for Line {
    fn point_at(&self, u: f64) -> Point {
        self.basis + self.direction * u
    }

    fn interval(&self, start: &Point, end: &Point) -> (f64, f64) {
        let start_proj = self.project(start);
        let end_proj = self.project(end);
        (start_proj, end_proj)
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

    fn period(&self) -> f64 {
        std::f64::INFINITY
    }
}
