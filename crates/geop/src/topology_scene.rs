use crate::{color::Category10Color, point::Point};

use crate::{edge::Edge, face::Face, volume::Volume};

pub struct TopologyScene {
    pub points: Vec<(Point, Category10Color)>,
    pub edges: Vec<(Edge, Category10Color)>,
    pub face: Vec<(Face, Category10Color)>,
    pub volumes: Vec<(Volume, Category10Color)>,
}

impl TopologyScene {
    pub fn new() -> TopologyScene {
        TopologyScene {
            points: Vec::new(),
            edges: Vec::new(),
            face: Vec::new(),
            volumes: Vec::new(),
        }
    }

    pub fn with_points(points: Vec<(Point, Category10Color)>) -> TopologyScene {
        TopologyScene {
            points,
            edges: Vec::new(),
            face: Vec::new(),
            volumes: Vec::new(),
        }
    }

    pub fn with_edges(edges: Vec<(Edge, Category10Color)>) -> TopologyScene {
        TopologyScene {
            points: Vec::new(),
            edges,
            face: Vec::new(),
            volumes: Vec::new(),
        }
    }

    pub fn with_faces(face: Vec<(Face, Category10Color)>) -> TopologyScene {
        TopologyScene {
            points: Vec::new(),
            edges: Vec::new(),
            face,
            volumes: Vec::new(),
        }
    }

    pub fn with_volumes(volumes: Vec<(Volume, Category10Color)>) -> TopologyScene {
        TopologyScene {
            points: Vec::new(),
            edges: Vec::new(),
            face: Vec::new(),
            volumes,
        }
    }
}
