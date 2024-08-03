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
    async fn test_surface_log_operation_unit_x(#[future] renderer: Box<HeadlessRenderer>) {
        let face = primitive_sphere(Point::new_zero(), 0.3);

        let mut scene = Scene::new(vec![], vec![], vec![], vec![]);

        for p in face.surface.point_grid(1.0) {
            scene.points.push((p, Color::gray()));
            assert!(face.surface.on_surface(p));
            if let Some(p) = face.surface.log(Point::new(0.0, 0.0, -0.3), p) {
                scene.points.push((p, Color::red()));
            }
        }

        renderer
            .await
            .render_to_file(
                &scene,
                false,
                false,
                Point::new(0.0, -2.0, 1.0),
                std::path::Path::new("src/generated_images/geometry/log_unit_x.png"),
            )
            .await;
    }
}
