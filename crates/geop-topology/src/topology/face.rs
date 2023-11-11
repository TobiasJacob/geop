use core::panic;
use std::rc::Rc;

use geop_geometry::{
    points::point::Point,
    surfaces::{plane::Plane, sphere::{Sphere, SphereTransform}, surface::Surface}, transforms::Transform, EQ_THRESHOLD,
};

use crate::topology::edge::{Direction, EdgeCurve, EdgeEdgeIntersection};

use super::{
    edge::{EdgeContains, PointOrEdge},
    {contour::Contour, edge::Edge},
};

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

    pub fn transform(&self, transform: Transform) -> Self {
        match self {
            FaceSurface::Plane(plane) => FaceSurface::Plane(plane.transform(transform)),
            FaceSurface::Sphere(sphere) => FaceSurface::Sphere(match sphere.transform(transform) {
                SphereTransform::Ellipsoid() => panic!("Ellipsoid not implemented"),
                SphereTransform::Sphere(sphere) => sphere,
            }),
        }
    }

    pub fn contains_edge(&self, edge: &Edge) -> bool {
        if !self.surface().on_surface(*edge.start) {
            return false;
        }
        if !self.surface().on_surface(*edge.end) {
            return false;
        }
        match self {
            FaceSurface::Plane(plane) => {
                match &*edge.curve {
                    EdgeCurve::Line(line) => {
                        return plane.normal().dot(line.direction).abs() < EQ_THRESHOLD && plane.on_surface(line.basis);
                    }
                    EdgeCurve::Circle(circle) => {
                        return circle.normal.dot(plane.normal()) < EQ_THRESHOLD && plane.on_surface(circle.basis)
                    },
                    EdgeCurve::Ellipse(_) => todo!("Not implemented"),
                }
            }
            FaceSurface::Sphere(sphere) => {
                todo!("Not implemented");
            }
        }
    }

    pub fn intersect_edge_different_surface(&self, edge: &Edge) -> Vec<Point> {
        assert!(!self.contains_edge(edge));
        match self {
            FaceSurface::Plane(plane) => {
                match &*edge.curve {
                    EdgeCurve::Line(line) => {
                        if plane.normal().dot(line.direction).abs() < EQ_THRESHOLD {
                            return Vec::new();
                        }
                        let t = (plane.normal().dot(plane.basis) - plane.normal().dot(line.basis)) / plane.normal().dot(line.direction);
                        let p = line.basis + line.direction * t;
                        if !self.surface().on_surface(p) {
                            return Vec::new();
                        }
                        return vec![p];
                    }
                    EdgeCurve::Circle(circle) => {
                        if circle.normal.dot(plane.normal()) < EQ_THRESHOLD {
                            return Vec::new();
                        }
                        let t = (plane.normal().dot(plane.basis) - plane.normal().dot(circle.basis)) / plane.normal().dot(circle.normal);
                        let p = circle.basis + circle.normal * t;
                        if !self.surface().on_surface(p) {
                            return Vec::new();
                        }
                        return vec![p];
                    },
                    EdgeCurve::Ellipse(_) => todo!("Not implemented"),
                }
            }
            FaceSurface::Sphere(sphere) => {
                todo!("Not implemented");
            }
        }
    }

    pub fn neg(&self) -> FaceSurface {
        match self {
            FaceSurface::Plane(plane) => FaceSurface::Plane(plane.neg()),
            FaceSurface::Sphere(sphere) => FaceSurface::Sphere(sphere.neg()),
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
    pub boundaries: Vec<Contour>, // Coutner-clockwise
    pub surface: Rc<FaceSurface>,
}

pub enum FaceIntersection {
    Face(Face),
    Contour(Contour),
    Edge(Edge),
    Point(Point),
}

#[derive(Clone, Debug, PartialEq)]
pub enum FaceContainsPoint {
    Inside,
    OnEdge(Rc<Edge>),
    OnPoint(Rc<Point>),
    Outside,
}

pub enum FaceContainsEdge {
    Inside,
    OnBorderSameDir,
    OnBorderOppositeDir,
    Outside,
}

pub enum FaceEdgeIntersection {
    PointInside(Point),
    PointOnEdge(Point, Rc<Edge>),
    PointOnPoint(Rc<Point>),
    Edge(Edge)
}

#[derive(Debug)]
pub enum EdgeSplit {
    AinB(Rc<Edge>),
    AonBSameSide(Rc<Edge>),
    AonBOpSide(Rc<Edge>),
    AoutB(Rc<Edge>),
    BinA(Rc<Edge>),
    BonASameSide(Rc<Edge>),
    BonAOpSide(Rc<Edge>),
    BoutA(Rc<Edge>),
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
        todo!("Returns an inner point with where normal vector is well defined.");
    }

    pub fn edge_from_to(&self, from: Rc<Point>, to: Rc<Point>) -> Rc<Edge> {
        match &*self.surface {
            FaceSurface::Plane(p) => {
                let curve = p.curve_from_to(*from, *to);
                return Rc::new(Edge::new(
                    from.clone(),
                    to.clone(),
                    Rc::new(EdgeCurve::Line(curve)),
                    Direction::Increasing,
                ));
            }
            FaceSurface::Sphere(s) => {
                let curve = s.curve_from_to(*from, *to);
                return Rc::new(Edge::new(
                    from.clone(),
                    to.clone(),
                    Rc::new(EdgeCurve::Circle(curve)),
                    Direction::Increasing,
                ));
            }
        }
    }

    fn boundary_tangent(&self, p: Point) -> Point {
        for contour in self.boundaries.iter() {
            match contour.contains(p) {
                EdgeContains::Inside => return contour.tangent(p),
                EdgeContains::OnPoint(_) => panic!("Tangent undefined on point"),
                EdgeContains::Outside => continue,
            }
        }
        panic!("Point is not on boundary");
    }

    pub fn contains_point(&self, other: Point) -> FaceContainsPoint {
        // If the point is on the border, it is part of the set
        for edge in self.all_edges() {
            match edge.contains(other) {
                EdgeContains::Inside => return FaceContainsPoint::OnEdge(edge.clone()),
                EdgeContains::OnPoint(point) => return FaceContainsPoint::OnPoint(point),
                EdgeContains::Outside => continue,
            }
        }
        // Draw a line from the point to a random point on the border.
        // Use a midpoint to have a well defined tangent. At an Edge, the check is more complicated.
        let q: Point = self.boundaries[0].edges[0].get_midpoint(
            *self.boundaries[0].edges[0].start,
            *self.boundaries[0].edges[0].end,
        );
        let curve = self.edge_from_to(Rc::new(other), Rc::new(q));

        // Find the closest intersection point and check by using the face normal and the curve tangent if the intersection is from inside or outside.
        let mut closest_distance = self.surface.surface().distance(other, q);
        let curve_dir = curve.tangent(q);
        let normal = self.surface.surface().normal(q);
        let contour_dir = self.boundaries[0].tangent(q);
        let mut closest_intersect_from_inside = contour_dir.cross(normal).dot(curve_dir) > 0.0;
        for contour in self.boundaries.iter() {
            let intersections = contour.intersect_edge_different_curve(&*curve);
            for point in intersections {
                let distance = self.surface.surface().distance(other, point);
                if distance < closest_distance {
                    let curve_dir = curve.tangent(point);
                    let normal = self.surface.surface().normal(point);
                    let contour_dir = contour.tangent(point);
                    closest_distance = distance;
                    closest_intersect_from_inside = contour_dir.cross(normal).dot(curve_dir) > 0.0;
                }
            }
        }

        match closest_intersect_from_inside {
            true => FaceContainsPoint::Inside,
            false => FaceContainsPoint::Outside,
        }
    }

    // Checks if an edge is inside the face. The edge has to lie within the surface.
    // It is not allowed to intersect any boundary other than in start and end point.
    // The edge either has to be fully inside or fully outside or has to be completely on the border.
    pub fn contains_edge_same_surface(&self, other: &Edge) -> FaceContainsEdge {
        assert!(self.surface.contains_edge(other));
        for contour in self.boundaries.iter() {
            let mut edge_valid = false;
            for edge in contour.edges.iter() {
                if **edge == *other {
                    edge_valid = true;
                }
                if edge.neg() == *other {
                    edge_valid = true;
                }
            }
            if !edge_valid {
                assert!(!contour.intersect_edge_different_curve(other).is_empty());
            }
        }
        let q = other.get_midpoint(*other.start, *other.end);
        match self.contains_point(q) {
            FaceContainsPoint::Inside => FaceContainsEdge::Inside,
            FaceContainsPoint::OnEdge(edge) => {
                panic!("Edge is on border, but not on any contour: {:?}", edge);
            },
            FaceContainsPoint::OnPoint(point) => {
                panic!("Edge is on point, but not on any contour: {:?}", point);
            },
            FaceContainsPoint::Outside => {
                FaceContainsEdge::Outside
            },
        }
    }

    pub fn intersect_edge_different_surface(&self, other: &Edge) -> Vec<Point> {
        assert!(!self.surface.contains_edge(other));
        let mut intersections = self.surface.intersect_edge_different_surface(other);
        intersections.drain(..).filter(|p| self.contains_point(*p) == FaceContainsPoint::Inside).collect()
    }

    pub fn split_parts<F>(&self, other: &Face, filter: F) -> Face
    where
        F: Fn(&EdgeSplit) -> bool,
    {
        assert!(self.surface == other.surface);

        // First split contours such that each edge either fully overlaps or does not intersect another one at all.
        let mut intersections = Vec::<Point>::new();
        for edge in self.boundaries.iter() {
            for other_edge in other.boundaries.iter() {
                intersections.extend(edge.find_split_points(other_edge));
            }
        }
        for int in intersections.iter() {
            println!("Intersection: {:?}", int);
        }

        let mut contours_self = self.boundaries.clone();
        let mut contours_other = other.boundaries.clone();

        for vert in intersections {
            contours_self = contours_self
                .into_iter()
                .map(|contour| contour.split_if_necessary(&vert))
                .collect();
            contours_other = contours_other
                .into_iter()
                .map(|contour| contour.split_if_necessary(&vert))
                .collect();
        }

        let mut edges = contours_self
            .into_iter()
            .map(|contour| {
                return contour
                    .edges
                    .into_iter()
                    .map(|edge| match other.contains_edge_same_surface(&edge) {
                        FaceContainsEdge::Inside => EdgeSplit::AinB(edge),
                        FaceContainsEdge::OnBorderSameDir => EdgeSplit::AonBSameSide(edge),
                        FaceContainsEdge::OnBorderOppositeDir => EdgeSplit::AonBOpSide(edge),
                        FaceContainsEdge::Outside => EdgeSplit::AoutB(edge),
                    })
                    .collect::<Vec<EdgeSplit>>();
            })
            .chain(contours_other.into_iter().map(|contour| {
                contour
                    .edges
                    .into_iter()
                    .map(|edge| match self.contains_edge_same_surface(&edge) {
                        FaceContainsEdge::Inside => EdgeSplit::BinA(edge),
                        FaceContainsEdge::OnBorderSameDir => EdgeSplit::BonASameSide(edge),
                        FaceContainsEdge::OnBorderOppositeDir => EdgeSplit::BonAOpSide(edge),
                        FaceContainsEdge::Outside => EdgeSplit::BoutA(edge),
                    })
                    .collect::<Vec<EdgeSplit>>()
            }))
            .flatten()
            .filter(filter)
            .map(|e| match e {
                EdgeSplit::AinB(edge) => edge,
                EdgeSplit::AonBSameSide(edge) => edge,
                EdgeSplit::AonBOpSide(edge) => edge,
                EdgeSplit::AoutB(edge) => edge,
                EdgeSplit::BinA(edge) => edge,
                EdgeSplit::BonASameSide(edge) => edge,
                EdgeSplit::BonAOpSide(edge) => edge,
                EdgeSplit::BoutA(edge) => edge,
            })
            .collect::<Vec<Rc<Edge>>>();

        for edge in edges.iter() {
            println!("Edge: {:?}", edge);
        }

        // Now find all the contours
        let mut contours = Vec::<Contour>::new();
        while let Some(current_edge) = edges.pop() {
            let mut new_contour = vec![current_edge];
            loop {
                let next_i = edges.iter().position(|edge| {
                    edge.start == new_contour[new_contour.len() - 1].end
                        || edge.end == new_contour[new_contour.len() - 1].end
                });
                match next_i {
                    Some(i) => {
                        if edges[i].start == new_contour[new_contour.len() - 1].end {
                            new_contour.push(edges.remove(i));
                        } else {
                            new_contour.push(Rc::new(edges.remove(i).neg()));
                        }
                    }
                    None => {
                        assert!(new_contour[0].start == new_contour[new_contour.len() - 1].end);
                        contours.push(Contour::new(new_contour));
                        break;
                    }
                }
            }
        }

        return Face::new(contours, self.surface.clone());
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

    pub fn surface_union(&self, other: &Face) -> Face {
        self.split_parts(other, |mode| match mode {
            EdgeSplit::AinB(_) => false,
            EdgeSplit::AonBSameSide(_) => true,
            EdgeSplit::AonBOpSide(_) => false,
            EdgeSplit::AoutB(_) => true,
            EdgeSplit::BinA(_) => false,
            EdgeSplit::BonASameSide(_) => false,
            EdgeSplit::BonAOpSide(_) => false,
            EdgeSplit::BoutA(_) => true,
        })
    }

    pub fn surface_intersection(&self, other: &Face) -> Face {
        self.split_parts(other, |mode| match mode {
            EdgeSplit::AinB(_) => true,
            EdgeSplit::AonBSameSide(_) => true,
            EdgeSplit::AonBOpSide(_) => false,
            EdgeSplit::AoutB(_) => false,
            EdgeSplit::BinA(_) => true,
            EdgeSplit::BonASameSide(_) => false,
            EdgeSplit::BonAOpSide(_) => false,
            EdgeSplit::BoutA(_) => false,
        })
    }

    pub fn surface_difference(&self, other: &Face) -> Face {
        return self.surface_intersection(&other.neg());
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