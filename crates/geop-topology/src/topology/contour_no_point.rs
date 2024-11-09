use std::fmt::Display;

use geop_geometry::{
    curves::{bounds::Bounds, curve::Curve, CurveLike},
    point::Point,
    transforms::Transform,
};

use crate::topology::edge::Edge;

use super::{contour::ContourTangent, contour_single_point::ContourSinglePoint};

#[derive(Debug, Clone)]
pub struct ContourNoPoint {
    pub curve: Curve,
}

impl ContourNoPoint {
    pub fn new(curve: Curve) -> ContourNoPoint {
        ContourNoPoint { curve }
    }

    pub fn all_points(&self) -> Vec<Point> {
        return vec![];
    }

    pub fn flip(&self) -> ContourNoPoint {
        ContourNoPoint::new(self.curve.neg())
    }

    pub fn transform(&self, transform: Transform) -> ContourNoPoint {
        ContourNoPoint::new(self.curve.transform(transform))
    }

    pub fn tangent(&self, p: Point) -> ContourTangent {
        ContourTangent::OnEdge(self.curve.tangent(p).unwrap())
    }

    pub fn get_midpoint(&self) -> Point {
        self.curve.get_midpoint(None).unwrap()
    }

    // Gets the subcurve between these two points. It is guaranteed that there will be no zero length edges.
    pub fn get_subcurve(&self, start: Point, end: Point) -> Edge {
        assert!(self.curve.on_curve(start));
        assert!(self.curve.on_curve(end));
        assert!(start != end);

        Edge::new(Bounds::new(start, end).unwrap(), self.curve.clone())
    }

    pub fn insert_point(&self, point: Point) -> ContourSinglePoint {
        assert!(self.curve.on_curve(point));
        ContourSinglePoint::new(self.curve.clone(), point)
    }
}

impl Display for ContourNoPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ContourNoPoint {:?}", self.curve)
    }
}
