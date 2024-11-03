use crate::{
    bounding_box::BoundingBox,
    color::Category10Color,
    efloat::EFloat64,
    geometry_error::{GeometryError, GeometryResult},
    geometry_scene::GeometryScene,
    point::Point,
    transforms::Transform,
    HORIZON_DIST,
};

use super::{curve::Curve, CurveLike};

#[derive(Debug, Clone)]
pub struct Line {
    pub basis: Point,
    pub direction: Point,
}

impl Line {
    pub fn new(basis: Point, direction: Point) -> GeometryResult<Line> {
        if !direction.is_normalized() {
            return Err(
                GeometryError::new("Direction must be normalized".to_string()).with_context_scene(
                    format!("Create a line at {} with direction {}.", basis, direction),
                    GeometryScene::with_points(vec![
                        (basis, Category10Color::Orange),
                        (basis + direction, Category10Color::Green),
                    ]),
                ),
            );
        }
        Ok(Line { basis, direction })
    }

    pub fn transform(&self, transform: Transform) -> Self {
        let basis = transform * self.basis;
        let direction = transform * (self.direction + self.basis) - basis;
        Line::new(basis, direction.normalize().unwrap()).expect("Direction must be normalized")
    }

    pub fn neg(&self) -> Line {
        Line::new(self.basis, -self.direction).expect("Direction is already normalized")
    }
}

impl CurveLike for Line {
    fn transform(&self, transform: Transform) -> Curve {
        Curve::Line(self.transform(transform))
    }

    fn neg(&self) -> Curve {
        Curve::Line(self.neg())
    }

    fn tangent(&self, _p: Point) -> GeometryResult<Point> {
        Ok(self.direction.clone())
    }

    fn on_curve(&self, p: Point) -> bool {
        let v = p - self.basis;
        let v = v - self.direction * (v.dot(self.direction));
        v.norm() == 0.0
    }

    fn distance(&self, x: Point, y: Point) -> GeometryResult<EFloat64> {
        if !self.on_curve(x) {
            return Err(GeometryError::new(format!("Point x {} is not on line", x))
                .with_context_scene(
                    format!("Calculate the distance between {} and {}.", x, y),
                    GeometryScene {
                        points: vec![(x, Category10Color::Orange)],
                        curves: vec![(Curve::Line(self.clone()), Category10Color::Gray)],
                        surfaces: vec![],
                    },
                ));
        };
        if !self.on_curve(y) {
            return Err(GeometryError::new(format!("Point y {} is not on line", y))
                .with_context_scene(
                    format!("Calculate the distance between {} and {}.", x, y),
                    GeometryScene {
                        points: vec![(y, Category10Color::Orange)],
                        curves: vec![(Curve::Line(self.clone()), Category10Color::Gray)],
                        surfaces: vec![],
                    },
                ));
        };
        let v = x - y;
        Ok(v.norm())
    }

    fn interpolate(
        &self,
        start: Option<Point>,
        end: Option<Point>,
        t: f64,
    ) -> GeometryResult<Point> {
        match (start, end) {
            (Some(start), Some(end)) => {
                assert!(self.on_curve(start));
                assert!(self.on_curve(end));
                Ok(start + (end - start) * EFloat64::from(t))
            }
            (Some(start), None) => Ok(start + self.direction * EFloat64::from(t * HORIZON_DIST)),
            (None, Some(end)) => {
                Ok(end - self.direction * EFloat64::from((1.0 - t) * HORIZON_DIST))
            }
            (None, None) => {
                Ok(self.basis + self.direction * EFloat64::from((t - 0.5) * 2.0 * HORIZON_DIST))
            }
        }
    }

    // Checks if m is between x and y. m==x and m==y are true.
    fn between(&self, m: Point, start: Option<Point>, end: Option<Point>) -> GeometryResult<bool> {
        assert!(self.on_curve(m));
        match (start, end) {
            (Some(start), Some(end)) => {
                assert!(self.on_curve(start));
                assert!(self.on_curve(end));
                Ok((m - start).dot(self.direction) >= 0.0 && (m - end).dot(self.direction) <= 0.0)
            }
            (Some(start), None) => {
                assert!(self.on_curve(start));
                Ok((m - start).dot(self.direction) >= 0.0)
            }
            (None, Some(end)) => {
                assert!(self.on_curve(end));
                Ok((m - end).dot(self.direction) <= 0.0)
            }
            (None, None) => Ok(true),
        }
    }

    fn get_midpoint(&self, start: Option<Point>, end: Option<Point>) -> GeometryResult<Point> {
        match (start, end) {
            (Some(start), Some(end)) => {
                assert!(self.on_curve(start));
                assert!(self.on_curve(end));
                Ok(((start + end) / EFloat64::two()).unwrap())
            }
            (Some(start), None) => Ok(start + self.direction * EFloat64::from(HORIZON_DIST)),
            (None, Some(end)) => Ok(end - self.direction * EFloat64::from(HORIZON_DIST)),
            (None, None) => Ok(self.basis),
        }
    }

    fn project(&self, p: Point) -> Point {
        let v = p - self.basis;
        self.basis + self.direction * v.dot(self.direction)
    }

    fn get_bounding_box(
        &self,
        _interval_self: Option<Point>,
        _midpoint_self: Option<Point>,
    ) -> GeometryResult<BoundingBox> {
        todo!()
    }

    fn shrink_bounding_box(
        &self,
        _start: Option<Point>,
        _end: Option<Point>,
        _bounding_box: BoundingBox,
    ) -> GeometryResult<BoundingBox> {
        todo!()
    }

    fn sort(&self, points: Vec<Option<Point>>) -> Vec<Option<Point>> {
        let mut points = points;
        points.sort_unstable_by(|a, b| {
            if let Some(a) = a {
                if let Some(b) = b {
                    let v = *a - *b;
                    v.dot(self.direction).partial_cmp(&0.0).unwrap()
                } else {
                    std::cmp::Ordering::Less
                }
            } else {
                if let Some(_b) = b {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            }
        });
        points
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Line) -> bool {
        self.direction == other.direction && (self.basis - other.basis).is_parallel(self.direction)
    }
}
