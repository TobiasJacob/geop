use crate::{surfaces::surface::Surface, curves::curve::Curve, points::point::Point};

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
    todo!("curve_surface_intersection")
}
