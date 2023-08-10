use crate::points::point::Point;

use super::curve::Curve;

pub struct Line {
    pub basis: Point,
    pub direction: Point
}

impl Line {
    pub fn new(basis: Point, slope: Point) -> Line {
        Line {
            basis,
            direction: slope.normalize(),
        }
    }
}

impl Curve for Line {
    fn project(&self, p: &Point) -> (f64, f64) {
        let v = *p - self.basis;
        let u = v.dot(self.direction) / self.direction.norm();
        let v = v.dot(self.direction) / self.direction.norm();
        (u, v)
    }

    fn point_at(&self, u: f64) -> Point {
        self.basis + self.direction * u
    }

    fn derivative(&self, _: f64) -> Point {
        self.direction
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Line) -> bool {
        self.basis == other.basis && self.direction.normalize() == other.direction.normalize()
    }
}
