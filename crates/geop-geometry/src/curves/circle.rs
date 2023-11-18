use std::rc::Rc;

use crate::{points::point::Point, transforms::Transform, EQ_THRESHOLD};

use super::{curve::{Curve, TangentParameter}, ellipse::Ellipse};

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
        assert!(normal.dot(radius).abs() < EQ_THRESHOLD, "Radius and normal must be orthogonal");
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

    fn neg(&self) -> Rc<dyn Curve> {
        Rc::new(self.neg())
    }

    fn tangent(&self, p: Point) -> Point {
        assert!(self.on_manifold(p));
        (p - self.basis).cross(self.dir_cross).normalize()
    }

    fn on_manifold(&self, p: Point) -> bool {
        (p - self.basis).dot(self.normal).abs() < EQ_THRESHOLD && ((p - self.basis).norm() - self.radius.norm()).abs() < EQ_THRESHOLD
    }

    fn metric(&self, x: Point, u: TangentParameter, v: TangentParameter) -> f64 {
        u.0 * v.0
    }

    fn distance(&self, x: Point, y: Point) -> f64 {
        assert!(self.on_manifold(x));
        assert!(self.on_manifold(y));
        let v1 = x - self.basis;
        let v2 = y - self.basis;
        let angle = v1.angle(v2);
        angle * self.radius.norm()
    }

    fn exp(&self, x: Point, u: TangentParameter) -> Point {
        assert!(self.on_manifold(x));
        let x = x - self.basis;
        x * u.0.cos() + self.normal.cross(x) * u.0.sin()
    }
    
    fn log(&self, x: Point, y: Point) -> TangentParameter {
        assert!(self.on_manifold(x));
        assert!(self.on_manifold(y));
        let x = x - self.basis;
        let y = y - self.basis;
        let angle = x.angle(y);
        TangentParameter(angle)
    }

    fn parallel_transport(&self, v: TangentParameter, x: Point, y: Point) -> TangentParameter {
        v
    }

    fn between(&self, m: Point, start: Point, end: Point) -> bool {
        assert!(self.on_manifold(m));
        assert!(self.on_manifold(start));
        assert!(self.on_manifold(end));
        let mut angle_m = (m - self.basis).angle(self.radius);
        let angle_start = (start - self.basis).angle(self.radius);
        let mut angle_end = (end - self.basis).angle(self.radius);
        if angle_start < angle_end {
            angle_end += 2.0 * std::f64::consts::PI;
        }
        if angle_m < angle_start {
            angle_m += 2.0 * std::f64::consts::PI;
        }
        angle_start <= angle_m && angle_m <= angle_end
    }

    fn get_midpoint(&self, start: Point, end: Point) -> Point {
        assert!(self.on_manifold(start));
        assert!(self.on_manifold(end));
        let start = start - self.basis;
        let end = end - self.basis;
        let mid = (start + end) / 2.0;
        let mid = mid.normalize() * self.radius.norm();
        let p1 = mid + self.basis;
        if self.between(p1, start, end) {
            return p1;
        } else {
            return -mid + self.basis;
        }
    }

    fn point_at(&self, t: f64, start: Point, end: Point) -> Point {
        assert!(self.on_manifold(start));
        assert!(self.on_manifold(end));
        let start = start - self.basis;
        let end = end - self.basis;
        let x_start = self.radius.dot(start);
        let x_end = self.radius.dot(end);
        let y_start = self.dir_cross.dot(start);
        let y_end = self.dir_cross.dot(end);
        let angle1 = x_start.atan2(y_start);
        let angle2 = x_end.atan2(y_end);
        let angle = angle1 + t * (angle2 - angle1);
        angle.cos() * self.radius + angle.sin() * self.dir_cross + self.basis
    }
}

// Implement partial eqality for Circle
impl PartialEq for Circle {
    fn eq(&self, other: &Circle) -> bool {
        self.basis == other.basis && self.normal == other.normal && self.radius == other.radius
    }
}
