use crate::{points::point::Point, EQ_THRESHOLD, curves::circle::Circle};

use super::surface::{Surface, SurfaceCurve, TangentPoint};

#[derive(Clone, Debug)]
pub struct Sphere {
    pub basis: Point,
    pub radius: f64,
}

impl Sphere {
    pub fn new(basis: Point, radius: f64) -> Sphere {
        Sphere {
            basis,
            radius,
        }
    }

    pub fn curve_from_to(&self, p: Point, q: Point) -> Circle {
        let normal = (p - self.basis).cross(q - self.basis).normalize();
        return Circle::new(self.basis, normal, (q - self.basis).normalize());
    }
}

impl Surface for Sphere {
    fn on_surface(&self, p: Point) -> bool {
        let diff = p - self.basis;
        let dist = diff.norm_sq();
        (dist - self.radius * self.radius).abs() < EQ_THRESHOLD
    }

    // fn point_at(&self, u: f64, v: f64) -> Point {
    //     let x = self.basis.x + self.radius * u.cos() * v.sin();
    //     let y = self.basis.y + self.radius * u.sin() * v.sin();
    //     let z = self.basis.z + self.radius * v.cos();
    //     Point::new(x, y, z)
    // }

    // fn project(&self, p: &Point) -> (f64, f64) {
    //     let v = *p - self.basis;
    //     let u = v.dot(Point::new(1.0, 0.0, 0.0));
    //     let v = v.dot(Point::new(0.0, 1.0, 0.0));
    //     (u, v)
    // }

    // fn derivative_u(&self, u: f64, v: f64) -> Point {
    //     let x = -self.radius * u.sin() * v.sin();
    //     let y = self.radius * u.cos() * v.sin();
    //     let z = 0.0;
    //     Point::new(x, y, z)
    // }

    // fn derivative_v(&self, u: f64, v: f64) -> Point {
    //     let x = self.radius * u.cos() * v.cos();
    //     let y = self.radius * u.sin() * v.cos();
    //     let z = -self.radius * v.sin();
    //     Point::new(x, y, z)
    // }

    fn normal(&self, p: Point) -> Point {
        (p - self.basis).normalize()
    }

    // fn normalize(&mut self) {
    //     // Use this to make redundant representations of surfaces unique
    //     self.radius = self.radius.abs();
    // }

    // fn is_normalized(&self) -> bool {
    //     self.radius >= 0.0
    // }
    fn metric(&self, x:Point, u: TangentPoint, v: TangentPoint) -> f64 {
        assert!(u.0.z.abs() < EQ_THRESHOLD);
        assert!(v.0.z.abs() < EQ_THRESHOLD);
        u.0.dot(v.0)
    }

    fn distance(&self, x: Point, y: Point) -> f64 {
        assert!(self.on_surface(x));
        assert!(self.on_surface(y));
        let angle = (x - self.basis).angle(y - self.basis);
        self.radius * angle
    }

    fn exp(&self, x: Point, u: TangentPoint) -> Point {
        assert!(self.on_surface(x));
        assert!(u.0.z.abs() < EQ_THRESHOLD);
        let u_norm = u.0.norm();
        let u_normalized = u.0 / u_norm;
        x * u_norm.cos() * self.radius + u_normalized.cross(x) * u_norm.sin() * self.radius + self.basis
    }

    fn log(&self, x: Point, y: Point) -> TangentPoint {
        assert!(self.on_surface(x));
        assert!(self.on_surface(y));
        let x = (x - self.basis) / self.radius;
        let y = (y - self.basis) / self.radius;
        let dir = y - x.dot(y) * x;
        let dir_norm = dir.norm();
        TangentPoint(self.distance(x, y) * dir / dir_norm)
    }

    fn parallel_transport(&self, v: TangentPoint, x: Point, y: Point) -> Point {
        assert!(v.0.z.abs() < EQ_THRESHOLD);
        assert!(self.on_surface(x));
        assert!(self.on_surface(y));
        let x = (x - self.basis) / self.radius;
        let y = (y - self.basis) / self.radius;
        let u = self.log(x, y);
        let u_norm = u.0.norm();
        let u_normalized = u.0 / u_norm;
        -x * u_norm.sin() * u_normalized.dot(v.0) + u_normalized * u_norm.cos() * u_normalized.dot(v.0) + v.0 + u_normalized * u_normalized.dot(v.0)
    }

    fn geodesic(&self, p: Point, q: Point) -> SurfaceCurve {
        let normal = (p - self.basis).cross(q - self.basis).normalize();
        let circle = Circle::new(self.basis, normal, (q - self.basis).normalize());
        SurfaceCurve::Circle(circle)
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Sphere) -> bool {
        self.basis == other.basis && (self.radius - other.radius).abs() < EQ_THRESHOLD
    }
}