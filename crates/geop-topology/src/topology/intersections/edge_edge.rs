use std::rc::Rc;

use geop_geometry::{
    curve_curve_intersection::curve_curve::{curve_curve_intersection, CurveCurveIntersection},
    points::point::Point,
};

use crate::topology::{
    contains::edge_point::{edge_point_contains, EdgePointContains},
    edge::Edge, remesh::edge::{edge_split, EdgeRemesh, edge_remesh},
};

// Intersect between start1/2 and end1/2. Returns None if there is no intersection.
// Keep in mind that all curves are treated as infinite lines, such that start after end means that the line starts, goes to +infinity, goes to -infinty and then ends.
// Vec is used bc. e.g. two half circles might have to distinct intersections at both ends.
pub fn edge_edge_same_curve_intersection(
    edge_self: &Edge,
    edge_other: &Edge,
) -> Vec<Edge> {
    let same_dir = edge_self
        .curve
        .tangent(edge_self.start)
        .dot(edge_other.curve.tangent(edge_self.start))
        > 0.0;
    let edge_other = if same_dir {
        edge_other.clone()
    } else {
        edge_other.clone().flip()
    };
    assert!(edge_self.curve == edge_other.curve);

    println!("Intersecting edge {:?} and {:?}", edge_self, edge_other);
    let intervals: Vec<EdgeRemesh> = edge_split(edge_self, &edge_other).drain(..).filter(|int| {
        match int {
            EdgeRemesh::AinB(_, _) => true,
            EdgeRemesh::AoutB(_, _) => false,
            EdgeRemesh::BinA(_, _) => false,
            EdgeRemesh::BoutA(_, _) => false,
        }
    }).collect();
    println!("Intervals: {:?}", intervals);

    return edge_remesh(&edge_self.curve, intervals)
}

pub enum EdgeEdgeIntersection {
    None,
    Points(Vec<Point>),
    Edges(Vec<Edge>),
}

// All intersections where it crosses other edge. The end points are included.
pub fn edge_edge_intersection(edge_self: &Edge, edge_other: &Edge) -> EdgeEdgeIntersection {
    match curve_curve_intersection(&edge_self.curve, &edge_other.curve) {
        CurveCurveIntersection::Curve(_) => {
            EdgeEdgeIntersection::Edges(edge_edge_same_curve_intersection(edge_self, edge_other))
        }
        CurveCurveIntersection::Points(mut points) => {
            let intersections = points
            .drain(..)
            .filter(|p| 
                edge_point_contains(edge_self, *p) != EdgePointContains::Outside
                    && edge_point_contains(edge_other, *p) != EdgePointContains::Outside
            )
            .collect();
            EdgeEdgeIntersection::Points(intersections)
        }
        CurveCurveIntersection::None => {
            EdgeEdgeIntersection::None
        }
    }
}