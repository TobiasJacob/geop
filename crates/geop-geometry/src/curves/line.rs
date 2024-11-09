use std::fmt::Display;

use geop_algebra::efloat::EFloat64;

use crate::{
    bounding_box::BoundingBox,
    color::Category10Color,
    geometry_error::{GeometryError, GeometryResult, WithContext},
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

    fn assert_on_curve(&self, p: Point, variable_name: &str) -> GeometryResult<()> {
        if !self.on_curve(p) {
            return Err(GeometryError::new(format!(
                "Point {} {} is not on line {}",
                variable_name, p, self
            )));
        }
        Ok(())
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
        let error_context = |err: GeometryError| {
            err.with_context_scene(
                format!("Calculate the distance between {} and {}.", x, y),
                GeometryScene {
                    points: vec![(x, Category10Color::Orange), (y, Category10Color::Orange)],
                    curves: vec![(Curve::Line(self.clone()), Category10Color::Gray)],
                    surfaces: vec![],
                },
            )
        };

        self.assert_on_curve(x, "x").with_context(&error_context)?;
        self.assert_on_curve(y, "y").with_context(&error_context)?;
        let v = x - y;
        Ok(v.norm())
    }

    fn interpolate(
        &self,
        start: Option<Point>,
        end: Option<Point>,
        t: f64,
    ) -> GeometryResult<Point> {
        let error_context = |err: GeometryError| {
            err.with_context_scene(
                format!(
                    "Interpolating between {:?} and {:?} with t={}",
                    start, end, t
                ),
                GeometryScene {
                    points: vec![
                        (start, Category10Color::Orange),
                        (end, Category10Color::Blue),
                    ]
                    .into_iter()
                    .filter_map(|(p, c)| p.map(|p| (p, c)))
                    .collect(),
                    curves: vec![(Curve::Line(self.clone()), Category10Color::Gray)],
                    surfaces: vec![],
                },
            )
        };

        match (start, end) {
            (Some(start), Some(end)) => {
                self.assert_on_curve(start, "start")
                    .with_context(&error_context)?;
                self.assert_on_curve(end, "end")
                    .with_context(&error_context)?;
                Ok(start + (end - start) * EFloat64::from(t))
            }
            (Some(start), None) => {
                self.assert_on_curve(start, "start")
                    .with_context(&error_context)?;
                Ok(start + self.direction * EFloat64::from(t * HORIZON_DIST))
            }
            (None, Some(end)) => {
                self.assert_on_curve(end, "end")
                    .with_context(&error_context)?;
                Ok(end - self.direction * EFloat64::from((1.0 - t) * HORIZON_DIST))
            }
            (None, None) => {
                Ok(self.basis + self.direction * EFloat64::from((t - 0.5) * 2.0 * HORIZON_DIST))
            }
        }
    }

    // Checks if m is between x and y. m==x and m==y are true.
    fn between(&self, m: Point, start: Option<Point>, end: Option<Point>) -> GeometryResult<bool> {
        let error_context = |err: GeometryError| {
            err.with_context_scene(
                format!("Checking if {} is between {:?} and {:?}", m, start, end),
                GeometryScene {
                    points: vec![
                        (start, Category10Color::Orange),
                        (end, Category10Color::Blue),
                        (Some(m), Category10Color::Green),
                    ]
                    .into_iter()
                    .filter_map(|(p, c)| p.map(|p| (p, c)))
                    .collect(),
                    curves: vec![(Curve::Line(self.clone()), Category10Color::Gray)],
                    surfaces: vec![],
                },
            )
        };

        self.assert_on_curve(m, "m").with_context(&error_context)?;
        match (start, end) {
            (Some(start), Some(end)) => {
                self.assert_on_curve(start, "start")
                    .with_context(&error_context)?;
                self.assert_on_curve(end, "end")
                    .with_context(&error_context)?;
                Ok((m - start).dot(self.direction) >= 0.0 && (m - end).dot(self.direction) <= 0.0)
            }
            (Some(start), None) => {
                self.assert_on_curve(start, "start")
                    .with_context(&error_context)?;
                Ok((m - start).dot(self.direction) >= 0.0)
            }
            (None, Some(end)) => {
                self.assert_on_curve(end, "end")
                    .with_context(&error_context)?;
                Ok((m - end).dot(self.direction) <= 0.0)
            }
            (None, None) => Ok(true),
        }
    }

    fn get_midpoint(&self, start: Option<Point>, end: Option<Point>) -> GeometryResult<Point> {
        let error_context = |err: GeometryError| {
            err.with_context_scene(
                format!("Midpoint between {:?} and {:?}", start, end),
                GeometryScene {
                    points: vec![
                        (start, Category10Color::Orange),
                        (end, Category10Color::Blue),
                    ]
                    .into_iter()
                    .filter_map(|(p, c)| p.map(|p| (p, c)))
                    .collect(),
                    curves: vec![(Curve::Line(self.clone()), Category10Color::Gray)],
                    surfaces: vec![],
                },
            )
        };

        match (start, end) {
            (Some(start), Some(end)) => {
                if start == end {
                    return Err(error_context(GeometryError::new(
                        "Start and end are the same".to_string(),
                    )));
                }

                self.assert_on_curve(start, "start")
                    .with_context(&error_context)?;
                self.assert_on_curve(end, "end")
                    .with_context(&error_context)?;
                Ok(((start + end) / EFloat64::two()).unwrap())
            }
            (Some(start), None) => {
                self.assert_on_curve(start, "start")
                    .with_context(&error_context)?;
                Ok(start + self.direction * EFloat64::from(HORIZON_DIST))
            }
            (None, Some(end)) => {
                self.assert_on_curve(end, "end")
                    .with_context(&error_context)?;
                Ok(end - self.direction * EFloat64::from(HORIZON_DIST))
            }
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

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Line {} + t * {}", self.basis, self.direction)
    }
}
