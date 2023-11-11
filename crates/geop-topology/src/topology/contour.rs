use std::rc::Rc;

use geop_geometry::{points::point::Point, transforms::Transform};

use super::edge::{Edge, EdgeContains, EdgeEdgeIntersection, self};

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
    fn get_edge_index(&self, point: Point) -> Option<usize> {
        for (i, edge) in self.edges.iter().enumerate() {
            if edge.contains(point) != EdgeContains::Outside {
                return Some(i);
            }
        }
        None
    }

    pub fn tangent(&self, p: Point) -> Point {
        assert!(self.contains(p) != EdgeContains::Outside);
        let i = self.get_edge_index(p).unwrap();
        return self.edges[i].tangent(p);
    }

    // Checks if the contour contains the point, and if so, splits the edge into two edges.
    // It is guaranteed that this happens in order, meaning that the first edge returned will contain the start point of the original edge, and the second edge will contain the end point of the original edge.
    pub fn split_if_necessary(&self, other: &Point) -> Contour {
        if self.contains(*other) != EdgeContains::Inside {
            return self.clone();
        }

        let edge_index = self.get_edge_index(*other).unwrap();
        let edge = self.edges[edge_index].split_if_necessary(other);
        assert!(edge.len() == 2);

        let mut edges = self.edges.clone();
        edges.remove(edge_index);
        edges.insert(edge_index, edge[1].clone());
        edges.insert(edge_index, edge[0].clone());

        return Contour::new(edges);
    }

    // Gets the subcurve between these two points. It is guaranteed that there will be no zero length edges.
    pub fn get_subcurve(&self, start: Rc<Point>, end: Rc<Point>) -> Vec<Rc<Edge>> {
        assert!(start != end);

        let mut result = Vec::<Rc<Edge>>::new();
        let start_i = self
            .get_edge_index(*start)
            .expect("Start point has to be on edge");
        let end_i = self
            .get_edge_index(*end)
            .expect("End point has to be on edge");

        if start_i == end_i {
            let edge = Edge::new(
                start.clone(),
                end.clone(),
                self.edges[start_i].curve.clone(),
                self.edges[start_i].direction,
            );
            result.push(Rc::new(edge));
        }

        let mut edge = &self.edges[start_i];
        if start != edge.end {
            result.push(Rc::new(Edge::new(
                start.clone(),
                edge.end.clone(),
                edge.curve.clone(),
                edge.direction,
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
                edge.direction,
            )));
        }
        result
    }

    pub fn intersect_edge_different_curve(&self, other: &Edge) -> Vec<Point> {
        let mut intersections = Vec::<Point>::new();
        for edge in self.edges.iter() {
            let edge_intersections = edge.intersect_different_curve(other);
            intersections.extend(edge_intersections.into_iter());
        }

        intersections
    }

    // Gets all intersections between this contour and another contour.
    // Vertices of Edges are not considered as part of the contour, hence, the intersection of two contours at the same point is empty.
    pub fn find_split_points(&self, other: &Contour) -> Vec<Point> {
        let mut intersections = Vec::<Point>::new();
        for edge_other in other.edges.iter() {
            for edge in self.edges.iter() {
                if edge.curve == edge_other.curve {
                    continue;
                }
                let edge_intersections = edge.intersect_different_curve(edge_other);
                intersections.extend(edge_intersections.into_iter());
            }
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
