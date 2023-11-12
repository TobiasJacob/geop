use std::rc::Rc;

use geop_geometry::{points::point::Point, curves::line::Line};

use crate::topology::{volume::{Volume, VolumeContainsPoint}, edge::{Edge, edge_curve::EdgeCurve}, intersections::edge_edge::EdgeEdgeIntersection};

use super::face_point::{FaceContainsPoint, face_contains_point};


pub fn volume_contains_point(volume: &Volume, other: Point) -> VolumeContainsPoint {
    // first check if point is on any other face
    for face in volume.faces.iter() {
        match face_contains_point(face, other) {
            FaceContainsPoint::Inside => return VolumeContainsPoint::OnFace(face.clone()),
            FaceContainsPoint::OnEdge(edge) => return VolumeContainsPoint::OnEdge(edge),
            FaceContainsPoint::OnPoint(point) => return VolumeContainsPoint::OnPoint(point),
            FaceContainsPoint::Outside => {}
        }
    }

    // choose a random point on a face
    let q = volume.faces[0].inner_point();
    let curve = Edge::new(
        Rc::new(other.clone()), 
        Rc::new(q.clone()),
        Rc::new(EdgeCurve::Line(Line::new(other, q - other))));

    // Find the closest intersection point with any other face and use the normal to determine if the point is inside or outside
    for face in volume.faces.iter() {
        let intersections = todo!("Find intersections with face boundaries"); //face.intersect_edge(&curve);
    }
    let mut closest_distance = (other - q).norm();
    let curve_dir = q - other;
    let normal = volume.normal(q);
    let mut closest_intersect_from_inside = normal.is_from_inside(curve_dir);
    for face in volume.faces.iter() {
        let edge_intersections: Vec<EdgeEdgeIntersection> = todo!("Find intersections with face boundaries"); //face.intersect_edge(&curve);
        let mut intersections = Vec::<Point>::new();
        for intersection in edge_intersections {
            match intersection {
                EdgeEdgeIntersection::Point(point) => {
                    intersections.push(*point);
                },
                EdgeEdgeIntersection::Edge(edge) => {
                    intersections.push(*edge.start);
                    intersections.push(*edge.end);
                }
            }
        }
        for point in intersections {
            let distance = (other - point).norm();
            if distance < closest_distance {
                let curve_dir = curve.tangent(point);
                let normal = volume.normal(point);
                closest_distance = distance;
                closest_intersect_from_inside = normal.is_from_inside(curve_dir);
            }
        }
    }

    match closest_intersect_from_inside {
        true => VolumeContainsPoint::Inside,
        false => VolumeContainsPoint::Outside,
    }
}