use curve::Curve;

use crate::{
    bounding_box::BoundingBox,
    efloat::SemiPositiveEFloat64,
    points::point::{NormalizedPoint, Point},
    transforms::Transform,
};

pub mod curve;

// Alphabetic order
pub mod circle;
pub mod ellipse;
pub mod helix;
pub mod line;

// CurveLike is a trait that all curves should implement.
pub trait CurveLike {
    // Transform
    fn transform(&self, transform: Transform) -> Curve;

    // Change the direction of the curve
    fn neg(&self) -> Curve;

    // Normalized Tangent / Direction of the curve at the given point.
    fn tangent(&self, p: Point) -> NormalizedPoint;

    // Checks if point is on the curve.
    fn on_curve(&self, p: Point) -> bool;

    // Returns the distance between x and y.
    fn distance(&self, x: Point, y: Point) -> SemiPositiveEFloat64;

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

    // Sorts a list of point such that for three consecutive points (p1, p2, p3) p2 is between p1 and p3.
    // For the first and last point, it is (p2, p3, ..., p1) and (p2, p1, ..., p3) respectively.
    fn sort(&self, points: Vec<Option<Point>>) -> Vec<Option<Point>>;
}
