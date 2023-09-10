use geop_topology::topology::object::Object;

use crate::{triangle_buffer::TriangleBuffer, face::{rasterize_face_into_triangle_list, rasterize_face_into_line_list}, edge_buffer::EdgeBuffer};


pub fn rasterize_object_into_face_list(object: &Object, color: [f32; 4]) -> TriangleBuffer {
    let mut buffer = TriangleBuffer::empty();

    for face in object.faces.iter() {
        println!("Rasterizing face: {}", face);
        let face_buffer = rasterize_face_into_triangle_list(face, color);
        buffer.join(&face_buffer);
    }

    buffer
}

pub fn rasterize_object_into_line_list(object: &Object, color: [f32; 4]) -> EdgeBuffer {
    let mut buffer = EdgeBuffer::empty();

    for face in object.faces.iter() {
        let face_buffer = rasterize_face_into_line_list(face, color);
        buffer.join(&face_buffer);
    }

    buffer
}
