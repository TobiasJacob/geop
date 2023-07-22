use crate::geometry::points::{point3d::Point3d, point2d::Point2d};

pub trait Surface {
    fn get_value(&self, u: Point2d) -> Point3d;
    fn project(&self, x: Point3d) -> Point2d;
    fn normalize(&mut self);
    fn is_normalized(&self) -> bool;
}
