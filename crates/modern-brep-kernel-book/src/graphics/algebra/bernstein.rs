#[cfg(test)]
mod tests {
    use std::vec;

    use crate::tests::renderer;
    use geop_geometry::curves::bernstein_polynomial::BernsteinPolynomial;
    use geop_geometry::efloat::EFloat64;
    use geop_geometry::{point::Point, transforms::Transform};
    use geop_rasterize::{
        edge_buffer::{EdgeBuffer, RenderEdge},
        functions::{rasterize_coordinate_system, rasterize_multidimensional_function},
        triangle_buffer::TriangleBuffer,
        vertex_buffer::VertexBuffer,
    };
    use geop_topology::topology::scene::Color;
    use geop_wgpu::headless_renderer::HeadlessRenderer;
    use rstest::rstest;

    #[rstest]
    async fn test_bernstein(#[future] renderer: Box<HeadlessRenderer>) {
        let curve = BernsteinPolynomial::new(vec![
            Point::unit_y() * EFloat64::from(0.0) + Point::unit_x() * EFloat64::from(0.0),
            Point::unit_y() * EFloat64::from(0.6) + Point::unit_x() * EFloat64::from(0.25),
            Point::unit_y() * EFloat64::from(0.1) + Point::unit_x() * EFloat64::from(0.5),
            Point::unit_y() * EFloat64::from(0.8) + Point::unit_x() * EFloat64::from(0.75),
            Point::unit_y() * EFloat64::from(0.3) + Point::unit_x() * EFloat64::from(1.0),
        ]);

        let mut edge_buffer = rasterize_multidimensional_function(&curve, Color::black(), 0.0, 1.0);
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
                let curve =
                    BernsteinPolynomial::bernstein_basis(i, n, Point::unit_y(), Point::unit_x());
                println!("{}: {}", &curve, &curve.to_monomial_polynom());
                let mut edge_buffer_i =
                    rasterize_multidimensional_function(&curve, Color::black(), -2.0, 3.0);
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

    #[rstest]
    async fn test_bezier_curve(#[future] renderer: Box<HeadlessRenderer>) {
        let mut edge_buffer = EdgeBuffer::empty();

        let control_points = vec![
            Point::from_f64(0.0, 0.2, 0.0),
            Point::from_f64(0.3333, 1.0, 0.0),
            Point::from_f64(0.6667, 0.0, 0.0),
            Point::from_f64(1.0, 0.8, 0.0),
        ];
        // For loop to add the control point lines to the edge buffer
        for i in 0..control_points.len() - 1 {
            let edge_buffer_i = EdgeBuffer::new(vec![RenderEdge::new(
                control_points[i],
                control_points[(i + 1) % control_points.len()],
                Color::gray(),
            )]);
            edge_buffer.join(&edge_buffer_i);
        }

        let curve = BernsteinPolynomial::new(control_points);

        // let curve = BernsteinBasis::new(i, n).unwrap();
        let edge_buffer_i = rasterize_multidimensional_function(&curve, Color::black(), 0.0, 1.0);
        edge_buffer.join(&edge_buffer_i);

        let coordinate_system_buffer = rasterize_coordinate_system(
            Point::zero(),
            Point::ones(),
            Point::from_f64(0.1, 0.1, 0.1),
        );
        edge_buffer.join(&coordinate_system_buffer);

        let mut renderer = renderer.await;
        renderer
            .render_buffers_to_file(
                VertexBuffer::empty(),
                edge_buffer,
                TriangleBuffer::empty(),
                false,
                (-0.5, 1.5),
                (-0.1, 1.1),
                std::path::Path::new("src/generated_images/algebra/bezier_curve.png"),
            )
            .await;
    }

    // #[rstest]
    // async fn test_subdivision(#[future] renderer: Box<HeadlessRenderer>) {
    //     let mut edge_buffer = EdgeBuffer::empty();

    //     let control_points = vec![
    //         EFloat64::from(0.3),
    //         EFloat64::from(0.1),
    //         EFloat64::from(0.9),
    //         EFloat64::from(0.5),
    //     ];
    //     // For loop to add the control point lines to the edge buffer
    //     for i in 0..control_points.len() - 1 {
    //         let edge_buffer_i = EdgeBuffer::new(vec![RenderEdge::new(
    //             control_points[i] * Point::unit_y()
    //                 + Point::unit_x() * EFloat64::from(i as f64 / 3.0),
    //             control_points[(i + 1) % control_points.len()] * Point::unit_y()
    //                 + Point::unit_x() * EFloat64::from((i + 1) as f64 / 3.0),
    //             Color::gray(),
    //         )]);
    //         edge_buffer.join(&edge_buffer_i);
    //     }

    //     let curve = BernsteinPolynomial::new(control_points.clone());

    //     let split_point = 0.4;
    //     let (curve1, curve2) = curve.subdivide(EFloat64::from(split_point));

    //     let mut left_buffer =
    //         rasterize_multidimensional_function_in_1d(&curve1, Color::red(), 0.0, 1.0);
    //     left_buffer.transform(&Transform::from_scale(Point::from_f64(
    //         split_point,
    //         1.0,
    //         1.0,
    //     )));
    //     edge_buffer.join(&left_buffer);
    //     let mut right_buffer =
    //         rasterize_multidimensional_function_in_1d(&curve2, Color::blue(), 0.0, 1.0);

    //     right_buffer.transform(&Transform::from_scale(Point::from_f64(
    //         1.0 - split_point,
    //         1.0,
    //         1.0,
    //     )));
    //     right_buffer.transform(&Transform::from_translation(
    //         Point::unit_x() * EFloat64::from(split_point),
    //     ));

    //     edge_buffer.join(&right_buffer);

    //     // Now visualize the control points for curve1
    //     let mut control_points_buffer = EdgeBuffer::empty();
    //     for i in 0..curve1.coefficients.len() - 1 {
    //         let p1 = curve1.coefficients[i] * Point::unit_y()
    //             + Point::unit_x() * EFloat64::from(i as f64 / 3.0 * split_point);
    //         let p2 = curve1.coefficients[(i + 1) % curve1.coefficients.len()] * Point::unit_y()
    //             + Point::unit_x() * EFloat64::from((i + 1) as f64 / 3.0 * split_point);

    //         control_points_buffer.add(RenderEdge::new(p1, p2, Color::gray()));
    //     }
    //     edge_buffer.join(&control_points_buffer);

    //     // Now visualize the control points for curve2
    //     let mut control_points_buffer = EdgeBuffer::empty();
    //     for i in 0..curve2.coefficients.len() - 1 {
    //         let p1 = curve2.coefficients[i] * Point::unit_y()
    //             + Point::unit_x() * EFloat64::from(i as f64 / 3.0 * (1.0 - split_point))
    //             + Point::unit_x() * EFloat64::from(split_point);
    //         let p2 = curve2.coefficients[(i + 1) % curve2.coefficients.len()] * Point::unit_y()
    //             + Point::unit_x() * EFloat64::from((i + 1) as f64 / 3.0 * (1.0 - split_point))
    //             + Point::unit_x() * EFloat64::from(split_point);

    //         control_points_buffer.add(RenderEdge::new(p1, p2, Color::gray()));
    //     }
    //     edge_buffer.join(&control_points_buffer);

    //     let coordinate_system_buffer = rasterize_coordinate_system(
    //         Point::zero(),
    //         Point::ones(),
    //         Point::from_f64(0.1, 0.1, 0.1),
    //     );
    //     edge_buffer.join(&coordinate_system_buffer);

    //     let mut renderer = renderer.await;
    //     renderer
    //         .render_buffers_to_file(
    //             VertexBuffer::empty(),
    //             edge_buffer,
    //             TriangleBuffer::empty(),
    //             false,
    //             (-0.5, 1.5),
    //             (-0.1, 1.1),
    //             std::path::Path::new("src/generated_images/algebra/subdivision.png"),
    //         )
    //         .await;
    // }

    // #[rstest]
    // async fn test_root_finding1(#[future] renderer: Box<HeadlessRenderer>) {
    //     let curve = BernsteinPolynomial::new(vec![
    //         Point::unit_z() * EFloat64::from(0.3),
    //         Point::unit_z() * EFloat64::from(-0.7),
    //         Point::unit_z() * EFloat64::from(0.8),
    //         Point::unit_z() * EFloat64::from(0.1),
    //         Point::unit_z() * EFloat64::from(-0.8),
    //         Point::unit_z() * EFloat64::from(0.4),
    //         Point::unit_z() * EFloat64::from(0.1),
    //     ]);

    //     let curve_buffer = rasterize_multidimensional_function(&curve, Color::black(), 0.0, 1.0);

    //     let roots = curve.find_roots_z();

    //     let mut edge_buffer = EdgeBuffer::empty();
    //     edge_buffer.join(&curve_buffer);

    //     let coordinate_system_buffer = rasterize_coordinate_system(
    //         Point::zero(),
    //         Point::ones(),
    //         Point::from_f64(0.1, 0.1, 0.1),
    //     );
    //     edge_buffer.join(&coordinate_system_buffer);

    //     let mut vertex_buffer = VertexBuffer::new(vec![]);
    //     for root in roots.unwrap() {
    //         println!("Root: {:?}", root);
    //         let root_point = Point::from_f64(root.to_f64(), 0.0, 0.0);
    //         vertex_buffer.add(RenderVertex::new(root_point, Color::red()));

    //         // Draw a vertical line at the root
    //         let edge_buffer_i = EdgeBuffer::new(vec![RenderEdge::new(
    //             root_point + Point::unit_y() * EFloat64::from(-0.1),
    //             root_point + Point::unit_y() * EFloat64::from(0.1),
    //             Color::black(),
    //         )]);
    //         edge_buffer.join(&edge_buffer_i);
    //     }

    //     let mut renderer = renderer.await;
    //     renderer
    //         .render_buffers_to_file(
    //             vertex_buffer,
    //             edge_buffer,
    //             TriangleBuffer::empty(),
    //             false,
    //             (-0.5, 1.5),
    //             (-0.1, 1.1),
    //             std::path::Path::new("src/generated_images/algebra/roots.png"),
    //         )
    //         .await;
    // }

    // #[rstest]
    // async fn test_root_finding2(#[future] renderer: Box<HeadlessRenderer>) {
    //     let curve = BernsteinPolynomial::new(vec![
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(1.0),
    //     ]);

    //     let curve_buffer = rasterize_multidimensional_function(&curve, Color::black(), 0.0, 1.0);

    //     let roots = curve.find_roots_z();

    //     let mut edge_buffer = EdgeBuffer::empty();
    //     edge_buffer.join(&curve_buffer);

    //     let coordinate_system_buffer = rasterize_coordinate_system(
    //         Point::zero(),
    //         Point::ones(),
    //         Point::from_f64(0.1, 0.1, 0.1),
    //     );
    //     edge_buffer.join(&coordinate_system_buffer);

    //     let mut vertex_buffer = VertexBuffer::new(vec![]);
    //     for root in roots.unwrap() {
    //         println!("Root: {:?}", root);
    //         let root_point = Point::from_f64(root.to_f64(), 0.0, 0.0);
    //         vertex_buffer.add(RenderVertex::new(root_point, Color::red()));

    //         // Draw a vertical line at the root
    //         let edge_buffer_i = EdgeBuffer::new(vec![RenderEdge::new(
    //             root_point + Point::unit_y() * EFloat64::from(-0.1),
    //             root_point + Point::unit_y() * EFloat64::from(0.1),
    //             Color::black(),
    //         )]);
    //         edge_buffer.join(&edge_buffer_i);
    //     }

    //     let mut renderer = renderer.await;
    //     renderer
    //         .render_buffers_to_file(
    //             vertex_buffer,
    //             edge_buffer,
    //             TriangleBuffer::empty(),
    //             false,
    //             (-0.5, 1.5),
    //             (-0.1, 1.1),
    //             std::path::Path::new("src/generated_images/algebra/roots2.png"),
    //         )
    //         .await;
    // }

    // #[rstest]
    // async fn test_root_finding3(#[future] renderer: Box<HeadlessRenderer>) {
    //     let curve = BernsteinPolynomial::new(vec![
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(0.0),
    //         Point::unit_z() * EFloat64::from(1.0),
    //     ]);

    //     let curve_buffer = rasterize_multidimensional_function(&curve, Color::black(), 0.0, 1.0);

    //     let roots = curve.find_roots_z();

    //     let mut edge_buffer = EdgeBuffer::empty();
    //     edge_buffer.join(&curve_buffer);

    //     let coordinate_system_buffer = rasterize_coordinate_system(
    //         Point::zero(),
    //         Point::ones(),
    //         Point::from_f64(0.1, 0.1, 0.1),
    //     );
    //     edge_buffer.join(&coordinate_system_buffer);

    //     let mut vertex_buffer = VertexBuffer::new(vec![]);
    //     for root in roots.unwrap() {
    //         println!("Root: {:?}", root);
    //         let root_point = Point::from_f64(root.to_f64(), 0.0, 0.0);
    //         vertex_buffer.add(RenderVertex::new(root_point, Color::red()));

    //         // Draw a vertical line at the root
    //         let edge_buffer_i = EdgeBuffer::new(vec![RenderEdge::new(
    //             root_point + Point::unit_y() * EFloat64::from(-0.1),
    //             root_point + Point::unit_y() * EFloat64::from(0.1),
    //             Color::black(),
    //         )]);
    //         edge_buffer.join(&edge_buffer_i);
    //     }

    //     let mut renderer = renderer.await;
    //     renderer
    //         .render_buffers_to_file(
    //             vertex_buffer,
    //             edge_buffer,
    //             TriangleBuffer::empty(),
    //             false,
    //             (-0.5, 1.5),
    //             (-0.1, 1.1),
    //             std::path::Path::new("src/generated_images/algebra/roots3.png"),
    //         )
    //         .await;
    // }
}
