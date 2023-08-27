use std::{rc::Rc, vec};

use geop_geometry::{points::point::Point, curves::{line::Line, circle::Circle, ellipse::Ellipse, curve::Curve}, curve_curve_intersection::{circle_circle::{circle_circle_intersection, CircleCircleIntersection}, line_line::{line_line_intersection, LineLineIntersection}}, EQ_THRESHOLD};

use crate::{topology::vertex::Vertex, PROJECTION_THRESHOLD};

#[derive(PartialEq, Clone, Debug)]
pub enum EdgeCurve {
    Line(Line),
    Circle(Circle),
    Ellipse(Ellipse),
}
impl EdgeCurve {
    pub fn curve(&self) -> &dyn Curve {
        match self {
            EdgeCurve::Line(line) => line,
            EdgeCurve::Circle(circle) => circle,
            EdgeCurve::Ellipse(ellipse) => ellipse,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Increasing,
    Decreasing,
}

#[derive(Clone, Debug)]
pub struct Edge {
    pub start: Vertex,
    pub end: Vertex,
    pub curve: Rc<EdgeCurve>,
    pub direction: Direction,

    start_u: f64,
    end_u: f64,
}

#[derive(Clone, Debug)]
pub enum EdgeIntersection {
    Vertex(Vertex),
    Edge(Edge),
}

#[derive(Clone, Debug, PartialEq)]
pub enum EdgeContains {
    Inside,
    Outside,
    OnVertex,
}


// Represents an Edge, defined by a curve, and a start and end vertex.
// It is important to know that the start and end vertex are not considered a part of the edge.
// E.g. "intersection" between two edges at end points are not considered intersections.
impl Edge {
    pub fn new(start: Vertex, end: Vertex, curve: Rc<EdgeCurve>, direction: Direction) -> Edge {
        assert!(start != end); // Prevent zero length edges
        let start_u = curve.curve().project(*start.point);
        let end_u_p = curve.curve().project(*end.point);
        assert!(start_u.1 < PROJECTION_THRESHOLD, "Start point is {start:?} not on curve {curve:?}, projection returned {start_u:?}");
        assert!(end_u_p.1 < PROJECTION_THRESHOLD, "End point is {end:?} not on curve {curve:?}, projection returned {end_u_p:?}");
        // It might seem weired to do this here and not simple add for example a curve.periodic() function if start < end.
        // The reason is that for edges it is possible to find parameter spaces relativly easy.
        // For surfaces, this is much more complicated, because we need a valid parameter space within a face that could span poles, which is bounded by an Contour.
        // In both cases, the parameter space is defined by the start and end point of the curve or the outer edge loop.
        // So the code that generates the parameter space (which depends on start and end) belongs here.
        let end_u = match *curve {
            EdgeCurve::Line(_) => end_u_p.0,
            EdgeCurve::Circle(_) => match direction {
                Direction::Increasing => match end_u_p < start_u {
                    true => end_u_p.0 + 2.0 * std::f64::consts::PI,
                    false => end_u_p.0,
                },
                Direction::Decreasing => match end_u_p > start_u {
                    true => end_u_p.0 - 2.0 * std::f64::consts::PI,
                    false => end_u_p.0,
                },
            }
            EdgeCurve::Ellipse(_) => match direction {
                Direction::Increasing => match end_u_p < start_u {
                    true => end_u_p.0 + 2.0 * std::f64::consts::PI,
                    false => end_u_p.0,
                },
                Direction::Decreasing => match end_u_p > start_u {
                    true => end_u_p.0 - 2.0 * std::f64::consts::PI,
                    false => end_u_p.0,
                },
            }
        };

        let start_u = start_u.0;

        Edge {
            start,
            end,
            curve,
            direction,
            start_u,
            end_u,
        }
    }

    pub fn neg(&self) -> Edge {
        Edge::new(self.end.clone(), self.start.clone(), self.curve.clone(), match self.direction {
            Direction::Increasing => Direction::Decreasing,
            Direction::Decreasing => Direction::Increasing,
        })
    }

    // Checks if the edge contains the point, and if so, splits the edge into two edges.
    // It is guaranteed that this happens in order, meaning that the first edge returned will contain the start point of the original edge, and the second edge will contain the end point of the original edge.
    pub fn split_if_necessary(&self, other: &Vertex) -> Vec<Rc<Edge>> {
        if self.contains(*other.point) != EdgeContains::Inside {
            return vec![Rc::new(self.clone())];
        }
        return vec![Rc::new(Edge::new(self.start.clone(), other.clone(), self.curve.clone(), self.direction)), Rc::new(Edge::new(other.clone(), self.end.clone(), self.curve.clone(), self.direction))];
    }

    pub fn get_midpoint(&self, a: Point, b: Point) -> Point {
        let a = self.curve.curve().project(a).0;
        let b = self.curve.curve().project(b).0;
        let mid = (a + b) / 2.0;
        self.curve.curve().point_at(mid)
    }

    // Avoid using these functions as they are not well defined for periodic curves.
    pub fn point_at(&self, u: f64) -> Point {
        assert!(u >= 0.0 && u < 1.0);
        let u = self.start_u + u * (self.end_u - self.start_u);
        self.curve.curve().point_at(u)
    }
    
    pub fn tangent(&self, p: Point) -> Point {
        assert!(self.contains(p) != EdgeContains::Outside);
        match &*self.curve {
            EdgeCurve::Circle(c) => c.derivative(p).normalize(),
            EdgeCurve::Ellipse(e) => e.derivative(p).normalize(),
            EdgeCurve::Line(l) => l.derivative(p).normalize(),
        }
    }

    pub fn project(&self, point: Point) -> Option<f64> {
        let u_p = self.curve.curve().project(point);
        if u_p.1 > PROJECTION_THRESHOLD {
            return None;
        }
        let u = match u_p.0 > self.start_u {
            true => u_p.0,
            false => u_p.0 + 2.0 * std::f64::consts::PI,
        };
        if u < self.start_u - EQ_THRESHOLD || u > self.end_u + EQ_THRESHOLD {
            return None;
        }
        Some((u - self.start_u) / (self.end_u - self.start_u))
    }

    pub fn contains(&self, other: Point) -> EdgeContains {
        let u = self.project(other);
        match u {
            Some(u) => {
                if u < EQ_THRESHOLD || u > 1.0 - EQ_THRESHOLD {
                    EdgeContains::OnVertex
                } else {
                    EdgeContains::Inside
                }
            },
            None => EdgeContains::Outside,
        }
    }

    // pub fn rasterize(&self) -> Vec<Point> {
    //     let num_points = 40 as usize;

    //     (0..num_points).map(|i| {
    //         let t = i as f64 / (num_points - 1) as f64;
    //         let point = self.curve.curve().point_at(t);
    //         point
    //     }).collect()
    // }

    // All intersections where it crosses other edge. The end points are not included. The list is sorted from start to end.
    pub fn intersections(&self, other: &Edge) -> Vec<EdgeIntersection> {
        match *self.curve {
            EdgeCurve::Circle(ref circle) => {
                match *other.curve {
                    EdgeCurve::Circle(ref other_circle) => {
                        match circle_circle_intersection(circle, other_circle) {
                            CircleCircleIntersection::TwoPoint(a, b) => {
                                let mut intersections = Vec::new();
                                let u_a = circle.project(a).0;
                                let u_b = circle.project(b).0;
                                if self.direction == Direction::Increasing {
                                    if u_a < u_b {
                                        intersections.push(Vertex::new(Rc::new(a)));
                                        intersections.push(Vertex::new(Rc::new(b)));
                                    } else {
                                        intersections.push(Vertex::new(Rc::new(b)));
                                        intersections.push(Vertex::new(Rc::new(a)));
                                    }
                                } else {
                                    if u_a < u_b {
                                        intersections.push(Vertex::new(Rc::new(b)));
                                        intersections.push(Vertex::new(Rc::new(a)));
                                    } else {
                                        intersections.push(Vertex::new(Rc::new(a)));
                                        intersections.push(Vertex::new(Rc::new(b)));
                                    }
                                }
                                 intersections.into_iter().filter(|intersection| {
                                    self.contains(*intersection.point) == EdgeContains::Inside && other.contains(*intersection.point) == EdgeContains::Inside
                                }).map(|i| EdgeIntersection::Vertex(i)).collect()
                            },
                            CircleCircleIntersection::OnePoint(a) => {
                                vec![EdgeIntersection::Vertex(Vertex::new(Rc::new(a)))]
                            },
                            CircleCircleIntersection::None => {
                                vec![]
                            }
                            CircleCircleIntersection::Circle(_) => {
                                let mut self_start_u = circle.project(*self.start.point).0;
                                let mut self_end_u = circle.project(*self.end.point).0;
                                let mut other_start_u = other_circle.project(*other.start.point).0;
                                let mut other_end_u = other_circle.project(*other.end.point).0;

                                if self_end_u < self_start_u.max(other_start_u) {
                                    self_start_u += 2.0 * std::f64::consts::PI;
                                    self_end_u += 2.0 * std::f64::consts::PI;
                                }
                                
                                if other_end_u < self_start_u.max(other_start_u) {
                                    other_start_u += 2.0 * std::f64::consts::PI;
                                    other_end_u += 2.0 * std::f64::consts::PI;
                                }

                                let start_u = self_start_u.max(other_start_u);
                                let end_u = self_end_u.min(other_end_u);

                                if self.start == other.start && self.end == other.end {
                                    vec![EdgeIntersection::Edge(Edge::new(self.start.clone(), self.end.clone(), self.curve.clone(), self.direction.clone()))]
                                } else if end_u < start_u {
                                    vec![]
                                } else if self.start == other.start {
                                    vec![EdgeIntersection::Edge(Edge::new(self.start.clone(), Vertex::new(Rc::new(circle.point_at(end_u))), self.curve.clone(), self.direction.clone()))]
                                } else if self.start == other.end {
                                    vec![EdgeIntersection::Edge(Edge::new(self.start.clone(), Vertex::new(Rc::new(circle.point_at(end_u))), self.curve.clone(), self.direction.clone()))]
                                } else if self.end == other.start {
                                    vec![EdgeIntersection::Edge(Edge::new(Vertex::new(Rc::new(circle.point_at(start_u))), self.end.clone(), self.curve.clone(), self.direction.clone()))]
                                } else if self.end == other.end {
                                    vec![EdgeIntersection::Edge(Edge::new(Vertex::new(Rc::new(circle.point_at(start_u))), self.end.clone(), self.curve.clone(), self.direction.clone()))]
                                } else {
                                    vec![EdgeIntersection::Edge(Edge::new(Vertex::new(Rc::new(circle.point_at(start_u))), Vertex::new(Rc::new(circle.point_at(end_u))), self.curve.clone(), self.direction.clone()))]
                                }
                            }
                        }
                    },
                    EdgeCurve::Ellipse(ref ellipse) => {
                        todo!("Implement circle-ellipse intersection")
                    },
                    EdgeCurve::Line(ref line) => {
                        todo!("Implement circle-line intersection")
                    },
                }
            },
            EdgeCurve::Ellipse(ref ellipse) => {
                todo!("Implement ellipse intersection")
            },
            EdgeCurve::Line(ref line) => {
                match *other.curve {
                    EdgeCurve::Circle(ref circle) => {
                        todo!("Implement line-circle intersection")
                    },
                    EdgeCurve::Ellipse(ref ellipse) => {
                        todo!("Implement line-ellipse intersection")
                    },
                    EdgeCurve::Line(ref other_line) => {
                        match line_line_intersection(line, other_line) {
                            LineLineIntersection::Point(a) => {
                                let mut intersections = Vec::new();
                                // Check if it is contained
                                if self.contains(a) == EdgeContains::Inside && other.contains(a) == EdgeContains::Inside {
                                    intersections.push(EdgeIntersection::Vertex(Vertex::new(Rc::new(a))));
                                }
                                intersections
                            },
                            LineLineIntersection::None => {
                                vec![]
                            },
                            LineLineIntersection::Line(_) => {
                                let start_u_other = self.curve.curve().project(*other.start.point).0;
                                let end_u_other = self.curve.curve().project(*other.end.point).0;

                                let (other_start, other_end) = match self.direction {
                                    Direction::Increasing => match start_u_other < end_u_other {
                                        true => (&other.start, &other.end),
                                        false => (&other.end, &other.start),
                                    },
                                    Direction::Decreasing => match start_u_other < end_u_other {
                                        true => (&other.end, &other.start),
                                        false => (&other.start, &other.end),
                                    },
                                };

                                let start = match self.curve.curve().distance(*self.start.point, *self.end.point) < self.curve.curve().distance(*other_start.point, *self.end.point) {
                                    true => self.start.clone(),
                                    false => other_start.clone(),
                                };

                                let end = match self.curve.curve().distance(*self.end.point, *start.point) < self.curve.curve().distance(*other_end.point, *start.point) {
                                    true => self.end.clone(),
                                    false => other_end.clone(),
                                };

                                if start == end {
                                    return vec![EdgeIntersection::Vertex(start)];
                                }

                                return vec![EdgeIntersection::Edge(Edge::new(start, end, self.curve.clone(), self.direction))];
                            },
                        }
                    },
                }
            },
        }
    }


    // pub fn remesh(&self, other: &Edge) -> (Vec<Edge>, Vec<Edge>) {
    //     let (intersections_a, intersections_b) = self.intersections(other);
    //     let mut edges_a = Vec::<Edge>::with_capacity(intersections_a.len() + 1);
    //     edges_a.push(Edge::new(self.start, intersections_a[0].1, self.curve));
    //     for i in 0..intersections_a.len() - 1 {
    //         edges_a.push(Edge::new(intersections_a[i].1, intersections_a[i + 1].1, self.curve));
    //     }
    //     edges_a.push(Edge::new(intersections_a[intersections_a.len() - 1].1, self.end, self.curve));

    //     let mut edges_b = Vec::<Edge>::with_capacity(intersections_b.len() + 1);
    //     edges_b.push(Edge::new(other.start, intersections_b[0].1, other.curve));
    //     for i in 0..intersections_b.len() - 1 {
    //         edges_b.push(Edge::new(intersections_b[i].1, intersections_b[i + 1].1, other.curve));
    //     }
    //     edges_b.push(Edge::new(intersections_b[intersections_b.len() - 1].1, other.end, other.curve));

    //     Ok((edges_a, edges_b))
    // }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        (Rc::ptr_eq(&self.curve, &other.curve) || self.curve == other.curve) && self.start == other.start && self.end == other.end
    }
}