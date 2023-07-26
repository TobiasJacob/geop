use crate::geometry::points::point3d::Point3d;

// This represents an oriented curve.
pub trait Curve3d {
    // Calculate the point at a given parameter value.
    fn point_at(&self, u: f64) -> Point3d;
    // Project an interval to the parameter space, while making sure that the end value is larger than the start value. This is imporant for periodic curves.
    fn interval(&self, start: Point3d, end: Point3d) -> (f64, f64);
    // For curves that have redundant representations (e. g. the normal vector of a circle is redundand), this function should normalize the curve.
    fn normalize(&mut self);
    // Check if the curve is normalized.
    fn is_normalized(&self) -> bool;
    // Calculate the period of the curve. Is f64::INFINITY for curves that are not periodic.
    fn period(&self) -> f64;
}