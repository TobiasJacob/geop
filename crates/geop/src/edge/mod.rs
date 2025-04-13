use std::fmt::{Display, Formatter};

use crate::{
    curves::{curve::Curve, CurveLike},
    efloat::EFloat64,
    point::Point,
    transforms::Transform,
};

use crate::contains::edge_point::{edge_point_contains, EdgePointContains};

#[derive(Clone, Debug)]
pub enum EdgeBounds {
    NoBounds,
    Anchored(Point), // Used after "Split Edge (Circle) by Point" step in 3D booleans
    StartEnd(Point, Point),
}

#[derive(Clone, Debug)]
pub struct Edge {
    // pub bounds: EdgeBounds, // TODO: Use this instead of start and end
    pub start: Option<Point>,
    pub end: Option<Point>,
    pub curve: Curve,
}
// Represents an Edge, defined by a curve, and a start and end point.
// It is important to know that the start and end point are not considered a part of the edge.
// E.g. "intersection" between two edges at end points are not considered intersections.
impl Edge {
    pub fn new(start: Option<Point>, end: Option<Point>, curve: Curve) -> Edge {
        assert!(start != end || start.is_none());
        if let Some(start) = start {
            assert!(curve.on_curve(start));
        }
        if let Some(end) = end {
            assert!(curve.on_curve(end));
        }
        Edge { start, end, curve }
    }

    pub fn from_curve(curve: Curve) -> Edge {
        Edge::new(None, None, curve)
    }

    pub fn neg(&self) -> Edge {
        Edge::new(self.end.clone(), self.start.clone(), self.curve.clone())
    }

    pub fn flip(&self) -> Edge {
        Edge::new(self.end.clone(), self.start.clone(), self.curve.neg())
    }

    pub fn transform(&self, transform: Transform) -> Edge {
        Edge::new(
            transform * self.start,
            transform * self.end,
            self.curve.transform(transform),
        )
    }

    pub fn get_midpoint(&self) -> Point {
        self.curve.get_midpoint(self.start, self.end).unwrap()
    }

    pub fn tangent(&self, p: Point) -> Point {
        assert!(edge_point_contains(self, p) != EdgePointContains::Outside);
        self.curve.tangent(p).unwrap().normalize().unwrap()
    }

    pub fn interpolate(&self, t: f64) -> Point {
        assert!(t >= 0.0 && t <= 1.0);
        self.curve.interpolate(self.start, self.end, t).unwrap()
    }

    pub fn length(&self) -> Option<EFloat64> {
        match (self.start, self.end) {
            (Some(start), Some(end)) => Some(self.curve.distance(start, end).unwrap()),
            _ => None,
        }
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        if self.start == other.start && self.end == other.end {
            return self.curve == other.curve;
        }
        if self.start == other.end && self.end == other.start {
            return self.curve == other.curve.neg();
        }
        return false;
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match &self.curve {
            Curve::Line(_line) => write!(f, "Line {:?} - {:?}", self.start, self.end),
            Curve::Circle(circle) => write!(
                f,
                "Circle (at {:?} with normal {:?} and radius {:?}) {:?} - {:?}",
                circle.basis, circle.normal, circle.radius, self.start, self.end
            ),
            Curve::Ellipse(_) => write!(f, "Ellipse {:?} - {:?}", self.start, self.end),
            Curve::Helix(_) => write!(f, "Helix {:?} - {:?}", self.start, self.end),
        }
    }
}
