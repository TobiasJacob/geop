use std::{rc::Rc, f32::consts::E};

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
        EdgeCurveIntersection::EdgeCurve(_) => {
            // Now that we project the start and end point of edge_other on edge_self, we have to take care of the case that the edges are facing in opposite directions.
            let same_dir = edge_self.curve.curve().tangent(*edge_self.start).dot(edge_other.curve.curve().tangent(*edge_self.start)) > 0.0;
            let intersection = match same_dir { 
                true => {
                    edge_self.curve.curve().intersect(*edge_self.start, *edge_self.end, *edge_other.start, *edge_other.end)
                },
                false => {
                    edge_self.curve.curve().intersect(*edge_self.start, *edge_self.end, *edge_other.end, *edge_other.start)
                }
            };
            match intersection {
                CurveIntersection::None => {
                    vec![]
                },
                CurveIntersection::Point(p) => {
                    assert!(edge_contains_point(edge_self, p.clone()) != EdgeContains::Outside, "edge_self: {:}, edge_other: {:}, point: {:?}", edge_self, edge_other, p);
                    assert!(edge_contains_point(edge_other, p.clone()) != EdgeContains::Outside, "edge_self: {:}, edge_other: {:}, point: {:?}", edge_self, edge_other, p);
                    vec![EdgeEdgeIntersection::Point(Rc::new(p))]
                        // .iter()
                        // .filter(|p| {edge_contains_point(edge_self, **p) == EdgeContains::Inside && edge_contains_point(edge_other, **p) == EdgeContains::Inside})
                        // .map(|p| {EdgeEdgeIntersection::Point(Rc::new(*p))})
                        // .collect()
                },
                CurveIntersection::Interval(p1, p2) => {
                    assert!(edge_contains_point(edge_self, p1.clone()) != EdgeContains::Outside, "edge_self: {:}, edge_other: {:}, point: {:?}", edge_self, edge_other, p1);
                    assert!(edge_contains_point(edge_self, p2.clone()) != EdgeContains::Outside, "edge_self: {:}, edge_other: {:}, point: {:?}", edge_self, edge_other, p2);
                    assert!(edge_contains_point(edge_other, p1.clone()) != EdgeContains::Outside, "edge_self: {:}, edge_other: {:}, point: {:?}", edge_self, edge_other, p1);
                    assert!(edge_contains_point(edge_other, p2.clone()) != EdgeContains::Outside, "edge_self: {:}, edge_other: {:}, point: {:?}", edge_self, edge_other, p2);
                    vec![EdgeEdgeIntersection::Edge(Edge::new(Rc::new(p1), Rc::new(p2), edge_self.curve.clone()))]
                        // .iter()
                        // .filter(|p| {edge_contains_point(edge_self, **p) == EdgeContains::Inside && edge_contains_point(edge_other, **p) == EdgeContains::Inside})
                        // .map(|p| {EdgeEdgeIntersection::Point(Rc::new(*p))})
                        // .collect()
                }
            }
        },
        EdgeCurveIntersection::Points(points) => {
            points.iter().filter(|p| {edge_contains_point(edge_self, **p) != EdgeContains::Outside && edge_contains_point(edge_other, **p) != EdgeContains::Outside}).map(|p| EdgeEdgeIntersection::Point(Rc::new(*p))).collect()
        },
        EdgeCurveIntersection::None => {
            vec![]
        },
    }
}
