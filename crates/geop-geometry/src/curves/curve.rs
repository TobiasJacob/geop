use std::rc::Rc;

use crate::{points::point::Point, transforms::Transform};


#[derive(Clone, Debug)]
pub struct TangentParameter(pub f64);

// This represents an oriented curve. Curves with redundant representations (e.g. a line with the direction vector not being normalized) have to be normalized in the new function. Unnormalized curves are not allowed.
pub trait Curve {
    // Transform
    fn transform(&self, transform: Transform) -> Rc<dyn Curve>;
    fn neg(&self) -> Rc<dyn Curve>;
    // fn project(&self, p: Point) -> (f64, f64);
    // Tangent / Direction of the curve at the given point. Not normalized.
    fn tangent(&self, p: Point) -> Point;

    // Returns the Riemannian metric between u and v
    fn metric(&self, x: Point, u: TangentParameter, v: TangentParameter) -> f64;
    // Returns the Riemannian distance between x and y (x and y on manifold).
    fn distance(&self, x: Point, y: Point) -> f64;
    // Exponential of u at base x. u_z is ignored.
    fn exp(&self, x: Point, u: TangentParameter) -> Point;
    // Log of y at base x. Z coordinate is set to 0.
    fn log(&self, x: Point, y: Point) -> TangentParameter;
    // Parallel transport of v from x to y.
    fn parallel_transport(&self, v: TangentParameter, x: Point, y: Point) -> Point;
}
