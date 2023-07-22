use crate::geometry::{surfaces::{sphere::Sphere, plane::Plane}, curves::{circle3d::Circle3d, line3d::Line3d}, points::point3d::Point3d};

pub enum LineSphereIntersection {
    Line3d(Line3d),
    Point3d(Point3d),
    None
}

pub fn intersect(a: Line3d, b: Sphere) -> LineSphereIntersection {
    todo!("Intersection")
}
