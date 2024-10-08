use crate::{bounding_box::BoundingBox, curves::CurveLike, points::point::Point, EQ_THRESHOLD};

// Alphabetical order
pub mod circle_circle;
pub mod circle_line;
pub mod curve_curve;
pub mod ellipse_ellipse;
pub mod helix_circle;
pub mod helix_line;
pub mod line_line;

const PRECISION: f64 = EQ_THRESHOLD / 100.0;

fn curve_curve_intersection_numerical_iteration(
    edge_self: &dyn CurveLike,
    edge_other: &dyn CurveLike,
    interval_self: (Point, Point),
    interval_other: (Point, Point),
) -> Vec<Point> {
    // println!("curve_curve_intersection_numerical_iteration");
    // println!("interval_self: {:?}", interval_self);
    // println!("interval_other: {:?}", interval_other);
    // For enhanced numerical stability, for small intervals, we approximate the curve as a line.
    let bounding_box_self = match (interval_self.0 - interval_self.1).norm_sq() < EQ_THRESHOLD {
        true => BoundingBox::with_2_points(interval_self.0, interval_self.1),
        false => edge_self.get_bounding_box(Some(interval_self.0), Some(interval_self.1)),
    };
    let bounding_box_other = match (interval_other.0 - interval_other.1).norm_sq() < EQ_THRESHOLD {
        true => BoundingBox::with_2_points(interval_other.0, interval_other.1),
        false => edge_other.get_bounding_box(Some(interval_other.0), Some(interval_other.1)),
    };

    // println!("bounding box self: {:?}", bounding_box_self);
    // println!("bounding box other: {:?}", bounding_box_other);

    // println!(
    //     "bounding box self intersects bounding box other: {:?}",
    //     bounding_box_self.intersects(&bounding_box_other, 0.0)
    // );

    if !bounding_box_self.intersects(&bounding_box_other, 0.0) {
        return Vec::new();
    }

    let midpoint_self = edge_self.get_midpoint(Some(interval_self.0), Some(interval_self.1));
    // println!("midpoint self: {:?}", midpoint_self);
    if bounding_box_self.max_size() < PRECISION {
        return vec![midpoint_self];
    }

    let midpoint_other = edge_other.get_midpoint(Some(interval_other.0), Some(interval_other.1));
    // println!("midpoint other: {:?}", midpoint_other);
    if bounding_box_other.max_size() < PRECISION {
        return vec![midpoint_other];
    }

    let mut result: Vec<Point> = Vec::new();
    result.extend(curve_curve_intersection_numerical_iteration(
        edge_self,
        edge_other,
        (interval_self.0, midpoint_self),
        (interval_other.0, midpoint_other),
    ));
    result.extend(curve_curve_intersection_numerical_iteration(
        edge_self,
        edge_other,
        (interval_self.0, midpoint_self),
        (midpoint_other, interval_other.1),
    ));
    result.extend(curve_curve_intersection_numerical_iteration(
        edge_self,
        edge_other,
        (midpoint_self, interval_self.1),
        (interval_other.0, midpoint_other),
    ));
    result.extend(curve_curve_intersection_numerical_iteration(
        edge_self,
        edge_other,
        (midpoint_self, interval_self.1),
        (midpoint_other, interval_other.1),
    ));
    // }
    // assert!(result.len() > 0);

    result
}

// Finds the intersection between two curves. They have to be intersecting only at a finite number of points.
pub fn curve_curve_intersection_numerical(
    edge_self: &dyn CurveLike,
    edge_other: &dyn CurveLike,
) -> Vec<Point> {
    let self_p0 = edge_self.get_midpoint(None, None);
    let self_p1 = edge_self.get_midpoint(Some(self_p0), None);
    let other_p0 = edge_other.get_midpoint(None, None);
    let other_p1 = edge_other.get_midpoint(Some(other_p0), None);
    let mut result = Vec::new();
    result.extend(curve_curve_intersection_numerical_iteration(
        edge_self,
        edge_other,
        (self_p0, self_p1),
        (other_p0, other_p1),
    ));
    result.extend(curve_curve_intersection_numerical_iteration(
        edge_self,
        edge_other,
        (self_p0, self_p1),
        (other_p1, other_p0),
    ));
    result.extend(curve_curve_intersection_numerical_iteration(
        edge_self,
        edge_other,
        (self_p1, self_p0),
        (other_p0, other_p1),
    ));
    result.extend(curve_curve_intersection_numerical_iteration(
        edge_self,
        edge_other,
        (self_p1, self_p0),
        (other_p1, other_p0),
    ));
    // Filter out duplicate points
    let mut unique_points = Vec::new();
    for p in result {
        if !unique_points.iter().any(|x| {
            let diff: Point = p - *x;
            diff.norm() < PRECISION * 10000.0
        }) {
            unique_points.push(p);
        }
    }

    for p in &unique_points {
        println!("unique point: {:?}", p);
    }
    unique_points
}
