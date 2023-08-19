use std::{rc::Rc, slice::Iter};

use geop_geometry::{surfaces::{plane::Plane, sphere::Sphere, surface::Surface}, points::point::Point, curves::line::Line};

use crate::{PROJECTION_THRESHOLD, topology::{edge::{Direction, EdgeCurve, EdgeIntersection}, edge_loop::remesh_multiple_multiple}};

use super::{{edge_loop::EdgeLoop, edge::Edge}, vertex::Vertex};


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


#[derive(Clone, Debug)]
pub struct Face {
    pub outer_loop: EdgeLoop, // Clockwise
    pub inner_loops: Vec<EdgeLoop>, // Coutner-clockwise
    pub surface: Rc<FaceSurface>,
    // convex_boundary: EdgeLoop, // TODO: Probably not needed
    // center_point: Point, // TODO: Probably not needed
}

pub enum FaceIntersection {
    Face(Face),
    EdgeLoop(EdgeLoop),
    Edge(Edge),
    Vertex(Vertex)
}

// Every face is homeomorphic to a disk or a square, hence we can use a parametrization of the form (u, v) \in [0, 1]^2.
// We will assert that the Face is shaped such that there is a midpoint, and each line from the midpoint to the boundary is within the face.
// The centerpoint cannot be on the boundary, and the boundary cannot intersect itself.
impl Face {
    pub fn all_edges(&self) -> Vec<Rc<Edge>> {
        let mut edges = Vec::<Rc<Edge>>::new();
        for edge in self.outer_loop.edges.iter() {
            edges.push(edge.clone());
        }

        for edge_loop in self.inner_loops.iter() {
            for edge in edge_loop.edges.iter() {
                edges.push(edge.clone());
            }
        }
        return edges;
    }

    pub fn contains(&self, other: &Point) -> bool {
        todo!("Implement contains");
    }

    pub fn subsurface(&self, cutting_edges: Vec<Edge>) -> Face {
        todo!("Cut this face with the given edges, such that outer loop is cw");
    }

    pub fn intersect(&self, other: &Face) -> Vec<FaceIntersection> {
        todo!("Implement intersect");
    }

    pub fn split_parts(&self, other: &Face) -> Option<(Face, Vec<Face>)> {
        assert!(self.surface == other.surface);
        
        let edge_loops_self = self.inner_loops.clone();
        edge_loops_self.push(self.outer_loop.clone());

        let edge_loops_other = other.inner_loops.clone();
        edge_loops_other.push(other.outer_loop.clone());
        
        let remeshed = remesh_multiple_multiple(edge_loops_self, edge_loops_other);

        if did_not_intersect {
            return None;
        }
        // Now its simple.
        // All clockwise edge loops are caveties in the union.
        // The largest counter clockwise edge loop is the outer loop of the union.
        // All remaining counter clockwise edge loops are intersections.


        (split_self, split_other)
    }

    pub fn union(&self, other: &Face) -> Vec<Face> {
        assert!(self.surface == other.surface);

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