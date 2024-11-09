use std::fmt::Display;

use geop_geometry::{
    curves::{bounds::Bounds, curve::Curve, CurveLike},
    point::Point,
    transforms::Transform,
};

use crate::topology::edge::Edge;

use super::{contour::ContourTangent, contour_multi_point::ContourMultiPoint};

#[derive(Debug, Clone)]
pub struct ContourSinglePoint {
    pub curve: Curve,
    pub point: Point,
}

impl ContourSinglePoint {
    pub fn new(curve: Curve, point: Point) -> ContourSinglePoint {
        ContourSinglePoint { curve, point }
    }

    pub fn all_points(&self) -> Vec<Point> {
        return vec![];
    }

    pub fn flip(&self) -> ContourSinglePoint {
        ContourSinglePoint::new(self.curve.neg(), self.point)
    }

    pub fn transform(&self, transform: Transform) -> ContourSinglePoint {
        ContourSinglePoint::new(self.curve.transform(transform), transform * self.point)
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

    pub fn insert_point(&self, point: Point) -> ContourMultiPoint {
        ContourMultiPoint::new(vec![
            Edge::new(Bounds::new(self.point, point).unwrap(), self.curve.clone()),
            Edge::new(Bounds::new(point, self.point).unwrap(), self.curve.clone()),
        ])
    }
}

impl Display for ContourSinglePoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ContourSinglePoint {:?} {:?}", self.curve, self.point)
    }
}
