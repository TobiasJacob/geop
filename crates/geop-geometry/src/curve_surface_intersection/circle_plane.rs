use crate::{
    curves::circle::Circle,
    points::point::Point,
    surfaces::{plane::Plane, SurfaceLike},
};

pub enum CirclePlaneIntersection {
    None,
    Points(Vec<Point>),
    Circle(Circle),
}

pub fn circle_plane_intersection(circle: &Circle, plane: &Plane) -> CirclePlaneIntersection {
    // Check if circle and plane are coplanar
    if plane.on_surface(circle.basis) {
        // Check if normals are parallel
        if circle.normal.is_parallel(plane.normal(plane.basis)) {
            return CirclePlaneIntersection::Circle(circle.clone());
        }
    }
    todo!("Implement circle-plane intersection.");
}
