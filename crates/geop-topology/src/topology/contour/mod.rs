pub mod connected_egde_contour;
pub mod curve_contour;

use std::fmt::Display;

use connected_egde_contour::ConnectedEdgeContour;
use curve_contour::CurveContour;
use geop_geometry::{point::Point, transforms::Transform};

use super::edge::Edge;

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
    ConnectedEdge(ConnectedEdgeContour),
    Curve(CurveContour),
}

impl Contour {
    pub fn from_edges(edges: Vec<Edge>) -> Contour {
        Contour::ConnectedEdge(ConnectedEdgeContour::new(edges))
    }

    pub fn from_curve(curve: CurveContour) -> Contour {
        Contour::Curve(curve)
    }

    pub fn all_points(&self) -> Vec<Point> {
        match self {
            Contour::ConnectedEdge(connected_edge) => connected_edge.all_points(),
            Contour::Curve(curve) => curve.all_points(),
        }
    }

    pub fn flip(&self) -> Contour {
        match self {
            Contour::ConnectedEdge(connected_edge) => Contour::ConnectedEdge(connected_edge.flip()),
            Contour::Curve(curve) => Contour::Curve(curve.flip()),
        }
    }

    pub fn transform(&self, transform: Transform) -> Contour {
        match self {
            Contour::ConnectedEdge(connected_edge) => {
                Contour::ConnectedEdge(connected_edge.transform(transform))
            }
            Contour::Curve(curve) => Contour::Curve(curve.transform(transform)),
        }
    }

    pub fn tangent(&self, p: Point) -> ContourTangent {
        match self {
            Contour::ConnectedEdge(connected_edge) => connected_edge.tangent(p),
            Contour::Curve(curve) => curve.tangent(p),
        }
    }

    pub fn get_midpoint(&self) -> Point {
        match self {
            Contour::ConnectedEdge(connected_edge) => connected_edge.get_midpoint(),
            Contour::Curve(curve) => curve.get_midpoint(),
        }
    }

    // Gets the subcurve between these two points. It is guaranteed that there will be no zero length edges.
    pub fn get_subcurve(&self, start: Point, end: Point) -> Vec<Edge> {
        match self {
            Contour::ConnectedEdge(connected_edge) => connected_edge.get_subcurve(start, end),
            Contour::Curve(curve) => curve.get_subcurve(start, end),
        }
    }

    pub fn get_subcurve_single_point(&self, point: Point) -> Vec<Edge> {
        match self {
            Contour::ConnectedEdge(connected_edge) => {
                connected_edge.get_subcurve_single_point(point)
            }
            Contour::Curve(curve) => curve.get_subcurve_single_point(point),
        }
    }
}

impl Display for Contour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Contour::ConnectedEdge(connected_edge) => write!(f, "{}", connected_edge),
            Contour::Curve(curve) => write!(f, "{}", curve),
        }
    }
}
