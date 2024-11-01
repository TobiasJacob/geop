use crate::{
    curves::{circle::Circle, line::Line},
    point::Point,
    EQ_THRESHOLD,
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
        if (projection.norm() - circle.radius.norm()).abs() < EQ_THRESHOLD {
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
    let dir = line.direction.normalize();
    let projection = v.dot(dir);
    let distance_sq = v.norm_sq() - projection * projection;
    let radius_sq = circle.radius.norm_sq();

    if distance_sq > radius_sq + EQ_THRESHOLD {
        CircleLineIntersection::None
    } else if (radius_sq - distance_sq).abs() < EQ_THRESHOLD {
        CircleLineIntersection::OnePoint(line.basis + dir * projection)
    } else {
        let offset = (radius_sq - distance_sq).sqrt();
        CircleLineIntersection::TwoPoint(
            line.basis + dir * (projection - offset),
            line.basis + dir * (projection + offset),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_line_intersection() {
        let c = Circle::new(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 1.0), 1.0);
        let l = Line::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0));
        let i = circle_line_intersection(&c, &l);
        match i {
            CircleLineIntersection::TwoPoint(p1, p2) => {
                assert_eq!(p1, Point::new(-1.0, 0.0, 0.0));
                assert_eq!(p2, Point::new(1.0, 0.0, 0.0));
            }
            _ => panic!("Expected two point intersection"),
        }
    }
}
