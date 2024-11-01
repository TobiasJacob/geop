use geop_geometry::point::Point;

use crate::{
    primitive_objects::edges::line::primitive_line,
    topology::{contour::Contour, edge::Edge},
};

pub fn primitive_rectangle_curve(center: Point, u_dir: Point, v_dir: Point) -> Contour {
    let mut points = Vec::<Point>::new();
    points.push(center + u_dir + v_dir);
    points.push(center - u_dir + v_dir);
    points.push(center - u_dir - v_dir);
    points.push(center + u_dir - v_dir);

    let mut edges = Vec::<Edge>::new();
    for i in 0..points.len() {
        edges.push(primitive_line(points[i], points[(i + 1) % points.len()]));
    }

    Contour::new(edges)
}
