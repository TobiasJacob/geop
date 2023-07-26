use crate::geometry::points::point::Point;

pub trait Surface {
    fn point_at(&self, u: f64, v: f64) -> Point;
    fn normalize(&mut self);
    fn is_normalized(&self) -> bool;
}
