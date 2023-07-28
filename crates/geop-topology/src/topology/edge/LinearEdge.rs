use std::rc::Rc;

use geop_geometry::{geometry::{points::point::Point, curves::{line::Line, circle::Circle, ellipse::Ellipse, curve::Curve}}, EQ_THRESHOLD, intersections::{circle_circle::{circle_circle_intersection, CircleCircleIntersection}, line_line::{line_line_intersection, LineLineIntersection}, self}};

use crate::topology::Vertex::Vertex;

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
        // It might seem weired to do this here and not simple add for example a curve.periodic() function if start < end.
        // The reason is that for edges it is possible to find parameter spaces relativly easy.
        // For surfaces, this is much more complicated, because we need a valid parameter space within a face that could span poles, which is bounded by an EdgeLoop.
        // In both cases, the parameter space is defined by the start and end point of the curve or the outer edge loop.
        // So the code that generates the parameter space (which depends on start and end) belongs here.
        let end_u = match *curve {
            LinearEdgeCurve::Line(_) => end_u_p,
            LinearEdgeCurve::Circle(_) => match end_u_p > start_u {
                true => end_u_p,
                false => end_u_p + 2.0 * std::f64::consts::PI,
            }
            LinearEdgeCurve::Ellipse(_) => match end_u_p > start_u {
                true => end_u_p,
                false => end_u_p + 2.0 * std::f64::consts::PI,
            }
        };
        
        LinearEdge {
            start,
            end,
            curve,
            start_u: curve.curve().project(&start.point),
            end_u: curve.curve().project(&start.point),
        }
    }

    pub fn point_at(&self, u: f64) -> Point {
        let u = self.start_u + u * (self.end_u - self.start_u);
        self.curve.curve().point_at(u)
    }

    pub fn project(&self, point: &Point) -> f64 {
        let u_p = self.curve.curve().project(point);
        let u = match u_p > self.start_u {
            true => u_p,
            false => u_p + 2.0 * std::f64::consts::PI,
        };
        (u - self.start_u) / (self.end_u - self.start_u)
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

        let intersections = intersections?.iter().map(|intersection| {
            let u = self.project(intersection);
            let v = other.project(intersection);
            let vertex = Vertex::new(Rc::new(*intersection));
            (u, v, vertex)
        }).filter(|&e| {
            let (u, v, vertex) = e;
            u > 0.0 + EQ_THRESHOLD && u < 1.0 - EQ_THRESHOLD && v > 0.0 + EQ_THRESHOLD && v < 1.0 - EQ_THRESHOLD
        }).collect::<Vec<(f64, f64, Vertex)>>();

        let mut intersectionsA = intersections.iter().map(|e| {
            let (u, v, vertex) = *e;
            (u, vertex)
        }).collect::<Vec<(f64, Vertex)>>();

        let mut intersectionsB = intersections.iter().map(|e| {
            let (u, v, vertex) = *e;
            (v, vertex)
        }).collect::<Vec<(f64, Vertex)>>();

        intersectionsA.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        intersectionsB.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        Ok((intersectionsA, intersectionsB))
    }

    pub fn remesh(&self, other: &LinearEdge) -> Result<(Vec<LinearEdge>, Vec<LinearEdge>), &str> {
        let (intersectionsA, intersectionsB) = self.inner_intersections(other)?;
        let mut edgesA = Vec::<LinearEdge>::with_capacity(intersectionsA.len() + 1);
        edgesA.push(LinearEdge::new(self.start, intersectionsA[0].1, self.curve));
        for i in 0..intersectionsA.len() - 1 {
            edgesA.push(LinearEdge::new(intersectionsA[i].1, intersectionsA[i + 1].1, self.curve));
        }
        edgesA.push(LinearEdge::new(intersectionsA[intersectionsA.len() - 1].1, self.end, self.curve));

        let mut edgesB = Vec::<LinearEdge>::with_capacity(intersectionsB.len() + 1);
        edgesB.push(LinearEdge::new(other.start, intersectionsB[0].1, other.curve));
        for i in 0..intersectionsB.len() - 1 {
            edgesB.push(LinearEdge::new(intersectionsB[i].1, intersectionsB[i + 1].1, other.curve));
        }
        edgesB.push(LinearEdge::new(intersectionsB[intersectionsB.len() - 1].1, other.end, other.curve));

        Ok((edgesA, edgesB))
    }
}

impl PartialEq for LinearEdge {
    fn eq(&self, other: &LinearEdge) -> bool {
        (Rc::ptr_eq(&self.curve, &other.curve) || self.curve == other.curve) && self.start == other.start && self.end == other.end
    }
}