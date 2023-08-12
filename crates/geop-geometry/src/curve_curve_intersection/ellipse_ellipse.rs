use crate::{points::point::Point, curves::ellipse::Ellipse};

#[derive(Debug)]
pub enum EllipseEllipseIntersection {
    Ellipse(Ellipse),
    FourPoint(Point, Point, Point, Point),
    TwoPoint(Point, Point),
    OnePoint(Point),
    None
}

pub fn ellipse_ellipse_intersection(_: &Ellipse, _: &Ellipse) -> EllipseEllipseIntersection {
    todo!("Implement ellipse_ellipse_intersection")
}
