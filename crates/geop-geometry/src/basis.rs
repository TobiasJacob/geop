use crate::{
    color::Category10Color,
    geometry_error::{GeometryError, GeometryResult},
    geometry_scene::GeometryScene,
    point::Point,
};

// Simple struct that guarantees that the basis is orthogonal. The length of the vectors is not guaranteed to be 1.
pub struct Basis {
    pub basis: Point,
    pub x: Point,
    pub y: Point,
    pub z: Point,
}

impl Basis {
    pub fn from_points(basis: Point, x: Point, y: Point, z: Point) -> GeometryResult<Basis> {
        if !x.is_perpendicular(y) || !x.is_perpendicular(z) || !y.is_perpendicular(z) {
            return Err(GeometryError::new(
                "The basis vectors are not orthogonal.".to_string(),
                GeometryScene::with_points(vec![
                    (basis, Category10Color::Gray),
                    (x, Category10Color::Red),
                    (y, Category10Color::Blue),
                    (z, Category10Color::Green),
                ]),
            ));
        }
        Ok(Basis { basis, x, y, z })
    }
}
