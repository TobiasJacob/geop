use crate::{curves::curve::Curve, points::point::Point};

use super::{
    circle_circle::{circle_circle_intersection, CircleCircleIntersection},
    circle_line::{circle_line_intersection, CircleLineIntersection},
    line_line::{line_line_intersection, LineLineIntersection},
};

pub enum CurveCurveIntersection {
    None,
    Points(Vec<Point>),
    Curve(Curve),
}

pub fn curve_curve_intersection(edge_self: &Curve, edge_other: &Curve) -> CurveCurveIntersection {
    match edge_self {
        Curve::Line(line) => match edge_other {
            Curve::Line(other_line) => match line_line_intersection(line, other_line) {
                LineLineIntersection::None => CurveCurveIntersection::None,
                LineLineIntersection::Point(p) => CurveCurveIntersection::Points(vec![p]),
                LineLineIntersection::Line(l) => CurveCurveIntersection::Curve(Curve::Line(l)),
            },
            Curve::Circle(other_circle) => match circle_line_intersection(other_circle, line) {
                CircleLineIntersection::None => CurveCurveIntersection::None,
                CircleLineIntersection::OnePoint(p) => CurveCurveIntersection::Points(vec![p]),
                CircleLineIntersection::TwoPoint(p1, p2) => {
                    CurveCurveIntersection::Points(vec![p1, p2])
                }
            },
            Curve::Ellipsis(_) => todo!("Implement this"),
        },
        Curve::Circle(circle) => match edge_other {
            Curve::Line(other_line) => match circle_line_intersection(circle, other_line) {
                CircleLineIntersection::None => CurveCurveIntersection::None,
                CircleLineIntersection::OnePoint(p) => CurveCurveIntersection::Points(vec![p]),
                CircleLineIntersection::TwoPoint(p1, p2) => {
                    CurveCurveIntersection::Points(vec![p1, p2])
                }
            },
            Curve::Circle(other_circle) => match circle_circle_intersection(circle, other_circle) {
                CircleCircleIntersection::None => CurveCurveIntersection::None,
                CircleCircleIntersection::OnePoint(p) => CurveCurveIntersection::Points(vec![p]),
                CircleCircleIntersection::TwoPoint(p1, p2) => {
                    CurveCurveIntersection::Points(vec![p1, p2])
                }
                CircleCircleIntersection::Circle(c) => {
                    CurveCurveIntersection::Curve(Curve::Circle(c))
                }
            },
            Curve::Ellipsis(_) => todo!("Implement this"),
        },
        Curve::Ellipsis(_) => todo!("Implement this"),
    }
}
