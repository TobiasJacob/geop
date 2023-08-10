use crate::{curves::line::Line, points::point::Point};

pub enum LineLineIntersection {
    Line(Line),
    Point(Point),
    None
}

pub fn line_line_intersection(a: &Line, b: &Line) -> LineLineIntersection {
    let n = b.direction.cross(a.direction);
    let p = b.basis;
    let v = a.direction;
    let a = a.basis;

    if n.norm() < crate::EQ_THRESHOLD {
        if (n.dot(a - p)).abs() < crate::EQ_THRESHOLD {
            return LineLineIntersection::Line(Line::new(a, v));
        } else {
            return LineLineIntersection::None;
        }
    }

    let t = (n.dot(p) - n.dot(a)) / n.dot(v);
    LineLineIntersection::Point(a + v * t)
}
