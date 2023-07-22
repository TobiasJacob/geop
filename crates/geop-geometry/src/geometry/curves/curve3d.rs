use crate::geometry::points::point3d::Point3d;

pub trait Curve3d {
    fn get_value(&self, u: f64) -> Point3d;
    fn project(&self, x: Point3d) -> f64;
    fn normalize(&mut self);
    fn is_normalized(&self) -> bool;
}