use crate::geometry::points::point3d::Point3d;

use super::curve3d::Curve3d;

pub struct Circle3d {
    pub basis: Point3d,
    pub dir_u: Point3d,
    pub dir_v: Point3d
}
