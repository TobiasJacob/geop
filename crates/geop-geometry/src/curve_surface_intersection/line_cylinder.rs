use crate::{curves::line::Line, point::Point, surfaces::cylinder::Cylinder, EQ_THRESHOLD};

pub enum CylinderLineIntersection {
    Line(Line),
    TwoPoints(Point, Point),
    Point(Point),
    None,
}

pub fn line_cylinder_intersection(line: &Line, cylinder: &Cylinder) -> CylinderLineIntersection {
    if cylinder.extend_dir.is_parallel(line.direction) {
        let distance = line.basis - cylinder.basis;
        let distance = distance - distance.dot(cylinder.extend_dir) * cylinder.extend_dir;
        let radius = distance.norm();
        if (radius - cylinder.radius.norm()).abs() < f64::EPSILON {
            return CylinderLineIntersection::Line(line.clone());
        }
        return CylinderLineIntersection::None;
    }

    // see https://en.wikipedia.org/wiki/Line-cylinder_intersection
    // The math is easier if we translate everything so that the line passes through the origin
    // So we point b as the cylinder's basis, in the coordinate frame of the line
    let b = cylinder.basis - line.basis;
    let a = cylinder.extend_dir.normalize().unwrap();
    let r = cylinder.radius.norm();

    let n = line.direction.clone().normalize().unwrap();

    let n_cross_a = n.cross(a);
    let left_term = n_cross_a.dot(b.cross(a));
    let bottom_term = n_cross_a.dot(n_cross_a);
    let determinant_sq = n_cross_a.dot(n_cross_a * r * r) - b.dot(n_cross_a).powi(2);

    if determinant_sq < 0.0 {
        return CylinderLineIntersection::None;
    }
    if determinant_sq.abs() < EQ_THRESHOLD {
        let d = left_term / bottom_term;
        let p = line.basis + line.direction * d;
        return CylinderLineIntersection::Point(p);
    }

    let determinant = determinant_sq.sqrt();
    let d1 = (left_term + determinant) / bottom_term;
    let d2 = (left_term - determinant) / bottom_term;

    let p1 = line.basis + line.direction * d1;
    let p2 = line.basis + line.direction * d2;

    return CylinderLineIntersection::TwoPoints(p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{curves::line::Line, point::Point, surfaces::cylinder::Cylinder};

    #[test]
    fn test_line_cylinder_intersection_line() {
        let line = Line::new(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 1.0));
        let cylinder = Cylinder::new(
            Point::new(10.0, 0.0, 0.0),
            Point::new(0.0, 0.0, 1.0),
            10.0,
            true,
        );
        match line_cylinder_intersection(&line, &cylinder) {
            CylinderLineIntersection::Line(line) => {
                assert_eq!(line.basis, Point::new(0.0, 0.0, 0.0));
                assert_eq!(line.direction, Point::new(0.0, 0.0, 1.0));
            }
            _ => {
                panic!("Should be a line!");
            }
        }
    }

    #[test]
    fn test_line_cylinder_intersection_none() {
        let line = Line::new(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 1.0));
        let cylinder = Cylinder::new(
            Point::new(10.0, 0.0, 0.0),
            Point::new(0.0, 0.0, 1.0),
            1.0,
            true,
        );
        match line_cylinder_intersection(&line, &cylinder) {
            CylinderLineIntersection::None => {}
            _ => {
                panic!("Should be None!");
            }
        }

        let line_2 = Line::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 1.0, 1.0));
        match line_cylinder_intersection(&line_2, &cylinder) {
            CylinderLineIntersection::None => {}
            _ => {
                panic!("Should be None!");
            }
        }
    }

    #[test]
    fn test_line_cylinder_intersection_two_points() {
        let line = Line::new(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 1.0));
        let cylinder = Cylinder::new(
            Point::new(10.0, 0.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            1.5,
            true,
        );
        match line_cylinder_intersection(&line, &cylinder) {
            CylinderLineIntersection::TwoPoints(p1, p2) => {
                assert_eq!(p1, Point::new(0.0, 0.0, 1.5));
                assert_eq!(p2, Point::new(0.0, 0.0, -1.5));
            }
            _ => {
                panic!("Should be two points!");
            }
        }
    }

    #[test]
    fn test_line_cylinder_intersection_point() {
        let cylinder = Cylinder::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(0.0, 0.0, 1.0),
            1.0,
            true,
        );
        let line = Line::new(Point::new(1.0, 0.0, 1.0), Point::new(0.0, 1.0, 0.0));
        match line_cylinder_intersection(&line, &cylinder) {
            CylinderLineIntersection::Point(p) => {
                assert_eq!(p, Point::new(1.0, 0.0, 1.0));
            }
            _ => {
                panic!("Should be a point!");
            }
        }
    }
}
