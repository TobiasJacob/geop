use geop_topology::topology::volume::Volume;

use crate::{triangle_buffer::TriangleBuffer, face::{rasterize_face_into_triangle_list, rasterize_face_into_line_list}, edge_buffer::EdgeBuffer};


pub fn rasterize_volume_into_face_list(volume: &Volume, color: [f32; 4]) -> TriangleBuffer {
    let mut buffer = TriangleBuffer::empty();

    for face in volume.faces.iter() {
        println!("Rasterizing face: {}", face);
        let face_buffer = rasterize_face_into_triangle_list(face, color);
        buffer.join(&face_buffer);
    }

    buffer
}

pub fn rasterize_volume_into_line_list(volume: &Volume, color: [f32; 4]) -> EdgeBuffer {
    let mut buffer = EdgeBuffer::empty();

    for face in volume.faces.iter() {
        let face_buffer = rasterize_face_into_line_list(face, color);
        buffer.join(&face_buffer);
    }

    buffer
}
