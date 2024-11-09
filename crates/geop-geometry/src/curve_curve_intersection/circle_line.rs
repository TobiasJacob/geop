use crate::{
    curves::{circle::Circle, line::Line},
    point::Point,
};

#[derive(Debug)]
pub enum CircleLineIntersection {
    TwoPoint(Point, Point),
    OnePoint(Point),
    None,
}
pub fn circle_line_intersection(circle: &Circle, line: &Line) -> CircleLineIntersection {
    if circle.normal.is_parallel(line.direction) {
        let diff = circle.basis - line.basis;
        let distance = diff.dot(line.direction);
        let projection = diff - distance * line.direction;
        if (projection.norm() - circle.radius.norm()) == 0.0 {
            let point = line.basis + distance * line.direction;
            return CircleLineIntersection::OnePoint(point);
        }
        return CircleLineIntersection::None;
    }

    assert!(
        circle.normal.is_perpendicular(line.direction),
        "3D circle-line intersection is not implemented yet"
    );

    let v = circle.basis - line.basis;
    let dir = line.direction.normalize().unwrap();
    let projection = v.dot(dir);
    let distance_sq = v.norm_sq() - projection * projection;
    let radius_sq = circle.radius.norm_sq();

    if distance_sq > radius_sq.lower_bound {
        CircleLineIntersection::None
    } else if (radius_sq - distance_sq) == 0.0 {
        CircleLineIntersection::OnePoint(line.basis + dir * projection)
    } else {
        let offset = (radius_sq - distance_sq).sqrt().unwrap();
        CircleLineIntersection::TwoPoint(
            line.basis + dir * (projection - offset),
            line.basis + dir * (projection + offset),
        )
    }
}

#[cfg(test)]
mod tests {
    use geop_algebra::efloat::EFloat64;

    use super::*;

    #[test]
    fn test_circle_line_intersection() {
        let c = Circle::new(
            Point::from_f64(0.0, 0.0, 0.0),
            Point::from_f64(0.0, 0.0, 1.0),
            EFloat64::one(),
        );
        let l = Line::new(
            Point::from_f64(0.0, 0.0, 0.0),
            Point::from_f64(1.0, 0.0, 0.0),
        )
        .unwrap();
        let i = circle_line_intersection(&c, &l);
        match i {
            CircleLineIntersection::TwoPoint(p1, p2) => {
                assert_eq!(p1, Point::from_f64(-1.0, 0.0, 0.0));
                assert_eq!(p2, Point::from_f64(1.0, 0.0, 0.0));
            }
            _ => panic!("Expected two point intersection"),
        }
    }
}
