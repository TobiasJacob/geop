use std::{rc::Rc, cmp::Ordering, fmt::Debug};

use crate::{points::point::Point, transforms::Transform, EQ_THRESHOLD};

use super::{line::Line, circle::{Circle, CircleTransform}, ellipse::Ellipse};


#[derive(Debug, Clone, PartialEq)]
pub enum Curve {
    Line(Line),
    Circle(Circle),
    Ellipse(Ellipse),
}

pub enum CurveIntersection {
    None,
    Point(Point),
    Interval(Point, Point),
    IntervalAndPoint(Point, Point, Point), // Migh happen, e.g. if two half circles intersect at both ends.
    DualInterval(Point, Point, Point, Point), 
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
            Curve::Ellipse(ellipse) => {
                Curve::Ellipse(ellipse.transform(transform))
            }
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
    pub fn interpolate(&self, start: Point, end: Point, t: f64) -> Point {
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
    pub fn between(&self, m: Point, start: Point, end: Point) -> bool {
        match self {
            Curve::Line(line) => line.between(m, start, end),
            Curve::Circle(circle) => circle.between(m, start, end),
            Curve::Ellipse(ellipse) => ellipse.between(m, start, end),
        }
    }
    // Get the midpoint between start and end. Not that this is well defined even on a circle, because the midpoint is between start and end.
    pub fn get_midpoint(&self, start: Point, end: Point) -> Point {
        match self {
            Curve::Line(line) => line.get_midpoint(start, end),
            Curve::Circle(circle) => circle.get_midpoint(start, end),
            Curve::Ellipse(ellipse) => ellipse.get_midpoint(start, end),
        }
    }

    // Intersect between start1/2 and end1/2. Returns None if there is no intersection.
    // Keep in mind that all curves are treated as infinite lines, such that start after end means that the line starts, goes to +infinity, goes to -infinty and then ends.
    pub fn intersect(&self, start1: Point, end1: Point, start2: Point, end2: Point) -> CurveIntersection {
        print!("intersect: {:?}, {:?}, {:?}, {:?}\n", start1, end1, start2, end2);
        assert!(start1 != end1);
        assert!(start2 != end2);
        let mut solutions = vec![];
        for (s, e) in [(&start1, &end1), (&start2, &end2), (&start1, &end2), (&start2, &end1)] {
            if self.between(*s, start1, end1) && self.between(*e, start1, end1) && self.between(*s, start2, end2) && self.between(*e, start2, end2) {
                println!("intersect_done: {:?}, {:?}\n", s, e);
                let mut already_in_solution = false;
                for (s2, e2) in solutions.iter() {
                    if s == s2 && e == e2 {
                        already_in_solution = true;
                        break;
                    }
                }
                if !already_in_solution {
                    solutions.push((s.clone(), e.clone()));
                }
            }
        }
        match solutions.len() {
            0 => {
                return CurveIntersection::None;
            },
            1 => {
                let (s, e) = solutions[0].clone();
                if s == e {
                    return CurveIntersection::Point(s.clone());
                } else {
                    return CurveIntersection::Interval(s.clone(), e.clone());
                }
            },
            2 => {
                let (s1, e1) = solutions[0].clone();
                let (s2, e2) = solutions[1].clone();
                if s1 == s2 && e1 == e2 {
                    panic!("Should not happen");
                } else if s1 == e1 {
                    return CurveIntersection::IntervalAndPoint(s2.clone(), e2.clone(), s1.clone());
                } else if s2 == e2 {
                    return CurveIntersection::IntervalAndPoint(s1.clone(), e1.clone(), s2.clone());
                } else {
                    return CurveIntersection::DualInterval(s1.clone(), e1.clone(), s2.clone(), e2.clone());
                }
            },
            _ => {
                panic!("More than two intersections. Should not happen.");
            }
        }
    }
}
