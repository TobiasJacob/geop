use std::rc::Rc;

use geop_geometry::{
    curve_curve_intersection::curve_curve::{curve_curve_intersection, CurveCurveIntersection},
    curves::curve::Curve,
    points::point::Point,
};

use crate::topology::{
    contains::edge_point::{edge_point_contains, EdgePointContains},
    edge::Edge,
};

#[derive(Clone, Debug)]
pub enum EdgeEdgeIntersection {
    Point(Rc<Point>),
    Edge(Edge),
}

// Intersect between start1/2 and end1/2. Returns None if there is no intersection.
// Keep in mind that all curves are treated as infinite lines, such that start after end means that the line starts, goes to +infinity, goes to -infinty and then ends.
// Vec is used bc. e.g. two half circles might have to distinct intersections at both ends.
pub fn edge_edge_same_curve_intersection(edge_self: &Edge, edge_other: &Edge) -> Vec<EdgeEdgeIntersection> {
    let same_dir = edge_self
    .curve
    .tangent(*edge_self.start)
    .dot(edge_other.curve.tangent(*edge_self.start))
    > 0.0;
    if same_dir {
        assert!(edge_self.curve.tangent(*edge_self.start).dot(edge_other.curve.tangent(*edge_self.start)) > 0.0);
        assert!(edge_self.curve.tangent(*edge_self.end).dot(edge_other.curve.tangent(*edge_self.end)) > 0.0);
    } else {
        assert!(edge_self.curve.tangent(*edge_self.start).dot(edge_other.curve.tangent(*edge_self.start)) < 0.0);
        assert!(edge_self.curve.tangent(*edge_self.end).dot(edge_other.curve.tangent(*edge_self.end)) < 0.0);
    }

    let start1 = edge_self.start.clone();
    let end1 = edge_self.end.clone();
    let start2 = if same_dir {
        edge_other.start.clone()
    } else {
        edge_other.end.clone()
    };
    let end2 = if same_dir {
        edge_other.end.clone()
    } else {
        edge_other.start.clone()
    };
    let curve = edge_self.curve.clone();

    assert!(start1 != end1);
    assert!(start2 != end2);
    let mut solutions = vec![];
    for (s, e) in [
        (&start1, &end1),
        (&start2, &end2),
        (&start1, &end2),
        (&start2, &end1),
    ] {
        if curve.between(**s, *start1, *end1)
            && curve.between(**e, *start1, *end1)
            && curve.between(**s, *start2, *end2)
            && curve.between(**e, *start2, *end2)
        {
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
            return vec![];
        }
        1 => {
            let (s, e) = solutions[0].clone();
            if s == e {
                return vec![EdgeEdgeIntersection::Point(s.clone())];
            } else {
                return vec![EdgeEdgeIntersection::Edge(Edge::new(s.clone(), e.clone(), edge_self.curve.clone()))];
            }
        }
        2 => {
            let (s1, e1) = solutions[0].clone();
            let (s2, e2) = solutions[1].clone();
            if s1 == s2 && e1 == e2 {
                panic!("Should not happen");
            } else if s1 == e1 {
                // return CurveIntersection::IntervalAndPoint(s2.clone(), e2.clone(), s1.clone());
                return vec![EdgeEdgeIntersection::Point(s1.clone()), EdgeEdgeIntersection::Edge(Edge::new(s2.clone(), e2.clone(), edge_self.curve.clone()))];
            } else if s2 == e2 {
                // return CurveIntersection::IntervalAndPoint(s1.clone(), e1.clone(), s2.clone());
                return vec![EdgeEdgeIntersection::Point(s2.clone()), EdgeEdgeIntersection::Edge(Edge::new(s1.clone(), e1.clone(), edge_self.curve.clone()))];
            } else {
                // return CurveIntersection::DualInterval(
                //     s1.clone(),
                //     e1.clone(),
                //     s2.clone(),
                //     e2.clone(),
                // );
                return vec![EdgeEdgeIntersection::Edge(Edge::new(s1.clone(), e1.clone(), edge_self.curve.clone())), EdgeEdgeIntersection::Edge(Edge::new(s2.clone(), e2.clone(), edge_self.curve.clone()))];
            }
        }
        _ => {
            panic!("More than two intersections. Should not happen.");
        }
    }
}

// All intersections where it crosses other edge. The end points are not included. The list is sorted from start to end.
pub fn edge_edge_intersections(edge_self: &Edge, edge_other: &Edge) -> Vec<EdgeEdgeIntersection> {
    match curve_curve_intersection(&*edge_self.curve, &*edge_other.curve) {
        CurveCurveIntersection::Curve(_) => {
            edge_edge_same_curve_intersection(edge_self, edge_other)
        }
        CurveCurveIntersection::Points(points) => points
            .iter()
            .filter(|p| {
                edge_point_contains(edge_self, **p) != EdgePointContains::Outside
                    && edge_point_contains(edge_other, **p) != EdgePointContains::Outside
            })
            .map(|p| EdgeEdgeIntersection::Point(Rc::new(*p)))
            .collect(),
        CurveCurveIntersection::None => {
            vec![]
        }
    }
}
