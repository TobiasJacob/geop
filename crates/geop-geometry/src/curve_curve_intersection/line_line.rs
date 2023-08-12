use crate::{curves::line::Line, points::point::Point};

#[derive(Debug)]
pub enum LineLineIntersection {
    Line(Line),
    Point(Point),
    None
}

pub fn line_line_intersection(a: &Line, b: &Line) -> LineLineIntersection {
    let v1 = a.direction;
    let v2 = b.direction;
    let p1 = a.basis;
    let p2 = b.basis;

    if v1.is_parallel(v2) {
        if (p1 - p2).is_parallel(v1) {
            return LineLineIntersection::Line(Line::new(p1, v1));
        } else {
            return LineLineIntersection::None;
        }
    }

    let t = (p2 - p1).cross(v2).dot(v1) / v1.cross(v2).norm_sq();
    let s = (p2 - p1).cross(v1).dot(v2) / v1.cross(v2).norm_sq();

    if t.is_finite() && s.is_finite() {
        return LineLineIntersection::Point(p1 + v1 * t);
    } else {
        return LineLineIntersection::None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_line_intersection() {
        let l1 = Line::new(Point::new(-2.0, 1.0, 4.0), Point::new(1.0, 0.0, 0.0));
        let l2 = Line::new(Point::new(-2.0, 1.0, 4.0), Point::new(0.0, 1.0, 0.0));
        let i = line_line_intersection(&l1, &l2);
        match i {
            LineLineIntersection::Point(p) => {
                assert_eq!(p, Point::new(-2.0, 1.0, 4.0));
            },
            _ => panic!("Expected point intersection")
        }
    }
}