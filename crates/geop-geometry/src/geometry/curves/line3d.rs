use crate::geometry::points::point3d::Point3d;

use super::curve3d::Curve3d;

pub struct Line3d {
    pub basis: Point3d,
    pub slope: Point3d,
    pub is_normalized: bool
}

impl Line3d {
    pub fn new(basis: Point3d, slope: Point3d) -> Line3d {
        Line3d {
            basis,
            slope,
            is_normalized: false
        }
    }
}

impl Curve3d for Line3d {
    fn point_at(&self, u: f64) -> Point3d {
        self.basis + self.slope * u
    }

    fn project(&self, x: Point3d) -> f64 {
        let v = x - self.basis;
        v.dot(self.slope) / self.slope.norm()
    }

    fn normalize(&mut self) {
        if !self.is_normalized {
            self.slope = self.slope / self.slope.norm();
            self.is_normalized = true;
        }
    }

    fn is_normalized(&self) -> bool {
        self.is_normalized
    }
}
