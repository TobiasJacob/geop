use std::backtrace::Backtrace;

pub enum AlgebraError {
    Context {
        message: String,
        inner_error: Box<AlgebraError>,
    },
    Root {
        message: String,
        backtrace: Backtrace,
    },
}

impl AlgebraError {
    pub fn new(message: String) -> AlgebraError {
        let backtrace = Backtrace::capture();
        AlgebraError::Root { message, backtrace }
    }

    pub fn with_context(self, message: String) -> AlgebraError {
        AlgebraError::Context {
            message,
            inner_error: Box::new(self),
        }
    }
}

impl std::fmt::Display for AlgebraError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AlgebraError::Context {
                message,
                inner_error,
            } => {
                write!(f, "{}", inner_error)?;
                writeln!(f, "  AlgebraContext: {}", message)?;
                Ok(())
            }
            AlgebraError::Root { message, backtrace } => {
                writeln!(f, "AlgebraError")?;
                writeln!(f, "Backtrace: {}", backtrace)?;
                writeln!(f, "RootError: {}", message)
            }
        }
    }
}

impl std::fmt::Debug for AlgebraError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for AlgebraError {
    fn description(&self) -> &str {
        "A algebra error occurred"
    }
}

impl From<&str> for AlgebraError {
    fn from(message: &str) -> Self {
        AlgebraError::new(message.to_string())
    }
}

pub type AlgebraResult<T> = Result<T, AlgebraError>;

pub trait WithContext<T> {
    fn with_context(
        self,
        context_generator: &dyn Fn(AlgebraError) -> AlgebraError,
    ) -> AlgebraResult<T>;
}

impl<T> WithContext<T> for AlgebraResult<T> {
    fn with_context(
        self,
        context_generator: &dyn Fn(AlgebraError) -> AlgebraError,
    ) -> AlgebraResult<T> {
        match self {
            Ok(v) => Ok(v),
            Err(err) => Err(context_generator(err)),
        }
    }
}
