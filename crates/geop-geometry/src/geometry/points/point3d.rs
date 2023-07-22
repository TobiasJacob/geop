struct Point3d {
    x: f64,
    y: f64,
    z: f64
}

impl Point3d {
    fn new(x: f64, y: f64, z: f64) -> Point3d {
        Point3d { x, y, z }
    }

    fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn dot(&self, other: Point3d) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Add for Point3d {
    type Output = Self;

    fn add(self, other: Point3d) -> Point3d {
        Point3d::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Point3d {
    type Output = Self;

    fn sub(self, other: Point3d) -> Point3d {
        Point3d::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}
