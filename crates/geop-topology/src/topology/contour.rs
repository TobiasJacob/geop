use std::{rc::Rc, vec};

use geop_geometry::{points::point::Point};

use crate::topology::vertex::Vertex;

use super::{edge::{Edge, EdgeIntersection}};

#[derive(Debug, Clone)]
pub struct Contour {
    pub edges: Vec<Rc<Edge>>,
}

// An Contour is a closed loop of edges which is not self intersecting (because otherwise project would not be defined for self intersection point).
// It has a defined inside and outside, which is determined by the direction of the edges.
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

    pub fn neg(&self) -> Contour {
        todo!()
    }

    pub fn contains(&self, point: &Point) -> bool {
        return self.get_edge_index(point).is_some();
    }

    fn get_edge_index(&self, point: &Point) -> Option<usize> {
        for (i, edge) in self.edges.iter().enumerate() {
            if edge.contains(point) {
                return Some(i);
            }
        }
        None
    }

    // Gets the subcurve between these two points
    pub fn get_subcurve(&self, start: Vertex, end: Vertex) -> Vec<Rc<Edge>> {
        let mut result = Vec::<Rc<Edge>>::new();
        let mut start_i = self
            .get_edge_index(&start.point)
            .expect("Start point has to be on edge");
        let mut end_edge = self
            .get_edge_index(&end.point)
            .expect("End point has to be on edge");

        if start_i == end_edge {
            let edge = Rc::new(Edge::new(start, end, self.edges[start_i].curve, self.edges[start_i].direction));
            result.push(edge);
        }

        let mut edge = self.edges[start_i];
        result.push(Rc::new(Edge::new(start, edge.end, edge.curve, edge.direction)));
        for i in start_i + 1..end_edge {
            edge = self.edges[i % self.edges.len()];
            result.push(edge.clone());
        }
        let mut edge = self.edges[end_edge % self.edges.len()];
        result.push(Rc::new(Edge::new(edge.start, end, edge.curve, edge.direction)));
        result
    }

    pub fn get_subcurves(&self, vertices: Vec<(Vertex, Vertex)>) -> Vec<Vec<Rc<Edge>>> {
        let mut result = Vec::<Vec<Rc<Edge>>>::new();
        for (seg_start, seg_end) in vertices.iter() {
            let segment = self.get_subcurve(seg_start.clone(), seg_end.clone());            
            result.push(segment);
        }
        result
    }

    pub fn intersect(&self, other: &Contour) -> Vec<Vertex> {
        let mut intersections = Vec::<Vertex>::new();
        for edge_self in self.edges.iter() {
            for edge_other in other.edges.iter() {
                let edge_intersections = edge_self.intersections(edge_other);
                for edge_intersection in edge_intersections {
                    match edge_intersection {
                        EdgeIntersection::Vertex(vertex) => {
                            if !intersections.contains(&vertex) {
                                intersections.push(vertex);
                            }
                        },
                        EdgeIntersection::Edge(edge) => {
                            if !intersections.contains(&edge.start) {
                                intersections.push(edge.start);
                            }
                            if !intersections.contains(&edge.end) {
                                intersections.push(edge.end);
                            }
                        },
                    }
                }
            }
        }

        intersections
    }


    // Takes 2 Contours and connects them at intersecting points with new vertices.
    // If there are overlapping edges, there will be a vertex for the beginning and the end of the overlapping edges, and a connecting edge for each loop.
    // If there are no intersections, the outer vector will have length 1.
    fn split_if_necessary(&self, other: &Contour) -> (Contour, Contour) {
        let mut split_verts: Vec<Vertex> = self.intersect(other);

        let mut edges_self = self.edges.clone();
        let mut edges_other = other.edges.clone();
        for vert in split_verts.iter() {
            let mut new_edges_self = Vec::<Rc<Edge>>::new();
            for edge in edges_self.iter() {
                let new_edges = edge.split_if_necessary(vert.clone());
                new_edges_self.extend(new_edges);
            }
            edges_self = new_edges_self;

            let mut new_edges_other = Vec::<Rc<Edge>>::new();
            for edge in edges_other.iter() {
                let new_edges = edge.split_if_necessary(vert.clone());
                new_edges_other.extend(new_edges);
            }
            edges_other = new_edges_other;
        }

        (Contour::new(edges_self), Contour::new(edges_other))
    }

    // Splits this edge loop with another edge loop.
    // This makes sure that the resulting edge loops are closed and do not intersect each other anymore.
    // Neighbouring edge loops will share the same end points for the edges, and the two neighbouring edges will face opposite direction.
    // If edge loops are not overlapping, the result will be two edge loops.
    pub fn remesh(&self, other: &Contour) -> Vec<Contour> {
        let (mut segments_self, mut segments_other) = self.split_if_necessary(other);
        let mut edges_self = segments_self.edges;
        let mut edges_other = segments_other.edges;
        for edge in segments_self.edges.iter() {
            println!("Edge: {:?} - {:?}", edge.start.point, edge.end.point);
        }

        for edge in segments_other.edges.iter() {
            println!("Edge: {:?} - {:?}", edge.start.point, edge.end.point);
        }

        let pop_next_segment = |next_segment_is_self: bool, start: &Vertex| -> Option<Rc<Edge>> {
            let relevant_segments = if next_segment_is_self {
                &mut edges_self
            } else {
                &mut edges_other
            };
            for (i, edge) in relevant_segments.iter().enumerate() {
                if edge.start == *start {
                    let edge = relevant_segments.remove(i);
                    return Some(edge);
                }
            }
            None
        };

        let mut contours = Vec::new();
        let mut next_segment_is_self = false;
        loop {
            let mut contour: Vec<Rc<Edge>> = Vec::new();
            if edges_self.len() > 0 {
                contour.push(edges_self.pop().unwrap());
                next_segment_is_self = false;
            } else if edges_other.len() > 0 {
                contour.push(edges_other.pop().unwrap());
                next_segment_is_self = true;
            } else {
                break;
            }
            while contour[0].start != contour[contour.len() - 1].end {
                let end_point = contour[contour.len() - 1].end;
                let next_segment = pop_next_segment(next_segment_is_self, &end_point);
                match next_segment {
                    Some(next_segment) => {
                        contour.push(next_segment);
                        next_segment_is_self = !next_segment_is_self;
                    },
                    None => {
                        let next_segment = pop_next_segment(!next_segment_is_self, &end_point).expect("Edge has to be in one of the edge loops");
                        contour.push(next_segment);
                    }
                }
            }
            contours.push(Contour::new(contour));

            if let Some(mut next_segment) = segments_self.edges.pop().or_else(|| segments_other.edges.pop()) {
                break;
            }
        }

        contours
    }

    pub fn remesh_multiple(&self, other: &[Contour]) -> Vec<Contour> {
        let mut result = vec![self.clone()];
        
        for other_contour in other {
            let mut new_result = Vec::<Contour>::new();
            for contour in result {
                let mut new_contours = contour.remesh(other_contour);
                new_result.extend(new_contours.drain(..));
            }
            result = new_result;
        }

        result
    }

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
}

pub fn remesh_multiple_multiple(contours_self: &[Contour], contours_other: &[Contour]) -> Vec<Contour> {
    let mut result = Vec::from(contours_self);
    
    for other_contour in contours_other {
        let mut new_result = Vec::<Contour>::new();
        let mut new_contours = other_contour.remesh_multiple(result.as_slice());
        result = new_result;
    }

    result
}
