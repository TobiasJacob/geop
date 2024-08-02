use geop_geometry::{
    curves::{curve::Curve, line::Line},
    points::point::Point,
};

use crate::{
    intersections::face_edge::{face_edge_intersection, FaceEdgeIntersection},
    topology::{edge::Edge, face::Face, volume::Volume},
};

use super::face_point::{face_point_contains, FacePointContains};

pub enum VolumePointContains {
    Inside,
    OnFace(Face),
    OnEdge(Edge),
    OnPoint(Point),
    Outside,
}

pub fn volume_point_contains(volume: &Volume, other: Point) -> VolumePointContains {
    // first check if point is on any other face
    for face in volume.all_faces().iter() {
        match face_point_contains(face, other) {
            FacePointContains::Inside => return VolumePointContains::OnFace(face.clone()),
            FacePointContains::OnEdge(edge) => return VolumePointContains::OnEdge(edge),
            FacePointContains::OnPoint(point) => return VolumePointContains::OnPoint(point),
            FacePointContains::Outside => {}
            FacePointContains::NotOnSurface => {}
        }
    }

    // choose a random point on a face
    let q = volume.all_faces()[0].inner_point();
    let edge = Edge::new(
        Some(other.clone()),
        Some(q.clone()),
        Curve::Line(Line::new(other, q - other)),
    );

    // Find the closest intersection point with any other face and use the normal to determine if the point is inside or outside
    let mut closest_distance = (other - q).norm();
    let curve_dir = q - other;
    let normal = volume.boundary_normal(q);
    let mut closest_intersect_from_inside = normal.is_from_inside(curve_dir);
    for face in volume.all_faces().iter() {
        let intersections = face_edge_intersection(face, &edge);
        match intersections {
            FaceEdgeIntersection::Edges(edges) => {
                for edge in edges {
                    let distance = (other - edge.get_midpoint()).norm();
                    if distance < closest_distance {
                        let curve_dir = edge.curve.tangent(edge.get_midpoint());
                        let normal = volume.boundary_normal(edge.get_midpoint());
                        closest_distance = distance;
                        closest_intersect_from_inside = normal.is_from_inside(curve_dir);
                    }
                }
            }
            FaceEdgeIntersection::Points(points) => {
                for point in points {
                    let distance = (other - point).norm();
                    if distance < closest_distance {
                        let curve_dir = edge.curve.tangent(point);
                        let normal = volume.boundary_normal(point);
                        closest_distance = distance;
                        closest_intersect_from_inside = normal.is_from_inside(curve_dir);
                    }
                }
            }
            FaceEdgeIntersection::None => {}
        }
    }

    match closest_intersect_from_inside {
        true => VolumePointContains::Inside,
        false => VolumePointContains::Outside,
    }
}
