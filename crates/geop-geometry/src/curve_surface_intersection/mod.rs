use crate::{curves::CurveLike, efloat::EFloat64, point::Point, surfaces::SurfaceLike};

pub mod circle_cylinder;
pub mod circle_plane;
pub mod circle_sphere;
pub mod curve_surface;
pub mod line_cylinder;
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
            let tangent = curve.tangent(initial_guess).unwrap();
            initial_guess = initial_guess + tangent * EFloat64::from(step_size);
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
