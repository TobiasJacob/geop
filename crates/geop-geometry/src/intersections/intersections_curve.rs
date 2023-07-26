
use crate::geometry::curves::circle3d::Circle3d;
use crate::geometry::curves::curve3d::Curve3d;
use crate::geometry::curves::ellipse3d::Ellipse3d;
use crate::geometry::curves::line3d::Line3d;
use crate::geometry::points::point3d::Point3d;

use super::line_line::{self, line_line};

pub enum IntersectableCurve3dResult {
    MultiPoint(Vec<Point3d>),
    Ellipse3d(Ellipse3d),
    Circle(Circle3d),
    Line3d(Line3d),
    Point3d(Point3d),
    None
}

pub enum IntersectableCurve3d {
    Line3d(Line3d),
    Circle3d(Circle3d),
    Ellipse3d(Ellipse3d),
}

impl IntersectableCurve3d {
    // Get the curve.
    pub fn curve(&self) -> &dyn Curve3d {
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

    pub fn period(&self) -> f64 {
        match self {
            IntersectableCurve3d::Line3d(line) => line.period(),
            _ => {todo!("asdf")}
        }
    }
}