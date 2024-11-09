use geop_geometry::{
    curve_curve_intersection::curve_curve::{curve_curve_intersection, CurveCurveIntersection},
    point::Point,
};

use crate::topology::{contour::Contour, edge::Edge};

// Finds the closest intersection point between a ray and a contour.
pub fn ray_contour_hit_test(ray: &Edge, contour: &Contour) -> Option<Point> {
    match contour {
        Contour::ContourNoPoint(curve_contour) => {
            match curve_curve_intersection(&ray.edge.curve, &curve_contour.curve) {
                CurveCurveIntersection::FinitePoints(points) => {
                    return Some(intersection);
                }
            }
        }
        Contour::ContourSinglePoint(single_contour) => {
            if ray.origin == single_contour.point {
                return Some(single_contour.point);
            }
            match single_contour.curve.intersect_ray(ray) {
                Some(intersection) => {
                    return Some(intersection);
                }
                None => {
                    return None;
                }
            }
        }
        Contour::ContourMultiPoint(composite_contour) => {
            for edge in composite_contour.edges.iter() {
                let intersection: Option<Point> = edge.intersect_ray(ray);
                match intersection {
                    Some(intersection) => {
                        return Some(intersection);
                    }
                    None => {}
                }
            }
        }
    }
    return None;
}
