use crate::{curves::CurveLike, points::point::Point, surfaces::SurfaceLike};

pub mod circle_plane;
pub mod curve_surface;
pub mod line_plane;
pub mod line_sphere;

pub fn curve_surface_intersection_numerical(
    curve: &dyn CurveLike,
    surface: &dyn SurfaceLike,
    initial_guesses: Vec<Point>,
    step_size: f64,
) -> Vec<Point> {
    let mut result = Vec::with_capacity(initial_guesses.len());

    for initial_guess in initial_guesses {
        let mut initial_guess = initial_guess;
        for _ in 0..100 {
            let grad = surface.unsigned_l2_squared_distance_gradient(initial_guess);
            let tangent = curve.tangent(initial_guess);
            initial_guess = initial_guess + tangent * step_size;
            initial_guess = curve.project(initial_guess);

            if grad.is_none() {
                break;
            }

            if surface.on_surface(initial_guess) {
                result.push(initial_guess);
                break;
            }
        }
    }

    result
}
