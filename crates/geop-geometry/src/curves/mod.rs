use crate::efloat::EFloat64;
use curve::Curve;

use crate::{
    bounding_box::BoundingBox, geometry_error::GeometryResult, point::Point, transforms::Transform,
};

pub mod curve;

pub mod bernstein_polynomial;
pub mod bspline_curve;
pub mod circle;
pub mod ellipse;
pub mod helix;
pub mod line;
pub mod monomial_polynom;
pub mod nurbs_curve;

// CurveLike is a trait that all curves should implement.
pub trait CurveLike {
    // Transform
    fn transform(&self, transform: Transform) -> Curve;

    // Change the direction of the curve
    fn neg(&self) -> Curve;

    // Normalized Tangent / Direction of the curve at the given point.
    fn tangent(&self, p: Point) -> GeometryResult<Point>;

    // Checks if point is on the curve.
    fn on_curve(&self, p: Point) -> bool;

    // Returns the distance between x and y. Fails if x and y are not on the curve.
    fn distance(&self, x: Point, y: Point) -> GeometryResult<EFloat64>;

    // Interpolate between start and end at t. t is between 0 and 1.
    fn interpolate(
        &self,
        start: Option<Point>,
        end: Option<Point>,
        t: f64,
    ) -> GeometryResult<Point>;

    // Checks if m is between x and y. m==x and m==y are true.
    fn between(&self, m: Point, start: Option<Point>, end: Option<Point>) -> GeometryResult<bool>;

    // Get the midpoint between start and end.
    // This will guarantee that between(start, midpoint, end) is true and midpoint != start and midpoint != end.
    // If start or end is None, the midpoint is a point that is a unit distance away from the other point.
    // This operation will fail if start == end.
    fn get_midpoint(&self, start: Option<Point>, end: Option<Point>) -> GeometryResult<Point>;

    // Finds the closest point on the curve to the given point.
    fn project(&self, p: Point) -> Point;

    // Returns a bounding box that contains the curve.
    fn get_bounding_box(
        &self,
        start: Option<Point>,
        end: Option<Point>,
    ) -> GeometryResult<BoundingBox>;

    // Shrinks a bounding box to the smallest box that contains still the same part of the curve. The new box is <= the old box.
    fn shrink_bounding_box(
        &self,
        start: Option<Point>,
        end: Option<Point>,
        bounding_box: BoundingBox,
    ) -> GeometryResult<BoundingBox>;

    // Sorts a list of point such that for three consecutive points (p1, p2, p3) p2 is between p1 and p3.
    // For the first and last point, it is (p2, p3, ..., p1) and (p2, p1, ..., p3) respectively.
    fn sort(&self, points: Vec<Option<Point>>) -> Vec<Option<Point>>;
}
