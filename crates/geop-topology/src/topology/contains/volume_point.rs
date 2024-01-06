use geop_geometry::{
    curves::{curve::Curve, line::Line},
    points::point::Point,
};

use crate::topology::{
    edge::Edge,
    face::Face,
    intersections::face_edge::{face_edge_intersection, FaceEdgeIntersection},
    volume::Volume,
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
    for face in volume.faces.iter() {
        match face_point_contains(face, other) {
            FacePointContains::Inside => return VolumePointContains::OnFace(face.clone()),
            FacePointContains::OnEdge(edge) => return VolumePointContains::OnEdge(edge),
            FacePointContains::OnPoint(point) => return VolumePointContains::OnPoint(point),
            FacePointContains::Outside => {}
        }
    }

    // choose a random point on a face
    let q = volume.faces[0].inner_point();
    let edge = Edge::new(
        other.clone(),
        q.clone(),
        Curve::Line(Line::new(other, q - other)),
    );

    // Find all intersections with boundary faces
    let mut intersection_points = Vec::<Point>::new();
    for face in volume.faces.iter() {
        let intersections = face_edge_intersection(face, &edge);
        match intersections {
            FaceEdgeIntersection::Points(points) => {
                intersection_points.extend(points);
            }
            FaceEdgeIntersection::Edges(edges) => {
                for edge in edges {
                    intersection_points.push(edge.start.clone());
                    intersection_points.push(edge.end.clone());
                }
            }
            FaceEdgeIntersection::None => {}
        }
    }

    // Find the closest intersection point with any other face and use the normal to determine if the point is inside or outside
    let mut closest_distance = (other - q).norm();
    let curve_dir = q - other;
    let normal = volume.normal(q);
    let mut closest_intersect_from_inside = normal.is_from_inside(curve_dir);

    for point in intersection_points {
        let distance = (point - other).norm();
        if distance < closest_distance {
            closest_distance = distance;
            let curve_dir = point - other;
            let normal = volume.normal(point);
            closest_intersect_from_inside = normal.is_from_inside(curve_dir);
        }
    }

    match closest_intersect_from_inside {
        true => VolumePointContains::Inside,
        false => VolumePointContains::Outside,
    }
}
