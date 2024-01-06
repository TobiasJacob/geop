use geop_geometry::{curves::curve::Curve, points::point::Point};

use crate::topology::{
    contains::edge_point::{edge_point_contains, EdgePointContains},
    edge::Edge,
    split_if_necessary::point_split_edge::split_edge_by_points_if_necessary,
};

pub fn edge_split_points(a: &Edge, b: &Edge) -> Vec<Point> {
    let mut split_points = Vec::<Point>::new();

    if !split_points.contains(&a.start) {
        split_points.push(a.start.clone());
    }
    if !split_points.contains(&a.end) {
        split_points.push(a.end.clone());
    }
    if !split_points.contains(&b.start) {
        split_points.push(b.start.clone());
    }
    if !split_points.contains(&b.end) {
        split_points.push(b.end.clone());
    }

    split_points
}

#[derive(Debug)]
pub enum EdgeRemesh {
    AinB(Edge),
    AoutB(Edge),
    BinA(Edge),
    BoutA(Edge),
}

pub fn edge_split(edge_a: &Edge, edge_b: &Edge) -> Vec<EdgeRemesh> {
    let intersections = edge_split_points(edge_a, edge_b);

    let mut edges_a = split_edge_by_points_if_necessary(
        edge_a,
        intersections.as_slice(),
    );
    let mut edges_b = split_edge_by_points_if_necessary(
        edge_b,
        intersections.as_slice(),
    );

    let mut result = Vec::<EdgeRemesh>::new();
    for a in edges_a.drain(..) {
        let mid_point = a.curve.get_midpoint(a.start, a.end);
        match edge_point_contains(edge_b, mid_point) {
            EdgePointContains::Inside => {
                result.push(EdgeRemesh::AinB(
                    a
                ));
            }
            EdgePointContains::Outside => {
                result.push(EdgeRemesh::AoutB(
                    a
                ));
            }
            EdgePointContains::OnPoint(_) => {
                panic!("This should not happen")
            }
        }
    }

    for b in edges_b.drain(..) {
        let mid_point = b.curve.get_midpoint(b.start, b.end);
        match edge_point_contains(edge_a, mid_point) {
            EdgePointContains::Inside => {
                result.push(EdgeRemesh::BinA(
                    b
                ));
            }
            EdgePointContains::Outside => {
                result.push(EdgeRemesh::BoutA(
                    b
                ));
            }
            EdgePointContains::OnPoint(_) => {
                panic!("This should not happen")
            }
        }
    }

    result
}

pub fn edge_remesh(curve: &Curve, mut intervals_intermediate: Vec<EdgeRemesh>) -> Vec<Edge> {
    intervals_intermediate
        .drain(..)
        .map(|interval| match interval {
            EdgeRemesh::AinB(a) => a,
            EdgeRemesh::AoutB(a) => a,
            EdgeRemesh::BinA(b) => b,
            EdgeRemesh::BoutA(b) => b,
        })
        .collect()
}
