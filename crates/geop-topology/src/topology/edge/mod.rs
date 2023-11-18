pub mod edge_curve;

use std::{
    fmt::{Display, Formatter},
    rc::Rc,
};

use geop_geometry::{
    points::point::Point,
    EQ_THRESHOLD, transforms::Transform, curves::curve::Curve,
};

use crate::{PROJECTION_THRESHOLD, topology::contains::edge_point::{EdgeContains, edge_contains_point}};

use self::edge_curve::EdgeCurve;

#[derive(Clone, Debug)]
pub struct Edge {
    pub start: Rc<Point>,
    pub end: Rc<Point>,
    pub curve: Rc<EdgeCurve>,
}
// Represents an Edge, defined by a curve, and a start and end point.
// It is important to know that the start and end point are not considered a part of the edge.
// E.g. "intersection" between two edges at end points are not considered intersections.
impl Edge {
    pub fn new(start: Rc<Point>, end: Rc<Point>, curve: Rc<EdgeCurve>) -> Edge {
        assert!(start != end); // Prevent zero length edges
        assert!(curve.curve().on_manifold(*start));
        assert!(curve.curve().on_manifold(*end));
        Edge {
            start,
            end,
            curve,
        }
    }

    pub fn neg(&self) -> Edge {
        Edge::new(
            self.end.clone(),
            self.start.clone(),
            Rc::new(self.curve.neg()),
        )
    }

    pub fn transform(&self, transform: Transform) -> Edge {
        Edge::new(
            Rc::new(transform * *self.start),
            Rc::new(transform * *self.end),
            Rc::new(self.curve.transform(transform)),
        )
    }

    pub fn get_midpoint(&self, a: Point, b: Point) -> Point {
        assert!(self.curve.curve().on_manifold(a));
        assert!(self.curve.curve().on_manifold(b));
        self.curve.curve().get_midpoint(a, b)
    }

    pub fn tangent(&self, p: Point) -> Point {
        assert!(edge_contains_point(self, p) != EdgeContains::Outside);
        self.curve.curve().tangent(p).normalize()
    }

    pub fn point_at(&self, t: f64) -> Point {
        assert!(t >= 0.0 && t <= 1.0);
        self.curve.curve().point_at(self.start, self.end, t)
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        (Rc::ptr_eq(&self.curve, &other.curve) || self.curve == other.curve)
            && self.start == other.start
            && self.end == other.end
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self.curve.as_ref() {
            EdgeCurve::Line(_line) => write!(f, "Line {:?} - {:?}", self.start, self.end),
            EdgeCurve::Circle(_circle) => write!(f, "Circle {:?} - {:?}", self.start, self.end),
            EdgeCurve::Ellipse(_ellipse) => write!(f, "Ellipse {:?} - {:?}", self.start, self.end),
        }
    }
}
