use geop_geometry::points::point::Point;
use geop_rasterize::vertex_buffer::{RenderVertex, VertexBuffer};
use wgpu::{util::DeviceExt, TextureFormat};

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct InstanceRaw {
    pub position: [f32; 3],
}

pub struct RenderPipelineVertex {
    vertex_buffer: wgpu::Buffer,
    instace_buffer: wgpu::Buffer,
    num_vertices: u32,
    num_instances: u32,
    render_pipeline: wgpu::RenderPipeline,
}

// Counter clockwise
fn render_face(
    point1: Point,
    point2: Point,
    point3: Point,
    point4: Point,
    color: [f32; 4],
) -> VertexBuffer {
    return VertexBuffer::new(vec![
        RenderVertex::new(point1, color),
        RenderVertex::new(point2, color),
        RenderVertex::new(point3, color),
        RenderVertex::new(point1, color),
        RenderVertex::new(point3, color),
        RenderVertex::new(point4, color),
    ]);
}

fn cube_vertex_buffer(size: f64, color: [f32; 4]) -> VertexBuffer {
    // Simple 12 triangles to form a cube
    let size = size / 2.0;
    let point1 = Point::new(size, size, -size);
    let point2 = Point::new(size, -size, -size);
    let point3 = Point::new(-size, -size, -size);
    let point4 = Point::new(-size, size, -size);
    let point5 = Point::new(size, size, size);
    let point6 = Point::new(size, -size, size);
    let point7 = Point::new(-size, -size, size);
    let point8 = Point::new(-size, size, size);

    let mut result = VertexBuffer::empty();
    result.join(&render_face(point1, point2, point3, point4, color));
    result.join(&render_face(point8, point7, point6, point5, color));

    result.join(&render_face(point1, point5, point6, point2, color));
    result.join(&render_face(point7, point8, point4, point3, color));

    result.join(&render_face(point5, point1, point4, point8, color));
    result.join(&render_face(point2, point6, point7, point3, color));
    result
}

impl RenderPipelineVertex {
    pub fn new(
        device: &wgpu::Device,
        texture_format: TextureFormat,
        vertices: &VertexBuffer,
        label: &str,
        render_pipeline_layout: &wgpu::PipelineLayout,
    ) -> RenderPipelineVertex {
        let render_vertices = cube_vertex_buffer(0.02, [0.1, 0.1, 0.1, 1.0]);
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{label} Vertex Buffer")),
            contents: render_vertices.to_u8_slice(),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let num_vertices = render_vertices.vertices.len() as u32;

        let instance_data = vertices
            .vertices
            .iter()
            .map(|v| InstanceRaw {
                position: v.position,
            })
            .collect::<Vec<_>>();
        let instace_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{label} Instance Buffer")),
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let num_instances = instance_data.len() as u32;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader_point.wgsl").into()),
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(&format!("{label} Render Pipeline")),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main", // 1.
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<RenderVertex>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                                shader_location: 1,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                        ],
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 2,
                            format: wgpu::VertexFormat::Float32x3,
                        }],
                    },
                ],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    // 4.
                    format: texture_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None,
            cache: None,
        });

        RenderPipelineVertex {
            vertex_buffer,
            instace_buffer,
            num_vertices,
            num_instances,
            render_pipeline,
        }
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.render_pipeline); // 2.
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instace_buffer.slice(..));
        render_pass.draw(0..self.num_vertices, 0..self.num_instances);
    }
}
