#[cfg(test)]
mod tests {
    use geop_algebra::{
        bernstein_basis::BernsteinBasis, bernstein_polynomial::BernsteinPolynomial,
        efloat::EFloat64,
    };
    use geop_geometry::point::Point;
    use geop_rasterize::{
        edge_buffer,
        functions::{
            rasterize_coordinate_system, rasterize_multidimensional_function_in_1d,
            rasterize_onedimensional_function,
        },
        triangle_buffer::TriangleBuffer,
        vertex_buffer::VertexBuffer,
    };
    use geop_topology::topology::scene::Color;
    use geop_wgpu::headless_renderer::HeadlessRenderer;
    use rstest::rstest;

    use crate::tests::renderer;

    #[rstest]
    async fn test_bernstein(#[future] renderer: Box<HeadlessRenderer>) {
        let curve = BernsteinPolynomial::new(vec![
            EFloat64::from(0.0),
            EFloat64::from(0.6),
            EFloat64::from(0.1),
            EFloat64::from(0.8),
            EFloat64::from(0.3),
        ]);

        let mut edge_buffer = rasterize_multidimensional_function_in_1d(&curve, Color::black());
        edge_buffer.join(&rasterize_coordinate_system(
            Point::zero(),
            Point::ones(),
            Point::ones() * EFloat64::from(0.1),
        ));

        let mut renderer = renderer.await;
        renderer
            .render_buffers_to_file(
                VertexBuffer::empty(),
                edge_buffer,
                TriangleBuffer::empty(),
                false,
                (-0.1, 1.1),
                (-0.1, 1.1),
                std::path::Path::new("src/generated_images/algebra/bernstein.png"),
            )
            .await;
    }
    #[rstest]
    async fn test_bernstein_basis(#[future] renderer: Box<HeadlessRenderer>) {
        let mut edge_buffer = rasterize_coordinate_system(
            Point::zero(),
            Point::ones(),
            Point::ones() * EFloat64::from(0.1),
        );

        for i in 0..=5 {
            let curve = BernsteinBasis::new(i, 5).unwrap();
            let edge_buffer_i = rasterize_onedimensional_function(&curve, Color::black());
            edge_buffer.join(&edge_buffer_i);
        }

        let mut renderer = renderer.await;
        renderer
            .render_buffers_to_file(
                VertexBuffer::empty(),
                edge_buffer,
                TriangleBuffer::empty(),
                false,
                (-0.1, 1.1),
                (-0.1, 1.1),
                std::path::Path::new("src/generated_images/algebra/bernstein_basis.png"),
            )
            .await;
    }
}
