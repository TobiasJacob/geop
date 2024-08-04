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
            faces::sphere::primitive_sphere,
        },
        topology::{
            contour::Contour,
            face::Face,
            scene::{Color, Scene},
        },
    };

    fn generate_scene() -> Scene {
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
        scene
    }

    fn half_sphere_scene() -> Scene {
        let mut scene = Scene::new(vec![], vec![], vec![], vec![]);

        let mut sphere = primitive_sphere(Point::new_zero(), 1.0);
        sphere.boundary = Some(Contour::new(vec![primitive_circle(
            Point::new_zero(),
            -Point::new(0.5, 3.0, 0.5).normalize(),
            1.0,
        )]));

        scene.faces.push((sphere, Color::white()));
        scene
    }
    use geop_wgpu::headless_renderer::HeadlessRenderer;
    use rstest::rstest;

    use crate::tests::renderer;

    #[rstest]
    async fn test_face1(#[future] renderer: Box<HeadlessRenderer>) {
        let scene = generate_scene();
        let mut renderer = renderer.await;
        renderer
            .render_to_file(
                &scene,
                false,
                false,
                Point::new(0.0, -4.0, 0.0),
                std::path::Path::new("src/generated_images/topology/face1.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_face1wire(#[future] renderer: Box<HeadlessRenderer>) {
        let scene = generate_scene();
        let mut renderer = renderer.await;
        renderer
            .render_to_file(
                &scene,
                false,
                true,
                Point::new(0.0, -4.0, 0.0),
                std::path::Path::new("src/generated_images/topology/face1wire.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_face2(#[future] renderer: Box<HeadlessRenderer>) {
        let scene = half_sphere_scene();
        let mut renderer = renderer.await;
        renderer
            .render_to_file(
                &scene,
                false,
                false,
                Point::new(0.0, -4.0, 0.0),
                std::path::Path::new("src/generated_images/topology/face2.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_face2wire(#[future] renderer: Box<HeadlessRenderer>) {
        let scene = half_sphere_scene();
        let mut renderer = renderer.await;
        renderer
            .render_to_file(
                &scene,
                false,
                true,
                Point::new(0.0, -4.0, 0.0),
                std::path::Path::new("src/generated_images/topology/face2wire.png"),
            )
            .await;
    }
}
