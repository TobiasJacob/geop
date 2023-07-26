use crate::geometry::points::{point3d::Point3d, point2d::Point2d};

pub trait Surface {
    fn point_at(&self, u: Point2d) -> Point3d;
    fn normalize(&mut self);
    fn is_normalized(&self) -> bool;
    fn period(&self) -> Point2d;
}
