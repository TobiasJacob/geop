use geop_geometry::points::point::Point;

use crate::topology::{
    edge::Edge, face::Face, intersections::edge_edge::{edge_edge_intersection, EdgeEdgeIntersection},
};

use super::edge_point::{edge_point_contains, EdgePointContains};

#[derive(Clone, Debug, PartialEq)]
pub enum FacePointContains {
    Inside,
    OnEdge(Edge),
    OnPoint(Point),
    Outside,
}

impl FacePointContains {
    pub fn is_on_edge(&self) -> bool {
        match self {
            FacePointContains::OnEdge(edge) => true,
            _ => false,
        }
    }
}

pub fn face_point_contains(face: &Face, point: Point) -> FacePointContains {
    // println!("face_contains_point: {:?}", point);
    // println!("face: {:}", face);
    // If the point is on the border, it is part of the set
    for edge in face.all_edges() {
        match edge_point_contains(&edge, point) {
            EdgePointContains::Inside => return FacePointContains::OnEdge(edge.clone()),
            EdgePointContains::OnPoint(point) => return FacePointContains::OnPoint(point),
            EdgePointContains::Outside => continue,
        }
    }
    // Draw a line from the point to a random point on the border.
    let q = face.boundaries[0].edges[0].start.clone();
    let curve = face.edge_from_to(point, q);

    // Find the closest intersection point and check by using the face normal and the curve tangent if the intersection is from inside or outside.
    let mut closest_distance = face.surface.distance(point, q);
    let curve_dir = curve.tangent(q);
    let normal = face.surface.normal(q);
    let contour_dir = face.boundaries[0].tangent(q);
    let mut closest_intersect_from_inside = contour_dir.is_inside(normal, curve_dir);

    let mut intersection_points = Vec::<Point>::new();
    for edge in face.all_edges() {
        match edge_edge_intersection(&edge, &curve) {
            EdgeEdgeIntersection::Points(points) => {
                intersection_points.extend(points);
            }
            EdgeEdgeIntersection::Edges(edges) => {
                for edge in edges {
                    intersection_points.push(edge.start.clone());
                    intersection_points.push(edge.end.clone());
                }
            }
            EdgeEdgeIntersection::None => {}
        }
    }

    for int in intersection_points {
        // println!("int: {:?}", int);
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
