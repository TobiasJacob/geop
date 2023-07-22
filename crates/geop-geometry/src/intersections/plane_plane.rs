use crate::geometry::{surfaces::{sphere::Sphere, plane::Plane}, curves::{circle3d::Circle3d, line3d::Line3d}, points::point3d::Point3d};

pub enum PlanePlaneIntersection {
    Line3d(Line3d),
    None
}

pub fn intersect(a: Plane, b: Plane) -> PlanePlaneIntersection {
    todo!("Intersection")
}
