use crate::{
    color::Category10Color, curves::curve::Curve, point::Point, surfaces::surface::Surface,
};

pub struct GeometryScene {
    pub points: Vec<(Point, Category10Color)>,
    pub curves: Vec<(Curve, Category10Color)>,
    pub surfaces: Vec<(Surface, Category10Color)>,
}

impl GeometryScene {
    pub fn new() -> GeometryScene {
        GeometryScene {
            points: Vec::new(),
            curves: Vec::new(),
            surfaces: Vec::new(),
        }
    }

    pub fn with_points(points: Vec<(Point, Category10Color)>) -> GeometryScene {
        GeometryScene {
            points,
            curves: Vec::new(),
            surfaces: Vec::new(),
        }
    }

    pub fn with_curves(curves: Vec<(Curve, Category10Color)>) -> GeometryScene {
        GeometryScene {
            points: Vec::new(),
            curves,
            surfaces: Vec::new(),
        }
    }

    pub fn with_surfaces(surfaces: Vec<(Surface, Category10Color)>) -> GeometryScene {
        GeometryScene {
            points: Vec::new(),
            curves: Vec::new(),
            surfaces,
        }
    }
}
