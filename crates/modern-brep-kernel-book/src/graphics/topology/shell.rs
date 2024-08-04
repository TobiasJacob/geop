#[cfg(test)]
mod tests {
    use crate::tests::renderer;
    use geop_geometry::points::point::Point;
    use geop_topology::{
        primitive_objects::volumes::cube::primitive_cube,
        topology::scene::{Color, Scene},
    };
    use geop_wgpu::headless_renderer::HeadlessRenderer;
    use rstest::rstest;

    #[rstest]
    async fn test_shell1(#[future] renderer: Box<HeadlessRenderer>) {
        let mut scene = Scene::new(vec![], vec![], vec![], vec![]);
        scene
            .volumes
            .push((primitive_cube(1.0, 1.0, 1.0), Color::white()));
        let mut renderer = renderer.await;
        renderer
            .render_to_file(
                &scene,
                false,
                false,
                Point::new(2.0, -2.0, 2.0),
                std::path::Path::new("src/generated_images/topology/shell1.png"),
            )
            .await;
    }
}
