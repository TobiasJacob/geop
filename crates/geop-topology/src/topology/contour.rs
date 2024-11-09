use std::fmt::Display;

use geop_geometry::{point::Point, transforms::Transform};

use super::{
    contour_multi_point::ContourMultiPoint, contour_no_point::ContourNoPoint,
    contour_single_point::ContourSinglePoint, edge::Edge,
};

#[derive(Clone, Debug, PartialEq)]
pub enum EdgeIndex {
    OnEdge(usize),
    OnCorner(usize, usize),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ContourTangent {
    OnEdge(Point),
    OnCorner(Point, Point), // Ingoung and outgoing tangent
}

impl ContourTangent {
    pub fn expect_on_edge(&self) -> &Point {
        match self {
            ContourTangent::OnEdge(t) => t,
            ContourTangent::OnCorner(_, _) => panic!("Expected on edge"),
        }
    }
    pub fn expect_on_corner(&self) -> (&Point, &Point) {
        match self {
            ContourTangent::OnEdge(_) => panic!("Expected on corner"),
            ContourTangent::OnCorner(t1, t2) => (t1, t2),
        }
    }
    pub fn is_inside(&self, normal: Point, curve_dir: Point) -> bool {
        let (tangent1, tangent2) = match self {
            ContourTangent::OnEdge(tangent) => (tangent, tangent),
            ContourTangent::OnCorner(tangent1, tangent2) => (tangent1, tangent2),
        };
        // Check sign of det(tangent1 - curve_dir, tangent2 - curve_dir, normal - curve_dir)
        let curve_dir = -curve_dir.normalize().unwrap();
        let tangent1 = -tangent1.normalize().unwrap();
        let tangent2 = tangent2.normalize().unwrap();
        let det = (tangent1 - curve_dir)
            .cross(tangent2 - curve_dir)
            .dot(normal - curve_dir);
        det > 0.0
    }
}

#[derive(Clone, Debug)]
pub enum Contour {
    ContourMultiPoint(ContourMultiPoint),
    ContourSinglePoint(ContourSinglePoint),
    ContourNoPoint(ContourNoPoint),
}

impl Contour {
    pub fn from_edges(edges: Vec<Edge>) -> Contour {
        Contour::ContourMultiPoint(ContourMultiPoint::new(edges))
    }

    pub fn from_curve(curve: ContourNoPoint) -> Contour {
        Contour::ContourNoPoint(curve)
    }

    pub fn all_points(&self) -> Vec<Point> {
        match self {
            Contour::ContourMultiPoint(connected_edge) => connected_edge.all_points(),
            Contour::ContourSinglePoint(single_point) => single_point.all_points(),
            Contour::ContourNoPoint(curve) => curve.all_points(),
        }
    }

    pub fn flip(&self) -> Contour {
        match self {
            Contour::ContourMultiPoint(connected_edge) => {
                Contour::ContourMultiPoint(connected_edge.flip())
            }
            Contour::ContourSinglePoint(single_point) => {
                Contour::ContourSinglePoint(single_point.flip())
            }
            Contour::ContourNoPoint(curve) => Contour::ContourNoPoint(curve.flip()),
        }
    }

    pub fn transform(&self, transform: Transform) -> Contour {
        match self {
            Contour::ContourMultiPoint(connected_edge) => {
                Contour::ContourMultiPoint(connected_edge.transform(transform))
            }
            Contour::ContourSinglePoint(single_point) => {
                Contour::ContourSinglePoint(single_point.transform(transform))
            }
            Contour::ContourNoPoint(curve) => Contour::ContourNoPoint(curve.transform(transform)),
        }
    }

    pub fn tangent(&self, p: Point) -> ContourTangent {
        match self {
            Contour::ContourMultiPoint(connected_edge) => connected_edge.tangent(p),
            Contour::ContourSinglePoint(single_point) => single_point.tangent(p),
            Contour::ContourNoPoint(curve) => curve.tangent(p),
        }
    }

    pub fn get_midpoint(&self) -> Point {
        match self {
            Contour::ContourMultiPoint(connected_edge) => connected_edge.get_midpoint(),
            Contour::ContourSinglePoint(single_point) => single_point.get_midpoint(),
            Contour::ContourNoPoint(curve) => curve.get_midpoint(),
        }
    }

    // Gets the subcurve between these two points. It is guaranteed that there will be no zero length edges.
    pub fn get_subcurve(&self, start: Point, end: Point) -> Vec<Edge> {
        match self {
            Contour::ContourMultiPoint(connected_edge) => connected_edge.get_subcurve(start, end),
            Contour::ContourSinglePoint(single_point) => single_point.get_subcurve(start, end),
            Contour::ContourNoPoint(curve) => vec![curve.get_subcurve(start, end)],
        }
    }

    pub fn insert_point(&self, point: Point) -> Contour {
        match self {
            Contour::ContourMultiPoint(connected_edge) => {
                Contour::ContourMultiPoint(connected_edge.insert_point(point))
            }
            Contour::ContourSinglePoint(single_point) => {
                Contour::ContourMultiPoint(single_point.insert_point(point))
            }
            Contour::ContourNoPoint(curve) => {
                Contour::ContourSinglePoint(curve.insert_point(point))
            }
        }
    }
}

impl Display for Contour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Contour::ContourMultiPoint(connected_edge) => write!(f, "{}", connected_edge),
            Contour::ContourSinglePoint(single_point) => write!(f, "{}", single_point),
            Contour::ContourNoPoint(curve) => write!(f, "{}", curve),
        }
    }
}
