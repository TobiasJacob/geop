struct Point2d {
    x: f64,
    y: f64,
}

impl Point2d {
    fn new(x: f64, y: f64) -> Point2d {
        Point2d { x, y }
    }

    fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn dot(&self, other: Point2d) -> f64 {
        self.x * other.x + self.y * other.y
    }
}


impl Add for Point2d {
    type Output = Self;

    fn add(self, other: Point2d) -> Point2d {
        Point2d::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Point2d {
    type Output = Self;

    fn sub(self, other: Point2d) -> Point2d {
        Point2d::new(self.x - other.x, self.y - other.y)
    }
}
