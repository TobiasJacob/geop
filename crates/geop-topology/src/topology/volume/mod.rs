use std::rc::Rc;

use geop_geometry::{transforms::Transform, points::point::Point, curves::line::Line};

use super::{contour::Contour, face::Face, edge::{Edge, edge_curve::EdgeCurve}, intersections::edge_edge::EdgeEdgeIntersection, contains::face_point::{face_contains_point, FaceContainsPoint}};

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

pub enum VolumeNormal {
    OnFace(Point),
    OnEdge(Point, Point),
    OnPoint(Vec<Point>),
}

impl VolumeNormal {
    pub fn is_from_inside(&self, point: Point) -> bool {{
        match self {
            VolumeNormal::OnFace(normal) => normal.dot(point) > 0.0,
            VolumeNormal::OnEdge(normal1, normal2) => normal1.dot(point) > 0.0 && normal2.dot(point) > 0.0,
            VolumeNormal::OnPoint(normals) => {
                for normal in normals {
                    if normal.dot(point) < 0.0 {
                        return false;
                    }
                }
                true
            }
        }
    }}
}

#[derive(Debug)]
pub enum VolumeShellIntersection {
    Contour(Contour),
    Point(Point),
}

impl Volume {
    pub fn new(faces: Vec<Rc<Face>>) -> Volume {
        assert!(faces.len() > 0, "Volume must have at least one face");
        Volume { faces }
    }
    
    pub fn transform(&self, transform: Transform) -> Volume {
        Volume { faces: self.faces.iter().map(|f| Rc::new(f.transform(transform))).collect() }
    }

    pub fn normal(&self, point: Point) -> VolumeNormal {
        let mut relevant_normals = Vec::<Point>::new();
        for face in self.faces.iter() {
            match face_contains_point(face, point) {
                FaceContainsPoint::Inside | FaceContainsPoint::OnEdge(_) | FaceContainsPoint::OnPoint(_) => {
                    relevant_normals.push(face.normal(point));
                },
                FaceContainsPoint::Outside => {}
            }
        }
        match relevant_normals.len() {
            0 => panic!("Point is not inside volume"),
            1 => VolumeNormal::OnFace(relevant_normals[0]),
            2 => {
                VolumeNormal::OnEdge(relevant_normals[0], relevant_normals[1])
            }
            _ => {
                VolumeNormal::OnPoint(relevant_normals)
            }
        }
    }
}
