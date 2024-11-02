use geop_geometry::{efloat::EFloat64, point::Point};
use geop_topology::topology::scene::Color;

// This is called RenderVertex to distinguish it from Vertex from topology package.
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct RenderVertex {
    pub min_position: [f32; 3],
    pub max_position: [f32; 3],
    pub color: [f32; 4],
}

impl RenderVertex {
    pub fn new(p: Point, color: Color) -> Self {
        RenderVertex {
            min_position: [
                p.x.lower_bound as f32,
                p.y.lower_bound as f32,
                p.z.lower_bound as f32,
            ],
            max_position: [
                p.x.upper_bound as f32,
                p.y.upper_bound as f32,
                p.z.upper_bound as f32,
            ],
            color: [color.r, color.g, color.b, color.a],
        }
    }

    pub fn point(&self) -> Point {
        Point::new(
            EFloat64::new(
                self.max_position[0] as f64 + 1e-15,
                self.min_position[0] as f64 - 1e-15,
            ),
            EFloat64::new(
                self.max_position[1] as f64 + 1e-15,
                self.min_position[1] as f64 - 1e-15,
            ),
            EFloat64::new(
                self.max_position[2] as f64 + 1e-15,
                self.min_position[2] as f64 - 1e-15,
            ),
        )
    }
}

impl PartialEq for RenderVertex {
    fn eq(&self, other: &Self) -> bool {
        self.point() == other.point() && self.color == other.color
    }
}

// Implement conversion to Point
impl From<RenderVertex> for Point {
    fn from(v: RenderVertex) -> Self {
        v.point()
    }
}

pub struct VertexBuffer {
    pub vertices: Vec<RenderVertex>,
}

impl VertexBuffer {
    // New is only visible in this crate
    pub fn new(vertices: Vec<RenderVertex>) -> Self {
        VertexBuffer { vertices }
    }

    pub fn empty() -> Self {
        VertexBuffer {
            vertices: Vec::new(),
        }
    }

    pub fn to_u8_slice(&self) -> &[u8] {
        bytemuck::cast_slice(&self.vertices)
    }

    pub fn join(&mut self, other: &VertexBuffer) {
        self.vertices.extend_from_slice(&other.vertices);
    }
}
