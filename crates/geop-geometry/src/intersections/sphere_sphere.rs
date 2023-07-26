use crate::geometry::{surfaces::sphere::Sphere, curves::circle3d::Circle3d, points::point3d::Point3d};

pub enum SphereSphereIntersection {
    Sphere(Sphere),
    Circle(Circle3d),
    Point(Point3d),
    None
}

pub fn intersect(a: &Sphere, b: &Sphere) -> SphereSphereIntersection {
    let r_a: f64 = a.radius;
    let r_b: f64 = b.radius;
    let a: Point3d = a.basis;
    let b: Point3d = b.basis;

    let d = (a - b).norm();
    if d > r_a + r_b {
        return SphereSphereIntersection::None;
    } else if d < (r_a - r_b).abs() {
        return SphereSphereIntersection::None;
    } else if d < crate::EQ_THRESHOLD && (r_a - r_b).abs() < crate::EQ_THRESHOLD {
        return SphereSphereIntersection::Sphere(Sphere::new(a, r_a.min(r_b)));
    } else {
        let x = (r_a.powi(2) - r_b.powi(2) + d.powi(2)) / (2.0 * d);
        let y = (r_a.powi(2) - x.powi(2)).sqrt();
        let z = (b - a) / d;
        let p = a + z * x;
        let n = z.cross(Point3d::new(0.0, 0.0, 1.0)).cross(z);
        if y < crate::EQ_THRESHOLD {
            return SphereSphereIntersection::Point(p);
        }
        return SphereSphereIntersection::Circle(Circle3d::new(p, n, y));
    }
}
