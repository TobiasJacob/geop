use crate::geometry::points::point::Point;

use super::surface::Surface;

pub struct Cylinder {
    pub basis: Point,
    pub extend: Point,
    pub radius: f64,
}

impl Cylinder {
    pub fn new(basis: Point, extend: Point, radius: f64) -> Self {
        Self {
            basis,
            extend,
            radius,
        }
    }
}

impl Surface for Sphere {
    fn point_at(&self, u: f64, v: f64) -> Point {
        let x = self.basis.x + self.radius * u.cos() * v.sin();
        let y = self.basis.y + self.radius * u.sin() * v.sin();
        let z = self.basis.z + self.radius * v.cos();
        Point::new(x, y, z)
    }

    fn project(&self, x: Point) -> (f64, f64) {
        let v = x - self.basis;
        let u = v.dot(Point::new(1.0, 0.0, 0.0));
        let v = v.dot(Point::new(0.0, 1.0, 0.0));
        (u, v)
    }

    fn normalize(&mut self) {
        // Use this to make redundant representations of surfaces unique
        self.radius = self.radius.abs();
    }

    fn is_normalized(&self) -> bool {
        self.radius >= 0.0
    }
}