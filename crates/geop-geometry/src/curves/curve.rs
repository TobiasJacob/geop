use std::fmt::Debug;

use crate::{
    bounding_box::BoundingBox,
    efloat::SemiPositiveEFloat64,
    points::point::{NormalizedPoint, Point},
    transforms::Transform,
};

use super::{
    circle::{Circle, CircleTransform},
    ellipse::Ellipse,
    helix::Helix,
    line::Line,
    CurveLike,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Curve {
    Line(Line),
    Circle(Circle),
    Ellipse(Ellipse),
    Helix(Helix),
}

// This represents a curve, which can be a line or a circle.
impl CurveLike for Curve {
    // Transform
    fn transform(&self, transform: Transform) -> Curve {
        match self {
            Curve::Line(line) => Curve::Line(line.transform(transform)),
            Curve::Circle(circle) => match circle.transform(transform) {
                CircleTransform::Circle(circle) => Curve::Circle(circle),
                CircleTransform::Ellipse() => todo!("Implement this"),
            },
            Curve::Ellipse(ellipse) => Curve::Ellipse(ellipse.transform(transform)),
            Curve::Helix(helix) => Curve::Helix(helix.transform(transform)),
        }
    }

    // Change the direction of the curve
    fn neg(&self) -> Curve {
        match self {
            Curve::Line(line) => Curve::Line(line.neg()),
            Curve::Circle(circle) => Curve::Circle(circle.neg()),
            Curve::Ellipse(ellipse) => Curve::Ellipse(ellipse.neg()),
            Curve::Helix(helix) => Curve::Helix(helix.neg()),
        }
    }

    // Normalized Tangent / Direction of the curve at the given point.
    fn tangent(&self, p: Point) -> NormalizedPoint {
        match self {
            Curve::Line(line) => line.tangent(p),
            Curve::Circle(circle) => circle.tangent(p),
            Curve::Ellipse(ellipse) => ellipse.tangent(p),
            Curve::Helix(helix) => helix.tangent(p),
        }
    }

    // Checks if point is on the curve.
    fn on_curve(&self, p: Point) -> bool {
        match self {
            Curve::Line(line) => line.on_curve(p),
            Curve::Circle(circle) => circle.on_curve(p),
            Curve::Ellipse(ellipse) => ellipse.on_curve(p),
            Curve::Helix(helix) => helix.on_curve(p),
        }
    }

    // Returns the distance between x and y.
    fn distance(&self, x: Point, y: Point) -> SemiPositiveEFloat64 {
        match self {
            Curve::Line(line) => line.distance(x, y),
            Curve::Circle(circle) => circle.distance(x, y),
            Curve::Ellipse(ellipse) => ellipse.distance(x, y),
            Curve::Helix(helix) => helix.distance(x, y),
        }
    }

    // Interpolate between start and end at t. t is between 0 and 1.
    fn interpolate(&self, start: Option<Point>, end: Option<Point>, t: f64) -> Point {
        match self {
            Curve::Line(line) => line.interpolate(start, end, t),
            Curve::Circle(circle) => circle.interpolate(start, end, t),
            Curve::Ellipse(ellipse) => ellipse.interpolate(start, end, t),
            Curve::Helix(helix) => helix.interpolate(start, end, t),
        }
    }

    // Checks if m is between x and y. m==x and m==y are true.
    fn between(&self, m: Point, start: Option<Point>, end: Option<Point>) -> bool {
        match self {
            Curve::Line(line) => line.between(m, start, end),
            Curve::Circle(circle) => circle.between(m, start, end),
            Curve::Ellipse(ellipse) => ellipse.between(m, start, end),
            Curve::Helix(helix) => helix.between(m, start, end),
        }
    }

    // Get the midpoint between start and end.
    // This will guarantee that between(start, midpoint, end) is true and midpoint != start and midpoint != end.
    // If start or end is None, the midpoint is a point that is a unit distance away from the other point.
    fn get_midpoint(&self, start: Option<Point>, end: Option<Point>) -> Point {
        match self {
            Curve::Line(line) => line.get_midpoint(start, end),
            Curve::Circle(circle) => circle.get_midpoint(start, end),
            Curve::Ellipse(ellipse) => ellipse.get_midpoint(start, end),
            Curve::Helix(helix) => helix.get_midpoint(start, end),
        }
    }

    // Finds the closest point on the curve to the given point.
    fn project(&self, p: Point) -> Point {
        match self {
            Curve::Line(line) => line.project(p),
            Curve::Circle(circle) => circle.project(p),
            Curve::Ellipse(ellipse) => ellipse.project(p),
            Curve::Helix(helix) => helix.project(p),
        }
    }

    fn get_bounding_box(
        &self,
        interval_self: Option<Point>,
        midpoint_self: Option<Point>,
    ) -> BoundingBox {
        match self {
            Curve::Line(line) => line.get_bounding_box(interval_self, midpoint_self),
            Curve::Circle(circle) => circle.get_bounding_box(interval_self, midpoint_self),
            Curve::Ellipse(ellipse) => ellipse.get_bounding_box(interval_self, midpoint_self),
            Curve::Helix(helix) => helix.get_bounding_box(interval_self, midpoint_self),
        }
    }

    // Sorts a list of point such that for three consecutive points (p1, p2, p3) p2 is between p1 and p3.
    // For the first and last point, it is (p2, p3, ..., p1) and (p2, p1, ..., p3) respectively.
    fn sort(&self, points: Vec<Option<Point>>) -> Vec<Option<Point>> {
        match self {
            Curve::Line(line) => line.sort(points),
            Curve::Circle(circle) => circle.sort(points),
            Curve::Ellipse(ellipse) => ellipse.sort(points),
            Curve::Helix(helix) => helix.sort(points),
        }
    }
}
