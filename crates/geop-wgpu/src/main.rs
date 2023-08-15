use std::rc::Rc;

use geop_geometry::{curves::{line::Line, circle::Circle}, points::point::Point, EQ_THRESHOLD};
use geop_rasterize::edge_loop::{rasterize_edge_loop_into_line_list, rasterize_edge_loops_into_line_list};
use geop_topology::topology::{edge::{edge_loop::EdgeLoop, edge::{Edge, EdgeCurve, Direction}}, vertex::Vertex};
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
    let v4: Vertex = Vertex::new(Rc::new(Point::new(0.4, 0.8, 0.0)));
    let v5 = Vertex::new(Rc::new(Point::new(0.4, 0.6, 0.0)));
    let v6 = Vertex::new(Rc::new(Point::new(0.2, 0.6, 0.0)));

    let edge_loop = EdgeLoop::new(vec![
        linear_edge(v1.clone(), v2.clone()),
        linear_edge(v2.clone(), v3.clone()),
        linear_edge(v3.clone(), v4.clone()),
        // circular_edge(v4.clone(), v6.clone(), *v5.point),
        linear_edge(v4.clone(), v1.clone()),
    ]);

    // Loop shifted by 0.1 in x and y direction
    let shift = Point::new(0.1, 0.1, 0.0);
    let edge_loop_shifted = EdgeLoop::new(vec![
        linear_edge(Vertex::new(Rc::new(*v1.point + shift)), Vertex::new(Rc::new(*v2.point + shift))),
        linear_edge(Vertex::new(Rc::new(*v2.point + shift)), Vertex::new(Rc::new(*v3.point + shift))),
        linear_edge(Vertex::new(Rc::new(*v3.point + shift)), Vertex::new(Rc::new(*v4.point + shift))),
        // circular_edge(Vertex::new(Rc::new(*v4.point + shift)), Vertex::new(Rc::new(*v6.point + shift)), *v5.point + shift),
        linear_edge(Vertex::new(Rc::new(*v4.point + shift)), Vertex::new(Rc::new(*v1.point + shift))),
    ]);

    let remesh = edge_loop.remesh_self_other(&edge_loop_shifted).unwrap();

    let vertex_buffer = rasterize_edge_loops_into_line_list(
        &remesh.as_slice()[0..1],
        [1.0, 1.0, 1.0]
    );
    let window = GeopWindow::new(vertex_buffer).await;
    window.show();
}

fn main() {
    pollster::block_on(run());
}