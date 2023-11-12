use geop_geometry::{curves::{circle::{Circle, CircleTransform}, curve::Curve, ellipse::Ellipse, line::Line}, transforms::Transform};

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
