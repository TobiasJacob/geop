use std::fmt::Display;

use crate::{algebra_error::AlgebraResult, efloat::EFloat64};
use crate::primitives::point::Point;

#[derive(Debug, Clone)]
pub struct TriangleFace {
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub normal: Point,
}

impl TriangleFace {
    pub fn try_new(a: Point, b: Point, c: Point) -> AlgebraResult<TriangleFace> {
        // Calculate the normal direction using the cross product.
        let ab = b - a;
        let ac = c - a;
        let normal = ab.cross(ac).normalize()?;
        Ok(TriangleFace { a, b, c, normal })
    }

    /// Computes the oriented distance of a point from the plane defined by this face.
    pub fn distance_to_point(&self, p: &Point) -> EFloat64 {
        let ap = *p - self.a;
        ap.dot(self.normal)
    }
}

impl Display for TriangleFace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Face: a={}, b={}, c={}, normal={}",
            self.a, self.b, self.c, self.normal
        )
    }
}

/// Computes the distance from a point `p` to the line defined by points `a` and `b`.
pub fn distance_point_to_line(p: &Point, a: &Point, b: &Point) -> EFloat64 {
    let ap = *p - *a;
    let ab = *b - *a;
    let ab_norm = ab.norm();
    if ab_norm == 0.0 {
        return ap.norm();
    }
    let cross = ap.cross(ab);
    let cross_norm = cross.norm();
    (cross_norm / ab_norm).unwrap()
}

/// Implementation of the Quickhull algorithm for 3D.
/// First, a tetrahedron (base hull) is determined, then points are added iteratively.
pub fn quickhull(mut points: Vec<Point>) -> AlgebraResult<Vec<TriangleFace>> {
    if points.len() < 4 {
        return Err("At least four points are required for convex hull".into());
    }

    // 1. Determine two points with the minimum and maximum x-values (they are definitely part of the hull).
    let mut min_x = points[0];
    let mut max_x = points[0];
    for p in points.iter() {
        if p.x < min_x.x {
            min_x = *p;
        }
        if p.x > max_x.x {
            max_x = *p;
        }
    }

    // 2. Determine the point that is farthest from the line (min_x, max_x).
    let mut max_distance = EFloat64::from(-1.0);
    let mut third_point = points[0];
    for p in points.iter() {
        let d = distance_point_to_line(p, &min_x, &max_x);
        if d > max_distance {
            max_distance = d;
            third_point = *p;
        }
    }

    // 3. Determine the point that is farthest from the plane (min_x, max_x, third_point).
    let temp_face = TriangleFace::try_new(min_x, max_x, third_point)?;
    max_distance = EFloat64::from(-1.0);
    let mut fourth_point = points[0];
    for p in points.iter() {
        let d = temp_face.distance_to_point(p).abs();
        if d > max_distance {
            max_distance = d;
            fourth_point = *p;
        }
    }

    // 4. Create the initial tetrahedron (four faces).
    let mut faces = Vec::new();
    // Check the orientation of the fourth point relative to the plane (min_x, max_x, third_point).
    if temp_face.distance_to_point(&fourth_point) < 0.0 {
        faces.push(TriangleFace::try_new(min_x, max_x, third_point)?);
        faces.push(TriangleFace::try_new(min_x, third_point, fourth_point)?);
        faces.push(TriangleFace::try_new(min_x, fourth_point, max_x)?);
        faces.push(TriangleFace::try_new(max_x, fourth_point, third_point)?);
    } else {
        faces.push(TriangleFace::try_new(min_x, third_point, max_x)?);
        faces.push(TriangleFace::try_new(min_x, fourth_point, third_point)?);
        faces.push(TriangleFace::try_new(min_x, max_x, fourth_point)?);
        faces.push(TriangleFace::try_new(max_x, third_point, fourth_point)?);
    }

    // 5. Iterative expansion of the hull:
    // For every point that lies outside (i.e., in front of a face), all visible faces are removed
    // and replaced by new faces connecting the "horizon" (boundary) of the visible faces with the point.
    let mut changed = true;
    while changed {
        changed = false;
        while let Some(p) = points.pop() {
            // Check if a point lies outside of any face.
            let mut is_outside = false;
            for face in &faces {
                if face.distance_to_point(&p) > 0.0 {
                    is_outside = true;
                    break;
                }
            }
            if is_outside {
                changed = true;
                // Find all faces that can "see" the point (visible faces).
                let mut visible_faces = Vec::new();
                for (i, face) in faces.iter().enumerate() {
                    if face.distance_to_point(&p) > 0.0 {
                        visible_faces.push(i);
                    }
                }
                // Determine the boundary edges (edges that belong only to one visible face).
                let mut boundary_edges = Vec::new();
                for &i in &visible_faces {
                    let face = &faces[i];
                    let edges = vec![(face.a, face.b), (face.b, face.c), (face.c, face.a)];
                    for edge in edges {
                        // Check if this edge also occurs in a non-visible face.
                        let mut shared = false;
                        for (j, other_face) in faces.iter().enumerate() {
                            if !visible_faces.contains(&j) {
                                let other_edges = vec![
                                    (other_face.a, other_face.b),
                                    (other_face.b, other_face.c),
                                    (other_face.c, other_face.a),
                                ];
                                if other_edges.contains(&(edge.1, edge.0)) {
                                    shared = true;
                                    break;
                                }
                            }
                        }
                        if shared {
                            boundary_edges.push(edge);
                        }
                    }
                }

                // Remove all visible faces.
                faces = faces
                    .into_iter()
                    .enumerate()
                    .filter(|(i, _)| !visible_faces.contains(i))
                    .map(|(_, f)| f)
                    .collect();
                // Create new faces connecting the boundary edges with the point.
                for edge in boundary_edges {
                    faces.push(TriangleFace::try_new(edge.0, edge.1, p)?);
                }
                // Restart the outer loop after the change.
                break;
            }
        }
    }

    Ok(faces)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_point_to_line() {
        let p = Point::from_f64(1.0, 1.0, 1.0);
        let a = Point::from_f64(0.0, 0.0, 0.0);
        let b = Point::from_f64(2.0, 0.0, 0.0);
        assert_eq!(distance_point_to_line(&p, &a, &b), 1.0);
    }

    #[test]
    fn test_quickhull() -> AlgebraResult<()> {
        let points = vec![
            Point::from_f64(0.0, 0.0, 0.0),
            Point::from_f64(1.0, 0.0, 0.0),
            Point::from_f64(0.0, 1.0, 0.0),
            Point::from_f64(0.0, 0.0, 1.0),
            Point::from_f64(1.0, 1.0, 0.0),
            Point::from_f64(1.0, 0.0, 1.0),
            Point::from_f64(0.0, 1.0, 1.0),
            Point::from_f64(1.0, 1.0, 1.0),
        ];
        let faces = quickhull(points)?;
        for face in &faces {
            println!("{}", face);
        }
        assert_eq!(faces.len(), 12);
        Ok(())
    }
}
