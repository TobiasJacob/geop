#[cfg(test)]
mod tests {
    use geop_booleans::{
        remesh::volume::{volume_split, volume_split_edges, VolumeSplit},
        split_if_necessary::edge_split_face::split_faces_by_edges_if_necessary,
    };
    use geop_geometry::{point::Point, transforms::Transform};
    use geop_topology::{
        primitive_objects::volumes::cube::primitive_cube,
        topology::{
            scene::{Color, Scene},
            volume::Volume,
        },
    };

    fn generate_secene_1() -> (Volume, Volume) {
        let v1 = primitive_cube(2.0, 1.0, 1.0)
            .transform(Transform::from_translation(Point::new(-1.0, 0.0, 0.0)));
        let v2 = primitive_cube(2.0, 1.0, 0.5)
            .transform(Transform::from_translation(Point::new(1.0, 0.0, 0.0)));

        (v1, v2)
    }

    fn generate_secene_2() -> (Volume, Volume) {
        let v1 = primitive_cube(2.0, 1.0, 1.0)
            .transform(Transform::from_translation(Point::new(-1.0, 0.0, 0.0)));
        let v2 = primitive_cube(2.0, 0.5, 0.5)
            .transform(Transform::from_translation(Point::new(1.0, 0.0, 0.0)));

        (v1, v2)
    }

    fn _generate_secene_3() -> (Volume, Volume) {
        let v1 = primitive_cube(2.0, 1.0, 1.0)
            .transform(Transform::from_translation(Point::new(-1.0, 0.0, 0.0)));
        let v2 = primitive_cube(1.0, 0.5, 0.5)
            .transform(Transform::from_translation(Point::new(0.5, 0.0, 0.0)));

        (v1, v2)
    }

    use geop_wgpu::headless_renderer::HeadlessRenderer;
    use rstest::rstest;

