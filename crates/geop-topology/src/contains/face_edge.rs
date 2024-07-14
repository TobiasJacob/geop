use geop_geometry::curve_surface_intersection::curve_surface::curve_surface_intersection;

use crate::{
    contains::face_point::{face_point_contains, FacePointContains},
    intersections::edge_edge::{edge_edge_intersection, EdgeEdgeIntersection},
    topology::{edge::Edge, face::Face},
};

pub enum FaceEdgeContains {
    Inside,
    OnBorderSameDir,
    OnBorderOppositeDir,
    Outside,
}

// Checks if an edge is inside the face. This guarantees that the edge is not touching any curves. The start and end point of the edge can be on the border, since they are not considered a part of the edge.
pub fn face_edge_contains(face: &Face, edge: &Edge) -> FaceEdgeContains {
    assert!(curve_surface_intersection(&edge.curve, &*face.surface).is_curve());
    // Check that there are no intersections with the face boundaries
    // TODO: Test this code
    for edge2 in face.boundary.edges.iter() {
        let intersections = edge_edge_intersection(edge, edge2);
        match intersections {
            EdgeEdgeIntersection::Edges(int_edges) => {
                for int_edge in int_edges {
                    assert!(int_edge.start == edge.start || int_edge.start == edge.end);
                    assert!(int_edge.end == edge.start || int_edge.end == edge.end);
                }
            }
            EdgeEdgeIntersection::Points(int_points) => {
                for int_point in int_points {
                    assert!(int_point == edge.start || int_point == edge.end);
                }
            }
            EdgeEdgeIntersection::None => (),
        }
    }

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
