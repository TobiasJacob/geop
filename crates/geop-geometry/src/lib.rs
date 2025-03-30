pub const HORIZON_DIST: f64 = 100.0; // A big number to represent the distance to the horizon. Used only for visualization purposes.

pub mod algebra_error;
pub mod bounding_box;
pub mod color;
pub mod coordinate_system;
pub mod curve_curve_intersection;
pub mod curve_surface_intersection;
pub mod curves;
pub mod efloat;
pub mod factorial;
pub mod geometry_error;
pub mod geometry_scene;
pub mod intersection;
pub mod point;
pub mod primitives;
pub mod surface_surface_intersection;
pub mod surfaces;
pub mod transforms;

use efloat::EFloat64;
use point::Point;

pub trait MultiDimensionFunction {
    fn eval(&self, t: EFloat64) -> Point;
}
