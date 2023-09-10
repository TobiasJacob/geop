use crate::{curves::ellipse::Ellipse, points::point::Point};

#[derive(Debug)]
pub enum EllipseEllipseIntersection {
    Ellipse(Ellipse),
    FourPoint(Point, Point, Point, Point),
    TwoPoint(Point, Point),
    OnePoint(Point),
    None,
}

pub fn ellipse_ellipse_intersection(_: &Ellipse, _: &Ellipse) -> EllipseEllipseIntersection {
    todo!("Implement ellipse_ellipse_intersection")
}
