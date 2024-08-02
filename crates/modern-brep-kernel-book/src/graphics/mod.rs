pub mod geometry;

#[cfg(test)]
mod tests {
    use geop_topology::{primitive_objects::volumes::cube::primitive_cube, topology::scene::Scene};
    use geop_wgpu::headless_renderer::HeadlessRenderer;
    use rstest::rstest;

    use crate::tests::renderer;

    #[rstest]
    async fn test_headless_renderer_light(#[future] renderer: Box<HeadlessRenderer>) {
        let volume = primitive_cube(1.0, 1.0, 1.0);
        let scene = Scene::new(vec![volume], vec![], vec![], vec![]);
        renderer
            .await
            .render_to_file(
                &scene,
                false,
                std::path::Path::new("src/generated_images/test_light.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_headless_renderer_dark(#[future] renderer: Box<HeadlessRenderer>) {
        let volume = primitive_cube(1.0, 1.0, 1.0);
        let scene = Scene::new(vec![volume], vec![], vec![], vec![]);
        renderer
            .await
            .render_to_file(
                &scene,
                true,
                std::path::Path::new("src/generated_images/test_dark.png"),
            )
            .await;
    }
}
