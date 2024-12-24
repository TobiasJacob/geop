#[cfg(test)]
mod tests {
    use geop_algebra::{efloat::EFloat64, monomial_polynom::MonomialPolynom};
    use geop_geometry::point::Point;
    use geop_rasterize::{
        functions::{rasterize_coordinate_system, rasterize_onedimensional_function},
        triangle_buffer::TriangleBuffer,
        vertex_buffer::VertexBuffer,
    };
    use geop_topology::topology::scene::Color;
    use geop_wgpu::headless_renderer::HeadlessRenderer;
    use rstest::rstest;

    use crate::tests::renderer;

    #[rstest]
    async fn test_monomial(#[future] renderer: Box<HeadlessRenderer>) {
        let mut edge_buffer = rasterize_coordinate_system(
            Point::zero(),
            Point::ones(),
            Point::ones() * EFloat64::from(0.1),
        );

        let curve = MonomialPolynom::new(vec![
            EFloat64::from(0.2),
            EFloat64::from(0.4),
            EFloat64::from(1.8),
            EFloat64::from(-1.7),
        ]);
        edge_buffer.join(&rasterize_onedimensional_function(
            &curve,
            Color::black(),
            0.0,
            1.0,
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
                std::path::Path::new("src/generated_images/algebra/monomial_polynom.png"),
            )
            .await;
    }
}
