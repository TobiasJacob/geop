use crate::{
    curves::{curve::Curve, CurveLike},
    points::point::Point,
};

// Alphabetical order
pub mod circle_circle;
pub mod circle_line;
pub mod curve_curve;
pub mod ellipsis_ellipsis;
pub mod line_line;

const PRECISION: f64 = 1e-6;

fn curve_curve_intersection_numerical_iteration(
    edge_self: &dyn CurveLike,
    edge_other: &dyn CurveLike,
    interval_self: (Option<Point>, Option<Point>),
    interval_other: (Option<Point>, Option<Point>),
) -> Vec<Point> {
    println!("curve_curve_intersection_numerical_iteration");
    let bounding_box_self = edge_self.get_bounding_box(interval_self.0, interval_self.1);
    let bounding_box_other = edge_other.get_bounding_box(interval_other.0, interval_other.1);

    if !bounding_box_self.intersects(&bounding_box_other) {
        return Vec::new();
    }

    let midpoint_self = edge_self.get_midpoint(interval_self.0, interval_self.1);
    if bounding_box_self.min_size() < PRECISION {
        return vec![midpoint_self];
    }

    let midpoint_other = edge_other.get_midpoint(interval_other.0, interval_other.1);
    if bounding_box_other.min_size() < PRECISION {
        return vec![midpoint_other];
    }

    let mut result: Vec<Point> = Vec::new();
    result.extend(curve_curve_intersection_numerical_iteration(
        edge_self,
        edge_other,
        (interval_self.0, Some(midpoint_self)),
        (interval_other.0, Some(midpoint_other)),
    ));
    result.extend(curve_curve_intersection_numerical_iteration(
        edge_self,
        edge_other,
        (interval_self.0, Some(midpoint_self)),
        (Some(midpoint_other), interval_other.1),
    ));
    result.extend(curve_curve_intersection_numerical_iteration(
        edge_self,
        edge_other,
        (Some(midpoint_self), interval_self.1),
        (interval_other.0, Some(midpoint_other)),
    ));
    result.extend(curve_curve_intersection_numerical_iteration(
        edge_self,
        edge_other,
        (Some(midpoint_self), interval_self.1),
        (Some(midpoint_other), interval_other.1),
    ));
    result
}

// Finds the intersection between two curves. They have to be intersecting only at a finite number of points.
pub fn curve_curve_intersection_numerical(
    edge_self: &dyn CurveLike,
    edge_other: &dyn CurveLike,
) -> Vec<Point> {
    let result = curve_curve_intersection_numerical_iteration(
        edge_self,
        edge_other,
        (None, None),
        (None, None),
    );
    // Filter out duplicate points
    let mut unique_points = Vec::new();
    for p in result {
        if !unique_points.iter().any(|x| {
            let diff: Point = p - *x;
            diff.norm() < PRECISION
        }) {
            unique_points.push(p);
        }
    }
    unique_points
}
