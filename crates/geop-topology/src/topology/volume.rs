use std::rc::Rc;

use geop_geometry::{transforms::Transform, points::point::Point, curves::line::Line};

use crate::topology::{face::FaceContainsPoint, edge::{EdgeCurve, Direction}};

use super::{contour::Contour, face::Face, edge::Edge};

pub struct Volume {
    pub faces: Vec<Rc<Face>>,
}


pub enum VolumeContainsPoint {
    Inside,
    OnFace(Rc<Face>),
    OnEdge(Rc<Edge>),
    OnPoint(Rc<Point>),
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

    pub fn contains_point(&self, point: Point) -> VolumeContainsPoint {
        // first check if point is on any other face
        for face in self.faces.iter() {
            match face.contains_point(point) {
                FaceContainsPoint::Inside => return VolumeContainsPoint::OnFace(face.clone()),
                FaceContainsPoint::OnEdge(edge) => return VolumeContainsPoint::OnEdge(edge),
                FaceContainsPoint::OnPoint(point) => return VolumeContainsPoint::OnPoint(point),
                FaceContainsPoint::Outside => {}
            }
        }

        // choose a random point on a face
        let q = self.faces[0].inner_point();
        let edge = Edge::new(
            Rc::new(q.clone()),
            Rc::new(point.clone()), 
            Rc::new(EdgeCurve::Line(Line::new(q, point - q))), 
            Direction::Increasing);

        // Find the closest intersection point with any other face and use the normal to determine if the point is inside or outside
        for face in self.faces.iter() {
            let intersections = face.intersect_edge_different_surface(&edge);
        }
        todo!("Implement contains_point");
    }
}
