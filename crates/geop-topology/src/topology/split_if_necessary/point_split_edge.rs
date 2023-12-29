use std::rc::Rc;

use geop_geometry::points::point::Point;

use crate::topology::{
    contains::edge_point::{edge_point_contains, EdgePointContains},
    edge::Edge,
};

pub fn split_edge_by_point_if_necessary(edge: Rc<Edge>, point: Rc<Point>) -> Vec<Rc<Edge>> {
    if edge_point_contains(&*edge, *point) != EdgePointContains::Inside {
        return vec![edge];
    }
    vec![
        Rc::new(Edge::new(
            edge.start.clone(),
            point.clone(),
            edge.curve.clone(),
        )),
        Rc::new(Edge::new(
            point.clone(),
            edge.end.clone(),
            edge.curve.clone(),
        )),
    ]
}

pub fn split_edges_by_point_if_necessary(edges: Vec<Rc<Edge>>, point: Rc<Point>) -> Vec<Rc<Edge>> {
    let mut result = Vec::<Rc<Edge>>::new();
    for edge in edges {
        result.extend(split_edge_by_point_if_necessary(edge, point.clone()));
    }
    result
}
