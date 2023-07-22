use crate::geometry::{surfaces::{sphere::Sphere, plane::Plane}, curves::{circle3d::Circle3d, line3d::Line3d}, points::point3d::Point3d};

pub enum LinePlaneIntersection {
    Line3d(Line3d),
    Point3d(Point3d),
    None
}

pub fn intersect(sp_a: Line3d, pl_b: Plane) -> LinePlaneIntersection {
    todo!("Intersection")
}
