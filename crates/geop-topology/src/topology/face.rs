use std::rc::Rc;

use geop_geometry::{surfaces::{plane::Plane, sphere::Sphere, surface::Surface}, points::point::Point, curves::line::Line};

use crate::{PROJECTION_THRESHOLD, topology::edge::edge::{Direction, EdgeCurve}};

use super::{edge::{edge_loop::EdgeLoop, edge::Edge}, vertex::Vertex};


#[derive(PartialEq, Clone, Debug)]
pub enum FaceSurface {
    Plane(Plane),
    Sphere(Sphere),
}
impl FaceSurface {
    pub fn surface(&self) -> &dyn Surface {
        match self {
            FaceSurface::Plane(plane) => plane,
            FaceSurface::Sphere(sphere) => sphere,
        }
    }
}


pub struct Face {
    pub outer_loop: EdgeLoop,
    pub inner_loops: Vec<EdgeLoop>,
    pub surface: Rc<FaceSurface>,
    center_point: Point,
}

// Every face is homeomorphic to a disk or a square, hence we can use a parametrization of the form (u, v) \in [0, 1]^2.
// We will assert that the Face is shaped such that there is a midpoint, and each line from the midpoint to the boundary is within the face.
// The centerpoint cannot be on the boundary, and the boundary cannot intersect itself.
impl Face {
    pub fn new(outer_loop: EdgeLoop, inner_loops: Vec<EdgeLoop>, surface: Rc<FaceSurface>, center_point: Point) -> Face {
        Face {
            outer_loop,
            inner_loops,
            surface,
            center_point
        }
    }

    pub fn point_at(&self, u: f64, v: f64) -> Point {
        let anchor_point = self.outer_loop.point_at(u);
        match &*self.surface {
            FaceSurface::Plane(plane) => {
                self.center_point + (anchor_point - self.center_point) * v
            },
            FaceSurface::Sphere(sphere) => {
                let axis = (sphere.basis - self.center_point).cross(anchor_point - self.center_point).normalize();
                let angle = (anchor_point - self.center_point).angle(anchor_point - sphere.basis);
                sphere.basis + axis.rotate(self.center_point - sphere.basis, angle * v)
            },
        }
    }

    pub fn project(&self, p: &Point) -> (f64, f64) {
        match &*self.surface {
            FaceSurface::Plane(plane) => {
                let direction = *p - self.center_point;
                let anchor_point = self.outer_loop.intersect(Line::new(self.center_point, direction));
                let u = self.outer_loop.project(&anchor_point).expect("Point not on boundary");
                let v = direction.norm() / (anchor_point - self.center_point).norm();
                (u, v)
            },
            FaceSurface::Sphere(sphere) => {
                todo!("Implement projection for sphere")
            },
        }
    }
}

//     pub fn intersect(&self, other: &Face) {
//         if (self.surface.equals(&other.surface)) { // Results in a Face
//             // let outer_bounds = self.outer_loop.edges[0].split(other.outer_loop.edges[0]);
//             // for (edge1, edge2) in outer_bounds {
//             //     let inner_dir = cross_product(self.surface.normal(edge1.vertices[0]), edge1.tangent(edge1.vertices[1]));
//             //     let edge1_prod = dot_product(inner_dir, edge1.tangent(edge1.vertices[0]));
//             //     let edge2_prod = dot_product(inner_dir, edge2.tangent(edge2.vertices[0]));
//             //     if edge1_prod < edge2_prod {
//             //         // Keep edge1
//             //     } else {
//             //         // Keep edge2
//             //     }
//             // }
//         }
//         // Results in a line
//         let intersection_curve = self.surface.intersect(&other.surface);

//         let outer_bounds = intersection_curve.intersections(self.outer_loop);

//         let inner_bounds = self.inner_loops[0].edges[0].intersections(intersection_curve);
//     }

//     pub fn split(&self, other: &Face) {
//         let intersection_curve = self.surface.intersect(&other.surface);
//         let outer_bounds = intersection_curve.intersections(self.outer_loop);
//         let inner_bounds = self.inner_loops[0].edges[0].intersections(intersection_curve);
//     }

//     pub fn union(&self, other: &Face) {
//         assert!(self.surface.equals(&other.surface));
//     }
//     pub fn difference(&self, other: &Face) {
//         assert!(self.surface.equals(&other.surface));
//     }
//     pub fn intersection(&self, other: &Face) {
//         assert!(self.surface.equals(&other.surface));
//     }
// }