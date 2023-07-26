use crate::geometry::{curves::line3d::Line3d, points::point3d::Point3d};

use super::intersections_curve::IntersectableCurve3dResult;

pub enum LineLineIntersection {
    Line3d(Line3d),
    Point3d(Point3d),
    None
}

impl From<LineLineIntersection> for IntersectableCurve3dResult {
    fn from(intersection: LineLineIntersection) -> Self {
        match intersection {
            LineLineIntersection::Line3d(line) => IntersectableCurve3dResult::Line3d(line),
            LineLineIntersection::Point3d(point) => IntersectableCurve3dResult::Point3d(point),
            LineLineIntersection::None => IntersectableCurve3dResult::None
        }
    }
}

pub fn line_line(a: &Line3d, b: &Line3d) -> LineLineIntersection {
    let n = b.direction.cross(a.direction);
    let p = b.basis;
    let v = a.direction;
    let a = a.basis;

    if n.norm() < crate::EQ_THRESHOLD {
        if (n.dot(a - p)).abs() < crate::EQ_THRESHOLD {
            return LineLineIntersection::Line3d(Line3d::new(a, v));
        } else {
            return LineLineIntersection::None;
        }
    }

    let t = (n.dot(p) - n.dot(a)) / n.dot(v);
    LineLineIntersection::Point3d(a + v * t)
}
