#[cfg(test)]
mod tests {
    use std::vec;

    use geop_algebra::primitives::triangle::quickhull;
    use geop_geometry::point::Point;
    use geop_rasterize::{
        edge_buffer::EdgeBuffer, triangle_buffer::TriangleBuffer, vertex_buffer::VertexBuffer,
    };
    use geop_wgpu::headless_renderer::HeadlessRenderer;
    use rstest::rstest;

    use crate::tests::renderer;

    #[rstest]
    async fn test_convex_hull(#[future] renderer: Box<HeadlessRenderer>) {
        let points = vec![
            Point::from_f64(-1.0, -1.0, -1.0),
            Point::from_f64(1.0, -1.0, -1.0),
            Point::from_f64(-1.0, 1.0, -1.0),
            Point::from_f64(-1.0, -1.0, 1.0),
            Point::from_f64(1.0, 1.0, -1.0),
            Point::from_f64(1.0, -1.0, 1.0),
            Point::from_f64(-1.0, 1.0, 1.0),
            Point::from_f64(1.0, 1.0, 1.0),
        ];
        let faces = quickhull(points).unwrap();
        for face in &faces {
            println!("{}", face);
        }

        let mut renderer = renderer.await;
        renderer
            .render_buffers_to_file_3d(
                VertexBuffer::empty(),
                EdgeBuffer::empty(),
                TriangleBuffer::from_geop_triangle_buffer(faces),
                false,
                Point::from_f64(2.0, -2.0, 3.0),
                std::path::Path::new("src/generated_images/algebra/convex_hull.png"),
            )
            .await;
    }
}
