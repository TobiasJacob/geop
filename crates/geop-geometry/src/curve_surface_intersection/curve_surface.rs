use crate::{curves::curve::Curve, points::point::Point, surfaces::surface::Surface};

use super::{
    circle_cylinder::{circle_cylinder_intersection, CircleCylinderIntersection},
    circle_plane::{circle_plane_intersection, CirclePlaneIntersection},
    line_cylinder::{line_cylinder_intersection, CylinderLineIntersection},
    line_plane::{line_plane_intersection, LinePlaneIntersection},
};

pub enum CurveSurfaceIntersection {
    None,
    Points(Vec<Point>),
    Curve(Curve),
}

impl CurveSurfaceIntersection {
    pub fn is_none(&self) -> bool {
        match self {
            CurveSurfaceIntersection::None => true,
            _ => false,
        }
    }

    pub fn is_points(&self) -> bool {
        match self {
            CurveSurfaceIntersection::Points(_) => true,
            _ => false,
        }
    }

    pub fn is_curve(&self) -> bool {
        match self {
            CurveSurfaceIntersection::Curve(_) => true,
            _ => false,
        }
    }
}

pub fn curve_surface_intersection(curve: &Curve, surface: &Surface) -> CurveSurfaceIntersection {
    match curve {
        Curve::Line(line) => match surface {
            Surface::Plane(plane) => match line_plane_intersection(line, plane) {
                LinePlaneIntersection::Line(line) => {
                    CurveSurfaceIntersection::Curve(Curve::Line(line))
                }
                LinePlaneIntersection::Point(point) => {
                    CurveSurfaceIntersection::Points(vec![point])
                }
                LinePlaneIntersection::None => CurveSurfaceIntersection::None,
            },
            Surface::Sphere(_sphere) => {
                todo!("Implement line-sphere intersection.");
            }
            Surface::Cylinder(cylinder) => match line_cylinder_intersection(line, cylinder) {
                CylinderLineIntersection::Line(line) => {
                    CurveSurfaceIntersection::Curve(Curve::Line(line))
                }
                CylinderLineIntersection::TwoPoints(point1, point2) => {
                    CurveSurfaceIntersection::Points(vec![point1, point2])
                }
                CylinderLineIntersection::Point(point) => {
                    CurveSurfaceIntersection::Points(vec![point])
                }
                CylinderLineIntersection::None => CurveSurfaceIntersection::None,
            },
        },
        Curve::Circle(circle) => match surface {
            Surface::Plane(plane) => match circle_plane_intersection(circle, plane) {
                CirclePlaneIntersection::None => CurveSurfaceIntersection::None,
                CirclePlaneIntersection::TwoPoints(p1, p2) => {
                    CurveSurfaceIntersection::Points(vec![p1, p2])
                }
                CirclePlaneIntersection::OnePoint(p) => CurveSurfaceIntersection::Points(vec![p]),
                CirclePlaneIntersection::Circle(circle) => {
                    CurveSurfaceIntersection::Curve(Curve::Circle(circle))
                }
            },
            Surface::Sphere(_sphere) => {
                todo!("Implement circle-sphere intersection.");
            }
            Surface::Cylinder(cylinder) => match circle_cylinder_intersection(circle, cylinder) {
                CircleCylinderIntersection::Circle(circle) => {
                    CurveSurfaceIntersection::Curve(Curve::Circle(circle))
                }
                CircleCylinderIntersection::TwoPoints(point1, point2) => {
                    CurveSurfaceIntersection::Points(vec![point1, point2])
                }
                CircleCylinderIntersection::OnePoint(point) => {
                    CurveSurfaceIntersection::Points(vec![point])
                }
                CircleCylinderIntersection::None => CurveSurfaceIntersection::None,
            },
        },
        Curve::Ellipse(_) => todo!("Implement this"),
        Curve::Helix(_) => todo!("Implement this"),
    }
}
