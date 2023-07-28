use core::num;
use std::rc::Rc;

use geop_geometry::geometry::points::point::Point;


use crate::topology::Vertex::Vertex;

use super::{LinearEdge::LinearEdge, CircularEdge::CircularEdge};

pub enum EdgeLoopCurve {
    Linear(Vec<Rc<LinearEdge>>),
    Circular(Rc<CircularEdge>)
}

pub struct EdgeLoop {
    pub curve: EdgeLoopCurve,
}

// An EdgeLoop is a closed loop of edges which is not self intersecting.
impl EdgeLoop {
    pub fn new(edges: Vec<Rc<LinearEdge>>) -> EdgeLoop {
        for i in 0..edges.len() {
            let edge = edges[i].clone();
            let next_edge = edges[(i + 1) % edges.len()].clone();
            assert!(edge.end == next_edge.start);
        }
        assert!(edges.len() > 0);
        assert!(edges[0].start == edges[edges.len() - 1].end);
        EdgeLoop {
            curve: EdgeLoopCurve::Linear(edges),
        }
    }

    pub fn point_at(&self, u: f64) -> Point {
        u = u % 1.0;
        match &self.curve {
            EdgeLoopCurve::Linear(edges) => {
                let mut u = u * edges.len() as f64;
                let mut i = u.floor() as usize;
                u = u - i as f64;
                let edge = edges[i].clone();
                edge.point_at(u)
            },
            EdgeLoopCurve::Circular(edge) => {
                todo!("Not yet implemented");
            }
        }
    }

    pub fn project(&self, point: &Point) -> Option<f64> {
        match &self.curve {
            EdgeLoopCurve::Linear(edges) => {
                let mut u = 0.0;
                for edge in edges {
                    match edge.project(point) {
                        Some(u_p) => {
                            return Some((u + u_p) / edges.len() as f64);
                        },
                        None => {
                            u += 1.0;
                        }
                    }
                }
                None
            },
            EdgeLoopCurve::Circular(edge) => {
                todo!("Not yet implemented");
            }
        }
    }

    pub fn rasterize(&self) -> Vec<Point> {
        match &self.curve {
            EdgeLoopCurve::Linear(edges) => {
                edges.iter().map(|edge| edge.rasterize()).flatten().collect()
            },
            EdgeLoopCurve::Circular(edge) => {
                todo!("Not yet implemented");
            }
        }
    }

    fn get_subcurve(self, start: Vertex, end: Vertex) -> Result<Vec<LinearEdge>, &'static str> {
        let u_start = match self.project(&start.point) {
            Some(it) => it,
            None => return Err("First point is not on the edge loop"),
        };
        let u_end = match self.project(&end.point) {
            Some(it) => it,
            None => return Err("Second point is not on the edge loop"),
        };
        if u_end < u_start {
            u_end += 1.0;
        }

        match self.curve {
            EdgeLoopCurve::Linear(ref edges_self) => {
                let mut edges = Vec::new();
                let start_i = (u_start * edges_self.len() as f64).floor() as usize;
                let end_i = (u_end * edges_self.len() as f64).floor() as usize;
                if start_i == end_i {
                    edges.push(LinearEdge::new(start, end, edges_self[start_i].curve));
                } else {
                    edges.push(LinearEdge::new(start, edges_self[start_i].end, edges_self[start_i].curve));
                    for i in start_i + 1..end_i {
                        edges.push(LinearEdge::new(edges_self[i].start, edges_self[i].end, edges_self[i].curve));
                    }
                    edges.push(LinearEdge::new(edges_self[end_i].start, end, edges_self[end_i].curve));
                }
                Ok(edges)
            },
            EdgeLoopCurve::Circular(ref edge_self) => {
                todo!("Not yet implemented");
            }            
        }
    }

    // A list of all intersections that are not yet end points or vertices.
    fn intersections(&self, other: &EdgeLoop) -> Result<(Vec<Vec<LinearEdge>>, Vec<Vec<LinearEdge>>), &str> {
        match self.curve {
            EdgeLoopCurve::Linear(ref edges_self) => {
                match other.curve {
                    EdgeLoopCurve::Linear(ref edges_other) => {
                        let mut intersections_self = Vec::new();
                        let mut intersections_other = Vec::new();
                        for (i_self, edge_self) in edges_self.iter().enumerate() {
                            for (i_other, edge_other) in edges_other.iter().enumerate() {
                                let (intersections_edge_self, intersections_edge_other) = edge_self.inner_intersections(&edge_other)?;
                                for (u, vertex) in intersections_edge_self {
                                    intersections_self.push(((u + i_self as f64) / edges_self.len() as f64, vertex));
                                }
                                for (u, vertex) in intersections_edge_other {
                                    intersections_other.push(((u + i_other as f64) / edges_other.len() as f64, vertex));
                                }
                            }
                        }
                        intersections_other.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                        intersections_self.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

                        // Due to the properties of edge loops, the edges intersecting the other edge loop can be generated by zipping two following vertices.
                        assert!(intersections_self.len() % 2 == 0);
                        assert!(intersections_other.len() == intersections_self.len());

                        let mut segments_self = Vec::with_capacity(intersections_self.len());
                        for i in 0..intersections_other.len() / 2 {
                            let edge_self = self.get_subcurve(intersections_self[i].1, intersections_self[(i + 1) % intersections_self.len()].1)?;
                            segments_self.push(edge_self);
                        }

                        let mut segments_other = Vec::with_capacity(intersections_other.len());
                        for i in 0..intersections_other.len() / 2 {
                            let edge_other = other.get_subcurve(intersections_other[i].1, intersections_other[(i + 1) % intersections_other.len()].1)?;
                            segments_other.push(edge_other);
                        }

                        Ok((segments_self, segments_other))
                    },
                    EdgeLoopCurve::Circular(ref edge_other) => {
                        todo!("Not yet implemented")
                    }
                }
            },
            EdgeLoopCurve::Circular(ref edge_self) => {
                match other.curve {
                    EdgeLoopCurve::Linear(ref edges_other) => {
                        todo!("Not yet implemented")
                    },
                    EdgeLoopCurve::Circular(ref edge_other) => {
                        todo!("Not yet implemented")
                    }
                }
            }
        }
    }

    // Splits this edge loop with another edge loop.
    // This makes sure that the resulting edge loops are closed and do not intersect each other anymore.
    // Neighbouring edge loops will share the same end points for the edges, and the two neighbouring edges will face opposite direction.
    pub fn remesh_self_other(&self, other: &EdgeLoop) -> Result<Vec<EdgeLoop>, &str> {
        let (mut segments_self, segments_other) = self.intersections(other)?;

        let find_segment_starting = |vertex: Vertex, segment_is_self: bool| -> Vec<LinearEdge> {
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
            let mut edge_loop: Vec<Rc<LinearEdge>> = Vec::new();
            let mut next_segment = current_segment;
            while next_segment[next_segment.len()].end != edge_loop[0].start {
                edge_loop.extend(next_segment.drain(..).map(|edge| Rc::new(edge)));
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