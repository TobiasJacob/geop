use crate::{
    curves::{curve::Curve, line::Line},
    points::point::Point,
    transforms::Transform,
    EQ_THRESHOLD, HORIZON_DIST,
};

use super::surface::TangentPoint;

#[derive(Clone, Debug)]
pub struct Plane {
    pub basis: Point,
    pub u_slope: Point,
    pub v_slope: Point,
}

impl Plane {
    pub fn new(basis: Point, u_slope: Point, v_slope: Point) -> Plane {
        assert!(!u_slope.cross(v_slope).is_zero());
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

    pub fn normal(&self) -> Point {
        self.u_slope.cross(self.v_slope)
    }

    pub fn neg(&self) -> Self {
        Plane::new(self.basis, self.u_slope, -self.v_slope)
    }

    pub fn on_surface(&self, p: Point) -> bool {
        let normal = self.normal();
        let p_project = p.dot(normal);
        let b_project = self.basis.dot(normal);
        (p_project - b_project).abs() < EQ_THRESHOLD
    }

    pub fn metric(&self, _x: Point, u: TangentPoint, v: TangentPoint) -> f64 {
        assert!(u.0.z.abs() < EQ_THRESHOLD);
        assert!(v.0.z.abs() < EQ_THRESHOLD);
        u.0.dot(v.0)
    }

    pub fn distance(&self, x: Point, y: Point) -> f64 {
        (x - y).norm()
    }

    pub fn exp(&self, x: Point, u: TangentPoint) -> Point {
        assert!(self.on_surface(x));
        assert!(u.0.z.abs() < EQ_THRESHOLD);
        x + u.0.x * self.u_slope + u.0.y * self.v_slope
    }

    pub fn log(&self, x: Point, y: Point) -> TangentPoint {
        assert!(self.on_surface(x));
        assert!(self.on_surface(y));
        let diff = y - x;
        TangentPoint(Point::new(
            diff.dot(self.u_slope),
            diff.dot(self.v_slope),
            0.0,
        ))
    }

    pub fn parallel_transport(&self, v: TangentPoint, x: Point, y: Point) -> Point {
        assert!(self.on_surface(x));
        assert!(self.on_surface(y));
        assert!(v.0.z.abs() < EQ_THRESHOLD);
        v.0
    }

    pub fn geodesic(&self, p: Point, q: Point) -> Curve {
        Curve::Line(Line::new(p, q - p))
    }

    pub fn point_grid(&self) -> Vec<Point> {
        vec![
            self.basis - self.u_slope * HORIZON_DIST - self.v_slope * HORIZON_DIST,
            self.basis + self.u_slope * HORIZON_DIST - self.v_slope * HORIZON_DIST,
            self.basis - self.u_slope * HORIZON_DIST + self.v_slope * HORIZON_DIST,
            self.basis + self.u_slope * HORIZON_DIST + self.v_slope * HORIZON_DIST,
        ]
    }
}

impl PartialEq for Plane {
    fn eq(&self, other: &Plane) -> bool {
        self.u_slope.is_parallel(other.u_slope)
            && self.v_slope.is_parallel(other.v_slope)
            && (self.basis - other.basis)
                .dot(self.u_slope.cross(other.u_slope))
                .abs()
                < EQ_THRESHOLD
    }
}
