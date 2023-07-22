use std::rc::Rc;

use geop_geometry::geometry::points::point3d::Point3d;

pub struct Vertex {
    pub point: Rc<Point3d>
}