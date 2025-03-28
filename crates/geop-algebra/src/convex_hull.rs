use std::fmt::Display;

use crate::{
    algebra_error::{AlgebraError, AlgebraResult},
    intersection::line::line_triangle_intersection,
    line::Line,
    point::Point,
    triangle::{quickhull, TriangleFace},
};

use crate::intersection::{
    line::line_line_intersection, point::point_line_intersection,
    point::point_triangle_intersection, triangle::triangle_triangle_intersection,
};

/// A convex hull that can represent different geometric objects based on the number of points:
/// - Single point
/// - Line segment (two points)
/// - Triangle (three points)
/// - Convex polyhedron (more than three points)
#[derive(Debug, Clone)]
pub enum ConvexHull {
    Point(Point),
    Line(Line),
    Triangle(TriangleFace),
    Polyhedron(Vec<TriangleFace>),
}

impl ConvexHull {
    /// Creates a new convex hull from a set of points using the Quickhull algorithm.
    pub fn try_new(points: Vec<Point>) -> AlgebraResult<Self> {
        // Filter out duplicate points
        let mut unique_points = Vec::new();
        for point in points {
            if !unique_points.iter().any(|&p| p == point) {
                unique_points.push(point);
            }
        }

        match unique_points.len() {
            0 => Err(AlgebraError::new(
                "Cannot create convex hull from empty set of points".to_string(),
            )),
            1 => Ok(Self::Point(unique_points[0])),
            2 => Ok(Self::Line(Line::new(unique_points[0], unique_points[1]))),
            3 => Ok(Self::Triangle(TriangleFace::try_new(
                unique_points[0],
                unique_points[1],
                unique_points[2],
            )?)),
            _ => {
                let faces = quickhull(unique_points)?;
                Ok(Self::Polyhedron(faces))
            }
        }
    }

    /// Checks if this convex hull intersects with another convex hull using the separating axis theorem.
    pub fn intersects(&self, other: &ConvexHull) -> bool {
        match (self, other) {
            (Self::Point(p1), Self::Point(p2)) => p1 == p2,
            (Self::Point(p), Self::Line(l)) | (Self::Line(l), Self::Point(p)) => {
                point_line_intersection(p, l)
            }
            (Self::Point(p), Self::Triangle(t)) | (Self::Triangle(t), Self::Point(p)) => {
                point_triangle_intersection(p, t)
            }
            (Self::Point(p), Self::Polyhedron(faces))
            | (Self::Polyhedron(faces), Self::Point(p)) => {
                for face in faces {
                    if face.distance_to_point(p) > 0.0 {
                        return false;
                    }
                }
                true
            }
            (Self::Line(l1), Self::Line(l2)) => line_line_intersection(l1, l2),
            (Self::Line(l), Self::Triangle(t)) | (Self::Triangle(t), Self::Line(l)) => {
                line_triangle_intersection(l, t)
            }
            (Self::Line(l), Self::Polyhedron(faces)) | (Self::Polyhedron(faces), Self::Line(l)) => {
                for face in faces {
                    if line_triangle_intersection(l, face) {
                        return true;
                    }
                }
                false
            }
            (Self::Triangle(t1), Self::Triangle(t2)) => triangle_triangle_intersection(t1, t2),
            (Self::Triangle(t), Self::Polyhedron(faces))
            | (Self::Polyhedron(faces), Self::Triangle(t)) => {
                for face in faces {
                    if triangle_triangle_intersection(t, face) {
                        return true;
                    }
                }
                false
            }
            (Self::Polyhedron(faces1), Self::Polyhedron(faces2)) => {
                for face1 in faces1 {
                    for face2 in faces2 {
                        if triangle_triangle_intersection(face1, face2) {
                            return true;
                        }
                    }
                }
                false
            }
        }
    }
}

impl Display for ConvexHull {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Point(_) => write!(f, "ConvexHull::Point"),
            Self::Line(_) => write!(f, "ConvexHull::Line"),
            Self::Triangle(_) => write!(f, "ConvexHull::Triangle"),
            Self::Polyhedron(faces) => {
                write!(f, "ConvexHull::Polyhedron with {} faces", faces.len())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convex_hull_creation() -> AlgebraResult<()> {
        // Test single point
        let point = Point::from_f64(1.0, 2.0, 3.0);
        let hull = ConvexHull::try_new(vec![point])?;
        assert!(matches!(hull, ConvexHull::Point(p) if p == point));

        // Test line segment
        let p1 = Point::from_f64(0.0, 0.0, 0.0);
        let p2 = Point::from_f64(1.0, 1.0, 1.0);
        let hull = ConvexHull::try_new(vec![p1, p2])?;
        assert!(matches!(hull, ConvexHull::Line(_)));

        // Test triangle
        let p3 = Point::from_f64(0.0, 1.0, 0.0);
        let hull = ConvexHull::try_new(vec![p1, p2, p3])?;
        assert!(matches!(hull, ConvexHull::Triangle(_)));

        // Test polyhedron
        let p4 = Point::from_f64(0.0, 0.0, 1.0);
        let hull = ConvexHull::try_new(vec![p1, p2, p3, p4])?;
        assert!(matches!(hull, ConvexHull::Polyhedron(_)));

        Ok(())
    }

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
            Point::from_f64(2.0, 0.0, 0.0),
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
