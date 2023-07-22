trait Curve2d {
    fn get_value(&self, u: f64) -> Point2d;
    fn project(&self, x: Point2d) -> f64;
    fn normalize(&mut self);
    fn is_normalized(&self) -> bool;
}