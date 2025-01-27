use geop_algebra::{efloat::EFloat64, MultiDimensionFunction, OneDimensionFunction};
use geop_geometry::{color::Category10Color, point::Point};
use geop_topology::topology::scene::Color;

use crate::edge_buffer::{EdgeBuffer, RenderEdge};

pub fn rasterize_coordinate_system(min: Point, max: Point, ticks: Point) -> EdgeBuffer {
    let mut edges = Vec::<RenderEdge>::new();

    edges.push(RenderEdge::new(
        Point::new(min.x, EFloat64::zero(), EFloat64::zero()),
        Point::new(max.x, EFloat64::zero(), EFloat64::zero()),
        Color::from_category_color(Category10Color::Red),
    ));
    let mut x = min.x;
    while x <= max.x {
        edges.push(RenderEdge::new(
            Point::new(x, -ticks.x * EFloat64::from(0.2), EFloat64::zero()),
            Point::new(x, ticks.x * EFloat64::from(0.2), EFloat64::zero()),
            Color::from_category_color(Category10Color::Red),
        ));
        x = x + ticks.x;
    }

    edges.push(RenderEdge::new(
        Point::new(EFloat64::zero(), min.y, EFloat64::zero()),
        Point::new(EFloat64::zero(), max.y, EFloat64::zero()),
        Color::from_category_color(Category10Color::Green),
    ));
    let mut y = min.y;
    while y <= max.y {
        edges.push(RenderEdge::new(
            Point::new(-ticks.y * EFloat64::from(0.2), y, EFloat64::zero()),
            Point::new(ticks.y * EFloat64::from(0.2), y, EFloat64::zero()),
            Color::from_category_color(Category10Color::Green),
        ));
        y = y + ticks.y;
    }

    EdgeBuffer::new(edges)
}

// Rasterizes an edge loop into triangle list.
pub fn rasterize_multidimensional_function(
    edge: &impl MultiDimensionFunction<Point>,
    color: Color,
) -> EdgeBuffer {
    let n = 100;
    let mut edges = Vec::<RenderEdge>::with_capacity(n);
    for j in 0..n {
        let v1 = (j as f64) / n as f64;
        let v2 = ((j + 1) as f64) / n as f64;
        edges.push(RenderEdge::new(
            edge.eval(EFloat64::from(v1)),
            edge.eval(EFloat64::from(v2)),
            color,
        ));
    }
    EdgeBuffer::new(edges)
}

pub fn rasterize_multidimensional_function_in_1d(
    edge: &impl MultiDimensionFunction<EFloat64>,
    color: Color,
    x_min: f64,
    x_max: f64,
) -> EdgeBuffer {
    let n = 100;
    let mut edges = Vec::<RenderEdge>::with_capacity(n);
    for j in 0..n {
        let v1 = (j as f64) / n as f64 * (x_max - x_min) + x_min;
        let v2 = ((j + 1) as f64) / n as f64 * (x_max - x_min) + x_min;
        edges.push(RenderEdge::new(
            Point::unit_x() * EFloat64::from(v1) + Point::unit_y() * edge.eval(EFloat64::from(v1)),
            Point::unit_x() * EFloat64::from(v2) + Point::unit_y() * edge.eval(EFloat64::from(v2)),
            color,
        ));
    }
    EdgeBuffer::new(edges)
}

pub fn rasterize_onedimensional_function(
    edge: &impl OneDimensionFunction,
    color: Color,
    x_min: f64,
    x_max: f64,
) -> EdgeBuffer {
    let n = 100;
    let mut edges = Vec::<RenderEdge>::with_capacity(n);
    for j in 0..n {
        let v1 = (j as f64) / n as f64 * (x_max - x_min) + x_min;
        let v2 = ((j + 1) as f64) / n as f64 * (x_max - x_min) + x_min;
        edges.push(RenderEdge::new(
            Point::unit_x() * EFloat64::from(v1) + Point::unit_y() * edge.eval(EFloat64::from(v1)),
            Point::unit_x() * EFloat64::from(v2) + Point::unit_y() * edge.eval(EFloat64::from(v2)),
            color,
        ));
    }
    EdgeBuffer::new(edges)
}
