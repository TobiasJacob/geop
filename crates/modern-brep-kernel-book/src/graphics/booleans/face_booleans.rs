#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use geop_booleans::{
        difference::face_face::face_face_difference,
        intersections::face_face::{face_face_intersection, FaceFaceIntersection},
        remesh::face::{face_remesh, face_split, face_split_points, FaceSplit},
        union::face::face_face_union,
    };
    use geop_geometry::{
        efloat::EFloat64,
        point::Point,
        surfaces::{plane::Plane, surface::Surface},
        transforms::Transform,
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

    fn generate_scene() -> (Face, Face) {
        let p1 = Point::from_f64(-1.0, 0.0, 1.0);
        let p2 = Point::from_f64(-1.0, 0.0, -1.0);
        let p3 = Point::from_f64(1.0, 0.0, -1.0);
        let p4 = Point::from_f64(1.0, 0.0, 1.0);

        let mut edges = Vec::new();
        for (p1, p2) in &[(p1, p2), (p2, p3), (p3, p4)] {
            edges.push(primitive_line(*p1, *p2).unwrap());
        }
        edges.push(primitive_arc(p4, p1, EFloat64::from(1.6), -Point::unit_y()));

        let hole = primitive_circle(
            Point::from_f64(0.0, 0.0, 0.2),
            Point::unit_y(),
            EFloat64::from(0.3),
        );

        let hole2 = primitive_rectangle_curve(
            Point::from_f64(0.0, 0.0, -0.5),
            Point::unit_x() * EFloat64::from(0.5),
            -Point::unit_z() * EFloat64::from(0.1),
        );

        let face1 = Face::new(
            vec![Contour::new(edges), Contour::new(vec![hole]), hole2],
            Rc::new(Surface::Plane(Plane::new(
                Point::zero(),
                Point::unit_x(),
                Point::unit_z(),
            ))),
        );

        let face2 = face1.transform(Transform::from_translation(Point::from_f64(0.2, 0.0, 0.0)));

        (face1, face2)
    }

    use geop_wgpu::headless_renderer::HeadlessRenderer;
    use rstest::rstest;

    use crate::tests::renderer;
    #[rstest]
    async fn test_face_boolean_split_points(#[future] renderer: Box<HeadlessRenderer>) {
        let (face1, face2) = generate_scene();
        let mut scene = Scene::new(vec![], vec![], vec![], vec![]);

        for edge in face1.all_edges() {
            scene.edges.push((edge.clone(), Color::white()));
        }
        for edge in face2.all_edges() {
            scene.edges.push((edge.clone(), Color::white()));
        }

        let split_points = face_split_points(&face1, &face2);
        for point in split_points {
            scene.points.push((point, Color::gray()));
        }

        let mut renderer = renderer.await;
        renderer
            .render_to_file(
                &scene,
                false,
                true,
                Point::from_f64(0.0, -4.0, 0.0),
                std::path::Path::new("src/generated_images/booleans/face_split_points.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_face_boolean_split(#[future] renderer: Box<HeadlessRenderer>) {
        let (face1, face2) = generate_scene();
        let mut scene = Scene::new(vec![], vec![], vec![], vec![]);

        let face_splits = face_split(&face1, &face2);
        for split in face_splits {
            let (edge, color) = match split {
                FaceSplit::AinB(edge) => (edge, Color::ten_different_colors(0)),
                FaceSplit::AonBSameSide(edge) => (edge, Color::ten_different_colors(1)),
                FaceSplit::AonBOpSide(edge) => (edge, Color::ten_different_colors(2)),
                FaceSplit::AoutB(edge) => (edge, Color::ten_different_colors(3)),
                FaceSplit::BinA(edge) => (edge, Color::ten_different_colors(4)),
                FaceSplit::BonASameSide(edge) => (edge, Color::ten_different_colors(5)),
                FaceSplit::BonAOpSide(edge) => (edge, Color::ten_different_colors(6)),
                FaceSplit::BoutA(edge) => (edge, Color::ten_different_colors(7)),
            };
            let midpoint = edge.get_midpoint();
            let edge = edge.transform(Transform::from_translation(midpoint * EFloat64::from(0.1)));
            scene.edges.push((edge, color));
        }

        let mut renderer = renderer.await;
        renderer
            .render_to_file(
                &scene,
                false,
                true,
                Point::from_f64(0.0, -4.0, 0.0),
                std::path::Path::new("src/generated_images/booleans/face_splits.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_face_boolean_remesh(#[future] renderer: Box<HeadlessRenderer>) {
        let (face1, face2) = generate_scene();
        let mut scene = Scene::new(vec![], vec![], vec![], vec![]);

        let edges = face_split(&face1, &face2)
            .drain(..)
            .filter(|mode| match mode {
                FaceSplit::AinB(_) => true,
                FaceSplit::AonBSameSide(_) => true,
                FaceSplit::AonBOpSide(_) => false,
                FaceSplit::AoutB(_) => false,
                FaceSplit::BinA(_) => true,
                FaceSplit::BonASameSide(_) => false,
                FaceSplit::BonAOpSide(_) => false,
                FaceSplit::BoutA(_) => false,
            })
            .collect::<Vec<FaceSplit>>();

        let contours = face_remesh(edges);
        for contour in contours {
            let contour = contour.transform(Transform::from_translation(Point::from_f64(
                0.0, 0.001, 0.0,
            )));
            for edge in contour.edges {
                scene.edges.push((edge.clone(), Color::red()));
            }
        }

        let mut renderer = renderer.await;
        renderer
            .render_to_file(
                &scene,
                false,
                true,
                Point::from_f64(0.0, -4.0, 0.0),
                std::path::Path::new("src/generated_images/booleans/face_splits_remesh.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_face_boolean_intersection(#[future] renderer: Box<HeadlessRenderer>) {
        let (face1, face2) = generate_scene();
        let mut scene = Scene::new(vec![], vec![], vec![], vec![]);

        for edge in face1.all_edges() {
            scene.edges.push((edge.clone(), Color::white()));
        }
        for edge in face2.all_edges() {
            scene.edges.push((edge.clone(), Color::white()));
        }

        let intersection_face = face_face_intersection(&face1, &face2);
        match intersection_face {
            FaceFaceIntersection::Faces(faces) => {
                assert!(faces.len() == 1);
                for face in faces {
                    let face = face.transform(Transform::from_translation(Point::from_f64(
                        0.0, 0.001, 0.0,
                    )));
                    for edge in face.all_edges() {
                        scene.edges.push((edge.clone(), Color::red()));
                    }
                    scene.faces.push((face, Color::white()));
                }
            }
            _ => {
                panic!("No intersection found")
            }
        }

        let mut renderer = renderer.await;
        renderer
            .render_to_file(
                &scene,
                false,
                false,
                Point::from_f64(0.0, -4.0, 0.0),
                std::path::Path::new("src/generated_images/booleans/face_intersection.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_face_boolean_difference(#[future] renderer: Box<HeadlessRenderer>) {
        let (face1, face2) = generate_scene();
        let mut scene = Scene::new(vec![], vec![], vec![], vec![]);

        for edge in face1.all_edges() {
            scene.edges.push((edge.clone(), Color::white()));
        }
        for edge in face2.all_edges() {
            scene.edges.push((edge.clone(), Color::white()));
        }

        let faces = face_face_difference(&face1, &face2);
        for face in faces {
            let face = face.transform(Transform::from_translation(Point::from_f64(
                0.0, 0.001, 0.0,
            )));
            for edge in face.all_edges() {
                scene.edges.push((edge.clone(), Color::red()));
            }
            scene.faces.push((face, Color::white()));
        }

        let mut renderer = renderer.await;
        renderer
            .render_to_file(
                &scene,
                false,
                false,
                Point::from_f64(0.0, -4.0, 0.0),
                std::path::Path::new("src/generated_images/booleans/face_difference.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_face_boolean_union(#[future] renderer: Box<HeadlessRenderer>) {
        let (face1, face2) = generate_scene();
        let mut scene = Scene::new(vec![], vec![], vec![], vec![]);

        for edge in face1.all_edges() {
            scene.edges.push((edge.clone(), Color::white()));
        }
        for edge in face2.all_edges() {
            scene.edges.push((edge.clone(), Color::white()));
        }

        let faces = face_face_union(&face1, &face2);
        for face in faces {
            let face = face.transform(Transform::from_translation(Point::from_f64(
                0.0, 0.001, 0.0,
            )));
            for edge in face.all_edges() {
                scene.edges.push((edge.clone(), Color::red()));
            }
            scene.faces.push((face, Color::white()));
        }

        let mut renderer = renderer.await;
        renderer
            .render_to_file(
                &scene,
                false,
                false,
                Point::from_f64(0.0, -4.0, 0.0),
                std::path::Path::new("src/generated_images/booleans/face_union.png"),
            )
            .await;
    }
}
