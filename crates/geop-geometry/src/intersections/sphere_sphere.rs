use crate::geometry::{surfaces::sphere::Sphere, curves::circle3d::Circle3d, points::point3d::Point3d};

pub enum SphereSphereIntersection {
    Circle(Circle3d),
    Point(Point3d),
    None
}

pub fn intersect(a: Sphere, b: Sphere) -> SphereSphereIntersection {
    todo!("Intersection")
}
