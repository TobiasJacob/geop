use crate::{
    bounding_box::BoundingBox, efloat::EFloat64, geometry_error::GeometryResult, point::Point,
    transforms::Transform,
};

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
    pub fn new(basis: Point, normal: Point, radius: EFloat64) -> Circle {
        assert!(normal.is_normalized());
        let radius = match Point::unit_x().cross(normal).norm_sq().lower_bound
            > Point::unit_y().cross(normal).norm_sq().lower_bound
        {
            true => Point::unit_x().cross(normal).normalize().unwrap() * radius,
            false => Point::unit_y().cross(normal).normalize().unwrap() * radius,
        };
        assert!(
            normal.dot(radius) == 0.0,
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
        assert!(transform.uniform_scale_factor() > 0.0, "Circle can only be transformed with uniform scaling. An extension of this method is planned to return ellipsis.");
        CircleTransform::Circle(Circle::new(
            basis,
            normal.normalize().unwrap(),
            radius.norm(),
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

    fn tangent(&self, p: Point) -> GeometryResult<Point> {
        assert!(self.on_curve(p));
        Ok(self.normal.cross(p - self.basis).normalize().unwrap())
    }

    fn on_curve(&self, p: Point) -> bool {
        (p - self.basis).dot(self.normal) == 0.0
            && ((p - self.basis).norm() - self.radius.norm()) == 0.0
    }

    fn distance(&self, x: Point, y: Point) -> GeometryResult<EFloat64> {
        assert!(self.on_curve(x));
        assert!(self.on_curve(y));
        let angle = (x - self.basis).angle(y - self.basis);
        Ok(self.radius.norm() * angle.unwrap())
    }

    fn interpolate(
        &self,
        start: Option<Point>,
        end: Option<Point>,
        t: f64,
    ) -> GeometryResult<Point> {
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
                assert!(angle2 != angle1);
                if angle2.upper_bound < angle1.lower_bound {
                    angle2 = angle2 + EFloat64::two_pi();
                }
                let angle = angle1 + EFloat64::from(t) * (angle2 - angle1);
                Ok(angle.cos() * self.radius + angle.sin() * self.dir_cross + self.basis)
            }
            (Some(start), None) => {
                assert!(self.on_curve(start));
                let start = start - self.basis;
                let x_start = self.radius.dot(start);
                let y_start = self.dir_cross.dot(start);
                let angle1 = y_start.atan2(x_start);
                let angle = angle1 + EFloat64::from(t * std::f64::consts::PI * 2.0);
                Ok(angle.cos() * self.radius + angle.sin() * self.dir_cross + self.basis)
            }
            (None, Some(end)) => {
                assert!(self.on_curve(end));
                let end = end - self.basis;
                let x_end = self.radius.dot(end);
                let y_end = self.dir_cross.dot(end);
                let angle2 = y_end.atan2(x_end);
                let angle = angle2 + EFloat64::from(t * std::f64::consts::PI * 2.0);
                Ok(angle.cos() * self.radius + angle.sin() * self.dir_cross + self.basis)
            }
            (None, None) => {
                let angle = EFloat64::from(t * std::f64::consts::PI * 2.0);
                Ok(angle.cos() * self.radius + angle.sin() * self.dir_cross + self.basis)
            }
        }
    }

    // TODO: Assert start != end
    // Checks if m is between x and y. m==x and m==y are true.
    fn between(&self, m: Point, start: Option<Point>, end: Option<Point>) -> GeometryResult<bool> {
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
                if angle_end.upper_bound < angle_start.lower_bound {
                    angle_end = angle_end + EFloat64::two_pi();
                }
                if angle_m.upper_bound < angle_start.lower_bound {
                    angle_m = angle_m + EFloat64::two_pi();
                }
                Ok(angle_start <= angle_m.upper_bound && angle_m <= angle_end.upper_bound)
            }
            (Some(start), None) => {
                assert!(self.on_curve(start));
                Ok(true)
            }
            (None, Some(end)) => {
                assert!(self.on_curve(end));
                Ok(true)
            }
            (None, None) => Ok(true),
        }
    }

    fn get_midpoint(&self, start: Option<Point>, end: Option<Point>) -> GeometryResult<Point> {
        match (start, end) {
            (Some(start), Some(end)) => {
                assert!(self.on_curve(start));
                assert!(self.on_curve(end));
                let start_rel = start - self.basis;
                let end_rel = end - self.basis;
                let mid = ((start_rel + end_rel) / EFloat64::two()).unwrap();
                if mid.norm() == 0.0 {
                    return Ok(self.normal.cross(start_rel).normalize().unwrap()
                        * self.radius.norm()
                        + self.basis);
                }
                let mid = mid.normalize().unwrap() * self.radius.norm();
                let p1 = mid + self.basis;
                if self.between(p1, Some(start), Some(end)).unwrap() {
                    return Ok(p1);
                } else {
                    return Ok(-mid + self.basis);
                }
            }
            (Some(start), None) => {
                assert!(self.on_curve(start));
                return Ok(self.basis - (start - self.basis));
            }
            (None, Some(end)) => {
                assert!(self.on_curve(end));
                return Ok(self.basis - (end - self.basis));
            }
            (None, None) => {
                return Ok(self.basis + self.radius);
            }
        }
    }

    fn project(&self, p: Point) -> Point {
        let v = p - self.basis;
        let v = v - self.normal * (v.dot(self.normal));
        v.normalize().unwrap() * self.radius.norm() + self.basis
    }

    fn get_bounding_box(
        &self,
        _interval_self: Option<Point>,
        _midpoint_self: Option<Point>,
    ) -> GeometryResult<BoundingBox> {
        todo!("Implement this")
    }

    fn shrink_bounding_box(
        &self,
        _start: Option<Point>,
        _end: Option<Point>,
        _bounding_box: BoundingBox,
    ) -> GeometryResult<BoundingBox> {
        todo!("Implement this")
    }

    fn sort(&self, _points: Vec<Option<Point>>) -> Vec<Option<Point>> {
        todo!("Implement this")
    }
}

// Implement partial eqality for Circle
impl PartialEq for Circle {
    fn eq(&self, other: &Circle) -> bool {
        self.basis == other.basis && self.normal == other.normal && self.radius == other.radius
    }
}
