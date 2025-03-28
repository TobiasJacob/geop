use crate::{efloat::EFloat64, point::Point};

/// A line segment in 3D space defined by its start and end points.
#[derive(Debug, Clone, Copy)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    /// Creates a new line segment from two points.
    pub fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    /// Returns the start point of the line segment.
    pub fn start(&self) -> Point {
        self.start
    }

    /// Returns the end point of the line segment.
    pub fn end(&self) -> Point {
        self.end
    }

    /// Returns the direction vector of the line segment.
    pub fn direction(&self) -> Point {
        self.end - self.start
    }

    /// Returns the length of the line segment.
    pub fn length(&self) -> f64 {
        self.direction().norm().to_f64()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_creation() {
        let start = Point::from_f64(0.0, 0.0, 0.0);
        let end = Point::from_f64(1.0, 1.0, 1.0);
        let line = Line::new(start, end);

        assert_eq!(line.start(), start);
        assert_eq!(line.end(), end);
        assert_eq!(line.direction(), Point::from_f64(1.0, 1.0, 1.0));
        assert!((line.length() - 3.0_f64.sqrt()).abs() < 1e-10);
    }
}
