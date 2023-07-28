use std::rc::Rc;

use geop_geometry::{geometry::{points::point::Point, curves::{line::Line, circle::Circle, ellipse::Ellipse, curve::Curve}}, intersections::{circle_circle::{circle_circle_intersection, CircleCircleIntersection}, line_line::{line_line_intersection, LineLineIntersection}}};

use crate::{topology::Vertex::Vertex, PROJECTION_THRESHOLD};

#[derive(PartialEq)]
pub enum LinearEdgeCurve {
    Line(Line),
    Circle(Circle),
    Ellipse(Ellipse),
}
impl LinearEdgeCurve {
    pub fn curve(&self) -> &dyn Curve {
        match self {
            LinearEdgeCurve::Line(line) => line,
            LinearEdgeCurve::Circle(circle) => circle,
            LinearEdgeCurve::Ellipse(ellipse) => ellipse,
        }
    }
}

pub struct LinearEdge {
    pub start: Vertex,
    pub end: Vertex,
    pub curve: Rc<LinearEdgeCurve>,
    start_u: f64,
    end_u: f64,
}

// TODO: Implement an periodic / circular edge
impl LinearEdge {
    pub fn new(start: Vertex, end: Vertex, curve: Rc<LinearEdgeCurve>) -> LinearEdge {
        let start_u = curve.curve().project(&start.point);
        let end_u_p = curve.curve().project(&end.point);
        assert!(start_u.1 < PROJECTION_THRESHOLD);
        assert!(end_u_p.1 < PROJECTION_THRESHOLD);
        // It might seem weired to do this here and not simple add for example a curve.periodic() function if start < end.
        // The reason is that for edges it is possible to find parameter spaces relativly easy.
        // For surfaces, this is much more complicated, because we need a valid parameter space within a face that could span poles, which is bounded by an EdgeLoop.
        // In both cases, the parameter space is defined by the start and end point of the curve or the outer edge loop.
        // So the code that generates the parameter space (which depends on start and end) belongs here.
        let end_u = match *curve {
            LinearEdgeCurve::Line(_) => end_u_p.0,
            LinearEdgeCurve::Circle(_) => match end_u_p > start_u {
                true => end_u_p.0,
                false => end_u_p.0 + 2.0 * std::f64::consts::PI,
            }
            LinearEdgeCurve::Ellipse(_) => match end_u_p > start_u {
                true => end_u_p.0,
                false => end_u_p.0 + 2.0 * std::f64::consts::PI,
            }
        };
        
        LinearEdge {
            start,
            end,
            curve,
            start_u: curve.curve().project(&start.point).0,
            end_u,
        }
    }

    pub fn point_at(&self, u: f64) -> Point {
        assert!(u >= 0.0 && u < 1.0);
        let u = self.start_u + u * (self.end_u - self.start_u);
        self.curve.curve().point_at(u)
    }

    pub fn project(&self, point: &Point) -> Option<f64> {
        let u_p = self.curve.curve().project(point);
        if u_p.1 > PROJECTION_THRESHOLD {
            return None;
        }
        let u = match u_p.0 > self.start_u {
            true => u_p.0,
            false => u_p.0 + 2.0 * std::f64::consts::PI,
        };
        if u < self.start_u || u > self.end_u {
            return None;
        }
        Some((u - self.start_u) / (self.end_u - self.start_u))
    }

    pub fn rasterize(&self) -> Vec<Point> {
        let num_points = 40 as usize;

        (0..num_points).map(|i| {
            let t = i as f64 / (num_points - 1) as f64;
            let point = self.curve.curve().point_at(t);
            point
        }).collect()
    }

