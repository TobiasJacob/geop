#[cfg(test)]
mod tests {
    use geop_geometry::points::point::Point;
    use geop_topology::{
        primitive_objects::faces::sphere::primitive_sphere,
        topology::scene::{Color, Scene},
    };
    use geop_wgpu::headless_renderer::HeadlessRenderer;
    use rstest::rstest;

    use crate::tests::renderer;

    #[rstest]
    async fn test_primitive_sphere(#[future] renderer: Box<HeadlessRenderer>) {
        let face = primitive_sphere(Point::new_zero(), 1.0);
        let scene = Scene::new(vec![], vec![(face, Color::white())], vec![], vec![]);
        renderer
            .await
            .render_to_file(
                &scene,
                false,
                std::path::Path::new("src/generated_images/geometry/primitive_sphere.png"),
            )
            .await;
    }
}
