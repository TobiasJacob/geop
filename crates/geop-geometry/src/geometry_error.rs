use std::backtrace::Backtrace;

use crate::geometry_scene::GeometryScene;

pub enum GeometryError {
    Context {
        message: String,
        error_scene: Option<GeometryScene>,
        inner_error: Box<GeometryError>,
    },
    Root {
        message: String,
        backtrace: Backtrace,
    },
}

impl GeometryError {
    pub fn new(message: String) -> GeometryError {
        let backtrace = Backtrace::capture();
        GeometryError::Root { message, backtrace }
    }

    pub fn with_context(self, message: String) -> GeometryError {
        GeometryError::Context {
            message,
            error_scene: None,
            inner_error: Box::new(self),
        }
    }

    pub fn with_context_scene(self, message: String, error_scene: GeometryScene) -> GeometryError {
        GeometryError::Context {
            message,
            error_scene: Some(error_scene),
            inner_error: Box::new(self),
        }
    }
}

impl std::fmt::Display for GeometryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GeometryError::Context {
                message,
                error_scene: _,
                inner_error,
            } => {
                write!(f, "{}", inner_error)?;
                writeln!(f, "  GeometryContext: {}", message)?;
                Ok(())
            }
            GeometryError::Root { message, backtrace } => {
                writeln!(f, "GeometryError")?;
                writeln!(f, "Backtrace: {}", backtrace)?;
                writeln!(f, "RootError: {}", message)
            }
        }
    }
}

impl std::fmt::Debug for GeometryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "GeometryError: {}", self)
    }
}

impl std::error::Error for GeometryError {
    fn description(&self) -> &str {
        "A geometry error occurred"
    }
}

pub type GeometryResult<T> = Result<T, GeometryError>;
