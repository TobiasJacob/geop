use crate::{
    curves::{circle::Circle, curve::Curve},
    points::point::Point,
    transforms::Transform,
    EQ_THRESHOLD,
};

use super::surface::TangentPoint;

#[derive(Clone, Debug)]
pub struct Sphere {
    pub basis: Point,
    pub radius: f64,
    pub normal_outwards: bool,
}

pub enum SphereTransform {
    Sphere(Sphere),
    Ellipsoid(), // TODO: Implement this
}

impl Sphere {
    pub fn new(basis: Point, radius: f64, normal_outwards: bool) -> Sphere {
        Sphere {
            basis,
            radius,
            normal_outwards,
        }
    }

    pub fn curve_from_to(&self, p: Point, q: Point) -> Circle {
        let normal = (p - self.basis).cross(q - self.basis).normalize();
        return Circle::new(self.basis, normal, (q - self.basis).norm());
    }

    pub fn transform(&self, transform: Transform) -> SphereTransform {
        let basis = transform * self.basis;
        let radius = self.radius * transform.uniform_scale_factor();
        SphereTransform::Sphere(Sphere::new(basis, radius, self.normal_outwards))
    }

    pub fn normal(&self, p: Point) -> Point {
        assert!(self.on_surface(p));
        if self.normal_outwards {
            (self.basis - p).normalize()
        } else {
            (p - self.basis).normalize()
        }
    }

    pub fn neg(&self) -> Sphere {
        Sphere::new(self.basis, self.radius, !self.normal_outwards)
    }

    pub fn on_surface(&self, p: Point) -> bool {
        let diff = p - self.basis;
        let dist = diff.norm_sq();
        (dist - self.radius * self.radius).abs() < EQ_THRESHOLD
    }

    pub fn metric(&self, _x: Point, u: TangentPoint, v: TangentPoint) -> f64 {
        assert!(u.0.z.abs() < EQ_THRESHOLD);
        assert!(v.0.z.abs() < EQ_THRESHOLD);
        u.0.dot(v.0)
    }

    pub fn distance(&self, x: Point, y: Point) -> f64 {
        assert!(self.on_surface(x));
        assert!(self.on_surface(y));
        let angle = (x - self.basis).angle(y - self.basis);
        self.radius * angle
    }

    pub fn exp(&self, x: Point, u: TangentPoint) -> Point {
        assert!(self.on_surface(x));
        assert!(u.0.z.abs() < EQ_THRESHOLD);
        let u_norm = u.0.norm();
        let u_normalized = u.0 / u_norm;
        x * u_norm.cos() * self.radius
            + u_normalized.cross(x) * u_norm.sin() * self.radius
            + self.basis
    }

    pub fn log(&self, x: Point, y: Point) -> TangentPoint {
        assert!(self.on_surface(x));
        assert!(self.on_surface(y));
        let x = (x - self.basis) / self.radius;
        let y = (y - self.basis) / self.radius;
        let dir = y - x.dot(y) * x;
        let dir_norm = dir.norm();
        TangentPoint(self.distance(x, y) * dir / dir_norm)
    }

    pub fn parallel_transport(&self, v: TangentPoint, x: Point, y: Point) -> Point {
        assert!(v.0.z.abs() < EQ_THRESHOLD);
        assert!(self.on_surface(x));
        assert!(self.on_surface(y));
        let x = (x - self.basis) / self.radius;
        let y = (y - self.basis) / self.radius;
        let u = self.log(x, y);
        let u_norm = u.0.norm();
        let u_normalized = u.0 / u_norm;
        -x * u_norm.sin() * u_normalized.dot(v.0)
            + u_normalized * u_norm.cos() * u_normalized.dot(v.0)
            + v.0
            + u_normalized * u_normalized.dot(v.0)
    }

    pub fn geodesic(&self, p: Point, q: Point) -> Curve {
        let normal = (p - self.basis).cross(q - self.basis).normalize();
        let circle = Circle::new(self.basis, normal, (q - self.basis).norm());
        Curve::Circle(circle)
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Sphere) -> bool {
        self.basis == other.basis && (self.radius - other.radius).abs() < EQ_THRESHOLD
    }
}
