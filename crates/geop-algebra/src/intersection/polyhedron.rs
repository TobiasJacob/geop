use crate::{line::Line, point::Point, triangle::TriangleFace};

use super::line::line_triangle_intersection;

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

/// Checks if two polyhedra intersect
pub fn polyhedron_polyhedron_intersection(
    faces1: &[TriangleFace],
    faces2: &[TriangleFace],
) -> bool {
    // check if one polyhedron is completely inside the other
    if point_polyhedron_intersection(&faces1[0].a, faces2) {
        return true;
    }
    // Check if any triangle from one polyhedron intersects with any triangle from the other
    for face1 in faces1 {
        for face2 in faces2 {
            if super::triangle::triangle_triangle_intersection(face1, face2) {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra_error::AlgebraResult;

    #[test]
    fn test_point_polyhedron_intersection() -> AlgebraResult<()> {
        let faces = vec![
            // Bottom face (z=0)
            TriangleFace::try_new(
                Point::from_f64(0.0, 0.0, 0.0),
                Point::from_f64(1.0, 0.0, 0.0),
                Point::from_f64(0.0, 1.0, 0.0),
            )?,
            // Front face (y=0)
            TriangleFace::try_new(
                Point::from_f64(0.0, 0.0, 0.0),
                Point::from_f64(0.0, 0.0, 1.0),
                Point::from_f64(1.0, 0.0, 0.0),
            )?,
            // Left face (x=0)
            TriangleFace::try_new(
                Point::from_f64(0.0, 0.0, 0.0),
                Point::from_f64(0.0, 1.0, 0.0),
                Point::from_f64(0.0, 0.0, 1.0),
            )?,
            // Top face (z=1)
            TriangleFace::try_new(
                Point::from_f64(1.0, 0.0, 0.0),
                Point::from_f64(0.0, 0.0, 1.0),
                Point::from_f64(0.0, 1.0, 0.0),
            )?,
        ];

        // Point inside polyhedron
        let p1 = Point::from_f64(0.25, 0.25, 0.25);
        assert!(point_polyhedron_intersection(&p1, &faces));

        // Point outside polyhedron
        let p2 = Point::from_f64(2.0, 2.0, 2.0);
        assert!(!point_polyhedron_intersection(&p2, &faces));

        // Point on face
        let p3 = Point::from_f64(0.5, 0.5, 0.0);
        assert!(point_polyhedron_intersection(&p3, &faces));

        Ok(())
    }

    #[test]
    fn test_polyhedron_polyhedron_intersection() -> AlgebraResult<()> {
        let faces1 = vec![
            // Bottom face (z=0)
            TriangleFace::try_new(
                Point::from_f64(0.0, 0.0, 0.0),
                Point::from_f64(1.0, 0.0, 0.0),
                Point::from_f64(0.0, 1.0, 0.0),
            )?,
            // Front face (y=0)
            TriangleFace::try_new(
                Point::from_f64(0.0, 0.0, 0.0),
                Point::from_f64(0.0, 0.0, 1.0),
                Point::from_f64(1.0, 0.0, 0.0),
            )?,
            // Left face (x=0)
            TriangleFace::try_new(
                Point::from_f64(0.0, 0.0, 0.0),
                Point::from_f64(0.0, 1.0, 0.0),
                Point::from_f64(0.0, 0.0, 1.0),
            )?,
            // Top face (z=1)
            TriangleFace::try_new(
                Point::from_f64(1.0, 0.0, 0.0),
                Point::from_f64(0.0, 0.0, 1.0),
                Point::from_f64(0.0, 1.0, 0.0),
            )?,
        ];

        let faces2 = vec![
            // Bottom face (z=0)
            TriangleFace::try_new(
                Point::from_f64(0.5, 0.5, 0.0),
                Point::from_f64(1.5, 0.5, 0.0),
                Point::from_f64(0.5, 1.5, 0.0),
            )?,
            // Front face (y=0)
            TriangleFace::try_new(
                Point::from_f64(0.5, 0.5, 0.0),
                Point::from_f64(0.5, 0.5, 1.0),
                Point::from_f64(1.5, 0.5, 0.0),
            )?,
            // Left face (x=0)
            TriangleFace::try_new(
                Point::from_f64(0.5, 0.5, 0.0),
                Point::from_f64(0.5, 1.5, 0.0),
                Point::from_f64(0.5, 0.5, 1.0),
            )?,
            // Top face (z=1)
            TriangleFace::try_new(
                Point::from_f64(1.5, 0.5, 0.0),
                Point::from_f64(0.5, 0.5, 1.0),
                Point::from_f64(0.5, 1.5, 0.0),
            )?,
        ];

        // Intersecting polyhedra
        assert!(polyhedron_polyhedron_intersection(&faces1, &faces2));

        // Non-intersecting polyhedra
        let faces3 = vec![
            // Bottom face (z=0)
            TriangleFace::try_new(
                Point::from_f64(2.0, 0.0, 0.0),
                Point::from_f64(3.0, 0.0, 0.0),
                Point::from_f64(2.0, 1.0, 0.0),
            )?,
            // Front face (y=0)
            TriangleFace::try_new(
                Point::from_f64(2.0, 0.0, 0.0),
                Point::from_f64(2.0, 0.0, 1.0),
                Point::from_f64(3.0, 0.0, 0.0),
            )?,
            // Left face (x=0)
            TriangleFace::try_new(
                Point::from_f64(2.0, 0.0, 0.0),
                Point::from_f64(2.0, 1.0, 0.0),
                Point::from_f64(2.0, 0.0, 1.0),
            )?,
            // Top face (z=1)
            TriangleFace::try_new(
                Point::from_f64(3.0, 0.0, 0.0),
                Point::from_f64(2.0, 0.0, 1.0),
                Point::from_f64(2.0, 1.0, 0.0),
            )?,
        ];

        let vertices3 = vec![
            Point::from_f64(2.0, 0.0, 0.0),
            Point::from_f64(3.0, 0.0, 0.0),
            Point::from_f64(2.0, 1.0, 0.0),
            Point::from_f64(2.0, 0.0, 1.0),
        ];

        assert!(!polyhedron_polyhedron_intersection(&faces1, &faces3));

        Ok(())
    }
}
