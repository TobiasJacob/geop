use crate::{
    contains::surface_edge::surface_edge_contains,
    topology::{contour::Contour, face::Face},
};

use super::face_edge::{face_edge_contains, FaceEdgeContains};

#[derive(Debug, PartialEq)]
pub enum FaceContourContains {
    Inside,
    Outside,
    Wiggly,
    Equals,
}

// Checks if an edge is inside the face. This guarantees that the edge is not touching any curves. The start and end point of the edge can be on the border, since they are not considered a part of the edge.
pub fn face_contour_contains(face: &Face, contour: &Contour) -> FaceContourContains {
    for edge in contour.edges.iter() {
        assert!(surface_edge_contains(&face.surface, edge));
    }
    // TODO: Make an assertian that there are no intersections with the face boundaries
    // for int in countour_edge_intersection_points(face, edge) {
    //     assert!(*edge.start == *int || *edge.end == *int);
    // }

    let mut inside = 0;
    let mut outside = 0;
    for edge in contour.edges.iter() {
        match face_edge_contains(face, edge) {
            FaceEdgeContains::Inside => inside += 1,
            FaceEdgeContains::Outside => outside += 1,
            FaceEdgeContains::OnBorderSameDir => (),
            FaceEdgeContains::OnBorderOppositeDir => (),
        }
    }

    if inside == 0 {
        return FaceContourContains::Outside;
    } else if outside == 0 {
        return FaceContourContains::Inside;
    } else if inside > 0 && outside > 0 {
        return FaceContourContains::Wiggly;
    } else {
        return FaceContourContains::Equals;
    }
}
