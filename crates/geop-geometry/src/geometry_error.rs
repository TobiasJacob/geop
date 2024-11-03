use crate::geometry_scene::GeometryScene;

pub struct GeometryError {
    pub message: String,
    pub error_scene: GeometryScene,
    pub inner_error: Option<Box<GeometryError>>,
}

impl GeometryError {
    pub fn new(message: String, error_scene: GeometryScene) -> GeometryError {
        GeometryError {
            message,
            error_scene,
            inner_error: None,
        }
    }

    pub fn with_inner(
        message: String,
        error_scene: GeometryScene,
        inner_error: GeometryError,
    ) -> GeometryError {
        GeometryError {
            message,
            error_scene,
            inner_error: Some(Box::new(inner_error)),
        }
    }
}

impl std::fmt::Display for GeometryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "GeometryError: {}", self.message)
    }
}

impl std::fmt::Debug for GeometryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "GeometryError: {}", self.message)
    }
}

impl std::error::Error for GeometryError {
    fn description(&self) -> &str {
        &self.message
    }
}

pub type GeometryResult<T> = Result<T, GeometryError>;
