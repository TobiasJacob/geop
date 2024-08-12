#[cfg(test)]
mod tests {
    use geop_geometry::{
        curves::{curve::Curve, CurveLike},
        points::point::Point,
    };
    use geop_topology::{
        primitive_objects::edges::{
            circle::primitive_circle, ellipsis::primitive_ellipsis, line::primitive_infinite_line,
        },
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
                Point::new(0.0, -3.0, 0.0),
                std::path::Path::new("src/generated_images/geometry/primitive_line.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_primitive_circle(#[future] renderer: Box<HeadlessRenderer>) {
        let edge = primitive_circle(Point::new_zero(), Point::new_unit_y(), 1.0);
        let scene = Scene::new(vec![], vec![], vec![(edge, Color::white())], vec![]);
        renderer
            .await
            .render_to_file(
                &scene,
                false,
                false,
                Point::new(0.0, -3.0, 0.0),
                std::path::Path::new("src/generated_images/geometry/primitive_circle.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_ellipsis(#[future] renderer: Box<HeadlessRenderer>) {
        let ellipsis1 = primitive_ellipsis(
            Point::new_zero(),
            Point::new_unit_y(),
            Point::new_unit_x() * 1.5,
            Point::new_unit_z() * 0.5,
        );
        let mut scene = Scene::new(
            vec![],
            vec![],
            vec![(ellipsis1.clone(), Color::white())],
            vec![],
        );
        match ellipsis1.curve {
            Curve::Ellipsis(e) => {
                for p in e.get_extremal_points() {
                    assert!(e.on_curve(p));
                    scene.points.push((p, Color::red()));
                }
            }
            _ => {}
        }
        renderer
            .await
            .render_to_file(
                &scene,
                false,
                false,
                Point::new(0.0, -3.0, 0.0),
                std::path::Path::new("src/generated_images/geometry/primitive_ellipsis.png"),
            )
            .await;
    }
}
