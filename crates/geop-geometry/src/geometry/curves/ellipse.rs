use crate::{geometry::points::point::Point, EQ_THRESHOLD};

use super::curve::Curve;

pub struct Ellipse {
    pub basis: Point,
    pub dir_u: Point,
    pub dir_v: Point
}

impl Ellipse {
    pub fn new(basis: Point, dir_u: Point, dir_v: Point) -> Ellipse {
        Ellipse {
            basis,
            dir_u,
            dir_v
        }
    }

    fn project(&self, x: &Point) -> f64 {
        let v = *x - self.basis;
        let u = v.dot(self.dir_u) / self.dir_u.norm();
        let v = v - self.dir_u * u;
        let v = v.dot(self.dir_v) / self.dir_v.norm();
        let v = v.atan2(u);
        v / (2.0 * std::f64::consts::PI)
    }
}

impl Curve for Ellipse {
    fn point_at(&self, u: f64) -> Point {
        self.basis + self.dir_u * u.cos() + self.dir_v * u.sin()
    }

    fn project(&self, p: &Point) -> f64 {
        let v = *p - self.basis;
        let u = v.dot(self.dir_u) / self.dir_u.norm();
        let v = v - self.dir_u * u;
        let v = v.dot(self.dir_v) / self.dir_v.norm();
        let v = v.atan2(u);
        v / (2.0 * std::f64::consts::PI)
    }

    fn derivative(&self, u: f64) -> Point {
        -self.dir_u * u.sin() + self.dir_v * u.cos()
    }

    fn normalize(&mut self) {
        // Ellipse is always normalized, as each representation is unique
    }

    fn is_normalized(&self) -> bool {
        true
    }
}

impl PartialEq for Ellipse {
    fn eq(&self, other: &Ellipse) -> bool {
        self.basis == other.basis && self.dir_u == other.dir_u && self.dir_v == other.dir_v
    }
}