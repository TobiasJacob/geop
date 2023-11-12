use std::rc::Rc;

use geop_geometry::{points::point::Point, transforms::Transform};

use super::edge::{Edge, EdgeContains, EdgeIntersection};

pub enum ContourCorner<T> {
    OnEdge(T),
    OnCorner(T, T)
}

impl<T> ContourCorner<T> {
    pub fn expect_on_edge(&self) -> &T {
        match self {
            ContourCorner::OnEdge(t) => t,
            ContourCorner::OnCorner(_, _) => panic!("Expected on edge"),
        }
    }
}

impl ContourCorner<Point> {
    pub fn is_inside(&self, normal: Point, curve_dir: Point) -> bool {
        match self {
            ContourCorner::OnEdge(tangent) => { tangent.cross(normal).dot(curve_dir) > 0.0 }
            ContourCorner::OnCorner(tangent1, tangent2) => { 
                tangent1.cross(normal).dot(curve_dir) > 0.0 &&
                tangent2.cross(normal).dot(curve_dir) > 0.0
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
            let contains = edge.contains(point);
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
    fn get_edge_index(&self, point: Point) -> ContourCorner<usize> {
        for (i, edge) in self.edges.iter().enumerate() {
            match edge.contains(point) {
                EdgeContains::Inside => { return ContourCorner::<usize>::OnEdge(i);}
                EdgeContains::OnPoint(p) => {
                    match p == edge.end {
                        true => { return ContourCorner::<usize>::OnCorner(i, (i + 1) % self.edges.len()) }
                        false => { panic!("Checks are in order, so this case should have been detected in the previous iteration.")}
                    }
                },
                EdgeContains::Outside => {}
            }
        }
        panic!("Not on contour")
    }

    pub fn tangent(&self, p: Point) -> ContourCorner<Point> {
        match self.get_edge_index(p) {
            ContourCorner::OnCorner(i1, i2) => {
                ContourCorner::<Point>::OnCorner(
                    self.edges[i1].tangent(p),
                    self.edges[i2].tangent(p)
                )
            },
            ContourCorner::OnEdge(i) => {
                ContourCorner::<Point>::OnEdge(self.edges[i].tangent(p))
            },
        }
    }

    // Checks if the contour contains the point, and if so, splits the edge into two edges.
    // It is guaranteed that this happens in order, meaning that the first edge returned will contain the start point of the original edge, and the second edge will contain the end point of the original edge.
    pub fn split_if_necessary(&self, other: &Point) -> Contour {
        if self.contains(*other) != EdgeContains::Inside {
            return self.clone();
        }

        let edge_index = match self.get_edge_index(*other) {
            ContourCorner::OnEdge(i) => { i }
            ContourCorner::OnCorner(i1, i2) => { return self.clone(); }
        };
        let edge = self.edges[edge_index].split_if_necessary(other);
        assert!(edge.len() == 2);

        let mut edges = self.edges.clone();
        edges.remove(edge_index);
        edges.insert(edge_index, edge[1].clone());
        edges.insert(edge_index, edge[0].clone());

        return Contour::new(edges);
    }

    pub fn split_edges_if_necessary(&self, other: Vec<Rc<Edge>>) -> Vec<Rc<Edge>> {
        todo!();
    }

    // Gets the subcurve between these two points. It is guaranteed that there will be no zero length edges.
    pub fn get_subcurve(&self, start: Rc<Point>, end: Rc<Point>) -> Vec<Rc<Edge>> {
        assert!(start != end);

        let mut result = Vec::<Rc<Edge>>::new();
        let start_i = match self.get_edge_index(*start) {
            ContourCorner::OnEdge(i) => { i }
            ContourCorner::OnCorner(i1, i2) => { i1 }
        };
        let end_i = match self.get_edge_index(*end) {
            ContourCorner::OnEdge(i) => { i }
            ContourCorner::OnCorner(i1, i2) => { i1 }
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

    pub fn intersect_edge(&self, other: &Edge) -> Vec<EdgeIntersection> {
        let mut intersections = Vec::<EdgeIntersection>::new();
        for edge in self.edges.iter() {
            let edge_intersections = edge.intersections(other);
            intersections.extend(edge_intersections.into_iter());
        }

        intersections
    }

    // Gets all intersections between this contour and another contour.
    // Vertices of Edges are not considered as part of the contour, hence, the intersection of two contours at the same point is empty.
    pub fn intersect_contour(&self, other: &Contour) -> Vec<EdgeIntersection> {
        let mut intersections = Vec::<EdgeIntersection>::new();
        for edge_other in other.edges.iter() {
            intersections.extend(self.intersect_edge(edge_other).into_iter());
        }
        intersections
    }

    // Avoid using these functions
    pub fn point_at(&self, u: f64) -> Point {
        let u = u * self.edges.len() as f64;
        let i = u.floor() as usize;
        let u = u - i as f64;
        let i = i % self.edges.len();
        return self.edges[i].point_at(u);
    }

    // Avoid using these functions
    pub fn project(&self, point: Point) -> Option<f64> {
        for (i, edge) in self.edges.iter().enumerate() {
            let u = edge.project(point);
            match u {
                Some(u) => {
                    return Some((i as f64 + u) / self.edges.len() as f64);
                }
                None => {}
            }
        }
        None
    }
}
