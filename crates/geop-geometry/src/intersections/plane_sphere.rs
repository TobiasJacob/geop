use crate::{geometry::{surfaces::{sphere::Sphere, plane::Plane}, curves::circle::Circle, points::point::Point}, EQ_THRESHOLD};

pub enum PlaneSphereIntersection {
    Circle(Circle),
    Point(Point),
    None
}

pub fn intersect_intersection(a: &Sphere, b: &Plane) -> PlaneSphereIntersection {
    let r: f64 = a.radius;
    let b: Point = b.basis;
    let a: Point = a.basis;

    let discriminant = r.powi(2) - (a - b).norm().powi(2);

    if discriminant > EQ_THRESHOLD {
        let center = a + (b - a) * (r.powi(2) / (a - b).norm().powi(2));
        let normal = (b - a) / (b - a).norm();
        PlaneSphereIntersection::Circle(Circle::new(center, normal, discriminant.sqrt()))
    } else if discriminant <= EQ_THRESHOLD && discriminant >= -EQ_THRESHOLD {
        PlaneSphereIntersection::Point(a)
    } else {
        PlaneSphereIntersection::None
    }
}
