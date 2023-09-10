use std::{rc::Rc, vec};

use geop_geometry::points::point::Point;

use crate::topology::vertex::Vertex;

use super::edge::{Edge, EdgeContains, EdgeIntersection};

#[derive(Debug, Clone)]
pub struct Contour {
    pub edges: Vec<Rc<Edge>>,
}

fn pop_next_segment(
    edges_self: &mut Vec<Rc<Edge>>,
    edges_other: &mut Vec<Rc<Edge>>,
    next_segment_is_self: bool,
    start: &Vertex,
) -> Option<Rc<Edge>> {
    let relevant_segments: &mut Vec<Rc<Edge>> = if next_segment_is_self {
        edges_self
    } else {
        edges_other
    };
    for (i, edge) in relevant_segments.iter().enumerate() {
        if edge.start == *start {
            let edge = relevant_segments.remove(i);
            return Some(edge);
        }
    }
    None
}

// An Contour is a closed loop of edges which is not self intersecting (because otherwise project would not be defined for self intersection point).
// It has a defined inside and outside, which is determined by the direction of the edges.
// The vertices of edges are not part of the contour, e.g. the intersection of two contours at the same vertex is empty.
// Keep in mind that the contour is still closed, but the vertices are "next to" the edges, not "part of" the edges, because otherwise two neighbouring edges would overlap at the vertex, making things a lot more complicated.
impl Contour {
    pub fn new(edges: Vec<Rc<Edge>>) -> Contour {
        for i in 0..edges.len() {
            let edge = edges[i].clone();
            let next_edge = edges[(i + 1) % edges.len()].clone();
            assert!(edge.end == next_edge.start);
        }
        assert!(edges.len() > 0);
        assert!(edges[0].start == edges[edges.len() - 1].end);
        Contour { edges }
    }

    pub fn all_vertices(&self) -> Vec<Vertex> {
        let mut vertices = Vec::<Vertex>::new();
        for edge in self.edges.iter() {
            vertices.push(edge.start.clone());
        }
        vertices.push(self.edges.last().unwrap().end.clone());
        return vertices;
    }

    pub fn neg(&self) -> Contour {
        let edges = self
            .edges
            .iter()
            .rev()
            .map(|e| Rc::new(e.neg()))
            .collect::<Vec<Rc<Edge>>>();
        Contour::new(edges)
    }

    pub fn contains(&self, point: Point) -> EdgeContains {
        for (i, edge) in self.edges.iter().enumerate() {
            let contains = edge.contains(point);
            match contains {
                EdgeContains::OnVertex => {
                    return EdgeContains::OnVertex;
                }
                EdgeContains::Inside => {
                    return EdgeContains::Inside;
                }
                EdgeContains::Outside => {}
            }
        }
        return EdgeContains::Outside;
    }

    // Returns an edge that contains the point, or None if the point is not on the contour.
    // It can also be the start or the end point of an edge, hence, if this function is used, take special care of the case where this case.
    fn get_edge_index(&self, point: Point) -> Option<usize> {
        for (i, edge) in self.edges.iter().enumerate() {
            if edge.contains(point) != EdgeContains::Outside {
                return Some(i);
            }
        }
        None
    }

    pub fn tangent(&self, p: Point) -> Point {
        assert!(self.contains(p) != EdgeContains::Outside);
        let i = self.get_edge_index(p).unwrap();
        return self.edges[i].tangent(p);
    }

    // Checks if the contour contains the point, and if so, splits the edge into two edges.
    // It is guaranteed that this happens in order, meaning that the first edge returned will contain the start point of the original edge, and the second edge will contain the end point of the original edge.
    pub fn split_if_necessary(&self, other: &Vertex) -> Contour {
        if self.contains(*other.point) != EdgeContains::Inside {
            return self.clone();
        }

        let edge_index = self.get_edge_index(*other.point).unwrap();
        let edge = self.edges[edge_index].split_if_necessary(other);
        assert!(edge.len() == 2);

        let mut edges = self.edges.clone();
        edges.remove(edge_index);
        edges.insert(edge_index, edge[1].clone());
        edges.insert(edge_index, edge[0].clone());

        return Contour::new(edges);
    }

