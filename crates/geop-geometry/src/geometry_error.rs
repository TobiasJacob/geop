use std::backtrace::Backtrace;

use geop_algebra::algebra_error::AlgebraError;

use crate::geometry_scene::GeometryScene;

pub enum GeometryErrorRoot {
    InGeometryCrate {
        message: String,
        backtrace: Backtrace,
    },
    FromAlgebraError {
        algebra_error: AlgebraError,
    },
}

pub enum GeometryError {
    Context {
        message: String,
        error_scene: Option<GeometryScene>,
        inner_error: Box<GeometryError>,
    },
    Root(GeometryErrorRoot),
}

impl GeometryError {
    pub fn new(message: String) -> GeometryError {
        let backtrace = Backtrace::capture();
        GeometryError::Root(GeometryErrorRoot::InGeometryCrate { message, backtrace })
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
            GeometryError::Root(GeometryErrorRoot::InGeometryCrate { message, backtrace }) => {
                writeln!(f, "GeometryError")?;
                writeln!(f, "Backtrace: {}", backtrace)?;
                writeln!(f, "RootError: {}", message)
            }
            GeometryError::Root(GeometryErrorRoot::FromAlgebraError { algebra_error }) => {
                write!(f, "{}", algebra_error)
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

impl From<&str> for GeometryError {
    fn from(message: &str) -> Self {
        GeometryError::new(message.to_string())
    }
}

impl From<AlgebraError> for GeometryError {
    fn from(error: AlgebraError) -> Self {
        GeometryError::new(error.to_string())
    }
}

pub type GeometryResult<T> = Result<T, GeometryError>;

pub trait WithContext<T> {
    fn with_context(
        self,
        context_generator: &dyn Fn(GeometryError) -> GeometryError,
    ) -> GeometryResult<T>;
}

impl<T> WithContext<T> for GeometryResult<T> {
    fn with_context(
        self,
        context_generator: &dyn Fn(GeometryError) -> GeometryError,
    ) -> GeometryResult<T> {
        match self {
            Ok(v) => Ok(v),
            Err(err) => Err(context_generator(err)),
        }
    }
}
