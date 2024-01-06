use geop_geometry::points::point::Point;

use crate::topology::{
    contains::edge_point::{edge_point_contains, EdgePointContains},
    edge::Edge,
};

pub fn split_edge_by_point_if_necessary(edge: &Edge, point: &Point) -> Vec<Edge> {
    if edge_point_contains(edge, *point) != EdgePointContains::Inside {
        vec![edge.clone()]
    } else {
        vec![
            Edge::new(
                edge.start.clone(),
                point.clone(),
                edge.curve.clone(),
            ),
            Edge::new(
                point.clone(),
                edge.end.clone(),
                edge.curve.clone(),
            ),
        ]
    }
}

pub fn split_edge_by_points_if_necessary(edge: &Edge, points: &[Point]) -> Vec<Edge> {
    let mut result = vec![edge.clone()];
    for p in points {
        let mut new_result = Vec::<Edge>::new();
        for edge in result.iter() {
            new_result.extend(split_edge_by_point_if_necessary(edge, p));
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
