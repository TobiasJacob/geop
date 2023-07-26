use crate::geometry::points::point2d::Point2d;
use crate::geometry::points::point3d::Point3d;

use super::surface::Surface;

pub struct Plane {
    pub basis: Point3d,
    pub u_slope: Point3d,
    pub v_slope: Point3d
}

impl Plane {
    pub fn new(basis: Point3d, u_slope: Point3d, v_slope: Point3d) -> Plane {
        Plane {
            basis,
            u_slope,
            v_slope
        }
    }

    fn project(&self, x: Point3d) -> Point2d {
        let v = x - self.basis;
        let u = v.dot(self.u_slope) / self.u_slope.norm();
        let v = v - self.u_slope * u;
        let v = v.dot(self.v_slope) / self.v_slope.norm();
        Point2d::new(u, v)
    }
}

impl Surface for Plane {
    fn point_at(&self, u: Point2d) -> Point3d {
        self.basis + self.u_slope * u.x + self.v_slope * u.y
    }

    fn normalize(&mut self) {
        self.u_slope = self.u_slope / self.u_slope.norm();
        self.v_slope = self.v_slope / self.v_slope.norm();
    }

    fn is_normalized(&self) -> bool {
        self.u_slope.norm() == 1.0 && self.v_slope.norm() == 1.0
    }

    fn period(&self) -> Point2d {
        Point2d::new(f64::INFINITY, f64::INFINITY)
    }
}