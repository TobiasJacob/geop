use std::iter;

use wgpu::util::DeviceExt;
use winit::{event::*, window::Window};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::{render_pipeline::RenderPipeline, camera_pipeline::CameraPipeline};

pub struct WindowState {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Window,
    camera_pipeline: CameraPipeline,

    traingle_pipeline: RenderPipeline,
    line_pipeline: RenderPipeline,
}

impl WindowState {
    pub async fn new(window: Window, vertices_line: &[u8], vertices_triangle: &[u8]) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                },
                // Some(&std::path::Path::new("trace")), // Trace path
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an Srgb surface texture. Using a different
        // one will result all the colors comming out darker. If you want to support non
        // Srgb surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let camera_pipeline = CameraPipeline::new(&device, &config);


        let traingle_pipeline = RenderPipeline::new(
            &device,
            &config,
            vertices_triangle,
            "Triangle",
            wgpu::PrimitiveTopology::TriangleList,
            &camera_pipeline.render_pipeline_layout,
        );

        let line_pipeline = RenderPipeline::new(
            &device,
            &config,
            vertices_line,
            "Line",
            wgpu::PrimitiveTopology::LineList,
            &camera_pipeline.render_pipeline_layout,
        );

        Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
            traingle_pipeline,
            line_pipeline,
            camera_pipeline,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.size
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    #[allow(unused_variables)]
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let time_in_seconds = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        let pi = std::f32::consts::PI;
        let time_in_seconds = (time_in_seconds % 1000000) as f32 / 1000.0;
        let rotations_per_second = 0.1;
        let omega = time_in_seconds * 2.0 * pi * rotations_per_second;

        self.camera_pipeline.camera.eye.x = omega.sin() * 2.0;
        self.camera_pipeline.camera.eye.y = omega.cos() * 2.0;
        self.camera_pipeline.camera_uniform.update_view_proj(&self.camera_pipeline.camera);
        self.queue.write_buffer(&self.camera_pipeline.camera_buffer, 0, bytemuck::cast_slice(&[self.camera_pipeline.camera_uniform]));

        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.1,
                            b: 0.1,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            
            render_pass.set_bind_group(0, &self.camera_pipeline.camera_bind_group, &[]);
            self.line_pipeline.render(&mut render_pass);
            self.traingle_pipeline.render(&mut render_pass);
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
