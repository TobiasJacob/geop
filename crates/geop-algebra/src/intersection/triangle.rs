use crate::triangle::TriangleFace;

/// Checks if two triangles intersect
pub fn triangle_triangle_intersection(t1: &TriangleFace, t2: &TriangleFace) -> bool {
    // Check if any edge of t1 intersects with any edge of t2
    let edges1 = [(t1.a, t1.b), (t1.b, t1.c), (t1.c, t1.a)];
    let edges2 = [(t2.a, t2.b), (t2.b, t2.c), (t2.c, t2.a)];

    for (p1, p2) in edges1.iter() {
        for (p3, p4) in edges2.iter() {
            let dir1 = *p2 - *p1;
            let dir2 = *p4 - *p3;
            let cross = dir1.cross(dir2);
            let cross_norm_sq = cross.norm_sq();
            if cross_norm_sq == 0.0 {
                continue;
            }
            let dir = *p3 - *p1;
            let t = dir.cross(dir2).dot(cross) / cross_norm_sq;
            let u = dir.cross(dir1).dot(cross) / cross_norm_sq;
            if let (Ok(t), Ok(u)) = (t, u) {
                if t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0 {
                    return true;
                }
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{algebra_error::AlgebraResult, point::Point};

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

        Ok(())
    }
}
