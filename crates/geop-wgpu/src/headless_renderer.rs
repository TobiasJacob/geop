use geop_rasterize::{
    edge::{
        rasterize_edge_into_line_list, rasterize_edge_into_vertex_list,
        rasterize_edges_into_line_list, rasterize_edges_into_vertex_list,
    },
    edge_buffer::EdgeBuffer,
    face::{
        rasterize_face_into_line_list, rasterize_face_into_triangle_list,
        rasterize_face_into_vertex_list,
    },
    triangle_buffer::TriangleBuffer,
    vertex_buffer::{RenderVertex, VertexBuffer},
    volume::{
        rasterize_volume_into_line_list, rasterize_volume_into_triangle_list,
        rasterize_volume_into_vertex_list,
    },
};
use geop_topology::topology::scene::{Color, Scene};
use winit::dpi::PhysicalSize;

use crate::pipeline_manager::PipelineManager;

pub struct HeadlessRenderer {
    pipeline_manager: PipelineManager,
    queue: wgpu::Queue,
    device: wgpu::Device,
    texture_view: wgpu::TextureView,
    output_buffer: wgpu::Buffer,
    texture: wgpu::Texture,
    texture_size: u32,
    copy_size: wgpu::Extent3d,
}

impl HeadlessRenderer {
    pub async fn new() -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&Default::default(), None)
            .await
            .unwrap();
        let texture_size = 1024u32;

        let texture_format = wgpu::TextureFormat::Rgba8UnormSrgb;

        let texture_desc = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: texture_size,
                height: texture_size,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: texture_format,
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
            view_formats: &[texture_format],
        };
        let texture = device.create_texture(&texture_desc);
        let texture_view = texture.create_view(&Default::default());

        let pipeline_manager = PipelineManager::new(
            &device,
            PhysicalSize::new(texture_size, texture_size),
            texture_format,
        )
        .await;

        // we need to store this for later
        let u32_size = std::mem::size_of::<u32>() as u32;

        let output_buffer_size = (u32_size * texture_size * texture_size) as wgpu::BufferAddress;
        let output_buffer_desc = wgpu::BufferDescriptor {
            size: output_buffer_size,
            usage: wgpu::BufferUsages::COPY_DST
        // this tells wpgu that we want to read this buffer from the cpu
        | wgpu::BufferUsages::MAP_READ,
            label: None,
            mapped_at_creation: false,
        };
        let output_buffer = device.create_buffer(&output_buffer_desc);

        // We need to scope the mapping variables so that we can
        // unmap the buffer

        HeadlessRenderer {
            pipeline_manager,
            queue,
            device,
            texture_view,
            output_buffer,
            texture,
            texture_size,
            copy_size: texture_desc.size,
        }
    }

    pub async fn render_to_file(
        &mut self,
        scene: &Scene,
        dark_mode: bool,
        file_path: &std::path::Path,
    ) {
        let (background_color, face_color, edge_color, point_color) =
            Color::standard_pallet(dark_mode);
        let u32_size = std::mem::size_of::<u32>() as u32;
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let render_pass_desc = wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: background_color.r as f64,
                            g: background_color.g as f64,
                            b: background_color.b as f64,
                            a: background_color.a as f64,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.pipeline_manager.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            };
            let mut vertex_buffer = VertexBuffer::empty();
            let mut edge_buffer = EdgeBuffer::empty();
            let mut triangle_buffer = TriangleBuffer::empty();

            for (volume, color) in scene.volumes.iter() {
                vertex_buffer.join(&rasterize_volume_into_vertex_list(
                    volume,
                    *color * point_color,
                ));
                edge_buffer.join(&rasterize_volume_into_line_list(
                    volume,
                    *color * edge_color,
                ));
                triangle_buffer.join(&rasterize_volume_into_triangle_list(
                    volume,
                    *color * face_color,
                ));
            }

            for (face, color) in scene.faces.iter() {
                vertex_buffer.join(&rasterize_face_into_vertex_list(
                    &face,
                    *color * point_color,
                ));
                edge_buffer.join(&rasterize_face_into_line_list(&face, *color * edge_color));
                triangle_buffer.join(&rasterize_face_into_triangle_list(
                    face,
                    *color * face_color,
                ));
            }

            for (edge, color) in scene.edges.iter() {
                vertex_buffer.join(&rasterize_edge_into_vertex_list(edge, *color * point_color));
                edge_buffer.join(&rasterize_edge_into_line_list(edge, *color * edge_color));
            }

            vertex_buffer.join(&VertexBuffer::new(
                scene
                    .points
                    .iter()
                    .map(|(p, color)| RenderVertex::new(p.clone(), *color * point_color))
                    .collect(),
            ));

            let mut render_pass = encoder.begin_render_pass(&render_pass_desc);
            self.pipeline_manager
                .update_edges(&self.queue, &edge_buffer);
            self.pipeline_manager
                .update_triangles(&self.queue, &triangle_buffer);
            self.pipeline_manager
                .update_vertices(&self.queue, &vertex_buffer);
            self.pipeline_manager.run_pipelines(&mut render_pass);
        }

        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::ImageCopyBuffer {
                buffer: &self.output_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(u32_size * self.texture_size),
                    rows_per_image: Some(self.texture_size),
                },
            },
            self.copy_size,
        );
        self.queue.submit(Some(encoder.finish()));

        let buffer_slice = self.output_buffer.slice(..);

        // NOTE: We have to create the mapping THEN device.poll() before await
        // the future. Otherwise the application will freeze.
        let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        self.device.poll(wgpu::Maintain::Wait);
        rx.receive().await.unwrap().unwrap();

        let data = buffer_slice.get_mapped_range();

        use image::{ImageBuffer, Rgba};
        let buffer =
            ImageBuffer::<Rgba<u8>, _>::from_raw(self.texture_size, self.texture_size, data)
                .unwrap();
        // Create folder if it doesn't exist
        std::fs::create_dir_all(file_path.parent().unwrap()).unwrap();
        buffer.save(file_path).unwrap();
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use rstest::fixture;

    #[fixture]
    pub async fn renderer() -> Box<HeadlessRenderer> {
        Box::new(HeadlessRenderer::new().await)
    }
}
