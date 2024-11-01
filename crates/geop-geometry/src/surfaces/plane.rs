use crate::{
    curves::{curve::Curve, line::Line},
    point::Point,
    transforms::Transform,
    EQ_THRESHOLD, HORIZON_DIST,
};

use super::{
    surface::{Surface, TangentPoint},
    SurfaceLike,
};

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
            u_slope: u_slope.normalize().unwrap(),
            v_slope: v_slope.normalize().unwrap(),
        }
    }

    fn transform(&self, transform: Transform) -> Self {
        let basis = transform * self.basis;
        let u_slope = transform * (self.u_slope + self.basis) - basis;
        let v_slope = transform * (self.v_slope + self.basis) - basis;
        Plane::new(
            basis,
            u_slope.normalize().unwrap(),
            v_slope.normalize().unwrap(),
        )
    }

    fn normal(&self) -> Point {
        self.u_slope.cross(self.v_slope)
    }

    fn neg(&self) -> Self {
        Plane::new(self.basis, self.u_slope, -self.v_slope)
    }

    pub fn point_grid_dense(&self, density: f64, horizon_dist: f64) -> Vec<Point> {
        let n = (density + 1.1) as usize;
        let mut points = Vec::new();
        for i in 0..n {
            for j in 0..n {
                let u = i as f64 / (n as f64 - 1.0);
                let v = j as f64 / (n as f64 - 1.0);
                let point = self.basis
                    + (u - 0.5) * horizon_dist * self.u_slope
                    + (v - 0.5) * horizon_dist * self.v_slope;
                points.push(point);
            }
        }
        points
    }

    pub fn is_parallel(&self, other: &Plane) -> bool {
        self.normal().is_parallel(other.normal())
    }
}

impl SurfaceLike for Plane {
    fn transform(&self, transform: Transform) -> Surface {
        Surface::Plane(self.transform(transform))
    }

    fn normal(&self, p: Point) -> Point {
        assert!(self.on_surface(p));
        self.normal()
    }

    fn neg(&self) -> Surface {
        Surface::Plane(self.neg())
    }

    fn on_surface(&self, p: Point) -> bool {
        let normal = self.normal();
        let p_project = p.dot(normal);
        let b_project = self.basis.dot(normal);
        (p_project - b_project).abs() < EQ_THRESHOLD
    }

    fn metric(&self, _x: Point, u: TangentPoint, v: TangentPoint) -> f64 {
        u.dot(v)
    }

    fn distance(&self, x: Point, y: Point) -> f64 {
        (x - y).norm()
    }

    fn exp(&self, x: Point, u: TangentPoint) -> Point {
        assert!(self.on_surface(x));
        x + u
    }

    fn log(&self, x: Point, y: Point) -> Option<TangentPoint> {
        assert!(self.on_surface(x));
        assert!(self.on_surface(y));
        Some(y - x)
    }

    fn parallel_transport(
        &self,
        v: Option<TangentPoint>,
        x: Point,
        y: Point,
    ) -> Option<TangentPoint> {
        assert!(self.on_surface(x));
        assert!(self.on_surface(y));
        v
    }

    fn geodesic(&self, p: Point, q: Point) -> Curve {
        assert!(p != q);
        Curve::Line(Line::new(p, q - p))
    }

    fn point_grid(&self, density: f64) -> Vec<Point> {
        self.point_grid_dense(density, HORIZON_DIST)
    }

    fn project(&self, point: Point) -> Point {
        let normal = self.normal();
        let distance = (point - self.basis).dot(normal);
        point - distance * normal
    }

    fn unsigned_l2_squared_distance_gradient(&self, point: Point) -> Option<Point> {
        let normal = self.normal();
        let distance = (point - self.basis).dot(normal);
        Some(-normal * distance)
    }
}

impl PartialEq for Plane {
    fn eq(&self, other: &Plane) -> bool {
        self.normal() == other.normal()
            && (self.basis - other.basis).dot(self.normal()).abs() < EQ_THRESHOLD
    }
}
