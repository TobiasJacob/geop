use crate::{
    contains::face_point::{face_point_contains, FacePointContains},
    topology::{edge::Edge, face::Face},
};

#[derive(Clone, Debug, PartialEq)]
pub enum FaceEdgeContains {
    Inside,
    OnBorderSameDir,
    OnBorderOppositeDir,
    Outside,
    NotSameSurface,
}

// Checks if the edge is on the surface, and if the midpoint of an edge is inside the face.
pub fn face_edge_contains(face: &Face, edge: &Edge) -> FaceEdgeContains {
    let p = edge.get_midpoint();
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
        FacePointContains::OnPoint(_) => match face
            .boundary_tangent(p)
            .expect_on_edge()
            .dot(edge.tangent(p))
            > 0.0
        {
            true => FaceEdgeContains::OnBorderSameDir,
            false => FaceEdgeContains::OnBorderOppositeDir,
        },
        FacePointContains::NotOnSurface => FaceEdgeContains::NotSameSurface,
    }
}
