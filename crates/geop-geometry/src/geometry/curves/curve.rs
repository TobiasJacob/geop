use crate::geometry::points::point::Point;

pub trait CircularCurveParameterSpace {
    fn interval(&self) -> (f64, f64);
    fn length(&self) -> f64;
    fn point_at(&self, u: f64) -> Point;
    fn project(&self, point: &Point) -> f64;
    fn derivative(&self, u: f64) -> Point;
    fn rasterize(&self) -> Vec<Point>;
}

pub trait LinearCurveParameterSpace {
    fn point_at(&self, u: f64) -> Point;
    fn project(&self, point: &Point) -> f64;
    fn derivative(&self, u: f64) -> Point;
    fn rasterize(&self, us: Vec<f64>) -> Vec<Point>;
}

// This represents an oriented curve.
pub trait Curve {
    // Construct a new parameter space topology such that the parameter is continuous and monotonically increasing between start and end.
    fn construct_parameter_space(&self, origin: &Point) -> dyn CurveParameterSpace;
    // Tangent / Direction of the curve at the given point. Normalized.
    fn tangent(&self, u: Point) -> Point;
    // For curves that have redundant representations (e. g. the normal vector of a circle is redundand), this function should normalize the curve.
    fn normalize(&mut self);
    // Check if the curve is normalized.
    fn is_normalized(&self) -> bool;
    // Calculate the period of the curve. Is f64::INFINITY for curves that are not periodic.
    fn period(&self) -> f64;
    // Sorts points in the order of the curve.
    fn sort_points(&self, points: &mut Vec<Point>);
}