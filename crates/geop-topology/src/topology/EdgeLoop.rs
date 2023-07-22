use std::rc::Rc;

use geop_geometry::geometry::points::point3d::Point3d;


use super::Edge::Edge;

pub struct EdgeLoop {
    pub edges: Vec<Rc<Edge>>
}

impl EdgeLoop {
    pub fn new(edges: Vec<Rc<Edge>>) -> EdgeLoop {
        EdgeLoop {
            edges
        }
    }

    pub fn rasterize(&self) -> Vec<Point3d> {
        self.edges.iter().flat_map(|edge| edge.rasterize()).collect()
    }
}