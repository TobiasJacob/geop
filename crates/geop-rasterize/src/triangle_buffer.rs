use geop_geometry::points::point::Point;

use crate::vertex_buffer::RenderVertex;

// This is a triangle that can be rendered on-screen. The vertices are in counter clockwise order.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RenderTriangle {
    a: RenderVertex,
    b: RenderVertex,
    c: RenderVertex,
}

impl RenderTriangle {
    pub fn new(a: Point, b: Point, c: Point, color: [f32; 3]) -> Self {
        RenderTriangle {
            a: RenderVertex::new(a, color),
            b: RenderVertex::new(b, color),
            c: RenderVertex::new(c, color),
        }
    }
}

pub struct TriangleBuffer {
    triangles: Vec<RenderTriangle>,
}

impl TriangleBuffer {
    // New is only visible in this crate
    pub fn new(triangles: Vec<RenderTriangle>) -> Self {
        TriangleBuffer { triangles }
    }

    pub fn to_u8_slice(&self) -> &[u8] {
        bytemuck::cast_slice(&self.triangles)
    }

    pub fn join(&mut self, other: &TriangleBuffer) {
        self.triangles.extend_from_slice(&other.triangles);
    }
}

