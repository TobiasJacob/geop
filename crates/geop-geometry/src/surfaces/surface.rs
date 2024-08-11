use crate::{curves::curve::Curve, points::point::Point, transforms::Transform, HORIZON_DIST};

use super::{
    cylinder::Cylinder,
    plane::Plane,
    sphere::{Sphere, SphereTransform},
};

pub type TangentPoint = Point;

pub trait SurfaceLike {
    // // Angle between a and b at x.
    // fn angle(&self, x: Point, a: Point, b: Point) -> f64;
}

#[derive(PartialEq, Clone, Debug)]
pub enum Surface {
    Plane(Plane),
    Sphere(Sphere),
    Cylinder(Cylinder),
}
impl Surface {
    // Transforms the surface by the given transform.
    pub fn transform(&self, transform: Transform) -> Surface {
        match self {
            Surface::Plane(plane) => Surface::Plane(plane.transform(transform)),
            Surface::Sphere(sphere) => Surface::Sphere(match sphere.transform(transform) {
                SphereTransform::Ellipsoid() => todo!("Ellipsoid not implemented"),
                SphereTransform::Sphere(sphere) => sphere,
            }),
            Surface::Cylinder(cylinder) => Surface::Cylinder(cylinder.transform(transform)),
        }
    }

    // Change normal direction of the surface.
    pub fn neg(&self) -> Surface {
        match self {
            Surface::Plane(plane) => Surface::Plane(plane.neg()),
            Surface::Sphere(sphere) => Surface::Sphere(sphere.neg()),
            Surface::Cylinder(cylinder) => Surface::Cylinder(cylinder.neg()),
        }
    }

    // Returns the normal of the surface at point p.
    pub fn normal(&self, p: Point) -> Point {
        match self {
            Surface::Plane(plane) => plane.normal(),
            Surface::Sphere(sphere) => sphere.normal(p),
            Surface::Cylinder(cylinder) => cylinder.normal(p),
        }
    }
    // Checks if the point p is on the surface.
    pub fn on_surface(&self, p: Point) -> bool {
        match self {
            Surface::Plane(plane) => plane.on_surface(p),
            Surface::Sphere(sphere) => sphere.on_surface(p),
            Surface::Cylinder(cylinder) => cylinder.on_surface(p),
        }
    }

    // Returns the Riemannian metric between u and v
    pub fn metric(&self, x: Point, u: TangentPoint, v: TangentPoint) -> f64 {
        match self {
            Surface::Plane(plane) => plane.metric(x, u, v),
            Surface::Sphere(sphere) => sphere.metric(x, u, v),
            Surface::Cylinder(cylinder) => cylinder.metric(x, u, v),
        }
    }
    // Returns the Riemannian distance between x and y.
    pub fn distance(&self, x: Point, y: Point) -> f64 {
        match self {
            Surface::Plane(plane) => plane.distance(x, y),
            Surface::Sphere(sphere) => sphere.distance(x, y),
            Surface::Cylinder(cylinder) => cylinder.distance(x, y),
        }
    }
    // Exponential of u at base x. u_z is ignored.
    pub fn exp(&self, x: Point, u: TangentPoint) -> Point {
        match self {
            Surface::Plane(plane) => plane.exp(x, u),
            Surface::Sphere(sphere) => sphere.exp(x, u),
            Surface::Cylinder(cylinder) => cylinder.exp(x, u),
        }
    }
    // Log of y at base x. Z coordinate is set to 0.
    pub fn log(&self, x: Point, y: Point) -> Option<TangentPoint> {
        match self {
            Surface::Plane(plane) => plane.log(x, y),
            Surface::Sphere(sphere) => sphere.log(x, y),
            Surface::Cylinder(cylinder) => cylinder.log(x, y),
        }
    }
    // Parallel transport of v from x to y.
    pub fn parallel_transport(
        &self,
        v: Option<TangentPoint>,
        x: Point,
        y: Point,
    ) -> Option<TangentPoint> {
        match self {
            Surface::Plane(plane) => plane.parallel_transport(v, x, y),
            Surface::Sphere(sphere) => sphere.parallel_transport(v, x, y),
            Surface::Cylinder(cylinder) => cylinder.parallel_transport(v, x, y),
        }
    }
    // Returns the geodesic between p and q.
    pub fn geodesic(&self, x: Point, y: Point) -> Curve {
        match self {
            Surface::Plane(plane) => plane.geodesic(x, y),
            Surface::Sphere(sphere) => sphere.geodesic(x, y),
            Surface::Cylinder(cylinder) => cylinder.geodesic(x, y),
        }
    }
    // Returns a point grid on the surface, which can be used for visualization.
    pub fn point_grid(&self, density: f64) -> Vec<Point> {
        match self {
            Surface::Plane(plane) => plane.point_grid(density, HORIZON_DIST),
            Surface::Sphere(sphere) => sphere.point_grid(density),
            Surface::Cylinder(cylinder) => cylinder.point_grid(density, HORIZON_DIST),
        }
    }
    // Finds the closest point on the surface to the given point.
    pub fn project(&self, point: Point) -> Point {
        match self {
            Surface::Plane(plane) => plane.project(point),
            Surface::Sphere(sphere) => sphere.project(point),
            Surface::Cylinder(cylinder) => cylinder.project(point),
        }
    }
}
