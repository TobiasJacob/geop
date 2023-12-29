

use crate::{points::point::Point, transforms::Transform, EQ_THRESHOLD};

#[derive(Debug, Clone)]
pub struct Line {
    pub basis: Point,
    pub direction: Point,
}

impl Line {
    pub fn new(basis: Point, direction: Point) -> Line {
        Line {
            basis,
            direction: direction.normalize(),
        }
    }

    pub fn transform(&self, transform: Transform) -> Self {
        let basis = transform * self.basis;
        let direction = transform * (self.direction + self.basis) - basis;
        Line::new(basis, direction.normalize())
    }

    pub fn neg(&self) -> Line {
        Line::new(self.basis, -self.direction)
    }

    pub fn tangent(&self, _p: Point) -> Point {
        self.direction.clone()
    }

    pub fn on_manifold(&self, p: Point) -> bool {
        let v = p - self.basis;
        let v = v - self.direction * (v.dot(self.direction));
        v.norm() < EQ_THRESHOLD
    }

    pub fn interpolate(&self, start: Point, end: Point, t: f64) -> Point {
        assert!(self.on_manifold(start));
        assert!(self.on_manifold(end));
        start + (end - start) * t
    }

    // fn metric(&self, x: Point, u: TangentParameter, v: TangentParameter) -> f64 {
    //     u.0 * v.0
    // }

    // fn distance(&self, p1: Point, p2: Point) -> f64 {
    //     return (p2 - p1).norm();
    // }

    // fn exp(&self, x: Point, u: TangentParameter) -> Point {
    //     assert!(self.on_manifold(x), "x is not on the manifold");
    //     x + self.direction * u.0
    // }
    // // Log of y at base x. Z coordinate is set to 0.
    // fn log(&self, x: Point, y: Point) -> TangentParameter {
    //     assert!(self.on_manifold(x), "x is not on the manifold");
    //     assert!(self.on_manifold(y), "y is not on the manifold");
    //     let v = y - x;
    //     TangentParameter(self.direction.dot(v))
    // }
    // // Parallel transport of v from x to y.
    // fn parallel_transport(&self, v: TangentParameter, x: Point, y: Point) -> TangentParameter {
    //     assert!(self.on_manifold(x), "x is not on the manifold");
    //     assert!(self.on_manifold(y), "y is not on the manifold");
    //     v
    // }

    // Checks if m is between x and y. m==x and m==y are true.
    pub fn between(&self, m: Point, start: Point, end: Point) -> bool {
        assert!(self.on_manifold(m));
        assert!(self.on_manifold(start));
        assert!(self.on_manifold(end));
        (m - start).dot(self.direction) >= 0.0 && (m - end).dot(self.direction) <= 0.0
    }

    pub fn get_midpoint(&self, start: Point, end: Point) -> Point {
        assert!(self.on_manifold(start));
        assert!(self.on_manifold(end));
        (start + end) / 2.0
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Line) -> bool {
        self.basis == other.basis && self.direction == other.direction
    }
}
