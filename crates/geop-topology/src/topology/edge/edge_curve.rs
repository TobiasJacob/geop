use geop_geometry::{curves::{circle::{Circle, CircleTransform}, curve::Curve, ellipse::Ellipse, line::Line}, transforms::Transform, points::point::Point, curve_curve_intersection::{line_line::{LineLineIntersection, line_line_intersection}, circle_line::{circle_line_intersection, CircleLineIntersection}, circle_circle::{circle_circle_intersection, CircleCircleIntersection}}};

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

    // pub fn intersect(&self, other: &EdgeCurve) -> Vec<Point> {
    //     assert!(self != other);
    //     match self {
    //         EdgeCurve::Circle(circle) => match other {
    //             EdgeCurve::Circle(other) => {
    //                 let intersection = circle_circle_intersection(circle, &other);
    //                 match intersection {
    //                     CircleCircleIntersection::Circle(circle) => panic!("Circles should not be equal"),
    //                                             CircleCircleIntersection::OnePoint(point) => vec![point],
    //                     CircleCircleIntersection::TwoPoint(point_a, point_b) => vec![point_a, point_b],
    //                     CircleCircleIntersection::None => vec![],
    //                 }
    //             }
    //             EdgeCurve::Ellipse(other) => {
    //                 todo!("Implement intersection between circle and ellipse");
    //             }
    //             EdgeCurve::Line(other) => {
    //                 let intersection = circle_line_intersection(circle, other);
    //                 match intersection {
    //                     CircleLineIntersection::OnePoint(point) => vec![point],
    //                     CircleLineIntersection::TwoPoint(point_a, point_b) => vec![point_a, point_b],
    //                     CircleLineIntersection::None => vec![],
    //                 }
    //             }
    //         },
    //         EdgeCurve::Line(line) => match other {
    //             EdgeCurve::Circle(other) => {
    //                 let intersection = circle_line_intersection(&other, line);
    //                 match intersection {
    //                     CircleLineIntersection::OnePoint(point) => vec![point],
    //                     CircleLineIntersection::TwoPoint(point_a, point_b) => vec![point_a, point_b],
    //                     CircleLineIntersection::None => vec![],
    //                 }
    //             }
    //             EdgeCurve::Ellipse(other) => {
    //                 todo!("Implement intersection between line and ellipse");
    //             }
    //             EdgeCurve::Line(other) => {
    //                 let intersection = line_line_intersection(line, &other);
    //                 match intersection {
    //                     LineLineIntersection::Point(point) => vec![point],
    //                     LineLineIntersection::Line(_) => vec![],
    //                     LineLineIntersection::None => vec![],
    //                 }
    //             }
    //         },
    //         EdgeCurve::Ellipse(ellipse) => match other {
    //             EdgeCurve::Circle(other) => {
    //                 todo!("Implement intersection between ellipse and circle");
    //             }
    //             EdgeCurve::Ellipse(other) => {
    //                 todo!("Implement intersection between ellipse and ellipse");
    //             }
    //             EdgeCurve::Line(other) => {
    //                 todo!("Implement intersection between ellipse and line");
    //             }
    //         },
    //     }
    // }
}
