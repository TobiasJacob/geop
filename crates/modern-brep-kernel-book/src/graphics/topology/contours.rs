#[cfg(test)]
mod tests {
    use geop_geometry::points::point::Point;
    use geop_topology::{
        primitive_objects::edges::{arc::primitive_arc, line::primitive_line},
        topology::scene::{Color, Scene},
    };
    use geop_wgpu::headless_renderer::HeadlessRenderer;
    use rstest::rstest;

    use crate::tests::renderer;

    #[rstest]
    async fn test_contour(#[future] renderer: Box<HeadlessRenderer>) {
        let p1 = Point::new(-1.0, 0.0, 1.0);
        let p2 = Point::new(-1.0, 0.0, -1.0);
        let p3 = Point::new(1.0, 0.0, -1.0);
        let p4 = Point::new(1.0, 0.0, 1.0);

        let mut scene = Scene::new(vec![], vec![], vec![], vec![]);

        for p in &[p1, p2, p3, p4] {
            scene.points.push((p.clone(), Color::red()));
        }

        for (p1, p2) in &[(p1, p2), (p2, p3), (p3, p4)] {
            scene.edges.push((primitive_line(*p1, *p2), Color::white()));
        }

        scene
            .edges
            .push((primitive_arc(p4, p1, 1.6, -Point::unit_y()), Color::white()));

        renderer
            .await
            .render_to_file(
                &scene,
                false,
                false,
                Point::new(0.0, -4.0, 0.0),
                std::path::Path::new("src/generated_images/topology/contours.png"),
            )
            .await;
    }
}
