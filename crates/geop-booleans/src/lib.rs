pub mod contains;
pub mod curve_curve_intersection;
pub mod curve_surface_intersection;
pub mod difference;
pub mod intersections;
pub mod remesh;
pub mod split_if_necessary;
pub mod surface_surface_intersection;
pub mod union;

// use topology::scene::Scene;

// #[derive(Clone, Debug)]
// struct GeopTopologyError {
//     message: String,
//     scene: Option<Scene>,
// }

// impl GeopTopologyError {
//     fn new(message: String, scene: Option<Scene>) -> GeopTopologyError {
//         GeopTopologyError { message, scene }
//     }
// }

// impl std::fmt::Display for GeopTopologyError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "GeopTopologyError: {}", self.message)
//     }
// }

// impl std::error::Error for GeopTopologyError {}

// impl PartialEq for GeopTopologyError {
//     fn eq(&self, _other: &GeopTopologyError) -> bool {
//         true
//     }
// }
