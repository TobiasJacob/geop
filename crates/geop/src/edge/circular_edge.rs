pub enum CircularEdgeCurve {
    Circle,
    Ellipse,
}

pub enum CircularEdgeBounds {
    None,
    Anchor(Point),
    StartEnd(Point, Point),
}
