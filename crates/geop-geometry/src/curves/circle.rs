use crate::{points::point::Point, transforms::Transform, EQ_THRESHOLD};

use super::{curve::Curve, CurveLike};

#[derive(Debug, Clone)]
pub struct Circle {
    pub basis: Point,
    pub normal: Point,
    pub radius: Point,
    dir_cross: Point,
}

pub enum CircleTransform {
    Circle(Circle),
    Ellipse(), // TODO: Implement this
}

impl Circle {
    pub fn new(basis: Point, normal: Point, radius: f64) -> Circle {
        let radius = match Point::new_unit_x().cross(normal).norm_sq()
            > Point::new_unit_y().cross(normal).norm_sq()
        {
            true => Point::new_unit_x().cross(normal).normalize() * radius,
            false => Point::new_unit_y().cross(normal).normalize() * radius,
        };
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
        CircleTransform::Circle(Circle::new(basis, normal.normalize(), radius.norm()))
    }

    pub fn neg(&self) -> Circle {
        Circle::new(self.basis, -self.normal, self.radius.norm())
    }
}

impl CurveLike for Circle {
    fn transform(&self, transform: Transform) -> Curve {
        match self.transform(transform) {
            CircleTransform::Circle(circle) => Curve::Circle(circle),
            CircleTransform::Ellipse() => todo!("Implement this"),
        }
    }

    fn neg(&self) -> Curve {
        Curve::Circle(self.neg())
    }

    fn tangent(&self, p: Point) -> Point {
        assert!(self.on_curve(p));
        self.normal.cross(p - self.basis).normalize()
    }

    fn on_curve(&self, p: Point) -> bool {
        (p - self.basis).dot(self.normal).abs() < EQ_THRESHOLD
            && ((p - self.basis).norm() - self.radius.norm()).abs() < EQ_THRESHOLD
    }

    fn distance(&self, x: Point, y: Point) -> f64 {
        assert!(self.on_curve(x));
        assert!(self.on_curve(y));
        let angle = (x - self.basis).angle(y - self.basis);
        self.radius.norm() * angle
    }

    fn interpolate(&self, start: Option<Point>, end: Option<Point>, t: f64) -> Point {
        match (start, end) {
            (Some(start), Some(end)) => {
                assert!(self.on_curve(start));
                assert!(self.on_curve(end));
                let start = start - self.basis;
                let end = end - self.basis;
                let x_start = self.radius.dot(start);
                let x_end = self.radius.dot(end);
                let y_start = self.dir_cross.dot(start);
                let y_end = self.dir_cross.dot(end);
                let angle1 = y_start.atan2(x_start);
                let mut angle2 = y_end.atan2(x_end);
                if angle2 < angle1 {
                    angle2 += 2.0 * std::f64::consts::PI;
                }
                let angle = angle1 + t * (angle2 - angle1);
                angle.cos() * self.radius + angle.sin() * self.dir_cross + self.basis
            }
            (Some(start), None) => {
                let start = start - self.basis;
                let x_start = self.radius.dot(start);
                let y_start = self.dir_cross.dot(start);
                let angle1 = y_start.atan2(x_start);
                let angle = angle1 + t * std::f64::consts::PI * 2.0;
                angle.cos() * self.radius + angle.sin() * self.dir_cross + self.basis
            }
            (None, Some(end)) => {
                let end = end - self.basis;
                let x_end = self.radius.dot(end);
                let y_end = self.dir_cross.dot(end);
                let angle2 = y_end.atan2(x_end);
                let angle = angle2 + t * std::f64::consts::PI * 2.0;
                angle.cos() * self.radius + angle.sin() * self.dir_cross + self.basis
            }
            (None, None) => {
                let angle = t * std::f64::consts::PI * 2.0;
                angle.cos() * self.radius + angle.sin() * self.dir_cross + self.basis
            }
        }
    }

    // Checks if m is between x and y. m==x and m==y are true.
    fn between(&self, m: Point, start: Option<Point>, end: Option<Point>) -> bool {
        assert!(self.on_curve(m));
        match (start, end) {
            (Some(start), Some(end)) => {
                assert!(self.on_curve(start));
                assert!(self.on_curve(end));
                let start = start - self.basis;
                let end = end - self.basis;
                let m = m - self.basis;
                let angle_start = self.dir_cross.dot(start).atan2(self.radius.dot(start));
                let mut angle_end = self.dir_cross.dot(end).atan2(self.radius.dot(end));
                let mut angle_m = self.dir_cross.dot(m).atan2(self.radius.dot(m));
                if angle_end < angle_start {
                    angle_end += 2.0 * std::f64::consts::PI;
                }
                if angle_m < angle_start {
                    angle_m += 2.0 * std::f64::consts::PI;
                }
                angle_start <= angle_m && angle_m <= angle_end
            }
            (Some(start), None) => {
                assert!(self.on_curve(start));
                true
            }
            (None, Some(end)) => {
                assert!(self.on_curve(end));
                true
            }
            (None, None) => true,
        }
    }

    fn get_midpoint(&self, start: Option<Point>, end: Option<Point>) -> Point {
        match (start, end) {
            (Some(start), Some(end)) => {
                assert!(self.on_curve(start));
                assert!(self.on_curve(end));
                let start_rel = start - self.basis;
                let end_rel = end - self.basis;
                let mid = (start_rel + end_rel) / 2.0;
                if mid.norm() < EQ_THRESHOLD {
                    return self.normal.cross(start_rel).normalize() * self.radius.norm()
                        + self.basis;
                }
                let mid = mid.normalize() * self.radius.norm();
                let p1 = mid + self.basis;
                if self.between(p1, Some(start), Some(end)) {
                    return p1;
                } else {
                    return -mid + self.basis;
                }
            }
            (Some(start), None) => {
                assert!(self.on_curve(start));
                return self.basis - (start - self.basis);
            }
            (None, Some(end)) => {
                assert!(self.on_curve(end));
                return self.basis - (end - self.basis);
            }
            (None, None) => {
                return self.basis + self.radius;
            }
        }
    }

    fn project(&self, p: Point) -> Point {
        let v = p - self.basis;
        let v = v - self.normal * (v.dot(self.normal));
        v.normalize() * self.radius.norm() + self.basis
    }

    fn get_bounding_box(
        &self,
        interval_self: Option<Point>,
        midpoint_self: Option<Point>,
    ) -> crate::bounding_box::BoundingBox {
        todo!("Implement this")
    }
}

// Implement partial eqality for Circle
impl PartialEq for Circle {
    fn eq(&self, other: &Circle) -> bool {
        self.basis == other.basis && self.normal == other.normal && self.radius == other.radius
    }
}
