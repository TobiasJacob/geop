use geop_algebra::{efloat::EFloat64, MultiDimensionFunction, OneDimensionFunction};
use geop_geometry::point::Point;
use geop_topology::topology::scene::Color;

use crate::edge_buffer::{EdgeBuffer, RenderEdge};

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
) -> EdgeBuffer {
    let n = 100;
    let mut edges = Vec::<RenderEdge>::with_capacity(n);
    for j in 0..n {
        let v1 = (j as f64) / n as f64;
        let v2 = ((j + 1) as f64) / n as f64;
        edges.push(RenderEdge::new(
            Point::unit_x() * EFloat64::from(v1) + Point::unit_z() * edge.eval(EFloat64::from(v1)),
            Point::unit_x() * EFloat64::from(v2) + Point::unit_z() * edge.eval(EFloat64::from(v2)),
            color,
        ));
    }
    EdgeBuffer::new(edges)
}

pub fn rasterize_onedimensional_function(
    edge: &impl OneDimensionFunction,
    color: Color,
) -> EdgeBuffer {
    let n = 100;
    let mut edges = Vec::<RenderEdge>::with_capacity(n);
    for j in 0..n {
        let v1 = (j as f64) / n as f64;
        let v2 = ((j + 1) as f64) / n as f64;
        edges.push(RenderEdge::new(
            Point::unit_x() * EFloat64::from(v1) + Point::unit_y() * edge.eval(EFloat64::from(v1)),
            Point::unit_x() * EFloat64::from(v2) + Point::unit_y() * edge.eval(EFloat64::from(v2)),
            color,
        ));
    }
    EdgeBuffer::new(edges)
}
