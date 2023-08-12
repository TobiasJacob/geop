use crate::points::point::Point;

use super::curve::Curve;

#[derive(Debug)]
pub struct Line {
    pub basis: Point,
    pub direction: Point
}

impl Line {
    pub fn new(basis: Point, direction: Point) -> Line {
        Line {
            basis,
            direction: direction.normalize(),
        }
    }
}

impl Curve for Line {
    fn project(&self, p: Point) -> (f64, f64) {
        let v = p - self.basis;
        let u = v.dot(self.direction);
        let v = v.dot(self.direction);
        (u, v)
    }

    fn point_at(&self, u: f64) -> Point {
        self.basis + self.direction * u
    }

    fn derivative(&self, _: f64) -> Point {
        self.direction.clone()
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Line) -> bool {
        self.basis == other.basis && self.direction == other.direction
    }
}
