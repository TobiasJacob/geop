use geop_geometry::point::Point;
use geop_topology::topology::scene::Color;

// This is called RenderVertex to distinguish it from Vertex from topology package.
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct RenderVertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

impl RenderVertex {
    pub fn new(p: Point, color: Color) -> Self {
        RenderVertex {
            position: [
                p.x.lower_bound as f32,
                p.y.lower_bound as f32,
                p.z.lower_bound as f32,
            ],
            color: [
                color.r as f32,
                color.g as f32,
                color.b as f32,
                color.a as f32,
            ],
        }
    }

    pub fn point(&self) -> Point {
        Point::from_f64(
            self.position[0] as f64,
            self.position[1] as f64,
            self.position[2] as f64,
        )
    }
}

impl PartialEq for RenderVertex {
    fn eq(&self, other: &Self) -> bool {
        Point::from_f64(
            self.position[0] as f64,
            self.position[1] as f64,
            self.position[2] as f64,
        ) == Point::from_f64(
            other.position[0] as f64,
            other.position[1] as f64,
            other.position[2] as f64,
        ) && self.color == other.color
    }
}

// Implement conversion to Point
impl From<RenderVertex> for Point {
    fn from(v: RenderVertex) -> Self {
        Point::from_f64(
            v.position[0] as f64,
            v.position[1] as f64,
            v.position[2] as f64,
        )
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
