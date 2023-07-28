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
        let x = self.basis + self.direction * v + self.extend * u.cos();
        let y = self.basis + self.direction * v + self.direction.cross(self.extend) * u.sin();
        let z = self.basis + self.direction * v;
        Point::new(x, y, z)
    }

    fn project(&self, p: &Point) -> (f64, f64) {
        let v = *p - self.basis;
        let v = v - self.direction * v.dot(self.direction);
        let u = v.dot(self.extend) / self.extend.norm();
        let v = v.dot(self.direction) / self.direction.norm();
        let u = u.atan2(v);
        let v = v.atan2(u);
        (u / (2.0 * std::f64::consts::PI), v / (2.0 * std::f64::consts::PI))
    }

    fn derivative(&self, u: f64, v: f64) -> Point {
        let x = -self.extend * u.sin();
        let y = self.extend * u.cos();
        let z = self.direction;
        Point::new(x, y, z)
    }

    fn normalize(&mut self) {
        self.direction = self.direction.normalize();
    }
}

impl PartialEq for Cylinder {
    fn eq(&self, other: &Cylinder) -> bool {
        self.basis == other.basis && self.extend == other.extend && self.direction.normalize() == other.direction.normalize()
    }
}