use crate::{
    curves::{curve::Curve, CurveLike},
    efloat::EFloat64,
    point::Point,
    transforms::Transform,
};

pub struct CircleCoords {
    pub x: EFloat64,
    pub y: EFloat64,
}

impl CircleCoords {
    pub fn new(x: EFloat64, y: EFloat64) -> CircleCoords {
        assert!((x * x + y * y) == EFloat64::one());
        CircleCoords { x, y }
    }
}

trait CircleLike {
    fn map(&self, p: Point) -> Point;
    fn project(&self, p: Point) -> Point;
    fn transform(&self, transform: Transform) -> Curve;
}

impl CurveLike for dyn CircleLike {
    fn transform(&self, transform: Transform) -> Curve {
        self.transform(transform)
    }

    fn neg(&self) -> Curve {
        todo!()
    }

    fn tangent(&self, p: Point) -> Point {
        todo!()
    }

    fn on_curve(&self, p: Point) -> bool {
        todo!()
    }

    fn distance(&self, x: Point, y: Point) -> EFloat64 {
        todo!()
    }

    fn interpolate(&self, start: Option<Point>, end: Option<Point>, t: f64) -> Point {
        todo!()
    }

    fn between(&self, m: Point, start: Option<Point>, end: Option<Point>) -> bool {
        todo!()
    }

    fn get_midpoint(&self, start: Option<Point>, end: Option<Point>) -> Option<Point> {
        todo!()
    }

    fn project(&self, p: Point) -> Point {
        todo!()
    }

    fn get_bounding_box(
        &self,
        start: Option<Point>,
        end: Option<Point>,
    ) -> crate::bounding_box::BoundingBox {
        todo!()
    }

    fn sort(&self, points: Vec<Option<Point>>) -> Vec<Option<Point>> {
        todo!()
    }
}
