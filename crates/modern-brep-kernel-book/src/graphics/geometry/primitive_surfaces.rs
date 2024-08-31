#[cfg(test)]
mod tests {
    use geop_geometry::points::point::Point;
    use geop_rasterize::face::rasterize_face_into_triangle_list;
    use geop_topology::{
        primitive_objects::faces::{
            cylinder::primitive_cylinder, plane::primitive_plane, sphere::primitive_sphere,
        },
        topology::scene::{Color, Scene},
    };
    use geop_wgpu::headless_renderer::HeadlessRenderer;
    use rstest::rstest;

    use crate::tests::renderer;

    #[rstest]
    async fn test_primitive_plane(#[future] renderer: Box<HeadlessRenderer>) {
        let face = primitive_plane(Point::zero(), Point::unit_x(), Point::unit_z());
        let triangles = rasterize_face_into_triangle_list(&face, Color::white());
        let scene = Scene::new(vec![], vec![(face, Color::white())], vec![], vec![]);
        for t in triangles.triangles.iter() {
            println!("{:?}", t);
        }
        renderer
            .await
            .render_to_file(
                &scene,
                false,
                false,
                Point::new(0.0, -3.0, 0.0),
                std::path::Path::new("src/generated_images/geometry/primitive_plane.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_primitive_sphere(#[future] renderer: Box<HeadlessRenderer>) {
        let face = primitive_sphere(Point::zero(), 1.0);
        let scene = Scene::new(vec![], vec![(face, Color::light_gray())], vec![], vec![]);
        renderer
            .await
            .render_to_file(
                &scene,
                false,
                false,
                Point::new(0.0, -3.0, 0.0),
                std::path::Path::new("src/generated_images/geometry/primitive_sphere.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_primitive_cylinder(#[future] renderer: Box<HeadlessRenderer>) {
        let face = primitive_cylinder(Point::zero(), Point::unit_z(), 1.0);
        let scene = Scene::new(
            vec![],
            vec![(face.clone(), Color::light_gray())],
            vec![],
            vec![],
        );
        renderer
            .await
            .render_to_file(
                &scene,
                false,
                false,
                Point::new(0.0, -10.0, 0.0),
                std::path::Path::new("src/generated_images/geometry/primitive_cylinder.png"),
            )
            .await;
    }
}
