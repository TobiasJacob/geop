use super::point::point_triangle_intersection;
use crate::{algebra_error::AlgebraResult, efloat::EFloat64, point::Point, triangle::TriangleFace};

/// Checks if two triangles intersect using the Separating Axis Theorem
pub fn triangle_triangle_intersection(t1: &TriangleFace, t2: &TriangleFace) -> bool {
    // Get edges of both triangles
    let t1_edges = vec![t1.b - t1.a, t1.c - t1.a, t1.c - t1.b];
    let t2_edges = vec![t2.b - t2.a, t2.c - t2.a, t2.c - t2.b];

    // Check normal of t1
    let t1_normal = t1.normal;
    if !project_triangles_onto_axis(t1, t2, &t1_normal) {
        return false;
    }

    // Check normal of t2
    let t2_normal = t2.normal;
    if !project_triangles_onto_axis(t1, t2, &t2_normal) {
        return false;
    }

    // Check edges of t1
    for edge in t1_edges.iter() {
        if !project_triangles_onto_axis(t1, t2, &edge.cross(t1_normal)) {
            return false;
        }
    }

    // Check edges of t2
    for edge in t2_edges.iter() {
        if !project_triangles_onto_axis(t1, t2, &edge.cross(t2_normal)) {
            return false;
        }
    }

    // Check cross products of edges
    for t1_edge in t1_edges.iter() {
        for t2_edge in t2_edges.iter() {
            let axis = t1_edge.cross(*t2_edge);
            if axis.norm_sq() > 0.0 {
                // Avoid checking parallel edges
                if !project_triangles_onto_axis(t1, t2, &axis) {
                    return false;
                }
            }
        }
    }

    true
}

/// Projects both triangles onto the given axis and checks if their projections overlap
fn project_triangles_onto_axis(t1: &TriangleFace, t2: &TriangleFace, axis: &Point) -> bool {
    let (t1_min, t1_max) = project_triangle_onto_axis(t1, axis);
    let (t2_min, t2_max) = project_triangle_onto_axis(t2, axis);

    // println!("axis: {}", axis);
    // println!("t1: {}", t1);
    // println!("t2: {}", t2);
    // println!("t1_min: {}, t1_max: {}", t1_min, t1_max);
    // println!("t2_min: {}, t2_max: {}", t2_min, t2_max);

    // Check if projections overlap
    // No overlap if one projection is entirely on one side of the other
    !(t1_max < t2_min || t2_max < t1_min)
}

/// Projects a triangle onto the given axis and returns (min, max) of the projection
fn project_triangle_onto_axis(triangle: &TriangleFace, axis: &Point) -> (EFloat64, EFloat64) {
    let p1 = triangle.a.dot(*axis);
    let p2 = triangle.b.dot(*axis);
    let p3 = triangle.c.dot(*axis);

    (p1.min(p2).min(p3), p1.max(p2).max(p3))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra_error::AlgebraResult;

    #[test]
    fn test_triangle_triangle_intersection() -> AlgebraResult<()> {
        // Intersecting triangles
        let t1 = TriangleFace::try_new(
            Point::from_f64(0.0, 0.0, 0.0),
            Point::from_f64(1.0, 0.0, 0.0),
            Point::from_f64(0.0, 1.0, 0.0),
        )?;
        let t2 = TriangleFace::try_new(
            Point::from_f64(0.5, 0.5, 0.0),
            Point::from_f64(1.5, 0.5, 0.0),
            Point::from_f64(0.5, 1.5, 0.0),
        )?;
        assert!(triangle_triangle_intersection(&t1, &t2));

        // Non-intersecting triangles
        let t3 = TriangleFace::try_new(
            Point::from_f64(0.0, 0.0, 0.0),
            Point::from_f64(1.0, 0.0, 0.0),
            Point::from_f64(0.0, 1.0, 0.0),
        )?;
        let t4 = TriangleFace::try_new(
            Point::from_f64(2.0, 0.0, 0.0),
            Point::from_f64(3.0, 0.0, 0.0),
            Point::from_f64(2.0, 1.0, 0.0),
        )?;
        assert!(!triangle_triangle_intersection(&t3, &t4));

        // Triangles sharing a vertex
        let t5 = TriangleFace::try_new(
            Point::from_f64(0.0, 0.0, 0.0),
            Point::from_f64(1.0, 0.0, 0.0),
            Point::from_f64(0.0, 1.0, 0.0),
        )?;
        let t6 = TriangleFace::try_new(
            Point::from_f64(0.0, 0.0, 0.0),
            Point::from_f64(1.0, 0.0, 0.0),
            Point::from_f64(0.0, -1.0, 0.0),
        )?;
        assert!(triangle_triangle_intersection(&t5, &t6));

        // One triangle inside another
        let t7 = TriangleFace::try_new(
            Point::from_f64(0.0, 0.0, 0.0),
            Point::from_f64(2.0, 0.0, 0.0),
            Point::from_f64(0.0, 2.0, 0.0),
        )?;
        let t8 = TriangleFace::try_new(
            Point::from_f64(0.5, 0.5, 0.0),
            Point::from_f64(1.0, 0.5, 0.0),
            Point::from_f64(0.5, 1.0, 0.0),
        )?;
        assert!(triangle_triangle_intersection(&t7, &t8));

        // Coplanar triangles with no intersection
        let t9 = TriangleFace::try_new(
            Point::from_f64(0.0, 0.0, 0.0),
            Point::from_f64(1.0, 0.0, 0.0),
            Point::from_f64(0.0, 1.0, 0.0),
        )?;
        let t10 = TriangleFace::try_new(
            Point::from_f64(1.1, 0.0, 0.0),
            Point::from_f64(2.0, 0.0, 0.0),
            Point::from_f64(1.0, 1.0, 0.0),
        )?;
        assert!(!triangle_triangle_intersection(&t9, &t10));

        Ok(())
    }
}
