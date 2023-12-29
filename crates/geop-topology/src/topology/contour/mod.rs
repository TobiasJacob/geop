use std::rc::Rc;

use geop_geometry::{points::point::Point, transforms::Transform};

use super::{edge::{Edge}, contains::edge_point::{EdgeContains, edge_contains_point}, intersections::edge_edge::EdgeEdgeIntersection};

#[derive(Clone, Debug, PartialEq)]
pub enum EdgeIndex {
    OnEdge(usize),
    OnCorner(usize, usize)
}

#[derive(Clone, Debug, PartialEq)]
pub enum ContourTangent {
    OnEdge(Point),
    OnCorner(Point, Point)
}

impl ContourTangent {
    pub fn expect_on_edge(&self) -> &Point {
        match self {
            ContourTangent::OnEdge(t) => t,
            ContourTangent::OnCorner(_, _) => panic!("Expected on edge"),
        }
    }

    pub fn is_inside(&self, normal: Point, curve_dir: Point) -> bool {
        match self {
            ContourTangent::OnEdge(tangent) => { tangent.cross(normal).dot(curve_dir) > 0.0 }
            ContourTangent::OnCorner(tangent1, tangent2) => { 
                // Determine if it's a sharp or dull corner
                let is_sharp = tangent1.cross(*tangent2).dot(normal) >= 0.0;

                if is_sharp {
                    // Sharp Corner: Check if normal is between tangent1 and tangent2
                    tangent1.cross(normal).dot(curve_dir) > 0.0 && tangent2.cross(normal).dot(curve_dir) > 0.0
                } else {
                    // Dull Corner: Check if normal is between tangent1 or tangent2
                    tangent1.cross(normal).dot(curve_dir) > 0.0 || tangent2.cross(normal).dot(curve_dir) > 0.0
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Contour {
    pub edges: Vec<Rc<Edge>>,
}

// An Contour is a closed loop of edges which is not self intersecting (because otherwise project would not be defined for self intersection point).
// It has a defined inside and outside, which is determined by the direction of the edges.
// The points of edges are not part of the contour, e.g. the intersection of two contours at the same point is empty.
// Keep in mind that the contour is still closed, but the points are "next to" the edges, not "part of" the edges, because otherwise two neighbouring edges would overlap at the point, making things a lot more complicated.
impl Contour {
    pub fn new(edges: Vec<Rc<Edge>>) -> Contour {
        assert!(edges.len() > 0);
        for i in 0..edges.len() {
            let edge = edges[i].clone();
            let next_edge = edges[(i + 1) % edges.len()].clone();
            assert!(edge.end == next_edge.start);
        }
        assert!(edges[0].start == edges[edges.len() - 1].end);
        Contour { edges }
    }

    pub fn all_points(&self) -> Vec<Rc<Point>> {
        let mut points = Vec::<Rc<Point>>::new();
        for edge in self.edges.iter() {
            points.push(edge.start.clone());
        }
        points.push(self.edges.last().unwrap().end.clone());
        return points;
    }

    pub fn neg(&self) -> Contour {
        let edges = self
            .edges
            .iter()
            .rev()
            .map(|e| Rc::new(e.neg()))
            .collect::<Vec<Rc<Edge>>>();
        Contour::new(edges)
    }

    pub fn transform(&self, transform: Transform) -> Contour {
        let edges = self
            .edges
            .iter()
            .map(|e| Rc::new(e.transform(transform)))
            .collect::<Vec<Rc<Edge>>>();
        Contour::new(edges)
    }

    pub fn contains(&self, point: Point) -> EdgeContains {
        for (_i, edge) in self.edges.iter().enumerate() {
            let contains: EdgeContains = edge_contains_point(edge, point);
            match contains {
                EdgeContains::OnPoint(point) => {
                    return EdgeContains::OnPoint(point);
                }
                EdgeContains::Inside => {
                    return EdgeContains::Inside;
                }
                EdgeContains::Outside => {}
            }
        }
        return EdgeContains::Outside;
    }

    // Returns an edge that contains the point, or None if the point is not on the contour.
    // It can also be the start or the end point of an edge, hence, if this function is used, take special care of the case where this case.
    fn get_edge_index(&self, point: Point) -> EdgeIndex {
        for (i, edge) in self.edges.iter().enumerate() {
            match edge_contains_point(edge, point) {
                EdgeContains::Inside => { return EdgeIndex::OnEdge(i);}
                EdgeContains::OnPoint(p) => {
                    match p == edge.end {
                        true => { return EdgeIndex::OnCorner(i, (i + 1) % self.edges.len()) }
                        false => { return EdgeIndex::OnCorner((i + self.edges.len() - 1) % self.edges.len(), i)}
                    }
                },
                EdgeContains::Outside => {}
            }
        }
        panic!("Not on contour")
    }

    pub fn tangent(&self, p: Point) -> ContourTangent {
        assert!(self.contains(p) != EdgeContains::Outside);
        match self.get_edge_index(p) {
            EdgeIndex::OnCorner(i1, i2) => {
                ContourTangent::OnCorner(self.edges[i1].tangent(p), self.edges[i2].tangent(p))
            },
            EdgeIndex::OnEdge(i) => {
                ContourTangent::OnEdge(self.edges[i].tangent(p))
            },
        }
    }

    // Gets the subcurve between these two points. It is guaranteed that there will be no zero length edges.
    pub fn get_subcurve(&self, start: Rc<Point>, end: Rc<Point>) -> Vec<Rc<Edge>> {
        assert!(start != end);

        let mut result = Vec::<Rc<Edge>>::new();
        let start_i = match self.get_edge_index(*start) {
            EdgeIndex::OnEdge(i) => { i }
            EdgeIndex::OnCorner(i1, i2) => { i2 }
        };
        let end_i = match self.get_edge_index(*end) {
            EdgeIndex::OnEdge(i) => { i }
            EdgeIndex::OnCorner(i1, i2) => { i1 }
        };

        if start_i == end_i {
            let edge = Edge::new(
                start.clone(),
                end.clone(),
                self.edges[start_i].curve.clone(),
            );
            result.push(Rc::new(edge));
        }

        let mut edge = &self.edges[start_i];
        if start != edge.end {
            result.push(Rc::new(Edge::new(
                start.clone(),
                edge.end.clone(),
                edge.curve.clone(),
            )));
        }
        for i in start_i + 1..end_i {
            edge = &self.edges[i % self.edges.len()];
            result.push(edge.clone());
        }
        edge = &self.edges[end_i % self.edges.len()];
        if edge.start != end {
            result.push(Rc::new(Edge::new(
                edge.start.clone(),
                end.clone(),
                edge.curve.clone(),
            )));
        }
        result
    }
}
