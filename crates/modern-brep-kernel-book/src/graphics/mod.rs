pub mod booleans;
pub mod geometry;
pub mod topology;

#[cfg(test)]
mod tests {
    use geop_geometry::{efloat::EFloat64, point::Point};
    use geop_topology::{
        primitive_objects::volumes::cube::primitive_cube,
        topology::scene::{Color, Scene},
    };
    use geop_wgpu::headless_renderer::HeadlessRenderer;
    use rstest::rstest;

    use crate::tests::renderer;

    #[rstest]
    async fn test_headless_renderer_light(#[future] renderer: Box<HeadlessRenderer>) {
        let volume = primitive_cube(EFloat64::new(1.0), EFloat64::new(1.0), EFloat64::new(1.0));
        let scene = Scene::new(vec![(volume, Color::white())], vec![], vec![], vec![]);
        renderer
            .await
            .render_to_file(
                &scene,
                false,
                false,
                Point::from_f64(0.0, -2.0, 1.0),
                std::path::Path::new("src/generated_images/test_light.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_headless_renderer_dark(#[future] renderer: Box<HeadlessRenderer>) {
        let volume = primitive_cube(EFloat64::new(1.0), EFloat64::new(1.0), EFloat64::new(1.0));
        let scene = Scene::new(vec![(volume, Color::white())], vec![], vec![], vec![]);
        renderer
            .await
            .render_to_file(
                &scene,
                true,
                false,
                Point::from_f64(0.0, -2.0, 1.0),
                std::path::Path::new("src/generated_images/test_dark.png"),
            )
            .await;
    }
}
