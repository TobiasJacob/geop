use geop_geometry::{surfaces::surface::Surface, curves::curve::Curve};

use crate::topology::edge::Edge;

use super::edge_edge::EdgeEdgeIntersection;


pub fn surface_edge_intersection(surface: &Surface, other: &Edge) -> Vec<EdgeEdgeIntersection> {
    match surface {
        Surface::Plane(_plane) => match &*other.curve {
            Curve::Line(_line) => {
                let _p = todo!("asdf");
            }
            _ => todo!("Not implemented"),
        },
        _ => todo!("Not implemented"),
    }
}
