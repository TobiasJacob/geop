use crate::{curves::CurveLike, points::point::Point, surfaces::SurfaceLike};

pub mod circle_plane;
pub mod curve_surface;
pub mod line_plane;
pub mod line_sphere;

pub fn curve_surface_intersection_numerical(
    _curve: &dyn CurveLike,
    _surface: &dyn SurfaceLike,
) -> Vec<Point> {
    todo!()
}
