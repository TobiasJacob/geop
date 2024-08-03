use geop_topology::topology::{contour::Contour, scene::Color};

use crate::{edge::rasterize_edge_into_line_list, edge_buffer::EdgeBuffer};

// Rasterizes an edge loop into triangle list.
pub fn rasterize_contour_into_line_list(contour: &Contour, color: Color) -> EdgeBuffer {
    let mut edges = EdgeBuffer::new(Vec::new());

    for edge in contour.edges.iter() {
        edges.join(&&rasterize_edge_into_line_list(edge, color));
    }

    edges
}

// Rasterizes multiple edge loop into triangle list.
pub fn rasterize_contours_into_line_list(contour: &[Contour], color: Color) -> EdgeBuffer {
    contour
        .iter()
        .fold(EdgeBuffer::new(Vec::new()), |mut acc, contour| {
            acc.join(&rasterize_contour_into_line_list(contour, color));
            acc
        })
}
