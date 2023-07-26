use crate::{geometry::{surfaces::{sphere::Sphere, plane::Plane}, curves::circle3d::Circle3d, points::point3d::Point3d}, EQ_THRESHOLD};

pub enum PlaneSphereIntersection {
    Circle(Circle3d),
    Point(Point3d),
    None
}

pub fn intersect(a: &Sphere, b: &Plane) -> PlaneSphereIntersection {
    let r: f64 = a.radius;
    let b: Point3d = b.basis;
    let a: Point3d = a.basis;

    let discriminant = r.powi(2) - (a - b).norm().powi(2);

    if discriminant > EQ_THRESHOLD {
        let center = a + (b - a) * (r.powi(2) / (a - b).norm().powi(2));
        let normal = (b - a) / (b - a).norm();
        PlaneSphereIntersection::Circle(Circle3d::new(center, normal, discriminant.sqrt()))
    } else if discriminant <= EQ_THRESHOLD && discriminant >= -EQ_THRESHOLD {
        PlaneSphereIntersection::Point(a)
    } else {
        PlaneSphereIntersection::None
    }
}
