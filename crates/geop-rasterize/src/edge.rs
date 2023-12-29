use geop_topology::topology::{contour::Contour, edge::Edge};

use crate::edge_buffer::{EdgeBuffer, RenderEdge};

// Rasterizes an edge loop into triangle list.
pub fn rasterize_edge_into_line_list(edge: &Edge, color: [f32; 4]) -> EdgeBuffer {
    let n = 5;
    let mut edges = Vec::<RenderEdge>::with_capacity(n);
    for j in 0..n {
        let v1 = (j as f64) / n as f64;
        let v2 = ((j + 1) as f64) / n as f64;
        edges.push(RenderEdge::new(
            edge.interpolate(v1),
            edge.interpolate(v2),
            color,
        ));
    }
    EdgeBuffer::new(edges)
}

// Rasterizes multiple edge loop into triangle list.
pub fn rasterize_edges_into_line_list(edges: &[Edge], color: [f32; 4]) -> EdgeBuffer {
    edges
        .iter()
        .fold(EdgeBuffer::new(Vec::new()), |mut acc, edge| {
            acc.join(&rasterize_edge_into_line_list(edge, color));
            acc
        })
}
