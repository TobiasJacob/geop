use crate::{points::point::Point, transforms::Transform, EQ_THRESHOLD};

use super::ellipse::Ellipse;

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
        assert!(
            normal.dot(radius).abs() < EQ_THRESHOLD,
            "Radius and normal must be orthogonal"
        );
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

    pub fn tangent(&self, p: Point) -> Point {
        assert!(self.on_manifold(p));
        (p - self.basis).cross(self.dir_cross).normalize()
    }

    pub fn on_manifold(&self, p: Point) -> bool {
        (p - self.basis).dot(self.normal).abs() < EQ_THRESHOLD
            && ((p - self.basis).norm() - self.radius.norm()).abs() < EQ_THRESHOLD
    }

    pub fn interpolate(&self, start: Point, end: Point, t: f64) -> Point {
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

    // Checks if m is between x and y. m==x and m==y are true.
    pub fn between(&self, m: Point, start: Point, end: Point) -> bool {
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

    pub fn get_midpoint(&self, start: Point, end: Point) -> Point {
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
}

// Implement partial eqality for Circle
impl PartialEq for Circle {
    fn eq(&self, other: &Circle) -> bool {
        self.basis == other.basis && self.normal == other.normal && self.radius == other.radius
    }
}
