use crate::{points::point::Point, transforms::Transform, EQ_THRESHOLD, HORIZON_DIST};

use super::{curve::Curve, CurveLike};

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
}

impl CurveLike for Line {
    fn transform(&self, transform: Transform) -> Curve {
        Curve::Line(self.transform(transform))
    }

    fn neg(&self) -> Curve {
        Curve::Line(self.neg())
    }

    fn tangent(&self, _p: Point) -> Point {
        self.direction.clone()
    }

    fn on_curve(&self, p: Point) -> bool {
        let v = p - self.basis;
        let v = v - self.direction * (v.dot(self.direction));
        v.norm() < EQ_THRESHOLD
    }

    fn distance(&self, x: Point, y: Point) -> f64 {
        assert!(self.on_curve(x));
        assert!(self.on_curve(y));
        let v = x - y;
        v.norm()
    }

    fn interpolate(&self, start: Option<Point>, end: Option<Point>, t: f64) -> Point {
        match (start, end) {
            (Some(start), Some(end)) => {
                assert!(self.on_curve(start));
                assert!(self.on_curve(end));
                start + (end - start) * t
            }
            (Some(start), None) => start + self.direction * t * HORIZON_DIST,
            (None, Some(end)) => end - self.direction * t * HORIZON_DIST,
            (None, None) => self.basis + self.direction * (t - 0.5) * 2.0 * HORIZON_DIST,
        }
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
    fn between(&self, m: Point, start: Option<Point>, end: Option<Point>) -> bool {
        assert!(self.on_curve(m));
        match (start, end) {
            (Some(start), Some(end)) => {
                assert!(self.on_curve(start));
                assert!(self.on_curve(end));
                (m - start).dot(self.direction) >= 0.0 && (m - end).dot(self.direction) <= 0.0
            }
            (Some(start), None) => {
                assert!(self.on_curve(start));
                (m - start).dot(self.direction) >= 0.0
            }
            (None, Some(end)) => {
                assert!(self.on_curve(end));
                (m - end).dot(self.direction) <= 0.0
            }
            (None, None) => true,
        }
    }

    fn get_midpoint(&self, start: Option<Point>, end: Option<Point>) -> Point {
        match (start, end) {
            (Some(start), Some(end)) => {
                assert!(self.on_curve(start));
                assert!(self.on_curve(end));
                (start + end) / 2.0
            }
            (Some(start), None) => start + self.direction * HORIZON_DIST,
            (None, Some(end)) => end - self.direction * HORIZON_DIST,
            (None, None) => self.basis,
        }
    }

    fn project(&self, p: Point) -> Point {
        let v = p - self.basis;
        self.basis + self.direction * v.dot(self.direction)
    }

    fn get_bounding_box(
        &self,
        interval_self: Option<Point>,
        midpoint_self: Option<Point>,
    ) -> crate::bounding_box::BoundingBox {
        todo!()
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Line) -> bool {
        self.direction == other.direction && (self.basis - other.basis).is_parallel(self.direction)
    }
}
