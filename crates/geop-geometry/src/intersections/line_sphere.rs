use crate::{geometry::{surfaces::sphere::Sphere, curves::line::Line, points::point::Point}, EQ_THRESHOLD};

pub enum LineSphereIntersection {
    TwoPoints3d(Point, Point),
    Point3d(Point),
    None
}

pub fn intersect(line: &Line, sphere: &Sphere) -> LineSphereIntersection {
    let r: f64 = sphere.radius;
    let b: Point = sphere.basis;
    let a: Point = line.basis;
    let v: Point = line.direction;

    let discriminant = 4.0 * (v.dot(a - b)).powi(2) - 4.0 * (v.norm().powi(2)) * ((a - b).norm().powi(2) - r.powi(2));

    if discriminant > EQ_THRESHOLD {
        let t1 = (-2.0 * v.dot(a - b) + discriminant.sqrt()) / (2.0 * v.norm().powi(2));
        let t2 = (-2.0 * v.dot(a - b) - discriminant.sqrt()) / (2.0 * v.norm().powi(2));
        LineSphereIntersection::TwoPoints3d(a + v * t1, a + v * t2)
    } else if discriminant <= EQ_THRESHOLD && discriminant >= -EQ_THRESHOLD {
        let t = (-2.0 * v.dot(a - b)) / (2.0 * v.norm().powi(2));
        LineSphereIntersection::Point3d(a + v * t)
    } else {
        LineSphereIntersection::None
    }
}
