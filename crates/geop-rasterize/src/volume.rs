use geop_topology::topology::volume::Volume;

use crate::{
    edge_buffer::EdgeBuffer,
    face::{rasterize_face_into_line_list, rasterize_face_into_triangle_list},
    triangle_buffer::TriangleBuffer,
};

pub fn rasterize_volume_into_face_list(volume: &Volume, color: [f32; 4]) -> TriangleBuffer {
    let mut buffer = TriangleBuffer::empty();

    for face in volume.boundary.faces.iter() {
        println!("Rasterizing face: {}", face);
        let face_buffer = rasterize_face_into_triangle_list(face, color);
        buffer.join(&face_buffer);
    }

    for hole in volume.holes.iter() {
        for face in hole.faces.iter() {
            println!("Rasterizing face: {}", face);
            let face_buffer = rasterize_face_into_triangle_list(face, color);
            buffer.join(&face_buffer);
        }
    }

    buffer
}

pub fn rasterize_volume_into_line_list(volume: &Volume, color: [f32; 4]) -> EdgeBuffer {
    let mut buffer = EdgeBuffer::empty();

    for face in volume.boundary.faces.iter() {
        let face_buffer = rasterize_face_into_line_list(face, color);
        buffer.join(&face_buffer);
    }

    for hole in volume.holes.iter() {
        for face in hole.faces.iter() {
            let face_buffer = rasterize_face_into_line_list(face, color);
            buffer.join(&face_buffer);
        }
    }

    buffer
}
