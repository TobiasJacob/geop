use geop_geometry::curve_surface_intersection::curve_surface::curve_surface_intersection;

use crate::{
    intersections::edge_edge::{edge_edge_intersection, EdgeEdgeIntersection},
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
        assert!(curve_surface_intersection(&edge.curve, &*face.surface).is_curve());
    }
    // Check that there are no additional intersections between the face and the contour
    // TODO: Write a test case for this
    for edge in face.boundary.edges.iter() {
        for edge2 in contour.edges.iter() {
            let intersections = edge_edge_intersection(edge, edge2);
            match intersections {
                EdgeEdgeIntersection::Edges(int_edges) => {
                    for int_edge in int_edges {
                        assert!(edge.start == edge2.start || edge.start == edge2.end);
                        assert!(edge.end == edge2.start || edge.end == edge2.end);
                        assert!(int_edge.start == edge.start || int_edge.start == edge.end);
                        assert!(int_edge.end == edge.start || int_edge.end == edge.end);
                    }
                }
                EdgeEdgeIntersection::Points(int_points) => {
                    for int_point in int_points {
                        assert!(int_point == edge.start || int_point == edge.end);
                        assert!(int_point == edge2.start || int_point == edge2.end);
                    }
                }
                EdgeEdgeIntersection::None => (),
            }
        }
    }

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
