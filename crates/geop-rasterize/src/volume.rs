use geop_topology::topology::{scene::Color, volume::Volume};

use crate::{
    edge_buffer::EdgeBuffer,
    face::{rasterize_face_into_line_list, rasterize_face_into_triangle_list},
    triangle_buffer::TriangleBuffer,
    vertex_buffer::{RenderVertex, VertexBuffer},
};

pub fn rasterize_volume_into_triangle_list(volume: &Volume, color: Color) -> TriangleBuffer {
    let mut buffer = TriangleBuffer::empty();

    for face in volume.boundary.faces.iter() {
        println!("Rasterizing face: {}", face);
        let face_buffer = rasterize_face_into_triangle_list(face, color);
        buffer.join(&face_buffer);
    }

    for hole in volume.cavities.iter() {
        for face in hole.faces.iter() {
            println!("Rasterizing face: {}", face);
            let face_buffer = rasterize_face_into_triangle_list(face, color);
            buffer.join(&face_buffer);
        }
    }

    buffer
}

pub fn rasterize_volume_into_line_list(volume: &Volume, color: Color) -> EdgeBuffer {
    let mut buffer = EdgeBuffer::empty();

    for face in volume.boundary.faces.iter() {
        let face_buffer = rasterize_face_into_line_list(face, color);
        buffer.join(&face_buffer);
    }

    for hole in volume.cavities.iter() {
        for face in hole.faces.iter() {
            let face_buffer = rasterize_face_into_line_list(face, color);
            buffer.join(&face_buffer);
        }
    }

    buffer
}

pub fn rasterize_volume_into_vertex_list(volume: &Volume, color: Color) -> VertexBuffer {
    let mut buffer = VertexBuffer::empty();

    for face in volume.all_faces().iter() {
        for point in face.all_points().iter() {
            buffer.vertices.push(RenderVertex::new(*point, color));
        }
    }

    buffer
}
