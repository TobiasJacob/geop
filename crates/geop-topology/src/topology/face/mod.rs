use std::rc::Rc;

use geop_geometry::{points::point::Point, surfaces::surface::Surface, transforms::Transform};

use crate::topology::contains::{
    edge_point::EdgePointContains, surface_edge::surface_edge_contains,
};

use super::{
    contains::face_point::{face_point_contains, FacePointContains},
    contour::ContourTangent,
    {contour::Contour, edge::Edge},
};

#[derive(Clone, Debug)]
pub struct Face {
    pub boundaries: Vec<Contour>, // Coutner-clockwise
    pub surface: Rc<Surface>,
}

// Implements a Face. A Face is bounded by the outer_loop and might have holes in inner_loops.
// outer_loop has to be clockwise, if the face is looked at from normal direction (normal facing towards you).
// inner_loops have to be counter-clockwise, if the face is looked at from normal direction (normal facing towards you).
// The contours are not allowed to intersect in any way. Keep in mind that a point is not considered an intersection, hence it is allowed that the contours touch each other at points.
impl Face {
    pub fn new(boundaries: Vec<Contour>, surface: Rc<Surface>) -> Face {
        assert!(boundaries.len() > 0, "Face must have at least one boundary");
        for contour in boundaries.iter() {
            for edge in contour.edges.iter() {
                assert!(surface_edge_contains(&surface, edge));
            }
        }
        Face {
            boundaries,
            surface,
        }
    }

    pub fn transform(&self, transform: Transform) -> Face {
        Face::new(
            self.boundaries
                .iter()
                .map(|contour| contour.transform(transform))
                .collect(),
            Rc::new(self.surface.transform(transform)),
        )
    }

    pub fn all_points(&self) -> Vec<Point> {
        let mut points = Vec::<Point>::new();

        for contour in self.boundaries.iter() {
            points.extend(contour.all_points());
        }
        return points;
    }

    pub fn all_edges(&self) -> Vec<Edge> {
        let mut edges = Vec::<Edge>::new();

        for contour in self.boundaries.iter() {
            for edge in contour.edges.iter() {
                edges.push(edge.clone());
            }
        }
        return edges;
    }

    pub fn inner_point(&self) -> Point {
        todo!("Returns an inner point where normal vector is well defined.");
    }

    pub fn edge_from_to(&self, from: Point, to: Point) -> Edge {
        Edge::new(from.clone(), to.clone(), self.surface.geodesic(from, to))
    }

    pub fn boundary_tangent(&self, p: Point) -> ContourTangent {
        for contour in self.boundaries.iter() {
            match contour.contains(p) {
                EdgePointContains::Inside => return contour.tangent(p),
                EdgePointContains::OnPoint(_) => return contour.tangent(p),
                EdgePointContains::Outside => continue,
            }
        }
        panic!("Point is not on boundary");
    }

    pub fn normal(&self, p: Point) -> Point {
        match face_point_contains(self, p) {
            FacePointContains::Inside => (),
            _ => panic!("Point is not on face"),
        }
        self.surface.normal(p)
    }

    pub fn neg(&self) -> Face {
        Face {
            boundaries: self.boundaries.iter().rev().map(|l| l.flip()).collect(),
            surface: self.surface.clone(),
        }
    }

    pub fn flip(&self) -> Face {
        Face {
            boundaries: self.boundaries.iter().rev().map(|l| l.flip()).collect(),
            surface: Rc::new(self.surface.neg()),
        }
    }
}

// pretty print
impl std::fmt::Display for Face {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &*self.surface {
            Surface::Plane(p) => {
                writeln!(
                    f,
                    "Plane at basis = {:?} with normal = {:?}",
                    p.basis,
                    p.u_slope.cross(p.v_slope)
                )?;
                for contour in self.boundaries.iter() {
                    writeln!(f, "Contour:")?;
                    for edge in contour.edges.iter() {
                        writeln!(f, "  {}", edge)?;
                    }
                }
            }
            Surface::Sphere(_s) => {
                writeln!(f, "sphere is still todo")?;
            }
        };
        writeln!(f, "Boundaries:")
    }
}
