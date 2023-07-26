use std::rc::Rc;

use geop_geometry::{geometry::{points::{point::Point, point2d::Point2d}, surfaces::{surface::Surface, plane::Plane}}, intersections::surface_surface::IntersectableSurface};

use super::{Edge::Edge, EdgeLoop::EdgeLoop};

enum PlanarFaceSurface {
    Plane(Plane),
    Cylinder(Cylinder),
    Sphere(Sphere)
}

// No surface dimensions are periodic. Its boundary is a closed loop. Note that for example a small cutout from a sphere bound by an outer edge is a planar face, despite the sphere being periodic.
pub struct PlanarFace {
    pub outer_loop: EdgeLoop,
    pub inner_loops: Vec<EdgeLoop>,
    pub surface: Rc<PlanarFaceSurface>
}

enum CylindricalFaceSurface {
    Cylinder(Cylinder),
    Sphere(Sphere)
}

// One surface dimension is periodic. Two boundaries are needed.
pub struct CylindricalFace {
    pub outer_loop_1: EdgeLoop,
    pub outer_loop_2: EdgeLoop,
    pub inner_loops: Vec<EdgeLoop>,
    pub surface: Rc<CylindricalFaceSurface>
}

enum SphericalFaceSurface {
    Sphere(Sphere)
}
// Two surface dimensions are periodic / There are no boundaries needed
pub struct SphericalFace {
    pub inner_loops: Vec<EdgeLoop>,
    pub surface: Rc<SphericalFaceSurface>
}


impl Face {
    pub fn new(outer_loop: EdgeLoop, inner_loops: Vec<EdgeLoop>, surface: Rc<IntersectableSurface>) -> Face {
        Face {
            outer_loop,
            inner_loops,
            surface
        }
    }

    pub fn rasterize(&self) -> Vec<Vec<Point>> {
        let outer_edge: Vec<Point2d> = self.outer_loop.rasterize().iter().map(|point| self.surface.project(*point)).collect();
        let inner_edges: Vec<Vec<Point2d>> = self.inner_loops.iter().map(|edge| edge.rasterize().iter().map(|p| self.surface.project(*p)).collect()).collect();
        
        let x_min = outer_edge.iter().map(|p| p.x).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let x_max = outer_edge.iter().map(|p| p.x).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

        let mut raster: Vec<Vec<Point>> = Vec::new();

        let res = 50;
        for i in 0..res {
            let mut raster_line: Vec<Point> = Vec::new();
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

    pub fn intersect(&self, other: &Face) {
        if (self.surface.equals(&other.surface)) { // Results in a Face
            // let outer_bounds = self.outer_loop.edges[0].split(other.outer_loop.edges[0]);
            // for (edge1, edge2) in outer_bounds {
            //     let inner_dir = cross_product(self.surface.normal(edge1.vertices[0]), edge1.tangent(edge1.vertices[1]));
            //     let edge1_prod = dot_product(inner_dir, edge1.tangent(edge1.vertices[0]));
            //     let edge2_prod = dot_product(inner_dir, edge2.tangent(edge2.vertices[0]));
            //     if edge1_prod < edge2_prod {
            //         // Keep edge1
            //     } else {
            //         // Keep edge2
            //     }
            // }
        }
        // Results in a line
        let intersection_curve = self.surface.intersect(&other.surface);

        let outer_bounds = intersection_curve.intersections(self.outer_loop);

        let inner_bounds = self.inner_loops[0].edges[0].intersections(intersection_curve);
    }

    pub fn split(&self, other: &Face) {
        let intersection_curve = self.surface.intersect(&other.surface);
        let outer_bounds = intersection_curve.intersections(self.outer_loop);
        let inner_bounds = self.inner_loops[0].edges[0].intersections(intersection_curve);
    }

    pub fn union(&self, other: &Face) {
        assert!(self.surface.equals(&other.surface));
    }
    pub fn difference(&self, other: &Face) {
        assert!(self.surface.equals(&other.surface));
    }
    pub fn intersection(&self, other: &Face) {
        assert!(self.surface.equals(&other.surface));
    }
}