use crate::points::point::Point;

// This represents an oriented curve. Curves with redundant representations (e.g. a line with the direction vector not being normalized) have to be normalized in the new function. Unnormalized curves are not allowed.
pub trait Curve {
    // Projects a point onto the curve. Returns the parameter of the curve and the distance of the point to the curve.
    fn project(&self, p: Point) -> (f64, f64);
    // Returns the interval of the curve.
    fn point_at(&self, u: f64) -> Point;
    // Tangent / Direction of the curve at the given point. Not normalized.
    fn derivative(&self, u: f64) -> Point;
}
