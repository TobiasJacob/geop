use geop_geometry::{curves::CurveLike, point::Point};

use crate::topology::edge::Edge;

#[derive(Clone, Debug, PartialEq)]
pub enum EdgePointContains {
    Inside,
    Outside,
    OnPoint(Point),
}

pub fn edge_point_contains(edge: &Edge, point: Point) -> EdgePointContains {
    if !edge.curve.on_curve(point) {
        return EdgePointContains::Outside;
    }
    if point == edge.bounds.start || point == edge.bounds.end {
        return EdgePointContains::OnPoint(point);
    }
    if edge.curve.between(point, &edge.bounds).unwrap() {
        return EdgePointContains::Inside;
    }
    EdgePointContains::Outside
}

#[cfg(test)]
mod tests {
    use crate::primitive_objects::edges::line::primitive_line;
    // use crate::primitive_objects::volumes::cube::primitive_cube;
    // use crate::topology::scene::Scene;

    use super::*;
    use geop_geometry::point::Point;

    #[test]
    fn test_edge_point_contains() {
        let edge = primitive_line(
            Point::from_f64(0.0, 0.0, 0.0),
            Point::from_f64(1.0, 0.0, 0.0),
        )
        .unwrap();
        assert_eq!(
            edge_point_contains(&edge, Point::from_f64(0.5, 0.0, 0.0)),
            EdgePointContains::Inside
        );
        assert_eq!(
            edge_point_contains(&edge, Point::from_f64(0.0, 0.0, 0.0)),
            EdgePointContains::OnPoint(Point::from_f64(0.0, 0.0, 0.0))
        );
        assert_eq!(
            edge_point_contains(&edge, Point::from_f64(1.0, 0.0, 0.0)),
            EdgePointContains::OnPoint(Point::from_f64(1.0, 0.0, 0.0))
        );
        assert_eq!(
            edge_point_contains(&edge, Point::from_f64(1.5, 0.0, 0.0)),
            EdgePointContains::Outside
        );
    }

    // use geop_wgpu::headless_renderer::tests::renderer;
    // use geop_wgpu::headless_renderer::HeadlessRenderer;
    // use rstest::rstest;

    // #[rstest]
    // async fn test_headless_renderer_dark(#[future] renderer: Box<HeadlessRenderer>) {
    //     let volume = primitive_cube(1.0, 1.0, 1.0);
    //     let scene = Scene::new(vec![volume], vec![], vec![], vec![]);
    //     renderer
    //         .await
    //         .render_to_file(&scene, true, std::path::Path::new("test_dark3.png"))
    //         .await;
    // }
}
