use std::{rc::Rc, vec, f32::consts::E};

use geop_geometry::{geometry::{points::point::Point, curves::{line::Line, circle::Circle, ellipse::Ellipse, curve::Curve}}, intersections::{circle_circle::{circle_circle_intersection, CircleCircleIntersection}, line_line::{line_line_intersection, LineLineIntersection}}};

use crate::{topology::Vertex::Vertex, PROJECTION_THRESHOLD};

#[derive(PartialEq)]
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

pub struct Edge {
    pub start: Vertex,
    pub end: Vertex,
    pub curve: Rc<EdgeCurve>,
    start_u: f64,
    end_u: f64,
}

pub struct EdgeIntersectionPoint {
    pub u_self: f64,
    pub u_other: f64,
    pub vertex: Vertex,
}

impl EdgeIntersectionPoint {
    pub fn new(u_self: f64, u_other: f64, vertex: Vertex) -> EdgeIntersectionPoint {
        EdgeIntersectionPoint {
            u_self,
            u_other,
            vertex,
        }
    }
}

pub struct EdgeIntersectionLine {
    pub start_u_self: f64,
    pub end_u_self: f64,
    pub start_u_other: f64,
    pub end_u_other: f64,
    pub edge: Edge,
}

impl EdgeIntersectionLine {
    pub fn new(start_u_self: f64, end_u_self: f64, start_u_other: f64, end_u_other: f64, edge: Edge) -> EdgeIntersectionLine {
        EdgeIntersectionLine {
            start_u_self,
            end_u_self,
            start_u_other,
            end_u_other,
            edge,
        }
    }
}

pub enum EdgeIntersection {
    Vertex(Vertex),
    Edge(Edge),
}

impl EdgeIntersection {
    pub fn start_u_self(&self) -> f64 {
        match self {
            EdgeIntersection::Point(point) => point.u_self,
            EdgeIntersection::Edge(line) => line.start_u_self,
        }
    }

    pub fn start_u_other(&self) -> f64 {
        match self {
            EdgeIntersection::Point(point) => point.u_other,
            EdgeIntersection::Edge(line) => line.start_u_other,
        }
    }
}


// TODO: Implement an periodic / circular edge
impl Edge {
    pub fn new(start: Vertex, end: Vertex, curve: Rc<EdgeCurve>) -> Edge {
        let start_u = curve.curve().project(&start.point);
        let end_u_p = curve.curve().project(&end.point);
        assert!(start_u.1 < PROJECTION_THRESHOLD);
        assert!(end_u_p.1 < PROJECTION_THRESHOLD);
        // It might seem weired to do this here and not simple add for example a curve.periodic() function if start < end.
        // The reason is that for edges it is possible to find parameter spaces relativly easy.
        // For surfaces, this is much more complicated, because we need a valid parameter space within a face that could span poles, which is bounded by an EdgeLoop.
        // In both cases, the parameter space is defined by the start and end point of the curve or the outer edge loop.
        // So the code that generates the parameter space (which depends on start and end) belongs here.
        let end_u = match *curve {
            EdgeCurve::Line(_) => end_u_p.0,
            EdgeCurve::Circle(_) => match end_u_p < start_u {
                true => end_u_p.0 + 2.0 * std::f64::consts::PI,
                false => end_u_p.0,
            }
            EdgeCurve::Ellipse(_) => match end_u_p < start_u {
                true => end_u_p.0 + 2.0 * std::f64::consts::PI,
                false => end_u_p.0,
            }
        };
        
        Edge {
            start,
            end,
            curve,
            start_u: curve.curve().project(&start.point).0,
            end_u,
        }
    }

    pub fn point_at(&self, u: f64) -> Point {
        assert!(u >= 0.0 && u < 1.0);
        let u = self.start_u + u * (self.end_u - self.start_u);
        self.curve.curve().point_at(u)
    }

    pub fn project(&self, point: &Point) -> Option<f64> {
        let u_p = self.curve.curve().project(point);
        if u_p.1 > PROJECTION_THRESHOLD {
            return None;
        }
        let u = match u_p.0 > self.start_u {
            true => u_p.0,
            false => u_p.0 + 2.0 * std::f64::consts::PI,
        };
        if u < self.start_u || u > self.end_u {
            return None;
        }
        Some((u - self.start_u) / (self.end_u - self.start_u))
    }

    pub fn rasterize(&self) -> Vec<Point> {
        let num_points = 40 as usize;

        (0..num_points).map(|i| {
            let t = i as f64 / (num_points - 1) as f64;
            let point = self.curve.curve().point_at(t);
            point
        }).collect()
    }

