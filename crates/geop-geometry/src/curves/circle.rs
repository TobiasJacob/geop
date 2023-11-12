use std::rc::Rc;

use crate::{points::point::Point, transforms::Transform, EQ_THRESHOLD};

use super::{curve::Curve, ellipse::Ellipse};

#[derive(Debug, Clone)]
pub struct Circle {
    pub basis: Point,
    pub normal: Point,
    pub radius: Point,
    dir_cross: Point,
}

pub enum CircleTransform {
    Circle(Circle),
    Ellipse(Ellipse),
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

    pub fn transform(&self, transform: Transform) -> CircleTransform {
        let basis = transform * self.basis;
        let normal = transform * (self.normal + self.basis) - basis;
        let radius = transform * (self.radius + self.basis) - basis;
        let scale_factor = radius.norm() / self.radius.norm();
        assert!((normal.norm() - scale_factor * self.normal.norm()) < EQ_THRESHOLD, "Circle can only be transformed with uniform scaling. An extension of this method is planned to return ellipses.");
        CircleTransform::Circle(Circle::new(basis, normal.normalize(), radius))
    }

    pub fn neg(&self) -> Circle {
        Circle::new(self.basis, -self.normal, self.radius)
    }
}

impl Curve for Circle {
    fn transform(&self, transform: Transform) -> Rc<dyn Curve> {
        match self.transform(transform) {
            CircleTransform::Circle(c) => Rc::new(c),
            CircleTransform::Ellipse(e) => Rc::new(e),
        }
    }

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

    fn tangent(&self, _p: Point) -> Point {
        todo!("Implement derivative for Circle")
    }

    fn distance(&self, p1: Point, p2: Point) -> f64 {
        let angle = self.project(p1).0 - self.project(p2).0;
        return angle.abs() * self.radius.norm();
    }

    fn neg(&self) -> Rc<dyn Curve> {
        Rc::new(self.neg())
    }
}

// Implement partial eqality for Circle
impl PartialEq for Circle {
    fn eq(&self, other: &Circle) -> bool {
        self.basis == other.basis && self.normal == other.normal && self.radius == other.radius
    }
}
