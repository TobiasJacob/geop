#[cfg(test)]
mod tests {
    use std::vec;

    use geop_geometry::points::point::Point;
    use geop_topology::{
        intersections::edge_edge::{edge_edge_intersection, EdgeEdgeIntersection},
        primitive_objects::edges::{circle::primitive_circle, line::primitive_infinite_line},
        topology::scene::{Color, Scene},
    };
    use geop_wgpu::headless_renderer::HeadlessRenderer;
    use rstest::rstest;

    use crate::tests::renderer;

    #[rstest]
    async fn test_line_line_intersections(#[future] renderer: Box<HeadlessRenderer>) {
        let line1 =
            primitive_infinite_line(Point::new(-1.0, -0.5, 0.0), Point::new(1.0, -0.5, 0.0));
        let line2 = primitive_infinite_line(Point::new(-1.0, 0.5, 0.0), Point::new(1.0, 0.5, 0.0));
        let line3 = primitive_infinite_line(Point::new(-1.0, 0.0, 0.0), Point::new(0.0, 1.0, 0.0));

        let mut scene_edges = vec![
            (line1.clone(), Color::white()),
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
                std::path::Path::new("src/generated_images/geometry/line_line_intersections.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_circle_circle_intersections(#[future] renderer: Box<HeadlessRenderer>) {
        let circle1 = primitive_circle(Point::new(0.7, 0.3, 0.0), Point::new_unit_z(), 0.3);
        let circle2 = primitive_circle(Point::new(0.4, 0.3, 0.0), Point::new_unit_z(), 0.6);
        let circle3 = primitive_circle(Point::new(-0.4, 0.3, 0.0), Point::new_unit_z(), 0.8);

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
                std::path::Path::new(
                    "src/generated_images/geometry/circle_circle_intersections.png",
                ),
            )
            .await;
    }
}
