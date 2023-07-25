use std::rc::Rc;

use geop_geometry::geometry::points::point3d::Point3d;

use crate::topology::{Edge::Edge, Vertex::Vertex};

impl Edge {
    // Returns a sorted list of intersections. The intersections are sorted by the parameter of the first curve.
    pub fn intersections(&self, other: &Edge) -> Vec<Point3d> {
        let intersections = self.curve.intersections(&other.curve);
        let (u_min, u_max) = self.interval();
        intersections.into_iter().filter(|p| {
            let u = self.curve.project(*p);
            u_min <= u && u <= u_max
        }).collect::<Vec<Point3d>>()
    }
}

// Splits this curve into subcurves at the intersections with the other curve. Returns a sorted List of new edges.
pub fn split(a: &Edge, b: &Edge) -> (Vec<Edge>, Vec<Edge>) {
    let intersections_a = a.intersections(b);

    let vertices_a = intersections_a.into_iter().map(|p| {
        Vertex { point: Rc::new(p) }
    }).collect::<Vec<Vertex>>();
    // Creates a shallow copy of vertices_a, meaning that they still reference the same points
    let mut vertices_b = vertices_a.clone();
    vertices_b.sort_by(|b1, b2| b.curve.project(*b1.point).total_cmp(&b.curve.project(*b2.point)));

    let mut edges_a = Vec::with_capacity(vertices_a.len() + 1);
    let mut edges_b = Vec::with_capacity(vertices_a.len() + 1);

    // If the starting points are not connected, there is not going to be an intersection at the starting points, hence we have to connect the starting point to the first intersection.
    if !(a.vertices[0].equals(&b.vertices[0]) || b.vertices[1].equals(&a.vertices[1])) {
        edges_a.push(Edge::new([a.vertices[0].clone(), vertices_a[0].clone()], a.curve.clone()));
    }

    for i in 0..vertices_a.len() - 1 {
        edges_a.push(Edge::new([vertices_a[i].clone(), vertices_a[i + 1].clone()], a.curve.clone()));
    }

    // If the end points are not connected, there is not going to be an intersection at the end points, hence we have to connect the last intersection to the end point.
    if !(a.vertices[1].equals(&b.vertices[1]) || b.vertices[0].equals(&a.vertices[0])) {
        edges_a.push(Edge::new([vertices_a[vertices_a.len() - 1].clone(), a.vertices[1].clone()], a.curve.clone()));
    }

    // Same story for edge_b
    if !(b.vertices[0].equals(&a.vertices[0]) || a.vertices[1].equals(&b.vertices[1])) {
        edges_b.push(Edge::new([b.vertices[0].clone(), vertices_b[0].clone()], b.curve.clone()));
    }
    for i in 0..vertices_b.len() - 1 {
        edges_b.push(Edge::new([vertices_b[i].clone(), vertices_b[i + 1].clone()], b.curve.clone()));
    }
    if !(b.vertices[1].equals(&a.vertices[1]) || a.vertices[0].equals(&b.vertices[0])) {
        edges_b.push(Edge::new([vertices_b[vertices_b.len() - 1].clone(), b.vertices[1].clone()], b.curve.clone()));
    }

    return (edges_a, edges_b);
}