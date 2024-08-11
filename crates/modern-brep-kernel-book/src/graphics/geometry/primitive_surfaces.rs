#[cfg(test)]
mod tests {
    use geop_geometry::points::point::Point;
    use geop_rasterize::face::rasterize_face_into_triangle_list;
    use geop_topology::{
        primitive_objects::faces::{plane::primitive_plane, sphere::primitive_sphere},
        topology::scene::{Color, Scene},
    };
    use geop_wgpu::headless_renderer::HeadlessRenderer;
    use rstest::rstest;

    use crate::tests::renderer;

    #[rstest]
    async fn test_primitive_plane(#[future] renderer: Box<HeadlessRenderer>) {
        let face = primitive_plane(Point::new_zero(), Point::new_unit_x(), Point::new_unit_z());
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
        let face = primitive_sphere(Point::new_zero(), 1.0);
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
        let face = primitive_sphere(Point::new_zero(), 1.0);
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
}
