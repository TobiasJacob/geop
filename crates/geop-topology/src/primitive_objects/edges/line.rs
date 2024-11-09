use geop_geometry::{
    curves::{bounds::Bounds, curve::Curve, line::Line},
    point::Point,
};

use crate::{
    topology::{contour_no_point::ContourNoPoint, edge::Edge},
    topology_error::{TopologyError, TopologyResult},
};

pub fn primitive_line(start: Point, end: Point) -> TopologyResult<Edge> {
    // TODO: Make sure start != end and then use epect and unwrap
    let l = Line::new(start, (end - start).normalize().unwrap()).map_err(|e| {
        TopologyError::from(e).with_context(format!("Create linear edge from {} to {}", start, end))
    })?;
    Ok(Edge::new(Bounds::new(start, end)?, Curve::Line(l)))
}

pub fn primitive_infinite_line(p1: Point, p2: Point) -> ContourNoPoint {
    let l = Line::new(p1, (p2 - p1).normalize().unwrap()).unwrap();
    ContourNoPoint::new(Curve::Line(l))
}
