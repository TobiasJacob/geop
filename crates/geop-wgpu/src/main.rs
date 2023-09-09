use std::rc::Rc;

use geop_geometry::{curves::{line::Line, circle::Circle}, points::point::Point, EQ_THRESHOLD, surfaces::plane::Plane};
use geop_rasterize::{contour::{rasterize_contour_into_line_list, rasterize_contours_into_line_list}, vertex_buffer::{VertexBuffer, RenderVertex}, triangle_buffer::{TriangleBuffer, RenderTriangle}, face::rasterize_face_into_triangle_list};
use geop_topology::topology::{contour::Contour, edge::{Edge, EdgeCurve, Direction}, vertex::Vertex, face::{Face, FaceSurface}};
use geop_wgpu::window::GeopWindow;


pub fn linear_edge(s: Vertex, e: Vertex) -> Rc<Edge> {
    let p1 = *s.point;
    let p2 = *e.point;
    Rc::new(Edge::new(s, e, Rc::new(EdgeCurve::Line(Line::new(p1, p2 - p1))), Direction::Increasing))
}

pub fn circular_edge(s: Vertex, e: Vertex, center: Point) -> Rc<Edge> {
    assert!((*s.point - center).norm_sq() - (*e.point - center).norm_sq() < EQ_THRESHOLD, "Circular edge must have same distance to center point");
    let point = *s.point;
    Rc::new(Edge::new(s, e, Rc::new(EdgeCurve::Circle(Circle::new(center, Point::new(0.0, 0.0, 1.0), point - center))), Direction::Increasing))
}

async fn run() {
    let v1 = Vertex::new(Rc::new(Point::new(0.2, 0.2, 0.0)));
    let v2 = Vertex::new(Rc::new(Point::new(0.8, 0.2, 0.0)));
    let v3 = Vertex::new(Rc::new(Point::new(0.8, 0.8, 0.0)));
    let v4: Vertex = Vertex::new(Rc::new(Point::new(0.2, 0.8, 0.0)));

    let contour = Contour::new(vec![
        linear_edge(v1.clone(), v2.clone()),
        linear_edge(v2.clone(), v3.clone()),
        linear_edge(v3.clone(), v4.clone()),
        // circular_edge(v4.clone(), v6.clone(), *v5.point),
        linear_edge(v4.clone(), v1.clone()),
    ]);

    let v5 = Vertex::new(Rc::new(Point::new(0.5, 0.5, 0.0)));
    let v6 = Vertex::new(Rc::new(Point::new(0.5, 0.6, 0.0)));
    let v7 = Vertex::new(Rc::new(Point::new(0.6, 0.6, 0.0)));

    let inner_contour = Contour::new(vec![
        linear_edge(v5.clone(), v6.clone()),
        linear_edge(v6.clone(), v7.clone()),
        linear_edge(v7.clone(), v5.clone()),
    ]);


    let surface =Rc::new(FaceSurface::Plane(Plane::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0), Point::new(0.0, 1.0, 0.0))));

    let face1 = Face::new(vec![contour.clone(), inner_contour.clone()], surface.clone());

    // Loop shifted by 0.1 in x and y direction
    let shift = Point::new(-0.0, 0.2, 0.0);
    let contour_shifted = Contour::new(vec![
        linear_edge(Vertex::new(Rc::new(*v1.point + shift)), Vertex::new(Rc::new(*v2.point + shift))),
        linear_edge(Vertex::new(Rc::new(*v2.point + shift)), Vertex::new(Rc::new(*v3.point + shift))),
        linear_edge(Vertex::new(Rc::new(*v3.point + shift)), Vertex::new(Rc::new(*v4.point + shift))),
        // circular_edge(Vertex::new(Rc::new(*v4.point + shift)), Vertex::new(Rc::new(*v6.point + shift)), *v5.point + shift),
        linear_edge(Vertex::new(Rc::new(*v4.point + shift)), Vertex::new(Rc::new(*v1.point + shift))),
    ]);

    let face2 = Face::new(vec![contour_shifted.clone()], surface.clone());

    // let union_face = face1.surface_difference(&face2);


    // let vertex_buffer_line = rasterize_contours_into_line_list(
    //     &union_face.boundaries,
    //     [1.0, 1.0, 1.0]
    // );
    let vertex_buffer_triange = TriangleBuffer::new(vec![RenderTriangle::new(
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
        Point::new(0.0, 1.0, 0.0),
        [1.0, 1.0, 0.0]
    )]);
    let vertex_buffer_triange = rasterize_face_into_triangle_list(&face1, [0.0, 1.0, 0.0]);
    let vertex_buffer_triange_line = vertex_buffer_triange.to_line_list([1.0, 1.0, 1.0]);
    let window = GeopWindow::new(vertex_buffer_triange_line, vertex_buffer_triange).await;
    window.show();
}

fn main() {
    pollster::block_on(run());
}