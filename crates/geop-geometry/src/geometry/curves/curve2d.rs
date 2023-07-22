use crate::geometry::points::point2d::Point2d;

pub trait Curve2d {
    fn point_at(&self, u: f64) -> Point2d;
    fn project(&self, x: Point2d) -> f64;
    fn normalize(&mut self);
    fn is_normalized(&self) -> bool;
}