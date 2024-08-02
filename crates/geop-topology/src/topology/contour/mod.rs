use std::fmt::Display;

use geop_geometry::{points::point::Point, transforms::Transform};

use crate::contains::{
    contour_point::contour_point_contains,
    edge_point::{edge_point_contains, EdgePointContains},
};

use super::edge::Edge;

#[derive(Clone, Debug, PartialEq)]
pub enum EdgeIndex {
    OnEdge(usize),
    OnCorner(usize, usize),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ContourTangent {
    OnEdge(Point),
    OnCorner(Point, Point), // Ingoung and outgoing tangent
}

impl ContourTangent {
    pub fn expect_on_edge(&self) -> &Point {
        match self {
            ContourTangent::OnEdge(t) => t,
            ContourTangent::OnCorner(_, _) => panic!("Expected on edge"),
        }
    }
    pub fn expect_on_corner(&self) -> (&Point, &Point) {
        match self {
            ContourTangent::OnEdge(_) => panic!("Expected on corner"),
            ContourTangent::OnCorner(t1, t2) => (t1, t2),
        }
    }
    pub fn is_inside(&self, normal: Point, curve_dir: Point) -> bool {
        let (tangent1, tangent2) = match self {
            ContourTangent::OnEdge(tangent) => (tangent, tangent),
            ContourTangent::OnCorner(tangent1, tangent2) => (tangent1, tangent2),
        };
        // Check sign of det(tangent1 - curve_dir, tangent2 - curve_dir, normal - curve_dir)
        let curve_dir = -curve_dir.normalize();
        let tangent1 = -tangent1.normalize();
        let tangent2 = tangent2.normalize();
        let det = (tangent1 - curve_dir)
            .cross(tangent2 - curve_dir)
            .dot(normal - curve_dir);
        det > 0.0
    }
}

#[derive(Debug, Clone)]
pub struct Contour {
    pub edges: Vec<Edge>,
}

// An Contour is a closed loop of edges which is not self intersecting (because otherwise project would not be defined for self intersection point).
// It has a defined inside and outside, which is determined by the direction of the edges.
// The points of edges are not part of the contour, e.g. the intersection of two contours at the same point is empty.
// Keep in mind that the contour is still closed, but the points are "next to" the edges, not "part of" the edges, because otherwise two neighbouring edges would overlap at the point, making things a lot more complicated.
impl Contour {
    pub fn new(edges: Vec<Edge>) -> Contour {
        assert!(edges.len() > 0);
        for i in 0..edges.len() {
            let edge = edges[i].clone();
            let next_edge = edges[(i + 1) % edges.len()].clone();
            assert!(edge.end == next_edge.start);
        }
        assert!(edges[0].start == edges[edges.len() - 1].end);
        Contour { edges }
    }

    pub fn all_points(&self) -> Vec<Point> {
        let mut points = Vec::<Point>::new();
        for edge in self.edges.iter() {
            if let Some(start) = edge.start {
                points.push(start);
            }
        }
        return points;
    }

    pub fn flip(&self) -> Contour {
        let edges = self
            .edges
            .iter()
            .rev()
            .map(|e| e.flip())
            .collect::<Vec<Edge>>();
        Contour::new(edges)
    }

    pub fn transform(&self, transform: Transform) -> Contour {
        let edges = self
            .edges
            .iter()
            .map(|e| e.transform(transform))
            .collect::<Vec<Edge>>();
        Contour::new(edges)
    }

    // Returns an edge that contains the point, or None if the point is not on the contour.
    // It can also be the start or the end point of an edge, hence, if this function is used, take special care of the case where this case.
    fn get_edge_index(&self, point: Point) -> EdgeIndex {
        for (i, edge) in self.edges.iter().enumerate() {
            match edge_point_contains(edge, point) {
                EdgePointContains::Inside => {
                    return EdgeIndex::OnEdge(i);
                }
                EdgePointContains::OnPoint(p) => match Some(p) == edge.end {
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
        assert!(contour_point_contains(&self, p) != EdgePointContains::Outside);
        match self.get_edge_index(p) {
            EdgeIndex::OnCorner(i1, i2) => {
                // Tangent of i1 is incoming, tangent of i2 is outgoing
                ContourTangent::OnCorner(self.edges[i1].tangent(p), self.edges[i2].tangent(p))
            }
            EdgeIndex::OnEdge(i) => ContourTangent::OnEdge(self.edges[i].tangent(p)),
        }
    }

    // Gets the subcurve between these two points. It is guaranteed that there will be no zero length edges.
    pub fn get_subcurve(&self, start: Point, end: Point) -> Vec<Edge> {
        assert!(start != end);

        let mut result = Vec::<Edge>::new();
        let start_i = match self.get_edge_index(start) {
            EdgeIndex::OnEdge(i) => i,
            EdgeIndex::OnCorner(_i1, i2) => i2,
        };
        let end_i = match self.get_edge_index(end) {
            EdgeIndex::OnEdge(i) => i,
            EdgeIndex::OnCorner(i1, _i2) => i1,
        };

        if start_i == end_i {
            // Check if end comes before start, otherwise we have to go all the way around
            if self.edges[start_i]
                .curve
                .between(start, self.edges[start_i].start, Some(end))
            {
                result.push(Edge::new(
                    Some(start.clone()),
                    Some(end.clone()),
                    self.edges[start_i].curve.clone(),
                ));
                return result;
            }
        }

        let mut edge = &self.edges[start_i];
        if Some(start) != edge.end {
            result.push(Edge::new(
                Some(start.clone()),
                edge.end.clone(),
                edge.curve.clone(),
            ));
        }
        for i in start_i + 1..end_i {
            edge = &self.edges[i % self.edges.len()];
            result.push(edge.clone());
        }
        edge = &self.edges[end_i % self.edges.len()];
        if edge.start != Some(end) {
            result.push(Edge::new(
                edge.start.clone(),
                Some(end.clone()),
                edge.curve.clone(),
            ));
        }
        result
    }

    pub fn get_subcurve_single_point(&self, point: Point) -> Vec<Edge> {
        let i = match self.get_edge_index(point) {
            EdgeIndex::OnEdge(i) => i,
            EdgeIndex::OnCorner(i, _) => i,
        };
        let mut result = Vec::<Edge>::new();
        result.push(Edge::new(
            Some(point.clone()),
            self.edges[i].end.clone(),
            self.edges[i].curve.clone(),
        ));
        for j in 1..(self.edges.len() - 1) {
            let edge = self.edges[(i + j) % self.edges.len()].clone();
            if edge.end == Some(point) {
                result.push(edge);
                break;
            }
            result.push(edge);
        }
        result.push(Edge::new(
            self.edges[(i + self.edges.len() - 1) % self.edges.len()]
                .start
                .clone(),
            Some(point.clone()),
            self.edges[(i + self.edges.len() - 1) % self.edges.len()]
                .curve
                .clone(),
        ));

        result
    }
}

impl Display for Contour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Contour: ")?;
        for edge in self.edges.iter() {
            write!(f, "{:?} ", edge.start)?;
        }
        Ok(())
    }
}
