use surface::{Surface, TangentPoint};

use crate::{curves::curve::Curve, points::point::Point, transforms::Transform};

pub mod cylinder;
pub mod plane;
pub mod sphere;
pub mod surface;

pub trait SurfaceLike {
    // Transforms the surface by the given transform.
    fn transform(&self, transform: Transform) -> Surface;

    // Change normal direction of the surface.
    fn neg(&self) -> Surface;

    // Returns the normal of the surface at point p.
    fn normal(&self, p: Point) -> Point;

    // Checks if the point p is on the surface.
    fn on_surface(&self, p: Point) -> bool;

    // Returns the Riemannian metric between u and v
    fn metric(&self, x: Point, u: TangentPoint, v: TangentPoint) -> f64;

    // Returns the Riemannian distance between x and y.
    fn distance(&self, x: Point, y: Point) -> f64;

    // Exponential of u at base x. u_z is ignored.
    fn exp(&self, x: Point, u: TangentPoint) -> Point;

    // Log of y at base x. Z coordinate is set to 0.
    fn log(&self, x: Point, y: Point) -> Option<TangentPoint>;

    // Parallel transport of v from x to y.
    fn parallel_transport(
        &self,
        v: Option<TangentPoint>,
        x: Point,
        y: Point,
    ) -> Option<TangentPoint>;

    // Returns the geodesic between p and q.
    fn geodesic(&self, x: Point, y: Point) -> Curve;

    // Returns a point grid on the surface, which can be used for visualization.
    fn point_grid(&self, density: f64) -> Vec<Point>;

    // Finds the closest point on the surface to the given point.
    fn project(&self, point: Point) -> Point;

    // Returns a gradient that leads to the surface.
    fn unsigned_l2_squared_distance_gradient(&self, point: Point) -> Option<Point>;
}
