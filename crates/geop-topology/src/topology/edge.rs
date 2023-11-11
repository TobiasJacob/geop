use std::{
    fmt::{Display, Formatter},
    rc::Rc,
    vec,
};

use geop_geometry::{
    curve_curve_intersection::{
        circle_circle::{circle_circle_intersection, CircleCircleIntersection},
        line_line::{line_line_intersection, LineLineIntersection},
    },
    curves::{circle::{Circle, CircleTransform}, curve::Curve, ellipse::Ellipse, line::Line},
    points::point::Point,
    EQ_THRESHOLD, transforms::Transform,
};

use crate::PROJECTION_THRESHOLD;

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

    pub fn transform(&self, transform: Transform) -> EdgeCurve {
        match self {
            EdgeCurve::Line(line) => EdgeCurve::Line(line.transform(transform)),
            EdgeCurve::Circle(circle) => match circle.transform(transform) {
                CircleTransform::Circle(circle) => EdgeCurve::Circle(circle),
                CircleTransform::Ellipse(ellipse) => EdgeCurve::Ellipse(ellipse),
            },
            EdgeCurve::Ellipse(ellipse) => {
                EdgeCurve::Ellipse(ellipse.transform(transform))
            }
        }
    }

    pub fn intersect(&self, other: &EdgeCurve) -> Vec<Point>{
        assert!(self != other);
        match self {
            EdgeCurve::Line(line) => match other {
                EdgeCurve::Line(other_line) => {
                    match line_line_intersection(line, other_line) {
                        LineLineIntersection::Point(p) => {vec![p]}
                        LineLineIntersection::None => {vec![]}
                        LineLineIntersection::Line(_) => {panic!("Curves are different so they cannot be parallel")}
                    }
                }
                EdgeCurve::Circle(other_circle) => {
                    todo!("Implement line-circle intersection")
                }
                EdgeCurve::Ellipse(other_ellipse) => {
                    todo!("Implement line-ellipse intersection")
                }
            },
            EdgeCurve::Circle(circle) => match other {
                EdgeCurve::Line(line) => {
                    todo!("Implement circle-line intersection")
                }
                EdgeCurve::Circle(other_circle) => {
                    match circle_circle_intersection(circle, other_circle) {
                        CircleCircleIntersection::TwoPoint(a, b) => {
                            vec![a, b]
                        }
                        CircleCircleIntersection::OnePoint(a) => {
                            vec![a]
                        }
                        CircleCircleIntersection::None => {
                            vec![]
                        }
                        CircleCircleIntersection::Circle(_) => {
                            todo!("Implement circle-circle intersection")
                        }
                    }
                }
                EdgeCurve::Ellipse(other_ellipse) => {
                    todo!("Implement circle-ellipse intersection")
                }
            },
            EdgeCurve::Ellipse(ellipse) => match other {
                EdgeCurve::Line(line) => {
                    todo!("Implement ellipse-line intersection")
                }
                EdgeCurve::Circle(circle) => {
                    todo!("Implement ellipse-circle intersection")
                }
                EdgeCurve::Ellipse(other_ellipse) => {
                    todo!("Implement ellipse-ellipse intersection")
                }
            },
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
    pub start: Rc<Point>,
    pub end: Rc<Point>,
    pub curve: Rc<EdgeCurve>,
    pub direction: Direction,

    start_u: f64,
    end_u: f64,
}

// Helper type for cases where point or edge is returned.
pub enum PointOrEdge {
    Point(Point),
    Edge(Edge),
}

#[derive(Clone, Debug)]
pub enum EdgeEdgeIntersection {
    Point(Rc<Point>),
    Edge(Edge),
}

#[derive(Clone, Debug, PartialEq)]
pub enum EdgeContains {
    Inside,
    Outside,
    OnPoint(Rc<Point>),
}

// Represents an Edge, defined by a curve, and a start and end point.
// It is important to know that the start and end point are not considered a part of the edge.
// E.g. "intersection" between two edges at end points are not considered intersections.
impl Edge {
    pub fn new(start: Rc<Point>, end: Rc<Point>, curve: Rc<EdgeCurve>, direction: Direction) -> Edge {
        assert!(start != end); // Prevent zero length edges
        let start_u = curve.curve().project(*start);
        let end_u_p = curve.curve().project(*end);
        assert!(
            start_u.1 < PROJECTION_THRESHOLD,
            "Start point is {start:?} not on curve {curve:?}, projection returned {start_u:?}"
        );
        assert!(
            end_u_p.1 < PROJECTION_THRESHOLD,
            "End point is {end:?} not on curve {curve:?}, projection returned {end_u_p:?}"
        );
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
            },
            EdgeCurve::Ellipse(_) => match direction {
                Direction::Increasing => match end_u_p < start_u {
                    true => end_u_p.0 + 2.0 * std::f64::consts::PI,
                    false => end_u_p.0,
                },
                Direction::Decreasing => match end_u_p > start_u {
                    true => end_u_p.0 - 2.0 * std::f64::consts::PI,
                    false => end_u_p.0,
                },
            },
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
        Edge::new(
            self.end.clone(),
            self.start.clone(),
            self.curve.clone(),
            match self.direction {
                Direction::Increasing => Direction::Decreasing,
                Direction::Decreasing => Direction::Increasing,
            },
        )
    }

    pub fn transform(&self, transform: Transform) -> Edge {
        Edge::new(
            Rc::new(transform * *self.start),
            Rc::new(transform * *self.end),
            Rc::new(self.curve.transform(transform)),
            self.direction,
        )
    }

    // Checks if the edge contains the point, and if so, splits the edge into two edges.
    // It is guaranteed that this happens in order, meaning that the first edge returned will contain the start point of the original edge, and the second edge will contain the end point of the original edge.
    pub fn split_if_necessary(&self, other: &Point) -> Vec<Rc<Edge>> {
        if self.contains(*other) != EdgeContains::Inside {
            return vec![Rc::new(self.clone())];
        }
        return vec![
            Rc::new(Edge::new(
                self.start.clone(),
                Rc::new(other.clone()),
                self.curve.clone(),
                self.direction,
            )),
            Rc::new(Edge::new(
                Rc::new(other.clone()),
                self.end.clone(),
                self.curve.clone(),
                self.direction,
            )),
        ];
    }

    pub fn get_midpoint(&self, a: Point, b: Point) -> Point {
        if a == b {
            return a;
        }
        let a = self.project(a).expect("A is not on edge");
        let b = self.project(b).expect("B is not on edge");
        let mid = (a + b) / 2.0;
        self.point_at(mid)
    }

    // Avoid using these functions as they are not well defined for periodic curves.
    pub fn point_at(&self, u: f64) -> Point {
        assert!(u >= -EQ_THRESHOLD && u < 1.0 + EQ_THRESHOLD);
        let u = self.start_u + u * (self.end_u - self.start_u);
        self.curve.curve().point_at(u)
    }

    pub fn tangent(&self, p: Point) -> Point {
        // println!("Point: {:?}", p);
        // println!("Start: {:?}", self.start.point);
        // println!("End: {:?}", self.end.point);
        assert!(self.contains(p) != EdgeContains::Outside);
        match self.direction {
            Direction::Increasing => match &*self.curve {
                EdgeCurve::Circle(c) => c.derivative(p).normalize(),
                EdgeCurve::Ellipse(e) => e.derivative(p).normalize(),
                EdgeCurve::Line(l) => l.derivative(p).normalize(),
            },
            Direction::Decreasing => match &*self.curve {
                EdgeCurve::Circle(c) => (-c.derivative(p)).normalize(),
                EdgeCurve::Ellipse(e) => (-e.derivative(p)).normalize(),
                EdgeCurve::Line(l) => (-l.derivative(p)).normalize(),
            },
        }
    }

    pub fn project(&self, point: Point) -> Option<f64> {
        let u_p = self.curve.curve().project(point);
        if u_p.1 > PROJECTION_THRESHOLD {
            return None;
        }
        let u = u_p.0;
        match self.direction {
            Direction::Increasing => {
                if u < self.start_u - EQ_THRESHOLD || u > self.end_u + EQ_THRESHOLD {
                    return None;
                }
            }
            Direction::Decreasing => {
                if u > self.start_u + EQ_THRESHOLD || u < self.end_u - EQ_THRESHOLD {
                    return None;
                }
            }
        }
        Some((u - self.start_u) / (self.end_u - self.start_u))
    }

    pub fn contains(&self, other: Point) -> EdgeContains {
        let u = self.project(other);
        match u {
            Some(u) => {
                if u < EQ_THRESHOLD {
                    EdgeContains::OnPoint(self.start.clone())
                } else if u > 1.0 - EQ_THRESHOLD {
                    EdgeContains::OnPoint(self.end.clone())
                } else {
                    EdgeContains::Inside
                }
            }
            None => EdgeContains::Outside,
        }
    }

    pub fn intersect_same_curve(&self, other: &Edge) -> Edge {
        assert!(self.curve == other.curve);
        let start_u = self.curve.curve().project(*self.start).0;
        let end_u = self.curve.curve().project(*self.end).0;
        let other_start_u = self.curve.curve().project(*other.start).0;
        let other_end_u = self.curve.curve().project(*other.end).0;

        let start = match start_u < other_start_u {
            true => self.start.clone(),
            false => other.start.clone(),
        };

        let end = match end_u > other_end_u {
            true => self.end.clone(),
            false => other.end.clone(),
        };

        Edge::new(
            start,
            end,
            self.curve.clone(),
            match start_u < end_u {
                true => self.direction,
                false => match self.direction {
                    Direction::Increasing => Direction::Decreasing,
                    Direction::Decreasing => Direction::Increasing,
                },
            },
        )
    }

    // All intersections where it crosses other edge. The end points are not included. The list is sorted from start to end.
    pub fn intersect_different_curve(&self, other: &Edge) -> Vec<Point> {
        assert!(self.curve != other.curve);

        let mut result = self.curve.intersect(&*other.curve).drain(..).filter(|p| {
            self.contains(*p) == EdgeContains::Inside
                && other.contains(*p) == EdgeContains::Inside
        }).collect::<Vec<Point>>();

        result.sort_by(|a, b| {
            let a_u = self.project(*a).unwrap();
            let b_u = self.project(*b).unwrap();
            a_u.partial_cmp(&b_u).unwrap()
        });

        return result;
    }


    // All intersections where it crosses other edge. The end points are not included. The list is sorted from start to end.
    // pub fn intersections(&self, other: &Edge) -> Vec<EdgeEdgeIntersection> {
    //     match *self.curve {
    //         EdgeCurve::Circle(ref circle) => match *other.curve {
    //             EdgeCurve::Circle(ref other_circle) => {
    //                 match circle_circle_intersection(circle, other_circle) {
    //                     CircleCircleIntersection::TwoPoint(a, b) => {
    //                         let mut intersections = Vec::new();
    //                         let u_a = circle.project(a).0;
    //                         let u_b = circle.project(b).0;
    //                         if self.direction == Direction::Increasing {
    //                             if u_a < u_b {
    //                                 intersections.push(Rc::new(a));
    //                                 intersections.push(Rc::new(b));
    //                             } else {
    //                                 intersections.push(Rc::new(b));
    //                                 intersections.push(Rc::new(a));
    //                             }
    //                         } else {
    //                             if u_a < u_b {
    //                                 intersections.push(Rc::new(b));
    //                                 intersections.push(Rc::new(a));
    //                             } else {
    //                                 intersections.push(Rc::new(a));
    //                                 intersections.push(Rc::new(b));
    //                             }
    //                         }
    //                         intersections
    //                             .into_iter()
    //                             .filter(|intersection| {
    //                                 self.contains(**intersection) == EdgeContains::Inside
    //                                     && other.contains(**intersection)
    //                                         == EdgeContains::Inside
    //                             })
    //                             .map(|i| EdgeEdgeIntersection::Point(i))
    //                             .collect()
    //                     }
    //                     CircleCircleIntersection::OnePoint(a) => {
    //                         if self.contains(a) == EdgeContains::Inside
    //                             && other.contains(a) == EdgeContains::Inside
    //                         {
    //                             vec![EdgeEdgeIntersection::Point(Rc::new(a))]
    //                         } else {
    //                             vec![]
    //                         }
    //                     }
    //                     CircleCircleIntersection::None => {
    //                         vec![]
    //                     }
    //                     CircleCircleIntersection::Circle(_) => {
    //                         let mut self_start_u = circle.project(*self.start).0;
    //                         let mut self_end_u = circle.project(*self.end).0;
    //                         let mut other_start_u = other_circle.project(*other.start).0;
    //                         let mut other_end_u = other_circle.project(*other.end).0;

    //                         if self_end_u < self_start_u.max(other_start_u) {
    //                             self_start_u += 2.0 * std::f64::consts::PI;
    //                             self_end_u += 2.0 * std::f64::consts::PI;
    //                         }

    //                         if other_end_u < self_start_u.max(other_start_u) {
    //                             other_start_u += 2.0 * std::f64::consts::PI;
    //                             other_end_u += 2.0 * std::f64::consts::PI;
    //                         }

    //                         let start_u = self_start_u.max(other_start_u);
    //                         let end_u = self_end_u.min(other_end_u);

    //                         if self.start == other.start && self.end == other.end {
    //                             vec![EdgeEdgeIntersection::Edge(Edge::new(
    //                                 self.start.clone(),
    //                                 self.end.clone(),
    //                                 self.curve.clone(),
    //                                 self.direction.clone(),
    //                             ))]
    //                         } else if end_u < start_u {
    //                             vec![]
    //                         } else if self.start == other.start {
    //                             vec![EdgeEdgeIntersection::Edge(Edge::new(
    //                                 self.start.clone(),
    //                                 Rc::new(circle.point_at(end_u)),
    //                                 self.curve.clone(),
    //                                 self.direction.clone(),
    //                             ))]
    //                         } else if self.start == other.end {
    //                             vec![EdgeEdgeIntersection::Edge(Edge::new(
    //                                 self.start.clone(),
    //                                 Rc::new(circle.point_at(end_u)),
    //                                 self.curve.clone(),
    //                                 self.direction.clone(),
    //                             ))]
    //                         } else if self.end == other.start {
    //                             vec![EdgeEdgeIntersection::Edge(Edge::new(
    //                                 Rc::new(circle.point_at(start_u)),
    //                                 self.end.clone(),
    //                                 self.curve.clone(),
    //                                 self.direction.clone(),
    //                             ))]
    //                         } else if self.end == other.end {
    //                             vec![EdgeEdgeIntersection::Edge(Edge::new(
    //                                 Rc::new(circle.point_at(start_u)),
    //                                 self.end.clone(),
    //                                 self.curve.clone(),
    //                                 self.direction.clone(),
    //                             ))]
    //                         } else {
    //                             vec![EdgeEdgeIntersection::Edge(Edge::new(
    //                                 Rc::new(circle.point_at(start_u)),
    //                                 Rc::new(circle.point_at(end_u)),
    //                                 self.curve.clone(),
    //                                 self.direction.clone(),
    //                             ))]
    //                         }
    //                     }
    //                 }
    //             }
    //             EdgeCurve::Ellipse(ref _ellipse) => {
    //                 todo!("Implement circle-ellipse intersection")
    //             }
    //             EdgeCurve::Line(ref _line) => {
    //                 todo!("Implement circle-line intersection")
    //             }
    //         },
    //         EdgeCurve::Ellipse(ref _ellipse) => {
    //             todo!("Implement ellipse intersection")
    //         }
    //         EdgeCurve::Line(ref line) => {
    //             match *other.curve {
    //                 EdgeCurve::Circle(ref _circle) => {
    //                     todo!("Implement line-circle intersection")
    //                 }
    //                 EdgeCurve::Ellipse(ref _ellipse) => {
    //                     todo!("Implement line-ellipse intersection")
    //                 }
    //                 EdgeCurve::Line(ref other_line) => {
    //                     match line_line_intersection(line, other_line) {
    //                         LineLineIntersection::Point(a) => {
    //                             let mut intersections = Vec::new();
    //                             // Check if it is contained
    //                             if self.contains(a) == EdgeContains::Inside
    //                                 && other.contains(a) == EdgeContains::Inside
    //                             {
    //                                 intersections
    //                                     .push(EdgeEdgeIntersection::Point(Rc::new(a)));
    //                             }
    //                             intersections
    //                         }
    //                         LineLineIntersection::None => {
    //                             vec![]
    //                         }
    //                         LineLineIntersection::Line(_) => {
    //                             let start_u_other =
    //                                 self.curve.curve().project(*other.start).0;
    //                             let end_u_other = self.curve.curve().project(*other.end).0;

    //                             let (other_start, other_end) = match self.direction {
    //                                 Direction::Increasing => match start_u_other < end_u_other {
    //                                     true => (&other.start, &other.end),
    //                                     false => (&other.end, &other.start),
    //                                 },
    //                                 Direction::Decreasing => match start_u_other < end_u_other {
    //                                     true => (&other.end, &other.start),
    //                                     false => (&other.start, &other.end),
    //                                 },
    //                             };

    //                             // Now that we have the right order
    //                             let start_u_other =
    //                                 self.curve.curve().project(**other_start).0;
    //                             let end_u_other = self.curve.curve().project(**other_end).0;

    //                             if start_u_other > self.end_u || end_u_other < self.start_u {
    //                                 return vec![];
    //                             }

    //                             let start = match start_u_other < self.start_u {
    //                                 true => self.start.clone(),
    //                                 false => other_start.clone(),
    //                             };

    //                             let end = match self.end_u < end_u_other {
    //                                 true => self.end.clone(),
    //                                 false => other_end.clone(),
    //                             };

    //                             if start == end {
    //                                 assert!(
    //                                     self.contains(*start) != EdgeContains::Outside
    //                                         && other.contains(*start)
    //                                             != EdgeContains::Outside
    //                                 );
    //                                 return vec![EdgeEdgeIntersection::Point(start)];
    //                             }

    //                             // println!("Direction: {:?}", self.direction);
    //                             // println!("Start: {:?}", start);
    //                             // println!("End: {:?}", end);

    //                             // println!("Self start u: {:?}", self.start_u);
    //                             // println!("Self end u: {:?}", self.end_u);
    //                             // println!("Other start u: {:?}", start_u_other);
    //                             // println!("Other end u: {:?}", end_u_other);

    //                             // println!("Self: {:?}", self);
    //                             // println!("Other: {:?}", other);

    //                             assert!(self.contains(*start) != EdgeContains::Outside);
    //                             assert!(other.contains(*start) != EdgeContains::Outside);
    //                             assert!(self.contains(*end) != EdgeContains::Outside);
    //                             assert!(other.contains(*end) != EdgeContains::Outside);
    //                             return vec![EdgeEdgeIntersection::Edge(Edge::new(
    //                                 start,
    //                                 end,
    //                                 self.curve.clone(),
    //                                 self.direction,
    //                             ))];
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        (Rc::ptr_eq(&self.curve, &other.curve) || self.curve == other.curve)
            && self.start == other.start
            && self.end == other.end
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self.curve.as_ref() {
            EdgeCurve::Line(_line) => write!(f, "Line {:?} - {:?}", self.start, self.end),
            EdgeCurve::Circle(_circle) => write!(f, "Circle {:?} - {:?}", self.start, self.end),
            EdgeCurve::Ellipse(_ellipse) => write!(f, "Ellipse {:?} - {:?}", self.start, self.end),
        }
    }
}
