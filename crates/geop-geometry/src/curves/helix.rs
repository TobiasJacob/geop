use core::f64;

use crate::{
    bounding_box::BoundingBox, efloat::EFloat64, point::Point, transforms::Transform, HORIZON_DIST,
};

use super::{curve::Curve, CurveLike};

#[derive(Debug, Clone)]
pub struct Helix {
    pub basis: Point,
    pub pitch: Point,
    pub radius: Point,
    dir_cross: Point,
    right_winding: bool,
}

impl Helix {
    pub fn new(basis: Point, pitch: Point, radius: Point, right_winding: bool) -> Helix {
        assert!(
            pitch.dot(radius) == 0.0,
            "Radius and pitch must be orthogonal"
        );
        Helix {
            basis,
            pitch,
            radius,
            dir_cross: match right_winding {
                true => pitch.normalize().unwrap().cross(radius),
                false => -pitch.normalize().unwrap().cross(radius),
            },
            right_winding,
        }
    }

    pub fn transform(&self, transform: Transform) -> Self {
        let basis_old = self.basis;
        let basis = transform * self.basis;
        let pitch = transform * (self.pitch + basis_old) - basis;
        let radius = transform * (self.radius + basis_old) - basis;
        Helix::new(basis, pitch, radius, self.right_winding)
    }

    pub fn neg(&self) -> Helix {
        Helix::new(self.basis, -self.pitch, self.radius, self.right_winding)
    }

    pub fn point_at_pitch(&self, t: EFloat64) -> Point {
        self.basis
            + (t) * self.pitch
            + self.radius * (EFloat64::two_pi() * t).cos()
            + self.dir_cross * (EFloat64::two_pi() * t).sin()
    }
}

// Helix equation is r(t) = basis + t * pitch + cos(2pi * t) * radius + sin(2pi * t) * dir_cross
impl CurveLike for Helix {
    fn transform(&self, transform: Transform) -> Curve {
        Curve::Helix(self.transform(transform))
    }

    fn neg(&self) -> Curve {
        Curve::Helix(self.neg())
    }

    fn tangent(&self, p: Point) -> Point {
        assert!(self.on_curve(p));
        (self.pitch.cross(p - self.basis).normalize().unwrap()
            + (self.pitch / EFloat64::two_pi()).unwrap())
        .normalize()
        .unwrap()
    }

    fn on_curve(&self, p: Point) -> bool {
        let t = (p - self.basis).dot(self.pitch) / self.pitch.norm_sq();
        let t = t.unwrap();
        let p_expected = self.basis
            + t * self.pitch
            + self.radius * (EFloat64::two_pi() * t).cos()
            + self.dir_cross * (EFloat64::two_pi() * t).sin();
        p == p_expected
    }

    fn distance(&self, x: Point, y: Point) -> EFloat64 {
        assert!(self.on_curve(x));
        assert!(self.on_curve(y));
        let t_x = x.dot(self.pitch) / self.pitch.norm_sq();
        let t_y = y.dot(self.pitch) / self.pitch.norm_sq();
        let t_x = t_x.unwrap();
        let t_y = t_y.unwrap();
        return (t_x - t_y).abs() * self.radius.norm() * EFloat64::two_pi();
    }

