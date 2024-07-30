// use geop_rasterize::vertex_buffer::VertexBuffer;

// use crate::device_adapter::DeviceAdapter;

// struct HeadlessRenderer {}

// impl HeadlessRenderer {
//     pub fn new(
//         vertices_points: &VertexBuffer,
//         vertices_line: &[u8],
//         vertices_triangle: &[u8],
//         size: winit::dpi::PhysicalSize<u32>,
//     ) -> Self {
//         let device_adapter = DeviceAdapter::new(
//             vertices_points,
//             vertices_line,
//             vertices_triangle,
//             size,
//             None,
//         );

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

//         HeadlessRenderer {}
//     }
// }
