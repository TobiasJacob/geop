#[cfg(test)]
mod tests {
    use geop_geometry::points::point::Point;
    use geop_topology::{
        intersections::edge_edge::{edge_edge_intersection, EdgeEdgeIntersection},
        primitive_objects::faces::sphere::primitive_sphere,
        topology::scene::{Color, Scene},
    };
    use geop_wgpu::headless_renderer::HeadlessRenderer;
    use rstest::rstest;

    use crate::tests::renderer;

    #[rstest]
    async fn test_geodesics(#[future] renderer: Box<HeadlessRenderer>) {
        let face = primitive_sphere(Point::new_zero(), 1.0);

        let mut scene = Scene::new(vec![], vec![], vec![], vec![]);

        let points = vec![
            Point::new(0.1, -0.4, 0.7),
            Point::new(0.3, 0.2, 0.5),
            Point::new(0.5, 0.4, -0.3),
            Point::new(0.7, -0.2, -0.5),
            Point::new(0.9, -0.4, 0.3),
        ]
        .iter()
        .map(|p| p.normalize())
        .collect::<Vec<Point>>();

        for p in face.surface.point_grid(1.0) {
            scene.points.push((p, Color::gray()));
        }

        // for p in points.iter() {
        //     scene.points.push((*p, Color::green()));
        // }

        let mut geodesics = Vec::new();

        for (p1, p2) in points.iter().zip(points.iter().skip(1)) {
            let geodesic = face.edge_from_to(*p1, *p2);
            scene.edges.push((geodesic.clone(), Color::red()));
            geodesics.push(geodesic);
        }

        for g1 in geodesics.iter() {
            for g2 in geodesics.iter() {
                if g1 == g2 {
                    continue;
                }
                let intersection = edge_edge_intersection(g1, g2);
                match intersection {
                    EdgeEdgeIntersection::None => {}
                    EdgeEdgeIntersection::Points(points) => {
                        for p in points {
                            scene.points.push((p, Color::green()));
                        }
                    }
                    EdgeEdgeIntersection::Edges(_) => {
                        panic!("Unexpected edge-edge intersection");
                    }
                }
            }
        }

        renderer
            .await
            .render_to_file(
                &scene,
                false,
                false,
                Point::new(0.0, -2.0, 1.0),
                std::path::Path::new("src/generated_images/geometry/geodesics.png"),
            )
            .await;
    }
}
