use geop_geometry::points::point::Point;

use crate::topology::{
    contains::edge_point::{edge_point_contains, EdgePointContains},
    edge::Edge,
};

pub fn split_edge_by_points_if_necessary(edge: &Edge, points: &[Point]) -> Vec<Edge> {
    let mut result = vec![edge.clone()];
    for p in points {
        let mut new_result = Vec::<Edge>::new();
        for edge in result.iter() {
            if edge_point_contains(edge, *p) != EdgePointContains::Inside {
                new_result.push(edge.clone());
            } else {
                new_result.push(Edge::new(
                    edge.start.clone(),
                    p.clone(),
                    edge.curve.clone(),
                ));
                new_result.push(Edge::new(
                    p.clone(),
                    edge.end.clone(),
                    edge.curve.clone(),
                ));
            }
        }
        result = new_result;
    }
    result
}

pub fn split_edges_by_points_if_necessary(edges: Vec<Edge>, points: &Vec<Point>) -> Vec<Edge> {
    let mut result = Vec::<Edge>::new();
    for edge in edges {
        result.extend(split_edge_by_points_if_necessary(&edge, &points));
    }
    result
}
