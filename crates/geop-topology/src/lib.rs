pub mod debug_data;
pub mod topology;

pub mod contains;
pub mod operations;
pub mod primitive_objects;
pub mod topology_error;
pub mod topology_scene;

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
