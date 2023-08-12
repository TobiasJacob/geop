use std::rc::Rc;

use geop_geometry::points::point::Point;

#[derive(Clone)]
pub struct Vertex {
    pub point: Rc<Point>
}

impl Vertex {
    pub fn new(point: Rc<Point>) -> Vertex {
        Vertex {
            point
        }
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Vertex) -> bool {
        Rc::ptr_eq(&self.point, &other.point) || *self.point == *other.point
    }
}