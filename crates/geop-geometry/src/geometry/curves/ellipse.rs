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
    fn point_at(&self, start: &Point, u: f64) -> Point {
        let angle = u + self.project(start) * 2.0 * std::f64::consts::PI;
        let x = self.basis + self.dir_u * angle.cos() + self.dir_v * angle.sin();
        x
    }

    fn interval(&self, start: &Point, end: &Point) -> (f64, f64) {
        let start_angle = self.project(start);
        let end_angle = self.project(end);
        if start_angle + EQ_THRESHOLD <= end_angle {
            (start_angle, end_angle)
        } else {
            (start_angle, end_angle + 2.0 * std::f64::consts::PI)
        }
    }

    fn length(&self, start: &Point, end: &Point) -> f64 {
        let (start_angle, end_angle) = self.interval(start, end);
        (end_angle - start_angle) * self.dir_u.norm()
    }

    fn normalize(&mut self) {
        // Ellipse is always normalized, as each representation is unique
    }

    fn is_normalized(&self) -> bool {
        true
    }

    fn period(&self) -> f64 {
        1.0
    }
}