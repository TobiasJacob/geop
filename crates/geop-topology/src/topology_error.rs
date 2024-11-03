use std::backtrace::Backtrace;

use geop_geometry::geometry_error::GeometryError;

use crate::topology_scene::TopologyScene;

pub enum TopologyErrorRoot {
    InTopologyCrate {
        message: String,
        backtrace: Backtrace,
    },
    FromGeometryError {
        geometry_error: GeometryError,
    },
}

pub enum TopologyError {
    Context {
        message: String,
        error_scene: Option<TopologyScene>,
        inner_error: Box<TopologyError>,
    },
    Root(TopologyErrorRoot),
}

impl TopologyError {
    pub fn new(message: String) -> TopologyError {
        let backtrace = Backtrace::capture();
        TopologyError::Root(TopologyErrorRoot::InTopologyCrate { message, backtrace })
    }

    pub fn with_context(self, message: String) -> TopologyError {
        TopologyError::Context {
            message,
            error_scene: None,
            inner_error: Box::new(self),
        }
    }

    pub fn with_context_scene(self, message: String, error_scene: TopologyScene) -> TopologyError {
        TopologyError::Context {
            message,
            error_scene: Some(error_scene),
            inner_error: Box::new(self),
        }
    }
}

// From GeometryError
impl From<GeometryError> for TopologyError {
    fn from(error: GeometryError) -> TopologyError {
        TopologyError::Root(TopologyErrorRoot::FromGeometryError {
            geometry_error: error,
        })
    }
}

impl std::fmt::Display for TopologyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TopologyError::Context {
                message,
                error_scene: _,
                inner_error,
            } => {
                write!(f, "{}", inner_error)?;
                writeln!(f, "  TopologyContext: {}", message)
            }
            TopologyError::Root(root) => match root {
                TopologyErrorRoot::InTopologyCrate { message, backtrace } => {
                    writeln!(f, "TopologyError")?;
                    writeln!(f, "Backtrace: {}", backtrace)?;
                    writeln!(f, "RootError: {}", message)
                }
                TopologyErrorRoot::FromGeometryError { geometry_error } => {
                    write!(f, "{}", geometry_error)
                }
            },
        }
    }
}

impl std::fmt::Debug for TopologyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "GeometryError: {}", self)
    }
}

impl std::error::Error for TopologyError {
    fn description(&self) -> &str {
        "A geometry error occurred"
    }
}

pub type TopologyResult<T> = Result<T, TopologyError>;
