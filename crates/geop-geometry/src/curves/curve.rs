use std::fmt::Debug;

use crate::{points::point::Point, transforms::Transform};

use super::{
    circle::{Circle, CircleTransform},
    ellipse::Ellipse,
    line::Line,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Curve {
    Line(Line),
    Circle(Circle),
    Ellipse(Ellipse),
}

// This represents an oriented curve. Curves with redundant representations (e.g. a line with the direction vector not being normalized) have to be normalized in the new function. Unnormalized curves are not allowed.
impl Curve {
    // Transform
    pub fn transform(&self, transform: Transform) -> Curve {
        match self {
            Curve::Line(line) => Curve::Line(line.transform(transform)),
            Curve::Circle(circle) => match circle.transform(transform) {
                CircleTransform::Circle(circle) => Curve::Circle(circle),
                CircleTransform::Ellipse(ellipse) => Curve::Ellipse(ellipse),
            },
            Curve::Ellipse(ellipse) => Curve::Ellipse(ellipse.transform(transform)),
        }
    }

    pub fn neg(&self) -> Curve {
        match self {
            Curve::Line(line) => Curve::Line(line.neg()),
            Curve::Circle(circle) => Curve::Circle(circle.neg()),
            Curve::Ellipse(ellipse) => Curve::Ellipse(ellipse.neg()),
        }
    }

    // fn project(&self, p: Point) -> (f64, f64);
    // Tangent / Direction of the curve at the given point. Not normalized.
    pub fn tangent(&self, p: Point) -> Point {
        match self {
            Curve::Line(line) => line.tangent(p),
            Curve::Circle(circle) => circle.tangent(p),
            Curve::Ellipse(ellipse) => ellipse.tangent(p),
        }
    }

    // Checks if point is on manifold
    pub fn on_manifold(&self, p: Point) -> bool {
        match self {
            Curve::Line(line) => line.on_manifold(p),
            Curve::Circle(circle) => circle.on_manifold(p),
            Curve::Ellipse(ellipse) => ellipse.on_manifold(p),
        }
    }

    // Interpolate between start and end at t. t is between 0 and 1.
    pub fn interpolate(&self, start: Option<Point>, end: Option<Point>, t: f64) -> Point {
        match self {
            Curve::Line(line) => line.interpolate(start, end, t),
            Curve::Circle(circle) => circle.interpolate(start, end, t),
            Curve::Ellipse(ellipse) => ellipse.interpolate(start, end, t),
        }
    }

    // // Returns the Riemannian metric between u and v
    // fn metric(&self, x: Point, u: TangentParameter, v: TangentParameter) -> f64;
    // // Returns the Riemannian distance between x and y (x and y on manifold).
    // fn distance(&self, x: Point, y: Point) -> f64;
    // // Exponential of u at base x. u_z is ignored.
    // fn exp(&self, x: Point, u: TangentParameter) -> Point;
    // // Log of y at base x. Z coordinate is set to 0.
    // fn log(&self, x: Point, y: Point) -> TangentParameter;
    // // Parallel transport of v from x to y.
    // fn parallel_transport(&self, v: TangentParameter, x: Point, y: Point) -> TangentParameter;
    // Checks if m is between x and y. m==x and m==y are true.
    pub fn between(&self, m: Point, start: Option<Point>, end: Option<Point>) -> bool {
        match self {
            Curve::Line(line) => line.between(m, start, end),
            Curve::Circle(circle) => circle.between(m, start, end),
            Curve::Ellipse(ellipse) => ellipse.between(m, start, end),
        }
    }
    // Get the midpoint between start and end. Not that this is well defined even on a circle, because the midpoint is between start and end.
    // If start or end is None, the midpoint is a point that is a unit distance away from the other point.
    pub fn get_midpoint(&self, start: Option<Point>, end: Option<Point>) -> Point {
        match self {
            Curve::Line(line) => line.get_midpoint(start, end),
            Curve::Circle(circle) => circle.get_midpoint(start, end),
            Curve::Ellipse(ellipse) => ellipse.get_midpoint(start, end),
        }
    }

    pub fn project(&self, p: Point) -> Point {
        match self {
            Curve::Line(line) => line.project(p),
            Curve::Circle(circle) => circle.project(p),
            Curve::Ellipse(ellipse) => ellipse.project(p),
        }
    }
}
