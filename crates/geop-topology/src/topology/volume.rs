use std::rc::Rc;

use geop_geometry::{transforms::Transform, points::point::Point, curves::line::Line};

use crate::topology::{face::FaceContainsPoint, edge::{EdgeCurve, Direction}};

use super::{contour::Contour, face::Face, edge::{Edge, EdgeIntersection}};

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

    pub fn contains_point(&self, other: Point) -> VolumeContainsPoint {
        // first check if point is on any other face
        for face in self.faces.iter() {
            match face.contains_point(other) {
                FaceContainsPoint::Inside => return VolumeContainsPoint::OnFace(face.clone()),
                FaceContainsPoint::OnEdge(edge) => return VolumeContainsPoint::OnEdge(edge),
                FaceContainsPoint::OnPoint(point) => return VolumeContainsPoint::OnPoint(point),
                FaceContainsPoint::Outside => {}
            }
        }

        // choose a random point on a face
        let q = self.faces[0].inner_point();
        let curve = Edge::new(
            Rc::new(other.clone()), 
            Rc::new(q.clone()),
            Rc::new(EdgeCurve::Line(Line::new(other, q - other))), 
            Direction::Increasing);

        // Find the closest intersection point with any other face and use the normal to determine if the point is inside or outside
        for face in self.faces.iter() {
            let intersections = face.intersect_edge(&curve);
        }
        let mut closest_distance = (other - q).norm();
        let curve_dir = q - other;
        let normal = self.faces[0].normal(q);
        let mut closest_intersect_from_inside = normal.dot(curve_dir) > 0.0;
        for face in self.faces.iter() {
            let edge_intersections = face.intersect_edge(&curve);
            let mut intersections = Vec::<Point>::new();
            for intersection in edge_intersections {
                match intersection {
                    EdgeIntersection::Point(point) => {
                        intersections.push(*point);
                    },
                    EdgeIntersection::Edge(edge) => {
                        intersections.push(*edge.start);
                        intersections.push(*edge.end);
                    }
                }
            }
            for point in intersections {
                let distance = (other - point).norm();
                if distance < closest_distance {
                    let curve_dir = curve.tangent(point);
                    let normal = face.normal(point);
                    closest_distance = distance;
                    closest_intersect_from_inside = normal.dot(curve_dir) > 0.0;
                }
            }
        }

        match closest_intersect_from_inside {
            true => VolumeContainsPoint::Inside,
            false => VolumeContainsPoint::Outside,
        }
    }
}
