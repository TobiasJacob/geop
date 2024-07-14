use geop_geometry::transforms::Transform;

use super::shell::Shell;

pub struct Volume {
    pub boundary: Shell,   // Normal pointing outwards
    pub holes: Vec<Shell>, // Normal pointing inwards
}

impl Volume {
    pub fn new(boundary: Shell, holes: Vec<Shell>) -> Volume {
        Volume { boundary, holes }
    }

    pub fn transform(&self, transform: Transform) -> Volume {
        Volume {
            boundary: self.boundary.transform(transform),
            holes: self.holes.iter().map(|h| h.transform(transform)).collect(),
        }
    }
}
