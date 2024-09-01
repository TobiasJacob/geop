use geop_geometry::{
    curve_curve_intersection::curve_curve::{curve_curve_intersection, CurveCurveIntersection},
    curves::CurveLike,
    points::point::Point,
};

use crate::remesh::edge::{edge_remesh, edge_split, EdgeRemesh};
use geop_topology::{
    contains::edge_point::{edge_point_contains, EdgePointContains},
    topology::edge::Edge,
};
// Intersect between start1/2 and end1/2. Returns None if there is no intersection.
// Keep in mind that all curves are treated as infinite lines, such that start after end means that the line starts, goes to +infinity, goes to -infinty and then ends.
// Vec is used bc. e.g. two half circles might have to distinct intersections at both ends.

pub enum EdgeEdgeIntersection {
    None,
    Points(Vec<Point>),
    Edges(Vec<Edge>),
}

pub fn edge_edge_intersection(edge_self: &Edge, edge_other: &Edge) -> EdgeEdgeIntersection {
    match curve_curve_intersection(&edge_self.curve, &edge_other.curve) {
        CurveCurveIntersection::Curve(_) => {
            let same_dir = edge_self
                .curve
                .tangent(edge_self.get_midpoint())
                .dot(edge_other.curve.tangent(edge_self.get_midpoint()))
                > 0.0;
            let edge_other = if same_dir {
                edge_other.clone()
            } else {
                edge_other.clone().flip()
            };

            let intervals: Vec<EdgeRemesh> = edge_split(edge_self, &edge_other)
                .drain(..)
                .filter(|int| match int {
                    EdgeRemesh::AinB(_) => true,
                    EdgeRemesh::AoutB(_) => false,
                    EdgeRemesh::BinA(_) => false,
                    EdgeRemesh::BoutA(_) => false,
                })
                .collect();

            EdgeEdgeIntersection::Edges(edge_remesh(&edge_self.curve, intervals))
        }
        CurveCurveIntersection::FinitePoints(mut points) => {
            let intersections = points
                .drain(..)
                .filter(|p| {
                    edge_point_contains(edge_self, *p) != EdgePointContains::Outside
                        && edge_point_contains(edge_other, *p) != EdgePointContains::Outside
                })
                .collect();
            EdgeEdgeIntersection::Points(intersections)
        }
        CurveCurveIntersection::InfiniteDiscretePoints(point_array) => {
            let mut min_i = None;
            let mut max_i = None;
            if let Some(self_start) = edge_self.start {
                let i = (self_start - point_array.basis).dot(point_array.extend_dir);
                min_i = Some(i);
                max_i = Some(i);
            }
            if let Some(self_end) = edge_self.end {
                let i = (self_end - point_array.basis).dot(point_array.extend_dir);
                if min_i.is_none() || i < min_i.unwrap() {
                    min_i = Some(i);
                }
                if max_i.is_none() || i > max_i.unwrap() {
                    max_i = Some(i);
                }
            }
            if let Some(other_start) = edge_other.start {
                let i = (other_start - point_array.basis).dot(point_array.extend_dir);
                if min_i.is_none() || i < min_i.unwrap() {
                    min_i = Some(i);
                }
                if max_i.is_none() || i > max_i.unwrap() {
                    max_i = Some(i);
                }
            }
            if let Some(other_end) = edge_other.end {
                let i = (other_end - point_array.basis).dot(point_array.extend_dir);
                if min_i.is_none() || i < min_i.unwrap() {
                    min_i = Some(i);
                }
                if max_i.is_none() || i > max_i.unwrap() {
                    max_i = Some(i);
                }
            }

            match (min_i, max_i) {
                (Some(min_i), Some(max_i)) => {
                    let mut intersections = Vec::new();
                    for i in (min_i.ceil() as usize)..(max_i.floor() as usize) {
                        intersections.push(point_array.basis + i as f64 * point_array.extend_dir);
                    }
                    EdgeEdgeIntersection::Points(intersections)
                }
                _ => todo!("This case should not happen with the current features, but it could happen if geop is extended to support more complex curves"),
            }
        }
        CurveCurveIntersection::None => EdgeEdgeIntersection::None,
    }
}
