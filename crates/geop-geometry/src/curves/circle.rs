use crate::points::point::Point;

use super::curve::Curve;

#[derive(Debug, Clone)]
pub struct Circle {
    pub basis: Point,
    pub normal: Point,
    pub radius: Point,
    dir_cross: Point,
}

impl Circle {
    pub fn new(basis: Point, normal: Point, radius: Point) -> Circle {
        let normal = normal.normalize();
        Circle {
            basis,
            normal,
            radius,
            dir_cross: normal.cross(radius),
        }
    }
}

impl Curve for Circle {
    fn point_at(&self, u: f64) -> Point {
        self.radius * u.cos() + self.dir_cross * u.sin() + self.basis
    }

    fn project(&self, p: Point) -> (f64, f64) {
        let dir = p - self.basis;
        let dir = dir - self.normal * dir.dot(self.normal);
        let u = dir.dot(self.radius);
        let v = dir.dot(self.dir_cross);
        let angle = v.atan2(u);
        let dist = (dir.norm() - self.radius.norm()).abs();
        return (angle, dist);
    }

    fn derivative(&self, p: Point) -> Point {
        todo!("Implement derivative for Circle")
    }

    fn distance(&self, p1: Point, p2: Point) -> f64 {
        let angle = self.project(p1).0 - self.project(p2).0;
        return angle.abs() * self.radius.norm();
    }
}

// Implement partial eqality for Circle
impl PartialEq for Circle {
    fn eq(&self, other: &Circle) -> bool {
        self.basis == other.basis && self.normal == other.normal && self.radius == other.radius
    }
}
