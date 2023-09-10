use std::rc::Rc;

use crate::{curves::line::Line, points::point::Point, EQ_THRESHOLD, transforms::Transform};

use super::surface::{Surface, SurfaceCurve, TangentPoint};

#[derive(Clone, Debug)]
pub struct Plane {
    pub basis: Point,
    pub u_slope: Point,
    pub v_slope: Point,
}

impl Plane {
    pub fn new(basis: Point, u_slope: Point, v_slope: Point) -> Plane {
        Plane {
            basis,
            u_slope: u_slope.normalize(),
            v_slope: v_slope.normalize(),
        }
    }

    pub fn curve_from_to(&self, p: Point, q: Point) -> Line {
        return Line::new(p, q - p);
    }

    pub fn transform(&self, transform: Transform) -> Self {
        let basis = transform * self.basis;
        let u_slope = transform * (self.u_slope + self.basis) - basis;
        let v_slope = transform * (self.v_slope + self.basis) - basis;
        Plane::new(basis, u_slope.normalize(), v_slope.normalize())
    }
}

impl Surface for Plane {
    fn transform(&self, transform: Transform) -> Rc<dyn Surface> {
        Rc::new(self.transform(transform))
    }

    fn on_surface(&self, p: Point) -> bool {
        let diff = p - self.basis;
        let u = diff.dot(self.normal(p));
        u.abs() < EQ_THRESHOLD
    }

    // fn point_at(&self, u: f64, v: f64) -> Point {
    //     self.basis + self.u_slope * u + self.v_slope * v
    // }

    // fn project(&self, p: &Point) -> (f64, f64) {
    //     let v = *p - self.basis;
    //     let u = v.dot(self.u_slope) / self.u_slope.norm();
    //     let v = v - self.u_slope * u;
    //     let v = v.dot(self.v_slope) / self.v_slope.norm();
    //     (u, v)
    // }

    // fn derivative_u(&self, _u: f64, _v: f64) -> Point {
    //     self.u_slope
    // }

    // fn derivative_v(&self, _u: f64, _v: f64) -> Point {
    //     self.v_slope
    // }

    fn normal(&self, _p: Point) -> Point {
        self.u_slope.cross(self.v_slope)
    }

    // fn normalize(&mut self) {
    //     self.u_slope = self.u_slope / self.u_slope.norm();
    //     self.v_slope = self.v_slope / self.v_slope.norm();
    // }

    // fn is_normalized(&self) -> bool {
    //     self.u_slope.is_normalized() && self.v_slope.is_normalized()
    // }

    fn metric(&self, _x: Point, u: TangentPoint, v: TangentPoint) -> f64 {
        assert!(u.0.z.abs() < EQ_THRESHOLD);
        assert!(v.0.z.abs() < EQ_THRESHOLD);
        u.0.dot(v.0)
    }

    fn distance(&self, x: Point, y: Point) -> f64 {
        (x - y).norm()
    }

    fn exp(&self, x: Point, u: TangentPoint) -> Point {
        assert!(self.on_surface(x));
        assert!(u.0.z.abs() < EQ_THRESHOLD);
        x + u.0.x * self.u_slope + u.0.y * self.v_slope
    }

    fn log(&self, x: Point, y: Point) -> TangentPoint {
        assert!(self.on_surface(x));
        assert!(self.on_surface(y));
        let diff = y - x;
        TangentPoint(Point::new(
            diff.dot(self.u_slope),
            diff.dot(self.v_slope),
            0.0,
        ))
    }

    fn parallel_transport(&self, v: TangentPoint, x: Point, y: Point) -> Point {
        assert!(self.on_surface(x));
        assert!(self.on_surface(y));
        assert!(v.0.z.abs() < EQ_THRESHOLD);
        v.0
    }

    fn geodesic(&self, p: Point, q: Point) -> SurfaceCurve {
        SurfaceCurve::Line(Line::new(p, q - p))
    }
}

impl PartialEq for Plane {
    fn eq(&self, other: &Plane) -> bool {
            self.u_slope.is_parallel(other.u_slope)
            && self.v_slope.is_parallel(other.v_slope)
            && (self.basis - other.basis).dot(self.u_slope.cross(other.u_slope)).abs() < EQ_THRESHOLD
    }
}
