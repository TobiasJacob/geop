use crate::{line::Line, triangle::TriangleFace};

/// Checks if two line segments intersect
pub fn line_line_intersection(l1: &Line, l2: &Line) -> bool {
    let p = l1.start();
    let r = l1.direction();
    let q = l2.start();
    let s = l2.direction();

    let rxs = r.cross(s);

    // Check if rxs is nearly zero -> lines are parallel
    if rxs.norm() == 0.0 {
        // Check if the segments are collinear
        if (q - p).cross(r).norm() == 0.0 {
            // Project q onto r to get a parameter t0 and check overlap
            let t0 = (q - p).dot(r) / r.norm_sq();
            if let Ok(t0) = t0 {
                let tp = s.dot(r) / r.norm_sq();
                if let Ok(tp) = tp {
                    let t1 = t0 + tp;
                    return (t0 >= 0.0 && t0 <= 1.0) || (t1 >= 0.0 && t1 <= 1.0);
                }
            }
        }
        return false;
    }

    // For non-parallel lines, compute intersection parameters
    let t = (q - p).cross(s).dot(rxs) / rxs.norm_sq();
    let u = (q - p).cross(r).dot(rxs) / rxs.norm_sq();

    if let (Ok(t), Ok(u)) = (t, u) {
        t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0
    } else {
        false
    }
}

/// Checks if a line segment intersects with a triangle
pub fn line_triangle_intersection(l: &Line, t: &TriangleFace) -> bool {
    let dir = l.direction();
    let v0 = t.b - t.a;
    let v1 = t.c - t.a;
    let v2 = l.start() - t.a;
    let cross = dir.cross(v1);
    let det = v0.dot(cross);
    if det.abs() <= 0.0 {
        return false;
    }
    let inv_det = crate::efloat::EFloat64::from(1.0) / det;
    if let Ok(inv_det) = inv_det {
        let u = v2.dot(cross) * inv_det;
        let v = dir.dot(v2.cross(v0)) * inv_det;
        let t_param = v1.dot(v2.cross(v0)) * inv_det;
        return u >= 0.0 && v >= 0.0 && u + v <= 1.0 && t_param >= 0.0 && t_param <= 1.0;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{algebra_error::AlgebraResult, point::Point};

    #[test]
    fn test_line_line_intersection() -> AlgebraResult<()> {
        // Intersecting lines
        let l1 = Line::new(
            Point::from_f64(0.0, 0.0, 0.0),
            Point::from_f64(1.0, 1.0, 0.0),
        );
        let l2 = Line::new(
            Point::from_f64(0.0, 1.0, 0.0),
            Point::from_f64(1.0, 0.0, 0.0),
        );
        assert!(line_line_intersection(&l1, &l2));

        // Parallel non-intersecting lines
        let l3 = Line::new(
            Point::from_f64(0.0, 0.0, 0.0),
            Point::from_f64(1.0, 0.0, 0.0),
        );
        let l4 = Line::new(
            Point::from_f64(0.0, 1.0, 0.0),
            Point::from_f64(1.0, 1.0, 0.0),
        );
        assert!(!line_line_intersection(&l3, &l4));

        // Skew lines
        let l5 = Line::new(
            Point::from_f64(0.0, 0.0, 0.0),
            Point::from_f64(1.0, 0.0, 0.0),
        );
        let l6 = Line::new(
            Point::from_f64(0.0, 0.0, 1.0),
            Point::from_f64(1.0, 0.0, 1.0),
        );
        assert!(!line_line_intersection(&l5, &l6));

        Ok(())
    }

    #[test]
    fn test_line_triangle_intersection() -> AlgebraResult<()> {
        let triangle = TriangleFace::try_new(
            Point::from_f64(0.0, 0.0, 0.0),
            Point::from_f64(1.0, 0.0, 0.0),
            Point::from_f64(0.0, 1.0, 0.0),
        )?;

        // Line intersecting triangle
        let l1 = Line::new(
            Point::from_f64(0.25, 0.25, -1.0),
            Point::from_f64(0.25, 0.25, 1.0),
        );
        assert!(line_triangle_intersection(&l1, &triangle));

        // Line missing triangle
        let l2 = Line::new(
            Point::from_f64(0.25, 0.25, 1.0),
            Point::from_f64(0.25, 0.25, 2.0),
        );
        assert!(!line_triangle_intersection(&l2, &triangle));

        // Line parallel to triangle
        let l3 = Line::new(
            Point::from_f64(0.0, 0.0, 1.0),
            Point::from_f64(1.0, 0.0, 1.0),
        );
        assert!(!line_triangle_intersection(&l3, &triangle));

        Ok(())
    }
}
