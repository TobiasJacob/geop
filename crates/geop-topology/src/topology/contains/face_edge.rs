use crate::topology::{
    contains::{
        face_point::{face_point_contains, FacePointContains},
        surface_edge::surface_edge_contains,
    },
    edge::Edge,
    face::Face,
};

pub enum FaceEdgeContains {
    Inside,
    OnBorderSameDir,
    OnBorderOppositeDir,
    Outside,
}

// Checks if an edge is inside the face. This guarantees that the edge is not touching any curves. The start and end point of the edge can be on the border, since they are not considered a part of the edge.
pub fn face_edge_contains(face: &Face, edge: &Edge) -> FaceEdgeContains {
    assert!(surface_edge_contains(&face.surface, edge));
    // TODO: Make an assertian that there are no intersections with the face boundaries
    // for int in countour_edge_intersection_points(face, edge) {
    //     assert!(*edge.start == *int || *edge.end == *int);
    // }

    let p = edge.get_midpoint(edge.start, edge.end);
    match face_point_contains(face, p) {
        FacePointContains::Inside => FaceEdgeContains::Inside,
        FacePointContains::Outside => FaceEdgeContains::Outside,
        FacePointContains::OnEdge(_) => match face
            .boundary_tangent(p)
            .expect_on_edge()
            .dot(edge.tangent(p))
            > 0.0
        {
            true => FaceEdgeContains::OnBorderSameDir,
            false => FaceEdgeContains::OnBorderOppositeDir,
        },
        FacePointContains::OnPoint(_) => panic!("This case should not happen"),
    }
}
