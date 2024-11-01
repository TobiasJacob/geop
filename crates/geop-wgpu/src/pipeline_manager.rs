use geop_rasterize::{
    edge_buffer::EdgeBuffer, triangle_buffer::TriangleBuffer, vertex_buffer::VertexBuffer,
};
use wgpu::TextureFormat;

use crate::{
    camera_pipeline::CameraPipeline,
    render_pipeline_edge::RenderPipelineEdge,
    render_pipeline_triangle::RenderPipelineTriangle,
    render_pipeline_vertex::RenderPipelineVertex,
    texture::{self, Texture},
};

pub struct PipelineManager {
    camera_pipeline: CameraPipeline,
    traingle_pipeline: RenderPipelineTriangle,
    line_pipeline: RenderPipelineEdge,
    vertex_pipeline: RenderPipelineVertex,
    pub depth_texture: Texture,
}

impl PipelineManager {
    pub async fn new(
        device: &wgpu::Device,
        camera_size: winit::dpi::PhysicalSize<u32>,
        render_texture_format: TextureFormat,
    ) -> Self {
        let depth_texture = Texture::create_depth_texture(&device, &camera_size, "depth_texture");

        let camera_pipeline = CameraPipeline::new(device, camera_size);

        let traingle_pipeline = RenderPipelineTriangle::new(
            device,
            render_texture_format,
            "Triangle",
            &camera_pipeline.render_pipeline_layout,
        );

        let line_pipeline = RenderPipelineEdge::new(
            device,
            render_texture_format,
            "Edge",
            &camera_pipeline.render_pipeline_layout,
        );

        let vertex_pipeline = RenderPipelineVertex::new(
            &device,
            render_texture_format,
            "Vertex",
            &camera_pipeline.render_pipeline_layout,
        );

        PipelineManager {
            camera_pipeline,
            traingle_pipeline,
            line_pipeline,
            vertex_pipeline,
            depth_texture,
        }
    }

    pub fn update_triangles(&mut self, queue: &wgpu::Queue, triangles: &TriangleBuffer) {
        self.traingle_pipeline.update(queue, triangles);
    }

    pub fn update_edges(&mut self, queue: &wgpu::Queue, edges: &EdgeBuffer) {
        self.line_pipeline.update(queue, edges);
    }

    pub fn update_vertices(&mut self, queue: &wgpu::Queue, vertices: &VertexBuffer) {
        self.vertex_pipeline.update(queue, vertices);
    }

    pub fn update_camera(&mut self, queue: &wgpu::Queue, omega: f32) {
        self.camera_pipeline.camera.eye.x = omega.sin() * 2.0;
        self.camera_pipeline.camera.eye.y = omega.cos() * 2.0;
        self.camera_pipeline.camera.eye.z = omega.cos() * 2.0;
        self.camera_pipeline
            .camera_uniform
            .update_view_proj(&self.camera_pipeline.camera);
        queue.write_buffer(
            &self.camera_pipeline.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_pipeline.camera_uniform]),
        );
    }

    pub fn update_camera_pos(
        &mut self,
        queue: &wgpu::Queue,
        camera_pos: geop_geometry::point::Point,
    ) {
        self.camera_pipeline.camera.eye.x = camera_pos.x.lower_bound as f32;
        self.camera_pipeline.camera.eye.y = camera_pos.y.lower_bound as f32;
        self.camera_pipeline.camera.eye.z = camera_pos.z.lower_bound as f32;
        self.camera_pipeline
            .camera_uniform
            .update_view_proj(&self.camera_pipeline.camera);
        queue.write_buffer(
            &self.camera_pipeline.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_pipeline.camera_uniform]),
        )
    }

    pub fn run_pipelines<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_bind_group(0, &self.camera_pipeline.camera_bind_group, &[]);
        render_pass.set_bind_group(1, &self.camera_pipeline.light_bind_group, &[]);
        self.traingle_pipeline.render(render_pass);
        self.line_pipeline.render(render_pass);
        self.vertex_pipeline.render(render_pass);
    }

    pub fn resize(&mut self, device: &wgpu::Device, new_size: winit::dpi::PhysicalSize<u32>) {
        self.depth_texture =
            texture::Texture::create_depth_texture(&device, &new_size, "depth_texture");
    }
}
