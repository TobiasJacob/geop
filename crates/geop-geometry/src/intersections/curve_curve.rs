
use crate::geometry::curves::circle::Circle;
use crate::geometry::curves::curve::Curve;
use crate::geometry::curves::ellipse::Ellipse;
use crate::geometry::curves::line::Line;
use crate::geometry::points::point::Point;

use super::line_line::line_line;

pub enum IntersectableCurve3dResult {
    MultiPoint(Vec<Point>),
    Ellipse3d(Ellipse),
    Circle(Circle),
    Line3d(Line),
    Point3d(Point),
    None
}

pub enum IntersectableCurve3d {
    Line3d(Line),
    Circle3d(Circle),
    Ellipse3d(Ellipse),
}

impl IntersectableCurve3d {
    // Get the curve.
    pub fn curve(&self) -> &dyn Curve {
        match self {
            IntersectableCurve3d::Line3d(line) => line,
            IntersectableCurve3d::Circle3d(circle) => circle,
            IntersectableCurve3d::Ellipse3d(ellipse) => ellipse,
        }
    }

    // Returns a sorted list of intersections, starting with the smallest parameter value.
    pub fn intersections(&self, other: &IntersectableCurve3d) -> IntersectableCurve3dResult {
        match self {
            IntersectableCurve3d::Line3d(line) => match other {
                IntersectableCurve3d::Line3d(other_line) => {
                    line_line(line, other_line).into()
                },
                IntersectableCurve3d::Circle3d(circle) => {
                    todo!("asdf")
                },
                IntersectableCurve3d::Ellipse3d(ellipse) => {
                    todo!("asdf")
                }
            },
            _ => {todo!("asdf")}
        }
    }
}