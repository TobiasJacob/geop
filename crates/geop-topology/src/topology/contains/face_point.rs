use std::rc::Rc;

use geop_geometry::points::point::Point;

use crate::topology::{
    edge::Edge, face::Face, intersections::contour_edge::countour_edge_intersection_points,
};

use super::edge_point::{edge_contains_point, EdgeContains};

#[derive(Clone, Debug, PartialEq)]
pub enum FaceContainsPoint {
    Inside,
    OnEdge(Rc<Edge>),
    OnPoint(Rc<Point>),
    Outside,
}

pub fn face_contains_point(face: &Face, point: Point) -> FaceContainsPoint {
    // println!("face_contains_point: {:?}", point);
    // println!("face: {:}", face);
    // If the point is on the border, it is part of the set
    for edge in face.all_edges() {
        match edge_contains_point(&edge, point) {
            EdgeContains::Inside => return FaceContainsPoint::OnEdge(edge.clone()),
            EdgeContains::OnPoint(point) => return FaceContainsPoint::OnPoint(point),
            EdgeContains::Outside => continue,
        }
    }
    // Draw a line from the point to a random point on the border.
    let q: Point = *face.boundaries[0].edges[0].start;
    let curve = face.edge_from_to(Rc::new(point), Rc::new(q));

    // Find the closest intersection point and check by using the face normal and the curve tangent if the intersection is from inside or outside.
    let mut closest_distance = face.surface.distance(point, q);
    let curve_dir = curve.tangent(q);
    let normal = face.surface.normal(q);
    let contour_dir = face.boundaries[0].tangent(q);
    let mut closest_intersect_from_inside = contour_dir.is_inside(normal, curve_dir);

    for int in countour_edge_intersection_points(face, &*curve) {
        // println!("int: {:?}", int);
        let distance = face.surface.distance(point, *int);
        if distance < closest_distance {
            let curve_dir = curve.tangent(*int);
            let normal = face.surface.normal(*int);
            let contour_dir = face.boundary_tangent(*int);
            closest_distance = distance;
            closest_intersect_from_inside = contour_dir.is_inside(normal, curve_dir);
        }
    }

    match closest_intersect_from_inside {
        true => FaceContainsPoint::Inside,
        false => FaceContainsPoint::Outside,
    }
}
