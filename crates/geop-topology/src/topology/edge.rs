use std::fmt::{Display, Formatter};

use geop_geometry::{
    curves::{bounds::Bounds, curve::Curve, CurveLike},
    efloat::EFloat64,
    point::Point,
    transforms::Transform,
};

use crate::contains::edge_point::{edge_point_contains, EdgePointContains};

#[derive(Clone, Debug)]
pub struct Edge {
    pub bounds: Bounds,
    pub curve: Curve,
}
// Represents an Edge, defined by a curve, and a start and end point.
// It is important to know that the start and end point are not considered a part of the edge.
// E.g. "intersection" between two edges at end points are not considered intersections.
impl Edge {
    pub fn new(bounds: Bounds, curve: Curve) -> Edge {
        assert!(curve.on_curve(bounds.start));
        assert!(curve.on_curve(bounds.end));
        Edge { bounds, curve }
    }

    pub fn flip(&self) -> Edge {
        Edge::new(self.bounds.flip(), self.curve.neg())
    }

    pub fn transform(&self, transform: Transform) -> Edge {
        Edge::new(
            transform * self.bounds.clone(),
            self.curve.transform(transform),
        )
    }

    pub fn get_midpoint(&self) -> Point {
        self.curve.get_midpoint(Some(&self.bounds)).unwrap()
    }

    pub fn tangent(&self, p: Point) -> Point {
        assert!(edge_point_contains(self, p) != EdgePointContains::Outside);
        self.curve.tangent(p).unwrap().normalize().unwrap()
    }

    pub fn interpolate(&self, t: f64) -> Point {
        assert!(t >= 0.0 && t <= 1.0);
        self.curve
            .interpolate(Some(self.bounds.start), Some(self.bounds.end), t)
            .unwrap()
    }

    pub fn length(&self) -> Option<EFloat64> {
        Some(
            self.curve
                .distance(self.bounds.start, self.bounds.end)
                .unwrap(),
        )
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        if self.bounds == other.bounds {
            return self.curve == other.curve;
        }
        if self.bounds == other.bounds.flip() {
            return self.curve == other.curve.neg();
        }
        return false;
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match &self.curve {
            Curve::Line(_line) => write!(f, "Line {:?} - {:?}", self.bounds.start, self.bounds.end),
            Curve::Circle(circle) => write!(
                f,
                "Circle (at {:?} with normal {:?} and radius {:?}) {:?} - {:?}",
                circle.basis, circle.normal, circle.radius, self.bounds.start, self.bounds.end
            ),
            Curve::Ellipse(_) => {
                write!(f, "Ellipse {:?} - {:?}", self.bounds.start, self.bounds.end)
            }
            Curve::Helix(_) => write!(f, "Helix {:?} - {:?}", self.bounds.start, self.bounds.end),
        }
    }
}
