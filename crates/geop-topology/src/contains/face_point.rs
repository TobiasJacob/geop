use geop_geometry::{
    curve_curve_intersection::curve_curve::{curve_curve_intersection, CurveCurveIntersection},
    points::point::Point,
};

use crate::topology::{edge::Edge, face::Face};

use super::edge_point::{edge_point_contains, EdgePointContains};

#[derive(Clone, Debug, PartialEq)]
pub enum FacePointContains {
    Inside,
    OnEdge(Edge),
    OnPoint(Point),
    Outside,
    NotOnSurface,
}

pub fn face_point_contains(face: &Face, point: Point) -> FacePointContains {
    if !face.surface.on_surface(point) {
        return FacePointContains::NotOnSurface;
    }

    // If the point is on the border, it is part of the set
    for edge in face.all_edges() {
        match edge_point_contains(&edge, point) {
            EdgePointContains::Inside => return FacePointContains::OnEdge(edge.clone()),
            EdgePointContains::OnPoint(point) => return FacePointContains::OnPoint(point),
            EdgePointContains::Outside => continue,
        }
    }
    // Draw a line from the point to a random point on the border.
    let q = match face.get_boundary_point() {
        Some(q) => q,
        None => {
            return FacePointContains::Inside;
        }
    };
    let geodesic = face.edge_from_to(point, q);

    // Find the closest intersection point and check by using the face normal and the curve tangent if the intersection is from inside or outside.
    let mut closest_distance = face.surface.distance(point, q);
    let curve_dir = geodesic.tangent(q);
    let normal = face.surface.normal(q);
    let contour_dir = face.boundary_tangent(q);
    let mut closest_intersect_from_inside = contour_dir.is_inside(normal, curve_dir);

    let mut intersection_points = Vec::<Point>::new();
    for edge in face.all_edges() {
        match curve_curve_intersection(&edge.curve, &geodesic.curve) {
            CurveCurveIntersection::Points(points) => {
                for p in points {
                    if edge_point_contains(&geodesic, p) != EdgePointContains::Outside {
                        if edge_point_contains(&edge, p) != EdgePointContains::Outside {
                            intersection_points.push(p)
                        }
                    }
                }
            }
            CurveCurveIntersection::Curve(_curve) => {
                if let Some(start) = edge.start {
                    match edge_point_contains(&geodesic, start) {
                        EdgePointContains::Inside => intersection_points.push(start),
                        EdgePointContains::Outside => {}
                        EdgePointContains::OnPoint(_) => intersection_points.push(start),
                    }
                }
                if let Some(end) = edge.end {
                    match edge_point_contains(&geodesic, end) {
                        EdgePointContains::Inside => intersection_points.push(end),
                        EdgePointContains::Outside => {}
                        EdgePointContains::OnPoint(_) => intersection_points.push(end),
                    }
                }
            }
            CurveCurveIntersection::None => {}
        }
    }

    for int in intersection_points {
        let distance = face.surface.distance(point, int);
        if distance < closest_distance {
            let curve_dir = geodesic.tangent(int);
            let normal = face.surface.normal(int);
            let contour_dir = face.boundary_tangent(int);
            closest_distance = distance;
            closest_intersect_from_inside = contour_dir.is_inside(normal, curve_dir);
        }
    }

    match closest_intersect_from_inside {
        true => FacePointContains::Inside,
        false => FacePointContains::Outside,
    }
}
