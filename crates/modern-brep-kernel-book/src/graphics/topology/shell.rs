#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::tests::renderer;
    use geop_geometry::{
        point::Point,
        surfaces::{plane::Plane, surface::Surface},
    };
    use geop_topology::{
        operations::extrude::extrude,
        primitive_objects::{
            curves::rectangle::primitive_rectangle_curve,
            edges::{arc::primitive_arc, circle::primitive_circle, line::primitive_line},
            volumes::cube::primitive_cube,
        },
        topology::{
            contour::Contour,
            face::Face,
            scene::{Color, Scene},
        },
    };
    use geop_wgpu::headless_renderer::HeadlessRenderer;
    use rstest::rstest;

    #[rstest]
    async fn test_shell1(#[future] renderer: Box<HeadlessRenderer>) {
        let mut scene = Scene::new(vec![], vec![], vec![], vec![]);
        scene
            .volumes
            .push((primitive_cube(1.0, 1.0, 1.0), Color::light_gray()));
        let mut renderer = renderer.await;
        renderer
            .render_to_file(
                &scene,
                false,
                false,
                Point::new(2.0, -2.0, 2.0),
                std::path::Path::new("src/generated_images/topology/shell1.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_shell2(#[future] renderer: Box<HeadlessRenderer>) {
        let p1 = Point::new(-1.0, 0.0, 1.0);
        let p2 = Point::new(-1.0, 0.0, -1.0);
        let p3 = Point::new(1.0, 0.0, -1.0);
        let p4 = Point::new(1.0, 0.0, 1.0);

        let mut edges = Vec::new();
        for (p1, p2) in &[(p1, p2), (p2, p3), (p3, p4)] {
            edges.push(primitive_line(*p1, *p2));
        }
        edges.push(primitive_arc(p4, p1, 1.6, -Point::unit_y()));

        let hole = primitive_circle(Point::new(0.0, 0.0, 0.2), Point::unit_y(), 0.3);

        let hole2 = primitive_rectangle_curve(
            Point::new(0.0, 0.0, -0.5),
            Point::unit_x() * 0.5,
            -Point::unit_z() * 0.1,
        );

        let face1 = Face::new(
            vec![Contour::new(edges), Contour::new(vec![hole]), hole2],
            Rc::new(Surface::Plane(Plane::new(
                Point::zero(),
                Point::unit_x(),
                Point::unit_z(),
            ))),
        );

        let shell = extrude(face1, Point::unit_y());

        let mut scene = Scene::new(vec![], vec![], vec![], vec![]);
        scene.volumes.push((shell, Color::light_gray()));
        let mut renderer = renderer.await;
        renderer
            .render_to_file(
                &scene,
                false,
                false,
                Point::new(2.0, -2.0, 2.0),
                std::path::Path::new("src/generated_images/topology/shell2.png"),
            )
            .await;
    }
}
