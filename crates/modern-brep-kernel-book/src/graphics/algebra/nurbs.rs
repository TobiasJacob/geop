#[cfg(test)]
mod tests {
    use geop_geometry::{efloat::EFloat64, nurbs_curve::NurbsCurve};
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
    async fn test_nurbs_basis(#[future] renderer: Box<HeadlessRenderer>) {
        let mut edge_buffer = EdgeBuffer::empty();

        let knot_vector = vec![1, 2, 3, 4, 5, 6, 7]
            .iter()
            .map(|x| EFloat64::from(*x as f64))
            .collect::<Vec<EFloat64>>();
        let max_k = knot_vector.len() - 2;
        for k in 0..=max_k {
            for i in 0..=knot_vector.len() - k - 2 {
                let curve =
                    NurbsCurve::<EFloat64>::try_new_from_basis(i, k, knot_vector.clone()).unwrap();
                let mut edge_buffer_i =
                    rasterize_multidimensional_function_in_1d(&curve, Color::black(), -0.0, 8.0);
                let t = Transform::from_translation(
                    Point::unit_y() * EFloat64::from((max_k - k) as f64 / max_k as f64 * 6.0),
                ) * Transform::from_scale(Point::from_f64(1.0, 0.8, 1.0));
                edge_buffer_i.transform(&t);
                edge_buffer.join(&edge_buffer_i);

                let mut coordinate_system_buffer = rasterize_coordinate_system(
                    Point::zero(),
                    Point::from_f64(8.0, 0.0, 1.0),
                    Point::from_f64(1.0, 1.0, 1.0),
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
                (-0.5, 8.5),
                (-0.5, 7.5),
                std::path::Path::new("src/generated_images/algebra/nurbs_basis.png"),
            )
            .await;
    }
    #[rstest]
    async fn test_nurbs_basis_with_sharp_border(#[future] renderer: Box<HeadlessRenderer>) {
        let mut edge_buffer = EdgeBuffer::empty();

        let knot_vector = vec![1, 1, 1, 2, 3, 3, 3]
            .iter()
            .map(|x| EFloat64::from(*x as f64))
            .collect::<Vec<EFloat64>>();
        let max_k = knot_vector.len() - 2;
        for k in 0..=max_k {
            for i in 0..=knot_vector.len() - k - 2 {
                let curve =
                    NurbsCurve::<EFloat64>::try_new_from_basis(i, k, knot_vector.clone()).unwrap();
                let mut edge_buffer_i =
                    rasterize_multidimensional_function_in_1d(&curve, Color::black(), -0.0, 4.0);
                let t = Transform::from_translation(
                    Point::unit_y() * EFloat64::from((max_k - k) as f64 / max_k as f64 * 6.0),
                ) * Transform::from_scale(Point::from_f64(1.0, 0.8, 1.0));
                edge_buffer_i.transform(&t);
                edge_buffer.join(&edge_buffer_i);

                let mut coordinate_system_buffer = rasterize_coordinate_system(
                    Point::zero(),
                    Point::from_f64(4.0, 0.0, 1.0),
                    Point::from_f64(1.0, 1.0, 1.0),
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
                (-0.5, 4.5),
                (-0.5, 7.5),
                std::path::Path::new(
                    "src/generated_images/algebra/nurbs_basis_with_sharp_border.png",
                ),
            )
            .await;
    }
}
