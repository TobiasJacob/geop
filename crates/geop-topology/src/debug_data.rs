use geop_geometry::points::point::Point;

use crate::topology::{edge::Edge, face::Face};

#[derive(Debug, Clone)]
pub enum DebugColor {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    White,
    Black,
    Transparent,
}

impl DebugColor {
    pub fn to_color(&self) -> [f32; 4] {
        let a = 0.1;
        match self {
            DebugColor::Red => [1.0, 0.0, 0.0, a],
            DebugColor::Green => [0.0, 1.0, 0.0, a],
            DebugColor::Blue => [0.0, 0.0, 1.0, a],
            DebugColor::Yellow => [1.0, 1.0, 0.0, a],
            DebugColor::Cyan => [0.0, 1.0, 1.0, a],
            DebugColor::Magenta => [1.0, 0.0, 1.0, a],
            DebugColor::White => [1.0, 1.0, 1.0, a],
            DebugColor::Black => [0.0, 0.0, 0.0, a],
            DebugColor::Transparent => [0.0, 0.0, 0.0, a],
        }
    }
}

#[derive(Debug, Clone)]
pub struct DebugData {
    pub points: Vec<(Point, DebugColor)>,
    pub edges: Vec<(Edge, DebugColor)>,
    pub faces: Vec<(Face, DebugColor)>,
}

static mut DEBUG_DATA: Option<DebugData> = None;

fn init() {
    assert!(unsafe { DEBUG_DATA.is_none() });
    unsafe {
        DEBUG_DATA = Some(DebugData {
            points: Vec::new(),
            edges: Vec::new(),
            faces: Vec::new(),
        });
    }
}

pub fn add_point(point: Point, color: DebugColor) {
    unsafe {
        if DEBUG_DATA.is_none() {
            init();
        }
        DEBUG_DATA.as_mut().unwrap().points.push((point, color));
    }
}

pub fn add_edge(edge: Edge, color: DebugColor) {
    unsafe {
        if DEBUG_DATA.is_none() {
            init();
        }
        DEBUG_DATA.as_mut().unwrap().edges.push((edge, color));
    }
}

pub fn add_face(face: Face, color: DebugColor) {
    unsafe {
        if DEBUG_DATA.is_none() {
            init();
        }
        DEBUG_DATA.as_mut().unwrap().faces.push((face, color));
    }
}

pub fn get_debug_data() -> Option<DebugData> {
    unsafe { DEBUG_DATA.clone() }
}
