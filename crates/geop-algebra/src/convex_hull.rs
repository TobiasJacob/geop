use std::fmt::Display;

use crate::{
    algebra_error::{AlgebraError, AlgebraResult},
    efloat::EFloat64,
    line::Line,
    point::Point,
    triangle::{quickhull, TriangleFace},
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
        match points.len() {
            0 => Err(AlgebraError::new(
                "Cannot create convex hull from empty set of points".to_string(),
            )),
            1 => Ok(Self::Point(points[0])),
            2 => Ok(Self::Line(Line::new(points[0], points[1]))),
            3 => Ok(Self::Triangle(TriangleFace::try_new(
                points[0], points[1], points[2],
            )?)),
            _ => {
                let faces = quickhull(points)?;
                Ok(Self::Polyhedron(faces))
            }
        }
    }

    /// Checks if a point intersects with a line segment
    fn point_line_intersection(p: &Point, l: &Line) -> bool {
        let line_dir = l.direction();
        let point_dir = *p - l.start();
        let cross = line_dir.cross(point_dir);
        if cross.norm() > 0.0 {
            return false;
        }
        // Check if point is between start and end
        let t = point_dir.dot(line_dir) / line_dir.dot(line_dir);
        if let Ok(t) = t {
            return t >= 0.0 && t <= 1.0;
        }
        false
    }

    /// Checks if a point lies inside a triangle
    fn point_triangle_intersection(p: &Point, t: &TriangleFace) -> bool {
        let v0 = t.b - t.a;
        let v1 = t.c - t.a;
        let v2 = *p - t.a;
        let dot00 = v0.dot(v0);
        let dot01 = v0.dot(v1);
        let dot02 = v0.dot(v2);
        let dot11 = v1.dot(v1);
        let dot12 = v1.dot(v2);
        let inv_denom = EFloat64::from(1.0) / (dot00 * dot11 - dot01 * dot01);
        if let Ok(inv_denom) = inv_denom {
            let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
            let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;
            return u >= 0.0 && v >= 0.0 && u + v <= 1.0;
        }
        false
    }

    /// Checks if two line segments intersect
    fn line_line_intersection(l1: &Line, l2: &Line) -> bool {
        let dir1 = l1.direction();
        let dir2 = l2.direction();
        let cross = dir1.cross(dir2);
        let cross_norm = cross.norm();
        if cross_norm <= 0.0 {
            // Lines are parallel, check if they overlap
            let p1 = l1.start();
            let p2 = l2.start();
            let dir = p2 - p1;
            let t = dir.cross(dir2).dot(cross) / cross_norm;
            let u = dir.cross(dir1).dot(cross) / cross_norm;
            if let (Ok(t), Ok(u)) = (t, u) {
                return t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0;
            }
            return false;
        }
        true
    }

    /// Checks if a line segment intersects with a triangle
    fn line_triangle_intersection(l: &Line, t: &TriangleFace) -> bool {
        let dir = l.direction();
        let v0 = t.b - t.a;
        let v1 = t.c - t.a;
        let v2 = l.start() - t.a;
        let cross = dir.cross(v1);
        let det = v0.dot(cross);
        if det.abs() <= 0.0 {
            return false;
        }
        let inv_det = EFloat64::from(1.0) / det;
        if let Ok(inv_det) = inv_det {
            let u = v2.dot(cross) * inv_det;
            let v = dir.dot(v2.cross(v0)) * inv_det;
            let t_param = v1.dot(v2.cross(v0)) * inv_det;
            return u >= 0.0 && v >= 0.0 && u + v <= 1.0 && t_param >= 0.0 && t_param <= 1.0;
        }
        false
    }

    /// Checks if two triangles intersect
    fn triangle_triangle_intersection(t1: &TriangleFace, t2: &TriangleFace) -> bool {
        // Check if any vertex of t1 is inside t2 or vice versa
        for &p in [t1.a, t1.b, t1.c].iter() {
            if Self::point_triangle_intersection(&p, t2) {
                return true;
            }
        }
        for &p in [t2.a, t2.b, t2.c].iter() {
            if Self::point_triangle_intersection(&p, t1) {
                return true;
            }
        }
        // Check if any edge of t1 intersects t2 or vice versa
        for edge1 in [(t1.a, t1.b), (t1.b, t1.c), (t1.c, t1.a)] {
            for edge2 in [(t2.a, t2.b), (t2.b, t2.c), (t2.c, t2.a)] {
                let dir1 = edge1.1 - edge1.0;
                let dir2 = edge2.1 - edge2.0;
                let cross = dir1.cross(dir2);
                let cross_norm = cross.norm();
                if cross_norm <= 0.0 {
                    continue;
                }
                let normal = (cross / cross_norm).unwrap();
                let mut min_self = edge1.0.dot(normal);
                let mut max_self = min_self;
                let mut min_other = edge2.0.dot(normal);
                let mut max_other = min_other;
                for &vertex in [edge1.0, edge1.1].iter() {
                    let projected = vertex.dot(normal);
                    min_self = min_self.min(projected);
                    max_self = max_self.max(projected);
                }
                for &vertex in [edge2.0, edge2.1].iter() {
                    let projected = vertex.dot(normal);
                    min_other = min_other.min(projected);
                    max_other = max_other.max(projected);
                }
                if max_self < min_other || max_other < min_self {
                    return false;
                }
            }
        }
        true
    }

    /// Checks if this convex hull intersects with another convex hull using the separating axis theorem.
    pub fn intersects(&self, other: &ConvexHull) -> bool {
        match (self, other) {
            (Self::Point(p1), Self::Point(p2)) => p1 == p2,
            (Self::Point(p), Self::Line(l)) | (Self::Line(l), Self::Point(p)) => {
                Self::point_line_intersection(p, l)
            }
            (Self::Point(p), Self::Triangle(t)) | (Self::Triangle(t), Self::Point(p)) => {
                Self::point_triangle_intersection(p, t)
            }
            (Self::Point(p), Self::Polyhedron(faces))
            | (Self::Polyhedron(faces), Self::Point(p)) => {
                for face in faces {
                    if Self::point_triangle_intersection(p, face) {
                        return true;
                    }
                }
                false
            }
            (Self::Line(l1), Self::Line(l2)) => Self::line_line_intersection(l1, l2),
            (Self::Line(l), Self::Triangle(t)) | (Self::Triangle(t), Self::Line(l)) => {
                Self::line_triangle_intersection(l, t)
            }
            (Self::Line(l), Self::Polyhedron(faces)) | (Self::Polyhedron(faces), Self::Line(l)) => {
                for face in faces {
                    if Self::line_triangle_intersection(l, face) {
                        return true;
                    }
                }
                false
            }
            (Self::Triangle(t1), Self::Triangle(t2)) => {
                Self::triangle_triangle_intersection(t1, t2)
            }
            (Self::Triangle(t), Self::Polyhedron(faces))
            | (Self::Polyhedron(faces), Self::Triangle(t)) => {
                for face in faces {
                    if Self::triangle_triangle_intersection(t, face) {
                        return true;
                    }
                }
                false
            }
            (Self::Polyhedron(faces1), Self::Polyhedron(faces2)) => {
                for face1 in faces1 {
                    for face2 in faces2 {
                        if Self::triangle_triangle_intersection(face1, face2) {
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