    // All intersections with other edge. The end points are included, but if they overlap they will continue to refer to the same vertex. The list is unsorted.
    pub fn intersections(&self, other: &Edge) -> Vec<EdgeIntersection> {
        match *self.curve {
            EdgeCurve::Circle(ref circle) => {
                match *other.curve {
                    EdgeCurve::Circle(ref other_circle) => {
                        match circle_circle_intersection(circle, other_circle) {
                            CircleCircleIntersection::TwoPoint(a, b) => {
                                let mut intersections = Vec::new();

                                if let Some(u_a_self) = self.project(&a) {
                                    if let Some(u_a_other) = other.project(&a) {
                                        intersections.push(EdgeIntersection::Point(EdgeIntersectionPoint::new(u_a_self, u_a_other, Vertex::new(Rc::new(a)))));
                                    } else {
                                        panic!("Circle intersection point is not on other circle")
                                    }
                                } else {
                                    panic!("Circle intersection point is not on circle")
                                }

                                if let Some(u_b_self) = self.project(&b) {
                                    if let Some(u_b_other) = other.project(&b) {
                                        intersections.push(EdgeIntersection::Point(EdgeIntersectionPoint::new(u_b_self, u_b_other, Vertex::new(Rc::new(b)))));
                                    } else {
                                        panic!("Circle intersection point is not on other circle")
                                    }
                                } else {
                                    panic!("Circle intersection point is not on circle")
                                }

                                intersections
                            },
                            CircleCircleIntersection::OnePoint(a) => {
                                if let Some(u_a_self) = self.project(&a) {
                                    if let Some(u_a_other) = other.project(&a) {
                                        vec![EdgeIntersection::Point(EdgeIntersectionPoint::new(u_a_self, u_a_other, Vertex::new(Rc::new(a))))]
                                    } else {
                                        panic!("Circle intersection point is not on other circle")
                                    }
                                } else {
                                    panic!("Circle intersection point is not on circle")
                                }
                            },
                            CircleCircleIntersection::None => {
                                vec![]
                            }
                            CircleCircleIntersection::Circle(_) => {
                                let mut self_start_u = circle.project(&self.start.point).0;
                                let mut self_end_u = circle.project(&self.end.point).0;
                                let mut other_start_u = other_circle.project(&other.start.point).0;
                                let mut other_end_u = other_circle.project(&other.end.point).0;

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

                                let start_u_in_other_space = other.project(&self.point_at(start_u)).unwrap();
                                let end_u_in_other_space = other.project(&self.point_at(end_u)).unwrap();

                                if self.start == other.start && self.end == other.end {
                                    vec![EdgeIntersection::Edge(EdgeIntersectionLine::new(start_u, end_u, start_u_in_other_space, end_u_in_other_space, Edge::new(self.start, self.end, self.curve)))]
                                } else if end_u < start_u {
                                    vec![]
                                } else if self.start == other.start {
                                    vec![EdgeIntersection::Edge(EdgeIntersectionLine::new(start_u, end_u, start_u_in_other_space, end_u_in_other_space, Edge::new(self.start, Vertex::new(Rc::new(self.point_at(end_u))), self.curve)))]
                                } else if self.start == other.end {
                                    vec![EdgeIntersection::Edge(EdgeIntersectionLine::new(start_u, end_u, start_u_in_other_space, end_u_in_other_space, Edge::new(self.start, Vertex::new(Rc::new(self.point_at(end_u))), self.curve)))]
                                } else if self.end == other.start {
                                    vec![EdgeIntersection::Edge(EdgeIntersectionLine::new(start_u, end_u, start_u_in_other_space, end_u_in_other_space, Edge::new(Vertex::new(Rc::new(self.point_at(start_u))), self.end, self.curve)))]
                                } else if self.end == other.end {
                                    vec![EdgeIntersection::Edge(EdgeIntersectionLine::new(start_u, end_u, start_u_in_other_space, end_u_in_other_space, Edge::new(Vertex::new(Rc::new(self.point_at(start_u))), self.end, self.curve)))]
                                } else {
                                    vec![EdgeIntersection::Edge(EdgeIntersectionLine::new(start_u, end_u, start_u_in_other_space, end_u_in_other_space, Edge::new(Vertex::new(Rc::new(self.point_at(start_u))), Vertex::new(Rc::new(self.point_at(end_u))), self.curve)))]
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
                                if let Some(u_a_self) = self.project(&a) {
                                    if let Some(u_a_other) = other.project(&a) {
                                        intersections.push(EdgeIntersection::Point(EdgeIntersectionPoint::new(u_a_self, u_a_other, Vertex::new(Rc::new(a)))));
                                    }
                                }
                                intersections
                            },
                            LineLineIntersection::None => {
                                vec![]
                            },
                            LineLineIntersection::Line(_) => {
                                let mut start_u = other_line.project(&self.start.point).0;
                                let mut end_u = other_line.project(&self.end.point).0;
                                let mut other_start_u = other_line.project(&other.start.point).0;
                                let mut other_end_u = other_line.project(&other.end.point).0;

                                if self.start == other.start && self.end == other.end {
                                    vec![EdgeIntersection::Edge(EdgeIntersectionLine::new(start_u, end_u, other_start_u, other_end_u, Edge::new(self.start, self.end, self.curve)))]
                                } else if end_u < start_u {
                                    vec![]
                                } else if self.start == other.start {
                                    vec![EdgeIntersection::Edge(EdgeIntersectionLine::new(start_u, end_u, other_start_u, other_end_u, Edge::new(self.start, Vertex::new(Rc::new(self.point_at(end_u))), self.curve)))]
                                } else if self.start == other.end {
                                    vec![EdgeIntersection::Edge(EdgeIntersectionLine::new(start_u, end_u, other_start_u, other_end_u, Edge::new(self.start, Vertex::new(Rc::new(self.point_at(end_u))), self.curve)))]
                                } else if self.end == other.start {
                                    vec![EdgeIntersection::Edge(EdgeIntersectionLine::new(start_u, end_u, other_start_u, other_end_u, Edge::new(Vertex::new(Rc::new(self.point_at(start_u))), self.end, self.curve)))]
                                } else if self.end == other.end {
                                    vec![EdgeIntersection::Edge(EdgeIntersectionLine::new(start_u, end_u, other_start_u, other_end_u, Edge::new(Vertex::new(Rc::new(self.point_at(start_u))), self.end, self.curve)))]
                                } else {
                                    vec![EdgeIntersection::Edge(EdgeIntersectionLine::new(start_u, end_u, other_start_u, other_end_u, Edge::new(Vertex::new(Rc::new(self.point_at(start_u))), Vertex::new(Rc::new(self.point_at(end_u))), self.curve)))]
                                }
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