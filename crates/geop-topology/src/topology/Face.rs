use std::rc::Rc;

use geop_geometry::geometry::{points::{point3d::Point3d, point2d::Point2d}, surfaces::surface::Surface};

use super::Edge::Edge;


pub struct EdgeLoop {
    pub outer_loop: Edge,
    pub inner_loops: Vec<Edge>,
    pub surface: Rc<dyn Surface>
}

impl EdgeLoop {
    pub fn new(outer_loop: Edge, inner_loops: Vec<Edge>, surface: Rc<dyn Surface>) -> EdgeLoop {
        EdgeLoop {
            outer_loop,
            inner_loops,
            surface
        }
    }

    pub fn rasterize(&self) -> Vec<Vec<Point3d>> {
        let outer_edge: Vec<Point2d> = self.outer_loop.rasterize().iter().map(|point| self.surface.project(*point)).collect();
        let inner_edges: Vec<Vec<Point2d>> = self.inner_loops.iter().map(|edge| edge.rasterize().iter().map(|p| self.surface.project(*p)).collect()).collect();
        
        let x_min = outer_edge.iter().map(|p| p.x).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let x_max = outer_edge.iter().map(|p| p.x).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

        let mut raster: Vec<Vec<Point3d>> = Vec::new();

        let res = 50;
        for i in 0..res {
            let mut raster_line: Vec<Point3d> = Vec::new();
            let x = x_min + (x_max - x_min) * i as f64 / (res - 1) as f64;
            let y_min: f64 = todo!("Use some linear interpolation to find min y");
            let y_max: f64 = todo!("Use some linear interpolation to find max y");
            for j in 0..res {
                let y = y_min + (y_max - y_min) * j as f64 / (res - 1) as f64;
                let point = Point2d::new(x, y);
                let point = self.surface.point_at(point);
                raster_line.push(point);
            }
            raster.push(raster_line);
        }
        raster
    }
}