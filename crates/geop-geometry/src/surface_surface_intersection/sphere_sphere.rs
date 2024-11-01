use crate::geometry::{curves::circle::Circle, point::Point, surfaces::sphere::Sphere};

pub enum SphereSphereIntersection {
    Sphere(Sphere),
    Circle(Circle),
    Point(Point),
    None,
}

pub fn sphere_sphere_intersection(a: &Sphere, b: &Sphere) -> SphereSphereIntersection {
    let r_a: f64 = a.radius;
    let r_b: f64 = b.radius;
    let a: Point = a.basis;
    let b: Point = b.basis;

    let d = (a - b).norm();
    if d > r_a + r_b {
        return SphereSphereIntersection::None;
    } else if d < (r_a - r_b).abs() {
        return SphereSphereIntersection::None;
    } else if d <= 0.0 && (r_a - r_b).abs() <= 0.0 {
        return SphereSphereIntersection::Sphere(Sphere::new(a, r_a.min(r_b)));
    } else {
        let x = (r_a.powi(2) - r_b.powi(2) + d.powi(2)) / (2.0 * d);
        let y = (r_a.powi(2) - x.powi(2)).sqrt();
        let z = (b - a) / d;
        let p = a + z * x;
        let n = z.cross(Point::from_f64(0.0, 0.0, 1.0)).cross(z);
        if y <= 0.0 {
            return SphereSphereIntersection::Point(p);
        }
        return SphereSphereIntersection::Circle(Circle::new(p, n, y));
    }
}
