#[cfg(test)]
mod tests {

    use geop_geometry::{efloat::EFloat64, point::Point};
    use geop_topology::{
        primitive_objects::edges::{
            arc::primitive_arc, circle::primitive_circle, line::primitive_line,
        },
        topology::scene::{Color, Scene},
    };
    use geop_wgpu::headless_renderer::HeadlessRenderer;
    use rstest::rstest;

    use crate::tests::renderer;

    #[rstest]
    async fn test_edges(#[future] renderer: Box<HeadlessRenderer>) {
        let edge = primitive_line(
            Point::from_f64(-1.0, 0.0, 1.0),
            Point::from_f64(1.0, 0.0, 1.0),
        )
        .unwrap();
        let edge2 = primitive_arc(
            Point::from_f64(1.0, 0.0, 0.0),
            Point::from_f64(-1.0, 0.0, 0.0),
            EFloat64::from(3.0),
            -Point::unit_y(),
        );
        let edge3 = primitive_circle(
            Point::from_f64(0.0, 0.0, -1.0),
            -Point::unit_y(),
            EFloat64::from(0.6),
        );
        let mut scene = Scene::new(
            vec![],
            vec![],
            vec![
                (edge, Color::white()),
                (edge2, Color::white()),
                (edge3, Color::white()),
            ],
            vec![],
        );
        for (e, _) in scene.edges.iter() {
            match (e.start, e.end) {
                (None, None) => {}
                (None, Some(_)) => panic!(),
                (Some(_), None) => panic!(),
                (Some(p1), Some(p2)) => {
                    scene.points.push((p1, Color::gray()));
                    scene.points.push((p2, Color::gray()));
                }
            }
        }
        renderer
            .await
            .render_to_file(
                &scene,
                false,
                false,
                Point::from_f64(0.0, -3.0, 0.0),
                std::path::Path::new("src/generated_images/topology/edges.png"),
            )
            .await;
    }
}
