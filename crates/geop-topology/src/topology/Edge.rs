use std::rc::Rc;

use geop_geometry::geometry::{curves::curve3d::Curve3d, points::point3d::Point3d};

use crate::topology::Vertex::Vertex;

pub struct Edge {
    pub vertices: [Vertex; 2],
    pub curve: Rc<dyn Curve3d>
}

impl Edge {
    pub fn new(vertices: [Vertex; 2], curve: Rc<dyn Curve3d>) -> Edge {
        Edge {
            vertices,
            curve
        }
    }

    pub fn rasterize(&self) -> Vec<Point3d> {
        let num_points = 40 as usize;
        let start = self.curve.project(*self.vertices[0].point);
        let end = self.curve.project(*self.vertices[1].point);

        (0..num_points).map(|i| {
            let t = i as f64 / (num_points - 1) as f64;
            let point = self.curve.point_at(t);
            let point = point + (end - start) * t;
            point
        }).collect()
    }
}