    fn interpolate(&self, start: Option<Point>, end: Option<Point>, t: f64) -> Point {
        match (start, end) {
            (Some(start), Some(end)) => {
                assert!(self.on_curve(start));
                assert!(self.on_curve(end));
                let t_start = (start - self.basis).dot(self.pitch) / self.pitch.norm_sq();
                let t_end = (end - self.basis).dot(self.pitch) / self.pitch.norm_sq();
                let t_start = t_start.unwrap();
                let t_end = t_end.unwrap();
                let t = t_start + EFloat64::from(t) * (t_end - t_start);
                return self.basis
                    + self.pitch * t
                    + self.radius * (EFloat64::two_pi() * t).cos()
                    + self.dir_cross * (EFloat64::two_pi() * t).sin();
            }
            (Some(start), None) => {
                let t_start = (start - self.basis).dot(self.pitch) / self.pitch.norm_sq();
                let t_start = t_start.unwrap();
                let t = t_start + EFloat64::from(t * HORIZON_DIST);
                return self.basis
                    + self.pitch * t
                    + self.radius * (EFloat64::two_pi() * t).cos()
                    + self.dir_cross * (EFloat64::two_pi() * t).sin();
            }
            (None, Some(end)) => {
                let t_end = (end - self.basis).dot(self.pitch) / self.pitch.norm_sq();
                let t_end = t_end.unwrap();
                let t = t_end - EFloat64::from((1.0 - t) * HORIZON_DIST);
                return self.basis
                    + self.pitch * t
                    + self.radius * (EFloat64::two_pi() * t).cos()
                    + self.dir_cross * (EFloat64::two_pi() * t).sin();
            }
            (None, None) => {
                let t = (t - 0.5) * HORIZON_DIST;
                return self.basis
                    + self.pitch * EFloat64::from(t)
                    + self.radius * EFloat64::from(t * 2.0 * f64::consts::PI).cos()
                    + self.dir_cross * EFloat64::from(t * 2.0 * f64::consts::PI).sin();
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
                let t_start = start.dot(self.pitch) / self.pitch.norm_sq();
                let t_end = end.dot(self.pitch) / self.pitch.norm_sq();
                let t_m = m.dot(self.pitch) / self.pitch.norm_sq();
                let t_start = t_start.unwrap();
                let t_end = t_end.unwrap();
                let t_m = t_m.unwrap();
                t_start <= t_m.upper_bound && t_m <= t_end.upper_bound
            }
            (Some(start), None) => {
                assert!(self.on_curve(start));
                let t_start = start.dot(self.pitch) / self.pitch.norm_sq();
                let t_m = m.dot(self.pitch) / self.pitch.norm_sq();
                let t_start = t_start.unwrap();
                let t_m = t_m.unwrap();
                t_start <= t_m.upper_bound
            }
            (None, Some(end)) => {
                assert!(self.on_curve(end));
                let t_end = end.dot(self.pitch) / self.pitch.norm_sq();
                let t_m = m.dot(self.pitch) / self.pitch.norm_sq();
                let t_end = t_end.unwrap();
                let t_m = t_m.unwrap();
                t_m <= t_end.upper_bound
            }
            (None, None) => true,
        }
    }

    fn get_midpoint(&self, start: Option<Point>, end: Option<Point>) -> Option<Point> {
        match (start, end) {
            (Some(start), Some(end)) => {
                assert!(self.on_curve(start));
                assert!(self.on_curve(end));
                let t_start = (start - self.basis).dot(self.pitch) / self.pitch.norm_sq();
                let t_end = (end - self.basis).dot(self.pitch) / self.pitch.norm_sq();
                let t_start = t_start.unwrap();
                let t_end = t_end.unwrap();
                let t = (t_start + t_end) / EFloat64::from(2.0);
                let t = t.unwrap();
                return Some(
                    self.basis
                        + self.pitch * t
                        + self.radius * (EFloat64::two_pi() * t).cos()
                        + self.dir_cross * (EFloat64::two_pi() * t).sin(),
                );
            }
            (Some(start), None) => {
                assert!(self.on_curve(start));
                let t_start = (start - self.basis).dot(self.pitch) / self.pitch.norm_sq();
                let t_start = t_start.unwrap();
                let t = t_start + EFloat64::from(HORIZON_DIST);
                return Some(
                    self.basis
                        + self.pitch * t
                        + self.radius * (EFloat64::two_pi() * t).cos()
                        + self.dir_cross * (EFloat64::two_pi() * t).sin(),
                );
            }
            (None, Some(end)) => {
                assert!(self.on_curve(end));
                let t_end = (end - self.basis).dot(self.pitch) / self.pitch.norm_sq();
                let t_end = t_end.unwrap();
                let t = t_end - EFloat64::from(HORIZON_DIST);
                return Some(
                    self.basis
                        + self.pitch * t
                        + self.radius * (EFloat64::two_pi() * t).cos()
                        + self.dir_cross * (EFloat64::two_pi() * t).sin(),
                );
            }
            (None, None) => {
                return Some(self.basis + self.radius);
            }
        }
    }

    fn project(&self, _p: Point) -> Point {
        todo!("Implement this")
    }

    fn get_bounding_box(
        &self,
        _interval_self: Option<Point>,
        _midpoint_self: Option<Point>,
    ) -> BoundingBox {
        todo!("Implement this")
    }

    fn shrink_bounding_box(
        &self,
        start: Option<Point>,
        end: Option<Point>,
        bounding_box: BoundingBox,
    ) -> BoundingBox {
        todo!("Implement this")
    }

    fn sort(&self, _points: Vec<Option<Point>>) -> Vec<Option<Point>> {
        todo!("Implement this")
    }
}

// Implement partial eqality for Circle
impl PartialEq for Helix {
    fn eq(&self, other: &Helix) -> bool {
        self.basis == other.basis && self.pitch == other.pitch && self.radius == other.radius
    }
}
