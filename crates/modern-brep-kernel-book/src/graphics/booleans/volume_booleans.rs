#[cfg(test)]
mod tests {
    use geop_booleans::remesh::volume::volume_split_edges;
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
}
