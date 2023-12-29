use std::{rc::Rc, f32::consts::E};

use geop_geometry::{
    curve_curve_intersection::{
        curve_curve::{curve_curve_intersection, CurveCurveIntersection}
    },
    points::point::Point, curves::curve::CurveIntersection,
};

use crate::topology::{edge::{Edge}, contains::edge_point::{EdgeContains, edge_contains_point}};


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
    match curve_curve_intersection(&*edge_self.curve, &*edge_other.curve) {
        CurveCurveIntersection::Curve(_) => {
            // Now that we project the start and end point of edge_other on edge_self, we have to take care of the case that the edges are facing in opposite directions.
            let same_dir = edge_self.curve.tangent(*edge_self.start).dot(edge_other.curve.tangent(*edge_self.start)) > 0.0;
            println!("edge_self.curve = {:?}, edge_other.curve = {:?}, same_dir = {:?}", edge_self.curve, edge_other.curve, same_dir);
            let intersection = match same_dir { 
                true => {
                    edge_self.curve.intersect(*edge_self.start, *edge_self.end, *edge_other.start, *edge_other.end)
                },
                false => {
                    edge_self.curve.intersect(*edge_self.start, *edge_self.end, *edge_other.end, *edge_other.start)
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
                },
                CurveIntersection::IntervalAndPoint(s, e, p) => {
                    assert!(edge_contains_point(edge_self, p.clone()) != EdgeContains::Outside, "edge_self: {:}, edge_other: {:}, point: {:?}", edge_self, edge_other, p);
                    assert!(edge_contains_point(edge_other, p.clone()) != EdgeContains::Outside, "edge_self: {:}, edge_other: {:}, point: {:?}", edge_self, edge_other, p);
                    vec![EdgeEdgeIntersection::Edge(Edge::new(Rc::new(s), Rc::new(e.clone()), edge_self.curve.clone())), EdgeEdgeIntersection::Point(Rc::new(p))]
                        // .iter()
                        // .filter(|p| {edge_contains_point(edge_self, **p) == EdgeContains::Inside && edge_contains_point(edge_other, **p) == EdgeContains::Inside})
                        // .map(|p| {EdgeEdgeIntersection::Point(Rc::new(*p))})
                        // .collect()
                },
                CurveIntersection::DualInterval(s1, e1, s2, e2) => {
                    assert!(edge_contains_point(edge_self, s1.clone()) != EdgeContains::Outside, "edge_self: {:}, edge_other: {:}, point: {:?}", edge_self, edge_other, s1);
                    assert!(edge_contains_point(edge_self, e1.clone()) != EdgeContains::Outside, "edge_self: {:}, edge_other: {:}, point: {:?}", edge_self, edge_other, e1);
                    assert!(edge_contains_point(edge_self, s2.clone()) != EdgeContains::Outside, "edge_self: {:}, edge_other: {:}, point: {:?}", edge_self, edge_other, s2);
                    assert!(edge_contains_point(edge_self, e2.clone()) != EdgeContains::Outside, "edge_self: {:}, edge_other: {:}, point: {:?}", edge_self, edge_other, e2);
                    assert!(edge_contains_point(edge_other, s1.clone()) != EdgeContains::Outside, "edge_self: {:}, edge_other: {:}, point: {:?}", edge_self, edge_other, s1);
                    assert!(edge_contains_point(edge_other, e1.clone()) != EdgeContains::Outside, "edge_self: {:}, edge_other: {:}, point: {:?}", edge_self, edge_other, e1);
                    assert!(edge_contains_point(edge_other, s2.clone()) != EdgeContains::Outside, "edge_self: {:}, edge_other: {:}, point: {:?}", edge_self, edge_other, s2);
                    assert!(edge_contains_point(edge_other, e2.clone()) != EdgeContains::Outside, "edge_self: {:}, edge_other: {:}, point: {:?}", edge_self, edge_other, e2);
                    vec![EdgeEdgeIntersection::Edge(Edge::new(Rc::new(s1), Rc::new(e1), edge_self.curve.clone())), EdgeEdgeIntersection::Edge(Edge::new(Rc::new(s2), Rc::new(e2), edge_self.curve.clone()))]
                        // .iter()
                        // .filter(|p| {edge_contains_point(edge_self, **p) == EdgeContains::Inside && edge_contains_point(edge_other, **p) == EdgeContains::Inside})
                        // .map(|p| {EdgeEdgeIntersection::Point(Rc::new(*p))})
                        // .collect()
                }
            }
        },
        CurveCurveIntersection::Points(points) => {
            points.iter().filter(|p| {edge_contains_point(edge_self, **p) != EdgeContains::Outside && edge_contains_point(edge_other, **p) != EdgeContains::Outside}).map(|p| EdgeEdgeIntersection::Point(Rc::new(*p))).collect()
        },
        CurveCurveIntersection::None => {
            vec![]
        },
    }
}
