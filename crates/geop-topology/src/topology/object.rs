use std::rc::Rc;

use super::{contour::Contour, face::Face, vertex::Vertex};

pub struct Object {
    faces: Vec<Face>,
}

pub enum ObjectIntersection {
    TouchingContour(Contour),
    CrossingContour(Contour),
    TouchingVertex(Vertex),
}

impl Object {
    pub fn new(faces: Vec<Face>) -> Object {
        Object { faces }
    }

    pub fn intersect(&self, _other: &Object) -> Vec<Rc<ObjectIntersection>> {
        todo!("Implement intersect");
    }
}
