use std::{
    fmt::{Display, Formatter},
    rc::Rc,
};

use geop_geometry::{
    curves::{curve::Curve, line::Line},
    points::point::Point,
    transforms::Transform,
};

use crate::topology::contains::edge_point::{edge_point_contains, EdgePointContains};

#[derive(Clone, Debug)]
pub struct Edge {
    pub start: Rc<Point>,
    pub end: Rc<Point>,
    pub curve: Rc<Curve>,
}
// Represents an Edge, defined by a curve, and a start and end point.
// It is important to know that the start and end point are not considered a part of the edge.
// E.g. "intersection" between two edges at end points are not considered intersections.
impl Edge {
    pub fn new(start: Rc<Point>, end: Rc<Point>, curve: Rc<Curve>) -> Edge {
        assert!(start != end); // Prevent zero length edges
        assert!(curve.on_manifold(*start));
        assert!(curve.on_manifold(*end));
        Edge { start, end, curve }
    }

    pub fn new_line(start: Rc<Point>, end: Rc<Point>) -> Edge {
        let l = Line::new(*start, *end - *start);
        Edge::new(start, end, Rc::new(Curve::Line(l)))
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
        assert!(self.curve.on_manifold(a));
        assert!(self.curve.on_manifold(b));
        self.curve.get_midpoint(a, b)
    }

    pub fn tangent(&self, p: Point) -> Point {
        assert!(edge_point_contains(self, p) != EdgePointContains::Outside);
        self.curve.tangent(p).normalize()
    }

    pub fn interpolate(&self, t: f64) -> Point {
        assert!(t >= 0.0 && t <= 1.0);
        self.curve.interpolate(*self.start, *self.end, t)
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
            Curve::Line(_line) => write!(f, "Line {:?} - {:?}", self.start, self.end),
            Curve::Circle(_circle) => write!(f, "Circle {:?} - {:?}", self.start, self.end),
            Curve::Ellipse(_ellipse) => write!(f, "Ellipse {:?} - {:?}", self.start, self.end),
        }
    }
}
