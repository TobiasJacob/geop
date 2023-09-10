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

    pub fn intersect(&self, other: &Object) -> Vec<Rc<ObjectIntersection>> {
        todo!("Implement intersect");
    }

    // Remeshes this object with another object, dividing it into disjoint non-intersecting faces in 6 categories.
    pub fn remesh(
        &self,
        other: &Object,
    ) -> (
        Vec<Rc<Face>>,
        Vec<Rc<Face>>,
        Vec<Rc<Face>>,
        Vec<Rc<Face>>,
        Vec<Rc<Face>>,
        Vec<Rc<Face>>,
    ) {
        todo!("Implement remesh");
    }
}
