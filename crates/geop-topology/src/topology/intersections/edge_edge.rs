use std::rc::Rc;

use geop_geometry::{
    curve_curve_intersection::{
        circle_circle::{circle_circle_intersection, CircleCircleIntersection},
        line_line::{line_line_intersection, LineLineIntersection},
    },
    points::point::Point, curves::curve::CurveIntersection,
};

use crate::topology::{edge::{Edge, edge_curve::{EdgeCurve, edge_curve_edge_curve_intersect, EdgeCurveIntersection}}, contains::edge_point::{EdgeContains, edge_contains_point}};


#[derive(Clone, Debug)]
pub enum EdgeEdgeIntersection {
    Point(Rc<Point>),
    Edge(Edge),
}

pub fn edge_edge_same_curve_intersection(edge_self: &Edge, other: &Edge) -> Edge {
    assert!(edge_self.curve == other.curve);
    todo!("Split edge_edge_intersections into two functions")
}

pub fn edge_edge_different_curve_intersection(edge_self: &Edge, other: &Edge) -> Vec<Point> {
    assert!(edge_self.curve != other.curve);
    todo!("Split edge_edge_intersections into two functions")
}

// All intersections where it crosses other edge. The end points are not included. The list is sorted from start to end.
pub fn edge_edge_intersections(edge_self: &Edge, edge_other: &Edge) -> Vec<EdgeEdgeIntersection> {
    match edge_curve_edge_curve_intersect(&*edge_self.curve, &*edge_other.curve) {
        EdgeCurveIntersection::EdgeCurve(curve) => {
            match curve.curve().intersect(*edge_self.start, *edge_self.end, *edge_other.start, *edge_other.end) {
                CurveIntersection::None => {
                    vec![]
                },
                CurveIntersection::Point(p) => {
                    vec![EdgeEdgeIntersection::Point(Rc::new(p))]
                },
                CurveIntersection::Points(p1, p2) => {
                    vec![EdgeEdgeIntersection::Point(Rc::new(p1)), EdgeEdgeIntersection::Point(Rc::new(p2))]
                }
            }
        },
        EdgeCurveIntersection::Points(points) => {
            points.iter().filter(|p| {edge_contains_point(edge_self, **p) == EdgeContains::Inside && edge_contains_point(edge_other, **p) == EdgeContains::Inside}).map(|p| EdgeEdgeIntersection::Point(Rc::new(*p))).collect()
        },
        EdgeCurveIntersection::None => {
            vec![]
        },
    }
}
