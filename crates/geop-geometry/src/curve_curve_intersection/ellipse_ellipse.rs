use crate::{curves::ellipse::Ellipse, points::point::Point};

use super::numerical::curve_curve_intersection_numerical;

pub enum EllipseEllipseIntersection {
    Ellipse(Ellipse),
    OnePoint(Point),
    TwoPoint(Point, Point),
    ThreePoint(Point, Point, Point),
    FourPoint(Point, Point, Point, Point),
    None,
}

pub fn ellipse_ellipse_intersection(
    ellipse_self: &Ellipse,
    ellipse_other: &Ellipse,
) -> EllipseEllipseIntersection {
    if ellipse_self == ellipse_other {
        return EllipseEllipseIntersection::Ellipse(ellipse_self.clone());
    }

    let intersection_points = curve_curve_intersection_numerical(ellipse_self, ellipse_other);
    match intersection_points.len() {
        0 => EllipseEllipseIntersection::None,
        1 => EllipseEllipseIntersection::OnePoint(intersection_points[0]),
        2 => EllipseEllipseIntersection::TwoPoint(intersection_points[0], intersection_points[1]),
        3 => EllipseEllipseIntersection::ThreePoint(
            intersection_points[0],
            intersection_points[1],
            intersection_points[2],
        ),
        4 => EllipseEllipseIntersection::FourPoint(
            intersection_points[0],
            intersection_points[1],
            intersection_points[2],
            intersection_points[3],
        ),
        _ => panic!("Unexpected number of intersection points"),
    }
}
