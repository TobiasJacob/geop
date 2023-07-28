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
        assert!(u >= 0.0 && u < 1.0);
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

    // A list of all intersections that are not yet end points or vertices.
    fn inner_intersections(&self, other: &EdgeLoop) -> Result<(Vec<(f64, Vertex)>, Vec<(f64, Vertex)>), &str> {
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
                        Ok((intersections_self, intersections_other))
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

    // Connects all inner intersections with each other, such that the resulting edge loops are closed and do only intersect at the end points.
    // However, the resulting edge loops may still overlap each other. This could result in a non-manifold topology, so this function is private, and the public function split should be used instead.
    fn remesh(&self, other: &EdgeLoop) -> (Vec<EdgeLoop>, Vec<EdgeLoop>) {
        let mut edges_S: Vec<Edge> = Vec::new();
        let mut edges_O: Vec<Edge> = Vec::new();
        let intersections = self.inner_intersections(other);
        let vertices_self = intersections.iter().map(|p| {
            Vertex { point: Rc::new(*p) }
        }).collect::<Vec<Vertex>>();
        let mut vertices_other = vertices_self.clone();
        vertices_other.sort_by(|b1, b2| other.curve.project(*b1.point).total_cmp(&other.curve.project(*b2.point)));

        let mut edges_self: Vec<Edge> = Vec::with_capacity(vertices_self.len() + 1);
        for i in 0..vertices_self.len() {
            edges_self.push(Edge::new([vertices_self[i].clone(), vertices_self[(i + 1) % vertices_self.len()].clone()], self.curve.clone()));
        }

        let mut edges_other: Vec<Edge> = Vec::with_capacity(vertices_other.len() + 1);
        for i in 0..vertices_other.len() {
            edges_other.push(Edge::new([vertices_other[i].clone(), vertices_other[(i + 1) % vertices_other.len()].clone()], other.curve.clone()));
        }

        (edges_self, edges_other)
    }

    // Splits this edge loop with another edge loop.
    // This makes sure that the resulting edge loops are closed and do not intersect each other anymore.
    // Neighbouring edge loops will share the same end points for the edges, and the two neighbouring edges will face opposite direction.
    pub fn split(&self, other: &EdgeLoop) -> Vec<EdgeLoop> {
        // Each intersection as 4 connected edges. Self_in, Self_out, Other_in, Other_out.
        let (edges_self, edges_other) = self.remesh(other);
        // After generating all intersections, we have to loop around and at each intersection connect Self_out with Other_in and Other_out with Self_in.
        let mut edge_loops = Vec::new();

        let mut all_edges_not_visited: Vec<Edge> = edges_self.clone(); // This is a shallow copy, so the edges still reference the same points. 
        all_edges_not_visited.append(&mut edges_other.clone());

        while all_edges_not_visited.len() > 0 {
            let mut edge_loop = Vec::new();
            let mut edge = all_edges_not_visited.pop().unwrap();
            edge_loop.push(edge);
            let mut next_edge = edge;
            while next_edge.vertices[1] != edge.vertices[0] {
                let mut next_edges: Vec<&Edge>;
                // If the next edge is in edges_self, then we have to look for the next edge in edges_other and vice versa.
                if edges_self.contains(&next_edge) {
                    next_edges = edges_other.iter().filter(|e| e.vertices[0] == next_edge.vertices[1]).collect::<Vec<&Edge>>();
                } else {
                    next_edges = edges_self.iter().filter(|e| e.vertices[0] == next_edge.vertices[1]).collect::<Vec<&Edge>>();
                }
                assert!(next_edges.len() == 1);
                next_edge = next_edges.pop().unwrap().clone();
                edge_loop.push(next_edge);
                all_edges_not_visited.remove_item(&next_edge);
            }
            edge_loops.push(EdgeLoop::new(edge_loop));
        }

        return edge_loops;
    }
}