use std::rc::Rc;

use geop_geometry::{geometry::{points::point::Point, curves::{line::Line, circle::Circle, ellipse::Ellipse}}, EQ_THRESHOLD};

use crate::topology::Vertex::Vertex;

pub enum LinearEdgeCurve {
    Line(Line),
    Circle(Circle),
    Ellipse(Ellipse),
}

pub struct LinearEdge {
    pub start: Vertex,
    pub end: Vertex,
    pub curve: Rc<LinearEdgeCurve>
}

// TODO: Implement an periodic / circular edge
impl LinearEdge {
    pub fn new(start: Vertex, end: Vertex, curve: Rc<LinearEdgeCurve>) -> LinearEdge {
        LinearEdge {
            start,
            end,
            curve,
        }
    }

    pub fn interval(&self) -> (f64, f64) {
        return self.curve.curve().interval(&self.vertices[0].point, &self.vertices[1].point);
    }

    pub fn length(&self) -> f64 {
        self.parameter_space.length()
    }

    pub fn point_at(&self, u: f64) -> Point {
        let (start, end) = self.interval();
        self.curve.curve().point_at(start + u)
    }

    pub fn project(&self, point: &Point) -> f64 {
        let (start, end) = self.interval();
        let (start, u) = self.curve.curve().interval(&self.vertices[0].point, point);
        assert!(u <= end);
        return u - start;
    }

    pub fn rasterize(&self) -> Vec<Point> {
        let num_points = 40 as usize;
        let (start, end) = self.interval();

        (0..num_points).map(|i| {
            let t = i as f64 / (num_points - 1) as f64;
            let point = self.curve.curve().point_at(t);
            let point = point + (end - start) * t;
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
