use geop_geometry::{points::point::Point, transforms::Transform};

use crate::contains::face_point::{face_point_contains, FacePointContains};

use super::face::Face;

pub struct Shell {
    pub faces: Vec<Face>,
}

pub enum ShellNormal {
    OnFace(Point),
    OnEdge(Point, Point, Point), // Normal1, Normal2, tangent1(=negative of tangent2)
    OnPoint(Point, Point, Point), // Tangent1, tangent2, tangent3, ordered such that in right hand rule, the Shell is between the fingers
}

impl ShellNormal {
    pub fn is_from_inside(&self, curve_dir: Point) -> bool {
        {
            match self {
                ShellNormal::OnFace(normal) => normal.dot(curve_dir) > 0.0,
                ShellNormal::OnEdge(normal1, normal2, tangent1neg2) => {
                    let face1dir = normal1.cross(*tangent1neg2).normalize();
                    let face2dir = tangent1neg2.cross(*normal2).normalize();
                    // check determinant of the matrix [tangent1neg2, face1dir - curve_dir, face2dir - curve_dir]
                    let curve_dir = -curve_dir.normalize();
                    let det = (face1dir - curve_dir)
                        .cross(face2dir - curve_dir)
                        .dot(*tangent1neg2 - curve_dir);
                    det > 0.0
                }
                ShellNormal::OnPoint(tangent1, tangent2, tangent3) => {
                    // check determinant of the matrix [tangent1, face1dir - curve_dir, face2dir - curve_dir]
                    let tangent1 = tangent1.normalize();
                    let tangent2 = tangent2.normalize();
                    let tangent3 = tangent3.normalize();
                    let curve_dir = -curve_dir.normalize();
                    let curve_dir = -curve_dir.normalize();
                    let det = (tangent1 - curve_dir)
                        .cross(tangent2 - curve_dir)
                        .dot(tangent3 - curve_dir);
                    det > 0.0
                }
            }
        }
    }
}

impl Shell {
    pub fn new(faces: Vec<Face>) -> Shell {
        assert!(faces.len() > 0, "Shell must have at least one face");
        Shell { faces }
    }

    pub fn transform(&self, transform: Transform) -> Shell {
        Shell {
            faces: self.faces.iter().map(|f| f.transform(transform)).collect(),
        }
    }

    pub fn normal(&self, point: Point) -> ShellNormal {
        let mut relevant_faces = Vec::<&Face>::new();
        for face in self.faces.iter() {
            match face_point_contains(face, point) {
                FacePointContains::Inside
                | FacePointContains::OnEdge(_)
                | FacePointContains::OnPoint(_) => {
                    relevant_faces.push(face);
                }
                FacePointContains::Outside => {}
            }
        }
        match relevant_faces.len() {
            0 => panic!("Point is not on Shell boundary"),
            1 => ShellNormal::OnFace(relevant_faces[0].normal(point)),
            2 => ShellNormal::OnEdge(
                relevant_faces[0].normal(point),
                relevant_faces[1].normal(point),
                *relevant_faces[0].boundary_tangent(point).expect_on_edge(),
            ),
            3 => {
                let btangent1 = relevant_faces[0].boundary_tangent(point);
                let btangent2 = relevant_faces[1].boundary_tangent(point);
                let btangent3 = relevant_faces[2].boundary_tangent(point);
                let (_t1in, t1out) = btangent1.expect_on_corner();
                let (t2in, t2out) = btangent2.expect_on_corner();
                let (_t3in, t3out) = btangent3.expect_on_corner();
                // Now we have to order it like right hand rule, such that the Shell is between the fingers.
                // First face is between thumb and index finger
                // Now figure out if the face between index and middle finger is the second or third face
                match t1out.normalize() == t2in.normalize() {
                    true => ShellNormal::OnPoint(*t1out, *t2out, *t3out), // Retourn all outgoing tangents
                    false => ShellNormal::OnPoint(*t1out, *t3out, *t2out),
                }
            }
            _ => panic!("Corners with more than 3 edges are not yet supported"),
        }
    }
}