    // All intersections with other edge. The end points are not included.
    pub fn inner_intersections(&self, other: &LinearEdge) -> Result<(Vec<(f64, Vertex)>, Vec<(f64, Vertex)>), &str> {
        if self == other {
            return Err("The edges are equal")
        }
        let intersections: Result<Vec<Point>, &str> = match *self.curve {
            LinearEdgeCurve::Circle(ref circle) => {
                match *other.curve {
                    LinearEdgeCurve::Circle(ref other_circle) => {
                        match circle_circle_intersection(circle, other_circle) {
                            CircleCircleIntersection::TwoPoint(a, b) => {
                                Ok(vec![a, b])
                            },
                            CircleCircleIntersection::OnePoint(a) => {
                                Ok(vec![a])
                            },
                            CircleCircleIntersection::None => {
                                Ok(vec![])
                            }
                            _ => {
                                Err("Geometries overlap")
                            }
                        }
                    },
                    LinearEdgeCurve::Ellipse(ref ellipse) => {
                        todo!("Implement circle-ellipse intersection")
                    },
                    LinearEdgeCurve::Line(ref line) => {
                        todo!("Implement circle-line intersection")
                    },
                }
            },
            LinearEdgeCurve::Ellipse(ref ellipse) => {
                todo!("Implement ellipse intersection")
            },
            LinearEdgeCurve::Line(ref line) => {
                match *other.curve {
                    LinearEdgeCurve::Circle(ref circle) => {
                        todo!("Implement line-circle intersection")
                    },
                    LinearEdgeCurve::Ellipse(ref ellipse) => {
                        todo!("Implement line-ellipse intersection")
                    },
                    LinearEdgeCurve::Line(ref other_line) => {
                        match line_line_intersection(line, other_line) {
                            LineLineIntersection::Point(a) => {
                                Ok(vec![a])
                            },
                            LineLineIntersection::None => {
                                Ok(vec![])
                            },
                            LineLineIntersection::Line(a) => {
                                Err("Geometries overlap")
                            },
                        }
                    },
                }
            },
        };

        let intersections = intersections?.iter().filter_map(|intersection| {
            let u = self.project(intersection)?;
            let v = other.project(intersection)?;
            let vertex = Vertex::new(Rc::new(*intersection));
            Some((u, v, vertex))
        }).collect::<Vec<(f64, f64, Vertex)>>();

        let mut intersections_a = intersections.iter().map(|e| {
            let (u, v, vertex) = *e;
            (u, vertex)
        }).collect::<Vec<(f64, Vertex)>>();

        let mut intersections_b = intersections.iter().map(|e| {
            let (u, v, vertex) = *e;
            (v, vertex)
        }).collect::<Vec<(f64, Vertex)>>();

        intersections_a.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        intersections_b.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        Ok((intersections_a, intersections_b))
    }

    pub fn remesh(&self, other: &LinearEdge) -> Result<(Vec<LinearEdge>, Vec<LinearEdge>), &str> {
        let (intersections_a, intersections_b) = self.inner_intersections(other)?;
        let mut edges_a = Vec::<LinearEdge>::with_capacity(intersections_a.len() + 1);
        edges_a.push(LinearEdge::new(self.start, intersections_a[0].1, self.curve));
        for i in 0..intersections_a.len() - 1 {
            edges_a.push(LinearEdge::new(intersections_a[i].1, intersections_a[i + 1].1, self.curve));
        }
        edges_a.push(LinearEdge::new(intersections_a[intersections_a.len() - 1].1, self.end, self.curve));

        let mut edges_b = Vec::<LinearEdge>::with_capacity(intersections_b.len() + 1);
        edges_b.push(LinearEdge::new(other.start, intersections_b[0].1, other.curve));
        for i in 0..intersections_b.len() - 1 {
            edges_b.push(LinearEdge::new(intersections_b[i].1, intersections_b[i + 1].1, other.curve));
        }
        edges_b.push(LinearEdge::new(intersections_b[intersections_b.len() - 1].1, other.end, other.curve));

        Ok((edges_a, edges_b))
    }
}

impl PartialEq for LinearEdge {
    fn eq(&self, other: &LinearEdge) -> bool {
        (Rc::ptr_eq(&self.curve, &other.curve) || self.curve == other.curve) && self.start == other.start && self.end == other.end
    }
}