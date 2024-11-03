use geop_geometry::{
    curves::{curve::Curve, line::Line},
    point::Point,
};

use crate::{
    topology::edge::Edge,
    topology_error::{TopologyError, TopologyResult},
};

pub fn primitive_line(start: Point, end: Point) -> TopologyResult<Edge> {
    let l = Line::new(start, (end - start).normalize().unwrap()).map_err(|e| {
        TopologyError::from(e).with_context(format!("Create linear edge from {} to {}", start, end))
    })?;
    Ok(Edge::new(Some(start), Some(end), Curve::Line(l)))
}

pub fn primitive_infinite_line(p1: Point, p2: Point) -> Edge {
    let l = Line::new(p1, (p2 - p1).normalize().unwrap()).unwrap();
    Edge::new(None, None, Curve::Line(l))
}
