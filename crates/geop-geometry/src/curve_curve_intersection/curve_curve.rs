use crate::{points::point::Point, curves::curve::Curve};

use super::{line_line::{LineLineIntersection, line_line_intersection}, circle_line::{CircleLineIntersection, circle_line_intersection}, circle_circle::{circle_circle_intersection, CircleCircleIntersection}};


pub enum CurveCurveIntersection {
    None,
    Points(Vec<Point>),
    Curve(Curve),
}

pub fn curve_curve_intersection(edge_self: &Curve, edge_other: &Curve) -> CurveCurveIntersection {
    match edge_self {
        Curve::Line(line) => match edge_other {
            Curve::Line(other_line) => {
                match line_line_intersection(line, other_line) {
                    LineLineIntersection::None => CurveCurveIntersection::None,
                    LineLineIntersection::Point(p) => CurveCurveIntersection::Points(vec![p]),
                    LineLineIntersection::Line(l) => CurveCurveIntersection::Curve(Curve::Line(l)),
                }
            }
            Curve::Circle(other_circle) => {
                match circle_line_intersection(other_circle, line) {
                    CircleLineIntersection::None => CurveCurveIntersection::None,
                    CircleLineIntersection::OnePoint(p) => CurveCurveIntersection::Points(vec![p]),
                    CircleLineIntersection::TwoPoint(p1, p2) => CurveCurveIntersection::Points(vec![p1, p2]),
                }
            }
            Curve::Ellipse(_other_ellipse) => {
                todo!("Line-Ellipse intersection")
            }
        },
        Curve::Circle(circle) => match edge_other {
            Curve::Line(_other_line) => {
                todo!("Circle-Line intersection")
            }
            Curve::Circle(other_circle) => {
                match circle_circle_intersection(circle, other_circle) {
                    CircleCircleIntersection::None => CurveCurveIntersection::None,
                    CircleCircleIntersection::OnePoint(p) => CurveCurveIntersection::Points(vec![p]),
                    CircleCircleIntersection::TwoPoint(p1, p2) => CurveCurveIntersection::Points(vec![p1, p2]),
                    CircleCircleIntersection::Circle(c) => CurveCurveIntersection::Curve(Curve::Circle(c)),
                }
            }
            Curve::Ellipse(_other_ellipse) => {
                todo!("Circle-Ellipse intersection")
            }
        },
        Curve::Ellipse(_ellipse) => match edge_other {
            Curve::Line(_other_line) => {
                todo!("Ellipse-Line intersection")
            }
            Curve::Circle(_other_circle) => {
                todo!("Ellipse-Circle intersection")
            }
            Curve::Ellipse(_other_ellipse) => {
                todo!("Ellipse-Ellipse intersection")
            }
        },
    }
}
