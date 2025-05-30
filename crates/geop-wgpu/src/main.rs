use std::rc::Rc;
use std::{panic, vec};

use geop_booleans::difference::face_face::face_face_difference;
use geop_geometry::efloat::EFloat64;
use geop_geometry::{
    curves::{circle::Circle, curve::Curve, line::Line},
    point::Point,
    surfaces::{plane::Plane, surface::Surface},
    transforms::Transform,
};
use geop_rasterize::{
    edge_buffer::EdgeBuffer,
    triangle_buffer::TriangleBuffer,
    vertex_buffer::VertexBuffer,
    volume::{
        rasterize_volume_into_line_list, rasterize_volume_into_triangle_list,
        rasterize_volume_into_vertex_list,
    },
};
use geop_topology::primitive_objects::curves::rectangle::primitive_rectangle_curve;
use geop_topology::primitive_objects::edges::arc::primitive_arc;
use geop_topology::primitive_objects::edges::circle::primitive_circle;
use geop_topology::primitive_objects::edges::line::primitive_line;
use geop_topology::{
    operations::extrude::extrude,
    primitive_objects::faces::sphere::primitive_sphere,
    topology::{contour::Contour, edge::Edge, face::Face, scene::Color},
};
use geop_wgpu::window::GeopWindow;
use winit::{event_loop::EventLoop, window::WindowBuilder};

pub fn linear_edge(s: Point, e: Point) -> Edge {
    let p1 = s;
    let p2 = e;
    Edge::new(
        Some(s),
        Some(e),
        Curve::Line(Line::new(p1, (p2 - p1).normalize().unwrap()).unwrap()),
    )
}

pub fn circular_edge(s: Point, e: Point, center: Point) -> Edge {
    assert!(
        (s - center).norm_sq() - (e - center).norm_sq() <= 0.0,
        "Circular edge must have same distance to center point"
    );
    let point = s;
    Edge::new(
        Some(s),
        Some(e),
        Curve::Circle(
            Circle::try_new(
                center,
                Point::from_f64(0.0, 0.0, 1.0),
                (point - center).norm(),
            )
            .unwrap(),
        ),
    )
}

