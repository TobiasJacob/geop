use crate::geometry::points::point::Point;

use super::curve::Curve;

pub struct Circle {
    pub basis: Point,
    pub normal: Point,
    pub radius: f64,
    pub is_normalized: bool
}

impl Circle {
    pub fn new(basis: Point, normal: Point, radius: f64) -> Circle {
        Circle {
            basis,
            normal,
            radius,
            is_normalized: false
        }
    }

    fn project(&self, x: &Point) -> f64 {
        let v = *x - self.basis;
        let v = v - self.normal * v.dot(self.normal);
        let angle = v.y.atan2(v.x);
        angle
    }
}

impl Curve for Circle {
    fn point_at(&self, u: f64) -> Point {
        let x = self.basis.x + self.radius * u.cos();
        let y = self.basis.y + self.radius * u.sin();
        let z = self.basis.z;
        Point::new(x, y, z)
    }

    fn project(&self, p: &Point) -> f64 {
        let v = *p - self.basis;
        let v = v - self.normal * v.dot(self.normal);
        let angle = v.y.atan2(v.x);
        angle
    }

    fn derivative(&self, u: f64) -> Point {
        let x = -self.radius * u.sin();
        let y = self.radius * u.cos();
        let z = 0.0;
        Point::new(x, y, z)
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


}
