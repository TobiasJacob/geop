#[cfg(test)]
mod tests {
    use geop_geometry::{points::point::Point, surfaces::surface::Surface};
    use geop_topology::{
        contains::face_point::{face_point_contains, FacePointContains},
        primitive_objects::{
            edges::circle::primitive_circle,
            faces::{rectangle::primitive_rectangle, sphere::primitive_sphere},
        },
        topology::{
            contour::Contour,
            scene::{Color, Scene},
        },
    };
    use geop_wgpu::headless_renderer::HeadlessRenderer;
    use rstest::rstest;

    use crate::tests::renderer;

    #[rstest]
    async fn test_face_contains_sphere(#[future] renderer: Box<HeadlessRenderer>) {
        let mut scene = Scene::new(vec![], vec![], vec![], vec![]);

        let mut face = primitive_sphere(Point::zero(), 1.0);
        face.boundary = Some(Contour::new(vec![primitive_circle(
            Point::zero(),
            -Point::new(0.5, 0.5, 0.5),
            1.0,
        )]));

        for p in face.surface.point_grid(4.0) {
            match face_point_contains(&face, p) {
                FacePointContains::Inside => scene.points.push((p, Color::green())),
                FacePointContains::OnEdge(_) => scene.points.push((p, Color::blue())),
                FacePointContains::OnPoint(_) => scene.points.push((p, Color::gray())),
                FacePointContains::Outside => scene.points.push((p, Color::red())),
                FacePointContains::NotOnSurface => scene.points.push((p, Color::black())),
            };
        }
        scene.faces.push((face, Color::white()));

        renderer
            .await
            .render_to_file(
                &scene,
                false,
                false,
                Point::new(0.0, -2.0, 1.0),
                std::path::Path::new("src/generated_images/topology/face_contains.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_face_contains_rectangle(#[future] renderer: Box<HeadlessRenderer>) {
        let mut scene = Scene::new(vec![], vec![], vec![], vec![]);

        let mut face = primitive_rectangle(Point::zero(), Point::unit_x(), Point::unit_y());
        face.holes.push(Contour::new(vec![primitive_circle(
            Point::zero(),
            Point::new(0.0, 0.0, -1.0),
            0.5,
        )]));

        let plane = match &*face.surface {
            Surface::Plane(p) => p,
            _ => panic!("Surface is not a plane"),
        };

        for e in face.all_edges() {
            scene.edges.push((e, Color::white()));
        }

        for p in plane.point_grid(30.0, 3.0) {
            match face_point_contains(&face, p) {
                FacePointContains::Inside => scene.points.push((p, Color::green())),
                FacePointContains::OnEdge(_) => scene.points.push((p, Color::blue())),
                FacePointContains::OnPoint(_) => scene.points.push((p, Color::gray())),
                FacePointContains::Outside => scene.points.push((p, Color::red())),
                FacePointContains::NotOnSurface => scene.points.push((p, Color::black())),
            };
        }

        scene.faces.push((face, Color::white()));
        renderer
            .await
            .render_to_file(
                &scene,
                false,
                false,
                Point::new(0.0, 0.1, 2.20),
                std::path::Path::new("src/generated_images/topology/face_contains_rectangle.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_face_contains_rectangle2(#[future] renderer: Box<HeadlessRenderer>) {
        let mut scene = Scene::new(vec![], vec![], vec![], vec![]);

        let mut face = primitive_rectangle(Point::zero(), Point::unit_x(), Point::unit_y());
        face.holes.push(
            primitive_rectangle(Point::zero(), Point::unit_x() / 2.0, Point::unit_y() / 2.0)
                .boundary
                .unwrap()
                .flip(),
        );

        let plane = match &*face.surface {
            Surface::Plane(p) => p,
            _ => panic!("Surface is not a plane"),
        };

        for e in face.all_edges() {
            scene.edges.push((e, Color::white()));
        }

        for p in plane.point_grid(30.0, 3.0) {
            match face_point_contains(&face, p) {
                FacePointContains::Inside => scene.points.push((p, Color::green())),
                FacePointContains::OnEdge(_) => scene.points.push((p, Color::blue())),
                FacePointContains::OnPoint(_) => scene.points.push((p, Color::gray())),
                FacePointContains::Outside => scene.points.push((p, Color::red())),
                FacePointContains::NotOnSurface => scene.points.push((p, Color::black())),
            };
        }

        scene.faces.push((face, Color::white()));
        renderer
            .await
            .render_to_file(
                &scene,
                false,
                false,
                Point::new(0.0, 0.1, 2.20),
                std::path::Path::new("src/generated_images/topology/face_contains_rectangle2.png"),
            )
            .await;
    }
}
