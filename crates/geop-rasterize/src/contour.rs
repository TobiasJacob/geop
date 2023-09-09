use geop_topology::topology::{contour::Contour};

use crate::{vertex_buffer::{RenderVertex, VertexBuffer}, edge_buffer::{EdgeBuffer, RenderEdge}};

// Rasterizes an edge loop into triangle list.
pub fn rasterize_contour_into_line_list(contour: &Contour, color: [f32; 3]) -> EdgeBuffer {
    let n = 200;
    let mut edges = Vec::<RenderEdge>::with_capacity(2 * n);
    for i in 0..n {
        let u = i as f64 / n as f64;
        edges.push(RenderEdge::new(contour.point_at(u), contour.point_at(u + 1.0 / n as f64), color));
    }
    EdgeBuffer::new(edges)
}

// Rasterizes multiple edge loop into triangle list.
pub fn rasterize_contours_into_line_list(contour: &[Contour], color: [f32; 3]) -> EdgeBuffer {
    contour.iter().fold(EdgeBuffer::new(Vec::new()), |mut acc, contour| {
        acc.join(&rasterize_contour_into_line_list(contour, color));
        acc
    })
}

// // Rasterizes an edge loop into triangle list.
// pub fn rasterize_contour_triangle(contour: Contour, camera_pos: Point, width: f64, color: [f32; 3]) -> VertexBuffer {
//     let n = 50;
//     let mut points = Vec::<Point>::with_capacity(n);
//     for i in 0..n {
//         let u = i as f64 / n as f64;
//         points.push(contour.point_at(u));
//     }

//     let mut offset_points = Vec::<Point>::with_capacity(2 * n);
//     for i in 0..n {
//         let prev_p = points[(i + n - 1) % n];
//         let cur_p = points[i];
//         let next_p = points[(i + 1) % n];

//         let prev_dir = (cur_p - prev_p).normalize();
//         let next_dir = (next_p - cur_p).normalize();
//         let cur_dir = (next_dir + prev_dir).normalize();

//         let camera_dir = (camera_pos - cur_p).normalize();
//         let width_dir = cur_dir.cross(camera_dir).normalize();
//         offset_points.push(cur_p + width_dir * width);
//         offset_points.push(cur_p - width_dir * width);
//     }

//     let mut vertices = Vec::<RenderVertex>::with_capacity(6 * n);
//     for i in 0..n {
//         let e1 = offset_points[2 * i];
//         let e2 = offset_points[2 * i + 1];
//         let e3 = offset_points[2 * ((i + 1) % n)];
//         let e4 = offset_points[2 * ((i + 1) % n) + 1];

//         vertices.push(RenderVertex::new(e1, color));
//         vertices.push(RenderVertex::new(e3, color));
//         vertices.push(RenderVertex::new(e2, color));

//         vertices.push(RenderVertex::new(e2, color));
//         vertices.push(RenderVertex::new(e3, color));
//         vertices.push(RenderVertex::new(e4, color));
//     }

//     VertexBuffer::new(vertices)
// }