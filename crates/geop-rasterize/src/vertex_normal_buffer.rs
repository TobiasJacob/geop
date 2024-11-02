use float_next_after::NextAfter;
use geop_geometry::{efloat::EFloat64, point::Point};
use geop_topology::topology::scene::Color;

// This is called RenderVertex to distinguish it from Vertex from topology package.
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct RenderNormalVertex {
    pub min_position: [f32; 3],
    pub max_position: [f32; 3],
    pub color: [f32; 4],
    pub min_normal: [f32; 3],
    pub max_normal: [f32; 3],
}

impl RenderNormalVertex {
    pub fn new(p: Point, color: Color, normal: Point) -> Self {
        RenderNormalVertex {
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
            min_normal: [
                normal.x.lower_bound as f32,
                normal.y.lower_bound as f32,
                normal.z.lower_bound as f32,
            ],
            max_normal: [
                normal.x.upper_bound as f32,
                normal.y.upper_bound as f32,
                normal.z.upper_bound as f32,
            ],
        }
    }

    pub fn point(&self) -> Point {
        Point::new(
            EFloat64::new(
                (self.max_position[0]).next_after(f32::INFINITY) as f64 + 1e-15,
                (self.min_position[0]).next_after(f32::NEG_INFINITY) as f64 - 1e-15,
            ),
            EFloat64::new(
                (self.max_position[1]).next_after(f32::INFINITY) as f64 + 1e-15,
                (self.min_position[1]).next_after(f32::NEG_INFINITY) as f64 - 1e-15,
            ),
            EFloat64::new(
                (self.max_position[2]).next_after(f32::INFINITY) as f64 + 1e-15,
                (self.min_position[2]).next_after(f32::NEG_INFINITY) as f64 - 1e-15,
            ),
        )
    }
}

// Implement conversion to Point
impl From<RenderNormalVertex> for Point {
    fn from(v: RenderNormalVertex) -> Self {
        v.point()
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
