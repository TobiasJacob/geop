#[cfg(test)]
mod tests {
    use geop_algebra::{bernstein_polynomial::BernsteinPolynomial, efloat::EFloat64};
    use geop_geometry::{point::Point, transforms::Transform};
    use geop_rasterize::{
        edge_buffer::EdgeBuffer,
        functions::{rasterize_coordinate_system, rasterize_multidimensional_function_in_1d},
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

        let mut edge_buffer =
            rasterize_multidimensional_function_in_1d(&curve, Color::black(), 0.0, 1.0);
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
        let mut edge_buffer = EdgeBuffer::empty();

        let max_n = 5;
        for n in 0..max_n {
            for i in 0..=n {
                // let curve = BernsteinBasis::new(i, n).unwrap();
                let curve = BernsteinPolynomial::bernstein_basis(i, n);
                println!("{}: {}", &curve, &curve.to_monomial_polynom());
                let mut edge_buffer_i =
                    rasterize_multidimensional_function_in_1d(&curve, Color::black(), -2.0, 3.0);
                let t =
                    Transform::from_translation(
                        Point::unit_y() * EFloat64::from((max_n - 1 - n) as f64 / max_n as f64),
                    ) * Transform::from_scale(Point::from_f64(1.0, 1.0 / max_n as f64 * 0.8, 1.0));
                edge_buffer_i.transform(&t);
                edge_buffer.join(&edge_buffer_i);

                let mut coordinate_system_buffer = rasterize_coordinate_system(
                    Point::zero(),
                    Point::ones(),
                    Point::from_f64(0.1, 0.1, 0.1),
                );
                coordinate_system_buffer.transform(&t);
                edge_buffer.join(&coordinate_system_buffer);
            }
        }

        let mut renderer = renderer.await;
        renderer
            .render_buffers_to_file(
                VertexBuffer::empty(),
                edge_buffer,
                TriangleBuffer::empty(),
                false,
                (-0.5, 1.5),
                (-0.1, 1.1),
                std::path::Path::new("src/generated_images/algebra/bernstein_basis.png"),
            )
            .await;
    }
}
