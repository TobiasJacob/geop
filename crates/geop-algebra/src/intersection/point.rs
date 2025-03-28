use crate::{line::Line, point::Point, triangle::TriangleFace};

/// Checks if a point intersects with a line segment
pub fn point_line_intersection(p: &Point, l: &Line) -> bool {
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
pub fn point_triangle_intersection(p: &Point, t: &TriangleFace) -> bool {
    let v0 = t.b - t.a;
    let v1 = t.c - t.a;
    let v2 = *p - t.a;
    let dot00 = v0.dot(v0);
    let dot01 = v0.dot(v1);
    let dot02 = v0.dot(v2);
    let dot11 = v1.dot(v1);
    let dot12 = v1.dot(v2);
    let inv_denom = crate::efloat::EFloat64::from(1.0) / (dot00 * dot11 - dot01 * dot01);
    if let Ok(inv_denom) = inv_denom {
        let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
        let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;
        return u >= 0.0 && v >= 0.0 && u + v <= 1.0;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra_error::AlgebraResult;

    #[test]
    fn test_point_line_intersection() -> AlgebraResult<()> {
        let p1 = Point::from_f64(0.0, 0.0, 0.0);
        let p2 = Point::from_f64(1.0, 1.0, 1.0);
        let line = Line::try_new(p1, p2)?;

        // Point on line
        let p_on = Point::from_f64(0.5, 0.5, 0.5);
        assert!(point_line_intersection(&p_on, &line));

        // Point off line
        let p_off = Point::from_f64(0.5, 0.5, 1.0);
        assert!(!point_line_intersection(&p_off, &line));

        // Point beyond line
        let p_beyond = Point::from_f64(2.0, 2.0, 2.0);
        assert!(!point_line_intersection(&p_beyond, &line));

        Ok(())
    }

    #[test]
    fn test_point_triangle_intersection() -> AlgebraResult<()> {
        let p1 = Point::from_f64(0.0, 0.0, 0.0);
        let p2 = Point::from_f64(1.0, 0.0, 0.0);
        let p3 = Point::from_f64(0.0, 1.0, 0.0);
        let triangle = TriangleFace::try_new(p1, p2, p3)?;

        // Point inside triangle
        let p_inside = Point::from_f64(0.25, 0.25, 0.0);
        assert!(point_triangle_intersection(&p_inside, &triangle));

        // Point outside triangle
        let p_outside = Point::from_f64(1.0, 1.0, 0.0);
        assert!(!point_triangle_intersection(&p_outside, &triangle));

        // Point on triangle edge
        let p_edge = Point::from_f64(0.5, 0.0, 0.0);
        assert!(point_triangle_intersection(&p_edge, &triangle));

        // Point on triangle vertex
        let p_vertex = Point::from_f64(0.0, 0.0, 0.0);
        assert!(point_triangle_intersection(&p_vertex, &triangle));

        Ok(())
    }
}
