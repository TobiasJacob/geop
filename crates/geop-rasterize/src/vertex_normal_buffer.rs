use geop_geometry::point::Point;
use geop_topology::topology::scene::Color;

// This is called RenderVertex to distinguish it from Vertex from topology package.
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct RenderNormalVertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub normal: [f32; 3],
}

impl RenderNormalVertex {
    pub fn new(p: Point, color: Color, normal: Point) -> Self {
        RenderNormalVertex {
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
            normal: [
                normal.x.lower_bound as f32,
                normal.y.lower_bound as f32,
                normal.z.lower_bound as f32,
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

// Implement conversion to Point
impl From<RenderNormalVertex> for Point {
    fn from(v: RenderNormalVertex) -> Self {
        Point::from_f64(
            v.position[0] as f64,
            v.position[1] as f64,
            v.position[2] as f64,
        )
    }
}

pub struct VertexNormalBuffer {
    pub vertices: Vec<RenderNormalVertex>,
}

impl VertexNormalBuffer {
    // New is only visible in this crate
    pub fn new(vertices: Vec<RenderNormalVertex>) -> Self {
        Self { vertices }
    }

    pub fn empty() -> Self {
        Self {
            vertices: Vec::new(),
        }
    }

    pub fn to_u8_slice(&self) -> &[u8] {
        bytemuck::cast_slice(&self.vertices)
    }

    pub fn join(&mut self, other: &Self) {
        self.vertices.extend_from_slice(&other.vertices);
    }
}
