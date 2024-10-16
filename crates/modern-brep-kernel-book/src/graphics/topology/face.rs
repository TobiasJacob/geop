#[cfg(test)]
mod tests {

    use core::f64;
    use std::rc::Rc;

    use geop_geometry::{
        points::point::Point,
        surfaces::{plane::Plane, surface::Surface},
        transforms::Transform,
    };
    use geop_topology::{
        primitive_objects::{
            curves::rectangle::primitive_rectangle_curve,
            edges::{arc::primitive_arc, circle::primitive_circle, line::primitive_line},
            faces::{cylinder::primitive_cylinder, sphere::primitive_sphere},
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
        edges.push(primitive_arc(p4, p1, 1.6, -Point::unit_y()));

        let hole = primitive_circle(Point::new(0.0, 0.0, 0.2), Point::unit_y(), 0.3);

        let hole2 = primitive_rectangle_curve(
            Point::new(0.0, 0.0, -0.5),
            Point::unit_x() * 0.5,
            -Point::unit_z() * 0.1,
        );

        let face = Face::new(
            vec![Contour::new(edges), Contour::new(vec![hole]), hole2],
            Rc::new(Surface::Plane(Plane::new(
                Point::zero(),
                Point::unit_x(),
                Point::unit_z(),
            ))),
        );
        scene.faces.push((face, Color::light_gray()));
        scene
    }

    fn half_sphere_scene() -> Scene {
        let mut scene = Scene::new(vec![], vec![], vec![], vec![]);

        let mut sphere = primitive_sphere(Point::zero(), 1.0);
        let edge = primitive_circle(Point::zero(), -Point::new(0.5, 3.0, 0.5).normalize(), 1.0);
        sphere.boundaries.push(Contour::new(vec![edge.clone()]));

        scene.faces.push((sphere, Color::light_gray()));
        scene
    }

    fn cylinder_scene1() -> Scene {
        let mut scene = Scene::new(vec![], vec![], vec![], vec![]);
        let mut cylinder = primitive_cylinder(Point::zero(), Point::unit_z(), 1.0);

        cylinder.boundaries.push(Contour::new(vec![primitive_circle(
            Point::new(0.0, 0.0, -2.0),
            Point::unit_z(),
            1.0,
        )]));

        cylinder.boundaries.push(Contour::new(vec![primitive_circle(
            Point::new(0.0, 0.0, 2.0),
            -Point::unit_z(),
            1.0,
        )]));

        scene.faces.push((cylinder, Color::light_gray()));
        scene
    }

    fn cylinder_scene2() -> Scene {
        let mut scene = Scene::new(vec![], vec![], vec![], vec![]);
        let mut cylinder = primitive_cylinder(Point::zero(), Point::unit_z(), 1.0);

        cylinder = cylinder.flip();
        cylinder.boundaries.push(Contour::new(vec![primitive_circle(
            Point::new(0.0, 0.0, -2.0),
            -Point::unit_z(),
            1.0,
        )]));

        cylinder.boundaries.push(Contour::new(vec![primitive_circle(
            Point::new(0.0, 0.0, 2.0),
            Point::unit_z(),
            1.0,
        )]));
        cylinder = cylinder.transform(
            Transform::from_translation(Point::new(0.3, -0.45, 0.12))
                * Transform::from_euler_angles(-90.0 / 180.0 * f64::consts::PI, 0.0, 0.0), // * Transform::from_scale(Point::new(0.7, 0.7, 0.7)),
        );

        // cylinder = cylinder.transform(Transform::from_scale(Point::new(0.7, 0.7, 0.7)));

        scene.faces.push((cylinder, Color::light_gray()));
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
                Point::new(4.0, -4.0, 0.0),
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
                Point::new(4.0, -4.0, 0.0),
                std::path::Path::new("src/generated_images/topology/face2wire.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_face3(#[future] renderer: Box<HeadlessRenderer>) {
        let scene = cylinder_scene1();
        let mut renderer = renderer.await;
        renderer
            .render_to_file(
                &scene,
                false,
                false,
                Point::new(3.0, -3.0, 3.0),
                std::path::Path::new("src/generated_images/topology/face3.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_face3wire(#[future] renderer: Box<HeadlessRenderer>) {
        let scene = cylinder_scene1();
        let mut renderer = renderer.await;
        renderer
            .render_to_file(
                &scene,
                false,
                true,
                Point::new(3.0, -3.0, 3.0),
                std::path::Path::new("src/generated_images/topology/face3wire.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_face4(#[future] renderer: Box<HeadlessRenderer>) {
        let scene = cylinder_scene2();
        let mut renderer = renderer.await;
        renderer
            .render_to_file(
                &scene,
                false,
                false,
                Point::new(3.0, -3.0, 3.0),
                std::path::Path::new("src/generated_images/topology/face4.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_face4wire(#[future] renderer: Box<HeadlessRenderer>) {
        let scene = cylinder_scene2();
        let mut renderer = renderer.await;
        renderer
            .render_to_file(
                &scene,
                false,
                true,
                Point::new(3.0, -3.0, 3.0),
                std::path::Path::new("src/generated_images/topology/face4wire.png"),
            )
            .await;
    }
}
