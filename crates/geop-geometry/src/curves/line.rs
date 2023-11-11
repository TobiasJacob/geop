use std::rc::Rc;

use crate::{points::point::Point, transforms::Transform};

use super::curve::Curve;

#[derive(Debug, Clone)]
pub struct Line {
    pub basis: Point,
    pub direction: Point,
}

impl Line {
    pub fn new(basis: Point, direction: Point) -> Line {
        Line {
            basis,
            direction: direction.normalize(),
        }
    }

    pub fn transform(&self, transform: Transform) -> Self {
        let basis = transform * self.basis;
        let direction = transform * (self.direction + self.basis) - basis;
        Line::new(basis, direction.normalize())
    }
}

impl Curve for Line {
    fn transform(&self, transform: Transform) -> Rc<dyn Curve> {
        Rc::new(self.transform(transform))
    }

    fn project(&self, p: Point) -> (f64, f64) {
        let v = p - self.basis;
        let u = self.direction.dot(v);
        let perp = v - self.direction * u;
        let v = perp.norm();
        (u, v)
    }

    fn point_at(&self, u: f64) -> Point {
        self.basis + self.direction * u
    }

    fn derivative(&self, _p: Point) -> Point {
        self.direction.clone()
    }

    fn distance(&self, p1: Point, p2: Point) -> f64 {
        return (p2 - p1).norm();
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Line) -> bool {
        self.direction.is_parallel(other.direction) &&
            (other.basis - self.basis).is_parallel(self.direction)
    }
}
