use geop_rasterize::vertex_buffer::VertexBuffer;
use winit::dpi::PhysicalSize;

use crate::pipeline_manager::PipelineManager;

struct HeadlessRenderer {}

impl HeadlessRenderer {
    pub async fn new(
        vertices_points: &VertexBuffer,
        vertices_line: &[u8],
        vertices_triangle: &[u8],
    ) -> Self {
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
            vertices_points,
            vertices_line,
            vertices_triangle,
            PhysicalSize::new(texture_size, texture_size),
            texture_format,
            &device,
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

        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let render_pass_desc = wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            };
            let mut render_pass = encoder.begin_render_pass(&render_pass_desc);
            pipeline_manager.run_pipelines(&mut render_pass);
        }
        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::ImageCopyBuffer {
                buffer: &output_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(u32_size * texture_size),
                    rows_per_image: Some(texture_size),
                },
            },
            texture_desc.size,
        );
        queue.submit(Some(encoder.finish()));

        // We need to scope the mapping variables so that we can
        // unmap the buffer
        {
            let buffer_slice = output_buffer.slice(..);

            // NOTE: We have to create the mapping THEN device.poll() before await
            // the future. Otherwise the application will freeze.
            let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
            buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
                tx.send(result).unwrap();
            });
            device.poll(wgpu::Maintain::Wait);
            rx.receive().await.unwrap().unwrap();

            let data = buffer_slice.get_mapped_range();

            use image::{ImageBuffer, Rgba};
            let buffer =
                ImageBuffer::<Rgba<u8>, _>::from_raw(texture_size, texture_size, data).unwrap();
            buffer.save("image.png").unwrap();
        }
        output_buffer.unmap();

        HeadlessRenderer {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use geop_rasterize::volume::{
        rasterize_volume_into_face_list, rasterize_volume_into_line_list,
        rasterize_volume_into_vertex_list,
    };
    use geop_topology::primitive_objects::cube::primitive_cube;

    #[test]
    fn test_headless_renderer() {
        let volume = primitive_cube(1.0, 1.0, 1.0);

        let vertex_buffer = rasterize_volume_into_vertex_list(&volume, [0.2, 0.2, 0.2, 1.0]);
        let edge_buffer = rasterize_volume_into_line_list(&volume, [0.0, 0.0, 0.0, 1.0]);
        let triangle_buffer = rasterize_volume_into_face_list(&volume, [0.6, 0.6, 0.6, 1.0]);
        let _headless_renderer = pollster::block_on(HeadlessRenderer::new(
            &vertex_buffer,
            edge_buffer.to_u8_slice(),
            triangle_buffer.to_u8_slice(),
        ));
    }
}
