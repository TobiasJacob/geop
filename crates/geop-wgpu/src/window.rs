use geop_rasterize::{
    edge_buffer::EdgeBuffer, triangle_buffer::TriangleBuffer, vertex_buffer::VertexBuffer,
};
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

use crate::window_state::WindowState;

pub struct GeopWindow<'a> {
    state: WindowState<'a>,
}

impl<'a> GeopWindow<'a> {
    pub async fn new(
        vertex_buffer_points: VertexBuffer,
        vertex_buffer_line: EdgeBuffer,
        vertex_buffer_triange: TriangleBuffer,
        window: &'a Window,
    ) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                std::panic::set_hook(Box::new(console_error_panic_hook::hook));
                console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
            } else {
                env_logger::init();
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            // Winit prevents sizing with CSS, so we have to set
            // the size manually when on web.
            use winit::dpi::PhysicalSize;
            window.set_inner_size(PhysicalSize::new(450, 400));

            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("wasm-example")?;
                    let canvas = web_sys::Element::from(window.canvas());
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }

        // let vertex_buffer = rasterize_contour_triangle(
        //     contour,
        //     Point::new(0.0, 0.0, 100.0),
        //     0.01,
        //     [1.0, 1.0, 1.0]
        // );
        let state = WindowState::new(
            window,
            &vertex_buffer_points,
            &vertex_buffer_line,
            &vertex_buffer_triange,
        )
        .await;

        Self { state }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
    pub fn show(self, event_loop: EventLoop<()>) {
        let mut state = self.state;
        event_loop
            .run(move |event, control_flow| {
                match event {
                    Event::WindowEvent {
                        ref event,
                        window_id,
                    } if window_id == state.window().id() => {
                        if !state.input(event) {
                            // UPDATED!
                            match event {
                                WindowEvent::CloseRequested
                                | WindowEvent::KeyboardInput {
                                    event:
                                        KeyEvent {
                                            state: ElementState::Pressed,
                                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                                            ..
                                        },
                                    ..
                                } => control_flow.exit(),
                                WindowEvent::Resized(physical_size) => {
                                    state.resize(*physical_size);
                                }
                                // WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                //     // new_inner_size is &&mut so w have to dereference it twice
                                //     state.resize(**new_inner_size);
                                // }
                                WindowEvent::RedrawRequested
                                    if window_id == state.window().id() =>
                                {
                                    state.update();
                                    match state.render() {
                                        Ok(_) => {}
                                        // Reconfigure the surface if it's lost or outdated
                                        Err(
                                            wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated,
                                        ) => state.resize(state.size()),
                                        // The system is out of memory, we should probably quit
                                        Err(wgpu::SurfaceError::OutOfMemory) => control_flow.exit(),

                                        Err(wgpu::SurfaceError::Timeout) => {
                                            log::warn!("Surface timeout")
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    // ... at the end of the WindowEvent block
                    Event::AboutToWait => {
                        // RedrawRequested will only trigger once unless we manually
                        // request it.
                        state.window().request_redraw();
                    }
                    _ => {}
                }
            })
            .unwrap();
    }
}
