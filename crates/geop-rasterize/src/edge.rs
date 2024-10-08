use geop_geometry::{curves::curve::Curve, HORIZON_DIST};
use geop_topology::topology::{edge::Edge, scene::Color};

use crate::{
    edge_buffer::{EdgeBuffer, RenderEdge},
    vertex_buffer::{RenderVertex, VertexBuffer},
};

// Rasterizes an edge loop into triangle list.
pub fn rasterize_edge_into_line_list(edge: &Edge, color: Color) -> EdgeBuffer {
    let n = match edge.curve {
        Curve::Line(_) => 10,
        Curve::Circle(_) => 32,
        Curve::Ellipse(_) => 32,
        Curve::Helix(_) => 32 * HORIZON_DIST as usize,
    };
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
pub fn rasterize_edges_into_line_list(edges: &[Edge], color: Color) -> EdgeBuffer {
    edges
        .iter()
        .fold(EdgeBuffer::new(Vec::new()), |mut acc, edge| {
            acc.join(&rasterize_edge_into_line_list(edge, color));
            acc
        })
}

pub fn rasterize_edge_into_vertex_list(edge: &Edge, color: Color) -> VertexBuffer {
    let mut verts = Vec::with_capacity(2);
    if let Some(start) = edge.start {
        verts.push(RenderVertex::new(start, color));
    }
    if let Some(end) = edge.end {
        verts.push(RenderVertex::new(end, color));
    }
    VertexBuffer::new(verts)
}

pub fn rasterize_edges_into_vertex_list(edges: &[Edge], color: Color) -> VertexBuffer {
    let mut verts = Vec::with_capacity(edges.len() * 2);
    for e in edges {
        if let Some(start) = e.start {
            verts.push(RenderVertex::new(start, color));
        }
        if let Some(end) = e.end {
            verts.push(RenderVertex::new(end, color));
        }
    }
    VertexBuffer::new(verts)
}
