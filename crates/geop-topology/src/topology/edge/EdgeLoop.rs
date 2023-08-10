use std::rc::Rc;

use geop_geometry::geometry::points::point::Point;


use crate::topology::{Vertex::Vertex, edge::Edge::EdgeIntersection};

use super::Edge::Edge;

pub struct EdgeLoop {
    pub edges: Vec<Rc<Edge>>,
}

// An EdgeLoop is a closed loop of edges which is not self intersecting.
impl EdgeLoop {
    pub fn new(edges: Vec<Rc<Edge>>) -> EdgeLoop {
        for i in 0..edges.len() {
            let edge = edges[i].clone();
            let next_edge = edges[(i + 1) % edges.len()].clone();
            assert!(edge.end == next_edge.start);
        }
        assert!(edges.len() > 0);
        assert!(edges[0].start == edges[edges.len() - 1].end);
        EdgeLoop {
            edges,
        }
    }

    pub fn point_at(&self, u: f64) -> Point {
        let mut u = u % 1.0;
        u = u * self.edges.len() as f64;
        let i = u.floor() as usize;
        u = u - i as f64;
        let edge = self.edges[i].clone();
        edge.point_at(u)
    }

    pub fn project(&self, point: &Point) -> Option<f64> {
        let mut u = 0.0;
        for edge in self.edges {
            match edge.project(point) {
                Some(u_p) => {
                    return Some((u + u_p) / self.edges.len() as f64);
                },
                None => {
                    u += 1.0;
                }
            }
        }
        None
    }

    pub fn rasterize(&self) -> Vec<Point> {
        self.edges.iter().map(|edge| edge.rasterize()).flatten().collect()
    }

    fn get_subcurve(self, start: Vertex, end: Vertex) -> Result<Vec<Rc<Edge>>, &'static str> {
        let u_start = match self.project(&start.point) {
            Some(it) => it,
            None => return Err("First point is not on the edge loop"),
        };
        let mut u_end = match self.project(&end.point) {
            Some(it) => it,
            None => return Err("Second point is not on the edge loop"),
        };
        if u_end < u_start {
            u_end += 1.0;
        }
        let mut edges: Vec<Rc<Edge>> = Vec::new();
        let start_i = (u_start * self.edges.len() as f64).floor() as usize;
        let end_i = (u_end * self.edges.len() as f64).floor() as usize;
        if start_i == end_i {
            edges.push(Rc::new(Edge::new(start, end, self.edges[start_i].curve)));
        } else {
            edges.push(Rc::new(Edge::new(start, self.edges[start_i].end, self.edges[start_i].curve)));
            for i in start_i + 1..end_i {
                edges.push(edges[i]);
            }
            edges.push(Rc::new(Edge::new(self.edges[end_i].start, end, self.edges[end_i].curve)));
        }
        Ok(edges)
    }

    // Takes 2 EdgeLoops and connects them at intersecting points with new vertices. If there are overlapping edges, there will be a vertex for the beginning and the end of the overlapping edges, and a connecting edge for each loop. If there are no intersections, the outer vector will have length 1.
    fn split(&self, other: &EdgeLoop) -> (Vec<Vec<Rc<Edge>>>, Vec<Vec<Rc<Edge>>>) {
        // First, find all intersections and order them by position on the edge loop.
        let mut split_verts_self: Vec<(f64, Vertex)> = Vec::new();
        let mut split_verts_other: Vec<(f64, Vertex)> = Vec::new();
        for (i_self, edge_self) in self.edges.iter().enumerate() {
            for (i_other, edge_other) in other.edges.iter().enumerate() {
                let intersections = edge_self.intersections(&edge_other);
                for intersection in intersections {
                    match intersection {
                        EdgeIntersection::Vertex(intersect_vertex) => {
                            split_verts_self.push((self.project(&intersect_vertex.point).expect("Intersection point has to be on edge"), intersect_vertex));
                            split_verts_other.push((other.project(&intersect_vertex.point).expect("Intersection point has to be on edge"), intersect_vertex));
                        },
                        EdgeIntersection::Edge(intersect_edge) => {
                            split_verts_self.push((self.project(&intersect_edge.start.point).expect("Intersection point has to be on edge"), intersect_edge.start));
                            split_verts_self.push((self.project(&intersect_edge.end.point).expect("Intersection point has to be on edge"), intersect_edge.end));
                            split_verts_other.push((self.project(&intersect_edge.start.point).expect("Intersection point has to be on edge"), intersect_edge.start));
                            split_verts_other.push((self.project(&intersect_edge.end.point).expect("Intersection point has to be on edge"), intersect_edge.end));
                        },
                    }
                }
            }
        }
        split_verts_self.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        split_verts_other.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        // Due to the properties of edge loops, the edges intersecting the other edge loop can be generated by zipping two following vertices.
        let n: usize = split_verts_self.len();
        assert!(n % 2 == 0);
        assert!(n == split_verts_other.len());

        if n == 0 {
            return (vec![self.edges.clone()], vec![other.edges.clone()]);
        }

        let mut segments_self = Vec::with_capacity(n);
        for i in 0..n {
            let edge_self = self.get_subcurve(split_verts_self[i].1, split_verts_self[(i + 1) % n].1).expect("Intersection points have to be on edge");
            segments_self.push(edge_self);
        }

        let mut segments_other = Vec::with_capacity(n);
        for i in 0..n {
            let edge_other = other.get_subcurve(split_verts_other[i].1, split_verts_other[(i + 1) % n].1).expect("Intersection points have to be on edge");
            segments_other.push(edge_other);
        }

        (segments_self, segments_other)
    }

    // Splits this edge loop with another edge loop.
    // This makes sure that the resulting edge loops are closed and do not intersect each other anymore.
    // Neighbouring edge loops will share the same end points for the edges, and the two neighbouring edges will face opposite direction.
    pub fn remesh_self_other(&self, other: &EdgeLoop) -> Result<Vec<EdgeLoop>, &str> {
        let (mut segments_self, segments_other) = self.split(other);

        let find_segment_starting = |vertex: Vertex, segment_is_self: bool| -> Vec<Rc<Edge>> {
            let relevant_segments = if segment_is_self { segments_self } else { segments_other };
            for segment in relevant_segments {
                if segment[0].start == vertex {
                    return segment;
                }
            }
            panic!("Called find_segment_self_starting with invalid vertex") // This should never happen
        };

        let mut edge_loops = Vec::new();
        let mut segment_is_self = false;
        while let Some(mut current_segment) = segments_self.pop(){
            let mut edge_loop: Vec<Rc<Edge>> = Vec::new();
            let mut next_segment = current_segment;
            while next_segment[next_segment.len()].end != edge_loop[0].start {
                edge_loop.extend(next_segment.drain(..).map(|edge| edge));
                next_segment = find_segment_starting(next_segment[next_segment.len()].end, segment_is_self);
                // remove next_segment from segments_self if it is in there
                if let Some(i) = segments_self.iter().position(|segment| segment[0].start == next_segment[0].start) {
                    segments_self.remove(i);
                }
                segment_is_self = !segment_is_self;
            }
            edge_loops.push(EdgeLoop::new(edge_loop));
        }
        Ok(edge_loops)
    }
}