use crate::{
    contains::{
        face_edge::{face_edge_contains, FaceEdgeContains},
        face_point::{face_point_contains, FacePointContains},
    },
    remesh::edge,
    topology::{contour::Contour, edge::Edge, face::Face},
};

pub fn split_face_by_contour_if_necessary(face: &Face, contour: &Contour) -> Vec<Face> {
    // It is safe to assume that edges will start and end at the boundary of the face, since the face intersections were used to create the contours.
    // Only the edges of the contour that are inside the face are relevant.

    let mut relevant_edges = contour
        .edges
        .iter()
        .cloned()
        .filter(|edge| face_edge_contains(face, edge) == FaceEdgeContains::Inside)
        .collect::<Vec<Edge>>();

    // Approach:
    // Find an edge that is inside the face.
    // Then follow the contour until we reach the boundary of the face or the contour is closed.
    // If the contour is closed, we have a new face.
    // If the contour is not closed, we follow the contour in the opposite direction until we reach the boundary of the face.
    // Then we have a new face.

    let mut result = Vec::<Face>::new();
    result.push(face.clone());
    loop {
        let mut edges = Vec::<Edge>::new();
        match relevant_edges.pop() {
            Some(edge) => {
                edges.push(edge.clone());
            }
            None => {
                break;
            }
        }
        loop {
            let last_edge = edges.last().unwrap();

            if last_edge.end == edges.first().unwrap().start {
                // We closed the contour, so we have a new hole
                todo!("Create a new face with the hole");
                break;
            }
            // Check if we hit the boundary of the face
            match face_point_contains(face, last_edge.end) {
                FacePointContains::OnEdge(_) => {
                    // We hit the boundary of the face
                    break;
                }
                FacePointContains::OnPoint(_) => {
                    // We hit the boundary of the face
                    break;
                }
                FacePointContains::Inside => {
                    // We are inside the face, so we can continue following the contour
                }
                FacePointContains::Outside => {
                    // We are outside the face, so we can continue following the contour
                }
            }
            // Find the next edge that starts where the last edge ends
            let next_edge = relevant_edges
                .iter()
                .position(|edge| edge.start == last_edge.end || edge.end == last_edge.end);
            match next_edge {
                Some(index) => {
                    let edge = relevant_edges.swap_remove(index);
                    if edge.start == last_edge.end {
                        edges.push(edge);
                    } else {
                        edges.push(edge.flip());
                    }
                }
                None => {
                    break;
                }
            }
        }
        todo!("Create a new face with the contour");
    }
    todo!()
}

pub fn split_face_by_contours_if_necessary(face: &Face, contours: &[Contour]) -> Vec<Face> {
    let mut result = vec![face.clone()];
    for c in contours {
        let mut new_result = Vec::<Face>::new();
        for face in result.iter() {
            new_result.extend(split_face_by_contour_if_necessary(face, c));
        }
        result = new_result;
    }
    result
}

pub fn split_faces_by_contours_if_necessary(
    faces: Vec<Face>,
    contours: &Vec<Contour>,
) -> Vec<Face> {
    let mut result = Vec::<Face>::new();
    for face in faces {
        result.extend(split_face_by_contours_if_necessary(&face, &contours));
    }
    result
}
