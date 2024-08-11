use crate::{curves::curve::Curve, points::point::Point, surfaces::surface::Surface};

use super::{
    circle_plane::{circle_plane_intersection, CirclePlaneIntersection},
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
            Surface::Cylinder(_cylinder) => {
                todo!("Implement line-cylinder intersection.");
            }
        },
        Curve::Circle(circle) => match surface {
            Surface::Plane(plane) => match circle_plane_intersection(circle, plane) {
                CirclePlaneIntersection::None => CurveSurfaceIntersection::None,
                CirclePlaneIntersection::Points(points) => CurveSurfaceIntersection::Points(points),
                CirclePlaneIntersection::Circle(circle) => {
                    CurveSurfaceIntersection::Curve(Curve::Circle(circle))
                }
            },
            Surface::Sphere(_sphere) => {
                todo!("Implement circle-sphere intersection.");
            }
            Surface::Cylinder(_cylinder) => {
                todo!("Implement circle-cylinder intersection.");
            }
        },
        Curve::Ellipsis(_) => todo!("Implement this"),
    }
}
