use std::fmt::Display;

use crate::{
    algebra_error::AlgebraResult,
    efloat::EFloat64,
    point::Point,
    triangle::{quickhull, TriangleFace},
};

/// A convex hull represented by a collection of triangular faces.
#[derive(Debug, Clone)]
pub struct ConvexHull {
    faces: Vec<TriangleFace>,
}

impl ConvexHull {
    /// Creates a new convex hull from a set of points using the Quickhull algorithm.
    pub fn try_new(points: Vec<Point>) -> AlgebraResult<Self> {
        let faces = quickhull(points)?;
        Ok(Self { faces })
    }

    /// Returns the faces of the convex hull.
    pub fn faces(&self) -> &[TriangleFace] {
        &self.faces
    }

    /// Returns the unique vertices of the convex hull.
    fn unique_vertices(&self) -> Vec<Point> {
        let mut vertices = Vec::new();
        for face in &self.faces {
            for &vertex in [face.a, face.b, face.c].iter() {
                if !vertices.iter().any(|&v| v == vertex) {
                    vertices.push(vertex);
                }
            }
        }
        vertices
    }

    /// Checks if this convex hull intersects with another convex hull using the separating axis theorem.
    pub fn intersects(&self, other: &ConvexHull) -> bool {
        // Get unique vertices for both hulls
        let vertices1 = self.unique_vertices();
        let vertices2 = other.unique_vertices();

        // For each face of both hulls, check if it can be used as a separating axis
        for face in self.faces.iter().chain(other.faces.iter()) {
            let normal = face.normal;
            let mut min_self = EFloat64::from(1e10);
            let mut max_self = EFloat64::from(-1e10);
            let mut min_other = EFloat64::from(1e10);
            let mut max_other = EFloat64::from(-1e10);

            // Project all vertices of self onto the normal
            for &vertex in &vertices1 {
                let projected = vertex.dot(normal);
                min_self = min_self.min(projected);
                max_self = max_self.max(projected);
            }

            // Project all vertices of other onto the normal
            for &vertex in &vertices2 {
                let projected = vertex.dot(normal);
                min_other = min_other.min(projected);
                max_other = max_other.max(projected);
            }

            // If there's a gap between the projections, we found a separating axis
            if max_self < min_other || max_other < min_self {
                return false;
            }
        }

        // Check edges of both hulls as potential separating axes
        for face1 in &self.faces {
            for face2 in &other.faces {
                for edge1 in [(face1.a, face1.b), (face1.b, face1.c), (face1.c, face1.a)] {
                    for edge2 in [(face2.a, face2.b), (face2.b, face2.c), (face2.c, face2.a)] {
                        let dir1 = edge1.1 - edge1.0;
                        let dir2 = edge2.1 - edge2.0;
                        let cross = dir1.cross(dir2);
                        let cross_norm = cross.norm();

                        // Skip if edges are parallel
                        if cross_norm <= 0.0 {
                            continue;
                        }

                        let normal = (cross / cross_norm).unwrap();

                        // Project all vertices onto the normal
                        let mut min_self = vertices1[0].dot(normal);
                        let mut max_self = min_self;
                        let mut min_other = vertices2[0].dot(normal);
                        let mut max_other = min_other;

                        for &vertex in &vertices1 {
                            let projected = vertex.dot(normal);
                            min_self = min_self.min(projected);
                            max_self = max_self.max(projected);
                        }

                        for &vertex in &vertices2 {
                            let projected = vertex.dot(normal);
                            min_other = min_other.min(projected);
                            max_other = max_other.max(projected);
                        }

                        // If there's a gap between the projections, we found a separating axis
                        if max_self < min_other || max_other < min_self {
                            return false;
                        }
                    }
                }
            }
        }

        // No separating axis found, the hulls intersect
        true
    }
}

impl Display for ConvexHull {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConvexHull with {} faces", self.faces.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convex_hull_intersection() -> AlgebraResult<()> {
        // Create two non-intersecting convex hulls
        let points1 = vec![
            Point::from_f64(0.0, 0.0, 0.0),
            Point::from_f64(1.0, 0.0, 0.0),
            Point::from_f64(0.0, 1.0, 0.0),
            Point::from_f64(0.0, 0.0, 1.0),
        ];
        let points2 = vec![
            Point::from_f64(2.0, 0.0, 0.0),
            Point::from_f64(3.0, 0.0, 0.0),
            Point::from_f64(2.0, 1.0, 0.0),
            Point::from_f64(2.0, 0.0, 1.0),
        ];
        let hull1 = ConvexHull::try_new(points1)?;
        let hull2 = ConvexHull::try_new(points2)?;
        assert!(!hull1.intersects(&hull2));

        // Create two intersecting convex hulls
        let points3 = vec![
            Point::from_f64(0.0, 0.0, 0.0),
            Point::from_f64(1.0, 0.0, 0.0),
            Point::from_f64(0.0, 1.0, 0.0),
            Point::from_f64(0.0, 0.0, 1.0),
        ];
        let points4 = vec![
            Point::from_f64(0.5, 0.0, 0.0),
            Point::from_f64(1.5, 0.0, 0.0),
            Point::from_f64(0.5, 1.0, 0.0),
            Point::from_f64(0.5, 0.0, 1.0),
        ];
        let hull3 = ConvexHull::try_new(points3)?;
        let hull4 = ConvexHull::try_new(points4)?;
        assert!(hull3.intersects(&hull4));

        Ok(())
    }
}
