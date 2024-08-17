use curve::Curve;

use crate::{bounding_box::BoundingBox, points::point::Point, transforms::Transform};

pub mod curve;

// Alphabetic order
pub mod circle;
pub mod ellipse;
pub mod line;

// CurveLike is a trait that all curves should implement.
pub trait CurveLike {
    // Transform
    fn transform(&self, transform: Transform) -> Curve;

    // Change the direction of the curve
    fn neg(&self) -> Curve;

    // Normalized Tangent / Direction of the curve at the given point.
    fn tangent(&self, p: Point) -> Point;

    // Checks if point is on the curve.
    fn on_curve(&self, p: Point) -> bool;

    // Returns the distance between x and y.
    fn distance(&self, x: Point, y: Point) -> f64;

    // Interpolate between start and end at t. t is between 0 and 1.
    fn interpolate(&self, start: Option<Point>, end: Option<Point>, t: f64) -> Point;

    // Checks if m is between x and y. m==x and m==y are true.
    fn between(&self, m: Point, start: Option<Point>, end: Option<Point>) -> bool;

    // Get the midpoint between start and end.
    // This will guarantee that between(start, midpoint, end) is true and midpoint != start and midpoint != end.
    // If start or end is None, the midpoint is a point that is a unit distance away from the other point.
    fn get_midpoint(&self, start: Option<Point>, end: Option<Point>) -> Point;

    // Finds the closest point on the curve to the given point.
    fn project(&self, p: Point) -> Point;

    fn get_bounding_box(
        &self,
        interval_self: Option<Point>,
        midpoint_self: Option<Point>,
    ) -> BoundingBox;
}
