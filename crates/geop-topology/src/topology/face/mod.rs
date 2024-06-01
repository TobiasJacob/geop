use std::rc::Rc;

use geop_geometry::{points::point::Point, surfaces::surface::Surface, transforms::Transform};

use crate::contains::{
    edge_point::EdgePointContains,
    face_point::{face_point_contains, FacePointContains},
    surface_edge::surface_edge_contains,
};

use super::{
    contour::ContourTangent,
    {contour::Contour, edge::Edge},
};

#[derive(Clone, Debug)]
pub struct Face {
    pub boundary: Contour,   // Coutner-clockwise
    pub holes: Vec<Contour>, // Clockwise
    pub surface: Rc<Surface>,
}

// Implements a Face. A Face is bounded by the outer_loop and might have holes in inner_loops.
// outer_loop has to be clockwise, if the face is looked at from normal direction (normal facing towards you).
// inner_loops have to be counter-clockwise, if the face is looked at from normal direction (normal facing towards you).
// The contours are not allowed to intersect in any way. Keep in mind that a point is not considered an intersection, hence it is allowed that the contours touch each other at points.
impl Face {
    pub fn new(boundary: Contour, holes: Vec<Contour>, surface: Rc<Surface>) -> Face {
        for edge in boundary.edges.iter() {
            assert!(surface_edge_contains(&surface, edge));
        }
        for contour in holes.iter() {
            for edge in contour.edges.iter() {
                assert!(surface_edge_contains(&surface, edge));
            }
        }
        Face {
            boundary,
            holes,
            surface,
        }
    }

    pub fn transform(&self, transform: Transform) -> Face {
        Face::new(
            self.boundary.transform(transform),
            self.holes
                .iter()
                .map(|contour| contour.transform(transform))
                .collect(),
            Rc::new(self.surface.transform(transform)),
        )
    }

    pub fn all_points(&self) -> Vec<Point> {
        let mut points = Vec::<Point>::new();

        points.extend(self.boundary.all_points());
        for contour in self.holes.iter() {
            points.extend(contour.all_points());
        }
        return points;
    }

    pub fn all_edges(&self) -> Vec<Edge> {
        let mut edges = Vec::<Edge>::new();

        for edge in self.boundary.edges.iter() {
            edges.push(edge.clone());
        }
        for contour in self.holes.iter() {
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
        if self.boundary.contains(p) != EdgePointContains::Outside {
            return self.boundary.tangent(p);
        }
        for contour in self.holes.iter() {
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
            boundary: self.boundary.flip(),
            holes: self.holes.iter().rev().map(|l| l.flip()).collect(),
            surface: self.surface.clone(),
        }
    }

    pub fn flip(&self) -> Face {
        Face {
            boundary: self.boundary.flip(),
            holes: self.holes.iter().rev().map(|l| l.flip()).collect(),
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
                writeln!(f, "Boundary:")?;
                for contour in self.holes.iter() {
                    writeln!(f, "Hole:")?;
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
