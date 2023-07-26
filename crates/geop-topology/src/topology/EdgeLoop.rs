use std::rc::Rc;

use geop_geometry::geometry::points::point3d::Point3d;


use super::Edge::Edge;

pub struct EdgeLoop {
    pub edges: Vec<Rc<Edge>>
}

impl EdgeLoop {
    pub fn new(edges: Vec<Rc<Edge>>) -> EdgeLoop {
        for i in 0..edges.len() {
            let edge = edges[i].clone();
            let next_edge = edges[(i + 1) % edges.len()].clone();
            assert!(&edge.vertices[1].equals(&next_edge.vertices[0]));
        }

        EdgeLoop {
            edges
        }
    }

    pub fn rasterize(&self) -> Vec<Point3d> {
        self.edges.iter().flat_map(|edge| edge.rasterize()).collect()
    }

    // Splits this edge loop with another edge loop. This makes sure that the resulting edge loops are closed and do not intersect each other anymore.
    pub fn split(&self, other: &EdgeLoop) -> (Vec<EdgeLoop>, Vec<EdgeLoop>) {

    }
}