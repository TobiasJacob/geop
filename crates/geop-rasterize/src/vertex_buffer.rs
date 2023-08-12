use geop_geometry::points::point::Point;

// This is called RenderVertex to distinguish it from Vertex from topology package.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct RenderVertex {
    position: [f32; 3],
    color: [f32; 3]
}

impl RenderVertex {
    pub(crate) fn new(p: Point, color: [f32; 3]) -> Self {
        RenderVertex {
            position: [p.x as f32, p.y as f32, p.z as f32],
            color
        }
    }
}


pub struct VertexBuffer {
    vertices: Vec<RenderVertex>
}

impl VertexBuffer {
    // New is only visible in this crate
    pub(crate) fn new(vertices: Vec<RenderVertex>) -> Self {
        VertexBuffer {
            vertices
        }
    }

    pub fn to_u8_slice(&self) -> &[u8] {
        bytemuck::cast_slice(&self.vertices)
    }
}
