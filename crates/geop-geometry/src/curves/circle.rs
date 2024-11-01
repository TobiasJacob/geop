use crate::{
    efloat::{
        efloat::EFloat64, positive_efloat::PositiveEFloat64,
        semi_positive_efloat::SemiPositiveEFloat64,
    },
    points::{nonzero_point::NonZeroPoint, normalized_point::NormalizedPoint, point::Point},
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
            .cross(normal.as_point())
            .norm_sq()
            .as_efloat()
            .lower_bound
            > Point::unit_y()
                .cross(normal.as_point())
                .norm_sq()
                .as_efloat()
                .lower_bound
        {
            true => {
                Point::unit_x()
                    .cross(normal.as_point())
                    .normalize()
                    .unwrap()
                    * radius.as_efloat()
            }
            false => {
                Point::unit_y()
                    .cross(normal.as_point())
                    .normalize()
                    .unwrap()
                    .as_point()
                    * radius.as_efloat()
            }
        };
        assert!(
            normal.as_point().is_perpendicular(radius),
            "Radius and normal must be orthogonal"
        );
        Circle {
            basis,
            normal,
            radius: radius.expect_non_zero(),
            dir_cross: normal.as_point().cross(radius).expect_non_zero(),
        }
    }

    pub fn transform(&self, transform: Transform) -> CircleTransform {
        let basis = transform * self.basis;
        let normal = transform.transform_normalizedpoint_with_base(self.normal, self.basis);
        let radius = transform.transform_nonzeropoint_with_base(self.radius, self.basis);
        {
            let dir_cross = normal.as_point().cross(radius.as_point());
            let scale_factor_radius = radius.norm() / self.radius.norm();
            let scale_factor_normal = normal.as_nonzero().norm() / self.normal.as_nonzero().norm();
            let scale_factor_dir_cross = dir_cross.norm() / self.dir_cross.norm();

            assert!(scale_factor_normal.as_efloat() == scale_factor_radius.as_efloat(), "Circle can only be transformed with uniform scaling. An extension of this method is planned to return ellipsis.");
            assert!(scale_factor_normal.as_efloat() == scale_factor_dir_cross.as_efloat(), "Circle can only be transformed with uniform scaling. An extension of this method is planned to return ellipsis.");
        }
        CircleTransform::Circle(Circle::new(basis, normal, radius.norm()))
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
        self.normal
            .as_point()
            .cross(p - self.basis)
            .normalize()
            .unwrap()
    }

    fn on_curve(&self, p: Point) -> bool {
        (p - self.basis).is_perpendicular(self.normal.as_point())
            && ((p - self.basis).norm() - self.radius.norm().as_semi_positive()) == 0.0
    }

    fn distance(&self, x: Point, y: Point) -> SemiPositiveEFloat64 {
        assert!(self.on_curve(x));
        assert!(self.on_curve(y));
        let angle = (x - self.basis).angle(y - self.basis).unwrap();
        self.radius.norm().as_semi_positive() * angle
    }

    fn interpolate(&self, start: Option<Point>, end: Option<Point>, t: f64) -> Point {
        match (start, end) {
            (Some(start), Some(end)) => {
                assert!(self.on_curve(start));
                assert!(self.on_curve(end));
                let start = start - self.basis;
                let end = end - self.basis;
                let x_start = self.radius.as_point().dot(start);
                let x_end = self.radius.as_point().dot(end);
                let y_start = self.dir_cross.as_point().dot(start);
                let y_end = self.dir_cross.as_point().dot(end);
                let angle1 = y_start.atan2(x_start);
                let mut angle2 = y_end.atan2(x_end);
                if angle2.upper_bound < angle1.lower_bound {
                    angle2 = angle2 + EFloat64::new(2.0 * std::f64::consts::PI);
                }
                let angle = angle1 + EFloat64::new(t) * (angle2 - angle1);
                angle.cos() * self.radius.as_point()
                    + angle.sin() * self.dir_cross.as_point()
                    + self.basis
            }
            (Some(start), None) => {
                assert!(self.on_curve(start));
                let start = start - self.basis;
                let x_start = self.radius.as_point().dot(start);
                let y_start = self.dir_cross.as_point().dot(start);
                let angle1 = y_start.atan2(x_start);
                let angle = angle1 + EFloat64::new(t * 2.0 * std::f64::consts::PI);
                angle.cos() * self.radius.as_point()
                    + angle.sin() * self.dir_cross.as_point()
                    + self.basis
            }
            (None, Some(end)) => {
                assert!(self.on_curve(end));
                let end = end - self.basis;
                let x_end = self.radius.as_point().dot(end);
                let y_end = self.dir_cross.as_point().dot(end);
                let angle2 = y_end.atan2(x_end);
                let angle = angle2 + EFloat64::new(t * 2.0 * std::f64::consts::PI);
                angle.cos() * self.radius.as_point()
                    + angle.sin() * self.dir_cross.as_point()
                    + self.basis
            }
            (None, None) => {
                let angle = EFloat64::new(t * 2.0 * std::f64::consts::PI);
                angle.cos() * self.radius.as_point()
                    + angle.sin() * self.dir_cross.as_point()
                    + self.basis
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
                    .as_point()
                    .dot(start)
                    .atan2(self.radius.as_point().dot(start));
                let mut angle_end = self
                    .dir_cross
                    .as_point()
                    .dot(end)
                    .atan2(self.radius.as_point().dot(end));
                let mut angle_m = self
                    .dir_cross
                    .as_point()
                    .dot(m)
                    .atan2(self.radius.as_point().dot(m));
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
                match mid.normalize() {
                    Some(mid) => {
                        let p1 = mid.as_point() * self.radius.norm().as_efloat() + self.basis;
                        if self.between(p1, Some(start), Some(end)) {
                            return p1;
                        } else {
                            return -mid.as_point() * self.radius.norm().as_efloat() + self.basis;
                        }
                    }
                    None => {
                        return self.normal.as_point().cross(start_rel) + self.basis;
                    }
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
                return self.basis + self.radius.as_point();
            }
        }
    }

    fn project(&self, p: Point) -> Point {
        let v = p - self.basis;
        let v = self.normal.perpendicular_decomposition(v);
        match v.normalize() {
            Some(v) => v * self.radius.norm().as_efloat() + self.basis,
            None => self.basis + self.radius.as_point(),
        }
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
            && self.radius.as_point() == other.radius.as_point()
    }
}
