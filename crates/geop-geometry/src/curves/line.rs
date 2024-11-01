use crate::{point::Point, transforms::Transform, EQ_THRESHOLD, HORIZON_DIST};

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
            direction: direction.normalize().unwrap(),
        }
    }

    pub fn transform(&self, transform: Transform) -> Self {
        let basis = transform * self.basis;
        let direction = transform * (self.direction + self.basis) - basis;
        Line::new(basis, direction.normalize().unwrap())
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
            (None, Some(end)) => end - self.direction * (1.0 - t) * HORIZON_DIST,
            (None, None) => self.basis + self.direction * (t - 0.5) * 2.0 * HORIZON_DIST,
        }
    }

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
        _interval_self: Option<Point>,
        _midpoint_self: Option<Point>,
    ) -> crate::bounding_box::BoundingBox {
        todo!()
    }

    fn sort(&self, points: Vec<Option<Point>>) -> Vec<Option<Point>> {
        let mut points = points;
        points.sort_unstable_by(|a, b| {
            if let Some(a) = a {
                if let Some(b) = b {
                    let v = *a - *b;
                    v.dot(self.direction).partial_cmp(&0.0).unwrap()
                } else {
                    std::cmp::Ordering::Less
                }
            } else {
                if let Some(_b) = b {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            }
        });
        points
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Line) -> bool {
        self.direction == other.direction && (self.basis - other.basis).is_parallel(self.direction)
    }
}
