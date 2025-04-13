pub enum LineEdgeCurve {
    Line,
    Helix,
}

pub struct LineEdge {
    curve: LineEdgeCurve,
    start: Point,
    end: Point,
}
