pub mod booleans;
pub mod geometry;
pub mod topology;

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use geop_geometry::{efloat::EFloat64, geometry_error::GeometryError, point::Point};
    use geop_topology::{
        primitive_objects::{edges::line::primitive_line, volumes::cube::primitive_cube},
        topology::{
            edge::Edge,
            face::Face,
            scene::{Color, Scene},
        },
        topology_error::{TopologyError, TopologyErrorRoot, TopologyResult},
    };
    use geop_wgpu::headless_renderer::HeadlessRenderer;
    use rstest::rstest;

    use crate::tests::renderer;

    #[rstest]
    async fn test_headless_renderer_light(#[future] renderer: Box<HeadlessRenderer>) {
        let volume = primitive_cube(
            EFloat64::from(1.0),
            EFloat64::from(1.0),
            EFloat64::from(1.0),
        );
        let scene = Scene::new(vec![(volume, Color::white())], vec![], vec![], vec![]);
        renderer
            .await
            .render_to_file(
                &scene,
                false,
                false,
                Point::from_f64(0.0, -2.0, 1.0),
                std::path::Path::new("src/generated_images/test_light.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_headless_renderer_dark(#[future] renderer: Box<HeadlessRenderer>) {
        let volume = primitive_cube(
            EFloat64::from(1.0),
            EFloat64::from(1.0),
            EFloat64::from(1.0),
        );
        let scene = Scene::new(vec![(volume, Color::white())], vec![], vec![], vec![]);
        renderer
            .await
            .render_to_file(
                &scene,
                true,
                false,
                Point::from_f64(0.0, -2.0, 1.0),
                std::path::Path::new("src/generated_images/test_dark.png"),
            )
            .await;
    }

    fn try_something_impossible() -> TopologyResult<Edge> {
        primitive_line(
            Point::from_f64(0.0, 0.0, 0.0),
            Point::from_f64(0.0, 0.0, 0.0),
        )
    }

    fn geometry_error_to_scene(e: &GeometryError) -> Scene {
        match e {
            GeometryError::Root {
                message: _,
                backtrace: _,
            } => Scene::new(vec![], vec![], vec![], vec![]),
            GeometryError::Context {
                message: _,
                error_scene,
                inner_error,
            } => {
                let mut scene = geometry_error_to_scene(inner_error);
                if let Some(error_scene) = error_scene {
                    for (point, color) in error_scene.points.iter() {
                        scene
                            .points
                            .push((point.clone(), Color::from_category_color(color.clone())));
                    }
                    for (curve, color) in error_scene.curves.iter() {
                        scene.edges.push((
                            Edge::new(None, None, curve.clone()),
                            Color::from_category_color(color.clone()),
                        ));
                    }
                    for (surface, color) in error_scene.surfaces.iter() {
                        scene.faces.push((
                            Face::new(vec![], Rc::new(surface.clone())),
                            Color::from_category_color(color.clone()),
                        ));
                    }
                }
                scene
            }
        }
    }

    fn topology_error_to_scene(e: &TopologyError) -> Scene {
        match e {
            TopologyError::Root(root_cause) => match root_cause {
                TopologyErrorRoot::InTopologyCrate {
                    message: _,
                    backtrace: _,
                } => Scene::new(vec![], vec![], vec![], vec![]),
                TopologyErrorRoot::FromGeometryError { geometry_error } => {
                    geometry_error_to_scene(geometry_error)
                }
            },
            TopologyError::Context {
                message: _,
                error_scene,
                inner_error,
            } => {
                let mut scene = topology_error_to_scene(inner_error);
                if let Some(error_scene) = error_scene {
                    for (point, color) in error_scene.points.iter() {
                        scene
                            .points
                            .push((point.clone(), Color::from_category_color(color.clone())));
                    }
                    for (edge, color) in error_scene.edges.iter() {
                        scene
                            .edges
                            .push((edge.clone(), Color::from_category_color(color.clone())));
                    }
                    for (face, color) in error_scene.face.iter() {
                        scene
                            .faces
                            .push((face.clone(), Color::from_category_color(color.clone())));
                    }
                    for (volume, color) in error_scene.volumes.iter() {
                        scene
                            .volumes
                            .push((volume.clone(), Color::from_category_color(color.clone())));
                    }
                }
                scene
            }
        }
    }

    async fn render_failable_closure<F: FnOnce() -> TopologyResult<Scene>>(
        renderer: &mut HeadlessRenderer,
        file_name: &str,
        camera_pos: Point,
        f: F,
    ) {
        match f() {
            Ok(scene) => {
                renderer
                    .render_to_file(
                        &scene,
                        false,
                        false,
                        camera_pos,
                        std::path::Path::new(file_name),
                    )
                    .await;
            }
            Err(e) => {
                renderer
                    .render_to_file(
                        &topology_error_to_scene(&e),
                        false,
                        false,
                        camera_pos,
                        std::path::Path::new(file_name),
                    )
                    .await;

                TopologyResult::<()>::Err(e).unwrap();
            }
        }
    }

    #[rstest]
    #[should_panic]
    async fn test_error_handling(#[future] renderer: Box<HeadlessRenderer>) {
        render_failable_closure(
            &mut *renderer.await,
            "src/generated_images/test_error_handling.png",
            Point::from_f64(0.0, -2.0, 1.0),
            || {
                let line = try_something_impossible()?;
                let scene = Scene::new(vec![], vec![], vec![(line, Color::white())], vec![]);
                Ok(scene)
            },
        )
        .await
    }
}
