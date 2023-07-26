use std::rc::Rc;

use geop_geometry::geometry::points::point::Point;

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

    // Implement equality by checking that references point to the same memory adress
    pub fn equals(&self, other: &Vertex) -> bool {
        Rc::ptr_eq(&self.point, &other.point)
    }
}
