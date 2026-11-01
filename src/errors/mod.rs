use std::fmt;
use std::error::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppError {
    pub code: u32,
    pub message: String,
}

impl AppError {
    pub fn new(code: u32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    pub fn safe_convert<'a>(err: &'a (dyn Error + 'static)) -> Result<&'a AppError, String> {
        match err.downcast_ref::<AppError>() {
            Some(app_err) => Ok(app_err),
            None => Err(format!("Failed to convert error to AppError: {}", err)),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error [{}]: {}", self.code, self.message)
    }
}

impl Error for AppError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_error_creation_and_conversion() {
        let err = AppError::new(400, "Bad Request");
        assert_eq!(err.code, 400);
        assert_eq!(err.message, "Bad Request");

        let boxed: Box<dyn Error> = Box::new(err.clone());
        let converted = AppError::safe_convert(&*boxed).unwrap();
        assert_eq!(converted, &err);
    }
}
