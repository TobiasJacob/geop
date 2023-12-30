use std::rc::Rc;

use geop_geometry::curve_surface_intersection::curve_surface::curve_surface_intersection;

use crate::topology::{
    contains::face_point::{face_point_contains, FacePointContains},
    face::Face,
    intersections::contour_edge::countour_edge_intersection_points,
    regularize::edge::RegularizedEdge,
};

pub enum FaceEdgeContains {
    Inside,
    OnBorderSameDir,
    OnBorderOppositeDir,
    Outside,
}

// Checks if an edge is inside the face. This guarantees that the edge is not touching any curves. The start and end point of the edge can be on the border, since they are not considered a part of the edge.
pub fn face_edge_contains(face: &Face, edge: &RegularizedEdge) -> FaceEdgeContains {
    assert!(curve_surface_intersection(&edge.curve, &face.surface).is_curve());
    // TODO: Make an assertian that there are no intersections with the face boundaries
    let start = *edge.boundaries[0].0;
    let end = *edge.boundaries[0].1;
    for int in countour_edge_intersection_points(face, edge) {
        assert!(start == *int || end == *int);
    }

    let p = edge.curve.get_midpoint(start, end);
    match face_point_contains(face, &Rc::new(p)) {
        FacePointContains::Inside => FaceEdgeContains::Inside,
        FacePointContains::Outside => FaceEdgeContains::Outside,
        FacePointContains::OnEdge(_) => match face
            .boundary_tangent(p)
            .expect_on_edge()
            .dot(edge.curve.tangent(p))
            > 0.0
        {
            true => FaceEdgeContains::OnBorderSameDir,
            false => FaceEdgeContains::OnBorderOppositeDir,
        },
        FacePointContains::OnPoint(_) => panic!("This case should not happen"),
    }
}
