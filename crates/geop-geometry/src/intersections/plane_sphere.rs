use crate::geometry::{surfaces::{sphere::Sphere, plane::Plane}, curves::circle3d::Circle3d, points::point3d::Point3d};

pub enum PlaneSphereIntersection {
    Circle(Circle3d),
    Point(Point3d),
    None
}

pub fn intersect(a: Sphere, b: Plane) -> PlaneSphereIntersection {
    todo!("Intersection")
}
