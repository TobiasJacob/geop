#[cfg(test)]
mod tests {
    use geop_geometry::points::point::Point;
    use geop_topology::{
        primitive_objects::edges::{circle::primitive_circle, line::primitive_infinite_line},
        topology::scene::{Color, Scene},
    };
    use geop_wgpu::headless_renderer::HeadlessRenderer;
    use rstest::rstest;

    use crate::tests::renderer;

    #[rstest]
    async fn test_primitive_line(#[future] renderer: Box<HeadlessRenderer>) {
        let edge = primitive_infinite_line(Point::new(-1.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0));
        let scene = Scene::new(vec![], vec![], vec![(edge, Color::white())], vec![]);
        renderer
            .await
            .render_to_file(
                &scene,
                false,
                false,
                Point::new(0.0, -2.0, 1.0),
                std::path::Path::new("src/generated_images/geometry/primitive_line.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_primitive_circle(#[future] renderer: Box<HeadlessRenderer>) {
        let edge = primitive_circle(Point::new_zero(), Point::new_unit_z(), 1.0);
        let scene = Scene::new(vec![], vec![], vec![(edge, Color::white())], vec![]);
        renderer
            .await
            .render_to_file(
                &scene,
                false,
                false,
                Point::new(0.0, -2.0, 1.0),
                std::path::Path::new("src/generated_images/geometry/primitive_circle.png"),
            )
            .await;
    }
}
