use std::rc::Rc;

use super::{contour::Contour, face::Face, vertex::Vertex};

pub struct Volume {
    pub faces: Vec<Rc<Face>>,
}

pub enum VolumeIntersection {
    TouchingContour(Contour),
    CrossingContour(Contour),
    TouchingVertex(Vertex),
}

impl Volume {
    pub fn new(faces: Vec<Rc<Face>>) -> Volume {
        Volume { faces }
    }

    pub fn intersect(&self, _other: &Volume) -> Vec<Rc<VolumeIntersection>> {
        todo!("Implement intersect");
    }
}
