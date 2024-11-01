use crate::{
    efloat::{EFloat64, PositiveEFloat64, SemiPositiveEFloat64},
    points::point::{NonZeroPoint, NormalizedPoint, Point},
    transforms::Transform,
    EQ_THRESHOLD,
};

use super::{curve::Curve, CurveLike};

#[derive(Debug, Clone)]
pub struct Circle {
    pub basis: Point,
    pub normal: NormalizedPoint,
    pub radius: NonZeroPoint,
    dir_cross: NonZeroPoint,
}

pub enum CircleTransform {
    Circle(Circle),
    Ellipse(), // TODO: Implement this
}

impl Circle {
    pub fn new(basis: Point, normal: NormalizedPoint, radius: PositiveEFloat64) -> Circle {
        let radius = match Point::unit_x()
            .cross(normal.value)
            .norm_sq()
            .value
            .lower_bound
            > Point::unit_y()
                .cross(normal.value)
                .norm_sq()
                .value
                .lower_bound
        {
            true => {
                Point::unit_x()
                    .cross(normal.value)
                    .normalize()
                    .unwrap()
                    .value
                    * radius.value
            }
            false => {
                Point::unit_y()
                    .cross(normal.value)
                    .normalize()
                    .unwrap()
                    .value
                    * radius.value
            }
        };
        assert!(
            normal.value.is_perpendicular(radius),
            "Radius and normal must be orthogonal"
        );
        Circle {
            basis,
            normal,
            radius: radius.expect_non_zero(),
            dir_cross: normal.value.cross(radius).expect_non_zero(),
        }
    }

    pub fn transform(&self, transform: Transform) -> CircleTransform {
        let basis = transform * self.basis;
        let normal = transform * (self.normal.value + self.basis) - basis;
        let radius = transform * (self.radius.value + self.basis) - basis;
        let scale_factor = radius.norm().value / self.radius.norm();
        assert!((normal.norm().value - scale_factor * self.normal.value.norm().value) < EQ_THRESHOLD, "Circle can only be transformed with uniform scaling. An extension of this method is planned to return ellipsis.");
        CircleTransform::Circle(Circle::new(
            basis,
            normal.normalize().unwrap(),
            radius.norm().value.expect_positive(),
        ))
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

    fn tangent(&self, p: Point) -> NormalizedPoint {
        assert!(self.on_curve(p));
        self.normal.value.cross(p - self.basis).normalize().unwrap()
    }

    fn on_curve(&self, p: Point) -> bool {
        (p - self.basis).dot(self.normal.value).is_zero()
            && ((p - self.basis).norm().value - self.radius.norm().value).is_zero()
    }

    fn distance(&self, x: Point, y: Point) -> SemiPositiveEFloat64 {
        assert!(self.on_curve(x));
        assert!(self.on_curve(y));
        let angle = (x - self.basis).angle(y - self.basis).unwrap();
        (self.radius.norm().value * angle.value).expect_semi_positive()
    }

    fn interpolate(&self, start: Option<Point>, end: Option<Point>, t: f64) -> Point {
        match (start, end) {
            (Some(start), Some(end)) => {
                assert!(self.on_curve(start));
                assert!(self.on_curve(end));
                let start = start - self.basis;
                let end = end - self.basis;
                let x_start = self.radius.value.dot(start);
                let x_end = self.radius.value.dot(end);
                let y_start = self.dir_cross.value.dot(start);
                let y_end = self.dir_cross.value.dot(end);
                let angle1 = y_start.atan2(x_start);
                let mut angle2 = y_end.atan2(x_end);
                if angle2.upper_bound < angle1.lower_bound {
                    angle2 = angle2 + EFloat64::new(2.0 * std::f64::consts::PI);
                }
                let angle = angle1 + EFloat64::new(t) * (angle2 - angle1);
                angle.cos() * self.radius.value + angle.sin() * self.dir_cross.value + self.basis
            }
            (Some(start), None) => {
                assert!(self.on_curve(start));
                let start = start - self.basis;
                let x_start = self.radius.value.dot(start);
                let y_start = self.dir_cross.value.dot(start);
                let angle1 = y_start.atan2(x_start);
                let angle = angle1 + EFloat64::new(t * 2.0 * std::f64::consts::PI);
                angle.cos() * self.radius.value + angle.sin() * self.dir_cross.value + self.basis
            }
            (None, Some(end)) => {
                assert!(self.on_curve(end));
                let end = end - self.basis;
                let x_end = self.radius.value.dot(end);
                let y_end = self.dir_cross.value.dot(end);
                let angle2 = y_end.atan2(x_end);
                let angle = angle2 + EFloat64::new(t * 2.0 * std::f64::consts::PI);
                angle.cos() * self.radius.value + angle.sin() * self.dir_cross.value + self.basis
            }
            (None, None) => {
                let angle = EFloat64::new(t * 2.0 * std::f64::consts::PI);
                angle.cos() * self.radius.value + angle.sin() * self.dir_cross.value + self.basis
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
                let angle_start = self
                    .dir_cross
                    .value
                    .dot(start)
                    .atan2(self.radius.value.dot(start));
                let mut angle_end = self
                    .dir_cross
                    .value
                    .dot(end)
                    .atan2(self.radius.value.dot(end));
                let mut angle_m = self.dir_cross.value.dot(m).atan2(self.radius.value.dot(m));
                if angle_end.upper_bound < angle_start.lower_bound {
                    angle_end = angle_end + EFloat64::new(2.0 * std::f64::consts::PI);
                }
                if angle_m.upper_bound < angle_start.lower_bound {
                    angle_m = angle_m + EFloat64::new(2.0 * std::f64::consts::PI);
                }
                angle_start.upper_bound <= angle_m.lower_bound
                    && angle_m.upper_bound <= angle_end.lower_bound
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
                let mid = (start_rel + end_rel) / EFloat64::new(2.0).expect_positive();
                if mid.norm().value.is_zero() {
                    return self
                        .normal
                        .value
                        .cross(start_rel)
                        .normalize()
                        .unwrap()
                        .value
                        * self.radius.norm().value
                        + self.basis;
                }
                let mid = mid.normalize().unwrap().value * self.radius.norm().value;
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
                return self.basis + self.radius.value;
            }
        }
    }

    fn project(&self, p: Point) -> Point {
        let v = p - self.basis;
        let v = v - self.normal.value * (v.dot(self.normal.value));
        v.normalize().unwrap().value * self.radius.norm().value + self.basis
    }

    fn get_bounding_box(
        &self,
        _interval_self: Option<Point>,
        _midpoint_self: Option<Point>,
    ) -> crate::bounding_box::BoundingBox {
        todo!("Implement this")
    }

    fn sort(&self, _points: Vec<Option<Point>>) -> Vec<Option<Point>> {
        todo!("Implement this")
    }
}

// Implement partial eqality for Circle
impl PartialEq for Circle {
    fn eq(&self, other: &Circle) -> bool {
        self.basis == other.basis
            && self.normal == other.normal
            && self.radius.value == other.radius.value
    }
}
