pub mod camera_pipeline;
pub mod headless_renderer;
pub mod pipeline_manager;
pub mod render_pipeline_edge;
pub mod render_pipeline_triangle;
pub mod render_pipeline_vertex;
pub mod window;
pub mod window_state;

#[cfg(test)]
mod tests {
    use super::*;
    use geop_topology::{
        primitive_objects::volumes::cube::primitive_cube,
        topology::scene::{Color, Scene},
    };
    use headless_renderer::{tests::renderer, HeadlessRenderer};
    use rstest::rstest;

    #[rstest]
    async fn test_headless_renderer_light(#[future] renderer: Box<HeadlessRenderer>) {
        let volume = primitive_cube(1.0, 1.0, 1.0);
        let scene = Scene::new(vec![(volume, Color::white())], vec![], vec![], vec![]);
        renderer
            .await
            .render_to_file(&scene, false, std::path::Path::new("test_light.png"))
            .await;
    }

    #[rstest]
    async fn test_headless_renderer_dark(#[future] renderer: Box<HeadlessRenderer>) {
        let volume = primitive_cube(1.0, 1.0, 1.0);
        let scene = Scene::new(vec![(volume, Color::white())], vec![], vec![], vec![]);
        renderer
            .await
            .render_to_file(&scene, true, std::path::Path::new("test_dark.png"))
            .await;
    }
}
