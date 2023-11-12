pub mod face_surface;

use std::rc::Rc;

use geop_geometry::{
    points::point::Point,
    transforms::Transform,
};

use crate::topology::contains::edge_point::EdgeContains;

use self::face_surface::FaceSurface;

use super::{
    edge::edge_curve::EdgeCurve,
    {contour::Contour, edge::Edge}, contour::ContourCorner, contains::{face_point::{face_contains_point, FaceContainsPoint}, face_edge::{face_contains_edge, FaceContainsEdge}}, intersections::edge_edge::EdgeEdgeIntersection,
};

#[derive(Clone, Debug)]
pub struct Face {
    pub boundaries: Vec<Contour>, // Coutner-clockwise
    pub surface: Rc<FaceSurface>,
}

pub enum FaceIntersection {
    Face(Face),
    Contour(Contour),
    Edge(Edge),
    Point(Point),
}

// Implements a Face. A Face is bounded by the outer_loop and might have holes in inner_loops.
// outer_loop has to be clockwise, if the face is looked at from normal direction (normal facing towards you).
// inner_loops have to be counter-clockwise, if the face is looked at from normal direction (normal facing towards you).
// The contours are not allowed to intersect in any way. Keep in mind that a point is not considered an intersection, hence it is allowed that the contours touch each other at points.
impl Face {
    pub fn new(boundaries: Vec<Contour>, surface: Rc<FaceSurface>) -> Face {
        assert!(boundaries.len() > 0, "Face must have at least one boundary");
        for contour in boundaries.iter() {
            for edge in contour.edges.iter() {
                assert!(surface.contains_edge(edge));
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

    pub fn all_points(&self) -> Vec<Rc<Point>> {
        let mut points = Vec::<Rc<Point>>::new();

        for contour in self.boundaries.iter() {
            points.extend(contour.all_points());
        }
        return points;
    }

    pub fn all_edges(&self) -> Vec<Rc<Edge>> {
        let mut edges = Vec::<Rc<Edge>>::new();

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

    pub fn edge_from_to(&self, from: Rc<Point>, to: Rc<Point>) -> Rc<Edge> {
        match &*self.surface {
            FaceSurface::Plane(p) => {
                let curve = p.curve_from_to(*from, *to);
                return Rc::new(Edge::new(
                    from.clone(),
                    to.clone(),
                    Rc::new(EdgeCurve::Line(curve)),
                ));
            }
            FaceSurface::Sphere(s) => {
                let curve = s.curve_from_to(*from, *to);
                return Rc::new(Edge::new(
                    from.clone(),
                    to.clone(),
                    Rc::new(EdgeCurve::Circle(curve)),
                ));
            }
        }
    }

    pub fn boundary_tangent(&self, p: Point) -> ContourCorner<Point> {
        for contour in self.boundaries.iter() {
            match contour.contains(p) {
                EdgeContains::Inside => return contour.tangent(p),
                EdgeContains::OnPoint(_) => return contour.tangent(p),
                EdgeContains::Outside => continue,
            }
        }
        panic!("Point is not on boundary");
    }

    pub fn normal(&self, p: Point) -> Point {
        match face_contains_point(self, p) {
            FaceContainsPoint::Inside => (),
            _ => panic!("Point is not on face"),
        }
        self.surface.surface().normal(p)
    }

    pub fn neg(&self) -> Face {
        Face {
            boundaries: self.boundaries.iter().rev().map(|l| l.neg()).collect(),
            surface: self.surface.clone(),
        }
    }

    pub fn flip(&self) -> Face {
        Face {
            boundaries: self.boundaries.iter().rev().map(|l| l.neg()).collect(),
            surface: Rc::new(self.surface.neg()),
        }
    }

}

// pretty print
impl std::fmt::Display for Face {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &*self.surface {
            FaceSurface::Plane(p) => {
                writeln!(f, "Plane at basis = {:?} with normal = {:?}", p.basis, p.u_slope.cross(p.v_slope))?;
                for contour in self.boundaries.iter() {
                    writeln!(f, "Contour:")?;
                    for edge in contour.edges.iter() {
                        writeln!(f, "  {}", edge)?;
                    }
                }
            }
            FaceSurface::Sphere(s) => {
                writeln!(f, "sphere is still todo")?;
            }
        };
        writeln!(f, "Boundaries:")
    }
}