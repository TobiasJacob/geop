use crate::{geometry::points::point3d::Point3d, EQ_THRESHOLD};

use super::curve3d::Curve3d;

pub struct Ellipse3d {
    pub basis: Point3d,
    pub dir_u: Point3d,
    pub dir_v: Point3d
}

impl Ellipse3d {
    pub fn new(basis: Point3d, dir_u: Point3d, dir_v: Point3d) -> Ellipse3d {
        Ellipse3d {
            basis,
            dir_u,
            dir_v
        }
    }

    fn project(&self, x: &Point3d) -> f64 {
        let v = *x - self.basis;
        let u = v.dot(self.dir_u) / self.dir_u.norm();
        let v = v - self.dir_u * u;
        let v = v.dot(self.dir_v) / self.dir_v.norm();
        let v = v.atan2(u);
        v / (2.0 * std::f64::consts::PI)
    }
}

impl Curve3d for Ellipse3d {
    fn point_at(&self, u: f64) -> Point3d {
        self.basis + self.dir_u * u.cos() + self.dir_v * u.sin()
    }

    fn interval(&self, start: &Point3d, end: &Point3d) -> (f64, f64) {
        let start_angle = self.project(start);
        let end_angle = self.project(end);
        if start_angle + EQ_THRESHOLD <= end_angle {
            (start_angle, end_angle)
        } else {
            (start_angle, end_angle + 2.0 * std::f64::consts::PI)
        }
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