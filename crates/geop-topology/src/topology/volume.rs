use std::rc::Rc;

use geop_geometry::{transforms::Transform, points::point::Point};

use super::{contour::Contour, face::Face};

pub struct Volume {
    pub faces: Vec<Rc<Face>>,
}


pub enum VolumeContainsPoint {
    Inside,
    OnFace,
    OnEdge,
    OnPoint,
    Outside,
}

impl Volume {
    pub fn new(faces: Vec<Rc<Face>>) -> Volume {
        assert!(faces.len() > 0, "Volume must have at least one face");
        Volume { faces }
    }
    
    pub fn transform(&self, transform: Transform) -> Volume {
        Volume { faces: self.faces.iter().map(|f| Rc::new(f.transform(transform))).collect() }
    }

    pub fn contains_point(&self, point: Point) -> bool {
        todo!("Implement contains_point");
    }
}
