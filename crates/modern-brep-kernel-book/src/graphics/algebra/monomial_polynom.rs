#[cfg(test)]
mod tests {
    use crate::tests::renderer;
    use geop_geometry::curves::monomial_polynom::MonomialPolynom;
    use geop_geometry::efloat::EFloat64;
    use geop_geometry::point::Point;
    use geop_rasterize::{
        functions::{rasterize_coordinate_system, rasterize_multidimensional_function},
        triangle_buffer::TriangleBuffer,
        vertex_buffer::VertexBuffer,
    };
    use geop_topology::topology::scene::Color;
    use geop_wgpu::headless_renderer::HeadlessRenderer;
    use rstest::rstest;

    #[rstest]
    async fn test_monomial(#[future] renderer: Box<HeadlessRenderer>) {
        let mut edge_buffer = rasterize_coordinate_system(
            Point::zero(),
            Point::ones(),
            Point::ones() * EFloat64::from(0.1),
        );

        let curve = MonomialPolynom::new(vec![
            Point::unit_y() * EFloat64::from(0.2) + Point::unit_x() * EFloat64::from(0.0),
            Point::unit_y() * EFloat64::from(0.4) + Point::unit_x() * EFloat64::from(1.0),
            Point::unit_y() * EFloat64::from(1.8) + Point::unit_x() * EFloat64::from(0.0),
            Point::unit_y() * EFloat64::from(-1.7) + Point::unit_x() * EFloat64::from(0.0),
        ]);
        edge_buffer.join(&rasterize_multidimensional_function(
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