    // Gets the subcurve between these two points. It is guaranteed that there will be no zero length edges.
    pub fn get_subcurve(&self, start: Vertex, end: Vertex) -> Vec<Rc<Edge>> {
        assert!(start != end);

        let mut result = Vec::<Rc<Edge>>::new();
        let start_i = self
            .get_edge_index(*start.point)
            .expect("Start point has to be on edge");
        let end_i = self
            .get_edge_index(*end.point)
            .expect("End point has to be on edge");

        if start_i == end_i {
            let edge = Rc::new(Edge::new(
                start.clone(),
                end.clone(),
                self.edges[start_i].curve.clone(),
                self.edges[start_i].direction,
            ));
            result.push(edge);
        }

        let mut edge = &self.edges[start_i];
        if start != edge.end {
            result.push(Rc::new(Edge::new(
                start.clone(),
                edge.end.clone(),
                edge.curve.clone(),
                edge.direction,
            )));
        }
        for i in start_i + 1..end_i {
            edge = &self.edges[i % self.edges.len()];
            result.push(edge.clone());
        }
        edge = &self.edges[end_i % self.edges.len()];
        if edge.start != end {
            result.push(Rc::new(Edge::new(
                edge.start.clone(),
                end.clone(),
                edge.curve.clone(),
                edge.direction,
            )));
        }
        result
    }

    pub fn intersect_edge(&self, other: &Edge) -> Vec<EdgeIntersection> {
        let mut intersections = Vec::<EdgeIntersection>::new();
        for edge in self.edges.iter() {
            let edge_intersections = edge.intersections(other);
            // for intersection in edge_intersections.iter() {
            //     match intersection {
            //         EdgeIntersection::Vertex(vertex) => {
            //             assert!(other.contains(*vertex.point) != EdgeContains::Outside);
            //         },
            //         EdgeIntersection::Edge(edge) => {
            //             println!("Edge: {:?} - {:?}", edge.start.point, edge.end.point);
            //             assert!(other.contains(*edge.start.point) != EdgeContains::Outside);
            //             assert!(other.contains(*edge.end.point) != EdgeContains::Outside);
            //         }
            //     }
            // }
            intersections.extend(edge_intersections.into_iter());
        }

        intersections
    }

    // Gets all intersections between this contour and another contour.
    // Vertices of Edges are not considered as part of the contour, hence, the intersection of two contours at the same vertex is empty.
    pub fn intersect_contour(&self, other: &Contour) -> Vec<EdgeIntersection> {
        let mut intersections = Vec::<EdgeIntersection>::new();
        for edge_other in other.edges.iter() {
            intersections.extend(self.intersect_edge(edge_other).into_iter());
        }
        intersections
    }

    // Takes 2 Contours and connects them at intersecting points with new vertices if there are some.
    // If there are overlapping edges, there will be a vertex for the beginning and the end of the overlapping edges, and a connecting edge for each loop.
    // If there are no intersections, the outer vector will have length 1.
    // fn split_if_necessary(&self, other: &Contour) -> (Contour, Contour) {
    //     let split_verts: Vec<Vertex> = self.intersect(&other);

    //     let mut edges_self = self.edges.clone();
    //     let mut edges_other = other.edges.clone();
    //     for vert in split_verts.iter() {
    //         let mut new_edges_self = Vec::<Rc<Edge>>::new();
    //         for edge in edges_self.iter() {
    //             let new_edges = edge.split_if_necessary(vert);
    //             new_edges_self.extend(new_edges);
    //         }
    //         edges_self = new_edges_self;

    //         let mut new_edges_other = Vec::<Rc<Edge>>::new();
    //         for edge in edges_other.iter() {
    //             let new_edges = edge.split_if_necessary(vert);
    //             new_edges_other.extend(new_edges);
    //         }
    //         edges_other = new_edges_other;
    //     }

    //     (Contour::new(edges_self), Contour::new(edges_other))
    // }

    // Splits this edge loop with another edge loop.
    // This makes sure that the resulting edge loops are closed and do not intersect each other anymore.
    // Neighbouring edge loops will share the same end points for the edges, and the two neighbouring edges will face opposite direction.
    // If edge loops are not overlapping, the result will be two edge loops.
    // pub fn remesh(&self, other: &Contour) -> Vec<Contour> {
    //     let (segments_self, segments_other) = self.split_if_necessary(other);
    //     let mut edges_self = segments_self.edges;
    //     let mut edges_other = segments_other.edges;
    //     for edge in edges_self.iter() {
    //         println!("Edge: {:?} - {:?}", edge.start.point, edge.end.point);
    //     }

    //     for edge in edges_other.iter() {
    //         println!("Edge: {:?} - {:?}", edge.start.point, edge.end.point);
    //     }