    use crate::tests::renderer;
    #[rstest]
    async fn test_volume_split_edges(#[future] renderer: Box<HeadlessRenderer>) {
        let (volume1, volume2) = generate_secene_1();
        let split_edges = volume_split_edges(&volume1, &volume2);
        assert!(split_edges.len() == 4);

        let mut scene = Scene::new(vec![], vec![], vec![], vec![]);
        scene
            .volumes
            .push((volume1, Color::new(1.0, 1.0, 1.0, 0.4)));
        scene
            .volumes
            .push((volume2, Color::new(1.0, 1.0, 1.0, 0.4)));

        for e in split_edges {
            let e = e.transform(Transform::from_translation(e.get_midpoint() * 0.01));
            scene.edges.push((e, Color::red()));
        }

        let mut renderer = renderer.await;
        renderer
            .render_to_file(
                &scene,
                false,
                true,
                Point::new(2.0, -4.0, 2.0),
                std::path::Path::new("src/generated_images/booleans/volume_split_edges.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_face_subdivision(#[future] renderer: Box<HeadlessRenderer>) {
        let (volume1, volume2) = generate_secene_1();
        let split_edges = volume_split_edges(&volume1, &volume2);
        assert!(split_edges.len() == 4);

        let mut scene = Scene::new(vec![], vec![], vec![], vec![]);

        let faces = split_faces_by_edges_if_necessary(volume1.all_faces(), &split_edges);
        for f in faces {
            let mut midpoint = Point::zero();
            for e in f.boundaries[0].clone().edges.iter() {
                midpoint = midpoint + e.get_midpoint();
            }
            midpoint = (midpoint / f.boundaries[0].clone().edges.len() as f64).unwrap();
            let f = f.transform(Transform::from_translation(midpoint * 0.2));
            scene.faces.push((f, Color::white()));
        }

        let faces = split_faces_by_edges_if_necessary(volume2.all_faces(), &split_edges);
        for f in faces {
            let mut midpoint = Point::zero();
            for e in f.boundaries[0].clone().edges.iter() {
                midpoint = midpoint + e.get_midpoint();
            }
            midpoint = (midpoint / f.boundaries[0].clone().edges.len() as f64).unwrap();
            midpoint = midpoint + Point::new(0.5, 0.0, 0.0);
            let f = f.transform(Transform::from_translation(midpoint * 0.2));
            scene.faces.push((f, Color::white()));
        }

        let mut renderer = renderer.await;
        renderer
            .render_to_file(
                &scene,
                false,
                false,
                Point::new(2.0, -4.0, 2.0),
                std::path::Path::new("src/generated_images/booleans/face_subdivions.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_face_classification(#[future] renderer: Box<HeadlessRenderer>) {
        let (volume1, volume2) = generate_secene_1();
        let split_edges = volume_split_edges(&volume1, &volume2);
        assert!(split_edges.len() == 4);

        let mut scene = Scene::new(vec![], vec![], vec![], vec![]);

        let splits = volume_split(&volume1, &volume2);
        for split in splits {
            let f = split.face();
            let mut midpoint = Point::zero();
            for e in f.boundaries[0].clone().edges.iter() {
                midpoint = midpoint + e.get_midpoint();
            }
            midpoint = (midpoint / f.boundaries[0].clone().edges.len() as f64).unwrap();
            let f = f.transform(Transform::from_translation(midpoint * 0.2));

            let color = match split {
                VolumeSplit::AinB(_) => Color::ten_different_colors(0),
                VolumeSplit::AonBSameSide(_) => Color::ten_different_colors(1),
                VolumeSplit::AonBOpSide(_) => Color::ten_different_colors(2),
                VolumeSplit::AoutB(_) => Color::ten_different_colors(3),
                VolumeSplit::BinA(_) => Color::ten_different_colors(4),
                VolumeSplit::BonASameSide(_) => Color::ten_different_colors(5),
                VolumeSplit::BonAOpSide(_) => Color::ten_different_colors(6),
                VolumeSplit::BoutA(_) => Color::ten_different_colors(7),
            };

            scene.faces.push((f, color));
        }

        let mut renderer = renderer.await;
        renderer
            .render_to_file(
                &scene,
                false,
                false,
                Point::new(2.0, -4.0, 2.0),
                std::path::Path::new("src/generated_images/booleans/face_classification.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_union_splits(#[future] renderer: Box<HeadlessRenderer>) {
        let (volume1, volume2) = generate_secene_1();
        let split_edges = volume_split_edges(&volume1, &volume2);
        assert!(split_edges.len() == 4);

        let mut scene = Scene::new(vec![], vec![], vec![], vec![]);

        let splits = volume_split(&volume1, &volume2);
        let splits = splits.iter().filter(|split| match split {
            VolumeSplit::AinB(_) => false,
            VolumeSplit::AonBSameSide(_) => true,
            VolumeSplit::AonBOpSide(_) => false,
            VolumeSplit::AoutB(_) => true,
            VolumeSplit::BinA(_) => false,
            VolumeSplit::BonASameSide(_) => false,
            VolumeSplit::BonAOpSide(_) => false,
            VolumeSplit::BoutA(_) => true,
        });
        for split in splits {
            let f = split.face();
            let mut midpoint = Point::zero();
            for e in f.boundaries[0].clone().edges.iter() {
                midpoint = midpoint + e.get_midpoint();
            }
            midpoint = (midpoint / f.boundaries[0].clone().edges.len() as f64).unwrap();
            let f = f.transform(Transform::from_translation(midpoint * 0.2));

            let color = match split {
                VolumeSplit::AinB(_) => Color::ten_different_colors(0),
                VolumeSplit::AonBSameSide(_) => Color::ten_different_colors(1),
                VolumeSplit::AonBOpSide(_) => Color::ten_different_colors(2),
                VolumeSplit::AoutB(_) => Color::ten_different_colors(3),
                VolumeSplit::BinA(_) => Color::ten_different_colors(4),
                VolumeSplit::BonASameSide(_) => Color::ten_different_colors(5),
                VolumeSplit::BonAOpSide(_) => Color::ten_different_colors(6),
                VolumeSplit::BoutA(_) => Color::ten_different_colors(7),
            };

            scene.faces.push((f, color));
        }

        let mut renderer = renderer.await;
        renderer
            .render_to_file(
                &scene,
                false,
                false,
                Point::new(2.0, -4.0, 2.0),
                std::path::Path::new("src/generated_images/booleans/volume_union_splits.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_union_splits2(#[future] renderer: Box<HeadlessRenderer>) {
        let (volume1, volume2) = generate_secene_2();
        let split_edges = volume_split_edges(&volume1, &volume2);
        assert!(split_edges.len() == 4);

        let mut scene = Scene::new(vec![], vec![], vec![], vec![]);

        let splits = volume_split(&volume1, &volume2);
        // let splits = splits.iter().filter(|split| match split {
        //     VolumeSplit::AinB(_) => false,
        //     VolumeSplit::AonBSameSide(_) => true,
        //     VolumeSplit::AonBOpSide(_) => false,
        //     VolumeSplit::AoutB(_) => true,
        //     VolumeSplit::BinA(_) => false,
        //     VolumeSplit::BonASameSide(_) => false,
        //     VolumeSplit::BonAOpSide(_) => false,
        //     VolumeSplit::BoutA(_) => true,
        // });
        for split in splits {
            let f = split.face();
            let mut midpoint = Point::zero();
            for e in f.boundaries[0].clone().edges.iter() {
                midpoint = midpoint + e.get_midpoint();
            }
            midpoint = (midpoint / f.boundaries[0].clone().edges.len() as f64).unwrap();
            let f = f.transform(Transform::from_translation(midpoint * 0.2));

            let color = match split {
                VolumeSplit::AinB(_) => Color::ten_different_colors(0),
                VolumeSplit::AonBSameSide(_) => Color::ten_different_colors(1),
                VolumeSplit::AonBOpSide(_) => Color::ten_different_colors(2),
                VolumeSplit::AoutB(_) => Color::ten_different_colors(3),
                VolumeSplit::BinA(_) => Color::ten_different_colors(4),
                VolumeSplit::BonASameSide(_) => Color::ten_different_colors(5),
                VolumeSplit::BonAOpSide(_) => Color::ten_different_colors(6),
                VolumeSplit::BoutA(_) => Color::ten_different_colors(7),
            };

            scene.faces.push((f, color));
        }

        let mut renderer = renderer.await;
        renderer
            .render_to_file(
                &scene,
                false,
                false,
                Point::new(2.0, -4.0, 2.0),
                std::path::Path::new("src/generated_images/booleans/volume_union_splits2.png"),
            )
            .await;
    }
}
