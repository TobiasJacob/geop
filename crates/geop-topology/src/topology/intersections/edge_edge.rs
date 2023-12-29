use std::{rc::Rc, f32::consts::E};

use geop_geometry::{
    curve_curve_intersection::{
        curve_curve::{curve_curve_intersection, CurveCurveIntersection}
    },
    points::point::Point, curves::curve::{Curve},
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


pub enum CurveIntersection {
    None,
    Point(Point),
    Interval(Point, Point),
    IntervalAndPoint(Point, Point, Point), // Migh happen, e.g. if two half circles intersect at both ends.
    DualInterval(Point, Point, Point, Point), 
}
// Intersect between start1/2 and end1/2. Returns None if there is no intersection.
// Keep in mind that all curves are treated as infinite lines, such that start after end means that the line starts, goes to +infinity, goes to -infinty and then ends.
pub fn intersect_same_curve(curve: &Curve, start1: Point, end1: Point, start2: Point, end2: Point) -> CurveIntersection {
    print!("intersect: {:?}, {:?}, {:?}, {:?}\n", start1, end1, start2, end2);
    assert!(start1 != end1);
    assert!(start2 != end2);
    let mut solutions = vec![];
    for (s, e) in [(&start1, &end1), (&start2, &end2), (&start1, &end2), (&start2, &end1)] {
        if curve.between(*s, start1, end1) && curve.between(*e, start1, end1) && curve.between(*s, start2, end2) && curve.between(*e, start2, end2) {
            println!("intersect_done: {:?}, {:?}\n", s, e);
            let mut already_in_solution = false;
            for (s2, e2) in solutions.iter() {
                if s == s2 && e == e2 {
                    already_in_solution = true;
                    break;
                }
            }
            if !already_in_solution {
                solutions.push((s.clone(), e.clone()));
            }
        }
    }
    match solutions.len() {
        0 => {
            return CurveIntersection::None;
        },
        1 => {
            let (s, e) = solutions[0].clone();
            if s == e {
                return CurveIntersection::Point(s.clone());
            } else {
                return CurveIntersection::Interval(s.clone(), e.clone());
            }
        },
        2 => {
            let (s1, e1) = solutions[0].clone();
            let (s2, e2) = solutions[1].clone();
            if s1 == s2 && e1 == e2 {
                panic!("Should not happen");
            } else if s1 == e1 {
                return CurveIntersection::IntervalAndPoint(s2.clone(), e2.clone(), s1.clone());
            } else if s2 == e2 {
                return CurveIntersection::IntervalAndPoint(s1.clone(), e1.clone(), s2.clone());
            } else {
                return CurveIntersection::DualInterval(s1.clone(), e1.clone(), s2.clone(), e2.clone());
            }
        },
        _ => {
            panic!("More than two intersections. Should not happen.");
        }
    }
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
                    intersect_same_curve(&edge_self.curve, *edge_self.start, *edge_self.end, *edge_other.start, *edge_other.end)
                },
                false => {
                    intersect_same_curve(&edge_self.curve, *edge_self.start, *edge_self.end, *edge_other.end, *edge_other.start)
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
