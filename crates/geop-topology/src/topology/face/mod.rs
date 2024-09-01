use std::rc::Rc;

use geop_geometry::{
    curve_surface_intersection::curve_surface::curve_surface_intersection,
    points::point::Point,
    surfaces::{surface::Surface, SurfaceLike},
    transforms::Transform,
};

use crate::contains::{
    contour_point::contour_point_contains,
    edge_point::EdgePointContains,
    face_point::{face_point_contains, FacePointContains},
};

use super::{
    contour::ContourTangent,
    {contour::Contour, edge::Edge},
};

#[derive(Clone, Debug)]
pub struct Face {
    pub boundary: Option<Contour>, // Coutner-clockwise
    pub holes: Vec<Contour>,       // Clockwise
    pub surface: Rc<Surface>,
}

// Implements a Face. A Face is bounded by the outer_loop and might have holes in inner_loops.
// outer_loop has to be clockwise, if the face is looked at from normal direction (normal facing towards you).
// inner_loops have to be counter-clockwise, if the face is looked at from normal direction (normal facing towards you).
// The contours are not allowed to intersect in any way. Keep in mind that a point is not considered an intersection, hence it is allowed that the contours touch each other at points.
impl Face {
    pub fn new(boundary: Option<Contour>, holes: Vec<Contour>, surface: Rc<Surface>) -> Face {
        if let Some(boundary) = &boundary {
            for edge in boundary.edges.iter() {
                assert!(curve_surface_intersection(&edge.curve, &*surface).is_curve());
            }
        }
        for contour in holes.iter() {
            for edge in contour.edges.iter() {
                assert!(curve_surface_intersection(&edge.curve, &*surface).is_curve());
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
            match &self.boundary {
                Some(boundary) => Some(boundary.transform(transform)),
                None => None,
            },
            self.holes
                .iter()
                .map(|contour| contour.transform(transform))
                .collect(),
            Rc::new(self.surface.transform(transform)),
        )
    }

    pub fn all_points(&self) -> Vec<Point> {
        let mut points = Vec::<Point>::new();

        if let Some(boundary) = &self.boundary {
            points.extend(boundary.all_points());
        }
        for contour in self.holes.iter() {
            points.extend(contour.all_points());
        }
        return points;
    }

    pub fn all_edges(&self) -> Vec<Edge> {
        let mut edges = Vec::<Edge>::new();

        if let Some(boundary) = &self.boundary {
            for edge in boundary.edges.iter() {
                edges.push(edge.clone());
            }
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
        Edge::new(
            Some(from.clone()),
            Some(to.clone()),
            self.surface.geodesic(from, to),
        )
    }

    pub fn get_boundary_point(&self) -> Option<Point> {
        if let Some(boundary) = &self.boundary {
            return Some(boundary.edges[0].get_midpoint());
        }
        if self.holes.len() > 0 {
            return Some(self.holes[0].edges[0].get_midpoint());
        }
        None
    }

    pub fn boundary_tangent(&self, p: Point) -> ContourTangent {
        if let Some(boundary) = &self.boundary {
            if contour_point_contains(boundary, p) != EdgePointContains::Outside {
                return boundary.tangent(p);
            }
        }
        for contour in self.holes.iter() {
            if contour_point_contains(contour, p) != EdgePointContains::Outside {
                return contour.tangent(p);
            }
        }
        panic!("Point is not on boundary");
    }

    pub fn normal(&self, p: Point) -> Point {
        match face_point_contains(self, p) {
            FacePointContains::NotOnSurface => {
                panic!("Point is not on surface");
            }
            _ => self.surface.normal(p),
        }
    }

    pub fn neg(&self) -> Face {
        Face {
            boundary: match &self.boundary {
                Some(boundary) => Some(boundary.flip()),
                None => None,
            },
            holes: self.holes.iter().rev().map(|l| l.flip()).collect(),
            surface: self.surface.clone(),
        }
    }

    pub fn flip(&self) -> Face {
        Face {
            boundary: match &self.boundary {
                Some(boundary) => Some(boundary.flip()),
                None => None,
            },
            holes: self.holes.iter().map(|l| l.flip()).collect(),
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
                if let Some(boundary) = &self.boundary {
                    writeln!(f, "Boundary:")?;
                    for edge in boundary.edges.iter() {
                        writeln!(f, "  {}", edge)?;
                    }
                }
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
            Surface::Cylinder(c) => {
                writeln!(f, "Cylinder at bases = {:?} with extend_dir = {:?}, radius = {:?} and normal direction = {:?}", c.basis, c.extend_dir, c.radius, c.normal_outwards)?;
                if let Some(boundary) = &self.boundary {
                    writeln!(f, "Boundary:")?;
                    for edge in boundary.edges.iter() {
                        writeln!(f, "  {}", edge)?;
                    }
                }
                for contour in self.holes.iter() {
                    writeln!(f, "Hole:")?;
                    for edge in contour.edges.iter() {
                        writeln!(f, "  {}", edge)?;
                    }
                }
            }
        };
        Ok(())
    }
}
