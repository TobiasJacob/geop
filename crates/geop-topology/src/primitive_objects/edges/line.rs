use geop_geometry::{
    color::Category10Color,
    curves::{curve::Curve, line::Line},
    geometry_error::{ElevateToGeometry, GeometryError},
    point::Point,
};

use crate::{
    topology::edge::Edge,
    topology_error::{ElevateToTopology, TopologyError, TopologyResult},
    topology_scene::TopologyScene,
};

pub fn primitive_line(start: Point, end: Point) -> TopologyResult<Edge> {
    let context = |err: TopologyError| {
        err.with_context_scene(
            format!("Creating primitive line from {} to {}", start, end),
            TopologyScene::with_points(vec![
                (start, Category10Color::Blue),
                (end, Category10Color::Orange),
            ]),
        )
    };
    let context2 = |err: GeometryError| {
        GeometryError::from(err).with_context("Normalize direction of new line".to_string())
    };
    let direction = (end - start)
        .normalize()
        .elevate(&context2)
        .elevate(&context)?;
    // .map_err(|err| {
    //     TopologyError::from_geometry_error(err)
    //         .with_context("Normalize direction of new line".to_string())
    // })
    // .with_context(&context)?;
    let l = Line::new(start, direction).elevate(&context)?;
    Ok(Edge::new(Some(start), Some(end), Curve::Line(l)))
}

pub fn primitive_infinite_line(p1: Point, p2: Point) -> Edge {
    let l = Line::new(p1, (p2 - p1).normalize().unwrap()).unwrap();
    Edge::new(None, None, Curve::Line(l))
}
