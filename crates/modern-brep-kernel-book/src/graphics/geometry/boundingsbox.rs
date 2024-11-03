#[cfg(test)]
mod tests {
    use geop_geometry::{curves::CurveLike, efloat::EFloat64, point::Point};
    use geop_rasterize::boundingbox::rasterize_boundingbox_into_edges;
    use geop_topology::{
        primitive_objects::edges::ellipse::primitive_ellipse,
        topology::scene::{Color, Scene},
    };
    use geop_wgpu::headless_renderer::HeadlessRenderer;
    use rstest::rstest;

    use crate::tests::renderer;

    #[rstest]
    async fn test_ellipse_bb(#[future] renderer: Box<HeadlessRenderer>) {
        let ellipse1 = primitive_ellipse(
            Point::zero(),
            Point::unit_y(),
            Point::unit_x() * EFloat64::from(1.5),
            Point::unit_z() * EFloat64::from(0.5),
        );

        let p1 = ellipse1.curve.project(Point::from_f64(0.1, 0.1, 0.1));
        let p2 = ellipse1.curve.project(Point::from_f64(-0.1, 0.1, 0.1));
        let bounding_box = ellipse1.curve.get_bounding_box(Some(p1), Some(p2)).unwrap();

        let mut scene = Scene::new(
            vec![],
            vec![],
            vec![(ellipse1.clone(), Color::white())],
            vec![(p1, Color::blue()), (p2, Color::blue())],
        );
        scene.edges.extend(
            rasterize_boundingbox_into_edges(bounding_box)
                .iter()
                .map(|e| (e.clone(), Color::red())),
        );

        renderer
            .await
            .render_to_file(
                &scene,
                false,
                false,
                Point::from_f64(0.0, -3.0, 0.0),
                std::path::Path::new("src/generated_images/geometry/primitive_ellipse_bbox.png"),
            )
            .await;
    }
}
