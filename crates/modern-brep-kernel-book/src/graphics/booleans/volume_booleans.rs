#[cfg(test)]
mod tests {
    use geop_booleans::{
        remesh::{face, volume::volume_split_edges},
        split_if_necessary::edge_split_face::split_faces_by_edges_if_necessary,
    };
    use geop_geometry::{points::point::Point, transforms::Transform};
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
            midpoint = midpoint / f.boundaries[0].clone().edges.len() as f64;
            let f = f.transform(Transform::from_translation(midpoint * 0.2));
            scene.faces.push((f, Color::white()));
        }

        let faces = split_faces_by_edges_if_necessary(volume2.all_faces(), &split_edges);
        for f in faces {
            let mut midpoint = Point::zero();
            for e in f.boundaries[0].clone().edges.iter() {
                midpoint = midpoint + e.get_midpoint();
            }
            midpoint = midpoint / f.boundaries[0].clone().edges.len() as f64;
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
}
