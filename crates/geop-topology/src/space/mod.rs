use geop_geometry::geometry::points::point2d::Point2d;

struct TopologicalSpace2d {
    min: Point2d,
    max: Point2d,
    periodic: Point2d
}

// Topological space assumes:
// - No min or max or both have to be given for an axis
impl TopologicalSpace2d {
    pub fn from_plane() -> TopologicalSpace2d {
        TopologicalSpace2d {
            min: Point2d::new(f64::NEG_INFINITY, f64::NEG_INFINITY),
            max: Point2d::new(f64::INFINITY, f64::INFINITY),
            periodic: Point2d::new(f64::INFINITY, f64::INFINITY),
        }
    }

    pub fn from_cylinder() -> TopologicalSpace2d {
        TopologicalSpace2d {
            min: Point2d::new(f64::NEG_INFINITY, f64::NEG_INFINITY),
            max: Point2d::new(f64::INFINITY, f64::INFINITY),
            periodic: Point2d::new(2 * std::f64::consts::PI, f64::INFINITY),
        }
    }

    pub fn from_sphere() -> TopologicalSpace2d {
        TopologicalSpace2d {
            min: Point2d::new(f64::NEG_INFINITY, -std::f64::consts::PI),
            max: Point2d::new(f64::INFINITY, std::f64::consts::PI),
            periodic: Point2d::new(2 * std::f64::consts::PI, f64::INFINITY),
        }
    }

    pub fn new() {
        todo!("Asdf")
    }

    pub fn rasterize_line() {
        todo!("Asdf")
    }

    pub fn rasterize_curve() {
        todo!("Asdf")
    }
}