async fn run() {
    let event_loop = EventLoop::new().unwrap(); // Loop provided by winit for handling window events
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let result = panic::catch_unwind(|| {
        let v1 = Point::from_f64(0.2, 0.2, 0.0);
        let v2 = Point::from_f64(0.8, 0.2, 0.0);
        let v3 = Point::from_f64(0.8, 0.8, 0.0);
        let v4 = Point::from_f64(0.2, 0.8, 0.0);

        let contour = Contour::new(vec![
            linear_edge(v1.clone(), v2.clone()),
            linear_edge(v2.clone(), v3.clone()),
            linear_edge(v3.clone(), v4.clone()),
            // circular_edge(v4.clone(), v6.clone(), *v5.point),
            linear_edge(v4.clone(), v1.clone()),
        ]);

        let v5 = Point::from_f64(0.5, 0.5, 0.0);
        let v6 = Point::from_f64(0.5, 0.6, 0.0);
        let v7 = Point::from_f64(0.6, 0.6, 0.0);

        let inner_contour = Contour::new(vec![
            linear_edge(v5.clone(), v6.clone()),
            linear_edge(v6.clone(), v7.clone()),
            linear_edge(v7.clone(), v5.clone()),
        ]);

        let surface = Rc::new(Surface::Plane(Plane::new(
            Point::from_f64(0.0, 0.0, 0.0),
            Point::from_f64(1.0, 0.0, 0.0),
            Point::from_f64(0.0, 1.0, 0.0),
        )));

        // Loop shifted by 0.1 in x and y direction

        let face1 = Face::new(
            vec![contour.clone(), inner_contour.clone()],
            surface.clone(),
        );
        let face2 = face1.transform(Transform::from_translation(Point::from_f64(0.2, 0.2, 0.0)));

        let union_face = face_face_difference(&face2, &face1)[1].clone();

        let _object = extrude(union_face.clone(), Point::from_f64(0.0, 0.0, -0.5));

        let mut sphere = primitive_sphere(Point::zero(), EFloat64::one());
        sphere.boundaries.push(Contour::new(vec![primitive_circle(
            Point::zero(),
            Point::from_f64(0.5, 0.5, 0.5),
            EFloat64::one(),
        )]));

        let p1 = Point::from_f64(-1.0, 0.0, 1.0);
        let p2 = Point::from_f64(-1.0, 0.0, -1.0);
        let p3 = Point::from_f64(1.0, 0.0, -1.0);
        let p4 = Point::from_f64(1.0, 0.0, 1.0);

        let mut edges = Vec::new();
        for (p1, p2) in &[(p1, p2), (p2, p3), (p3, p4)] {
            edges.push(primitive_line(*p1, *p2).unwrap());
        }
        edges.push(primitive_arc(p4, p1, EFloat64::from(1.6), -Point::unit_y()));

        let hole = primitive_circle(
            Point::from_f64(0.0, 0.0, 0.2),
            Point::unit_y(),
            EFloat64::from(0.3),
        );

        let hole2 = primitive_rectangle_curve(
            Point::from_f64(0.0, 0.0, -0.5),
            Point::unit_x() * EFloat64::from(0.5),
            -Point::unit_z() * EFloat64::from(0.1),
        );

        let face1 = Face::new(
            vec![Contour::new(edges), Contour::new(vec![hole]), hole2],
            Rc::new(Surface::Plane(Plane::new(
                Point::zero(),
                Point::unit_x(),
                Point::unit_z(),
            ))),
        );

        let shell = extrude(face1, Point::unit_y());

        let mut triangles = TriangleBuffer::empty();
        let mut lines = EdgeBuffer::empty();
        let mut points = VertexBuffer::empty();

        // triangles.join(&rasterize_volume_into_triangle_list(
        //     &object,
        //     Color::new(1.0, 1.0, 1.0, 1.0),
        // ));
        // lines.join(&rasterize_volume_into_line_list(
        //     &object,
        //     Color::from_brightness(0.3),
        // ));
        // points.join(&rasterize_volume_into_vertex_list(
        //     &object,
        //     Color::from_brightness(0.1),
        // ));

        // triangles.join(&rasterize_face_into_triangle_list(&sphere, Color::white()));

        triangles.join(&rasterize_volume_into_triangle_list(
            &shell,
            Color::light_gray(),
        ));
        lines.join(&rasterize_volume_into_line_list(
            &shell,
            Color::from_brightness(0.3),
        ));
        points.join(&rasterize_volume_into_vertex_list(
            &shell,
            Color::from_brightness(0.1),
        ));
        return (points, lines, triangles);
    });
    match result {
        Ok((points, lines, triangles)) => {
            let window = GeopWindow::new(points, lines, triangles, &window).await;
            window.show(event_loop);
        }
        Err(e) => {
            println!("Error: {:?}", e);

            // let debug_data = get_debug_data().unwrap();

            // let mut lines = EdgeBuffer::empty();
            // for (edge, debug_color) in debug_data.edges.iter() {
            //     lines.join(&rasterize_edge_into_line_list(edge, debug_color.to_color()));
            // }
            // println!("Lines: {:?}", lines);

            // let mut triangles = TriangleBuffer::empty();
            // for (face, debug_color) in debug_data.faces.iter() {
            //     triangles.join(&rasterize_face_into_triangle_list(
            //         face,
            //         debug_color.to_color(),
            //     ));
            // }

            // let window = GeopWindow::new(VertexBuffer::empty(), lines, triangles, &window).await;
            // println!("Error: {:?}", e);
            // window.show(event_loop);
        }
    }
}

fn main() {
    pollster::block_on(run());
}
