#[cfg(test)]
mod tests {
    use geop_algebra::{
        bernstein_polynomial::BernsteinPolynomial, bspline_curve::BSplineCurve, efloat::EFloat64,
    };
    use geop_geometry::point::Point;
    use geop_rasterize::{
        functions::rasterize_multidimensional_function_in_1d, triangle_buffer::TriangleBuffer,
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
            EFloat64::from(1.0),
            EFloat64::from(0.0),
            EFloat64::from(1.5),
            EFloat64::from(0.3),
        ]);

        let edge_buffer = rasterize_multidimensional_function_in_1d(&curve, Color::black());

        let mut renderer = renderer.await;
        renderer
            .render_buffers_to_file(
                VertexBuffer::empty(),
                edge_buffer,
                TriangleBuffer::empty(),
                false,
                Point::from_f64(0.0, -4.0, 0.0),
                std::path::Path::new("src/generated_images/algebra/bernstein.png"),
            )
            .await;
    }
}
