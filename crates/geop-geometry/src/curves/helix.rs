use core::f64;

use crate::{
    efloat::EFloat64,
    points::point::{NonZeroPoint, NormalizedPoint, Point},
    transforms::Transform,
    HORIZON_DIST,
};

use super::{curve::Curve, CurveLike};

#[derive(Debug, Clone)]
pub struct Helix {
    pub basis: Point,
    pub pitch: NonZeroPoint,
    pub radius: NonZeroPoint,
    dir_cross: NonZeroPoint,
    right_winding: bool,
}

impl Helix {
    pub fn new(
        basis: Point,
        pitch: NonZeroPoint,
        radius: NonZeroPoint,
        right_winding: bool,
    ) -> Helix {
        assert!(
            pitch.value.is_perpendicular(radius.value),
            "Radius and pitch must be orthogonal"
        );
        Helix {
            basis,
            pitch,
            radius,
            dir_cross: match right_winding {
                true => pitch
                    .normalize()
                    .value
                    .cross(radius.value)
                    .expect_non_zero(),
                false => {
                    (-pitch.value.normalize().unwrap().value.cross(radius.value)).expect_non_zero()
                }
            },
            right_winding,
        }
    }

    pub fn transform(&self, transform: Transform) -> Self {
        let basis_old = self.basis;
        let basis = transform * self.basis;
        let pitch = transform * (self.pitch.value + basis_old) - basis;
        let radius = transform * (self.radius.value + basis_old) - basis;
        Helix::new(
            basis,
            pitch.expect_non_zero(),
            radius.expect_non_zero(),
            self.right_winding,
        )
    }

    pub fn neg(&self) -> Helix {
        Helix::new(
            self.basis,
            (-self.pitch.value).expect_non_zero(),
            self.radius,
            self.right_winding,
        )
    }

    pub fn point_at_pitch(&self, t: f64) -> Point {
        self.basis
            + EFloat64::new(t) * self.pitch.value
            + self.radius.value * EFloat64::new((2.0 * f64::consts::PI * t).cos())
            + self.dir_cross.value * EFloat64::new((2.0 * f64::consts::PI * t).sin())
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

    fn tangent(&self, p: Point) -> NormalizedPoint {
        assert!(self.on_curve(p));
        (*self.pitch.cross(p - self.basis).normalize().unwrap()
            + self.pitch.value / EFloat64::two_pi().expect_positive())
        .normalize()
        .unwrap()
    }

    fn on_curve(&self, p: Point) -> bool {
        let t = (p - self.basis).dot(*self.pitch) / self.pitch.norm_sq();
        let p_expected = self.basis
            + t * self.pitch
            + self.radius * (2.0 * f64::consts::PI * t).cos()
            + self.dir_cross * (2.0 * f64::consts::PI * t).sin();
        p == p_expected
    }

    fn distance(&self, x: Point, y: Point) -> f64 {
        assert!(self.on_curve(x));
        assert!(self.on_curve(y));
        let t_x = x.dot(self.pitch) / self.pitch.norm_sq();
        let t_y = y.dot(self.pitch) / self.pitch.norm_sq();
        return (t_x - t_y).abs() * self.radius.norm() * 2.0 * f64::consts::PI;
    }

    fn interpolate(&self, start: Option<Point>, end: Option<Point>, t: f64) -> Point {
        match (start, end) {
            (Some(start), Some(end)) => {
                assert!(self.on_curve(start));
                assert!(self.on_curve(end));
                let t_start = (start - self.basis).dot(self.pitch) / self.pitch.norm_sq();
                let t_end = (end - self.basis).dot(self.pitch) / self.pitch.norm_sq();
                let t = t_start + t * (t_end - t_start);
                return self.basis
                    + self.pitch * t
                    + self.radius * (2.0 * f64::consts::PI * t).cos()
                    + self.dir_cross * (2.0 * f64::consts::PI * t).sin();
            }
            (Some(start), None) => {
                let t_start = (start - self.basis).dot(self.pitch) / self.pitch.norm_sq();
                let t = t_start + t * HORIZON_DIST;
                return self.basis
                    + self.pitch * t
                    + self.radius * (2.0 * f64::consts::PI * t).cos()
                    + self.dir_cross * (2.0 * f64::consts::PI * t).sin();
            }
            (None, Some(end)) => {
                let t_end = (end - self.basis).dot(self.pitch) / self.pitch.norm_sq();
                let t = t_end - (1.0 - t) * HORIZON_DIST;
                return self.basis
                    + self.pitch * t
                    + self.radius * (2.0 * f64::consts::PI * t).cos()
                    + self.dir_cross * (2.0 * f64::consts::PI * t).sin();
            }
            (None, None) => {
                let t = (t - 0.5) * HORIZON_DIST;
                return self.basis
                    + self.pitch * t
                    + self.radius * (t * 2.0 * f64::consts::PI).cos()
                    + self.dir_cross * (t * 2.0 * f64::consts::PI).sin();
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
                t_start <= t_m && t_m <= t_end
            }
            (Some(start), None) => {
                assert!(self.on_curve(start));
                let t_start = start.dot(self.pitch) / self.pitch.norm_sq();
                let t_m = m.dot(self.pitch) / self.pitch.norm_sq();
                t_start <= t_m
            }
            (None, Some(end)) => {
                assert!(self.on_curve(end));
                let t_end = end.dot(self.pitch) / self.pitch.norm_sq();
                let t_m = m.dot(self.pitch) / self.pitch.norm_sq();
                t_m <= t_end
            }
            (None, None) => true,
        }
    }

    fn get_midpoint(&self, start: Option<Point>, end: Option<Point>) -> Point {
        match (start, end) {
            (Some(start), Some(end)) => {
                assert!(self.on_curve(start));
                assert!(self.on_curve(end));
                let t_start = (start - self.basis).dot(self.pitch) / self.pitch.norm_sq();
                let t_end = (end - self.basis).dot(self.pitch) / self.pitch.norm_sq();
                let t = (t_start + t_end) / 2.0;
                return self.basis
                    + self.pitch * t
                    + self.radius * (2.0 * f64::consts::PI * t).cos()
                    + self.dir_cross * (2.0 * f64::consts::PI * t).sin();
            }
            (Some(start), None) => {
                assert!(self.on_curve(start));
                let t_start = (start - self.basis).dot(self.pitch) / self.pitch.norm_sq();
                let t = t_start + HORIZON_DIST;
                return self.basis
                    + self.pitch * t
                    + self.radius * (2.0 * f64::consts::PI * t).cos()
                    + self.dir_cross * (2.0 * f64::consts::PI * t).sin();
            }
            (None, Some(end)) => {
                assert!(self.on_curve(end));
                let t_end = (end - self.basis).dot(self.pitch) / self.pitch.norm_sq();
                let t = t_end - HORIZON_DIST;
                return self.basis
                    + self.pitch * t
                    + self.radius * (2.0 * f64::consts::PI * t).cos()
                    + self.dir_cross * (2.0 * f64::consts::PI * t).sin();
            }
            (None, None) => {
                return self.basis + self.radius;
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
    ) -> crate::bounding_box::BoundingBox {
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
