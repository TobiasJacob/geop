use crate::points::point::Point;

use super::curve::Curve;

#[derive(Debug, Clone)]
pub struct Ellipse {
    pub basis: Point,
    pub dir0: Point,
    pub dir1: Point
}

impl Ellipse {
    pub fn new(basis: Point, dir0: Point, dir1: Point) -> Ellipse {
        Ellipse {
            basis,
            dir0,
            dir1
        }
    }
}

impl Curve for Ellipse {
    fn point_at(&self, u: f64) -> Point {
        self.basis + self.dir0 * u.cos() + self.dir1 * u.sin()
    }

    fn project(&self, p: Point) -> (f64, f64) {
        let v = p - self.basis;
        let u = v.dot(self.dir0) / self.dir0.norm();
        let v = v - self.dir0 * u;
        let v = v.dot(self.dir1) / self.dir1.norm();
        let v = v.atan2(u);
        let u = u.atan2(v);
        (u / (2.0 * std::f64::consts::PI), v / (2.0 * std::f64::consts::PI))
    }

    fn derivative(&self, p: Point) -> Point {
        let u = self.project(p).0;
        -self.dir0 * u.sin() + self.dir1 * u.cos()
    }
}

impl PartialEq for Ellipse {
    fn eq(&self, other: &Ellipse) -> bool {
        self.basis == other.basis && self.dir0 == other.dir0 && self.dir1 == other.dir1
    }
}