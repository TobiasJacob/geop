use crate::geometry::points::point3d::Point3d;

use super::curve3d::Curve3d;

pub struct Circle3d {
    pub basis: Point3d,
    pub normal: Point3d,
    pub radius: f64,
    pub is_normalized: bool
}

impl Circle3d {
    pub fn new(basis: Point3d, normal: Point3d, radius: f64) -> Circle3d {
        Circle3d {
            basis,
            normal,
            radius,
            is_normalized: false
        }
    }
}

impl Curve3d for Circle3d {
    fn point_at(&self, u: f64) -> Point3d {
        let angle = u * 2.0 * std::f64::consts::PI;
        let x = self.basis.x + self.radius * angle.cos();
        let y = self.basis.y + self.radius * angle.sin();
        let z = self.basis.z;
        Point3d::new(x, y, z)
    }

    fn project(&self, x: Point3d) -> f64 {
        let v = x - self.basis;
        let v = v - self.normal * v.dot(self.normal);
        let angle = v.y.atan2(v.x);
        angle / (2.0 * std::f64::consts::PI)
    }

    fn normalize(&mut self) {
        if !self.is_normalized {
            self.normal = self.normal / self.normal.norm();
            self.is_normalized = true;
        }
    }

    fn is_normalized(&self) -> bool {
        self.is_normalized
    }

    fn period(&self) -> f64 {
        1.0
    }
}
