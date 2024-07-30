use geop_rasterize::{
    edge_buffer::EdgeBuffer, triangle_buffer::TriangleBuffer, vertex_buffer::VertexBuffer,
};
use wgpu::TextureFormat;

use crate::{
    camera_pipeline::CameraPipeline, point_render_pipeline::PointRenderPipeline,
    render_pipeline::RenderPipeline,
};

pub struct PipelineManager {
    camera_pipeline: CameraPipeline,
    traingle_pipeline: RenderPipeline,
    line_pipeline: RenderPipeline,
    vertex_pipeline: PointRenderPipeline,
}

impl PipelineManager {
    pub async fn new(
        vertices_points: &VertexBuffer,
        vertices_line: &EdgeBuffer,
        vertices_triangle: &TriangleBuffer,
        size: winit::dpi::PhysicalSize<u32>,
        texture_format: TextureFormat,
        device: &wgpu::Device,
    ) -> Self {
        let camera_pipeline = CameraPipeline::new(device, size);

        let traingle_pipeline = RenderPipeline::new(
            device,
            texture_format,
            vertices_triangle.to_u8_slice(),
            "Triangle",
            wgpu::PrimitiveTopology::TriangleList,
            &camera_pipeline.render_pipeline_layout,
        );

        let line_pipeline = RenderPipeline::new(
            device,
            texture_format,
            vertices_line.to_u8_slice(),
            "Line",
            wgpu::PrimitiveTopology::LineList,
            &camera_pipeline.render_pipeline_layout,
        );

        let vertex_pipeline = PointRenderPipeline::new(
            &device,
            texture_format,
            vertices_points,
            "Vertex",
            &camera_pipeline.render_pipeline_layout,
        );

        PipelineManager {
            camera_pipeline,
            traingle_pipeline,
            line_pipeline,
            vertex_pipeline,
        }
    }

    pub fn update_camera(&mut self, queue: &wgpu::Queue, omega: f32) {
        self.camera_pipeline.camera.eye.x = omega.sin() * 2.0;
        self.camera_pipeline.camera.eye.y = omega.cos() * 2.0;
        self.camera_pipeline
            .camera_uniform
            .update_view_proj(&self.camera_pipeline.camera);
        queue.write_buffer(
            &self.camera_pipeline.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_pipeline.camera_uniform]),
        );
    }

    pub fn run_pipelines<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_bind_group(0, &self.camera_pipeline.camera_bind_group, &[]);
        self.traingle_pipeline.render(render_pass);
        self.line_pipeline.render(render_pass);
        self.vertex_pipeline.render(render_pass);
    }
}
