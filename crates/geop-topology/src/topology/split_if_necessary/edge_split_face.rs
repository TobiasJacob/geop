use geop_geometry::points::point::Point;

use crate::topology::{face::Face, edge::Edge, intersections::edge_edge::{edge_edge_intersection, EdgeEdgeIntersection}, split_if_necessary::point_split_edge::split_edges_by_points_if_necessary, contains::face_point::{face_point_contains, FacePointContains}, contour::Contour};

// Split by an edge that lies either inside or outside the face. Edge is not allowed to intersect the boundary, except at the start and end points.
pub fn split_face_by_edge_if_necessary(face: &Face, edge: &Edge) -> Vec<Face> {
    let edge_start = face_point_contains(face, edge.start);
    let edge_end = face_point_contains(face, edge.end);

    match edge_start {
        FacePointContains::OnEdge(edge_start) => {
            match edge_end {
                FacePointContains::OnEdge(edge_end) => {
                    let relevant_boundary = face.boundaries.iter().find(|boundary| boundary.edges.contains(&edge_start) && boundary.edges.contains(&edge_end)).expect("Edge start and end must be on the same boundary");
                    
                    let mut bound_a = relevant_boundary.get_subcurve(edge.end, edge.start);
                    bound_a.push(edge.clone());
                    let mut face_a = Face::new(vec![Contour::new(bound_a)], face.surface.clone());

                    let mut bound_b = relevant_boundary.get_subcurve(edge.start, edge.end);
                    bound_b.push(edge.clone());
                    let mut face_b = Face::new(vec![Contour::new(bound_b)], face.surface.clone());

                    // Now do the inner boundaries
                    for boundary in face.boundaries.iter().filter(|boundary| *boundary != relevant_boundary) {
                        let is_in_a = face_point_contains(&face_a, boundary.edges[0].start);
                        if is_in_a == FacePointContains::Inside {
                            face_a.boundaries.push(boundary.clone());
                        } else {
                            face_b.boundaries.push(boundary.clone());
                        }
                    }

                    vec![face_a, face_b]
                },
                _ => {vec![face.clone()]}
            }
        },
        _ => {vec![face.clone()]}
    }
}

pub fn split_face_by_edges_if_necessary(face: &Face, edges: &[Edge]) -> Vec<Face> {
    let mut result = vec![face.clone()];
    for edge in edges {
        let mut new_result = Vec::<Face>::new();
        for face in result.iter() {
            new_result.extend(split_face_by_edge_if_necessary(face, edge));
        }
        result = new_result;
    }
    result
}

pub fn split_faces_by_edges_if_necessary(faces: Vec<Face>, edges: &Vec<Edge>) -> Vec<Face> {
    let mut result = Vec::<Face>::new();
    for face in faces {
        result.extend(split_face_by_edges_if_necessary(&face, &edges));
    }
    result
}