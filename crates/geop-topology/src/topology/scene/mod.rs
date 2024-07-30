use geop_geometry::points::point::Point;

use super::{edge::Edge, face::Face, volume::Volume};

#[derive(Clone, Debug)]
pub struct Scene {
    pub volumes: Vec<Volume>,
    pub faces: Vec<Face>,
    pub edges: Vec<Edge>,
    pub points: Vec<Point>,
}
