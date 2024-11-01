#[cfg(test)]
mod tests {
    use geop_geometry::{efloat::EFloat64, point::Point};
    use geop_topology::{
        primitive_objects::edges::{
            circle::primitive_circle, ellipse::primitive_ellipse, helix::primitive_helix,
            line::primitive_infinite_line,
        },
        topology::scene::{Color, Scene},
    };
    use geop_wgpu::headless_renderer::HeadlessRenderer;
    use rstest::rstest;

    use crate::tests::renderer;

    #[rstest]
    async fn test_primitive_line(#[future] renderer: Box<HeadlessRenderer>) {
        let edge = primitive_infinite_line(
            Point::from_f64(-1.0, 0.0, 0.0),
            Point::from_f64(1.0, 0.0, 0.0),
        );
        let scene = Scene::new(vec![], vec![], vec![(edge, Color::white())], vec![]);
        renderer
            .await
            .render_to_file(
                &scene,
                false,
                false,
                Point::from_f64(0.0, -3.0, 0.0),
                std::path::Path::new("src/generated_images/geometry/primitive_line.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_primitive_circle(#[future] renderer: Box<HeadlessRenderer>) {
        let edge = primitive_circle(Point::zero(), Point::unit_y(), EFloat64::new(1.0));
        let scene = Scene::new(vec![], vec![], vec![(edge, Color::white())], vec![]);
        renderer
            .await
            .render_to_file(
                &scene,
                false,
                false,
                Point::from_f64(0.0, -3.0, 0.0),
                std::path::Path::new("src/generated_images/geometry/primitive_circle.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_ellipse(#[future] renderer: Box<HeadlessRenderer>) {
        let ellipse1 = primitive_ellipse(
            Point::zero(),
            Point::unit_y(),
            Point::unit_x() * EFloat64::new(1.5),
            Point::unit_z() * EFloat64::new(0.5),
        );
        let scene = Scene::new(
            vec![],
            vec![],
            vec![(ellipse1.clone(), Color::white())],
            vec![],
        );
        renderer
            .await
            .render_to_file(
                &scene,
                false,
                false,
                Point::from_f64(0.0, -3.0, 0.0),
                std::path::Path::new("src/generated_images/geometry/primitive_ellipse.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_helix(#[future] renderer: Box<HeadlessRenderer>) {
        let helix = primitive_helix(Point::zero(), Point::unit_z(), Point::unit_x(), true);
        let scene = Scene::new(
            vec![],
            vec![],
            vec![(helix.clone(), Color::white())],
            vec![],
        );
        renderer
            .await
            .render_to_file(
                &scene,
                false,
                false,
                Point::from_f64(0.0, -3.0, 0.0),
                std::path::Path::new("src/generated_images/geometry/primitive_helix.png"),
            )
            .await;
    }
}
