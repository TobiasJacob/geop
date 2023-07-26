use crate::geometry::points::point3d::Point3d;

use super::curve3d::Curve3d;

pub struct Line3d {
    pub basis: Point3d,
    pub direction: Point3d,
    pub is_normalized: bool
}

impl Line3d {
    pub fn new(basis: Point3d, slope: Point3d) -> Line3d {
        Line3d {
            basis,
            direction: slope,
            is_normalized: false
        }
    }

    fn project(&self, x: Point3d) -> f64 {
        let v = x - self.basis;
        v.dot(self.direction) / self.direction.norm()
    }
}

impl Curve3d for Line3d {
    fn point_at(&self, u: f64) -> Point3d {
        self.basis + self.direction * u
    }

    fn interval(&self, start: Point3d, end: Point3d) -> (f64, f64) {
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
