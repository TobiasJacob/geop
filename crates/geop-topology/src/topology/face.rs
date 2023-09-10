use std::rc::Rc;

use geop_geometry::{
    points::point::Point,
    surfaces::{plane::Plane, sphere::Sphere, surface::Surface},
};

use crate::topology::edge::{Direction, EdgeCurve, EdgeIntersection};

use super::{
    edge::EdgeContains,
    vertex::Vertex,
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
    Vertex(Vertex),
}

#[derive(Clone, Debug, PartialEq)]
pub enum FaceContainsPoint {
    Inside,
    OnEdge,
    OnVertex,
    Outside,
}

pub enum FaceContainsEdge {
    Inside,
    OnBorderSameDir,
    OnBorderOppositeDir,
    Outside,
    Wiggeling,
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
// The contours are not allowed to intersect in any way. Keep in mind that a vertex is not considered an intersection, hence it is allowed that the contours touch each other at vertices.
impl Face {
    pub fn new(boundaries: Vec<Contour>, surface: Rc<FaceSurface>) -> Face {
        assert!(boundaries.len() > 0, "Face must have at least one boundary");
        Face {
            boundaries,
            surface,
        }
    }

    pub fn all_vertices(&self) -> Vec<Vertex> {
        let mut vertices = Vec::<Vertex>::new();

        for contour in self.boundaries.iter() {
            vertices.extend(contour.all_vertices());
        }
        return vertices;
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

    pub fn edge_from_to(&self, from: &Vertex, to: &Vertex) -> Rc<Edge> {
        match &*self.surface {
            FaceSurface::Plane(p) => {
                let curve = p.curve_from_to(*from.point, *to.point);
                return Rc::new(Edge::new(
                    from.clone(),
                    to.clone(),
                    Rc::new(EdgeCurve::Line(curve)),
                    Direction::Increasing,
                ));
            }
            FaceSurface::Sphere(s) => {
                let curve = s.curve_from_to(*from.point, *to.point);
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
                EdgeContains::OnVertex => panic!("Tangent undefined on vertex"),
                EdgeContains::Outside => continue,
            }
        }
        panic!("Point is not on boundary");
    }

    pub fn contains_point(&self, other: Point) -> FaceContainsPoint {
        // If the point is on the border, it is part of the set
        for edge in self.all_edges() {
            match edge.contains(other) {
                EdgeContains::Inside => return FaceContainsPoint::OnEdge,
                EdgeContains::OnVertex => return FaceContainsPoint::OnVertex,
                EdgeContains::Outside => continue,
            }
        }
        // Draw a line from the point to a random point on the border.
        // Use a midpoint to have a well defined tangent. At an Edge, the check is more complicated.
        let q: Point = self.boundaries[0].edges[0].get_midpoint(
            *self.boundaries[0].edges[0].start.point,
            *self.boundaries[0].edges[0].end.point,
        );
        let curve = self.edge_from_to(&Vertex::new(Rc::new(other)), &Vertex::new(Rc::new(q)));

        // Find the closest intersection point
        let mut closest_distance = self.surface.surface().distance(other, q);
        let curve_dir = curve.tangent(q);
        let normal = self.surface.surface().normal(q);
        let contour_dir = self.boundaries[0].tangent(q);
        let mut closest_intersect_from_inside = contour_dir.cross(normal).dot(curve_dir) > 0.0;
        for contour in self.boundaries.iter() {
            let edge_intersections = contour.intersect_edge(&*curve);
            let mut intersections = Vec::<Vertex>::new();
            for intersection in edge_intersections {
                match intersection {
                    EdgeIntersection::Vertex(vertex) => {
                        intersections.push(vertex);
                    }
                    EdgeIntersection::Edge(edge) => {
                        intersections.push(edge.start);
                        intersections.push(edge.end);
                    }
                }
            }
            for vertex in intersections {
                let distance = self.surface.surface().distance(other, *vertex.point);
                if distance < closest_distance {
                    let curve_dir = curve.tangent(*vertex.point);
                    let normal = self.surface.surface().normal(*vertex.point);
                    let contour_dir = contour.tangent(*vertex.point);
                    closest_distance = distance;
                    closest_intersect_from_inside = contour_dir.cross(normal).dot(curve_dir) > 0.0;
                }
            }
        }

        // println!("Distance: {}", closest_distance);
        // println!("Closest intersect from inside: {}", closest_intersect_from_inside);
        match closest_intersect_from_inside {
            true => FaceContainsPoint::Inside,
            false => FaceContainsPoint::Outside,
        }
    }

    // Checks if an edge is inside the face. This guarantees that the edge is not touching any curves. The start and end point of the edge can be on the border, since they are not considered a part of the edge.
    pub fn contains_edge(&self, other: &Edge) -> FaceContainsEdge {
        // println!("Other: {:?}", other);
        let mut intersections = Vec::<Vertex>::new();
        for contour in self.boundaries.iter() {
            let intersection = contour.intersect_edge(other);
            for int in intersection {
                match int {
                    EdgeIntersection::Vertex(vertex) => intersections.push(vertex),
                    EdgeIntersection::Edge(edge) => {
                        intersections.push(edge.start.clone());
                        intersections.push(edge.end.clone());
                    }
                }
            }
        }

        let mut part_inside = false;
        let mut part_outside = false;
        for i in -1..intersections.len() as isize {
            let prev = if i == -1 {
                &other.start
            } else {
                &intersections[i as usize]
            };
            let next = if i == intersections.len() as isize - 1 {
                &other.end
            } else {
                &intersections[(i + 1) as usize]
            };
            let p = other.get_midpoint(*prev.point, *next.point);
            match self.contains_point(p) {
                FaceContainsPoint::Inside => part_inside = true,
                FaceContainsPoint::Outside => part_outside = true,
                FaceContainsPoint::OnEdge => (),
                FaceContainsPoint::OnVertex => (),
            }
        }

        let p = other.get_midpoint(*other.start.point, *other.end.point);

        match (part_inside, part_outside) {
            (true, true) => FaceContainsEdge::Wiggeling,
            (true, false) => FaceContainsEdge::Inside,
            (false, true) => FaceContainsEdge::Outside,
            (false, false) => match self.boundary_tangent(p).dot(other.tangent(p)) > 0.0 {
                true => FaceContainsEdge::OnBorderSameDir,
                false => FaceContainsEdge::OnBorderOppositeDir,
            },
        }
    }

    // pub fn intersect(&self, other: &Face) -> Vec<FaceIntersection> {
    //     todo!("Implement intersect");
    // }

    // pub fn subsurface(&self, cutting_contour: Contour) -> Rc<Face> {
    //     let mut contours_self = self.inner_loops.clone();
    //     contours_self.push(self.outer_loop.clone());

    //     let new_contours = cutting_contour.remesh_multiple(contours_self.as_slice());

    //     let (ccw_conts, cw_conts): (Vec<Contour>, Vec<Contour>) = new_contours.into_iter().partition(|l| self.contour_direction(l) == ContourDirection::CounterClockwise);
    //     assert!(ccw_conts.len() == 2, "Expected 2 counter clockwise edge loops, found {}", ccw_conts.len());
    //     let (outer_loops, invalid_loops): (Vec<Contour>, Vec<Contour>) = ccw_conts.into_iter().partition(|l| {
    //         for edge in &l.edges {
    //             if !self.contains_edge(edge) {
    //                 return false;
    //             }
    //         }
    //         true
    //     });
    //     assert!(outer_loops.len() == 1, "Expected 1 counter clockwise edge loop, found {}", outer_loops.len());
    //     let outer_loop = outer_loops[0].clone();
    //     let inner_loops = cw_conts;

    //     Rc::new(Face::new(
    //         outer_loop,
    //         inner_loops,
    //         self.surface.clone(),
    //     ))
    // }

    pub fn split_parts<F>(&self, other: &Face, filter: F) -> Face
    where
        F: Fn(&EdgeSplit) -> bool,
    {
        assert!(self.surface == other.surface);

        let mut intersections = Vec::<Vertex>::new();
        for edge in self.boundaries.iter() {
            for other_edge in other.boundaries.iter() {
                for intersection in edge.intersect_contour(&other_edge) {
                    match intersection {
                        EdgeIntersection::Vertex(vertex) => intersections.push(vertex),
                        EdgeIntersection::Edge(edge) => {
                            intersections.push(edge.start.clone());
                            intersections.push(edge.end.clone());
                        }
                    }
                }
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
                    .map(|edge| match other.contains_edge(&edge) {
                        FaceContainsEdge::Inside => EdgeSplit::AinB(edge),
                        FaceContainsEdge::OnBorderSameDir => EdgeSplit::AonBSameSide(edge),
                        FaceContainsEdge::OnBorderOppositeDir => EdgeSplit::AonBOpSide(edge),
                        FaceContainsEdge::Outside => EdgeSplit::AoutB(edge),
                        FaceContainsEdge::Wiggeling => panic!(
                            "Should not happen because contours were split at all intersections"
                        ),
                    })
                    .collect::<Vec<EdgeSplit>>();
            })
            .chain(contours_other.into_iter().map(|contour| {
                contour
                    .edges
                    .into_iter()
                    .map(|edge| match self.contains_edge(&edge) {
                        FaceContainsEdge::Inside => EdgeSplit::BinA(edge),
                        FaceContainsEdge::OnBorderSameDir => EdgeSplit::BonASameSide(edge),
                        FaceContainsEdge::OnBorderOppositeDir => EdgeSplit::BonAOpSide(edge),
                        FaceContainsEdge::Outside => EdgeSplit::BoutA(edge),
                        FaceContainsEdge::Wiggeling => panic!(
                            "Should not happen because contours were split at all intersections"
                        ),
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

    // pub fn split_parts(&self, other: &Face) -> Option<(Face, Vec<Face>)> {
    //     assert!(self.surface == other.surface);

    //     let mut contours_self = self.boundaries.clone();
    //     contours_self.push(self.outer_loop.clone());

    //     let mut contours_other = other.boundaries.clone();
    //     contours_other.push(other.outer_loop.clone());

    //     let remeshed = remesh_multiple_multiple(contours_self.as_slice(), contours_other.as_slice());
    //     let (mut ccw_conts, mut cw_conts): (Vec<Contour>, Vec<Contour>) = remeshed.into_iter().partition(|l| self.contour_direction(l) == ContourDirection::CounterClockwise);

    //     // Now its simple.
    //     // All clockwise edge loops are caveties in the union.
    //     // The largest counter clockwise edge loop is the outer loop of the union.
    //     // All remaining counter clockwise edge loops are intersections.
    //     let mut i_max = 0;
    //     let mut result_valid = false;
    //     for (i, ccw_cont) in ccw_conts.iter().enumerate() {
    //         let temp_face = Face::new(
    //             ccw_cont.clone(),
    //             vec![],
    //             self.surface.clone(),
    //         );
    //         if temp_face.contains_contour(&ccw_conts[i_max]) {
    //             i_max = i;
    //             result_valid = true;
    //         }
    //     }

    //     // This means the Faces did not intersect
    //     if !result_valid {
    //         return None;
    //     }

    //     let union_contour = ccw_conts.remove(i_max);

    //     let mut intersecions = Vec::<Face>::new();
    //     for ccw_cont in ccw_conts {
    //         let mut face = Face::new(
    //             ccw_cont.clone(),
    //             vec![],
    //             self.surface.clone(),
    //         );
    //         let (inner_loops, cw_conts_new): (Vec<Contour>, Vec<Contour>) = cw_conts.into_iter().partition(|l| face.contains_contour(l));
    //         cw_conts = cw_conts_new;
    //         face.boundaries = inner_loops;
    //         intersecions.push(face);
    //     }

    //     let union_face = Face::new(
    //         union_contour,
    //         cw_conts,
    //         self.surface.clone(),
    //     );

    //     Some((union_face, intersecions))
    // }

    pub fn neg(&self) -> Face {
        Face {
            boundaries: self.boundaries.iter().map(|l| l.neg()).collect(),
            surface: self.surface.clone(),
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
        self.split_parts(other, |mode| {
            println!("EdgeSplit: {:?}", mode);
            match mode {
                EdgeSplit::AinB(_) => false,
                EdgeSplit::AonBSameSide(_) => false,
                EdgeSplit::AonBOpSide(_) => true,
                EdgeSplit::AoutB(_) => true, // Maybe...
                EdgeSplit::BinA(_) => true,
                EdgeSplit::BonASameSide(_) => false,
                EdgeSplit::BonAOpSide(_) => false,
                EdgeSplit::BoutA(_) => false,
            }
        })
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
