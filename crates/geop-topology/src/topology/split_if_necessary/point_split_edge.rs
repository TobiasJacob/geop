use std::rc::Rc;

use geop_geometry::points::point::Point;

use crate::topology::{
    contains::edge_point::{edge_point_contains, EdgePointContains},
    edge::Edge,
};

pub fn split_edge_by_point_if_necessary(edge: &Edge, points: &[Rc<Point>]) -> Edge {
    let mut edge = edge.clone();
    for point in points.iter() {
        match edge_point_contains(&edge, point) {
            EdgePointContains::Inside(i) => {
                let intervals_a = &edge.boundaries[0..i];
                let interval_middle = &edge.boundaries[i];
                let intervals_c = &edge.boundaries[i + 1..];
    
                let intervals_b = vec![
                    (interval_middle.0.clone(), point.clone()),
                    (point.clone(), interval_middle.1.clone()),
                ];
                let intervals = [
                    intervals_a,
                    &intervals_b,
                    intervals_c,
                ].concat();
                edge = Edge::new(intervals, edge.curve.clone());
            }
            _ => {
            }
        }
    }
    edge
}
