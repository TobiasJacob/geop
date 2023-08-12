use std::rc::Rc;

use geop_geometry::{curves::line::Line, points::point::Point};
use geop_topology::topology::{edge::{edge_loop::EdgeLoop, edge::{Edge, EdgeCurve}}, vertex::Vertex};
use geop_wgpu::window::GeopWindow;


pub fn linear_edge(s: Vertex, e: Vertex) -> Rc<Edge> {
    let p1 = *s.point;
    let p2 = *e.point;
    Rc::new(Edge::new(s, e, Rc::new(EdgeCurve::Line(Line::new(p1, p2 - p1)))))
}

async fn run() {
    let v1 = Vertex::new(Rc::new(Point::new(0.2, 0.2, 0.0)));
    let v2 = Vertex::new(Rc::new(Point::new(0.8, 0.2, 0.0)));
    let v3 = Vertex::new(Rc::new(Point::new(0.8, 0.8, 0.0)));
    let v4 = Vertex::new(Rc::new(Point::new(0.2, 0.8, 0.0)));

    let edge_loop = EdgeLoop::new(vec![
        linear_edge(v1.clone(), v2.clone()),
        linear_edge(v2.clone(), v3.clone()),
        linear_edge(v3.clone(), v4.clone()),
        linear_edge(v4.clone(), v1.clone()),
    ]);

    let window = GeopWindow::new(edge_loop).await;
    window.show();
}

fn main() {
    pollster::block_on(run());
}