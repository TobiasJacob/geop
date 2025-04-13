use std::ops::Mul;

use crate::{color::Category10Color, point::Point};

use super::{edge::Edge, face::Face, volume::Volume};

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r, g, b, a }
    }

    pub fn from_category_color(color: Category10Color) -> Color {
        // #1f77b4
        // #ff7f0e
        // #2ca02c
        // #d62728
        // #9467bd
        // #8c564b
        // #e377c2
        // #7f7f7f
        // #bcbd22
        // #17becf
        match color {
            Category10Color::Blue => Color::new(0.12, 0.47, 0.71, 1.0),
            Category10Color::Orange => Color::new(1.0, 0.5, 0.0, 1.0),
            Category10Color::Green => Color::new(0.18, 0.59, 0.29, 1.0),
            Category10Color::Red => Color::new(0.85, 0.33, 0.1, 1.0),
            Category10Color::Purple => Color::new(0.58, 0.4, 0.74, 1.0),
            Category10Color::Brown => Color::new(0.55, 0.34, 0.29, 1.0),
            Category10Color::Pink => Color::new(0.75, 0.58, 0.83, 1.0),
            Category10Color::Gray => Color::new(0.5, 0.5, 0.5, 1.0),
            Category10Color::Olive => Color::new(0.74, 0.83, 0.56, 1.0),
            Category10Color::Cyan => Color::new(0.09, 0.65, 0.74, 1.0),
        }
    }

    pub fn standard_pallet(dark_mode: bool) -> (Color, Color, Color, Color) {
        let background_color = if dark_mode {
            Color::from_brightness(0.2)
        } else {
            Color::from_brightness(1.0)
        };
        let face_color = if dark_mode {
            Color::from_brightness(0.2)
        } else {
            Color::from_brightness(0.6)
        };
        let edge_color = if dark_mode {
            Color::from_brightness(0.7)
        } else {
            Color::from_brightness(0.2)
        };
        let point_color = if dark_mode {
            Color::from_brightness(0.8)
        } else {
            Color::from_brightness(0.1)
        };

        (background_color, face_color, edge_color, point_color)
    }

    pub fn from_brightness(brightness: f32) -> Color {
        Color {
            r: brightness,
            g: brightness,
            b: brightness,
            a: 1.0,
        }
    }

    pub fn white() -> Color {
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }

    pub fn red() -> Color {
        Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }

    pub fn green() -> Color {
        Color {
            r: 0.0,
            g: 1.0,
            b: 0.0,
            a: 1.0,
        }
    }

    pub fn blue() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 1.0,
            a: 1.0,
        }
    }

    pub fn gray() -> Color {
        Color {
            r: 0.5,
            g: 0.5,
            b: 0.5,
            a: 1.0,
        }
    }

    pub fn light_gray() -> Color {
        Color {
            r: 0.85,
            g: 0.85,
            b: 0.85,
            a: 1.0,
        }
    }

    pub fn black() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }

    pub fn ten_different_colors(i: usize) -> Color {
        vec![
            Color::new(1.0, 0.0, 0.0, 1.0),
            Color::new(0.0, 1.0, 0.0, 1.0),
            Color::new(0.0, 0.0, 1.0, 1.0),
            Color::new(1.0, 1.0, 0.0, 1.0),
            Color::new(1.0, 0.0, 1.0, 1.0),
            Color::new(0.0, 1.0, 1.0, 1.0),
            Color::new(0.5, 0.0, 0.0, 1.0),
            Color::new(0.0, 0.5, 0.0, 1.0),
            Color::new(0.0, 0.0, 0.5, 1.0),
            Color::new(0.5, 0.5, 0.0, 1.0),
        ][i % 10]
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Color {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
            a: self.a,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Scene {
    pub volumes: Vec<(Volume, Color)>,
    pub faces: Vec<(Face, Color)>,
    pub edges: Vec<(Edge, Color)>,
    pub points: Vec<(Point, Color)>,
}

impl Scene {
    pub fn new(
        volumes: Vec<(Volume, Color)>,
        faces: Vec<(Face, Color)>,
        edges: Vec<(Edge, Color)>,
        points: Vec<(Point, Color)>,
    ) -> Scene {
        Scene {
            volumes,
            faces,
            edges,
            points,
        }
    }

    pub fn empty() -> Scene {
        Scene {
            volumes: Vec::new(),
            faces: Vec::new(),
            edges: Vec::new(),
            points: Vec::new(),
        }
    }
}
