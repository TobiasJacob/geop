use crate::{curves::{circle::Circle, line::Line}, points::point::Point, EQ_THRESHOLD};

#[derive(Debug)]
pub enum CircleLineIntersection {
    TwoPoint(Point, Point),
    OnePoint(Point),
    None
}

pub fn circle_line_intersection(a: &Circle, b: &Line) -> CircleLineIntersection {
    assert!(a.normal.is_perpendicular(b.direction), "3D circle-line intersection is not implemented yet");
    // Assume that the line is normalized
    let v = b.basis - a.basis;
    let v = v.dot(b.direction);
    let w = v * v - (v * v - a.radius * a.radius);
    if w < -EQ_THRESHOLD {
        CircleLineIntersection::None
    } else if w < EQ_THRESHOLD {
        CircleLineIntersection::OnePoint(a.basis + b.direction * v)
    } else {
        let w = w.sqrt();
        CircleLineIntersection::TwoPoint(a.basis + b.direction * (v - w), a.basis + b.direction * (v + w))
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
            },
            _ => panic!("Expected two point intersection")
        }
    }
}