use geop_geometry::{curves::{circle::{Circle, CircleTransform}, curve::Curve, ellipse::Ellipse, line::Line}, transforms::Transform, curve_curve_intersection::{line_line::{line_line_intersection, LineLineIntersection}, circle_circle::{circle_circle_intersection, CircleCircleIntersection}, circle_line::{circle_line_intersection, CircleLineIntersection}}, points::point::Point};


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

    pub fn neg(&self) -> EdgeCurve {
        match self {
            EdgeCurve::Line(line) => EdgeCurve::Line(line.neg()),
            EdgeCurve::Circle(circle) => EdgeCurve::Circle(circle.neg()),
            EdgeCurve::Ellipse(ellipse) => EdgeCurve::Ellipse(ellipse.neg()),
        }
    }
}

pub enum EdgeCurveIntersection {
    None,
    Points(Vec<Point>),
    Curve(EdgeCurve),
}

pub fn edge_curve_edge_curve_intersect(edge_self: &EdgeCurve, edge_other: &EdgeCurve) -> EdgeCurveIntersection {
    match edge_self {
        EdgeCurve::Line(line) => match edge_other {
            EdgeCurve::Line(other_line) => {
                match line_line_intersection(line, other_line) {
                    LineLineIntersection::None => EdgeCurveIntersection::None,
                    LineLineIntersection::Point(p) => EdgeCurveIntersection::Points(vec![p]),
                    LineLineIntersection::Line(l) => EdgeCurveIntersection::Curve(EdgeCurve::Line(l)),
                }
            }
            EdgeCurve::Circle(other_circle) => {
                match circle_line_intersection(other_circle, line) {
                    CircleLineIntersection::None => EdgeCurveIntersection::None,
                    CircleLineIntersection::OnePoint(p) => EdgeCurveIntersection::Points(vec![p]),
                    CircleLineIntersection::TwoPoint(p1, p2) => EdgeCurveIntersection::Points(vec![p1, p2]),
                }
            }
            EdgeCurve::Ellipse(other_ellipse) => {
                todo!("Line-Ellipse intersection")
            }
        },
        EdgeCurve::Circle(circle) => match edge_other {
            EdgeCurve::Line(other_line) => {
                todo!("Circle-Line intersection")
            }
            EdgeCurve::Circle(other_circle) => {
                match circle_circle_intersection(circle, other_circle) {
                    CircleCircleIntersection::None => EdgeCurveIntersection::None,
                    CircleCircleIntersection::OnePoint(p) => EdgeCurveIntersection::Points(vec![p]),
                    CircleCircleIntersection::TwoPoint(p1, p2) => EdgeCurveIntersection::Points(vec![p1, p2]),
                    CircleCircleIntersection::Circle(c) => EdgeCurveIntersection::Curve(EdgeCurve::Circle(c)),
                }
            }
            EdgeCurve::Ellipse(other_ellipse) => {
                todo!("Circle-Ellipse intersection")
            }
        },
        EdgeCurve::Ellipse(ellipse) => match edge_other {
            EdgeCurve::Line(other_line) => {
                todo!("Ellipse-Line intersection")
            }
            EdgeCurve::Circle(other_circle) => {
                todo!("Ellipse-Circle intersection")
            }
            EdgeCurve::Ellipse(other_ellipse) => {
                todo!("Ellipse-Ellipse intersection")
            }
        },
    }
}
