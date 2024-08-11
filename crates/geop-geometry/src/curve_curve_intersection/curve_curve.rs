use crate::{curves::curve::Curve, points::point::Point};

use super::{
    circle_circle::{circle_circle_intersection, CircleCircleIntersection},
    circle_line::{circle_line_intersection, CircleLineIntersection},
    line_line::{line_line_intersection, LineLineIntersection},
};

// Finds the intersection between two curves. They have to be intersecting only at a finite number of points.
pub fn curve_curve_intersection_numerical(
    edge_self: &Curve,
    edge_other: &Curve,
    interval_self: (Option<Point>, Option<Point>),
    interval_other: (Option<Point>, Option<Point>),
) -> Vec<Point> {
    let midpoint_self = edge_self.get_midpoint(interval_self.0, interval_self.1);
    let midpoint_other = edge_other.get_midpoint(interval_other.0, interval_other.1);

    let bounding_box_self_0 = edge_self.get_bounding_box(interval_self.0, Some(midpoint_self));
    let bounding_box_self_1 = edge_self.get_bounding_box(Some(midpoint_self), interval_self.1);
    let bounding_box_other_0 = edge_other.get_bounding_box(interval_other.0, Some(midpoint_other));
    let bounding_box_other_1 = edge_other.get_bounding_box(Some(midpoint_other), interval_other.1);

    let mut result = Vec::new();
    if bounding_box_self_0.intersects(&bounding_box_other_0) {
        result.extend(curve_curve_intersection_numerical(
            edge_self,
            edge_other,
            (interval_self.0, Some(midpoint_self)),
            (interval_other.0, Some(midpoint_other)),
        ));
    }
    if bounding_box_self_0.intersects(&bounding_box_other_1) {
        result.extend(curve_curve_intersection_numerical(
            edge_self,
            edge_other,
            (interval_self.0, Some(midpoint_self)),
            (Some(midpoint_other), interval_other.1),
        ));
    }
    if bounding_box_self_1.intersects(&bounding_box_other_0) {
        result.extend(curve_curve_intersection_numerical(
            edge_self,
            edge_other,
            (Some(midpoint_self), interval_self.1),
            (interval_other.0, Some(midpoint_other)),
        ));
    }
    if bounding_box_self_1.intersects(&bounding_box_other_1) {
        result.extend(curve_curve_intersection_numerical(
            edge_self,
            edge_other,
            (Some(midpoint_self), interval_self.1),
            (Some(midpoint_other), interval_other.1),
        ));
    }
    result
}

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
