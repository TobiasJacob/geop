use std::rc::Rc;

use geop_geometry::{curves::curve::Curve, points::point::Point};

use crate::topology::{
    contains::edge_point::{edge_point_contains, EdgePointContains},
    edge::Edge,
    regularize::edge::edge_regularize,
    split_if_necessary::point_split_edge::split_edge_by_point_if_necessary,
};

pub fn edge_split_points(a: &Edge, b: &Edge) -> Vec<Rc<Point>> {
    let mut split_points = Vec::<Rc<Point>>::new();

    for (s, e) in a.boundaries.iter() {
        if !split_points.contains(s) {
            split_points.push(s.clone());
        }
        if !split_points.contains(e) {
            split_points.push(e.clone());
        }
    }

    for (s, e) in b.boundaries.iter() {
        if !split_points.contains(s) {
            split_points.push(s.clone());
        }
        if !split_points.contains(e) {
            split_points.push(e.clone());
        }
    }

    split_points
}

#[derive(Debug)]
pub enum EdgeRemesh {
    AinB(Rc<Point>, Rc<Point>),
    AoutB(Rc<Point>, Rc<Point>),
    BinA(Rc<Point>, Rc<Point>),
    BoutA(Rc<Point>, Rc<Point>),
}

pub fn edge_split(edge_a: &Edge, edge_b: &Edge) -> Vec<EdgeRemesh> {
    let intersections = edge_split_points(edge_a, edge_b);

    let mut edges_a = edge_regularize(&split_edge_by_point_if_necessary(
        edge_a,
        intersections.as_slice(),
    ));
    let mut edges_b = edge_regularize(&split_edge_by_point_if_necessary(
        edge_b,
        intersections.as_slice(),
    ));

    let mut result = Vec::<EdgeRemesh>::new();
    for a in edges_a.drain(..) {
        assert!(a.boundaries.len() == 1);
        let mid_point = a.curve.get_midpoint(*a.boundaries[0].0, *a.boundaries[0].1);
        match edge_point_contains(edge_b, &Rc::new(mid_point)) {
            EdgePointContains::Inside(_) => {
                result.push(EdgeRemesh::AinB(
                    a.boundaries[0].0.clone(),
                    a.boundaries[0].1.clone(),
                ));
            }
            EdgePointContains::Outside => {
                result.push(EdgeRemesh::AoutB(
                    a.boundaries[0].0.clone(),
                    a.boundaries[0].1.clone(),
                ));
            }
            EdgePointContains::OnPoint(_) => {
                panic!("This should not happen")
            }
        }
    }

    for b in edges_b.drain(..) {
        assert!(b.boundaries.len() == 1);
        let mid_point = b.curve.get_midpoint(*b.boundaries[0].0, *b.boundaries[0].1);
        match edge_point_contains(edge_a, &Rc::new(mid_point)) {
            EdgePointContains::Inside(_) => {
                result.push(EdgeRemesh::BinA(
                    b.boundaries[0].0.clone(),
                    b.boundaries[0].1.clone(),
                ));
            }
            EdgePointContains::Outside => {
                result.push(EdgeRemesh::BoutA(
                    b.boundaries[0].0.clone(),
                    b.boundaries[0].1.clone(),
                ));
            }
            EdgePointContains::OnPoint(_) => {
                panic!("This should not happen")
            }
        }
    }

    result
}

pub fn edge_remesh(curve: Rc<Curve>, mut intervals_intermediate: Vec<EdgeRemesh>) -> Edge {
    let intervals = intervals_intermediate
        .drain(..)
        .map(|interval| match interval {
            EdgeRemesh::AinB(start, end) => (start, end),
            EdgeRemesh::AoutB(start, end) => (start, end),
            EdgeRemesh::BinA(start, end) => (start, end),
            EdgeRemesh::BoutA(start, end) => (start, end),
        })
        .collect();

    Edge::new(intervals, curve)
}
