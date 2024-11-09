use std::fmt::Display;

use geop_geometry::{
    curves::{bounds::Bounds, CurveLike},
    point::Point,
    transforms::Transform,
};

use crate::{
    contains::edge_point::{edge_point_contains, EdgePointContains},
    topology::edge::Edge,
};

use super::contour::{ContourTangent, EdgeIndex};

#[derive(Debug, Clone)]
pub struct ContourMultiPoint {
    pub edges: Vec<Edge>,
}

impl ContourMultiPoint {
    pub fn new(edges: Vec<Edge>) -> ContourMultiPoint {
        assert!(edges.len() > 0);
        for i in 0..edges.len() {
            let edge = edges[i].clone();
            let next_edge = edges[(i + 1) % edges.len()].clone();
            assert!(edge.bounds.end == next_edge.bounds.start);
        }
        assert!(edges[0].bounds.start == edges[edges.len() - 1].bounds.end);
        ContourMultiPoint { edges }
    }

    pub fn all_points(&self) -> Vec<Point> {
        let mut points = Vec::<Point>::new();
        for edge in self.edges.iter() {
            points.push(edge.bounds.start);
        }
        return points;
    }

    pub fn flip(&self) -> ContourMultiPoint {
        let edges = self
            .edges
            .iter()
            .rev()
            .map(|e| e.flip())
            .collect::<Vec<Edge>>();
        ContourMultiPoint::new(edges)
    }

    pub fn transform(&self, transform: Transform) -> ContourMultiPoint {
        let edges = self
            .edges
            .iter()
            .map(|e| e.transform(transform))
            .collect::<Vec<Edge>>();
        ContourMultiPoint::new(edges)
    }

    // Returns an edge that contains the point, or None if the point is not on the contour.
    // It can also be the start or the end point of an edge, hence, if this function is used, take special care of the case where this case.
    fn get_edge_index(&self, point: Point) -> EdgeIndex {
        for (i, edge) in self.edges.iter().enumerate() {
            match edge_point_contains(edge, point) {
                EdgePointContains::Inside => {
                    return EdgeIndex::OnEdge(i);
                }
                EdgePointContains::OnPoint(p) => match p == edge.bounds.end {
                    true => return EdgeIndex::OnCorner(i, (i + 1) % self.edges.len()),
                    false => {
                        return EdgeIndex::OnCorner(
                            (i + self.edges.len() - 1) % self.edges.len(),
                            i,
                        )
                    }
                },
                EdgePointContains::Outside => {}
            }
        }
        panic!("Not on contour")
    }

    pub fn tangent(&self, p: Point) -> ContourTangent {
        // assert!(contour_point_contains(&self, p) != EdgePointContains::Outside);
        match self.get_edge_index(p) {
            EdgeIndex::OnCorner(i1, i2) => {
                // Tangent of i1 is incoming, tangent of i2 is outgoing
                ContourTangent::OnCorner(self.edges[i1].tangent(p), self.edges[i2].tangent(p))
            }
            EdgeIndex::OnEdge(i) => ContourTangent::OnEdge(self.edges[i].tangent(p)),
        }
    }

    pub fn get_midpoint(&self) -> Point {
        self.edges[0].get_midpoint()
    }

    // Gets the subcurve between these two points. It is guaranteed that there will be no zero length edges.
    pub fn get_subcurve(&self, start: Point, end: Point) -> Vec<Edge> {
        assert!(start != end);

        let mut result = Vec::<Edge>::new();
        let start_i = match self.get_edge_index(start) {
            EdgeIndex::OnEdge(i) => i,
            EdgeIndex::OnCorner(_i1, i2) => i2,
        };
        let mut end_i = match self.get_edge_index(end) {
            EdgeIndex::OnEdge(i) => i,
            EdgeIndex::OnCorner(i1, _i2) => i1,
        };

        if start_i == end_i {
            // Check if end comes before start, otherwise we have to go all the way around
            if self.edges[start_i]
                .curve
                .between(
                    start,
                    &Bounds::new(self.edges[start_i].bounds.start, end).unwrap(),
                )
                .unwrap()
            {
                result.push(Edge::new(
                    Bounds::new(start.clone(), end.clone()).unwrap(),
                    self.edges[start_i].curve.clone(),
                ));
                return result;
            }
        }

        if end_i <= start_i {
            end_i += self.edges.len();
        }

        let mut edge = &self.edges[start_i];
        if start != edge.bounds.end {
            result.push(Edge::new(
                Bounds::new(start.clone(), edge.bounds.end.clone()).unwrap(),
                edge.curve.clone(),
            ));
        }
        for i in start_i + 1..end_i {
            edge = &self.edges[i % self.edges.len()];
            result.push(edge.clone());
        }
        edge = &self.edges[end_i % self.edges.len()];
        if edge.bounds.start != end {
            result.push(Edge::new(
                Bounds::new(edge.bounds.start.clone(), end.clone()).unwrap(),
                edge.curve.clone(),
            ));
        }
        result
    }

    pub fn insert_point(&self, point: Point) -> ContourMultiPoint {
        let i = match self.get_edge_index(point) {
            EdgeIndex::OnEdge(i) => i,
            EdgeIndex::OnCorner(i, _) => i,
        };
        let mut result = Vec::<Edge>::new();
        if point != self.edges[i].bounds.end {
            result.push(Edge::new(
                Bounds::new(point.clone(), self.edges[i].bounds.end.clone()).unwrap(),
                self.edges[i].curve.clone(),
            ));
        }
        for j in 1..(self.edges.len() - 1) {
            let edge = self.edges[(i + j) % self.edges.len()].clone();
            if edge.bounds.end == point {
                result.push(edge);
                break;
            }
            result.push(edge);
        }
        if point
            != self.edges[(i + self.edges.len() - 1) % self.edges.len()]
                .bounds
                .start
        {
            result.push(Edge::new(
                Bounds::new(
                    self.edges[(i + self.edges.len() - 1) % self.edges.len()]
                        .bounds
                        .start
                        .clone(),
                    point.clone(),
                )
                .unwrap(),
                self.edges[(i + self.edges.len() - 1) % self.edges.len()]
                    .curve
                    .clone(),
            ));
        }
        ContourMultiPoint::new(result)
    }
}

impl Display for ContourMultiPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Contour: ")?;
        for edge in self.edges.iter() {
            write!(f, "{:?} ", edge.bounds.start)?;
        }
        Ok(())
    }
}
