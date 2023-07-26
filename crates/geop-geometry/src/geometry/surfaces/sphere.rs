use crate::geometry::points::{point3d::Point3d, point2d::Point2d};

use super::surface::Surface;

pub struct Sphere {
    pub basis: Point3d,
    pub radius: f64,
}

impl Sphere {
    pub fn new(basis: Point3d, radius: f64) -> Sphere {
        Sphere {
            basis,
            radius,
        }
    }

    fn project(&self, x: Point3d) -> Point2d {
        let v = x - self.basis;
        let u = v.dot(Point3d::new(1.0, 0.0, 0.0));
        let v = v.dot(Point3d::new(0.0, 1.0, 0.0));
        Point2d::new(u, v)
    }
}

impl Surface for Sphere {
    fn point_at(&self, u: Point2d) -> Point3d {
        let x = self.basis.x + self.radius * u.x.cos() * u.y.sin();
        let y = self.basis.y + self.radius * u.x.sin() * u.y.sin();
        let z = self.basis.z + self.radius * u.y.cos();
        Point3d::new(x, y, z)
    }

    fn normalize(&mut self) {
        // Use this to make redundant representations of surfaces unique
        self.radius = self.radius.abs();
    }

    fn is_normalized(&self) -> bool {
        self.radius >= 0.0
    }

    fn period(&self) -> Point2d {
        Point2d::new(2.0 * std::f64::consts::PI, std::f64::consts::PI)
    }
}
