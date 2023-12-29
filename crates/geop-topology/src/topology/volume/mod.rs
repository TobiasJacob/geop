use std::rc::Rc;

use geop_geometry::{transforms::Transform, points::point::Point, curves::line::Line};

use super::{contour::Contour, face::Face, edge::{Edge}, intersections::edge_edge::EdgeEdgeIntersection, contains::face_point::{face_contains_point, FaceContainsPoint}};

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
    OnEdge(Point, Point, Point), // Normal1, Normal2, tangent1(=negative of tangent2)
    OnPoint(Point, Point, Point), // Tangent1, tangent2, tangent3, ordered such that in right hand rule, the volume is between the fingers
}

impl VolumeNormal {
    pub fn is_from_inside(&self, curve_dir: Point) -> bool {{
        match self {
            VolumeNormal::OnFace(normal) => normal.dot(curve_dir) > 0.0,
            VolumeNormal::OnEdge(normal1, normal2, tangent1neg2) => {
                let tangent1 = normal1.cross(*tangent1neg2);
                let tangent2 = tangent1neg2.cross(*normal2);
                let tangent1 = tangent1.normalize();
                let tangent2 = tangent2.normalize();
                let tangent1neg2 = tangent1neg2.normalize();
                // Check determinant of (tangent1 - curve_dir, tangent2 - curve_dir, tangent1neg2 - curve_dir)
                let det = (tangent1 - curve_dir).cross(tangent2 - curve_dir).dot(tangent1neg2 - curve_dir);
                todo!("Write a testcase for this");
                det > 0.0
            },
            VolumeNormal::OnPoint(tangent1, tangent2, tangent3) => {
                let tangent1 = tangent1.normalize();
                let tangent2 = tangent2.normalize();
                let tangent3 = tangent3.normalize();
                // Check determinant of (tangent1 - curve_dir, tangent2 - curve_dir, tangent3 - curve_dir)
                let det = (tangent1 - curve_dir).cross(tangent2 - curve_dir).dot(tangent3 - curve_dir);
                todo!("Write a testcase for this");
                det > 0.0
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
        let mut relevant_faces = Vec::<&Rc<Face>>::new();
        for face in self.faces.iter() {
            match face_contains_point(face, point) {
                FaceContainsPoint::Inside | FaceContainsPoint::OnEdge(_) | FaceContainsPoint::OnPoint(_) => {
                    relevant_faces.push(face);
                },
                FaceContainsPoint::Outside => {}
            }
        }
        match relevant_faces.len() {
            0 => panic!("Point is not inside volume"),
            1 => VolumeNormal::OnFace(relevant_faces[0].normal(point)),
            2 => {
                VolumeNormal::OnEdge(relevant_faces[0].normal(point), relevant_faces[1].normal(point), *relevant_faces[0].boundary_tangent(point).expect_on_edge())
            }
            3 => {
                let btangent1 = relevant_faces[0].boundary_tangent(point);
                let btangent2 = relevant_faces[1].boundary_tangent(point);
                let btangent3 = relevant_faces[2].boundary_tangent(point);
                let (t1in, t1out) = btangent1.expect_on_corner();
                let (t2in, t2out) = btangent2.expect_on_corner();
                let (t3in, t3out) = btangent3.expect_on_corner();
                // Now we have to order it like right hand rule, such that the volume is between the fingers.
                // First face is between thumb and index finger
                // Now figure out if the face between index and middle finger is the second or third face
                match t1out.normalize() == t2in.normalize() {
                    true => VolumeNormal::OnPoint(*t1out, *t2out, *t3out), // Retourn all outgoing tangents
                    false => VolumeNormal::OnPoint(*t1out, *t3out, *t2out),
                }
            }
            _ => panic!("This case should never happen"),
        }
    }
}
