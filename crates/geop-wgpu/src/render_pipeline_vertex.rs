use geop_geometry::points::point::Point;
use geop_rasterize::vertex_buffer::{RenderVertex, VertexBuffer};
use geop_topology::topology::scene::Color;
use wgpu::{util::DeviceExt, TextureFormat};

use crate::texture;

pub struct RenderPipelineVertex {
    vertex_buffer: wgpu::Buffer,
    instace_buffer: wgpu::Buffer,
    num_vers_per_instance: u32,
    max_num_instances: usize,
    render_instances: u32,
    render_pipeline: wgpu::RenderPipeline,
}

// Counter clockwise
fn render_face(
    point1: Point,
    point2: Point,
    point3: Point,
    point4: Point,
    color: Color,
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

fn cube_vertex_buffer(size: f64, color: Color) -> VertexBuffer {
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
        label: &str,
        render_pipeline_layout: &wgpu::PipelineLayout,
    ) -> RenderPipelineVertex {
        let max_num_instances = 1024;

        let render_vertices = cube_vertex_buffer(0.02, Color::white());
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{label} Vertex Buffer")),
            contents: render_vertices.to_u8_slice(),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let num_vers_per_instance = render_vertices.vertices.len() as u32;
        let instace_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{label} Instance Buffer")),
            contents: vec![0u8; max_num_instances * std::mem::size_of::<RenderVertex>()].as_slice(),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader_vertex.wgsl").into()),
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
                        array_stride: std::mem::size_of::<RenderVertex>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 2,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                                shader_location: 3,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                        ],
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
            depth_stencil: Some(wgpu::DepthStencilState {
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less, // 1.
                stencil: wgpu::StencilState::default(),     // 2.
                bias: wgpu::DepthBiasState::default(),
            }),
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
            num_vers_per_instance,
            max_num_instances,
            render_instances: 0,
            render_pipeline,
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue, instances: &VertexBuffer) {
        if self.max_num_instances < instances.vertices.len() {
            panic!("Too many instances to render");
        }
        queue.write_buffer(&self.instace_buffer, 0, instances.to_u8_slice());
        self.render_instances = instances.vertices.len() as u32;
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.render_pipeline); // 2.
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instace_buffer.slice(..));
        render_pass.draw(0..self.num_vers_per_instance, 0..self.render_instances);
    }
}
