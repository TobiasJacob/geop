use std::rc::Rc;

use geop_geometry::{
    curve_surface_intersection::curve_surface::curve_surface_intersection,
    efloat::EFloat64,
    point::Point,
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
    pub boundaries: Vec<Contour>, // Outer boundary is Coutner-clockwise, inner ones are Clockwise. However, each of theh inner ones can also be the outside. So the only condition that applies to them is that they are not self-intersecting.
    pub surface: Rc<Surface>,
}

// Implements a Face. A Face is bounded by the outer_loop and might have holes in inner_loops.
// outer_loop has to be clockwise, if the face is looked at from normal direction (normal facing towards you).
// inner_loops have to be counter-clockwise, if the face is looked at from normal direction (normal facing towards you).
// The contours are not allowed to intersect in any way. Keep in mind that a point is not considered an intersection, hence it is allowed that the contours touch each other at points.
impl Face {
    pub fn new(boundaries: Vec<Contour>, surface: Rc<Surface>) -> Face {
        for contour in boundaries.iter() {
            for edge in contour.edges.iter() {
                assert!(curve_surface_intersection(&edge.curve, &*surface).is_curve());
            }
        }

        let f = Face {
            boundaries,
            surface,
        };
        f.inner_point(); // Check if inner point exists
        f
    }

    pub fn try_new_face(boundaries: Vec<Contour>, surface: Rc<Surface>) -> Option<Face> {
        for contour in boundaries.iter() {
            for edge in contour.edges.iter() {
                assert!(curve_surface_intersection(&edge.curve, &*surface).is_curve());
            }
        }

        let f = Face {
            boundaries,
            surface,
        };
        if f.try_inner_point().is_some() {
            return Some(f);
        }
        None
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

    pub fn try_inner_point(&self) -> Option<Point> {
        for e1 in self.boundaries[0].edges.iter() {
            for e2 in self.boundaries[0].edges.iter() {
                if e1 != e2 {
                    let geodesic = self.edge_from_to(e1.get_midpoint(), e2.get_midpoint());
                    let p = geodesic.get_midpoint();
                    if face_point_contains(self, p) == FacePointContains::Inside {
                        return Some(p);
                    }
                }
            }
        }
        None
    }

    pub fn inner_point(&self) -> Point {
        if self.boundaries.len() == 0 {
            return self.surface.point_grid(1.0)[0];
        }

        let p = self.boundaries[0].edges[0].get_midpoint();
        let dist = EFloat64::from(0.01);
        let normal = self.normal(p);
        let tangent = self.boundary_tangent(p);
        let extend_dir = normal.cross(*tangent.expect_on_edge()) * dist;
        let inner_point = self.surface.exp(p, extend_dir);
        if face_point_contains(self, inner_point) == FacePointContains::Inside {
            return inner_point;
        }
        for e1 in self.all_edges().iter() {
            for e2 in self.all_edges().iter() {
                if e1 != e2 {
                    let geodesic = self.edge_from_to(e1.get_midpoint(), e2.get_midpoint());
                    let p = geodesic.get_midpoint();
                    // println!("Checking {:?}", p);
                    if face_point_contains(self, p) == FacePointContains::Inside {
                        return p;
                    }
                }
            }
        }
        println!("Error creating face");
        for c in self.boundaries.iter() {
            println!("{}", c);
        }
        panic!("No inner point found");
    }

    pub fn edge_from_to(&self, from: Point, to: Point) -> Edge {
        Edge::new(
            Some(from.clone()),
            Some(to.clone()),
            self.surface.geodesic(from, to),
        )
    }

    pub fn get_boundary_point(&self) -> Option<Point> {
        if self.boundaries.len() > 0 {
            return Some(self.boundaries[0].edges[0].get_midpoint());
        }
        None
    }

    pub fn boundary_tangent(&self, p: Point) -> ContourTangent {
        for contour in self.boundaries.iter() {
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
            boundaries: self.boundaries.iter().rev().map(|l| l.flip()).collect(),
            surface: self.surface.clone(),
        }
    }

    pub fn flip(&self) -> Face {
        Face {
            boundaries: self.boundaries.iter().map(|l| l.flip()).collect(),
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
                    writeln!(f, "Boundary:")?;
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
                for contour in self.boundaries.iter() {
                    writeln!(f, "Boundary:")?;
                    for edge in contour.edges.iter() {
                        writeln!(f, "  {}", edge)?;
                    }
                }
            }
        };
        Ok(())
    }
}
