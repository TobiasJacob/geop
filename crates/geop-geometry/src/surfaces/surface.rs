use crate::{curves::curve::Curve, points::point::Point, transforms::Transform};

use super::{cylinder::Cylinder, plane::Plane, sphere::Sphere, SurfaceLike};

pub type TangentPoint = Point;

#[derive(PartialEq, Clone, Debug)]
pub enum Surface {
    Plane(Plane),
    Sphere(Sphere),
    Cylinder(Cylinder),
}

impl SurfaceLike for Surface {
    // Transforms the surface by the given transform.
    fn transform(&self, transform: Transform) -> Surface {
        match self {
            Surface::Plane(plane) => plane.transform(transform),
            Surface::Sphere(sphere) => sphere.transform(transform),
            Surface::Cylinder(cylinder) => cylinder.transform(transform),
        }
    }

    // Change normal direction of the surface.
    fn neg(&self) -> Surface {
        match self {
            Surface::Plane(plane) => plane.neg(),
            Surface::Sphere(sphere) => sphere.neg(),
            Surface::Cylinder(cylinder) => cylinder.neg(),
        }
    }

    // Returns the normal of the surface at point p.
    fn normal(&self, p: Point) -> Point {
        match self {
            Surface::Plane(plane) => plane.normal(p),
            Surface::Sphere(sphere) => sphere.normal(p),
            Surface::Cylinder(cylinder) => cylinder.normal(p),
        }
    }
    // Checks if the point p is on the surface.
    fn on_surface(&self, p: Point) -> bool {
        match self {
            Surface::Plane(plane) => plane.on_surface(p),
            Surface::Sphere(sphere) => sphere.on_surface(p),
            Surface::Cylinder(cylinder) => cylinder.on_surface(p),
        }
    }

    // Returns the Riemannian metric between u and v
    fn metric(&self, x: Point, u: TangentPoint, v: TangentPoint) -> f64 {
        match self {
            Surface::Plane(plane) => plane.metric(x, u, v),
            Surface::Sphere(sphere) => sphere.metric(x, u, v),
            Surface::Cylinder(cylinder) => cylinder.metric(x, u, v),
        }
    }
    // Returns the Riemannian distance between x and y.
    fn distance(&self, x: Point, y: Point) -> f64 {
        match self {
            Surface::Plane(plane) => plane.distance(x, y),
            Surface::Sphere(sphere) => sphere.distance(x, y),
            Surface::Cylinder(cylinder) => cylinder.distance(x, y),
        }
    }
    // Exponential of u at base x. u_z is ignored.
    fn exp(&self, x: Point, u: TangentPoint) -> Point {
        match self {
            Surface::Plane(plane) => plane.exp(x, u),
            Surface::Sphere(sphere) => sphere.exp(x, u),
            Surface::Cylinder(cylinder) => cylinder.exp(x, u),
        }
    }
    // Log of y at base x. Z coordinate is set to 0.
    fn log(&self, x: Point, y: Point) -> Option<TangentPoint> {
        match self {
            Surface::Plane(plane) => plane.log(x, y),
            Surface::Sphere(sphere) => sphere.log(x, y),
            Surface::Cylinder(cylinder) => cylinder.log(x, y),
        }
    }
    // Parallel transport of v from x to y.
    fn parallel_transport(
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
    fn geodesic(&self, x: Point, y: Point) -> Curve {
        match self {
            Surface::Plane(plane) => plane.geodesic(x, y),
            Surface::Sphere(sphere) => sphere.geodesic(x, y),
            Surface::Cylinder(cylinder) => cylinder.geodesic(x, y),
        }
    }
    // Returns a point grid on the surface, which can be used for visualization.
    fn point_grid(&self, density: f64) -> Vec<Point> {
        match self {
            Surface::Plane(plane) => plane.point_grid(density),
            Surface::Sphere(sphere) => sphere.point_grid(density),
            Surface::Cylinder(cylinder) => cylinder.point_grid(density),
        }
    }
    // Finds the closest point on the surface to the given point.
    fn project(&self, point: Point) -> Point {
        match self {
            Surface::Plane(plane) => plane.project(point),
            Surface::Sphere(sphere) => sphere.project(point),
            Surface::Cylinder(cylinder) => cylinder.project(point),
        }
    }

    // Returns a gradient that leads to the surface.
    fn unsigned_l2_squared_distance_gradient(&self, point: Point) -> Option<Point> {
        match self {
            Surface::Plane(plane) => plane.unsigned_l2_squared_distance_gradient(point),
            Surface::Sphere(sphere) => sphere.unsigned_l2_squared_distance_gradient(point),
            Surface::Cylinder(cylinder) => cylinder.unsigned_l2_squared_distance_gradient(point),
        }
    }
}
