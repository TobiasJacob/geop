#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use geop_geometry::{
        points::point::Point,
        surfaces::{plane::Plane, surface::Surface},
    };
    use geop_topology::{
        primitive_objects::{
            curves::rectangle::primitive_rectangle_curve,
            edges::{arc::primitive_arc, circle::primitive_circle, line::primitive_line},
        },
        topology::{
            contour::Contour,
            face::Face,
            scene::{Color, Scene},
        },
    };
    use geop_wgpu::headless_renderer::HeadlessRenderer;
    use rstest::rstest;

    use crate::tests::renderer;

    #[rstest]
    async fn test_face1(#[future] renderer: Box<HeadlessRenderer>) {
        let p1 = Point::new(-1.0, 0.0, 1.0);
        let p2 = Point::new(-1.0, 0.0, -1.0);
        let p3 = Point::new(1.0, 0.0, -1.0);
        let p4 = Point::new(1.0, 0.0, 1.0);

        let mut scene = Scene::new(vec![], vec![], vec![], vec![]);

        for p in &[p1, p2, p3, p4] {
            scene.points.push((p.clone(), Color::red()));
        }

        let mut edges = Vec::new();
        for (p1, p2) in &[(p1, p2), (p2, p3), (p3, p4)] {
            edges.push(primitive_line(*p1, *p2));
        }
        edges.push(primitive_arc(p4, p1, 1.6, -Point::new_unit_y()));

        let hole = primitive_circle(Point::new(0.0, 0.0, 0.2), Point::new_unit_y(), 0.3);

        let hole2 = primitive_rectangle_curve(
            Point::new(0.0, 0.0, -0.5),
            Point::new_unit_x() * 0.5,
            -Point::new_unit_z() * 0.1,
        );

        let face = Face::new(
            Some(Contour::new(edges)),
            vec![Contour::new(vec![hole]), hole2],
            Rc::new(Surface::Plane(Plane::new(
                Point::new_zero(),
                Point::new_unit_x(),
                Point::new_unit_z(),
            ))),
        );
        scene.faces.push((face, Color::white()));

        let mut renderer = renderer.await;
        renderer
            .render_to_file(
                &scene,
                false,
                true,
                Point::new(0.0, -4.0, 0.0),
                std::path::Path::new("src/generated_images/topology/face1.png"),
            )
            .await;
    }
}
