use std::fmt::Display;

use geop_geometry::{
    curves::{bounds::Bounds, curve::Curve, CurveLike},
    point::Point,
    transforms::Transform,
};

use crate::topology::edge::Edge;

use super::ContourTangent;

#[derive(Debug, Clone)]
pub struct CurveContour {
    pub curve: Curve,
}

impl CurveContour {
    pub fn new(curve: Curve) -> CurveContour {
        CurveContour { curve }
    }

    pub fn all_points(&self) -> Vec<Point> {
        return vec![];
    }

    pub fn flip(&self) -> CurveContour {
        CurveContour::new(self.curve.neg())
    }

    pub fn transform(&self, transform: Transform) -> CurveContour {
        CurveContour::new(self.curve.transform(transform))
    }

    pub fn tangent(&self, p: Point) -> ContourTangent {
        ContourTangent::OnEdge(self.curve.tangent(p).unwrap())
    }

    pub fn get_midpoint(&self) -> Point {
        self.curve.get_midpoint(None).unwrap()
    }

    // Gets the subcurve between these two points. It is guaranteed that there will be no zero length edges.
    pub fn get_subcurve(&self, start: Point, end: Point) -> Vec<Edge> {
        assert!(start != end);

        vec![Edge::new(
            Bounds::new(start, end).unwrap(),
            self.curve.clone(),
        )]
    }

    pub fn get_subcurve_single_point(&self, point: Point) -> Vec<Edge> {
        vec![Edge::new(
            Bounds::new(point, point).unwrap(),
            self.curve.clone(),
        )]
    }
}

impl Display for CurveContour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CurveContour {:?}", self.curve)
    }
}
