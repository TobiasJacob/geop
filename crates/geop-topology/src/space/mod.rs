
struct TopologicalSpace2d {
    min: (f64, f64),
    max: (f64, f64),
    periodic: (f64, f64)
}

// Topological space assumes:
// - No min or max or both have to be given for an axis
impl TopologicalSpace2d {
    pub fn from_plane() -> TopologicalSpace2d {
        TopologicalSpace2d {
            min: (f64::NEG_INFINITY, f64::NEG_INFINITY),
            max: (f64::INFINITY, f64::INFINITY),
            periodic: (f64::INFINITY, f64::INFINITY),
        }
    }

    pub fn from_cylinder() -> TopologicalSpace2d {
        TopologicalSpace2d {
            min: (f64::NEG_INFINITY, f64::NEG_INFINITY),
            max: (f64::INFINITY, f64::INFINITY),
            periodic: (2.0 * std::f64::consts::PI, f64::INFINITY),
        }
    }

    pub fn from_sphere() -> TopologicalSpace2d {
        TopologicalSpace2d {
            min: (f64::NEG_INFINITY, -std::f64::consts::PI),
            max: (f64::INFINITY, std::f64::consts::PI),
            periodic: (2.0 * std::f64::consts::PI, f64::INFINITY),
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

