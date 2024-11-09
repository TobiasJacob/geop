use geop_geometry::{bounding_box::BoundingBox, point::Point};
use geop_topology::{primitive_objects::edges::line::primitive_line, topology::edge::Edge};

pub fn rasterize_boundingbox_into_edges(bounding_box: BoundingBox) -> Vec<Edge> {
    let mut result = Vec::new();
    let min = bounding_box.min;
    let max = bounding_box.max;

    let p1 = Point::new(min.x, min.y, min.z);
    let p2 = Point::new(max.x, min.y, min.z);
    if p1 != p2 {
        result.push(primitive_line(p1, p2).unwrap());
    }

    let p1 = Point::new(max.x, min.y, min.z);
    let p2 = Point::new(max.x, max.y, min.z);
    if p1 != p2 {
        result.push(primitive_line(p1, p2).unwrap());
    }

    let p1 = Point::new(max.x, max.y, min.z);
    let p2 = Point::new(min.x, max.y, min.z);
    if p1 != p2 {
        result.push(primitive_line(p1, p2).unwrap());
    }

    let p1 = Point::new(min.x, max.y, min.z);
    let p2 = Point::new(min.x, min.y, min.z);
    if p1 != p2 {
        result.push(primitive_line(p1, p2).unwrap());
    }

    let p1 = Point::new(min.x, min.y, max.z);
    let p2 = Point::new(max.x, min.y, max.z);
    if p1 != p2 {
        result.push(primitive_line(p1, p2).unwrap());
    }

    let p1 = Point::new(max.x, min.y, max.z);
    let p2 = Point::new(max.x, max.y, max.z);
    if p1 != p2 {
        result.push(primitive_line(p1, p2).unwrap());
    }

    let p1 = Point::new(max.x, max.y, max.z);
    let p2 = Point::new(min.x, max.y, max.z);
    if p1 != p2 {
        result.push(primitive_line(p1, p2).unwrap());
    }

    let p1 = Point::new(min.x, max.y, max.z);
    let p2 = Point::new(min.x, min.y, max.z);
    if p1 != p2 {
        result.push(primitive_line(p1, p2).unwrap());
    }

    let p1 = Point::new(min.x, min.y, min.z);
    let p2 = Point::new(min.x, min.y, max.z);
    if p1 != p2 {
        result.push(primitive_line(p1, p2).unwrap());
    }

    let p1 = Point::new(max.x, min.y, min.z);
    let p2 = Point::new(max.x, min.y, max.z);
    if p1 != p2 {
        result.push(primitive_line(p1, p2).unwrap());
    }

    let p1 = Point::new(max.x, max.y, min.z);
    let p2 = Point::new(max.x, max.y, max.z);
    if p1 != p2 {
        result.push(primitive_line(p1, p2).unwrap());
    }

    let p1 = Point::new(min.x, max.y, min.z);
    let p2 = Point::new(min.x, max.y, max.z);
    if p1 != p2 {
        result.push(primitive_line(p1, p2).unwrap());
    }

    result
}
