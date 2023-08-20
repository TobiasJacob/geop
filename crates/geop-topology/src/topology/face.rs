use std::rc::Rc;

use geop_geometry::{surfaces::{plane::Plane, sphere::Sphere, surface::Surface}, points::point::Point, curves::line::Line};

use crate::{PROJECTION_THRESHOLD, topology::{edge::{Direction, EdgeCurve, EdgeIntersection}, contour::remesh_multiple_multiple}};

use super::{{contour::Contour, edge::Edge}, vertex::Vertex};


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

#[derive(Clone, Debug, PartialEq)]
pub enum ContourDirection {
    Clockwise,
    CounterClockwise,
}


#[derive(Clone, Debug)]
pub struct Face {
    pub outer_loop: Contour, // Clockwise
    pub inner_loops: Vec<Contour>, // Coutner-clockwise
    pub surface: Rc<FaceSurface>,
}

pub enum FaceIntersection {
    Face(Face),
    Contour(Contour),
    Edge(Edge),
    Vertex(Vertex)
}

// Every face is homeomorphic to a disk or a square, hence we can use a parametrization of the form (u, v) \in [0, 1]^2.
// We will assert that the Face is shaped such that there is a midpoint, and each line from the midpoint to the boundary is within the face.
// The centerpoint cannot be on the boundary, and the boundary cannot intersect itself.
impl Face {
    pub fn new(outer_loop: Contour, inner_loops: Vec<Contour>, surface: Rc<FaceSurface>) -> Face {
        Face {
            outer_loop,
            inner_loops,
            surface,
        }
    }

    pub fn all_edges(&self) -> Vec<Rc<Edge>> {
        let mut edges = Vec::<Rc<Edge>>::new();
        for edge in self.outer_loop.edges.iter() {
            edges.push(edge.clone());
        }

        for contour in self.inner_loops.iter() {
            for edge in contour.edges.iter() {
                edges.push(edge.clone());
            }
        }
        return edges;
    }

    pub fn contains_point(&self, other: &Point) -> bool {
        todo!("Implement contains");
    }

    pub fn contains_edge(&self, other: &Edge) -> bool {
        todo!("Implement contains");
    }

    pub fn contains_contour(&self, other: &Contour) -> bool {
        todo!("Implement contains");
    }

    pub fn contour_direction(&self, other: &Contour) -> ContourDirection {
        todo!("Implement contour_direction");
    }

    pub fn intersect(&self, other: &Face) -> Vec<FaceIntersection> {
        todo!("Implement intersect");
    }

    pub fn subsurface(&self, cutting_contour: Contour) -> Rc<Face> {
        let mut contours_self = self.inner_loops.clone();
        contours_self.push(self.outer_loop.clone());

        let new_contours = cutting_contour.remesh_multiple(contours_self.as_slice());

        let (ccw_conts, cw_conts): (Vec<Contour>, Vec<Contour>) = new_contours.into_iter().partition(|l| self.contour_direction(l) == ContourDirection::CounterClockwise);
        assert!(ccw_conts.len() == 2, "Expected 2 counter clockwise edge loops, found {}", ccw_conts.len());
        let (outer_loops, invalid_loops): (Vec<Contour>, Vec<Contour>) = ccw_conts.into_iter().partition(|l| {
            for edge in &l.edges {
                if !self.contains_edge(edge) {
                    return false;
                }
            }
            true
        });
        assert!(outer_loops.len() == 1, "Expected 1 counter clockwise edge loop, found {}", outer_loops.len());
        let outer_loop = outer_loops[0].clone();
        let inner_loops = cw_conts;

        Rc::new(Face::new(
            outer_loop,
            inner_loops,
            self.surface.clone(),
        ))
    }

    pub fn split_parts(&self, other: &Face) -> Option<(Face, Vec<Face>)> {
        assert!(self.surface == other.surface);
        
        let mut contours_self = self.inner_loops.clone();
        contours_self.push(self.outer_loop.clone());

        let mut contours_other = other.inner_loops.clone();
        contours_other.push(other.outer_loop.clone());
        
        let remeshed = remesh_multiple_multiple(contours_self.as_slice(), contours_other.as_slice());
        let (mut ccw_conts, mut cw_conts): (Vec<Contour>, Vec<Contour>) = remeshed.into_iter().partition(|l| self.contour_direction(l) == ContourDirection::CounterClockwise);

        // Now its simple.
        // All clockwise edge loops are caveties in the union.
        // The largest counter clockwise edge loop is the outer loop of the union.
        // All remaining counter clockwise edge loops are intersections.
        let mut i_max = 0;
        let mut result_valid = false;
        for (i, ccw_cont) in ccw_conts.iter().enumerate() {
            let temp_face = Face::new(
                ccw_cont.clone(),
                vec![],
                self.surface.clone(),
            );
            if temp_face.contains_contour(&ccw_conts[i_max]) {
                i_max = i;
                result_valid = true;
            }
        }

        // This means the Faces did not intersect
        if !result_valid {
            return None;
        }

        let union_contour = ccw_conts.remove(i_max);

        let mut intersecions = Vec::<Face>::new();
        for ccw_cont in ccw_conts {
            let mut face = Face::new(
                ccw_cont.clone(),
                vec![],
                self.surface.clone(),
            );
            let (inner_loops, cw_conts_new): (Vec<Contour>, Vec<Contour>) = cw_conts.into_iter().partition(|l| face.contains_contour(l));
            cw_conts = cw_conts_new;
            face.inner_loops = inner_loops;
            intersecions.push(face);
        }

        let union_face = Face::new(
            union_contour,
            cw_conts,
            self.surface.clone(),
        );

        Some((union_face, intersecions))
    }

    pub fn neg(&self) -> Face {
        let neg_outer_loop = self.outer_loop.neg();
        let neg_inner_loops = self.inner_loops.iter().map(|l| l.neg()).collect();
        Face {
            outer_loop: neg_outer_loop,
            inner_loops: neg_inner_loops,
            surface: self.surface.clone(),
        }
    }

    pub fn surface_union(&self, other: &Face) -> Option<Face> {
        assert!(self.surface == other.surface);
        Some(self.split_parts(other)?.0)
    }

    pub fn surface_intersection(&self, other: &Face) -> Option<Vec<Face>> {
        assert!(self.surface == other.surface);
        Some(self.split_parts(other)?.1)
    }

    pub fn surface_difference(&self, other: &Face) -> Option<Face> {
        assert!(self.surface == other.surface);
        Some(self.neg().surface_union(other)?.neg())
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