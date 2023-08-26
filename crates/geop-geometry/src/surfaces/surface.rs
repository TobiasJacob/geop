use crate::{points::point::Point, curves::curve::Curve};

pub trait Surface {
    // Constructs a parameter space which is guaranteed to be continuous and monotonically increasing in both u and v except for the vanishing point.
    // For some cases, like a sphere, the vanishing point is the point where the sphere is cut open to form the parameter space.
    // For some cases, like a plane, the vanishing point is the point at infinity, or somewhere else. In either way, the point does not matter.
    // For a torus, the vanishing point is the point where the two circles intersect that are used to cut open the torus.
    fn point_at(&self, u: f64, v: f64) -> Point;
    fn project(&self, p: &Point) -> (f64, f64);
    fn derivative_u(&self, u: f64, v: f64) -> Point;
    fn derivative_v(&self, u: f64, v: f64) -> Point;
    fn normal(&self, p: Point) -> Point;
    fn normalize(&mut self);
    fn is_normalized(&self) -> bool;
    // fn curve_from_to(&self, p: Point, q: Point) -> dyn Curve;
    // // Returns the metric between u and v
    // fn metric(&self, u: Point, v: Point) -> f64;
    // // Distance between x and y.
    // fn distance(&self, x: Point, y: Point) -> f64;
    // // Exponential of u at base x. u_z is ignored.
    // fn exp(&self, x: Point, u: Point) -> f64;
    // // Log of y at base x. Z coordinate is set to 0.
    // fn log(&self, x: Point, y: Point) -> Point;
    // // Parallel transport of v from x to y.
    // fn parallel_transport(&self, v: Point, x: Point, y: Point) -> Point;
    // // Returns the geodesic between p and q.
    // fn geodesic(&self, p: Point, q: Point) -> dyn Curve;
    // // Angle between a and b at x.
    // fn angle(&self, x: Point, a: Point, b: Point) -> f64;
}
