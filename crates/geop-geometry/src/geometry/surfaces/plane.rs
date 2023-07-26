use crate::geometry::points::point::Point;

use super::surface::Surface;

pub struct Plane {
    pub basis: Point,
    pub u_slope: Point,
    pub v_slope: Point
}

impl Plane {
    pub fn new(basis: Point, u_slope: Point, v_slope: Point) -> Plane {
        Plane {
            basis,
            u_slope,
            v_slope
        }
    }

}

impl Surface for Plane {
    fn point_at(&self, u: f64, v: f64) -> Point {
        self.basis + self.u_slope * u + self.v_slope * v
    }

    fn project(&self, p: &Point) -> (f64, f64) {
        let v = *p - self.basis;
        let u = v.dot(self.u_slope) / self.u_slope.norm();
        let v = v - self.u_slope * u;
        let v = v.dot(self.v_slope) / self.v_slope.norm();
        (u, v)
    }

    fn derivative_u(&self, u: f64, v: f64) -> Point {
        self.u_slope
    }

    fn derivative_v(&self, u: f64, v: f64) -> Point {
        self.v_slope
    }

    fn normal(&self, p: Point) -> Point {
        self.u_slope.cross(self.v_slope)
    }

    fn normalize(&mut self) {
        self.u_slope = self.u_slope / self.u_slope.norm();
        self.v_slope = self.v_slope / self.v_slope.norm();
    }

    fn is_normalized(&self) -> bool {
        self.u_slope.norm() == 1.0 && self.v_slope.norm() == 1.0
    }
}