use geop_geometry::{
    curve_curve_intersection::curve_curve::curve_curve_intersection, curves::curve::Curve,
};

use crate::topology::contour_no_point::ContourNoPoint;

pub fn ray_contour_hit_test(ray: &Edge, contour: ContourNoPoint) -> Option<Point> {
    let mut bb = ray.bounding_box();
    loop {
        bb = contour.shrink_bb(bb);
        bb = ray.shrink_bb(bb);
        if no_progress {
            let new_bbs = bb.subdivide();
            bb = ray.find_closes_bb_to_start(new_bbs);
        }
    }
}
