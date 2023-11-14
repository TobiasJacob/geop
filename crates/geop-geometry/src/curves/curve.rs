use std::{rc::Rc, cmp::Ordering};

use crate::{points::point::Point, transforms::Transform, EQ_THRESHOLD};


#[derive(Clone, Debug)]
pub struct TangentParameter(pub(super) f64);

pub enum CurveIntersection {
    None,
    Point(Point),
    Points(Point, Point),
}

// This represents an oriented curve. Curves with redundant representations (e.g. a line with the direction vector not being normalized) have to be normalized in the new function. Unnormalized curves are not allowed.
pub trait Curve {
    // Transform
    fn transform(&self, transform: Transform) -> Rc<dyn Curve>;
    fn neg(&self) -> Rc<dyn Curve>;
    // fn project(&self, p: Point) -> (f64, f64);
    // Tangent / Direction of the curve at the given point. Not normalized.
    fn tangent(&self, p: Point) -> Point;

    // Checks if point is on manifold
    fn on_manifold(&self, p: Point) -> bool;
    // Returns the Riemannian metric between u and v
    fn metric(&self, x: Point, u: TangentParameter, v: TangentParameter) -> f64;
    // Returns the Riemannian distance between x and y (x and y on manifold).
    fn distance(&self, x: Point, y: Point) -> f64;
    // Exponential of u at base x. u_z is ignored.
    fn exp(&self, x: Point, u: TangentParameter) -> Point;
    // Log of y at base x. Z coordinate is set to 0.
    fn log(&self, x: Point, y: Point) -> TangentParameter;
    // Parallel transport of v from x to y.
    fn parallel_transport(&self, v: TangentParameter, x: Point, y: Point) -> TangentParameter;
    // Checks if m is between x and y.
    fn between(&self, m: Point, start: Point, end: Point) -> bool {
        let start = self.log(m, start);
        let end = self.log(m, end);
        start.0 <= 0.0 && end.0 >= 0.0
    }
    // Intersect between start1/2 and end1/2. Returns None if there is no intersection.
    // Keep in mind that all curves are treated as infinite lines, such that start after end means that the line starts, goes to +infinity, goes to -infinty and then ends.
    fn intersect(&self, start1: Point, end1: Point, start2: Point, end2: Point) -> CurveIntersection {
        let start1_in = self.between(start1, start2, end2);
        let end1_in = self.between(end1, start2, end2);
        todo!("Implement intersect")
    }
}
