use geop_geometry::points::point::Point;

use crate::{
    intersections::edge_edge::{edge_edge_intersection, EdgeEdgeIntersection},
    topology::{edge::Edge, face::Face},
};

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
    let curve = face.edge_from_to(point, q);

    // Find the closest intersection point and check by using the face normal and the curve tangent if the intersection is from inside or outside.
    let mut closest_distance = face.surface.distance(point, q);
    let curve_dir = curve.tangent(q);
    let normal = face.surface.normal(q);
    let contour_dir = face.boundary_tangent(q);
    let mut closest_intersect_from_inside = contour_dir.is_inside(normal, curve_dir);

    let mut intersection_points = Vec::<Point>::new();
    for edge in face.all_edges() {
        match edge_edge_intersection(&edge, &curve) {
            EdgeEdgeIntersection::Points(points) => {
                intersection_points.extend(points);
            }
            EdgeEdgeIntersection::Edges(edges) => {
                for edge in edges {
                    if let Some(p) = edge.start {
                        intersection_points.push(p);
                    }
                    if let Some(p) = edge.end {
                        intersection_points.push(p);
                    }
                }
            }
            EdgeEdgeIntersection::None => {}
        }
    }

    for int in intersection_points {
        let distance = face.surface.distance(point, int);
        if distance < closest_distance {
            let curve_dir = curve.tangent(int);
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
