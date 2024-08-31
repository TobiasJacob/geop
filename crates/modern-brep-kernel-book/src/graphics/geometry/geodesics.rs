#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use geop_geometry::{points::point::Point, surfaces::surface::Surface};
    use geop_topology::{
        primitive_objects::faces::{cylinder::primitive_cylinder, sphere::primitive_sphere},
        topology::{
            edge::Edge,
            scene::{Color, Scene},
        },
    };
    use geop_wgpu::headless_renderer::HeadlessRenderer;
    use rstest::rstest;

    use crate::tests::renderer;

    #[rstest]
    async fn test_geodesics(#[future] renderer: Box<HeadlessRenderer>) {
        let face = primitive_sphere(Point::zero(), 1.0);

        let mut scene = Scene::new(vec![], vec![], vec![], vec![]);

        scene.faces.push((face.clone(), Color::light_gray()));

        let points = vec![
            Point::new(0.1, -1.0, 0.7),
            Point::new(-0.3, -0.8, 0.5),
            Point::new(0.5, -1.4, -0.3),
            Point::new(-0.2, -0.8, -0.5),
            Point::new(-0.2, -1.4, 0.3),
        ]
        .iter()
        .map(|p| p.normalize())
        .collect::<Vec<Point>>();

        for p in points.iter() {
            scene.points.push((*p, Color::white()));
        }

        let mut geodesics = Vec::new();

        for (p1, p2) in points.iter().zip(points.iter().skip(1)) {
            let geodesic = face.edge_from_to(*p1, *p2);
            scene.edges.push((geodesic.clone(), Color::gray()));
            geodesics.push(geodesic);
        }

        // // Show normal vectors of the geodesics
        // for g in geodesics.iter() {
        //     match &g.curve {
        //         Curve::Line(_) => panic!(),
        //         Curve::Circle(c) => {
        //             scene.points.push((c.normal * 1.5, Color::blue()));
        //         }
        //     }
        // }

        // for g1 in geodesics.iter() {
        //     for g2 in geodesics.iter() {
        //         if g1 == g2 {
        //             continue;
        //         }
        //         let intersection = edge_edge_intersection(g1, g2);
        //         match intersection {
        //             EdgeEdgeIntersection::None => {}
        //             EdgeEdgeIntersection::Points(points) => {
        //                 for p in points {
        //                     scene.points.push((p, Color::green()));
        //                 }
        //             }
        //             EdgeEdgeIntersection::Edges(_) => {
        //                 panic!("Unexpected edge-edge intersection");
        //             }
        //         }
        //     }
        // }

        renderer
            .await
            .render_to_file(
                &scene,
                false,
                false,
                Point::new(0.0, -3.0, 0.0),
                std::path::Path::new("src/generated_images/geometry/geodesics.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_geodesics2(#[future] renderer: Box<HeadlessRenderer>) {
        let face = primitive_cylinder(Point::zero(), Point::unit_z(), 1.0);

        let mut scene = Scene::new(vec![], vec![], vec![], vec![]);

        scene.faces.push((face.clone(), Color::light_gray()));

        let points = vec![
            Point::new(0.1, -1.0, 0.7),
            Point::new(-0.3, -0.8, 0.5),
            Point::new(1.5, -1.4, -0.3),
            Point::new(-0.2, -0.8, -0.5),
            Point::new(-0.2, -1.4, 0.2),
        ]
        .iter()
        .map(|p| {
            let mut p2 = p.clone();
            p2.z = 0.0;
            let mut p2 = p2.normalize();
            p2.z = p.z;
            p2
        })
        .collect::<Vec<Point>>();

        for p in points.iter() {
            scene.points.push((*p, Color::white()));
        }

        let mut geodesics = Vec::new();

        for (p1, p2) in points.iter().zip(points.iter().skip(1)) {
            let geodesic = face.edge_from_to(*p1, *p2);
            scene.edges.push((geodesic.clone(), Color::gray()));
            geodesics.push(geodesic);
        }

        // // Show normal vectors of the geodesics
        // for g in geodesics.iter() {
        //     match &g.curve {
        //         Curve::Line(_) => panic!(),
        //         Curve::Circle(c) => {
        //             scene.points.push((c.normal * 1.5, Color::blue()));
        //         }
        //     }
        // }

        // for g1 in geodesics.iter() {
        //     for g2 in geodesics.iter() {
        //         if g1 == g2 {
        //             continue;
        //         }
        //         let intersection = edge_edge_intersection(g1, g2);
        //         match intersection {
        //             EdgeEdgeIntersection::None => {}
        //             EdgeEdgeIntersection::Points(points) => {
        //                 for p in points {
        //                     scene.points.push((p, Color::green()));
        //                 }
        //             }
        //             EdgeEdgeIntersection::Edges(_) => {
        //                 panic!("Unexpected edge-edge intersection");
        //             }
        //         }
        //     }
        // }

        renderer
            .await
            .render_to_file(
                &scene,
                false,
                false,
                Point::new(0.0, -3.0, 0.0),
                std::path::Path::new("src/generated_images/geometry/geodesics2.png"),
            )
            .await;
    }

    #[rstest]
    async fn test_geodesics_debug(#[future] renderer: Box<HeadlessRenderer>) {
        let face = primitive_cylinder(Point::zero(), Point::unit_z(), 1.0);

        let mut scene = Scene::new(vec![], vec![], vec![], vec![]);

        scene.faces.push((face.clone(), Color::light_gray()));

        let points = vec![
            Point::new(0.1, -1.0, 0.7),
            Point::new(-0.3, -0.8, 0.5),
            Point::new(0.5, -1.4, -0.3),
            Point::new(-0.2, -0.8, -0.5),
            Point::new(-0.2, -1.4, 0.3),
        ]
        .iter()
        .map(|p| {
            let mut p2 = p.clone();
            p2.z = 0.0;
            let mut p2 = p2.normalize();
            p2.z = p.z;
            p2
        })
        .collect::<Vec<Point>>();

        for p in points.iter() {
            scene.points.push((*p, Color::white()));
        }

        for (p1, p2) in points.iter().zip(points.iter().skip(1)) {
            let geodesic = match face.surface.borrow() {
                Surface::Cylinder(cylinder) => cylinder.geodesic(*p1, *p2),
                _ => panic!(),
            };
            scene
                .edges
                .push((Edge::new(None, None, geodesic.clone()), Color::gray()));
        }

        renderer
            .await
            .render_to_file(
                &scene,
                false,
                false,
                Point::new(0.0, -3.0, 0.0),
                std::path::Path::new("src/generated_images/geometry/geodesics_debug.png"),
            )
            .await;
    }
}
