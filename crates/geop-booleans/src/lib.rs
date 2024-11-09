use geop_topology::topology::{
    contour::{contour_no_point::ContourNoPoint, contour_single_point::ContourSinglePoint},
    edge::Edge,
};

pub mod contains;
pub mod difference;
pub mod intersections;
pub mod remesh;
pub mod split_if_necessary;
pub mod union;

// use topology::scene::Scene;

pub enum BooleanPrimitiveEdge {
    ContourNoPoint(ContourNoPoint),
    ContourSinglePoint(ContourSinglePoint),
    Edge(Edge),
}

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
