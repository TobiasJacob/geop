pub struct ImplicitCurve {
    pub f1: Box<dyn Fn(f64, f64) -> f64>,
    pub f2: Box<dyn Fn(f64, f64) -> f64>,
}

trait AsImplicitCurve {
    pub fn as_implicit(&self) -> ImplicitCurve;
}
