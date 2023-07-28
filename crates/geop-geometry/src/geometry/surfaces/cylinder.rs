use crate::geometry::points::point::Point;

use super::surface::Surface;

pub struct Cylinder {
    pub basis: Point,
    pub extend: Point,
    pub direction: Point,
}

impl Cylinder {
    pub fn new(basis: Point, extend: Point, direction: Point) -> Cylinder {
        Cylinder {
            basis,
            extend,
            direction,
        }
    }
}

impl Surface for Cylinder {
    fn point_at(&self, u: f64, v: f64) -> Point {
        let x = self.basis + self.direction * v + self.extend * u.cos() + self.direction.cross(self.extend) * u.sin();
        x
    }

    fn project(&self, p: &Point) -> (f64, f64) {
        let v = *p - self.basis;
        let v = v - self.direction * v.dot(self.direction);
        let u = v.dot(self.extend) / self.extend.norm();
        let v = v.dot(self.direction) / self.direction.norm();
        let v = v.atan2(u);
        let u = u.atan2(v);
        (u / (2.0 * std::f64::consts::PI), v / (2.0 * std::f64::consts::PI))
    }

    fn derivative_u(&self, u: f64, v: f64) -> Point {
        let x = -self.extend * u.sin() + self.direction.cross(self.extend) * u.cos();
        x
    }

    fn derivative_v(&self, u: f64, v: f64) -> Point {
        let x = self.direction;
        x
    }

    fn normal(&self, p: Point) -> Point {
        let v = p - self.basis;
        let v = v - self.direction * v.dot(self.direction);
        let v = v.normalize();
        v
    }

    fn normalize(&mut self) {
        self.direction = self.direction.normalize();
    }

    fn is_normalized(&self) -> bool {
        self.direction.is_normalized()
    }
}

impl PartialEq for Cylinder {
    fn eq(&self, other: &Cylinder) -> bool {
        self.basis == other.basis && self.extend == other.extend && self.direction.normalize() == other.direction.normalize()
    }
}