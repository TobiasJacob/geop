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
        (self.curve.project(*self.vertices[0].point), self.curve.project(*self.vertices[1].point))
    }

    pub fn flip(&self) -> Edge {
        Edge::new([self.vertices[1].clone(), self.vertices[0].clone()], self.curve.clone())
    }
}