use std::rc::Rc;

use geop_geometry::{geometry::points::point3d::Point3d, intersections::IntersectableCurve3d};

use crate::topology::Vertex::Vertex;

pub struct Edge {
    pub vertices: [Vertex; 2],
    pub curve: Rc<IntersectableCurve3d>
}

impl Edge {
    pub fn new(vertices: [Vertex; 2], curve: Rc<IntersectableCurve3d>) -> Edge {
        Edge {
            vertices,
            curve
        }
    }

    pub fn rasterize(&self) -> Vec<Point3d> {
        let num_points = 40 as usize;
        let (start, end) = self.interval();

        (0..num_points).map(|i| {
            let t = i as f64 / (num_points - 1) as f64;
            let point = self.curve.point_at(t);
            let point = point + (end - start) * t;
            point
        }).collect()
    }

    pub fn interval(&self) -> (f64, f64) {
        let start = self.curve.project(*self.vertices[0].point);
        let end = self.curve.project(*self.vertices[1].point);
        if self.vertices[0].equals(&self.vertices[1]) || end <= start {
            return (start, end + self.curve.period());
        }
        (start, end)
    }

    // Returns a sorted list of intersections. The intersections are sorted by the parameter of the first curve.
    pub fn intersections(&self, other: &Edge) -> Vec<Point3d> {
        let intersections = self.curve.intersections(&other.curve);
        let (u_min, u_max) = self.interval();
        intersections.into_iter().filter(|p| {
            let u = self.curve.project(*p);
            u_min <= u && u <= u_max
        }).collect::<Vec<Point3d>>()
    }

    // Splits this curve into subcurves at the intersections with the other curve. Returns a sorted List of new edges.
    pub fn split(&self, other: &Edge) -> (Vec<Edge>, Vec<Edge>) {
        let intersections_self = self.intersections(other);

        let vertices_self = intersections_self.into_iter().map(|p| {
            Vertex { point: Rc::new(p) }
        }).collect::<Vec<Vertex>>();
        // Creates a shallow copy of vertices_a, meaning that they still reference the same points
        let mut vertices_other = vertices_self.clone();
        vertices_other.sort_by(|b1, b2| other.curve.project(*b1.point).total_cmp(&other.curve.project(*b2.point)));

        let mut edges_self = Vec::with_capacity(vertices_self.len() + 1);
        let mut edges_other = Vec::with_capacity(vertices_self.len() + 1);

        // If the starting points are not connected, there is not going to be an intersection at the starting points, hence we have to connect the starting point to the first intersection.
        if !(self.vertices[0].equals(&other.vertices[0]) || other.vertices[1].equals(&self.vertices[1])) {
            edges_self.push(Edge::new([self.vertices[0].clone(), vertices_self[0].clone()], self.curve.clone()));
        }

        for i in 0..vertices_self.len() - 1 {
            edges_self.push(Edge::new([vertices_self[i].clone(), vertices_self[i + 1].clone()], self.curve.clone()));
        }

        // If the end points are not connected, there is not going to be an intersection at the end points, hence we have to connect the last intersection to the end point.
        if !(self.vertices[1].equals(&other.vertices[1]) || other.vertices[0].equals(&self.vertices[0])) {
            edges_self.push(Edge::new([vertices_self[vertices_self.len() - 1].clone(), self.vertices[1].clone()], self.curve.clone()));
        }

        // Same story for edge_b
        if !(other.vertices[0].equals(&self.vertices[0]) || self.vertices[1].equals(&other.vertices[1])) {
            edges_other.push(Edge::new([other.vertices[0].clone(), vertices_other[0].clone()], other.curve.clone()));
        }
        for i in 0..vertices_other.len() - 1 {
            edges_other.push(Edge::new([vertices_other[i].clone(), vertices_other[i + 1].clone()], other.curve.clone()));
        }
        if !(other.vertices[1].equals(&self.vertices[1]) || self.vertices[0].equals(&other.vertices[0])) {
            edges_other.push(Edge::new([vertices_other[vertices_other.len() - 1].clone(), other.vertices[1].clone()], other.curve.clone()));
        }

        return (edges_self, edges_other);
    }
}
