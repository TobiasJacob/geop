use std::{rc::Rc, env::consts};

use geop_geometry::{geometry::{points::point::Point, curves::{ellipse::Ellipse, curve::Curve}, curves::circle::Circle}, intersections::circle_circle::{circle_circle_intersection, CircleCircleIntersection}};

#[derive(PartialEq)]
pub enum CircularEdgeCurve {
    Circle(Circle),
    Ellipse(Ellipse),
}
impl CircularEdgeCurve {
    pub fn curve(&self) -> &dyn Curve {
        match self {
            CircularEdgeCurve::Circle(circle) => circle,
            CircularEdgeCurve::Ellipse(ellipse) => ellipse,
        }
    }
}

pub struct CircularEdge {
    pub curve: Rc<CircularEdgeCurve>
}

// TODO: Implement an periodic / circular edge
impl CircularEdge {
    pub fn new(curve: Rc<CircularEdgeCurve>) -> CircularEdge {
        CircularEdge {
            curve,
        }
    }

    // In our mesh representation, the edges are equal if the curves are equal.
    pub fn equals(&self, other: &CircularEdge) -> bool {
        Rc::ptr_eq(&self.curve, &other.curve)
    }

    pub fn rasterize(&self) -> Vec<Point> {
        let num_points = 40 as usize;
        let (start, end): (f64, f64) = match *self.curve {
            CircularEdgeCurve::Circle(ref circle) => 
                (0.0 as f64, 2.0 * std::f64::consts::PI)
            ,
            CircularEdgeCurve::Ellipse(ref ellipse) => 
                (0.0 as f64, 2.0 * std::f64::consts::PI)
            ,
        };

        (0..num_points).map(|i| {
            let t = i as f64 / (num_points - 1) as f64;
            let point = self.curve.curve().point_at(t);
            let point = point + (end - start) * t;
            point
        }).collect()
    }

    // Returns a sorted list of intersections. The intersections are sorted by the parameter of the first curve. Start and end points are not included.
    pub fn inner_intersections(&self, other: &CircularEdge) -> Result<Vec<Point>, &str> {
        assert!(!self.equals(other)); // This means there are not infinity many intersections
        match *self.curve {
            CircularEdgeCurve::Circle(ref circle) => {
                match *other.curve {
                    CircularEdgeCurve::Circle(ref other_circle) => {
                        match circle_circle_intersection(circle, other_circle) {
                            CircleCircleIntersection::SinglePoint(point) => {
                                Ok(vec![point])
                            },
                            CircleCircleIntersection::TwoPoint(point1, point2) => {
                                Ok(vec![point1, point2])
                            },
                            CircleCircleIntersection::Circle(_) => {
                                Err("Two circles are equal.")
                            },
                            CircleCircleIntersection::None => {
                                Ok(vec![])
                            }
                        }
                    },
                    CircularEdgeCurve::Ellipse(ref other_ellipse) => {
                        todo!("Implement circle-ellipse intersection")
                    }
                }
            },
            CircularEdgeCurve::Ellipse(ref ellipse) => {
                todo!("Implement circle-ellipse intersection")
            }
        }
    }
}
