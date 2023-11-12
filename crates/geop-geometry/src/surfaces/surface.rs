
use std::rc::Rc;

use crate::{
    curves::{circle::Circle, ellipse::Ellipse, line::Line},
    points::point::Point, transforms::Transform,
};

pub enum SurfaceCurve {
    // Returns the geodesic between p and q.
    Line(Line),
    // Returns the curve between p and q.
    Circle(Circle),
    // Returns the curve between p and q.
    Ellipse(Ellipse),
}

#[derive(Clone, Debug)]
pub struct TangentPoint(pub Point);

pub trait Surface {
    fn transform(&self, transform: Transform) -> Rc<dyn Surface>;
    fn neg(&self) -> Rc<dyn Surface>;
    // fn project(&self, p: &Point) -> Point
    fn normal(&self, p: Point) -> Point;
    // Checks if the point p is contained in the surface.
    fn on_surface(&self, p: Point) -> bool;

    // Returns the Riemannian metric between u and v
    fn metric(&self, x: Point, u: TangentPoint, v: TangentPoint) -> f64;
    // Returns the Riemannian distance between x and y.
    fn distance(&self, x: Point, y: Point) -> f64;
    // Exponential of u at base x. u_z is ignored.
    fn exp(&self, x: Point, u: TangentPoint) -> Point;
    // Log of y at base x. Z coordinate is set to 0.
    fn log(&self, x: Point, y: Point) -> TangentPoint;
    // Parallel transport of v from x to y.
    fn parallel_transport(&self, v: TangentPoint, x: Point, y: Point) -> Point;
    // Returns the geodesic between p and q.
    fn geodesic(&self, x: Point, y: Point) -> SurfaceCurve;
    // // Angle between a and b at x.
    // fn angle(&self, x: Point, a: Point, b: Point) -> f64;
}
