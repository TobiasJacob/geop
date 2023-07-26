use crate::geometry::points::point::Point;

pub trait SurfaceParameterSpace {
    fn point_at(&self, u: f64, v: f64) -> Point;
    fn project(&self, point: &Point) -> (f64, f64);
    fn derivative(&self, u: f64, v: f64) -> (Point, Point);
    fn rasterize(&self, us: Vec<f64>, vs: Vec<f64>) -> Vec<Point>;
}

pub trait Surface {
    // Constructs a parameter space which is guaranteed to be continuous and monotonically increasing in both u and v except for the vanishing point.
    // For some cases, like a sphere, the vanishing point is the point where the sphere is cut open to form the parameter space.
    // For some cases, like a plane, the vanishing point is the point at infinity, or somewhere else. In either way, the point does not matter.
    // For a torus, the vanishing point is the point where the two circles intersect that are used to cut open the torus.
    fn construct_parameter_space(&self, vanishing_point: Point) -> dyn SurfaceParameterSpace;
    fn normal(&self, p: Point) -> Point;
    fn normalize(&mut self);
    fn is_normalized(&self) -> bool;
}
