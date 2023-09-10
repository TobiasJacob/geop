use geop_geometry::points::point::Point;

use crate::vertex_buffer::RenderVertex;

// This is a line to render it on-screen.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RenderEdge {
    pub start: RenderVertex,
    pub end: RenderVertex,
}

impl RenderEdge {
    pub fn new(start: Point, end: Point, color: [f32; 3]) -> Self {
        RenderEdge {
            start: RenderVertex::new(start, color),
            end: RenderVertex::new(end, color),
        }
    }
}

pub struct EdgeBuffer {
    pub edges: Vec<RenderEdge>,
}

impl EdgeBuffer {
    // New is only visible in this crate
    pub fn new(edges: Vec<RenderEdge>) -> Self {
        EdgeBuffer { edges }
    }

    pub fn empty() -> Self {
        EdgeBuffer {
            edges: Vec::new(),
        }
    }

    pub fn to_u8_slice(&self) -> &[u8] {
        bytemuck::cast_slice(&self.edges)
    }

    pub fn join(&mut self, other: &EdgeBuffer) {
        self.edges.extend_from_slice(&other.edges);
    }
}
