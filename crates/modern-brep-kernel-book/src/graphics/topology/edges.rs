#[cfg(test)]
mod tests {
    use geop_geometry::points::point::Point;
    use geop_topology::{
        primitive_objects::edges::arc::primitive_arc,
        topology::scene::{Color, Scene},
    };
    use geop_wgpu::headless_renderer::HeadlessRenderer;
    use rstest::rstest;

    use crate::tests::renderer;

    #[rstest]
    async fn test_arc(#[future] renderer: Box<HeadlessRenderer>) {
        let edge = primitive_arc(
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            0.6,
            Point::new_unit_z(),
        );
        let edge2 = primitive_arc(
            Point::new(1.0, 0.0, 0.0),
            Point::new(-1.0, 1.0, 0.0),
            1.6,
            Point::new_unit_z(),
        );
        let edge3 = primitive_arc(
            Point::new(-1.0, 1.0, 0.0),
            Point::new(0.0, 0.0, 0.0),
            1.6,
            Point::new_unit_z(),
        );
        let scene = Scene::new(
            vec![],
            vec![],
            vec![
                (edge, Color::white()),
                (edge2, Color::white()),
                (edge3, Color::white()),
            ],
            vec![
                (Point::new(0.0, 0.0, 0.0), Color::green()),
                (Point::new(1.0, 0.0, 0.0), Color::green()),
                (Point::new(-1.0, 1.0, 0.0), Color::green()),
            ],
        );
        renderer
            .await
            .render_to_file(
                &scene,
                false,
                false,
                Point::new(0.0, -2.0, 1.0),
                std::path::Path::new("src/generated_images/topology/arc.png"),
            )
            .await;
    }
}
