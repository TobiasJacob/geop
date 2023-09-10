use geop_topology::topology::object::Object;

use crate::{triangle_buffer::TriangleBuffer, face};


pub fn rasterize_object_into_face_list(object: &Object, color: [f64; 3]) -> TriangleBuffer {
    let buffer = TriangleBuffer::empty();

    for face in object.all_faces() {
        let face_buffer = rasterize_face_into_triangle_list(face, color);
        buffer.join(&face_buffer);
    }

    buffer
}

pub fn rasterize_object_into_line_list(object: &Object, arg: [f64; 3]) -> EdgeBuffer {
    let buffer = EdgeBuffer::empty();

    for face in object.all_faces() {
        let face_buffer = rasterize_face_into_line_list(face, arg);
        buffer.join(&face_buffer);
    }

    buffer
}