    //     let mut contours = Vec::new();
    //     let mut next_segment_is_self;
    //     loop {
    //         let mut contour: Vec<Rc<Edge>> = Vec::new();
    //         if edges_self.len() > 0 {
    //             contour.push(edges_self.pop().unwrap());
    //             next_segment_is_self = false;
    //         } else if edges_other.len() > 0 {
    //             contour.push(edges_other.pop().unwrap());
    //             next_segment_is_self = true;
    //         } else {
    //             break;
    //         }
    //         while contour[0].start != contour[contour.len() - 1].end {
    //             let end_point = &contour[contour.len() - 1].end;
    //             let next_segment = pop_next_segment(&mut edges_self, &mut edges_other, next_segment_is_self, &end_point);
    //             match next_segment {
    //                 Some(next_segment) => {
    //                     contour.push(next_segment);
    //                     next_segment_is_self = !next_segment_is_self;
    //                 },
    //                 None => {
    //                     let next_segment = pop_next_segment(&mut edges_self, &mut edges_other, !next_segment_is_self, &end_point).expect("Edge has to be in one of the edge loops");
    //                     contour.push(next_segment);
    //                 }
    //             }
    //         }
    //         contours.push(Contour::new(contour));
    //     }

    //     contours
    // }

    // It is important that the Contours in other do not overlap. This makes sure, that remeshing them with themselfs will not change anything.
    // pub fn remesh_multiple(&self, other: &[Contour]) -> Vec<Contour> {
    //     let mut result = vec![self.clone()];

    //     // Since all contours in other do not overlap, we can safely remesh them with each other.
    //     // It is guaranteed, that whenever we apply a remesh iteration, result will only intersect with other in places where self already intersected with other.
    //     // Hence, remeshing it again will keep the result untouched.
    //     for other_contour in other {
    //         let mut new_result = Vec::<Contour>::new();
    //         for contour in result {
    //             let new_contours = contour.remesh(other_contour);
    //             new_result.extend(new_contours.into_iter());
    //         }
    //         result = new_result;
    //     }

    //     result
    // }

    // // If no intersection is there, the result is None. Otherwise we can be sure that the result is a single edge loop.
    // pub fn union(&self, other: &Contour) -> Option<Contour> {
    //     let mut contours = self.remesh_self_other(other)?;

    //     // Find an outer vertex
    //     let mut outer_edge = &contours[0].edges[0];
    //     for contour in contours.iter() {
    //         for edge in contour.edges.iter() {

    //             match edge.start.point.x.partial_cmp(&outer_edge.start.point.x) {
    //                 Some(std::cmp::Ordering::Less) => {
    //                     outer_edge = edge;
    //                 },
    //                 _ => {
    //                     match edge.start.point.y.partial_cmp(&outer_edge.start.point.y) {
    //                         Some(std::cmp::Ordering::Less) => {
    //                             outer_edge = edge;
    //                         },
    //                         _ => {
    //                             match edge.start.point.z.partial_cmp(&outer_edge.start.point.z) {
    //                                 Some(std::cmp::Ordering::Less) => {
    //                                     outer_edge = edge;
    //                                 },
    //                                 _ => {}
    //                             }
    //                         }
    //                     }
    //                 }
    //             };
    //         }
    //     }

    //     // Find the edge loop which contain the outer vertex.
    //     let outer_contour_index = contours.iter().position(|contour| contour.edges.contains(&outer_edge)).unwrap();

    //     Some(contours.swap_remove(outer_contour_index))
    // }

    // Avoid using these functions
    pub fn point_at(&self, u: f64) -> Point {
        let u = u * self.edges.len() as f64;
        let i = u.floor() as usize;
        let u = u - i as f64;
        let i = i % self.edges.len();
        return self.edges[i].point_at(u);
    }

    // Avoid using these functions
    pub fn project(&self, point: Point) -> Option<f64> {
        for (i, edge) in self.edges.iter().enumerate() {
            let u = edge.project(point);
            match u {
                Some(u) => {
                    return Some((i as f64 + u) / self.edges.len() as f64);
                }
                None => {}
            }
        }
        None
    }
}

// It is required that the contours within contours_self and within contours_other do not overlap.
// pub fn remesh_multiple_multiple(contours_self: &[Contour], contours_other: &[Contour]) -> Vec<Contour> {
//     let mut result = Vec::from(contours_self);

//     for other_contour in contours_other {
//         let mut new_result = Vec::<Contour>::new();
//         let mut new_contours = other_contour.remesh_multiple(&result);
//         result = new_result;
//     }

//     result
// }
