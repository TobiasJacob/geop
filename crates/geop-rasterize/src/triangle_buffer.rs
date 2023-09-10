use geop_geometry::points::point::Point;

use crate::{
    edge_buffer::{EdgeBuffer, RenderEdge},
    vertex_buffer::RenderVertex,
};

// This is a triangle that can be rendered on-screen. The vertices are in counter clockwise order.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RenderTriangle {
    pub a: RenderVertex,
    pub b: RenderVertex,
    pub c: RenderVertex,
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
    pub triangles: Vec<RenderTriangle>,
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

    pub fn to_line_list(&self, color: [f32; 3]) -> EdgeBuffer {
        let mut edges = Vec::<RenderEdge>::with_capacity(3 * self.triangles.len());
        for triangle in &self.triangles {
            edges.push(RenderEdge::new(triangle.a.into(), triangle.b.into(), color));
            edges.push(RenderEdge::new(triangle.b.into(), triangle.c.into(), color));
            edges.push(RenderEdge::new(triangle.c.into(), triangle.a.into(), color));
        }
        EdgeBuffer::new(edges)
    }
}
