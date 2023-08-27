use std::rc::Rc;

use geop_geometry::{surfaces::{plane::Plane, sphere::Sphere, surface::Surface}, points::point::Point};

use crate::topology::edge::{Direction, EdgeCurve, EdgeIntersection};

use super::{{contour::Contour, edge::Edge}, vertex::Vertex, edge::EdgeContains};


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
    Vertex(Vertex)
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
    OnBorder,
    Outside,
    Wiggeling,
}

pub enum EdgeSplit {
    AinB(Rc<Edge>),
    AonB(Rc<Edge>),
    AoutB(Rc<Edge>),
    BinA(Rc<Edge>),
    BonA(Rc<Edge>),
    BoutA(Rc<Edge>),
}

// Implements a Face. A Face is bounded by the outer_loop and might have holes in inner_loops.
// outer_loop has to be clockwise, if the face is looked at from normal direction (normal facing towards you).
// inner_loops have to be counter-clockwise, if the face is looked at from normal direction (normal facing towards you).
// The contours are not allowed to intersect in any way. Keep in mind that a vertex is not considered an intersection, hence it is allowed that the contours touch each other at vertices.
impl Face {
    pub fn new(boundaries: Vec<Contour>, surface: Rc<FaceSurface>) -> Face {
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
                return Rc::new(Edge::new(from.clone(), to.clone(), Rc::new(EdgeCurve::Line(curve)), Direction::Increasing));
            },
            FaceSurface::Sphere(s) => {
                let curve = s.curve_from_to(*from.point, *to.point);
                return Rc::new(Edge::new(from.clone(), to.clone(), Rc::new(EdgeCurve::Circle(curve)), Direction::Increasing));
            },
        }
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
        let q = self.boundaries[0].edges[0].start.clone();
        let curve = self.edge_from_to(&Vertex::new(Rc::new(other)), &q);

        // Find the closest intersection point
        let mut closest_distance = std::f64::INFINITY;
        let mut closest_intersect_from_inside = false;
        for contour in self.boundaries.iter() {
            let intersections = contour.intersect_edge(&*curve);
            for vertex in intersections {
                let distance = self.surface.surface().distance(other, *vertex.point);
                if distance < closest_distance {
                    let curve_dir = curve.tangent(*vertex.point);
                    let normal = self.surface.surface().normal(*vertex.point);
                    let curve_prod = contour.tangent(*vertex.point);
                    closest_distance = distance;
                    closest_intersect_from_inside = curve_dir.cross(normal).dot(curve_prod) > 0.0; // TODO: Check this is correct
                }
            }
        }

        match closest_intersect_from_inside {
            true => FaceContainsPoint::Inside,
            false => FaceContainsPoint::Outside
        }
    }

    // Checks if an edge is inside the face. This guarantees that the edge is not touching any curves. The start and end point of the edge can be on the border, since they are not considered a part of the edge.
    pub fn contains_edge(&self, other: &Edge) -> FaceContainsEdge {
        let mut intersections = Vec::<Vertex>::new();
        for contour in self.boundaries.iter() {
            let intersection = contour.intersect_edge(other);
            intersections.extend(intersection);
        }
        
        let mut part_inside = false;
        let mut part_outside = false;
        for i in -1..intersections.len() as isize {
            let prev = if i == -1 { &other.start } else { &intersections[i as usize] };
            let next = if i == intersections.len() as isize - 1 { &other.end } else { &intersections[(i + 1) as usize] };
            let p = other.get_midpoint(*prev.point, *next.point);
            match self.contains_point(p) {
                FaceContainsPoint::Inside => part_inside = true,
                FaceContainsPoint::Outside => part_outside = true,
                FaceContainsPoint::OnEdge => (),
                FaceContainsPoint::OnVertex => (),
            }
        }
        match (part_inside, part_outside) {
            (true, true) => FaceContainsEdge::Wiggeling,
            (true, false) => FaceContainsEdge::Inside,
            (false, true) => FaceContainsEdge::Outside,
            (false, false) => FaceContainsEdge::OnBorder,
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

    
    pub fn split_parts<F>(&self, other: &Face, filter: F) -> Face where F: Fn(&EdgeSplit) -> bool {
        assert!(self.surface == other.surface);

        let mut intersections = Vec::<Vertex>::new();
        for edge in self.all_edges() {
            for other_edge in other.all_edges() {
                let intersection = edge.intersections(&other_edge);
                for int in intersection {
                    match int {
                        EdgeIntersection::Vertex(vertex) => intersections.push(vertex),
                        EdgeIntersection::Edge(edge) => intersections.push(edge.start),
                    }
                }
            }
        }

        let mut contours_self = self.boundaries.clone();
        let mut contours_other = other.boundaries.clone();

        for vert in intersections {
            contours_self = contours_self.into_iter().map(|contour| contour.split_if_necessary(&vert)).collect();
            contours_other = contours_other.into_iter().map(|contour| contour.split_if_necessary(&vert)).collect();
        }

        let mut edges = contours_self.into_iter().map(|contour| {
            return contour.edges.into_iter().map(|edge| {
                match other.contains_edge(&edge) {
                    FaceContainsEdge::Inside => EdgeSplit::AinB(edge),
                    FaceContainsEdge::OnBorder => EdgeSplit::AonB(edge),
                    FaceContainsEdge::Outside => EdgeSplit::AoutB(edge),
                    FaceContainsEdge::Wiggeling => panic!("Should not happen because contours were split at all intersections"),
                }
            }).collect::<Vec<EdgeSplit>>()
        }).chain(
            contours_other.into_iter().map(|contour| {
                other.all_edges().into_iter().map(|edge| {
                    match self.contains_edge(&edge) {
                        FaceContainsEdge::Inside => EdgeSplit::BinA(edge),
                        FaceContainsEdge::OnBorder => EdgeSplit::BonA(edge),
                        FaceContainsEdge::Outside => EdgeSplit::BoutA(edge),
                        FaceContainsEdge::Wiggeling => panic!("Should not happen because contours were split at all intersections"),
                    }
                }).collect::<Vec<EdgeSplit>>()
        })).flatten().filter(filter).map(|e| {
            match e {
                EdgeSplit::AinB(edge) => edge,
                EdgeSplit::AonB(edge) => edge,
                EdgeSplit::AoutB(edge) => edge,
                EdgeSplit::BinA(edge) => edge,
                EdgeSplit::BonA(edge) => edge,
                EdgeSplit::BoutA(edge) => edge,
            }
        }).collect::<Vec<Rc<Edge>>>();

        // Now find all the contours
        let mut contours = Vec::<Contour>::new();
        while let Some(current_edge) = edges.pop() {
            let mut new_contour = vec![current_edge];
            loop {
                let next_i = edges.iter().position(|edge| edge.start == new_contour[new_contour.len() - 1].end);
                match next_i {
                    Some(i) => {
                        new_contour.push(edges.remove(i));
                    },
                    None => {
                        assert!(new_contour[0].start == new_contour[new_contour.len() - 1].end);
                        contours.push(Contour::new(new_contour));
                        break;
                    },
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
        self.split_parts(other, |mode| {
            match mode {
                EdgeSplit::AinB(_) => false,
                EdgeSplit::AonB(_) => true,
                EdgeSplit::AoutB(_) => true,
                EdgeSplit::BinA(_) => false,
                EdgeSplit::BonA(_) => false,
                EdgeSplit::BoutA(_) => true,
            }
        })
    }

    pub fn surface_intersection(&self, other: &Face) -> Face {
        self.split_parts(other, |mode| {
            match mode {
                EdgeSplit::AinB(_) => true,
                EdgeSplit::AonB(_) => true,
                EdgeSplit::AoutB(_) => false,
                EdgeSplit::BinA(_) => true,
                EdgeSplit::BonA(_) => false,
                EdgeSplit::BoutA(_) => false,
            }
        })
    }

    pub fn surface_difference(&self, other: &Face) -> Face {
        self.split_parts(other, |mode| {
            match mode {
                EdgeSplit::AinB(_) => false,
                EdgeSplit::AonB(_) => true,
                EdgeSplit::AoutB(_) => true,
                EdgeSplit::BinA(_) => true,
                EdgeSplit::BonA(_) => false,
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