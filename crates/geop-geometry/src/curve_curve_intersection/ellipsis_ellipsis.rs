use crate::{
    curves::{curve::Curve, ellipsis::Ellipsis},
    points::point::Point,
};

use super::curve_curve_intersection_numerical;

pub enum EllipsisEllipsisIntersection {
    Ellipsis(Ellipsis),
    OnePoint(Point),
    TwoPoint(Point, Point),
    FourPoint(Point, Point, Point, Point),
    None,
}

pub fn ellipsis_ellipsis_intersection(
    ellipsis_self: &Ellipsis,
    ellipsis_other: &Ellipsis,
) -> EllipsisEllipsisIntersection {
    if ellipsis_self == ellipsis_other {
        return EllipsisEllipsisIntersection::Ellipsis(ellipsis_self.clone());
    }

    let intersection_points = curve_curve_intersection_numerical(ellipsis_self, ellipsis_other);
    match intersection_points.len() {
        0 => EllipsisEllipsisIntersection::None,
        1 => EllipsisEllipsisIntersection::OnePoint(intersection_points[0]),
        2 => EllipsisEllipsisIntersection::TwoPoint(intersection_points[0], intersection_points[1]),
        4 => EllipsisEllipsisIntersection::FourPoint(
            intersection_points[0],
            intersection_points[1],
            intersection_points[2],
            intersection_points[3],
        ),
        _ => panic!("Unexpected number of intersection points"),
    }
}
