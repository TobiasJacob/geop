use geop_geometry::{
    curve_surface_intersection::curve_surface::{
        curve_surface_intersection, CurveSurfaceIntersection,
    },
    points::point::Point,
};

use crate::{
    contains::face_point::{face_point_contains, FacePointContains},
    topology::{edge::Edge, face::Face},
};

pub enum FaceEdgeIntersection {
    None,
    Points(Vec<Point>),
    Edges(Vec<Edge>),
}

pub fn face_edge_intersection(face: &Face, edge: &Edge) -> FaceEdgeIntersection {
    match curve_surface_intersection(&edge.curve, &face.surface) {
        CurveSurfaceIntersection::Points(mut points) => FaceEdgeIntersection::Points(
            points
                .drain(..)
                .filter(|p| face_point_contains(face, *p) == FacePointContains::Inside)
                .collect(),
        ),
        CurveSurfaceIntersection::Curve(_e) => {
            todo!("Split edges by intersections")
        }
        CurveSurfaceIntersection::None => FaceEdgeIntersection::None,
    }
}
