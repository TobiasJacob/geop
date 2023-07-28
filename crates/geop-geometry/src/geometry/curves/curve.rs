use crate::geometry::points::point::Point;

// This represents an oriented curve.
pub trait Curve {
    // Projects a point onto the curve. Returns the parameter of the curve and the distance of the point to the curve.
    fn project(&self, p: &Point) -> (f64, f64);
    // Returns the interval of the curve.
    fn point_at(&self, u: f64) -> Point;
    // Tangent / Direction of the curve at the given point. Not normalized.
    fn derivative(&self, u: f64) -> Point;
    // For curves that have redundant representations (e. g. the normal vector of a circle is redundand), this function should normalize the curve.
    fn normalize(&mut self);
    // Check if the curve is normalized.
    fn is_normalized(&self) -> bool;
}