#[cfg(test)]
mod tests {
    use std::vec;

    use geop_booleans::intersections::edge_edge::{edge_edge_intersection, EdgeEdgeIntersection};
    use geop_geometry::{efloat::EFloat64, point::Point};
    use geop_topology::{
        primitive_objects::edges::{
            circle::primitive_circle, ellipse::primitive_ellipse, line::primitive_infinite_line,
        },
        topology::scene::{Color, Scene},
    };
    use geop_wgpu::headless_renderer::HeadlessRenderer;
    use rstest::rstest;

    use crate::tests::renderer;

    #[rstest]
    async fn test_line_line_intersections(#[future] renderer: Box<HeadlessRenderer>) {
        let line1 = primitive_infinite_line(
            Point::from_f64(-1.0, 0.0, -0.5),
            Point::from_f64(1.0, 0.0, -0.5),
        );
        let line2 = primitive_infinite_line(
            Point::from_f64(-1.0, 0.0, 0.5),
            Point::from_f64(1.0, 0.0, 0.5),
        );
        let line3 = primitive_infinite_line(
            Point::from_f64(-1.0, 0.0, -1.0),
            Point::from_f64(1.0, 0.0, 1.0),
        );

        let mut scene_edges = vec![
            (line2.clone(), Color::white()),
            (line3.clone(), Color::white()),
        ];
        let mut scene_points = vec![];
        for intersection in [
            edge_edge_intersection(&line1, &line1),
            edge_edge_intersection(&line2, &line3),
            edge_edge_intersection(&line1, &line3),
            edge_edge_intersection(&line1, &line2),
        ] {
            match intersection {
                EdgeEdgeIntersection::Edges(edges) => {
                    for edge in edges {
                        scene_edges.push((edge, Color::red()));
                    }
                }
                EdgeEdgeIntersection::Points(points) => {
                    for point in points {
                        scene_points.push((point, Color::red()));
                    }
                }
                EdgeEdgeIntersection::None => {}
            }
        }

        let scene = Scene::new(vec![], vec![], scene_edges, scene_points);

        renderer
            .await
            .render_to_file(
                &scene,
                false,
                false,
                Point::from_f64(0.0, -3.0, 0.0),
                std::path::Path::new("src/generated_images/geometry/line_line_intersections.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_circle_circle_intersections(#[future] renderer: Box<HeadlessRenderer>) {
        let circle1 = primitive_circle(
            Point::from_f64(0.7, 0.0, 0.3),
            Point::unit_y(),
            EFloat64::from(0.3),
        );
        let circle2 = primitive_circle(
            Point::from_f64(0.4, 0.0, 0.3),
            Point::unit_y(),
            EFloat64::from(0.6),
        );
        let circle3 = primitive_circle(
            Point::from_f64(-0.4, 0.0, 0.3),
            Point::unit_y(),
            EFloat64::from(0.8),
        );

        let mut scene_edges = vec![
            (circle1.clone(), Color::white()),
            (circle2.clone(), Color::white()),
            (circle3.clone(), Color::white()),
        ];
        let mut scene_points = vec![];
        for intersection in [
            edge_edge_intersection(&circle1, &circle1),
            edge_edge_intersection(&circle2, &circle3),
            edge_edge_intersection(&circle1, &circle3),
            edge_edge_intersection(&circle1, &circle2),
        ] {
            match intersection {
                EdgeEdgeIntersection::Edges(edges) => {
                    for edge in edges {
                        scene_edges.push((edge, Color::red()));
                    }
                }
                EdgeEdgeIntersection::Points(points) => {
                    for point in points {
                        scene_points.push((point, Color::red()));
                    }
                }
                EdgeEdgeIntersection::None => {}
            }
        }

        let scene = Scene::new(vec![], vec![], scene_edges, scene_points);

        renderer
            .await
            .render_to_file(
                &scene,
                false,
                false,
                Point::from_f64(0.0, -3.0, 0.0),
                std::path::Path::new(
                    "src/generated_images/geometry/circle_circle_intersections.png",
                ),
            )
            .await;
    }

    #[rstest]
    async fn test_circle_line_intersection(#[future] renderer: Box<HeadlessRenderer>) {
        let circle1 = primitive_circle(
            Point::from_f64(0.4, 0.0, 0.3),
            Point::unit_y(),
            EFloat64::from(0.6),
        );
        let line1 = primitive_infinite_line(
            Point::from_f64(-1.0, 0.0, 0.3),
            Point::from_f64(1.0, 0.0, 0.3),
        );
        let line2 = primitive_infinite_line(
            Point::from_f64(-1.0, 0.0, -0.1),
            Point::from_f64(1.0, 0.0, 0.1),
        );
        let line3 = primitive_infinite_line(
            Point::from_f64(-1.0, 0.0, 0.9),
            Point::from_f64(1.0, 0.0, 0.9),
        );
        let line4 = primitive_infinite_line(
            Point::from_f64(-1.0, 0.0, 1.0),
            Point::from_f64(1.0, 0.0, 1.0),
        );

        let mut scene_edges = vec![
            (circle1.clone(), Color::white()),
            (line1.clone(), Color::white()),
            (line2.clone(), Color::white()),
            (line3.clone(), Color::white()),
            (line4.clone(), Color::white()),
        ];
        let mut scene_points = vec![];
        for intersection in [
            edge_edge_intersection(&circle1, &line1),
            edge_edge_intersection(&circle1, &line2),
            edge_edge_intersection(&circle1, &line3),
            edge_edge_intersection(&circle1, &line4),
        ] {
            match intersection {
                EdgeEdgeIntersection::Edges(edges) => {
                    for edge in edges {
                        scene_edges.push((edge, Color::red()));
                    }
                }
                EdgeEdgeIntersection::Points(points) => {
                    for point in points {
                        scene_points.push((point, Color::red()));
                    }
                }
                EdgeEdgeIntersection::None => {}
            }
        }

        let scene = Scene::new(vec![], vec![], scene_edges, scene_points);

        renderer
            .await
            .render_to_file(
                &scene,
                false,
                false,
                Point::from_f64(0.0, -4.0, 0.0),
                std::path::Path::new("src/generated_images/geometry/circle_line_intersections.png"),
            )
            .await;
    }

    #[rstest]
    pub async fn ellipse_ellipse_intersection(#[future] renderer: Box<HeadlessRenderer>) {
        let ellipse1 = primitive_ellipse(
            Point::zero(),
            Point::unit_y(),
            Point::unit_x() * EFloat64::from(1.5),
            Point::unit_z() * EFloat64::from(0.5),
        );
        let ellipse2 = primitive_ellipse(
            Point::from_f64(1.0, 0.0, 0.0),
            Point::unit_y(),
            Point::unit_x() * EFloat64::from(0.5),
            Point::unit_z() * EFloat64::from(1.5),
        );

        let mut scene = Scene::empty();
        scene.edges.push((ellipse1.clone(), Color::white()));
        scene.edges.push((ellipse2.clone(), Color::white()));

        let intersections = edge_edge_intersection(&ellipse1, &ellipse2);
        match intersections {
            EdgeEdgeIntersection::Edges(edges) => {
                panic!("Unexpected edges: {:?}", edges);
            }
            EdgeEdgeIntersection::Points(points) => {
                for point in points {
                    scene.points.push((point, Color::red()));
                    println!("Intersection point: {:?}", point);
                }
            }
            EdgeEdgeIntersection::None => {}
        }

        renderer
            .await
            .render_to_file(
                &scene,
                false,
                false,
                Point::from_f64(0.0, -4.0, 0.0),
                std::path::Path::new(
                    "src/generated_images/geometry/ellipse_ellipse_intersection.png",
                ),
            )
            .await;
    }
}
