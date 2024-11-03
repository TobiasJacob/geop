use crate::{
    color::Category10Color,
    geometry_error::{GeometryError, GeometryResult},
    geometry_scene::GeometryScene,
    point::Point,
};

// Represents the bounds of a curve.
// Makes sure that start != end if the curve is bounded.
#[derive(Debug, Clone, PartialEq)]
pub struct Bounds {
    pub start: Point,
    pub end: Point,
}

impl Bounds {
    pub fn new(start: Point, end: Point) -> GeometryResult<Bounds> {
        if start == end {
            return Err(
                GeometryError::new("Bounds are equal".to_string()).with_context_scene(
                    format!("Try to create bounds with {start} and {end}"),
                    GeometryScene::with_points(vec![
                        (start, Category10Color::Orange),
                        (end, Category10Color::Green),
                    ]),
                ),
            );
        }
        Ok(Bounds { start, end })
    }

    pub fn flip(&self) -> Bounds {
        Bounds::new(self.end, self.start).expect("Bounds start and end are different")
    }
}
