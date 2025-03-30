use crate::primitives::triangle::TriangleFace;
use crate::primitives::line::Line;
use crate::primitives::point::Point;
use super::{line::line_triangle_intersection, triangle::triangle_triangle_intersection};

/// Checks if a point is inside a convex polyhedron
pub fn point_polyhedron_intersection(point: &Point, faces: &[TriangleFace]) -> bool {
    for face in faces {
        // For a point to be inside, it must be behind all faces (negative distance)
        // or on any face (zero distance)
        if face.distance_to_point(point) > 0.0 {
            return false;
        }
    }
    true
}

pub fn line_polyhedron_intersection(line: &Line, faces: &[TriangleFace]) -> bool {
    // check if line is completely inside the polyhedron
    if point_polyhedron_intersection(&line.start(), faces)
        && point_polyhedron_intersection(&line.end(), faces)
    {
        return true;
    }

    for face in faces {
        if line_triangle_intersection(line, face) {
            return true;
        }
    }
    false
}

pub fn triangle_polyhedron_intersection(triangle: &TriangleFace, faces: &[TriangleFace]) -> bool {
    if point_polyhedron_intersection(&triangle.a, faces) {
        return true;
    }
    for face in faces {
        if triangle_triangle_intersection(triangle, face) {
            return true;
        }
    }
    false
}

/// Checks if two polyhedra intersect
pub fn polyhedron_polyhedron_intersection(
    faces1: &[TriangleFace],
    faces2: &[TriangleFace],
) -> bool {
    if faces1.is_empty() || faces2.is_empty() {
        return false;
    }
    // check if one polyhedron is completely inside the other
    if point_polyhedron_intersection(&faces1[0].a, faces2) {
        return true;
    }
    if point_polyhedron_intersection(&faces2[0].a, faces1) {
        return true;
    }
    // Check if any triangle from one polyhedron intersects with any triangle from the other
    for face1 in faces1 {
        for face2 in faces2 {
            if triangle_triangle_intersection(face1, face2) {
                // println!(
                //     "triangle_triangle_intersection between {} and {}",
                //     face1, face2
                // );
                return true;
            }
        }
    }
    false
}
