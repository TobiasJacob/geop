use geop_topology::topology::contour::Contour;

use crate::edge_buffer::{EdgeBuffer, RenderEdge};

// Rasterizes an edge loop into triangle list.
pub fn rasterize_contour_into_line_list(contour: &Contour, color: [f32; 4]) -> EdgeBuffer {
    let n = 5;
    let n2 = contour.edges.len();
    let mut edges = Vec::<RenderEdge>::with_capacity(n * n2);
    for i in 0..n2 {
        for j in 0..n {
            let v1 = (j as f64) / n as f64;
            let v2 = ((j + 1) as f64) / n as f64;
            edges.push(RenderEdge::new(
                contour.edges[i].point_at(v1),
                contour.edges[i].point_at(v2),
                color,
            ));
        }
    }
    EdgeBuffer::new(edges)
}

// Rasterizes multiple edge loop into triangle list.
pub fn rasterize_contours_into_line_list(contour: &[Contour], color: [f32; 4]) -> EdgeBuffer {
    contour
        .iter()
        .fold(EdgeBuffer::new(Vec::new()), |mut acc, contour| {
            acc.join(&rasterize_contour_into_line_list(contour, color));
            acc
        })
}
