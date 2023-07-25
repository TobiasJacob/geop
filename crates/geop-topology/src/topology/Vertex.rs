use std::rc::Rc;

use geop_geometry::geometry::points::point3d::Point3d;

#[derive(Clone)]
pub struct Vertex {
    pub point: Rc<Point3d>
}

impl Vertex {
    pub fn new(point: Rc<Point3d>) -> Vertex {
        Vertex {
            point
        }
    }

    // Implement equality by checking that references point to the same memory adress
    pub fn equals(&self, other: &Vertex) -> bool {
        Rc::ptr_eq(&self.point, &other.point)
    }
}
