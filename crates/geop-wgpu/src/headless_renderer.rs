// use geop_rasterize::vertex_buffer::VertexBuffer;
// use wgpu::TextureDescriptor;

// use crate::device_adapter::DeviceAdapter;

// struct HeadlessRenderer {}

// impl HeadlessRenderer {
//     pub async fn new(
//         vertices_points: &VertexBuffer,
//         vertices_line: &[u8],
//         vertices_triangle: &[u8],
//         size: winit::dpi::PhysicalSize<u32>,
//     ) -> Self {
//         let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
//             backends: wgpu::Backends::all(),
//             dx12_shader_compiler: Default::default(),
//         });

//         let adapter = instance
//             .request_adapter(&wgpu::RequestAdapterOptions {
//                 power_preference: wgpu::PowerPreference::default(),
//                 compatible_surface: None,
//                 force_fallback_adapter: false,
//             })
//             .await
//             .unwrap();
//         let (device, queue) = adapter
//             .request_device(&Default::default(), None)
//             .await
//             .unwrap();
//         let texture_size = 256u32;

//         let texture_desc = wgpu::TextureDescriptor {
//             size: wgpu::Extent3d {
//                 width: texture_size,
//                 height: texture_size,
//                 depth_or_array_layers: 1,
//             },
//             mip_level_count: 1,
//             sample_count: 1,
//             dimension: wgpu::TextureDimension::D2,
//             format: wgpu::TextureFormat::Rgba8UnormSrgb,
//             usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
//             label: None,
//             view_formats: &[wgpu::TextureFormat::Rgba8UnormSrgb.into()],
//         };
//         let texture = device.create_texture(&texture_desc);
//         let texture_view = texture.create_view(&Default::default());

//         // we need to store this for later
//         let u32_size = std::mem::size_of::<u32>() as u32;

//         let output_buffer_size = (u32_size * texture_size * texture_size) as wgpu::BufferAddress;
//         let output_buffer_desc = wgpu::BufferDescriptor {
//             size: output_buffer_size,
//             usage: wgpu::BufferUsages::COPY_DST
//         // this tells wpgu that we want to read this buffer from the cpu
//         | wgpu::BufferUsages::MAP_READ,
//             label: None,
//             mapped_at_creation: false,
//         };
//         let output_buffer = device.create_buffer(&output_buffer_desc);

//         let mut encoder =
//             device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
//         {
//             let render_pass_desc = wgpu::RenderPassDescriptor {
//                 label: Some("Render Pass"),
//                 color_attachments: &[Some(wgpu::RenderPassColorAttachment {
//                     view: &texture_view,
//                     resolve_target: None,
//                     ops: wgpu::Operations {
//                         load: wgpu::LoadOp::Clear(wgpu::Color {
//                             r: 0.1,
//                             g: 0.2,
//                             b: 0.3,
//                             a: 1.0,
//                         }),
//                         store: wgpu::StoreOp::Store,
//                     },
//                 })],
//                 depth_stencil_attachment: None,
//             };
//             let mut render_pass = encoder.begin_render_pass(&render_pass_desc);

//             render_pass.set_pipeline(&render_pipeline);
//             render_pass.draw(0..3, 0..1);
//         }
//         encoder.copy_texture_to_buffer(
//             wgpu::ImageCopyTexture {
//                 aspect: wgpu::TextureAspect::All,
//                 texture: &texture,
//                 mip_level: 0,
//                 origin: wgpu::Origin3d::ZERO,
//             },
//             wgpu::ImageCopyBuffer {
//                 buffer: &output_buffer,
//                 layout: wgpu::ImageDataLayout {
//                     offset: 0,
//                     bytes_per_row: u32_size * texture_size,
//                     rows_per_image: texture_size,
//                 },
//             },
//             texture_desc.size,
//         );
//         queue.submit(Some(encoder.finish()));

//         // We need to scope the mapping variables so that we can
//         // unmap the buffer
//         {
//             let buffer_slice = output_buffer.slice(..);

//             // NOTE: We have to create the mapping THEN device.poll() before await
//             // the future. Otherwise the application will freeze.
//             let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
//             buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
//                 tx.send(result).unwrap();
//             });
//             device.poll(wgpu::Maintain::Wait);
//             rx.receive().await.unwrap().unwrap();

//             let data = buffer_slice.get_mapped_range();

//             use image::{ImageBuffer, Rgba};
//             let buffer =
//                 ImageBuffer::<Rgba<u8>, _>::from_raw(texture_size, texture_size, data).unwrap();
//             buffer.save("image.png").unwrap();
//         }
//         output_buffer.unmap();

//         HeadlessRenderer {}
//     }
// }
