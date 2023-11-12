use geop_geometry::points::point::Point;

use crate::topology::{
    contains::face_point::{face_contains_point, FaceContainsPoint},
    edge::Edge,
    face::Face,
    intersections::{self, edge_edge::EdgeEdgeIntersection, contour_edge::{countour_edge_intersection_points}},
};

pub enum FaceContainsEdge {
    Inside,
    OnBorderSameDir,
    OnBorderOppositeDir,
    Outside,
}

// Checks if an edge is inside the face. This guarantees that the edge is not touching any curves. The start and end point of the edge can be on the border, since they are not considered a part of the edge.
pub fn face_contains_edge(face: &Face, edge: &Edge) -> FaceContainsEdge {
    assert!(face.surface.contains_edge(edge));
    // TODO: Make an assertian that there are no intersections with the face boundaries
    for int in countour_edge_intersection_points(face, edge) {
        assert!(*edge.start == *int || *edge.end == *int);
    }

    let p = edge.get_midpoint(*edge.start, *edge.end);
    match face_contains_point(face, p) {
        FaceContainsPoint::Inside => FaceContainsEdge::Inside,
        FaceContainsPoint::Outside => FaceContainsEdge::Outside,
        FaceContainsPoint::OnEdge(_) => match face
            .boundary_tangent(p)
            .expect_on_edge()
            .dot(edge.tangent(p))
            > 0.0
        {
            true => FaceContainsEdge::OnBorderSameDir,
            false => FaceContainsEdge::OnBorderOppositeDir,
        },
        FaceContainsPoint::OnPoint(_) => panic!("This case should not happen"),
    }
}
