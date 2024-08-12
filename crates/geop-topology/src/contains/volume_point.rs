use geop_geometry::{
    curve_curve_intersection::curve_curve::{curve_curve_intersection, CurveCurveIntersection},
    curve_surface_intersection::curve_surface::{
        curve_surface_intersection, CurveSurfaceIntersection,
    },
    curves::{curve::Curve, line::Line, CurveLike},
    points::point::Point,
};

use crate::topology::{edge::Edge, face::Face, volume::Volume};

use super::{
    edge_point::{edge_point_contains, EdgePointContains},
    face_point::{face_point_contains, FacePointContains},
};

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
    let geodesic = Edge::new(
        Some(other.clone()),
        Some(q.clone()),
        Curve::Line(Line::new(other, q - other)),
    );

    let mut intersection_points = Vec::<Point>::new();
    for face in volume.all_faces().iter() {
        let intersections = curve_surface_intersection(&geodesic.curve, &*face.surface);
        match intersections {
            CurveSurfaceIntersection::Curve(_) => {
                for edge in face.all_edges() {
                    match curve_curve_intersection(&geodesic.curve, &edge.curve) {
                        CurveCurveIntersection::Points(points) => {
                            for point in points {
                                if edge_point_contains(&geodesic, point)
                                    != EdgePointContains::Outside
                                {
                                    intersection_points.push(point)
                                }
                            }
                        }
                        CurveCurveIntersection::Curve(_) => {
                            if let Some(start) = edge.start {
                                if edge_point_contains(&geodesic, start)
                                    != EdgePointContains::Outside
                                {
                                    intersection_points.push(start)
                                }
                            }
                            if let Some(end) = edge.end {
                                if edge_point_contains(&geodesic, end) != EdgePointContains::Outside
                                {
                                    intersection_points.push(end)
                                }
                            }
                        }
                        CurveCurveIntersection::None => {}
                    }
                }
            }
            CurveSurfaceIntersection::Points(points) => {
                for point in points {
                    match edge_point_contains(&geodesic, point) {
                        EdgePointContains::Inside => todo!(),
                        EdgePointContains::Outside => todo!(),
                        EdgePointContains::OnPoint(_) => todo!(),
                    }
                }
            }
            CurveSurfaceIntersection::None => {}
        }
    }

    // Find the closest intersection point with any other face and use the normal to determine if the point is inside or outside
    let mut closest_distance = (other - q).norm();
    let curve_dir = q - other;
    let normal = volume.boundary_normal(q);
    let mut closest_intersect_from_inside = normal.is_from_inside(curve_dir);
    for point in intersection_points.iter() {
        let distance = (other - *point).norm();
        if distance < closest_distance {
            let curve_dir = geodesic.curve.tangent(*point);
            let normal = volume.boundary_normal(*point);
            closest_distance = distance;
            closest_intersect_from_inside = normal.is_from_inside(curve_dir);
        }
    }
    match closest_intersect_from_inside {
        true => VolumePointContains::Inside,
        false => VolumePointContains::Outside,
    }
}
