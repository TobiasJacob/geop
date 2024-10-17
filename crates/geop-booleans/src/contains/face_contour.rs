use geop_geometry::curve_surface_intersection::curve_surface::curve_surface_intersection;

use geop_topology::topology::{contour::Contour, face::Face};

use super::face_edge::{face_edge_contains, FaceEdgeContains};

#[derive(Debug, PartialEq)]
pub enum FaceContourContains {
    Inside,
    Outside,
    Wiggly,
    Equals,
    NotSameSurface,
}

// Checks if a contour is completely on the surface of a face. Then it checks if the contour is inside, outside, or wiggly.
pub fn face_contour_contains(face: &Face, contour: &Contour) -> FaceContourContains {
    for edge in contour.edges.iter() {
        if !(curve_surface_intersection(&edge.curve, &*face.surface).is_curve()) {
            return FaceContourContains::NotSameSurface;
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
            FaceEdgeContains::NotSameSurface => return FaceContourContains::NotSameSurface,
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
