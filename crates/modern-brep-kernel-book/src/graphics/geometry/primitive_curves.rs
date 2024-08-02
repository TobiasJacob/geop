#[cfg(test)]
mod tests {
    use geop_geometry::points::point::Point;
    use geop_topology::{
        primitive_objects::{edges::line::primitive_line, volumes::cube::primitive_cube},
        topology::{edge::Edge, scene::Scene},
    };
    use geop_wgpu::headless_renderer::HeadlessRenderer;
    use rstest::rstest;

    use crate::tests::renderer;

    #[rstest]
    async fn test_primitive_line(#[future] renderer: Box<HeadlessRenderer>) {
        let edge = primitive_line(Point::new(-1.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0));
        let scene = Scene::new(vec![], vec![], vec![edge], vec![]);
        renderer
            .await
            .render_to_file(
                &scene,
                false,
                std::path::Path::new("src/generated_images/geometry/primitive_line.png"),
            )
            .await;
    }
}
