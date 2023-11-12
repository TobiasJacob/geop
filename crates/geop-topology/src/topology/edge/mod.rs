pub mod edge_curve;
pub mod split_if_necessary;
pub mod intersect_edge;
pub mod contains_point;


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

use self::edge_curve::EdgeCurve;

#[derive(Clone, Debug)]
pub struct Edge {
    pub start: Rc<Point>,
    pub end: Rc<Point>,
    pub curve: Rc<EdgeCurve>,

    start_u: f64,
    end_u: f64,
}

#[derive(Clone, Debug)]
pub enum EdgeIntersection {
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
    pub fn new(start: Rc<Point>, end: Rc<Point>, curve: Rc<EdgeCurve>) -> Edge {
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
            EdgeCurve::Circle(_) => match end_u_p < start_u {
                true => end_u_p.0 + 2.0 * std::f64::consts::PI,
                false => end_u_p.0,
            },
            EdgeCurve::Ellipse(_) => match end_u_p < start_u {
                true => end_u_p.0 + 2.0 * std::f64::consts::PI,
                false => end_u_p.0,
            },
        };

        let start_u = start_u.0;

        Edge {
            start,
            end,
            curve,
            start_u,
            end_u,
        }
    }

    pub fn neg(&self) -> Edge {
        Edge::new(
            self.end.clone(),
            self.start.clone(),
            Rc::new(self.curve.neg()),
        )
    }

    pub fn transform(&self, transform: Transform) -> Edge {
        Edge::new(
            Rc::new(transform * *self.start),
            Rc::new(transform * *self.end),
            Rc::new(self.curve.transform(transform)),
        )
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
        let u = u_p.0;
        if u < self.start_u - EQ_THRESHOLD || u > self.end_u + EQ_THRESHOLD {
            return None;
        }
        Some((u - self.start_u) / (self.end_u - self.start_u))
    }
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
