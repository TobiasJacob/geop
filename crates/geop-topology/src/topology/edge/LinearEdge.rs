use std::rc::Rc;

use geop_geometry::{geometry::{points::point::Point, curves::{line::Line, circle::Circle, ellipse::Ellipse}}, EQ_THRESHOLD};

use crate::topology::Vertex::Vertex;

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

    // Returns a sorted list of intersections. The intersections are sorted by the parameter of the first curve. Start and end points are not included.
    pub fn inner_intersections(&self, other: &LinearEdge) -> Vec<Point> {
        let intersections = self.curve.intersections(&other.curve);
        let (u_min, u_max) = self.interval();
        match intersections {
            geop_geometry::intersections::curve_curve::IntersectableCurveResult::MultiPoint(points) => {
                points.into_iter().filter(|p| {
                    let (_, u) = self.curve.curve().interval(&self.vertices[0].point, &p);
                    u_min + EQ_THRESHOLD < u && u < u_max - EQ_THRESHOLD
                }).collect::<Vec<Point>>()
            },
            geop_geometry::intersections::curve_curve::IntersectableCurveResult::Point(point) => {
                let (_, u) = self.curve.curve().interval(&self.vertices[0].point, &point);
                if u_min + EQ_THRESHOLD < u && u < u_max - EQ_THRESHOLD {
                    vec![point]
                } else {
                    Vec::new()
                }
            },
            _ => Vec::new()
        }
    }
}
